source ../arch/main.nu
source ../utils.nu
use std/log

export def qemu_run [bin, arch, batch: bool] {
    let split = arch_split $arch
    let isa = $split.isa
    let platform = $split.platform

    # Determine QEMU machine and CPU based on ISA
    let qemu_machine = "virt"
    let qemu_cpu = match $isa {
        "riscv32i" => "rv32",
        "riscv32im" => "rv32",
        "riscv32imac" => "rv32",
        "riscv32im_zve32x" => "rv32,v=true,vlen=128",
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
        "-kernel" $bin
    ]

    if $batch == false {
        log info $"QEMU command: (($qemu_cmd | str join ' '))"
        log info $"Press 'Ctrl+A X' to exit QEMU"
        log info "----------------------------------------"
    }

    # Run QEMU
    run-external ...$qemu_cmd
}
