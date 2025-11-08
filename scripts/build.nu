source ./ARCHS.nu
use std/log

def main [bin, arch] {
    load-env (prepare_env $arch)
    let target = get_target $arch

    log critical $"Building for architecture: ($arch), binary: ($bin), target: ($target)"

    cargo build --bin $bin --target $target --release
}
