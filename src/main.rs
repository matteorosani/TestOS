#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_os::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use test_os::println;
use bootloader::{BootInfo, entry_point};


entry_point!(kmain);

fn kmain(boot_info: &'static BootInfo) -> ! {
    println!("Hello World!");

    test_os::init(boot_info);

    #[cfg(test)]
    test_main();

    println!("It did't crash!");
    test_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    test_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_os::test::test_panic_handler(info)
}