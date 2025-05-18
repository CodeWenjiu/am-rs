#![no_std]
#![no_main]

use riscv32i_runtime::entry;

entry!(main);

static RODATA: &[u8] = b"Hello, world!";
static mut BSS: u8 = 0;
static mut DATA: u16 = 1;

fn main() -> ! {
    let _x = RODATA;
    let _y = &raw const BSS ;
    let _z = &raw const DATA;
    loop {}
}
