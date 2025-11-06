use std::{env, path::PathBuf};

pub enum Platform {
    Nemu,
}

impl Platform {
    fn fmt(&self) -> &'static str {
        match self {
            Platform::Nemu => "nemu",
        }
    }

    /// Parse platform from string (case-insensitive)
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "nemu" => Some(Platform::Nemu),
            _ => None,
        }
    }
}

/// Get platform from environment variable or use the provided default
fn get_platform() -> Platform {
    if let Ok(platform_env) = env::var("PLATFORM") {
        match Platform::from_str(&platform_env) {
            Some(platform) => {
                println!("cargo:trace=[build] platform={} (from env)", platform_env);
                platform
            }
            None => {
                panic!(
                    "Invalid PLATFORM environment variable: '{}'\n\
                     Valid values: nemu\n\
                     Or unset PLATFORM to use the default.",
                    platform_env
                );
            }
        }
    } else {
        panic!("PLATFORM environment variable not set");
    }
}

pub fn link_helper() {
    // Get platform from environment variable or use default
    let pla = get_platform();
    // Get the target triple
    let target = env::var("TARGET").expect("TARGET environment variable not set");

    // Extract the target prefix (part before the first '-')
    // e.g., "riscv32i-unknown-none-elf" -> "riscv32i"
    let target_prefix = target
        .split('-')
        .next()
        .expect("Invalid target triple format");

    // Get the workspace root (go up from bin/dummy to project root)
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.parent().unwrap().parent().unwrap();

    // Construct linker script filename based on target prefix
    // e.g., "riscv32i" -> "riscv32i_link.x"
    let linker_script_name = format!("{}_link.x", target_prefix);

    // Path to the linker script in platform/nemu
    let linker_script = workspace_root
        .join("platform")
        .join(pla.fmt())
        .join("linker_scripts")
        .join(&linker_script_name);

    // Check if linker script exists
    if !linker_script.exists() {
        panic!(
            "Linker script not found at: {}\n\
             Target: {}\n\
             Expected linker script: {}\n\
             Please ensure the linker script exists before building.",
            linker_script.display(),
            target,
            linker_script_name
        );
    }

    // Tell cargo to pass the linker script to the linker
    println!("cargo:rustc-link-arg=-T{}", linker_script.display());

    // Rerun if the linker script changes
    println!("cargo:rerun-if-changed={}", linker_script.display());
    println!("cargo:rerun-if-changed=build.rs");

    // Rerun if PLATFORM environment variable changes
    println!("cargo:rerun-if-env-changed=PLATFORM");

    // Print info for debugging
    println!(
        "cargo:trace=[build] linker={} platform={} target={}",
        linker_script_name,
        pla.fmt(),
        target_prefix
    );
}
