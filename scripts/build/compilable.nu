def is_compilable [bin, arch] {
    let arch_metadata = get_bin_matadata $bin | get arch?
    match $arch_metadata {
        null => true,
        _ => {
            if ($arch_metadata.atomic == true) {
                return ((arch_split $arch).isa == "riscv32imac")
            }
            print "oh"

            return true
        }
    }
}
