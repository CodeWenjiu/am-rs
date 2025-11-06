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
        use nemu_runtime::{
            self,
            print, println,
        };

        use alloc::{
            vec::Vec,
            string::String,
            boxed::Box,
        };
    };
}

#[macro_export]
macro_rules! binInit {
    [ ] => {
        $crate::preclude!();

        use embedded_alloc::LlffHeap;
        #[global_allocator]
        static ALLOCATOR: LlffHeap = LlffHeap::empty();
    };
}

#[macro_export]
macro_rules! heap_init {
    [ ] => {
        // Import linker symbols for heap region
        unsafe extern "C" {
            static mut _sdata: u8;
            static mut _edata: u8;
        }

        unsafe {
            let heap_start = core::ptr::addr_of_mut!(_sdata) as usize;
            let heap_end = core::ptr::addr_of_mut!(_edata) as usize;
            let heap_size = heap_end - heap_start;
            ALLOCATOR.init(heap_start, heap_size)
        }
    };
}
