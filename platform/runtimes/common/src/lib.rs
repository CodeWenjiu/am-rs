#![no_std]

macros::mod_flat!(heap);

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() -> ! {
            let f: fn() = $path;

            $crate::heap_init!();

            f();

            unsafe extern "Rust" {
                fn platform_exit(code: i32) -> !;
            }
            unsafe { platform_exit(0) }
        }
    };
}

#[macro_export]
macro_rules! addtest {
    () => {
        #[cfg(test)]
        mod on_test {
            #[test]
            fn main() {
                super::main();
            }
        }
    };
}
