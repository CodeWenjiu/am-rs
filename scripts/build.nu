source ./ARCHS.nu

def main [bin, arch] {
    load-env (prepare_env $arch)
    let target = get_target $arch

    print $"Building for architecture: ($arch), binary: ($bin), target: ($target)"

    cargo build --bin $bin --target $target --release
}
