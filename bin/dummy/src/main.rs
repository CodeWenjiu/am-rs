#![no_std]
#![no_main]

use nemu_runtime::entry;

entry!(main);

fn main() -> ! {
    loop {}
}
