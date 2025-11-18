def is_compilable [bin, arch] {
    let arch_metadata = get_bin_matadata $bin | get arch?
    match $arch_metadata {
        null => true
        {requirement: $some} => {
            match $some {
                {arch: "atomic"} => {
                    return ((arch_split $arch).isa == "riscv32imac")
                }
                {io: "graphic"} => {
                    return ((arch_split $arch).platform == "qemu")
                }
                _ => true
            }
            _ => true
        }
        _ => true
    }
}
