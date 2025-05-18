#![no_main] // 不再以main函数作为入口
#![no_std] // 无标准库

#[unsafe(no_mangle)] // 不进行变量/函数名修饰
pub unsafe extern "C" fn __start__() -> ! {
    main()
}

// #[unsafe(link_section = ".vector_table.reset_vector")]
// #[unsafe(no_mangle)]
// pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = __start__;

fn main() -> !  {
    loop {}
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
