source ./ARCHS.nu
source ./utils.nu
use std/log

export def get_disasm_dir [bin, arch] {
    let isa = get_isa $arch
    let platform = get_platform $arch
    $"target/disasm/($platform)/($isa)/($bin)"
}

def disasm [bin?, arch?] {
    load-env (prepare_env $arch)
    let target = get_target $arch
    let platform = get_platform $arch

    let disasm_dir = get_disasm_dir $bin $arch
    mkdir $disasm_dir

    log info $"Generating disassembly for architecture: ($arch), binary: ($bin), target: ($target)"

    cargo objdump --bin $bin --target $target --release --no-default-features --features $"runtime/($platform)" -- -d | save --force $"($disasm_dir)/image.txt"
    cargo objcopy --bin $bin --target $target --release --no-default-features --features $"runtime/($platform)" -- -O binary $"($disasm_dir)/image.bin"
    cp $"target/($target)/release/($bin)" $"($disasm_dir)/image.elf"
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
