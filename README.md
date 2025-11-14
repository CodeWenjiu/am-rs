# AM-RS
This project is inspired by [abstract-machine](https://github.com/NJU-ProjectN/abstract-machine) and aims to provide a relatively simple Rust bare-metal programming framework.

# Goals
## ISA Support
- RISC-V (32-bit) - Partially ✓
- x86 - ✗
- arm - ✗

## Abstraction Layers
- stdio - ✓
- Memory Allocator - ✓
- panic - ✓
- Interrupt and Exception Handling - ✗
- RTIC  - ✗
- tock  - ✗

# Usage
## Build
for example, to build `hello` binary for nemu in RISC-V arch `riscv32im`:
```sh
just build hello riscv32im-nemu
```
or build all binaries for qemu in RISC-V arch `riscv32im_zve32x`:
```sh
just build _ALL riscv32im_zve32x-qemu
```

## Disassembly
to generate disassembly and binary for `hello` binary for nemu in RISC-V arch `riscv32im`:
```sh
just disasm hello riscv32im-nemu
```
result will be in `target/disasm/nemu/riscv32im/hello/` folder.

## Run
For emulators that have been released and imported in `flake.nix`, you can run the `run` recipe to directly execute the compiled program.
```sh
just run hello riscv32im-qemu
```

## List All Binaries and Platforms
```sh
just list_bins
just list_archs
```

## List All Just Recipes
```sh
just # or
just --list
```

## Add new Binary
refer to bin/dummy.

## Add new Platform
- add new pla in PLATFORM_CONFIGS within scripts/arch/main.nu
- modify build-helper/src/lib.rs
- modify runtime
- add new platform crate in runtimes
