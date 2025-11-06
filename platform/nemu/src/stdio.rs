const SERIAL_PORT: usize = 0x100003f8;

#[inline]
pub fn putc(ch: u8) {
    unsafe {
        core::ptr::write_volatile(SERIAL_PORT as *mut u8, ch);
    }
}

use core::fmt::{self, Write};

pub struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            putc(byte);
        }
        Ok(())
    }
}

pub fn stdout() -> Stdout {
    Stdout
}
