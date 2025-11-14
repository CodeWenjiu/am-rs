#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(unstable_features)]

runtime::binInit!();
runtime::entry!(main);

fn main() {
    println!("=== AM-RS Stdin Test ===\n");

    // Test 1: Single character input (blocking)
    println!("Test 1: Single character input");
    println!("Press any key: ");
    let ch = stdin().getc();
    println!("You pressed: '{}' (0x{:02x})\n", ch as char, ch);

    // Test 2: Non-blocking input with polling
    println!("Test 2: Non-blocking input");
    println!("Press a key within 5000 iterations or wait...");
    let mut found = false;
    for i in 0..5000 {
        if let Some(ch) = stdin().try_getc() {
            println!("\nGot '{}' at iteration {}", ch as char, i);
            found = true;
            break;
        }
        if i % 500 == 0 {
            print!(".");
        }
    }
    if !found {
        println!("\nNo input received (timeout)");
    }
    println!();

    // Test 3: Read a line
    println!("Test 3: Line input");
    println!("Enter your name (press Enter): ");
    let mut name = String::new();
    stdin().read_line(&mut name);
    println!("Hello, {}!\n", name);

    // Test 4: Simple menu
    println!("Test 4: Menu selection");
    loop {
        println!("\n--- Menu ---");
        println!("1. Say Hello");
        println!("2. Count to 5");
        println!("3. Exit");
        println!("Choice: ");

        let choice = stdin().getc();
        println!("{}", choice as char);

        match choice {
            b'1' => {
                println!("Hello from AM-RS!");
            }
            b'2' => {
                for i in 1..=5 {
                    println!("  {}", i);
                }
            }
            b'3' => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice. Try again.");
            }
        }
    }

    // Test 5: Echo loop
    println!("\nTest 5: Echo mode");
    println!("Type characters (press 'q' to quit):");
    loop {
        let ch = stdin().getc();

        if ch == b'q' || ch == b'Q' {
            println!("\nExiting echo mode...");
            break;
        }

        // Echo the character
        print!("{}", ch as char);
    }

    println!("\n=== All tests completed! ===");
}
