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

#[macro_export]
macro_rules! preclude {
    [ ] => {
        extern crate alloc;
        use $crate::{
            print, println,
        };

        use alloc::{
            vec::Vec,
            string::String,
            boxed::Box,
        };
    };
}
