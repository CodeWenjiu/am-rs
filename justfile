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
Reinit:
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

# Build the project
build bin="" arch="":
    @nu scripts/build.nu {{ bin }} {{ arch }}

# Generate disassembly and binary
disasm bin="" arch="":
    @nu scripts/disasm.nu {{ bin }} {{ arch }}

# Run the program in QEMU
run bin arch: (disasm bin arch)
    @nu scripts/run/main.nu {{ bin }} {{ arch }}

# Clean build artifacts
clean:
    @cargo clean
