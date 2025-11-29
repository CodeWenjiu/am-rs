#![cfg_attr(not(test), no_std, no_main)]

runtime::binInit!();

fn main() {
    println!("Hello from Rust no_std!");

    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("Vec: {:?}", v.as_slice());

    let boxed = Box::new(42);
    println!("Boxed value: {}", *boxed);

    let mut s = String::from("Hello, ");
    s.push_str("heap allocation!");
    println!("{}", s);

    println!("All tests passed!");
}
