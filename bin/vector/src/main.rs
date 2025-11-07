#![no_std]
#![no_main]

#![allow(unstable_features)]

nemu_runtime::binInit!();

nemu_runtime::entry!(main);

use core::arch::asm;

fn vector_add_asm(a: &[f32], b: &[f32], c: &mut [f32], n: usize) {
    unsafe {
        asm!(
            // 设置向量长度 (vsetivli - 立即数版本)
            "vsetivli zero, 4, e32, m1, ta, ma",
            // 循环开始
            "1:",
            // 加载向量 a (vle32.v)
            "vle32.v v1, ({1})",
            // 加载向量 b (vle32.v)  
            "vle32.v v2, ({2})",
            // 向量加法 (vfadd.vv)
            "vfadd.vv v3, v1, v2",
            // 存储结果 (vse32.v)
            "vse32.v v3, ({3})",
            // 更新指针
            "addi {1}, {1}, 16",  // 4个f32 = 16字节
            "addi {2}, {2}, 16",
            "addi {3}, {3}, 16",
            // 减少计数并循环 (使用 addi 减法)
            "addi {0}, {0}, -4",
            "bnez {0}, 1b",
            inout(reg) n => _,  // 元素计数
            inout(reg) a.as_ptr() => _,  // a 指针
            inout(reg) b.as_ptr() => _,  // b 指针
            inout(reg) c.as_mut_ptr() => _,  // c 指针
        );
    }
}

fn main() -> ! {
    let a = [1.0f32, 2.0, 3.0, 4.0];
    let b = [5.0f32, 6.0, 7.0, 8.0];
    let mut c = [0.0f32; 4];
    
    vector_add_asm(&a, &b, &mut c, 4);
    println!("{:?}", c);

    loop {}
}
