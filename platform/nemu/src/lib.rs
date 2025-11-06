#![no_std]

pub mod preclude;

macros::mod_flat!(stdio, critical_section, heap);

#[unsafe(no_mangle)]
#[unsafe(link_section = ".reset_vector")]
pub unsafe extern "C" fn __start__() -> ! {
    unsafe extern "Rust" {
        unsafe fn main() -> !;
    }

    unsafe { main() }
}

#[cfg(not(test))]
use core::panic::PanicInfo;
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[unsafe(export_name = "main")]
        pub unsafe fn __main() -> ! {
            // type check the given path
            let f: fn() -> ! = $path;

            $crate::heap_init!();
            f()
        }
    };
}

#[macro_export]
macro_rules! binInit {
    [ ] => {
        $crate::preclude!();

        use embedded_alloc::LlffHeap;
        #[global_allocator]
        static ALLOCATOR: LlffHeap = LlffHeap::empty();
    };
}
