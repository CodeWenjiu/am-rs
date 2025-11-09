/// Common Stdout implementation
///
/// This struct provides a Write implementation that calls the platform-specific putc function.
/// Each platform must provide a `putc(ch: u8)` function.
pub struct Stdout;

impl core::fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // Call platform-specific putc function
        unsafe extern "Rust" {
            fn putc(ch: u8);
        }

        for byte in s.bytes() {
            unsafe { putc(byte) };
        }
        Ok(())
    }
}

/// Get a Stdout handle
pub fn stdout() -> Stdout {
    Stdout
}

/// Print macro for formatted output
///
/// This macro prints formatted text to stdout using the platform-specific putc function.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let _ = write!($crate::stdout(), $($arg)*);
        }
    };
}

/// Println macro for formatted output with newline
///
/// This macro prints formatted text to stdout with a newline using the platform-specific putc function.
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
