//! QEMU platform stdio implementation
//!
//! This module provides character I/O functions for the QEMU RISC-V virt machine.
//! It uses the 16550A UART controller at address 0x10000000.

/// UART base address (16550A compatible UART on QEMU virt machine)
const UART_BASE: usize = 0x10000000;

/// UART Line Status Register offset
const UART_LSR: usize = UART_BASE + 5;

/// UART Line Status Register bits
const UART_LSR_THRE: u8 = 0x20; // Transmit Holding Register Empty
const UART_LSR_DR: u8 = 0x01; // Data Ready (receive buffer has data)

/// Write a character to UART
///
/// This function waits until the transmit holding register is empty,
/// then writes the character to the UART.
///
/// # Arguments
/// * `ch` - Character byte to transmit
#[unsafe(no_mangle)]
pub fn putc(ch: u8) {
    unsafe {
        // Wait until transmit holding register is empty
        while (core::ptr::read_volatile(UART_LSR as *const u8) & UART_LSR_THRE) == 0 {}
        // Write character to UART data register
        core::ptr::write_volatile(UART_BASE as *mut u8, ch);
    }
}

/// Read a character from UART (blocking)
///
/// This function waits until data is available in the receive buffer,
/// then reads and returns the character.
///
/// # Returns
/// * The received character byte
///
/// # Safety
/// This function blocks indefinitely if no data arrives.
#[unsafe(no_mangle)]
pub fn getc() -> u8 {
    unsafe {
        // Wait until data is ready (receive buffer has data)
        while (core::ptr::read_volatile(UART_LSR as *const u8) & UART_LSR_DR) == 0 {}
        // Read character from UART data register
        core::ptr::read_volatile(UART_BASE as *const u8)
    }
}

/// Try to read a character from UART (non-blocking)
///
/// This function checks if data is available and returns it if so,
/// otherwise returns None immediately without blocking.
///
/// # Returns
/// * `Some(ch)` - Character byte if data is available
/// * `None` - No data available
#[unsafe(no_mangle)]
pub fn try_getc() -> Option<u8> {
    unsafe {
        // Check if data is ready
        if (core::ptr::read_volatile(UART_LSR as *const u8) & UART_LSR_DR) != 0 {
            // Data available, read it
            Some(core::ptr::read_volatile(UART_BASE as *const u8))
        } else {
            // No data available
            None
        }
    }
}
