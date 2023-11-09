use x86_64::structures::idt::InterruptDescriptorTable;
use lazy_static::lazy_static;

mod breakpoint;
mod double_fault;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint::breakpoint_handler);

        unsafe {
            idt.double_fault
            .set_handler_fn(double_fault::double_fault_handler)
            .set_stack_index(crate::gdt::tss::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_idt() {
    IDT.load()
}