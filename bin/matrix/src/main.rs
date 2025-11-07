#![no_std]
#![no_main]

nemu_runtime::binInit!();

nemu_runtime::entry!(main);

use core::arch::asm;

#[inline]
fn mwsub_mm(result_reg: usize, mat_a_reg: usize, mat_b_reg: usize) -> usize {
    let mut result = result_reg;
    unsafe {
        asm!(
            ".insn r 0x0B, 4, 4, {0}, {1}, {2}",
            inout(reg) result,
            in(reg) mat_a_reg,
            in(reg) mat_b_reg,
            options(nostack, preserves_flags)
        );
    }
    result
}

fn main() -> ! {
    mwsub_mm(0, 0, 0);
    loop {}
}
