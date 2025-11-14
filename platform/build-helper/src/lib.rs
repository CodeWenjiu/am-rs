use std::{env, path::Path};

pub enum Platform {
    Nemu,
    Qemu,
    Spike,
}

impl Platform {
    fn fmt(&self) -> &'static str {
        match self {
            Platform::Nemu => "nemu",
            Platform::Qemu => "qemu",
            Platform::Spike => "spike",
        }
    }

    /// Parse platform from string (case-insensitive)
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "nemu" => Some(Platform::Nemu),
            "qemu" => Some(Platform::Qemu),
            "spike" => Some(Platform::Spike),
            _ => None,
        }
    }
}

/// Get platform from environment variable
/// If ARCH is not set (e.g., in rust-analyzer), defaults to "nemu"
fn get_platform() -> Platform {
    let arch = match env::var("ARCH") {
        Ok(arch) => arch,
        Err(_) => {
            // Default to nemu for rust-analyzer and other tools that don't set ARCH
            println!("cargo:warning=ARCH not set, defaulting to nemu (rust-analyzer mode)");
            return Platform::Nemu;
        }
    };

    let platform_env = match arch.split_once('-') {
        Some((_, platform)) => platform,
        None => {
            println!(
                "cargo:warning=Invalid ARCH format '{}', defaulting to nemu",
                arch
            );
            return Platform::Nemu;
        }
    };

    match Platform::from_str(platform_env) {
        Some(platform) => {
            platform
        }
        None => {
            println!(
                "cargo:warning=Unknown platform '{}', defaulting to nemu",
                platform_env
            );
            Platform::Nemu
        }
    }
}

/// Link helper function to be called from build.rs
///
/// This function:
/// 1. Determines the platform from ARCH environment variable
/// 2. Finds the appropriate linker script based on target architecture
/// 3. Configures cargo to use that linker script
pub fn link_helper() {
    // Get platform from environment variable
    let platform = get_platform();

    // Get the target triple
    let target = env::var("TARGET").expect("TARGET environment variable not set");
    if target == "x86_64-unknown-linux-gnu" {
        // Skip linking for host target
        println!("cargo:warning=Host target detected, skipping linker script configuration");
        return;
    }

    // Extract the target prefix (part before the first '-')
    // e.g., "riscv32i-unknown-none-elf" -> "riscv32i"
    let target_prefix = match target.split('-').next() {
        Some(prefix) => prefix,
        None => {
            println!("cargo:warning=Invalid target format, using 'riscv32i' as default");
            "riscv32i"
        }
    };

    // Get the workspace root (go up from bin/xxx to project root)
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let workspace_root = Path::new(std::str::from_utf8(&output).unwrap().trim()).parent().unwrap().to_path_buf();

    // Construct linker script filename based on target prefix
    // e.g., "riscv32i" -> "riscv32i_link.x"
    let linker_script_name = format!("{}_link.x", target_prefix);

    // Path to the linker script in platform/<platform>/linker_scripts
    let linker_script = workspace_root
        .join("platform")
        .join("runtimes")
        .join(platform.fmt())
        .join("linker_scripts")
        .join(&linker_script_name);

    // Check if linker script exists
    if !linker_script.exists() {
        panic!(
            "Linker script not found at: {}\n\
             Platform: {}\n\
             Target: {}\n\
             Expected linker script: {}\n\
             Please ensure the linker script exists before building.",
            linker_script.display(),
            platform.fmt(),
            target,
            linker_script_name
        );
    }

    // Tell cargo to pass the linker script to the linker
    println!("cargo:rustc-link-arg=-T{}", linker_script.display());

    // Rerun if the linker script changes
    println!("cargo:rerun-if-changed={}", linker_script.display());
    println!("cargo:rerun-if-changed=build.rs");

    // Rerun if ARCH environment variable changes
    println!("cargo:rerun-if-env-changed=ARCH");
    println!("cargo:rerun-if-env-changed=TARGET");

    // Print info for debugging
    println!(
        "cargo:warning=Linker config: script={}, platform={}, target={}",
        linker_script_name,
        platform.fmt(),
        target_prefix
    );
}
