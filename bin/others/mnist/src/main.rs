#![no_std]
#![no_main]

runtime::binInit!();

runtime::entry!(main);

fn parse_weight_binary(data: &[u8]) -> (Vec<Vec<i8>>, f32) {
    // Read shape (rows, cols)
    let rows = i32::from_le_bytes(data[0..4].try_into().unwrap()) as usize;
    let cols = i32::from_le_bytes(data[4..8].try_into().unwrap()) as usize;

    // Read scale
    let scale = f32::from_le_bytes(data[8..12].try_into().unwrap());

    let mut weights = Vec::with_capacity(rows);
    for i in 0..rows {
        let start = 12 + i * cols;
        let end = start + cols;
        let row_weights: Vec<i8> = data[start..end].iter().map(|&b| b as i8).collect();
        weights.push(row_weights);
    }

    (weights, scale)
}

const Q16_SHIFT: u32 = 16;
/// Macro to convert float scale to Q16 fixed-point
macro_rules! scale_to_q16 {
    ($scale:expr) => {
        (($scale * (1 << Q16_SHIFT) as f32) as i32)
    };
}

fn main() {
    let fc1_weight_data = include_bytes!("../binarys/fc1_weight.bin");
    let fc2_weight_data = include_bytes!("../binarys/fc2_weight.bin");
    let fc3_weight_data = include_bytes!("../binarys/fc3_weight.bin");

    let (fc1_weights, fc1_scale) = parse_weight_binary(fc1_weight_data);
    let (fc2_weights, fc2_scale) = parse_weight_binary(fc2_weight_data);
    let (fc3_weights, fc3_scale) = parse_weight_binary(fc3_weight_data);

    println!("Model weights loaded successfully!");
    println!(
        "FC1: {}x{}, scale: {:.6}",
        fc1_weights.len(),
        fc1_weights[0].len(),
        fc1_scale
    );
    println!(
        "FC2: {}x{}, scale: {:.6}",
        fc2_weights.len(),
        fc2_weights[0].len(),
        fc2_scale
    );
    println!(
        "FC3: {}x{}, scale: {:.6}\n",
        fc3_weights.len(),
        fc3_weights[0].len(),
        fc3_scale
    );

    let fc1_scale_q16 = scale_to_q16!(fc1_scale);
    let fc2_scale_q16 = scale_to_q16!(fc2_scale);
    let fc3_scale_q16 = scale_to_q16!(fc3_scale);

    // Print quantization information
    println!("Quantization Scales (Fixed Point):");
    println!("  FC1_SCALE: {:.6} -> Q16: {}", fc1_scale, fc1_scale_q16);
    println!("  FC2_SCALE: {:.6} -> Q16: {}", fc2_scale, fc2_scale_q16);
    println!("  FC3_SCALE: {:.6} -> Q16: {}", fc3_scale, fc3_scale_q16);
    println!();
}
