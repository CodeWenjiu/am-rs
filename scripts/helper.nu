source ./ARCHS.nu
source ./utils.nu

# List all available binaries
export def "main list-bins" [] {
    get_allbin
}

# List all available binaries
export def "main list-archs" [] {
    $ARCHS
}

export def main [] {}
