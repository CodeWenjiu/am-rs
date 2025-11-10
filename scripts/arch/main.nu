use std/log

# ============================================================================
# Architecture Configuration
# ============================================================================
# This module provides the core configuration for supported ISAs and platforms.
# It contains only essential configuration data and validation functions needed
# by the build system.
#
# For query and utility functions, see arch_utils.nu
#
# ## Configuration Structure
# 1. SUPPORTED_ISAS: All ISAs known to the build system
# 2. ISA_TARGET_MAP: ISA to Rust target triple mappings
# 3. ISA_RUSTFLAGS: Special compiler flags for specific ISAs
# 4. PLATFORM_CONFIGS: Platform definitions with ISA constraints
#
# ## Core Functions
# - validate_arch: Validate an architecture string
# - get_isa: Extract ISA from architecture string
# - get_platform: Extract platform from architecture string
# - get_target: Get Rust target triple for an architecture
# - prepare_env: Prepare environment variables for building
#
# ============================================================================
# Configuration: Supported ISAs
# ============================================================================

# All ISAs that are known to the build system
const SUPPORTED_ISAS = ["riscv32i", "riscv32im", "riscv32imac", "riscv32imv"]

# ISA to Rust target mapping rules
# Note: Some ISAs may map to the same target with additional flags
const ISA_TARGET_MAP = {
    "riscv32i": "riscv32i-unknown-none-elf",
    "riscv32im": "riscv32im-unknown-none-elf",
    "riscv32imac": "riscv32imac-unknown-none-elf",
    "riscv32imv": "riscv32im-unknown-none-elf"  # V extension added via RUSTFLAGS
}

# ISAs that require special RUSTFLAGS
const ISA_RUSTFLAGS = {
    "riscv32imv": "-C target-feature=+f,+zve32x,+zve32f"  # Enable V extension but disable 64-bit subextensions, auto-vectorization, and limit vector bits to 32
}

# ============================================================================
# Configuration: Platform Constraints
# ============================================================================

# Platform configurations with ISA constraints
# Each platform declares which ISAs it supports
const PLATFORM_CONFIGS = {
    nemu: {
        supported_isas: ["riscv32i", "riscv32im", "riscv32imv"],
        description: "NEMU RISC-V Emulator"
    },
    qemu: {
        supported_isas: ["riscv32i", "riscv32im", "riscv32imac", "riscv32imv"],
        description: "QEMU System Emulator"
    }
    spike: {
        supported_isas: ["riscv32i", "riscv32im", "riscv32imv"],
        description: "Spike RISC-V ISA Simulator"
    }
}

# ============================================================================
# Core Functions
# ============================================================================

# Extract ISA from architecture string
# Example: "riscv32i-nemu" -> "riscv32i"
export def get_isa [arch: string] {
    let parts = $arch | split row "-"
    if ($parts | length) < 2 {
        error make {
            msg: $"Invalid architecture format: ($arch). Expected format: {{isa}}-{{platform}}"
        }
    }
    $parts | get 0
}

# Extract platform from architecture string
# Example: "riscv32i-nemu" -> "nemu"
export def get_platform [arch: string] {
    let parts = $arch | split row "-"
    if ($parts | length) < 2 {
        error make {
            msg: $"Invalid architecture format: ($arch). Expected format: {{isa}}-{{platform}}"
        }
    }
    $parts | get 1
}

# Check if a specific ISA-platform combination is valid
export def is_combination_valid [isa: string, platform: string] {
    if not ($platform in ($PLATFORM_CONFIGS | columns)) {
        return false
    }

    let supported_isas = $PLATFORM_CONFIGS | get $platform | get supported_isas
    $isa in $supported_isas
}

# Validate that an architecture string is supported
# This is the main validation function used by the build system
export def validate_arch [arch: string] {
    let isa = get_isa $arch
    let platform = get_platform $arch

    # Check if ISA exists globally
    if not ($isa in $SUPPORTED_ISAS) {
        error make {
            msg: $"Unsupported ISA: ($isa). Supported ISAs: ($SUPPORTED_ISAS | str join ', ')"
        }
    }

    # Check if platform exists
    let platforms = $PLATFORM_CONFIGS | columns
    if not ($platform in $platforms) {
        error make {
            msg: $"Unsupported platform: ($platform). Supported platforms: ($platforms | str join ', ')"
        }
    }

    # Check if this specific combination is valid
    if not (is_combination_valid $isa $platform) {
        let valid_isas = $PLATFORM_CONFIGS | get $platform | get supported_isas
        let platform_desc = $PLATFORM_CONFIGS | get $platform | get description
        error make {
            msg: $"Invalid combination: ($isa)-($platform)\n($platform_desc) does not support ISA '($isa)'.\nSupported ISAs for ($platform): ($valid_isas | str join ', ')"
        }
    }
}

# Get Rust target triple for a given architecture
export def get_target [arch: string] {
    validate_arch $arch
    let isa = get_isa $arch

    if ($isa in $ISA_TARGET_MAP) {
        $ISA_TARGET_MAP | get $isa
    } else {
        error make {
            msg: $"No target mapping found for ISA: ($isa)"
        }
    }
}

# Prepare environment variables for building a specific architecture
export def prepare_env [arch: string] {
    validate_arch $arch

    mut env_vars = {ARCH: $arch}

    let isa = get_isa $arch

    # Add RUSTFLAGS if ISA requires special flags
    if ($isa in $ISA_RUSTFLAGS) {
        let flags = $ISA_RUSTFLAGS | get $isa
        log warning $"FLAGS Prepared for ($isa): ($flags)"
        $env_vars.RUSTFLAGS = $flags
    }

    return $env_vars
}

# ============================================================================
# Internal Data Access (for arch_utils.nu)
# ============================================================================

# Export configuration data for use by utility modules
export def --env get_supported_isas [] {
    $SUPPORTED_ISAS
}

export def --env get_platform_configs [] {
    $PLATFORM_CONFIGS
}
