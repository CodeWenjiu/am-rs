source ./ARCHS.nu
source ./utils.nu
use std/log

def disasm [bin?, arch?] {
    load-env (prepare_env $arch)
    let target = get_target $arch

    let parts = $arch | split row "-"
    let isa = ($parts | get 0)
    let platform = ($parts | get 1)
    let disasm_dir = $"target/disasm/($platform)/($isa)/($bin)"
    mkdir $disasm_dir

    log info $"Generating disassembly for architecture: ($arch), binary: ($bin), target: ($target)"

    cp $"target/($target)/release/($bin)" $"($disasm_dir)/image.elf"
    cargo objdump --bin $bin --target $target --release -- -d | save --force $"($disasm_dir)/image.txt"
    cargo objcopy --bin $bin --target $target --release -- -O binary $"($disasm_dir)/image.bin"
}

def disasm_all_arch [bin] {
    log debug $"Generating disassembly for ($bin) for all architectures: ($ARCHS)"
    $ARCHS | par-each {|arch|
        disasm $bin $arch
    } | ignore
}

def main [bin?, arch?] {
    log critical "Starting disassembly process..."
    match $bin {
        null => {
            log critical "disassembly for all binaries for all architectures"
            get_allbin | par-each {|bin|
                disasm_all_arch $bin
            } | ignore
        }
        _ => {
            match $arch {
                null => {
                    disasm_all_arch $bin
                }
                _ => {
                    disasm $bin $arch
                }
            }
        }
    }
    log critical "Disassembly process completed"
}
