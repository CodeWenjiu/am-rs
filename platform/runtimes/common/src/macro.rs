/// Print macro for formatted output
///
/// This macro prints formatted text to stdout using the platform-specific putc function.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let _ = write!($crate::io::stdout(), $($arg)*);
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
            let _ = writeln!($crate::io::stdout(), $($arg)*);
        }
    };
}
