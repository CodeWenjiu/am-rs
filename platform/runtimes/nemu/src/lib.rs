#![no_std]

// Platform-specific modules
pub mod critical_section;
pub mod exit;
pub mod startup;
pub mod stdio;

pub use common::println;

// Generate common startup code
// Platform-specific _start is in startup.rs
common::common_startup!();

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
