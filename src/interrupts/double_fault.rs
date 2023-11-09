use x86_64::structures::idt::InterruptStackFrame;
use crate::println;

pub extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);

    loop {}
}

#[test_case]
fn test_double_fault_exception() {
    x86_64::instructions::interrupts::int3();
}