//! Spike platform exit implementation
//!
//! This module provides the platform-specific exit function that is called
//! when the user's main function returns.

//! Spike exit via environment call (ecall)
//!
//! When running under the proxy kernel (pk) or an environment that interprets
//! Linux-like syscalls, performing an ecall with a7 = 93 (SYS_exit) and a0 = code
//! terminates the program and reports the exit status.
//!
//! If not on a RISC-V target (e.g. host-side analysis), we provide a dummy
//! implementation that never returns but marks unreachable.

/// Platform-specific exit function
///
/// This function is called when the user's main function returns.
/// For Spike, issue an `ecall` with a7 = 93 (exit) and a0 = exit code.
///
/// # Arguments
/// * `code` - Exit code (0 for success, non-zero for failure)
#[unsafe(no_mangle)]
pub fn platform_exit(code: i32) -> ! {
    // RISC-V implementation: perform ecall SYS_exit (93)
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    unsafe {
        core::arch::asm!(
            "ecall",
            in("a0") code,     // exit status
            in("a7") 93u32,    // SYS_exit (cast to avoid type inference issues)
            options(noreturn)
        );
    }

    // Non-RISC-V fallback (should never be invoked in correct builds)
    #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
    {
        let _ = code;
        unreachable!("platform_exit called on non-RISC-V target. WTF???");
    }
}
