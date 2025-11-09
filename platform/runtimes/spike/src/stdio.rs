// HTIF host interface symbols (defined in linker script)
#[cfg(any(target_arch = "riscv32"))]
unsafe extern "C" {
    static mut tohost: u32;
    static mut fromhost: u32;
}

#[unsafe(no_mangle)]
pub fn putc(ch: u8) {
    // Bare-metal Spike (HTIF) console output:
    // Encode command: ((ch << 8) | device=1) << 1 | 1
    // We optionally wait for previous command to clear (tohost == 0) to avoid overwriting.
    #[cfg(any(target_arch = "riscv32"))]
    unsafe {
        // Wait until previous host transaction completes (host clears tohost to 0)
        while core::ptr::read_volatile(&raw const tohost) != 0 {}

        let cmd = (((ch as u32) << 8) | 1) << 1 | 1;
        core::ptr::write_volatile(&raw mut tohost, cmd);

        // Optionally read fromhost to allow host responses (ignore value)
        let _ = core::ptr::read_volatile(&raw const fromhost);

        // Optionally could wait for host to consume it again; for simple putc we don't block further.
    }

    // On non-RISC-V (e.g. analysis on host), do nothing.
    #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
    {
        let _ = ch;
    }
}
