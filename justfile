set shell := ["nu", "-c"]

# Available architectures and their target mappings

# Default recipe
_default:
    @just --list

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
build BIN="_ALL" ARCH="_ALL":
    @nu scripts/build/main.nu {{ BIN }} {{ ARCH }}

# Generate disassembly and binary
disasm BIN="_ALL" ARCH="_ALL":
    @nu scripts/build/disasm.nu {{ BIN }} {{ ARCH }}

# Run the program
run BIN ARCH:
    @nu scripts/run/main.nu run {{ BIN }} {{ ARCH }}

# Run the program natively
native-run BIN TARGET="main":
    @nu scripts/run/main.nu native run {{ BIN }} {{ TARGET }}

# Run the program in batch mode
test BIN="_ALL" ARCH="_ALL":
    @nu scripts/run/main.nu test {{ BIN }} {{ ARCH }}

# Clean build artifacts
clean:
    @cargo clean
