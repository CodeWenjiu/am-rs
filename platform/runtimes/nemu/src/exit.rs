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
    // On RISC-V targets: place exit code in a0 then trap with ebreak (never returns)
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    unsafe {
        core::arch::asm!(
            "ebreak",
            in("a0") code,
            options(noreturn)
        );
    }

    // On non-RISC-V targets: this function should never be called
    #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
    {
        let _ = code;
        unreachable!("WTF? platform_exit called on non-RISC-V target???");
    }
}
