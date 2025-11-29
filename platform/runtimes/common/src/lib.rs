//! Runtime common code shared by all platforms
//!
//! This module contains common macros and startup code that all platforms use.
//! Platforms only need to implement the platform-specific traits.

#![no_std]

macros::mod_pub!(io);
macros::mod_flat!(heap, r#macro);

#[macro_export]
macro_rules! libInit {
    () => {
        // Import platform-specific prelude
        $crate::preclude!();
    };
}

/// Common binInit macro for all platforms
///
/// This macro sets up the allocator and imports common preludes.
/// It should be called at the beginning of every binary.
#[macro_export]
macro_rules! binInit {
    () => {
        #[cfg(not(test))]
        mod on_not_test {
            // Import platform-specific prelude
            $crate::preclude!();

            // Setup global allocator
            use embedded_alloc::LlffHeap;
            #[global_allocator]
            static ALLOCATOR: LlffHeap = LlffHeap::empty();

            $crate::entry!(main);
        }

        $crate::addtest!(main);
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

            // If the binary was compiled with the RISC-V Vector extension enabled
            // (Rust target feature `v`), make sure the CPU vector state is enabled
            // before any vector instructions are executed. Some toolchains (LLVM)
            // may emit vector instructions that assume the V-extension state is
            // already set up; enabling it here avoids illegal-instruction traps
            // at startup on simulators/targets that require explicit state setup.
            // #[cfg(target_feature = "v")]
            // {
            // This sequence enables the relevant mstatus bits. The exact
            // bits set here mirror the small startup sequence used elsewhere
            // in the tree (and in upstream tests). Keep it minimal and
            // restricted to the `v`-enabled builds only.
            core::arch::asm!(
                "li x10, 0x200",
                "csrs mstatus, x10",
                options(nomem, nostack, preserves_flags)
            );
            // }

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

#[macro_export]
macro_rules! addtest {
    ($path:path) => {
        #[cfg(test)]
        mod on_test {
            use super::{$path as test_fn};

            #[test]
            fn main() {
                test_fn();
            }
        }
    };
}

/// Platform-specific prelude macro
///
/// Each platform should define this macro to import platform-specific items.
#[macro_export]
macro_rules! preclude {
    () => {
        // Import print macros from runtime crate
        use runtime::{self as std, print, println};

        // Import common alloc types
        extern crate alloc;
        use alloc::{boxed::Box, rc::Rc, string::String, vec, vec::Vec};
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
