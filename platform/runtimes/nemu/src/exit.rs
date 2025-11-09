//! NEMU platform exit implementation
//!
//! This module provides the platform-specific exit function that is called
//! when the user's main function returns.

/// Platform-specific exit function
///
/// This function is called when the user's main function returns.
/// For NEMU, we execute an ebreak instruction to halt the simulator.
#[unsafe(no_mangle)]
pub fn platform_exit() -> ! {
    unsafe {
        // Execute ebreak instruction to halt NEMU
        core::arch::asm!("ebreak", options(noreturn));
    }
}
