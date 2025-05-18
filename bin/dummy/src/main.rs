#![no_std]
#![no_main]

use riscv32i_runtime::entry;

entry!(main);

fn main() -> ! {
    let _x = 42;

    loop {}
}
