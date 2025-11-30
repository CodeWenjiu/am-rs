#![no_std]

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

#[cfg(not(test))]
mod panic_impl {
    use crate::exit::platform_exit;
    use core::panic::PanicInfo;

    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        // Can't print here, as it may cause another panic.
        // Just exit with a non-zero status code.
        platform_exit(1);
    }
}
