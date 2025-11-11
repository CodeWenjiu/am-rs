# ============================================================================
# Helper Commands
# ============================================================================
# This module provides user-facing helper commands for the justfile.
# It acts as a bridge between the justfile and the utility modules.

source ./arch/utils.nu
source ./utils.nu

# ============================================================================
# Binary Queries
# ============================================================================

# List all available binaries
export def "main list-bins" [] {
    get_allbin | to json
}

# ============================================================================
# Architecture Queries
# ============================================================================

# List all available architectures
export def "main list-archs" [] {
    get_all_archs | to json
}

# Show architecture configuration summary
export def "main arch-summary" [] {
    print_arch_summary
}

# Show ISA support matrix
export def "main isa-matrix" [] {
    print_isa_matrix
}

# Show platform capability matrix
export def "main platform-matrix" [] {
    print_platform_matrix
}

# ============================================================================
# Default
# ============================================================================

export def main [] {}
