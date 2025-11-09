#![no_std]
#![no_main]

runtime::binInit!();

runtime::entry!(main);

fn main() -> ! {
    println!("Hello_world!");
    loop {}
}
