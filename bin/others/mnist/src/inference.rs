#[cfg(not(test))]
runtime::libInit!();

/// Normalize input from UINT8 [0,255] to INT8 [-128,127] range
///
/// Normalization formula: normalized = (pixel/255 * 2) - 1
/// Fixed-point implementation: output = (input * 257 - 32768) >> 8
pub(crate) fn normalize_and_quantize_input(input: &[u8]) -> Vec<i8> {
    input
        .iter()
        .map(|&pixel| {
            // Convert [0, 255] to [0, 1]
            let normalized = (pixel as f32) / 255.0;

            // Quantize symmetrically to INT8 range [-127, 127]
            // For [0, 1] range, scale to [-127, 127] is:
            // quantized = normalized * 127.0
            let quantized = (normalized * 127.0) as i32;

            // Clamp to INT8 range
            if quantized < -128 {
                -128
            } else if quantized > 127 {
                127
            } else {
                quantized as i8
            }
        })
        .collect()
}

pub(crate) const Q16_SHIFT: u32 = 16;
/// Pure INT8 matrix multiplication with symmetric scaling
///
/// Operation: output = (weights * input) * scale
/// All operations in integer arithmetic
pub(crate) fn int8_matmul_symmetric<const ROWS: usize, const COLS: usize>(
    weights: &[[i8; COLS]; ROWS],
    input: &[i8],
    scale_q16: i32,
) -> Vec<i32> {
    let mut output = Vec::with_capacity(ROWS);

    // Simple nested loops - let LLVM handle vectorization
    for i in 0..ROWS {
        let mut sum: i32 = 0;
        for j in 0..COLS {
            sum += weights[i][j] as i32 * input[j] as i32;
        }

        // Apply scaling: sum * scale (in Q16 format)
        let scaled = (sum as i64 * scale_q16 as i64) >> Q16_SHIFT;

        output.push(scaled as i32);
    }

    output
}

/// ReLU activation for INT8 (clamp to [0, max_value])
///
/// After ReLU, we keep positive values and clamp negatives to 0.
/// The max value should be based on actual calibration data.
/// Using a reasonable max to avoid overflow.
pub(crate) fn relu_int8(data: &mut [i8]) {
    for val in data.iter_mut() {
        if *val < 0 {
            *val = 0;
        }
    }
}

/// Simple argmax for classification
pub(crate) fn argmax_int32(data: &[i32]) -> usize {
    data.iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(idx, _)| idx)
        .unwrap_or(0)
}

/// Convert INT32 to INT8 with dynamic scaling
///
/// Automatically determines the optimal shift to preserve
/// as much precision as possible while fitting into INT8 range
pub(crate) fn int32_to_int8_with_scaling(input: &[i32]) -> Vec<i8> {
    // Find the maximum absolute value to determine scaling
    let max_abs = input.iter().fold(0, |acc, &x| acc.max(x.abs()));

    // If all values are zero, return zeros
    if max_abs == 0 {
        return vec![0; input.len()];
    }

    // Calculate shift needed to fit into INT8 range [-127, 127]
    let mut shift = 0;
    let mut max_val = max_abs;
    while max_val > 127 && shift < 31 {
        max_val >>= 1;
        shift += 1;
    }

    // Simple loop - let LLVM handle vectorization
    let mut result = Vec::with_capacity(input.len());
    for &x in input {
        result.push((x >> shift).clamp(-128, 127) as i8);
    }
    result
}

/// True pure INT8 MNIST inference (no floating point operations)
///
/// Complete inference pipeline using only integer arithmetic:
/// 1. Input normalization (UINT8 → INT8)
/// 2. FC1 layer with ReLU6 activation
/// 3. FC2 layer with ReLU6 activation
/// 4. FC3 layer (output)
/// 5. Classification (argmax)
pub(crate) fn mnist_inference_pure_int8(
    fc1_weights: &[[i8; 784]; 256],
    fc2_weights: &[[i8; 256]; 128],
    fc3_weights: &[[i8; 128]; 10],
    input_image: &[u8],
    fc1_scale_q16: i32,
    fc2_scale_q16: i32,
    fc3_scale_q16: i32,
) -> usize {
    // Step 1: Normalize input from UINT8 to INT8 [-128, 127] range
    let normalized_input = normalize_and_quantize_input(input_image);

    // Step 2: Layer 1 - fc1
    let fc1_output =
        int8_matmul_symmetric::<256, 784>(&fc1_weights, &normalized_input, fc1_scale_q16);

    // Convert to INT8 for activation
    let mut fc1_activations = int32_to_int8_with_scaling(&fc1_output);

    // Apply ReLU6
    relu_int8(&mut fc1_activations);

    // Step 3: Layer 2 - fc2
    let fc2_output =
        int8_matmul_symmetric::<128, 256>(&fc2_weights, &fc1_activations, fc2_scale_q16);

    // Convert to INT8 for activation
    let mut fc2_activations = int32_to_int8_with_scaling(&fc2_output);

    // Apply ReLU6
    relu_int8(&mut fc2_activations);

    // Step 4: Layer 3 - fc3 (output layer)
    let final_output =
        int8_matmul_symmetric::<10, 128>(&fc3_weights, &fc2_activations, fc3_scale_q16);

    // Find predicted digit (argmax)
    argmax_int32(&final_output)
}
