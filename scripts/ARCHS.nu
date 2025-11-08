export const ARCHS = ["riscv32i-nemu", "riscv32im-nemu", "riscv32imv-nemu"]

export const ARCH_TARGETS = {
    "riscv32i-nemu": "riscv32i-unknown-none-elf",
    "riscv32im-nemu": "riscv32im-unknown-none-elf",
    "riscv32imv-nemu": "riscv32im-unknown-none-elf"
}

export def validate_arch [arch: string] {
    if not ($arch in $ARCHS) {
        error make {
            msg: $"Error: Expected arch in ($ARCHS), Got ($arch)"
        }
    }
}

export def prepare_env [arch: string] {
    validate_arch $arch

    mut env_vars = {ARCH: $arch}

    if $arch == "riscv32imv-nemu" {
        print "FLAGS Prepared for RVV"
        $env_vars.RUSTFLAGS = "-C target-feature=+v"
    }

    return $env_vars
}

export def get_target [arch: string] {
    ($ARCH_TARGETS | get $arch)
}
