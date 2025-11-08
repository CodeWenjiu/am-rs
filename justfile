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

# Get all the valid ARCH
get_allarch:
    @use scripts/ARCHS.nu get_allarch; get_allarch

# Get all the valid bin
get_allbin:
    @use scripts/utils.nu get_allbin; get_allbin

# Build the project
build bin="" arch="":
    @nu scripts/build.nu {{ bin }} {{ arch }}

# Generate disassembly and binary
disasm bin="" arch="":
    @nu scripts/disasm.nu {{ bin }} {{ arch }}

# Clean build artifacts
clean:
    @cargo clean
