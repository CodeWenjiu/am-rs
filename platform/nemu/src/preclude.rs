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
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { ALLOCATOR.init(core::ptr::addr_of_mut!(HEAP) as usize, HEAP_SIZE) }
    };
}
