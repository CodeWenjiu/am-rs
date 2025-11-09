const SERIAL_PORT: usize = 0x100003f8;

#[unsafe(no_mangle)]
pub fn putc(ch: u8) {
    unsafe {
        core::ptr::write_volatile(SERIAL_PORT as *mut u8, ch);
    }
}

#[unsafe(no_mangle)]
pub fn getc() -> u8 {
    unsafe { core::ptr::read_volatile(SERIAL_PORT as *const u8) }
}

#[unsafe(no_mangle)]
pub fn try_getc() -> Option<u8> {
    Some(getc())
}
