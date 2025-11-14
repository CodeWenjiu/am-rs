use std::env;

fn main() {
    // List all supported platform features
    // To add a new platform: just add the platform name to this array
    let platforms = ["nemu", "qemu", "spike"];

    // Check which platforms are enabled
    let enabled_platforms: Vec<&str> = platforms
        .iter()
        .filter(|&&platform| env::var(format!("CARGO_FEATURE_{}", platform.to_uppercase())).is_ok())
        .copied()
        .collect();

    // Validate exactly one platform is enabled
    match enabled_platforms.len() {
        0 => {
            let target = env::var("TARGET").expect("TARGET environment variable not set");
            if target != "x86_64-unknown-linux-gnu" {
                eprintln!("ERROR: No platform feature enabled!");
                eprintln!("Available platforms: {}", platforms.join(", "));
                eprintln!("Please enable exactly one platform feature.");
                eprintln!();
                eprintln!("Example: cargo build --features runtime/nemu");
                std::process::exit(1);
            }
        }
        1 => {
            println!("cargo:warning=Using platform: {}", enabled_platforms[0]);
        }
        _ => {
            eprintln!("ERROR: Multiple platform features enabled!");
            eprintln!("Enabled platforms: {}", enabled_platforms.join(", "));
            eprintln!("Available platforms: {}", platforms.join(", "));
            eprintln!("Please enable exactly ONE platform feature.");
            eprintln!();
            eprintln!("Example: cargo build --no-default-features --features runtime/nemu");
            std::process::exit(1);
        }
    }

    // Re-run if any feature changes
    println!("cargo:rerun-if-changed=build.rs");

    // Re-run when platform features change
    // When adding a new platform, add a corresponding line here with the uppercase feature name
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_NEMU");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_QEMU");
    // Example for future platforms:
    // println!("cargo:rerun-if-env-changed=CARGO_FEATURE_FPGA");
    // println!("cargo:rerun-if-env-changed=CARGO_FEATURE_NEXYS");
}
