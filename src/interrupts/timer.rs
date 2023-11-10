use x86_64::structures::idt::InterruptStackFrame;
use crate::print;

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame)
{
    print!(".");
    super::pic::notify_end_of_interrupt(super::index::InterruptIndex::Timer.as_u8())
}