//! Runtime common code shared by all platforms
//!
//! This module contains common macros and startup code that all platforms use.
//! Platforms only need to implement the platform-specific traits.

#![no_std]

/// Common binInit macro for all platforms
///
/// This macro sets up the allocator and imports common preludes.
/// It should be called at the beginning of every binary.
#[macro_export]
macro_rules! binInit {
    () => {
        // Import platform-specific prelude
        $crate::preclude!();

        // Setup global allocator
        use embedded_alloc::LlffHeap;
        #[global_allocator]
        static ALLOCATOR: LlffHeap = LlffHeap::empty();
    };
}

/// Entry point macro for main function
///
/// This macro wraps the user's main function and performs initialization.
#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[unsafe(export_name = "main")]
        pub unsafe fn __main() -> ! {
            // Type check the given path
            let f: fn() -> ! = $path;

            // Initialize heap
            $crate::heap_init!();

            // Call user's main
            f()
        }
    };
}

/// Heap initialization macro
///
/// Initializes the heap allocator using linker symbols.
#[macro_export]
macro_rules! heap_init {
    () => {
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

/// Platform-specific prelude macro
///
/// Each platform should define this macro to import platform-specific items.
#[macro_export]
macro_rules! preclude {
    () => {
        extern crate alloc;

        // Import print macros from runtime crate
        use runtime::{print, println};

        // Import common alloc types
        use alloc::{boxed::Box, string::String, vec::Vec};
    };
}

/// Generate startup code
///
/// This macro generates the `_start` and `__start__` entry points.
#[macro_export]
macro_rules! platform_startup {
    () => {
        #[unsafe(link_section = ".text.__start__")]
        #[unsafe(export_name = "__start__")]
        pub unsafe extern "C" fn __start__() -> ! {
            unsafe extern "Rust" {
                unsafe fn main() -> !;
            }

            unsafe { main() }
        }

        #[unsafe(export_name = "_start")]
        pub unsafe extern "C" fn _start() -> ! {
            unsafe { __start__() }
        }
    };
}
