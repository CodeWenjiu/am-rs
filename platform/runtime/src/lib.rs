#![no_std]

// Re-export common runtime code for all platforms
pub use runtime_common::*;

// Re-export the selected platform runtime based on features
#[cfg(feature = "nemu")]
pub use nemu_runtime::*;

#[cfg(feature = "qemu")]
pub use qemu_runtime::*;

// Re-export platform-specific print macros
// These macros are exported at the crate root by #[macro_export] in platform crates
#[cfg(feature = "nemu")]
pub use nemu_runtime::{print, println};

#[cfg(feature = "qemu")]
pub use qemu_runtime::{print, println};

// Compile-time check to ensure exactly one platform is selected
#[cfg(all(feature = "nemu", feature = "qemu"))]
compile_error!(
    "Cannot enable multiple platform features at once. Please select only one: nemu or qemu"
);

#[cfg(not(any(feature = "nemu", feature = "qemu")))]
compile_error!("No platform feature selected. Please enable one: nemu or qemu");
