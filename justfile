set windows-shell := ["nu", "-c"]
set shell := ["nu", "-c"]

# Available architectures and their target mappings

# Default recipe
_default:
    just --list

# Bump env dependencies to latest versions
bump-env:
    @nix flake update

# Environment ReInitialization
reinit:
    @print "ReInitializing development environment..."
    @direnv allow .

# Bump rust dependencies to latest versions
bump-rs:
    @cargo upgrade --incompatible

# List all the valid bin
list_bins:
    @nu scripts/helper.nu list-bins

# List all the valid ARCH
list_archs:
    @nu scripts/helper.nu list-archs

# Show architecture configuration summary
arch_summary:
    @nu scripts/helper.nu arch-summary

# Show ISA support matrix
isa_matrix:
    @nu scripts/helper.nu isa-matrix

# Show platform capability matrix
platform_matrix:
    @nu scripts/helper.nu platform-matrix

# Build the project
# Usage: just build [BIN] [ARCH]
#   or:  BIN=hello ARCH=riscv32i-qemu just build
build BIN="" ARCH="":
    @nu scripts/build.nu {{ BIN }} {{ ARCH }}

# Generate disassembly and binary
# Usage: just disasm [BIN] [ARCH]
#   or:  BIN=hello ARCH=riscv32i-qemu just disasm
disasm BIN="" ARCH="":
    @nu scripts/disasm.nu {{ BIN }} {{ ARCH }}

# Run the program
# Usage: just run [BIN] [ARCH]
#   or:  BIN=hello ARCH=riscv32i-qemu just run
run BIN="" ARCH="": (disasm BIN ARCH)
    @nu scripts/run/main.nu {{ BIN }} {{ ARCH }}

# Clean build artifacts
clean:
    @cargo clean

# Quick build/run shortcuts for common scenarios
# Build all binaries for a specific architecture
build-all ARCH="":
    @nu scripts/build.nu "" {{ ARCH }}

# Build specific binary for all architectures
build-bin BIN="":
    @nu scripts/build.nu {{ BIN }} ""

# Quick run for QEMU (default to riscv32i)
qemu BIN="" ISA="riscv32i":
    just disasm {{ BIN }} {{ ISA }}-qemu
    @nu scripts/run/main.nu {{ BIN }} {{ ISA }}-qemu

# Quick run for NEMU (default to riscv32i)
nemu BIN="" ISA="riscv32i":
    just disasm {{ BIN }} {{ ISA }}-nemu
    @nu scripts/run/main.nu {{ BIN }} {{ ISA }}-nemu

# Run on all QEMU platforms
qemu-all BIN="":
    just run {{ BIN }} riscv32i-qemu
    just run {{ BIN }} riscv32im-qemu
    just run {{ BIN }} riscv32imv-qemu

# Run on all NEMU platforms
nemu-all BIN="":
    just run {{ BIN }} riscv32i-nemu
    just run {{ BIN }} riscv32im-nemu
    just run {{ BIN }} riscv32imv-nemu
