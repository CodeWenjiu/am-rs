source ./ARCHS.nu

def main [bin, arch] {
    load-env (prepare_env $arch)
    let target = get_target $arch

    let parts = $arch | split row "-"
    let isa = ($parts | get 0)
    let platform = ($parts | get 1)
    let build_dir = $"build/($platform)/($isa)/($bin)"
    mkdir $build_dir

    print $"Generating disassembly for architecture: ($arch), binary: ($bin), target: ($target)"

    cargo objdump --bin $bin --target $target --release -- -d | save --force $"($build_dir)/image.txt"
    cargo objcopy --bin $bin --target $target --release -- -O binary $"($build_dir)/image.bin"
}
