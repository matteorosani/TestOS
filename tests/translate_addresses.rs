#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_os::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
use test_os::memory::init_frame_allocator;
use x86_64::{structures::paging::{Mapper, OffsetPageTable, Page, PageTableFlags, PhysFrame, Size4KiB, Translate}, PhysAddr, VirtAddr};

entry_point!(kmain);

static mut BOOT_INFO: Option<&'static BootInfo> = None;
static mut PAGE_TABLE: Option<OffsetPageTable<'static>> = None;

fn kmain(boot_info: &'static BootInfo) -> ! {
    test_os::gdt::init();
    test_os::interrupts::init();
    unsafe { BOOT_INFO = Some(boot_info) };
    let mapper =unsafe {
        test_os::memory::init_page_table(boot_info)
    };
    unsafe { PAGE_TABLE = Some(mapper) };
    test_main();
    loop {}
}

#[test_case]
fn identity_mapped_vga_buffer() {
    let p = match unsafe {
        match &PAGE_TABLE {
            Some(page_table) => page_table.translate_addr(VirtAddr::new(0xb8000)),
            None => panic!("")
        }
    } {
        None => panic!("Can't convert address"),
        Some(phy) => phy.as_u64()
    };
    if p != 0xb8000 {
        panic!("Wrong conversion")
    }
}

#[test_case]
fn virtual_address_mapped_to_zero() {
    unsafe {
        match &mut PAGE_TABLE {
            Some(page_table) => {
                let page = Page::<Size4KiB>::containing_address(VirtAddr::new(0x0));
                let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
                let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
                let mut frame_allocator = init_frame_allocator(BOOT_INFO.unwrap());
                page_table.map_to(page, frame, flags, &mut frame_allocator).expect("Can't map VirtAddr 0xdeadbeef").flush();
            },
            None => panic!("")
        };
    };
    let p = match unsafe {
        match &PAGE_TABLE {
            Some(page_table) => page_table.translate_addr(VirtAddr::new(0x0)),
            None => panic!("")
        }
    } {
        None => panic!("Can't convert address"),
        Some(phy) => phy.as_u64()
    };
    if p != 0xb8000 {
        panic!("Wrong conversion")
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_os::test::test_panic_handler(info)
}