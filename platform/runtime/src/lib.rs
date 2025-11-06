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
}

pub fn link_helper(pla: Platform) {
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

    // Print info for debugging
    println!(
        "cargo:info=Using linker script: {} for target: {}",
        linker_script.display(),
        target
    );
}
