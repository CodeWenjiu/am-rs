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
            static mut _sheap: u8;
            static mut _eheap: u8;
        }

        unsafe {
            let heap_start = core::ptr::addr_of_mut!(_sheap) as usize;
            let heap_end = core::ptr::addr_of_mut!(_eheap) as usize;
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

/// Generate common startup code
///
/// This macro generates the `__start__` entry point that calls main.
/// Each platform must provide its own `_start` function that:
/// 1. Initializes the stack pointer
/// 2. Jumps to `__start__`
#[macro_export]
macro_rules! common_startup {
    () => {
        #[unsafe(link_section = ".text.__start__")]
        #[unsafe(export_name = "__start__")]
        pub unsafe extern "C" fn __start__() -> ! {
            unsafe extern "Rust" {
                unsafe fn main() -> !;
            }

            unsafe { main() }
        }
    };
}

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
