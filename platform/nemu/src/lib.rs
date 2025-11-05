#![no_std] 

#[unsafe(no_mangle)] 
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

// 串口输出实现
// NEMU 的串口设备地址 (根据你的平台配置调整)
const SERIAL_PORT: usize = 0xa00003f8;

/// 输出单个字符到串口
#[inline]
pub fn putc(ch: u8) {
    unsafe {
        core::ptr::write_volatile(SERIAL_PORT as *mut u8, ch);
    }
}

// 实现 core::fmt::Write trait
use core::fmt::{self, Write};

struct Writer;

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            putc(byte);
        }
        Ok(())
    }
}

// 实现 print! 和 println! 宏
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    Writer.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    };
}
