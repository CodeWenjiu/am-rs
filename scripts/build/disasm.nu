source ../arch/utils.nu
source ../utils.nu
use std/log

source ./compilable.nu

export def get_disasm_dir [bin: string, arch: string] {
    let split = arch_split $arch
    let isa = $split.isa
    let platform = $split.platform
    $"target/disasm/($platform)/($isa)/($bin)"
}

def disasm [bin: string, arch: string] {
    load-env (prepare_env $arch)
    validate_bin $bin

    if (is_compilable $bin $arch) == false {
        log warning $"Skipping build for binary: ($bin) on architecture: ($arch) as it is not compilable."
        return
    }

    let split = arch_split $arch
    let target = $split.target
    let platform = $split.platform

    let disasm_dir = get_disasm_dir $bin $arch
    mkdir $disasm_dir

    log info $"Generating disassembly for architecture: ($arch), binary: ($bin), target: ($target)"

    cargo objdump --bin $bin --target $target --release --no-default-features --features $"runtime/($platform)" -- -d | save --force $"($disasm_dir)/image.txt"
    cargo objcopy --bin $bin --target $target --release --no-default-features --features $"runtime/($platform)" -- -O binary $"($disasm_dir)/image.bin"
    cp $"target/($target)/release/($bin)" $"($disasm_dir)/image.elf"
}

def disasm_arch [bin: string, arch: string] {
    match $bin {
        "_ALL" => {
            get_all_bins | each {|bin|
                disasm $bin $arch
            } | ignore
        }
        _ => {
            disasm $bin $arch
        }
    }
}

def main [bin: string, arch: string] {
    match $arch {
        "_ALL" => {
            get_all_archs | each {|arch|
                disasm_arch $bin $arch
            } | ignore
        }
        _ => {
            disasm_arch $bin $arch
        }
    }
}
