pub use crate::stdout;

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

pub use {print, println};
