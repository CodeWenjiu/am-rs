//! NEMU platform startup code
//!
//! This module provides the `_start` entry point that initializes
//! the stack pointer and jumps to the common `__start__` function.

/// Platform-specific startup code
///
/// This function is the first code that runs on the CPU.
/// It initializes the stack pointer using the linker symbol `_stack_top`
/// and then jumps to `__start__`.
#[unsafe(link_section = ".text._start")]
#[unsafe(export_name = "_start")]
#[unsafe(naked)]
pub unsafe extern "C" fn _start() -> ! {
    core::arch::naked_asm!(
        "
        # Load stack pointer from linker symbol
        # Stack grows downward, so we set sp to the top of RAM
        la sp, _stack_top

        # Jump to common startup code
        j isa_init
        "
    )
}
