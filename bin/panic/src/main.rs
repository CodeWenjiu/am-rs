#![no_std]
#![no_main]

runtime::binInit!();

runtime::entry!(main);

fn main() {
    panic!("I'm Panic!!!");
}
