use x86_64::structures::idt::InterruptDescriptorTable;
use lazy_static::lazy_static;

mod breakpoint;
mod double_fault;
mod pic;
mod timer;
mod keyboard;
mod index;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint::breakpoint_handler);

        unsafe {
            idt.double_fault
            .set_handler_fn(double_fault::double_fault_handler)
            .set_stack_index(crate::gdt::tss::DOUBLE_FAULT_IST_INDEX);
        }

        idt[index::InterruptIndex::Timer.as_usize()].set_handler_fn(timer::timer_interrupt_handler);

        idt[index::InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard::keyboard_interrupt_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

pub fn init_interrupts() {
    pic::init();
    x86_64::instructions::interrupts::enable();
}

pub fn init() {
    init_idt();
    init_interrupts();
}