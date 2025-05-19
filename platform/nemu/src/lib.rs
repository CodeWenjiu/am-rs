#![no_std] // 无标准库

#[unsafe(no_mangle)] // 不进行变量/函数名修饰
#[unsafe(link_section = ".reset_vector")]
pub unsafe extern "C" fn __start__() -> ! {

    unsafe extern "Rust" {
        unsafe fn main() -> !;
    }

    unsafe {main()}
}

use core::panic::PanicInfo;
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

            f()
        }
    }
}
