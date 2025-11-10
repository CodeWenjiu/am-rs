source ../arch/main.nu
source ../utils.nu
use std/log

export def spike_run [bin, arch] {
    let isa = get_isa $arch
    let platform = get_platform $arch

    let ISA = match $isa {
        "riscv32i" => "RV32I",
        "riscv32im" => "RV32IM",
        "riscv32imv" => "RV32IMV",
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
    let spike_cmd = [
        "spike"
        "--isa" $ISA
        "-m0x80000000:0x08000000"
        $bin
    ]

    log info $"SPIKE command: (($spike_cmd | str join ' '))"
    log info $"Press 'Ctrl+C' to Terminate and quit SPIKE"
    log info "----------------------------------------"

    # Run QEMU
    run-external ...$spike_cmd
}
