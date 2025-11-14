source ../arch/utils.nu
source ../utils.nu
use std/log

source ./compilable.nu

def build [bin: string, arch: string] {
    load-env (prepare_env $arch)
    validate_bin $bin

    if (is_compilable $bin $arch) == false {
        log warning $"Skipping build for binary: ($bin) on architecture: ($arch) as it is not compilable."
        return
    }

    let split = arch_split $arch
    let target = $split.target
    let platform = $split.platform
    
    log info $"Building for architecture: ($arch), binary: ($bin), target: ($target)"

    # Build with the appropriate runtime feature based on platform
    # Use --no-default-features to avoid conflict between default and specified features
    cargo build --bin $bin --target $target --release --no-default-features --features $"runtime/($platform)"
}

def build_arch [bin: string, arch: string] {
    match $bin {
        "_ALL" => {
            get_all_bins | each {|bin|
                build $bin $arch
            } | ignore
        }

        _ => {
            build $bin $arch
        }
    }
}

# Build all binaries for all architectures (default)
def main [bin: string, arch: string] {
    match $arch {
        "_ALL" => {
            get_all_archs | each {|arch|
                build_arch $bin $arch
            } | ignore
        }
        _ => {
            build_arch $bin $arch
        }
    }
}
