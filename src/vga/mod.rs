mod color;
mod buffer;
mod writer;
#[cfg(test)]
mod test;

use lazy_static::lazy_static;
use spin::Mutex;

pub use crate::vga::color::{Color, ColorCode};
pub use crate::vga::buffer::{BUFFER_HEIGHT, BUFFER_WIDTH};

lazy_static! {
    pub static ref WRITER: Mutex<writer::Writer> = Mutex::new(writer::Writer::new(
        0, 
        ColorCode::new(Color::Yellow, Color::Black),
        unsafe {
            &mut *(0xb8000 as *mut buffer::Buffer)
        }
    ));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!('\n'));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts::without_interrupts;

    // Disables interrupts during the printing to prevent deadlocks
    without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}