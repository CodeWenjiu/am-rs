#![no_std]

// Re-export common runtime code for all platforms
pub use common::*;

// ============================================================================
// Platform Selection
// ============================================================================
// The build.rs script ensures exactly one platform feature is enabled.
// To add a new platform:
// 1. Add it to the platforms array in build.rs
// 2. Add a new feature in Cargo.toml
// 3. Add a new #[cfg(feature = "...")] block below
// 4. Add the corresponding rerun-if-env-changed line in build.rs

#[cfg(feature = "nemu")]
pub use nemu_runtime::*;

#[cfg(feature = "qemu")]
pub use qemu_runtime::*;

#[cfg(feature = "spike")]
pub use spike_runtime::*;

// Future platforms can be added here:
// #[cfg(feature = "new_platform")]
// pub use new_platform_runtime::*;
