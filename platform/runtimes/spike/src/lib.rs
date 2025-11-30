#![no_std]

// Platform-specific modules
pub mod critical_section;
pub mod exit;
pub mod startup;
pub mod stdio;

#[unsafe(export_name = "__start__")]
#[unsafe(link_section = ".text.__start__")]
pub unsafe extern "C" fn __start__() -> ! {
    unsafe extern "Rust" {
        fn main() -> !;
    }
    unsafe {
        main();
    }
}

// Platform-specific panic handler
#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    use crate::exit::platform_exit;
    // println!("Panic: {}", info);
    platform_exit(1);
}
