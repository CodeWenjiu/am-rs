#![no_std]

// Platform-specific modules
pub mod critical_section;
pub mod stdio;

// Re-export stdio items for convenience
pub use stdio::{Stdout, putc, stdout};

// Note: print! and println! macros are automatically exported due to #[macro_export]

// Import common runtime macros and startup code from runtime-common crate
// These will be available when someone depends on qemu_runtime
pub use runtime_common::{binInit, entry, heap_init, platform_startup, preclude};

// Generate startup code using runtime's macro
platform_startup!();

// Platform-specific panic handler
#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panic: {}", info);
    loop {}
}
