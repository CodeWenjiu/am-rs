source ../arch/main.nu
source ../utils.nu
use std/log

export def spike_run [bin, arch, batch: bool] {
    let isa = get_isa $arch
    let platform = get_platform $arch

    let ISA = match $isa {
        "riscv32i" => "rv32i",
        "riscv32im" => "rv32im",
        "riscv32im_zve32x" => "rv32im_zve32x_zvl64b",
        _ => {
            log error $"Unknown ISA: ($isa)"
            return
        }
    }

    # Spike command
    # -d: Enable interactive debugger in batch mode
    # --isa: Specify ISA string
    # -m: Memory range
    let debug_flag = if $batch { ["-d"] } else { [] }
    
    let spike_cmd = ["spike" "--isa" $ISA "-m0x80000000:0x08000000"] ++ $debug_flag ++ [$bin]

    log info $"SPIKE command: (($spike_cmd | str join ' '))"
    log info $"Press 'Ctrl+C' to Terminate and quit SPIKE"
    log info "----------------------------------------"

    # Run QEMU
    run-external ...$spike_cmd
}
