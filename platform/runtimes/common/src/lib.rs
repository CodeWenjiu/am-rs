//! Runtime common code shared by all platforms
//!
//! This module contains common macros and startup code that all platforms use.
//! Platforms only need to implement the platform-specific traits.

#![no_std]

macros::mod_flat!(heap, stdio);

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
/// After the user's main returns, it calls the platform-specific exit function.
#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[unsafe(export_name = "main")]
        pub unsafe fn __main() -> ! {
            // Type check the given path
            let f: fn() = $path;

            // Initialize heap
            $crate::heap_init!();

            // Call user's main
            f();

            // Call platform-specific exit function with success code
            unsafe extern "Rust" {
                fn platform_exit(code: i32) -> !;
            }
            unsafe { platform_exit(0) }
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
        use alloc::{boxed::Box, rc::Rc, string::String, vec::Vec};
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
