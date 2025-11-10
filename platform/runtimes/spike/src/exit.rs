//! Spike platform exit implementation
//!
//! This module provides the platform-specific exit function that is called
//! when the user's main function returns.

//! Spike exit via HTIF (Host-Target Interface)
//!
//! In bare-metal Spike, we use HTIF to communicate exit to the host.
//! Setting tohost = 1 signals the host to terminate the simulation.
//!
//! If not on a RISC-V target (e.g. host-side analysis), we provide a dummy
//! implementation that never returns but marks unreachable.

// HTIF host interface symbols (defined in linker script)
#[cfg(any(target_arch = "riscv32"))]
unsafe extern "C" {
    static mut tohost: u64;
}

/// Platform-specific exit function
///
/// This function is called when the user's main function returns.
/// For Spike, set tohost = 1 to signal exit to the host.
///
/// # Arguments
/// * `code` - Exit code (0 for success, non-zero for failure)
#[unsafe(no_mangle)]
pub fn platform_exit(code: i32) -> ! {
    // RISC-V implementation: use HTIF to exit
    #[cfg(any(target_arch = "riscv32"))]
    unsafe {
        let _ = code;
        // Set tohost = 1 to signal exit (Spike interprets this as exit(0))
        core::ptr::write_volatile(&raw mut tohost, 1u64);
        // Loop forever; Spike will terminate the simulation
        loop {}
    }

    // Non-RISC-V fallback (should never be invoked in correct builds)
    #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
    {
        let _ = code;
        unreachable!("platform_exit called on non-RISC-V target. WTF???");
    }
}
