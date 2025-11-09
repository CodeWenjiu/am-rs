#![no_std]
#![no_main]
#![allow(unstable_features)]

runtime::binInit!();

runtime::entry!(main);

fn vector_add_auto(a: &[f32], b: &[f32], c: &mut [f32]) {
    for i in 0..a.len() {
        c[i] = a[i] + b[i];
    }
}

fn main() {
    let a = [1.0f32, 2.0, 3.0, 4.0];
    let b = [5.0f32, 6.0, 7.0, 8.0];
    let mut c = [0.0f32; 4];

    vector_add_auto(&a, &b, &mut c);
    println!("Vector: {:?}", c);
}
