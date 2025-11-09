source ./ARCHS.nu
source ./utils.nu
use std/log

def main [bin, arch] {
    load-env (prepare_env $arch)
    let target = get_target $arch
    let platform = get_platform $arch

    # Only support QEMU platform
    if $platform != "qemu" {
        log error $"This script only supports QEMU platform, but got: ($platform)"
        log error $"Please use an architecture with '-qemu' suffix"
        return
    }

    let parts = $arch | split row "-"
    let isa = ($parts | get 0)
    let disasm_dir = $"target/disasm/($platform)/($isa)/($bin)"
    let elf_file = $"($disasm_dir)/image.elf"

    # Check if the ELF file exists
    if not ($elf_file | path exists) {
        log error $"ELF file not found: ($elf_file)"
        log error "Please run 'just disasm' first to generate the image"
        return
    }

    log info $"Running ($bin) on QEMU for architecture: ($arch)"
    log info $"ELF file: ($elf_file)"

    # Determine QEMU machine and CPU based on ISA
    let qemu_machine = "virt"
    let qemu_cpu = match $isa {
        "riscv32i" => "rv32",
        "riscv32im" => "rv32",
        "riscv32imv" => "rv32,v=true,vlen=128",
        _ => {
            log error $"Unknown ISA: ($isa)"
            return
        }
    }

    # QEMU command
    # -machine virt: Use the virt machine (generic virtual platform)
    # -cpu: Specify CPU type
    # -m: Memory size (default 128M)
    # -nographic: No graphical output, use serial console
    # -serial mon:stdio: Redirect serial to stdio
    # -bios none: Don't load default BIOS
    # -kernel: Load our bare-metal ELF
    let qemu_cmd = [
        "qemu-system-riscv32"
        "-machine" $qemu_machine
        "-cpu" $qemu_cpu
        "-m" "128M"
        "-nographic"
        "-serial" "mon:stdio"
        "-bios" "none"
        "-kernel" $elf_file
    ]

    log info $"QEMU command: (($qemu_cmd | str join ' '))"
    log info "----------------------------------------"

    # Run QEMU
    run-external ...$qemu_cmd
}
