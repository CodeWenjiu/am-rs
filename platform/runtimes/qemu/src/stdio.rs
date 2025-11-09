const UART_BASE: usize = 0x10000000;
const UART_LSR: usize = UART_BASE + 5;
const UART_LSR_THRE: u8 = 0x20;

#[unsafe(no_mangle)]
pub fn putc(ch: u8) {
    unsafe {
        while (core::ptr::read_volatile(UART_LSR as *const u8) & UART_LSR_THRE) == 0 {}
        core::ptr::write_volatile(UART_BASE as *mut u8, ch);
    }
}
