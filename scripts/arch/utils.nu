# ============================================================================
# Architecture Utilities
# ============================================================================
# This module provides utility and query functions for architecture management.
# It builds on top of the core ARCHS.nu module to provide developer-friendly
# tools and information queries.
#
# ## Functions Overview
# - Query functions: get_all_archs, get_platform_isas, get_isa_archs, etc.
# - Check functions: is_isa_supported, is_platform_supported, is_combination_valid
# - Info functions: get_platform_description, print_arch_summary
#
# ============================================================================

source ./main.nu

# ============================================================================
# Platform Queries
# ============================================================================

# Get all platform names
export def get_supported_platforms [] {
    get_platform_configs | columns
}

# Check if a platform is supported
export def is_platform_supported [platform: string] {
    $platform in (get_supported_platforms)
}

# Get ISAs supported by a specific platform
export def get_platform_isas [platform: string] {
    if not (is_platform_supported $platform) {
        error make {
            msg: $"Unknown platform: ($platform). Supported platforms: (get_supported_platforms | str join ', ')"
        }
    }

    get_platform_configs | get $platform | get supported_isas
}

# Get platform description
export def get_platform_description [platform: string] {
    if not (is_platform_supported $platform) {
        error make {
            msg: $"Unknown platform: ($platform). Supported platforms: (get_supported_platforms | str join ', ')"
        }
    }

    get_platform_configs | get $platform | get description
}

# Get all architectures for a specific platform
# Example: get_platform_archs "qemu" -> ["riscv32i-qemu", "riscv32im-qemu", ...]
export def get_platform_archs [platform: string] {
    get_platform_isas $platform | each {|isa|
        $"($isa)-($platform)"
    }
}

# ============================================================================
# ISA Queries
# ============================================================================

# Check if an ISA is supported globally
export def is_isa_supported [isa: string] {
    $isa in (get_supported_isas)
}

# Get all architectures for a specific ISA
# Example: get_isa_archs "riscv32i" -> ["riscv32i-nemu", "riscv32i-qemu"]
export def get_isa_archs [isa: string] {
    if not (is_isa_supported $isa) {
        error make {
            msg: $"Unsupported ISA: ($isa). Supported ISAs: (get_supported_isas | str join ', ')"
        }
    }

    get_platform_configs | items {|platform, config|
        if ($isa in $config.supported_isas) {
            $"($isa)-($platform)"
        } else {
            null
        }
    } | where {|x| $x != null}
}

# ============================================================================
# Architecture Combination Queries
# ============================================================================

# Generate all valid architecture combinations based on platform constraints
# Format: "{isa}-{platform}"
# Example: ["riscv32i-nemu", "riscv32i-qemu", "riscv32im-nemu", ...]
export def get_all_archs [] {
    get_platform_configs | items {|platform, config|
        $config.supported_isas | each {|isa|
            $"($isa)-($platform)"
        }
    } | flatten
}

# ============================================================================
# Information and Summary
# ============================================================================

# Print a comprehensive summary of the architecture configuration
export def print_arch_summary [] {
    print "=== Architecture Configuration Summary ===\n"

    let isas = get_supported_isas
    let platforms = get_supported_platforms
    let archs = get_all_archs

    print $"Total ISAs: ($isas | length)"
    print $"Total Platforms: ($platforms | length)"
    print $"Valid Combinations: ($archs | length)\n"

    print "Platform Support Matrix:"
    get_platform_configs | items {|platform, config|
        let desc = $config.description
        print $"  ($platform): ($desc)"
        print $"    Supported ISAs: ($config.supported_isas)"
    }

    print "\nAll Valid Architectures:"
    print $archs
}

# Print ISA support matrix (which platforms support each ISA)
export def print_isa_matrix [] {
    print "=== ISA Support Matrix ===\n"

    get_supported_isas | each {|isa|
        let platforms = get_isa_archs $isa | each {|arch|
            (arch_split $arch).platform
        }

        print $"($isa):"
        print $"  Supported by: ($platforms)\n"
    }
}

# Print platform capability matrix
export def print_platform_matrix [] {
    print "=== Platform Capability Matrix ===\n"

    get_platform_configs | items {|platform, config|
        print $"($platform) - ($config.description)"
        print $"  Architectures: (get_platform_archs $platform | length)"
        print $"  Supported ISAs:"
        print $config.supported_isas
        print ""
    }
}
