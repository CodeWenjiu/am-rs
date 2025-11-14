source ../arch/utils.nu
source ../utils.nu
use std/log

source ./qemu.nu
source ./spike.nu
source ./nemu.nu

source ../build/disasm.nu

def get_elf [bin, arch] {
    let elf_file = $"(get_disasm_dir $bin $arch)/image.elf"
    return $elf_file
}

def pla_run [bin, arch, batch: bool] {
    let elf = get_elf $bin $arch
    let platform = (arch_split $arch).platform

    if (disasm $bin $arch) == false {
        return
    }

    match $platform {
        "qemu" => (qemu_run $elf $arch $batch)
        "spike" => (spike_run $elf $arch $batch)
        "nemu" => (nemu_run $elf $arch $batch)
        _ => (log error $"Unknown platform: ($platform)")
    }
}

def "main run" [bin, arch] {
    log info $"Running ($bin) for architecture: ($arch)"

    pla_run $bin $arch false
}

def is_test_involved [bin] {
    let test_matadata = get_bin_matadata $bin | get test?
    ($test_matadata != null and $test_matadata.involved == true)
}

def "main user" [] {
    get_all_bins |
        par-each {|bin|
            if (is_test_involved $bin) == false {
                return
            }

            print $"Testing binary: ($bin)"
        }
}

def test_arch [bin: string, arch: string] {
    match $bin {
        "_ALL" => {
            log info $"Testing architecture: ($arch)"
            let result = get_all_bins | par-each {|bin|
                if (is_test_involved $bin) == false {
                    return
                }
                let stdout = pla_run $bin $arch true
                {
                    binary: $bin
                    stdout: $stdout
                    quit_state: true
                }
            }
            print $result

            print {
                total: ($result | length)
                passed: ($result | where {|r| $r.quit_state == true} | length)
                failed: ($result | where {|r| $r.quit_state != true} | length)
            }
        }

        _ => {
            log info $"Testing ($bin) for architecture: ($arch)"
            pla_run $bin $arch true
        }
    }
}

def "main test" [bin: string, arch: string] {
    match $arch {
        "_ALL" => {
            get_all_archs | par-each {|arch|
                {
                    arch: $arch
                    result: (test_arch $bin $arch)
                }
            }
        }
        _ => {
            test_arch $bin $arch
        }
    }
}
