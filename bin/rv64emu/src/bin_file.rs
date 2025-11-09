//! Binary file module for rv64emu
//!
//! This module contains the kernel image binary data that will be loaded
//! into the emulator's memory and executed.
//!
//! ## Usage
//!
//! To include a kernel image, you can use one of these approaches:
//!
//! ### Option 1: Include binary file at compile time
//! ```rust,ignore
//! pub const LINUX_FILE: &[u8] = include_bytes!("../../../resources/linux.bin");
//! ```
//!
//! ### Option 2: Use a pre-defined binary array
//! ```rust,ignore
//! pub const LINUX_FILE: &[u8] = &[
//!     0x97, 0x02, 0x00, 0x00, // Binary data here...
//! ];
//! ```
//!
//! ### Option 3: Empty placeholder (current - no kernel loaded)
//! The emulator will start but won't execute anything meaningful.

/// Kernel image binary data
///
/// Currently empty - replace with your actual kernel image.
///
/// To add a kernel image:
/// 1. Place your kernel binary (e.g., Linux kernel, bare-metal program) in a resources directory
/// 2. Use `include_bytes!()` macro to embed it at compile time
/// 3. Example: `pub const LINUX_FILE: &[u8] = include_bytes!("../../../resources/linux.bin");`
pub const LINUX_FILE: &[u8] = &[];

// Uncomment one of the following examples based on your needs:

// Example 1: Load from file (recommended)
// Create a `resources` directory at the project root and place your kernel there
// pub const LINUX_FILE: &[u8] = include_bytes!("../../../resources/linux.bin");

// Example 2: Simple test program (32-bit RISC-V infinite loop)
// This is a minimal program that does nothing but loop
// pub const LINUX_FILE: &[u8] = &[
//     0x13, 0x00, 0x00, 0x00,  // nop
//     0x6f, 0xf0, 0x9f, 0xff,  // j -8 (infinite loop)
// ];

// Example 3: Hello world test program (if you have a simple RISC-V binary)
// pub const LINUX_FILE: &[u8] = include_bytes!("../test_programs/hello.bin");
