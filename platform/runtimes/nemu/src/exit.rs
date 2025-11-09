//! NEMU platform exit implementation
//!
//! This module provides the platform-specific exit function that is called
//! when the user's main function returns.

/// Platform-specific exit function
///
/// This function is called when the user's main function returns.
/// For NEMU, we execute an ebreak instruction to halt the simulator.
#[unsafe(no_mangle)]
pub fn platform_exit(code: i32) -> ! {
    unsafe {
        // Put exit code in a0 register before ebreak
        // NEMU will read a0 to get the exit code
        // Use out("a0") to directly place the value in a0 register
        core::arch::asm!(
            "ebreak",
            in("a0") code,
            options(noreturn)
        );
    }
}
