//! Device setup helper module
//!
//! This module contains helper functions for setting up emulated devices
//! on the system bus, keeping the main function clean and focused.

use alloc::{boxed::Box, rc::Rc, sync::Arc, vec::Vec};
use core::cell::RefCell;
use crossbeam_queue::SegQueue;

use rv64emu::{
    device::{
        device_16550a::Device16550aUART, device_memory::DeviceMemory,
        device_sifive_plic::SIFIVE_UART_IRQ, device_sifive_uart::DeviceSifiveUart,
        device_trait::MEM_BASE,
    },
    rv64core::bus::{Bus, DeviceType},
    tools::fifo_unbounded_new,
};

/// UART device handles for communication with the emulated system
pub struct UartDevices {
    /// Transmit FIFO - data from emulated system to host
    pub tx_fifo: Arc<SegQueue<u8>>,
    /// Receive FIFO - data from host to emulated system
    pub rx_fifo: Arc<SegQueue<u8>>,
}

/// Setup main memory (DRAM) on the bus
///
/// # Arguments
/// * `bus` - System bus to mount the memory on
/// * `size` - Size of memory in bytes
///
/// # Returns
/// * Size of allocated memory in bytes
pub fn setup_memory(bus: &Rc<RefCell<Bus>>, size: usize) -> usize {
    // Allocate memory
    let mut mem_vec = Vec::with_capacity(size);
    mem_vec.resize(size, 0u8);
    let mem = DeviceMemory::from_boxed_slice(mem_vec.into_boxed_slice());
    let mem_size = mem.size();

    // Mount on bus
    bus.borrow_mut().add_device(DeviceType {
        start: MEM_BASE,
        len: mem_size as u64,
        instance: Box::new(mem),
        name: "DRAM",
    });

    mem_size
}

/// Setup UART devices on the bus
///
/// Creates both 16550A and SiFive UART devices that share the same TX/RX FIFOs.
/// Both devices are mounted at their respective memory addresses.
///
/// # Arguments
/// * `bus` - System bus to mount the devices on
/// * `uart_16550a_addr` - Base address for 16550A UART
/// * `uart_16550a_size` - Size of 16550A UART address space
/// * `sifive_uart_addr` - Base address for SiFive UART
/// * `sifive_uart_size` - Size of SiFive UART address space
///
/// # Returns
/// * `UartDevices` - Handle containing TX and RX FIFOs for host communication
pub fn setup_uart_devices(
    bus: &Rc<RefCell<Bus>>,
    uart_16550a_addr: u64,
    uart_16550a_size: u64,
    sifive_uart_addr: u64,
    sifive_uart_size: u64,
) -> UartDevices {
    // Create shared FIFOs for UART communication
    let uart_tx_fifo = fifo_unbounded_new::<u8>();
    let uart_rx_fifo = fifo_unbounded_new::<u8>();

    // Setup 16550A UART device
    let device_16550_uart = Device16550aUART::new(uart_tx_fifo.clone(), uart_rx_fifo.clone());

    bus.borrow_mut().add_device(DeviceType {
        start: uart_16550a_addr,
        len: uart_16550a_size,
        instance: Box::new(device_16550_uart),
        name: "16550a_uart",
    });

    // Setup SiFive UART device
    let device_sifive_uart = DeviceSifiveUart::new(uart_tx_fifo.clone(), uart_rx_fifo.clone());

    // Register UART interrupt with PLIC
    bus.borrow_mut()
        .plic
        .instance
        .register_irq_source(SIFIVE_UART_IRQ, device_sifive_uart.irq_pending.clone());

    bus.borrow_mut().add_device(DeviceType {
        start: sifive_uart_addr,
        len: sifive_uart_size,
        instance: Box::new(device_sifive_uart),
        name: "sifive_uart",
    });

    UartDevices {
        tx_fifo: uart_tx_fifo,
        rx_fifo: uart_rx_fifo,
    }
}

/// Print device memory map information
///
/// # Arguments
/// * `bus` - System bus to display
pub fn print_device_map(bus: &Rc<RefCell<Bus>>) {
    runtime::println!("\n=== Device Memory Map ===");
    runtime::println!("{}", bus.borrow());
}
