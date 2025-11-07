#![no_std]
#![no_main]

nemu_runtime::binInit!();

nemu_runtime::entry!(main);

fn main() -> ! {
    loop {}
}
