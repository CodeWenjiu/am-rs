#![no_std]

pub mod preclude;

macros::mod_flat!(stdio, critical_section, heap);

#[unsafe(link_section = ".text.__start__")]
#[unsafe(export_name = "__start__")]
pub unsafe extern "C" fn __start__() -> ! {
    unsafe extern "Rust" {
        unsafe fn main() -> !;
    }

    unsafe { main() }
}

#[unsafe(export_name = "_start")]
pub unsafe extern "C" fn _start() -> ! {
    unsafe { __start__() }
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
