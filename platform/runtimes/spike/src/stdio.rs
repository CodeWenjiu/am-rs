// HTIF host interface symbols (defined in linker script)
#[cfg(any(target_arch = "riscv32"))]
unsafe extern "C" {
    static mut tohost: u64;
    static mut fromhost: u64;
}

#[unsafe(no_mangle)]
pub fn putc(ch: u8) {
    // Bare-metal Spike (HTIF) console output:
    // HTIF command: device=1 (console), cmd=1 (write), data=ch
    // tohost = (1 << 56) | (1 << 48) | ch
    #[cfg(any(target_arch = "riscv32"))]
    unsafe {
        // Wait until previous host transaction completes (host clears tohost to 0)
        while core::ptr::read_volatile(&raw const tohost) != 0 {}

        let cmd = (1u64 << 56) | (1u64 << 48) | (ch as u64);
        core::ptr::write_volatile(&raw mut tohost, cmd);

        // Wait for command to complete
        while core::ptr::read_volatile(&raw const tohost) != 0 {}

        // Read fromhost to acknowledge
        let _ = core::ptr::read_volatile(&raw const fromhost);
    }

    // On non-RISC-V (e.g. analysis on host), do nothing.
    #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
    {
        let _ = ch;
    }
}

#[unsafe(no_mangle)]
pub fn getc() -> u8 {
    // Bare-metal Spike (HTIF) console input:
    // HTIF command: device=1 (console), cmd=0 (read), data=0
    // tohost = (1 << 56) | (0 << 48) | 0
    #[cfg(any(target_arch = "riscv32"))]
    unsafe {
        // Wait until previous host transaction completes (host clears tohost to 0)
        while core::ptr::read_volatile(&raw const tohost) != 0 {}

        let cmd = (1u64 << 56) | (0u64 << 48) | 0u64;
        core::ptr::write_volatile(&raw mut tohost, cmd);

        // Wait for command to complete
        while core::ptr::read_volatile(&raw const tohost) != 0 {}

        // Read the character from fromhost
        let result = core::ptr::read_volatile(&raw const fromhost);
        (result & 0xff) as u8
    }

    // On non-RISC-V (e.g. analysis on host), do nothing.
    #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
    {
        0
    }
}

#[unsafe(no_mangle)]
pub fn try_getc() -> Option<u8> {
    // Spike HTIF does not support non-blocking input easily
    // For simplicity, always return None
    #[cfg(any(target_arch = "riscv32"))]
    {
        None
    }

    // On non-RISC-V (e.g. analysis on host), do nothing.
    #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
    {
        None
    }
}
