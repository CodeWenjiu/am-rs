#![cfg_attr(not(test), no_std, no_main)]

use std::io::stdin;

#[cfg(not(test))]
runtime::binInit!();
#[cfg(not(test))]
runtime::entry!(main);

fn main() {
    println!("=== AM-RS Stdin Test ===\n");

    // Test 1: Read a line
    println!("Test 1: Line input");
    println!("Enter your name (press Enter): ");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap_or(0);
    println!("Hello, {}!\n", name);

    println!("\n=== All tests completed! ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_line() {
        let mut name = String::new();
        stdin().read_line(&mut name).unwrap_or(0);
        assert_eq!(name, "John Doe\n");
    }
}
