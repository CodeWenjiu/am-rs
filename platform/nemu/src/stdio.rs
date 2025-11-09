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

// Print macros for NEMU platform
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let _ = write!($crate::stdout(), $($arg)*);
        }
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let _ = writeln!($crate::stdout(), $($arg)*);
        }
    };
}
