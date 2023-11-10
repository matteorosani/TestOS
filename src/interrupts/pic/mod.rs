pub mod intel8259;

pub fn init() {
    intel8259::init();
}

pub fn notify_end_of_interrupt(interrupt: u8) {
    unsafe {
        intel8259::PICS.lock().notify_end_of_interrupt(interrupt);
    }
}