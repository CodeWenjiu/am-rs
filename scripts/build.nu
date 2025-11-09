source ./arch/utils.nu
source ./utils.nu
use std/log

def build [bin, arch] {
    load-env (prepare_env $arch)
    let target = get_target $arch
    let platform = get_platform $arch

    log info $"Building for architecture: ($arch), binary: ($bin), target: ($target)"

    # Build with the appropriate runtime feature based on platform
    # Use --no-default-features to avoid conflict between default and specified features
    cargo build --bin $bin --target $target --release --no-default-features --features $"runtime/($platform)"
}

# Build a binary for all architectures
def "main build-all-arch" [bin: string] {
    let archs = get_all_archs
    log info $"Building ($bin) for all architectures: ($archs)"
    $archs | par-each {|arch|
        build $bin $arch
    } | ignore
}

# Build a specific binary for a specific architecture
def "main build-one" [bin: string, arch: string] {
    build $bin $arch
}

# Build all binaries for all architectures (default)
def main [bin?: string, arch?: string] {
    log critical "Starting build process..."
    match $bin {
        null => {
            log critical "Building all binaries for all architectures"
            get_allbin | par-each {|bin|
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
