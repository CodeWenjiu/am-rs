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
    @nu scripts/helper.nu list-bins | from json

# List all the valid ARCH
list_archs:
    @nu scripts/helper.nu list-archs | from json

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
build BIN="" ARCH="":
    @nu scripts/build.nu {{ BIN }} {{ ARCH }}

# Quick build/run shortcuts for common scenarios
build-all ARCH="":
    @nu scripts/build.nu "" {{ ARCH }}

# Build specific binary for all architectures
build-bin BIN="":
    @nu scripts/build.nu {{ BIN }} ""

# Generate disassembly and binary
disasm BIN="" ARCH="":
    @nu scripts/disasm.nu {{ BIN }} {{ ARCH }}

# Run the program
run BIN="" ARCH="": (disasm BIN ARCH)
    @nu scripts/run/main.nu {{ BIN }} {{ ARCH }} false

# Run the program in batch mode
batch BIN="" ARCH="": (disasm BIN ARCH)
    @nu scripts/run/main.nu {{ BIN }} {{ ARCH }} true

# Run all binaries for an architecture in batch mode
batch-all ARCH:
    @nu scripts/helper.nu list-bins | from json | each { |bin| just batch $bin {{ARCH}};}

# Clean build artifacts
clean:
    @cargo clean
