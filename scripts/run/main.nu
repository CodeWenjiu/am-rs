source ../ARCHS.nu
source ../utils.nu
use std/log

source ./qemu.nu
source ./nemu.nu

def main [bin, arch] {
    let isa = get_isa $arch
    let platform = get_platform $arch

    let disasm_dir = $"target/disasm/($platform)/($isa)/($bin)"
    let elf_file = $"($disasm_dir)/image.elf"

    log info $"Running ($bin) for architecture: ($arch)"
    log info $"ELF file: ($elf_file)"

    match $platform {
        "qemu" => (qemu_run $elf_file $arch)
        "nemu" => (nemu_run $elf_file $arch)
        _ => (log error $"Unknown platform: ($platform)")
    }
}
