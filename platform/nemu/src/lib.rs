#![no_std] // 无标准库

#[unsafe(no_mangle)] // 不进行变量/函数名修饰
#[unsafe(link_section = ".reset_vector")]
pub unsafe extern "C" fn __start__() -> ! {
    unsafe extern "C" {
        unsafe static mut _sbss: u8;
        unsafe static mut _ebss: u8;

        unsafe static mut _sdata: u8;
        unsafe static mut _edata: u8;
        unsafe static _sidata: u8;
    }

    use core::ptr;
    let count = &raw const _ebss as *const u8 as usize - &raw const _sbss as *const u8 as usize;
    unsafe { ptr::write_bytes( &raw mut _sbss as *mut u8, 0, count) };

    let count = &raw const _edata as *const u8 as usize - &raw const _sdata as *const u8 as usize;
    unsafe { ptr::copy_nonoverlapping(&_sidata as *const u8, &raw mut _sdata as *mut u8, count) };

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
