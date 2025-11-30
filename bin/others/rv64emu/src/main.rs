#![cfg_attr(not(test), no_std, no_main)]

#[cfg(not(test))]
runtime::binInit!();
#[cfg(test)]
runtime::addtest!();

use rv64emu::{
    rv64core::{
        bus::Bus,
        cpu_core::{CpuCoreBuild, CpuState},
    },
    rvsim::RVsim,
    tools::rc_refcell_new,
};

use std::{io::stdin, rc::Rc};

// Import modules
mod bin_file;
mod device_setup;

use device_setup::{print_device_map, setup_memory, setup_uart_devices};

// ============================================================================
// Configuration Constants
// ============================================================================

/// Size of emulated DRAM (8 MB default)
/// Increase if you need more memory for your kernel/program
const MEM_SIZE: usize = 8 * 1024 * 1024;

/// Instruction count interval for status printing
/// Set to 0 to disable periodic status updates
const PRINT_INTERVAL: u64 = 1_000_000;

/// CPU configuration parameters
mod cpu_config {
    pub const TLB_SIZE: usize = 16;
    pub const ICACHE_SIZE: usize = 0; // 0 = disabled
    pub const DECODE_CACHE_SIZE: usize = 1024 + 512;
    pub const MMU_TYPE: &str = "sv39"; // Options: sv39, sv48, sv57
    pub const ISA: &str = "rv64imac"; // RISC-V ISA string
    pub const BOOT_PC: u64 = 0x8000_0000;
    pub const HART_ID: usize = 0;
}

/// Device memory map addresses
mod device_addr {
    pub const UART_16550A: u64 = 0x1000_0000;
    pub const UART_16550A_SIZE: u64 = 0x1000;
    pub const SIFIVE_UART: u64 = 0xC000_0000;
    pub const SIFIVE_UART_SIZE: u64 = 0x1000;
}

// ============================================================================
// Main Entry Point
// ============================================================================

fn main() {
    println!("=== rv64emu on AM-RS Platform ===");
    println!("Initializing RISC-V 64-bit emulator...");

    // Create system bus that manages all devices (PLIC, CLINT, DRAM)
    let bus_u = rc_refcell_new(Bus::new());

    // Configure the RISC-V CPU
    let mut config = rv64emu::config::Config::new();
    config.set_tlb_size(cpu_config::TLB_SIZE);
    config.set_icache_size(cpu_config::ICACHE_SIZE);
    config.set_decode_cache_size(cpu_config::DECODE_CACHE_SIZE);
    config.set_mmu_type(cpu_config::MMU_TYPE);
    config.set_isa(cpu_config::ISA);
    let config = Rc::new(config);

    println!("CPU Configuration:");
    println!("  ISA: {}", cpu_config::ISA);
    println!("  MMU: {}", cpu_config::MMU_TYPE);
    println!("  TLB size: {}", cpu_config::TLB_SIZE);
    println!("  I-cache: {} bytes", cpu_config::ICACHE_SIZE);
    println!("  Decode cache: {} entries", cpu_config::DECODE_CACHE_SIZE);

    // Create hart0 (hardware thread) with supervisor mode support
    // - Boot PC: Where kernel starts (configured in cpu_config)
    // - Hart ID: Hardware thread identifier
    // - S-mode: enabled (supervisor mode for running Linux)
    let mut hart0 = CpuCoreBuild::new(bus_u.clone(), config)
        .with_boot_pc(cpu_config::BOOT_PC)
        .with_hart_id(cpu_config::HART_ID)
        .with_smode(true)
        .build();

    println!(
        "Hart {} initialized (boot PC: 0x{:08x})",
        cpu_config::HART_ID,
        cpu_config::BOOT_PC
    );

    // Setup main memory (DRAM)
    println!("Allocating {} KB of DRAM...", MEM_SIZE / 1024);
    let mem_size = setup_memory(&bus_u, MEM_SIZE);
    println!("DRAM mounted (size: {} bytes)", mem_size);

    // Setup UART devices
    let uart_devices = setup_uart_devices(
        &bus_u,
        device_addr::UART_16550A,
        device_addr::UART_16550A_SIZE,
        device_addr::SIFIVE_UART,
        device_addr::SIFIVE_UART_SIZE,
    );

    println!(
        "16550A UART device registered at 0x{:08x}",
        device_addr::UART_16550A
    );
    println!(
        "SiFive UART device registered at 0x{:08x}",
        device_addr::SIFIVE_UART
    );

    // Print device memory map
    print_device_map(&bus_u);

    // Set CPU state to running
    hart0.cpu_state = CpuState::Running;

    // Create simulator with the hart
    let harts_vec = vec![hart0];
    let mut sim = RVsim::new(harts_vec);

    // Load kernel image into memory
    if bin_file::LINUX_FILE.is_empty() {
        println!("\nWarning: No kernel image loaded (bin_file::LINUX_FILE is empty)");
        println!("To load a kernel, update bin_file::LINUX_FILE with your kernel binary");
    } else {
        println!(
            "\nLoading kernel image ({} bytes)...",
            bin_file::LINUX_FILE.len()
        );
        sim.load_image_from_slice(bin_file::LINUX_FILE);
        println!("Kernel loaded successfully");
    }

    println!("\n=== Starting Emulation ===");
    println!("Note: This is a bare-metal simulation loop");
    println!("UART output will be displayed here");
    if PRINT_INTERVAL > 0 {
        println!("Status updates every {} instructions\n", PRINT_INTERVAL);
    } else {
        println!("Status updates disabled\n");
    }

    // Main emulation loop
    let mut instruction_count = 0u64;

    loop {
        // Execute one instruction
        sim.run_once();
        instruction_count += 1;

        // Periodically print status (if enabled)
        if PRINT_INTERVAL > 0 && instruction_count % PRINT_INTERVAL == 0 {
            println!("[INFO] Executed {} instructions", instruction_count);
        }

        // Handle UART output (TX FIFO -> console)
        while let Some(c) = uart_devices.tx_fifo.pop() {
            // Output character from emulated UART to our platform's console
            print!("{c}");
        }

        // Handle UART input (console -> RX FIFO)
        // Use try_getc to check for input without blocking
        if let Some(input_char) = stdin().try_getc() {
            // Push received character to emulated UART's RX FIFO
            uart_devices.rx_fifo.push(input_char);
        }

        // Check if simulation should stop (you can add conditions here)
        // For now, it runs indefinitely
    }
}
