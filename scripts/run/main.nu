source ../arch/utils.nu
source ../utils.nu
use std/log

source ./qemu.nu
source ./spike.nu
source ./nemu.nu

def main [bin, arch, batch: bool] {
    let isa = get_isa $arch
    let platform = get_platform $arch

    let disasm_dir = $"target/disasm/($platform)/($isa)/($bin)"
    let elf_file = $"($disasm_dir)/image.elf"

    log info $"Running ($bin) for architecture: ($arch)"
    log info $"ELF file: ($elf_file)"

    match $platform {
        "qemu" => (qemu_run $elf_file $arch $batch)
        "spike" => (spike_run $elf_file $arch $batch)
        "nemu" => (nemu_run $elf_file $arch $batch)
        _ => (log error $"Unknown platform: ($platform)")
    }
}
