use core::alloc::{GlobalAlloc, Layout};
use core::mem::{self, align_of, size_of};
use core::ptr::{self, NonNull};

use super::align_up;
use super::locked::Locked;

struct FreeListNode {
    size: usize,
    next: Option<&'static mut FreeListNode>,
}

impl FreeListNode {
    const fn new(size: usize) -> Self {
        Self { 
            size, 
            next: None 
        }
    }

    fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}

pub struct FreeListAllocator {
    head: FreeListNode,
}

impl FreeListAllocator {
    pub const fn new() -> Self {
        Self {
            head: FreeListNode::new(0),
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.add_free_region(heap_start, heap_size)
    }

    unsafe fn add_free_region(&mut self, start_addr: usize, size: usize) {
        assert_eq!(align_up(start_addr, align_of::<FreeListNode>()), start_addr);
        assert!(size >= size_of::<FreeListNode>());

        let mut node = FreeListNode::new(size);
        let node_ptr = start_addr as *mut FreeListNode;

        let mut current = &mut self.head;

        while current.next.is_some() && current.next.as_ref().map(|x| x.start_addr()).unwrap() < start_addr {
            current = current.next.as_mut().unwrap();
        }
        node.next = current.next.take();
        node_ptr.write(node);
        current.next = Some(&mut *node_ptr);

        self.merge_regions()
    }

    fn merge_regions(&mut self) {
        let mut current = &mut self.head;

        while current.next.is_some() {
            if current.end_addr() == current.next.as_ref().unwrap().start_addr() {
                let next = current.next.take().unwrap();
                current.size += next.size;
                current.next = next.next.take();
            } else {
                current = current.next.as_mut().unwrap();
            }
        }
    }

    fn find_region(&mut self, size: usize, align: usize) -> Option<(&'static mut FreeListNode, usize)> {
        let mut current = &mut self.head;

        while let Some(ref mut region) = current.next {
            if let Ok(alloc_start) = Self::alloc_from_region(&region, size, align) {
                let next = region.next.take();
                let ret = Some((current.next.take().unwrap(), alloc_start));
                current.next = next;
                return ret;
            } else {
                current = current.next.as_mut().unwrap();
            }
        }

        None
    }

    fn alloc_from_region(region: &FreeListNode, size: usize, align: usize) -> Result<usize, ()> {
        let alloc_start = align_up(region.start_addr(), align);
        let alloc_end = alloc_start.checked_add(size).ok_or(())?;

        if alloc_end > region.end_addr() {
            return Err(());
        }

        let excess_size = region.end_addr() - alloc_end;
        if excess_size > 0 && excess_size <mem::size_of::<FreeListNode>() {
            return Err(());
        }
        Ok(alloc_start)
    }

    /// Adjust the given layout so that the resulting allocated memory
    /// region is also capable of storing a `ListNode`.
    ///
    /// Returns the adjusted size and alignment as a (size, align) tuple.
    fn size_align(layout: Layout) -> (usize, usize) {
        let layout = layout
            .align_to(mem::align_of::<FreeListNode>())
            .expect("adjusting alignment failed")
            .pad_to_align();
        let size = layout.size().max(mem::size_of::<FreeListNode>());
        (size, layout.align())
    }

    pub unsafe fn allocate_first_fit(&mut self, layout: Layout) -> Result<NonNull<u8>, ()> {
        let (size, align) = FreeListAllocator::size_align(layout);
        if let Some((region, alloc_start)) = self.find_region(size, align) {
            let alloc_end = alloc_start.checked_add(size).expect("overflow");
            let excess_size = region.end_addr() - alloc_end;
            if excess_size > 0 {
                self.add_free_region(alloc_end, excess_size);
            }
            NonNull::new(alloc_start as *mut u8).ok_or(())
        } else {
            Err(())
        }
    }

    pub unsafe fn deallocate(&mut self, ptr: NonNull<u8>, layout: Layout) {
        let (size, _) = FreeListAllocator::size_align(layout);
        self.add_free_region(ptr.as_ptr() as usize, size);
    }
}

unsafe impl GlobalAlloc for Locked<FreeListAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // perform layout adjustments
        let (size, align) = FreeListAllocator::size_align(layout);
        let mut allocator = self.lock();

        if let Some((region, alloc_start)) = allocator.find_region(size, align) {
            let alloc_end = alloc_start.checked_add(size).expect("overflow");
            let excess_size = region.end_addr() - alloc_end;
            if excess_size > 0 {
                allocator.add_free_region(alloc_end, excess_size);
            }
            alloc_start as *mut u8
        } else {
            ptr::null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // perform layout adjustments
        let (size, _) = FreeListAllocator::size_align(layout);
        self.lock().add_free_region(ptr as usize, size)
    }
}