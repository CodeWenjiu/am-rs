# AM-RS
This project is inspired by AbstractMachine and aims to provide a relatively simple Rust bare-metal programming framework.

# Goals
## ISA Support
- RISC-V (32-bit) - Partially Complete
- x86 - ✗
- arm - ✗

## Abstraction Layers
- stdio - ✓
- Memory Allocator - ✓
- panic - ✓
- RTIC  - ✗
- Interrupt and Exception Handling - ✗

# Usage
## Build
for example, to build `hello` binary for nemu in RISC-V arch `riscv32im`:
```nu
just build hello riscv32im-nemu
```
or build all binaries for qemu in RISC-V arch `riscv32im`:
```nu
just build "" riscv32im-qemu
```

## Run
For emulators that have been released and imported in `flake.nix`, you can run the `run` command to directly execute the compiled program.
```nu
just run hello riscv32im-qemu
```

## List Supported Binaries and Platforms
```nu
just list_bins
just list_archs
```

## Add new Binary
refer to bin/dummy.

## Add new Platform
- add new pla in PLATFORM_CONFIGS within scripts/arch/main.nu
- modify build-helper/src/lib.rs
- modify runtime
- add new platform crate in runtimes
