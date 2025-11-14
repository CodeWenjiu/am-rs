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
# - get_target: Get Rust target triple for an architecture
# - prepare_env: Prepare environment variables for building
#
# ============================================================================
# Configuration: Supported ISAs
# ============================================================================

# All ISAs that are known to the build system
const SUPPORTED_ISAS = ["riscv32i", "riscv32im", "riscv32imac", "riscv32im_zve32x"]

# ISA to Rust target mapping rules
# Note: Some ISAs may map to the same target with additional flags
const ISA_TARGET_MAP = {
    "riscv32i": "riscv32i-unknown-none-elf",
    "riscv32im": "riscv32im-unknown-none-elf",
    "riscv32imac": "riscv32imac-unknown-none-elf",
    "riscv32im_zve32x": "riscv32im-unknown-none-elf"
}

# ISAs that require special RUSTFLAGS
const ISA_RUSTFLAGS = {
    "riscv32im_zve32x": "-C target-feature=+zve32x,+zvl64b"
}

# ============================================================================
# Configuration: Platform Constraints
# ============================================================================

# Platform configurations with ISA constraints
# Each platform declares which ISAs it supports
const PLATFORM_CONFIGS = {
    nemu: {
        supported_isas: ["riscv32i", "riscv32im", "riscv32im_zve32x"],
        description: "NEMU RISC-V Emulator"
    },
    qemu: {
        supported_isas: ["riscv32i", "riscv32im", "riscv32imac", "riscv32im_zve32x"],
        description: "QEMU System Emulator"
    }
    spike: {
        supported_isas: ["riscv32i", "riscv32im", "riscv32im_zve32x"],
        description: "Spike RISC-V ISA Simulator"
    }
}

# ============================================================================
# Core Functions
# ============================================================================

export def arch_split [arch: string] {
    let parts = $arch | split row "-"
    if ($parts | length) != 2 {
        error make {
            msg: $"Invalid architecture format: ($arch). Expected format: {{isa}}-{{platform}}"
        }
    }
    let isa = $parts | get 0
    let platform = $parts | get 1
    let target = $ISA_TARGET_MAP | get $isa

    {
        isa: $isa
        platform: $platform
        target: $target
    }
}

# Get Rust target triple for a given architecture
export def get_target [arch: string] {
    validate_arch $arch
    let isa = (arch_split $arch).isa

    if ($isa in $ISA_TARGET_MAP) {
        $ISA_TARGET_MAP | get $isa
    } else {
        error make {
            msg: $"No target mapping found for ISA: ($isa)"
        }
    }
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
    let split = arch_split $arch
    let isa = $split.isa
    let platform = $split.platform

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

# Prepare environment variables for building a specific architecture
export def prepare_env [arch: string] {
    validate_arch $arch

    mut env_vars = {ARCH: $arch}

    let isa = (arch_split $arch).isa

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
