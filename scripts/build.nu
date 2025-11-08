source ./ARCHS.nu
source ./utils.nu
use std/log

def build [bin, arch] {
    load-env (prepare_env $arch)
    let target = get_target $arch

    log info $"Building for architecture: ($arch), binary: ($bin), target: ($target)"

    cargo build --bin $bin --target $target --release
}

# Build a binary for all architectures
export def "main build-all-arch" [bin: string] {
    log info $"Building ($bin) for all architectures: ($ARCHS)"
    $ARCHS | each {|arch|
        build $bin $arch
    } | ignore
}

# Build a specific binary for a specific architecture
export def "main build-one" [bin: string, arch: string] {
    build $bin $arch
}

# List all available binaries
export def "main list-bins" [] {
    let bins = (get_allbin)
    log info "Available binaries:"
    $bins | each {|b| print $"  - ($b)"}
}

# Build all binaries for all architectures (default)
export def main [bin?: string, arch?: string] {
    log critical "Starting build process..."
    match $bin {
        null => {
            log critical "Building all binaries for all architectures"
            get_allbin | each {|bin|
                main build-all-arch $bin
            } | ignore
        }
        _ => {
            match $arch {
                null => {
                    main build-all-arch $bin
                }
                _ => {
                    build $bin $arch
                }
            }
        }
    }
    log critical "Build process completed"
}
