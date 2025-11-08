set windows-shell := ["nu", "-c"]
set shell := ["nu", "-c"]

# Available architectures and their target mappings

# Default values
ARCH := "riscv32i-nemu"
BIN := "dummy"

# Default recipe
default:
    just --list

# Bump env dependencies to latest versions
bump-env:
    @nix flake update

# Environment ReInitialization
Reinit:
    @print "ReInitializing development environment..."
    @direnv allow .

# Bump rust dependencies to latest versions
bump-rs:
    @cargo upgrade --incompatible

# Build the project
build bin=BIN arch=ARCH:
    @nu ./scripts/build.nu {{ bin }} {{ arch }}

# Generate disassembly and binary
disasm bin=BIN arch=ARCH:
    @nu ./scripts/disasm.nu {{ bin }} {{ arch }}

# Clean build artifacts
clean:
    @cargo clean
    @rm -rf build
