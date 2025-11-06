set shell := ["nu", "-c"]

# Available architectures and their target mappings

ARCHS := "riscv32i-nemu riscv32im-nemu"
ARCH_TARGETS := '{"riscv32i-nemu": "riscv32i-unknown-none-elf", "riscv32im-nemu": "riscv32im-unknown-none-elf"}'

# Default values

ARCH := env_var_or_default("ARCH", "riscv32i-nemu")
BIN := env_var_or_default("BIN", "dummy")

# Default recipe
default:
    just --list

# Validate architecture helper
_validate-arch arch:
    @let archs = "{{ ARCHS }}" | split row " "; if not ($archs | any {|x| $x == "{{ arch }}"}) { print $"Error: Expected ARCH in ($archs), Got \"{{ arch }}\""; exit 1 }

# Get target for architecture
_get-target arch:
    '{{ ARCH_TARGETS }}' | from json | get "{{ arch }}"

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
build arch=ARCH bin=BIN:
    @just _validate-arch {{ arch }}
    @nu -c 'let arch_targets = "{{ ARCH_TARGETS }}" | from json; \
        let target = ($arch_targets | get {{ arch }}); \
        print $"Building for architecture: {{ arch }}, binary: {{ bin }}, target: ($target)"; \
        let parts = ("{{ arch }}" | split row "-"); \
        let platform = ($parts | get 1); \
        $env.PLATFORM = ($platform); \
        cargo build --bin {{ bin }} --target $target --release \
    '

# Generate disassembly and binary
disasm arch=ARCH bin=BIN:
    @just _validate-arch {{ arch }}
    @nu -c 'let arch_targets = "{{ ARCH_TARGETS }}" | from json; \
        let target = ($arch_targets | get {{ arch }}); \
        let parts = ("{{ arch }}" | split row "-"); \
        let isa = ($parts | get 0); \
        let platform = ($parts | get 1); \
        $env.PLATFORM = ($platform); \
        let build_dir = $"build/($platform)/($isa)/{{ bin }}"; \
        print $"Generating disassembly for architecture: {{ arch }}, binary: {{ bin }}"; \
        mkdir $build_dir; \
        cargo objdump --bin {{ bin }} --target $target --release -- -d | save --force $"($build_dir)/image.txt"; \
        cargo objcopy --bin {{ bin }} --target $target --release -- -O binary $"($build_dir)/image.bin" \
    '

# Clean build artifacts
clean:
    @cargo clean
    @rm -rf build
