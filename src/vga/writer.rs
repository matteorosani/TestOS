use core::fmt;

use super::{ColorCode, buffer::{Buffer, ScreenChar}, BUFFER_WIDTH, BUFFER_HEIGHT};

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    #[cfg(test)]
    pub buffer: &'static mut Buffer,
    #[cfg(not(test))]
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new(column_position: usize, color_code: ColorCode, buffer: &'static mut Buffer) -> Self {
        Self { 
            column_position, 
            color_code,
            buffer
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color_code = self.color_code;

                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code: color_code
                });
                self.column_position += 1;
            }
        }
    }

    pub fn new_line(&mut self) {
        for i in 1..BUFFER_HEIGHT {
            for j in 0..BUFFER_WIDTH {
                self.buffer.chars[i - 1][j].write(self.buffer.chars[i][j].read())
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
    }

    pub fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar { 
            ascii_character: b' ', 
            color_code: self.color_code 
        };
        for i in 0..BUFFER_WIDTH {
            self.buffer.chars[row][i].write(blank)
        }
        self.column_position = 0;
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe)
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}