use std::{env, path::PathBuf};

fn main() {
    // Get the workspace root (go up from bin/dummy to project root)
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.parent().unwrap().parent().unwrap();

    // Path to the linker script in platform/nemu
    let linker_script = workspace_root.join("platform").join("nemu").join("link.x");

    // Convert Windows backslashes to forward slashes for the linker
    let linker_script_str = linker_script.display().to_string().replace('\\', "/");

    // Tell cargo to pass the linker script to the linker
    println!("cargo:rustc-link-arg=-T{}", linker_script_str);

    // Rerun if the linker script changes
    println!("cargo:rerun-if-changed={}", linker_script.display());
    println!("cargo:rerun-if-changed=build.rs");

    // Print info for debugging
    println!("cargo:warning=Using linker script: {}", linker_script_str);
}
