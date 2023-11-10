#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_os::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use test_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");

    test_os::init();

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