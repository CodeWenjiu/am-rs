source ../arch/main.nu
source ../utils.nu
use std/log

export def nemu_run [bin, arch, batch: bool] {
    log error $"Sadly, nemu is not supported to run \(for now?\)."
    return
}
