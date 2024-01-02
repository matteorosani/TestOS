#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
use bootloader::entry_point;

use bootloader::BootInfo;
use x86_64::VirtAddr;

pub mod serial;
pub mod vga;
pub mod interrupts;
pub mod gdt;
pub mod test;
pub mod memory;


#[cfg(test)]
entry_point!(kmain);

#[cfg(test)]
/// Entry point for `cargo test`
fn kmain(boot_info: &'static BootInfo) -> ! {
    init(boot_info);
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    test::test_panic_handler(info)
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn init(boot_info: &'static BootInfo) {
    gdt::init();
    interrupts::init();
    unsafe {
        memory::init(VirtAddr::new(boot_info.physical_memory_offset));
    }
}