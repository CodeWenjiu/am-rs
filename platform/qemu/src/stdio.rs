const UART_BASE: usize = 0x10000000;
const UART_LSR: usize = UART_BASE + 5;
const UART_LSR_THRE: u8 = 0x20;

#[inline]
pub fn putc(ch: u8) {
    unsafe {
        while (core::ptr::read_volatile(UART_LSR as *const u8) & UART_LSR_THRE) == 0 {}
        core::ptr::write_volatile(UART_BASE as *mut u8, ch);
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

// Print macros for QEMU platform
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
