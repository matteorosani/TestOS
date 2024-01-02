#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_os::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
use x86_64::VirtAddr;

entry_point!(kmain);

static mut BOOT_INFO: Option<&'static BootInfo> = None;

fn kmain(boot_info: &'static BootInfo) -> ! {
    test_os::init(boot_info);
    unsafe { BOOT_INFO = Some(boot_info) };
    test_main();
    loop {}
}

#[test_case]
fn identity_mapped_vga_buffer() {
    let p = match unsafe { test_os::memory::translate_address(VirtAddr::new(0xb8000)) } {
        None => panic!("Can't convert address"),
        Some(phy) => phy.as_u64()
    };
    if p != 0xb8000 {
        panic!("Wrong conversion")
    }
}

#[test_case]
fn virtual_address_mapped_to_zero() {
    let p = match unsafe { test_os::memory::translate_address(VirtAddr::new(BOOT_INFO.expect("BootInfo not loaded correctly").physical_memory_offset)) } {
        None => panic!("Can't convert address"),
        Some(phy) => phy.as_u64()
    };
    if p != 0 {
        panic!("Wrong conversion")
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_os::test::test_panic_handler(info)
}