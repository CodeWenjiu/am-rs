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
