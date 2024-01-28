use bootloader::BootInfo;
use x86_64::{
    structures::paging::{PageTable, OffsetPageTable, FrameAllocator, Size4KiB},
    VirtAddr
};

mod boot_info;
pub mod heap;

pub unsafe fn init_page_table(boot_info: &'static BootInfo) -> OffsetPageTable<'static> {
    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    OffsetPageTable::new(active_level_4_table(physical_memory_offset), physical_memory_offset)
}

pub unsafe fn init_frame_allocator(boot_info: &'static BootInfo) -> impl FrameAllocator<Size4KiB> {
    boot_info::BootInfoFrameAllocator::init(&boot_info.memory_map)
}

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();
    let physical_address = level_4_table_frame.start_address();
    let virtual_address = physical_memory_offset + physical_address.as_u64();
    
    &mut *virtual_address.as_mut_ptr()
}