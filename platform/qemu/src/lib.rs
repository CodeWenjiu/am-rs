#![no_std]

// Platform-specific modules
pub mod critical_section;
pub mod stdio;

// Re-export stdio items for convenience
pub use stdio::putc;

// Re-export common runtime items from runtime-common
pub use runtime_common::{
    Stdout, binInit, entry, heap_init, platform_startup, preclude, print, println, stdout,
};

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
