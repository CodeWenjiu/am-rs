//! QEMU platform exit implementation
//!
//! This module provides the platform-specific exit function that is called
//! when the user's main function returns.

/// QEMU exit using SiFive Test Device
///
/// QEMU's virt machine provides a test device at 0x100000 that can be used
/// to exit QEMU with a specific exit code.
///
/// Write 0x5555 to exit with success (exit code 0)
/// Write 0x3333 to exit with failure (exit code 1)
const QEMU_TEST_DEVICE: usize = 0x100000;
const QEMU_EXIT_SUCCESS: u32 = 0x5555;
const QEMU_EXIT_FAILURE: u32 = 0x3333;

/// Platform-specific exit function
///
/// This function is called when the user's main function returns.
/// For QEMU, we write to the test device to trigger a clean shutdown.
///
/// # Arguments
/// * `code` - Exit code (0 for success, non-zero for failure)
#[unsafe(no_mangle)]
pub fn platform_exit(code: i32) -> ! {
    unsafe {
        // Map exit code to QEMU test device value
        // 0 -> 0x5555 (success), non-zero -> 0x3333 (failure)
        let exit_value = if code == 0 {
            QEMU_EXIT_SUCCESS
        } else {
            QEMU_EXIT_FAILURE
        };

        // Write to QEMU test device to trigger exit
        core::ptr::write_volatile(QEMU_TEST_DEVICE as *mut u32, exit_value);
    }

    // If the test device write didn't work, fall back to infinite loop
    loop {
        core::hint::spin_loop();
    }
}
