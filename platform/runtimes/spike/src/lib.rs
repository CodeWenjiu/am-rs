#![no_std]

// Platform-specific modules
pub mod critical_section;
pub mod exit;
pub mod startup;
pub mod stdio;

#[unsafe(export_name = "isa_init")]
#[unsafe(link_section = ".text.isa_init")]
pub unsafe extern "C" fn isa_init() -> ! {
    unsafe extern "Rust" {
        fn main() -> !;
    }

    unsafe {
        core::arch::asm!(
            "li x10, 0x200",
            "csrs mstatus, x10",
            options(nomem, nostack, preserves_flags)
        );
        main();
    }
}
