#![no_std]
#![no_main]

use nemu_runtime::{print, println};

nemu_runtime::entry!(main);

fn main() -> ! {
    println!("Hello from Rust no_std!");
    println!("Number: {}", 42);
    print!("Formatted: ");
    println!("0x{:x}", 0xdead_beefu32);
    
    loop {}
}
