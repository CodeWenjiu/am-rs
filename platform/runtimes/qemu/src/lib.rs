#![no_std]

// Platform-specific modules
pub mod critical_section;
pub mod exit;
pub mod startup;
pub mod stdio;

// Re-export stdio items for convenience
pub use stdio::{getc, putc, try_getc};

// Re-export common runtime items from runtime-common
pub use common::{
    Stdin, Stdout, binInit, common_startup, entry, heap_init, preclude, print, println, stdin,
    stdout,
};

// Generate common startup code
// Platform-specific _start is in startup.rs
common_startup!();

// Platform-specific panic handler
#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::exit::platform_exit;
    println!("Panic: {}", info);
    platform_exit(1);
}
