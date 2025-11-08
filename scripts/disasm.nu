source ./ARCHS.nu
use std/log

def main [bin, arch] {
    load-env (prepare_env $arch)
    let target = get_target $arch

    let parts = $arch | split row "-"
    let isa = ($parts | get 0)
    let platform = ($parts | get 1)
    let disasm_dir = $"target/disasm/($platform)/($isa)/($bin)"
    mkdir $disasm_dir

    log critical $"Generating disassembly for architecture: ($arch), binary: ($bin), target: ($target)"

    cargo objdump --bin $bin --target $target --release -- -d | save --force $"($disasm_dir)/image.txt"
    cargo objcopy --bin $bin --target $target --release -- -O binary $"($disasm_dir)/image.bin"
}
