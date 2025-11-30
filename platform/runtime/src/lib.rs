#![no_std]

#[cfg(feature = "nemu")]
pub use nemu_runtime::*;

#[cfg(feature = "qemu")]
pub use qemu_runtime::*;

#[cfg(feature = "spike")]
pub use spike_runtime::*;

macros::mod_pub!(io);
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let _ = write!($crate::io::stdout(), $($arg)*);
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
            let _ = writeln!($crate::io::stdout(), $($arg)*);
        }
    };
}

#[macro_export]
macro_rules! preclude {
    () => {
        use runtime::{self as std, prelude::*};
        use std::vec;
    };
}

#[macro_export]
macro_rules! addtest {
    () => {
        #[cfg(test)]
        mod on_test {
            #[test]
            fn main() {
                super::main();
            }
        }
    };
}

#[macro_export]
macro_rules! libInit {
    () => {
        $crate::preclude!();
    };
}

#[macro_export]
macro_rules! binInit {
    () => {
        $crate::preclude!();

        use embedded_alloc::LlffHeap;
        #[global_allocator]
        static ALLOCATOR: LlffHeap = LlffHeap::empty();

        $crate::entry!(main);
    };
}

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[unsafe(export_name = "main")]
        pub unsafe fn __main() -> ! {
            let f: fn() = $path;

            $crate::heap_init!();

            f();

            unsafe extern "Rust" {
                fn platform_exit(code: i32) -> !;
            }
            unsafe { platform_exit(0) }
        }
    };
}

#[macro_export]
macro_rules! heap_init {
    () => {
        unsafe {
            unsafe extern "C" {
                static mut _sheap: u8;
                static mut _eheap: u8;
            }

            let heap_start = core::ptr::addr_of_mut!(_sheap) as usize;
            let heap_end = core::ptr::addr_of_mut!(_eheap) as usize;
            let heap_size = heap_end - heap_start;
            ALLOCATOR.init(heap_start, heap_size);
        }
    };
}

#[cfg(all(not(test), any(feature = "nemu", feature = "qemu", feature = "spike")))]
mod panic_handler {
    use crate::{exit::platform_exit, println};
    use core::panic::PanicInfo;

    #[panic_handler]
    fn panic(info: &PanicInfo) -> ! {
        println!("Panic: {}", info);
        platform_exit(1)
    }
}

macros::mod_flat!(pub_macro);
extern crate alloc;
pub use alloc::rc;
pub use alloc::vec;

pub mod prelude {
    pub use crate::{print, println};
    pub use alloc::boxed::Box;
    pub use alloc::string::String;
    pub use alloc::vec::Vec;
}
