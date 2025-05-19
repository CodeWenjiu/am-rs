use std::{env, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    // build directory for this crate
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // extend the library search path
    println!("cargo:rustc-link-search={}", out_dir.display());

    // build target for this crate 
    // let target = build_target::target_triple().unwrap();

    // put `link.x` in the build directory
    // File::create(out_dir.join(target).join("link.x"))?.write_all(include_bytes!("link.x"))?;

    Ok(())
}
