#![no_std]
#![no_main]

runtime::binInit!();

runtime::entry!(main);

// Command line arguments simulation (for benchmark mode)
// In a real embedded system, this would come from boot parameters or configuration
static BENCHMARK_MODE: bool = true; // Set to true for benchmark-only mode

// Benchmark configuration
const BENCHMARK_ITERATIONS: usize = 1000;
const WARMUP_ITERATIONS: usize = 100;
const DETAILED_BENCHMARK_ITERATIONS: usize = 100;

// RISC-V cycle counter access
#[inline(always)]
fn read_cycle_counter() -> u32 {
    let cycles: u32;
    unsafe {
        core::arch::asm!("rdcycle {}", out(reg) cycles);
    }
    cycles
}

/// Run detailed performance analysis on individual functions
fn detailed_performance_analysis(
    fc1_weights: &[[i8; 784]; 256],
    _fc2_weights: &[[i8; 256]; 128],
    _fc3_weights: &[[i8; 128]; 10],
    fc1_scale_q16: i32,
    _fc2_scale_q16: i32,
    _fc3_scale_q16: i32,
) {
    println!("=== DETAILED PERFORMANCE ANALYSIS ===");

    let benchmark_image_data = include_bytes!("../test_images/test_image_00000.bin");
    let (image_data, _) = parse_image_binary(benchmark_image_data);
    let normalized_input = normalize_input_pure_int8(&image_data);

    // Benchmark individual components
    let mut total_cycles: u64 = 0;

    // Benchmark normalize_input_pure_int8
    let start = read_cycle_counter();
    for _ in 0..DETAILED_BENCHMARK_ITERATIONS {
        let _ = normalize_input_pure_int8(&image_data);
    }
    let end = read_cycle_counter();
    let norm_cycles = ((end - start) as u64) / DETAILED_BENCHMARK_ITERATIONS as u64;
    println!("normalize_input_pure_int8: {} cycles/call", norm_cycles);
    total_cycles += norm_cycles;

    // Benchmark FC1 matrix multiplication
    let start = read_cycle_counter();
    for _ in 0..DETAILED_BENCHMARK_ITERATIONS {
        let _ = int8_matmul_symmetric::<256, 784>(fc1_weights, &normalized_input, fc1_scale_q16);
    }
    let end = read_cycle_counter();
    let fc1_cycles = ((end - start) as u64) / DETAILED_BENCHMARK_ITERATIONS as u64;
    println!("FC1 matmul (256x784): {} cycles/call", fc1_cycles);
    total_cycles += fc1_cycles;

    // Benchmark int32_to_int8_with_scaling
    let fc1_output =
        int8_matmul_symmetric::<256, 784>(fc1_weights, &normalized_input, fc1_scale_q16);
    let start = read_cycle_counter();
    for _ in 0..DETAILED_BENCHMARK_ITERATIONS {
        let _ = int32_to_int8_with_scaling(&fc1_output);
    }
    let end = read_cycle_counter();
    let scale_cycles = ((end - start) as u64) / DETAILED_BENCHMARK_ITERATIONS as u64;
    println!("int32_to_int8_with_scaling: {} cycles/call", scale_cycles);
    total_cycles += scale_cycles;

    // Benchmark relu6_int8
    let mut fc1_activations = int32_to_int8_with_scaling(&fc1_output);
    let start = read_cycle_counter();
    for _ in 0..DETAILED_BENCHMARK_ITERATIONS {
        relu6_int8(&mut fc1_activations);
    }
    let end = read_cycle_counter();
    let relu_cycles = ((end - start) as u64) / DETAILED_BENCHMARK_ITERATIONS as u64;
    println!("relu6_int8: {} cycles/call", relu_cycles);
    total_cycles += relu_cycles;

    println!("Estimated total cycles per inference: {}", total_cycles);
    println!("Breakdown:");
    println!(
        "  - Input normalization: {:.1}%",
        (norm_cycles * 100) as f64 / total_cycles as f64
    );
    println!(
        "  - FC1 matmul: {:.1}%",
        (fc1_cycles * 100) as f64 / total_cycles as f64
    );
    println!(
        "  - Scaling: {:.1}%",
        (scale_cycles * 100) as f64 / total_cycles as f64
    );
    println!(
        "  - Activation: {:.1}%",
        (relu_cycles * 100) as f64 / total_cycles as f64
    );
}

// Compile-time weight parsing using const generics
const fn parse_weight_binary_const<const ROWS: usize, const COLS: usize>(
    data: &'static [u8],
) -> ([[i8; COLS]; ROWS], f32) {
    // Read scale at compile time
    let scale = f32::from_le_bytes([data[8], data[9], data[10], data[11]]);

    let mut weights = [[0i8; COLS]; ROWS];
    let mut i = 0;
    while i < ROWS {
        let mut j = 0;
        let start = 12 + i * COLS;
        while j < COLS {
            weights[i][j] = data[start + j] as i8;
            j += 1;
        }
        i += 1;
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

fn parse_image_binary(data: &[u8]) -> (Vec<u8>, u8) {
    // Read true label
    let true_label = data[8];

    // Read image data
    let image_data: Vec<u8> = data[9..].to_vec();

    (image_data, true_label)
}

/// Normalize input from UINT8 [0,255] to INT8 [-128,127] range
///
/// Normalization formula: normalized = (pixel/255 * 2) - 1
/// Fixed-point implementation: output = (input * 257 - 32768) >> 8
fn normalize_input_pure_int8(input: &[u8]) -> Vec<i8> {
    const SCALE_Q16: i32 = 257; // 2/255 * 32768 ≈ 257
    const OFFSET_Q16: i32 = 32768; // 1 * 32768

    input
        .iter()
        .map(|&pixel| {
            let temp = ((pixel as i32 * SCALE_Q16 - OFFSET_Q16) >> 8) as i32;

            // Clamp to INT8 range
            if temp < -128 {
                -128
            } else if temp > 127 {
                127
            } else {
                temp as i8
            }
        })
        .collect()
}

/// Pure INT8 matrix multiplication with symmetric scaling
///
/// Operation: output = (weights * input) * scale
/// All operations in integer arithmetic
fn int8_matmul_symmetric<const ROWS: usize, const COLS: usize>(
    weights: &[[i8; COLS]; ROWS],
    input: &[i8],
    scale_q16: i32,
) -> Vec<i32> {
    let mut output = Vec::with_capacity(ROWS);

    for i in 0..ROWS {
        // Matrix multiplication in i32
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

/// ReLU6 activation for INT8 (clamp to [0,6] range)
///
/// ReLU6 provides better quantization behavior than standard ReLU
/// by limiting the activation range
fn relu6_int8(data: &mut [i8]) {
    for val in data.iter_mut() {
        if *val < 0 {
            *val = 0;
        }
        if *val > 6 {
            *val = 6;
        }
    }
}

/// Convert INT32 to INT8 with dynamic scaling
///
/// Automatically determines the optimal shift to preserve
/// as much precision as possible while fitting into INT8 range
fn int32_to_int8_with_scaling(input: &[i32]) -> Vec<i8> {
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

    // Apply scaling and clamp
    input
        .iter()
        .map(|&x| (x >> shift).clamp(-127, 127) as i8)
        .collect()
}

/// Simple argmax for classification
fn argmax_int32(data: &[i32]) -> usize {
    data.iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(idx, _)| idx)
        .unwrap_or(0)
}

/// True pure INT8 MNIST inference (no floating point operations)
///
/// Complete inference pipeline using only integer arithmetic:
/// 1. Input normalization (UINT8 → INT8)
/// 2. FC1 layer with ReLU6 activation
/// 3. FC2 layer with ReLU6 activation
/// 4. FC3 layer (output)
/// 5. Classification (argmax)
fn mnist_inference_pure_int8(
    fc1_weights: &[[i8; 784]; 256],
    fc2_weights: &[[i8; 256]; 128],
    fc3_weights: &[[i8; 128]; 10],
    input_image: &[u8],
    fc1_scale_q16: i32,
    fc2_scale_q16: i32,
    fc3_scale_q16: i32,
) -> usize {
    // Step 1: Normalize input from UINT8 to INT8 [-128, 127] range
    let normalized_input = normalize_input_pure_int8(input_image);

    // Step 2: Layer 1 - fc1
    let fc1_output =
        int8_matmul_symmetric::<256, 784>(&fc1_weights, &normalized_input, fc1_scale_q16);

    // Convert to INT8 for activation
    let mut fc1_activations = int32_to_int8_with_scaling(&fc1_output);

    // Apply ReLU6
    relu6_int8(&mut fc1_activations);

    // Step 3: Layer 2 - fc2
    let fc2_output =
        int8_matmul_symmetric::<128, 256>(&fc2_weights, &fc1_activations, fc2_scale_q16);

    // Convert to INT8 for activation
    let mut fc2_activations = int32_to_int8_with_scaling(&fc2_output);

    // Apply ReLU6
    relu6_int8(&mut fc2_activations);

    // Step 4: Layer 3 - fc3 (output layer)
    let final_output =
        int8_matmul_symmetric::<10, 128>(&fc3_weights, &fc2_activations, fc3_scale_q16);

    // Find predicted digit (argmax)
    argmax_int32(&final_output)
}

fn main() {
    // Compile-time weight parsing with known dimensions
    const FC1_WEIGHT_DATA: &[u8] = include_bytes!("../binarys/fc1_weight.bin");
    const FC2_WEIGHT_DATA: &[u8] = include_bytes!("../binarys/fc2_weight.bin");
    const FC3_WEIGHT_DATA: &[u8] = include_bytes!("../binarys/fc3_weight.bin");

    // Parse weights at compile time
    const FC1_PARSED: ([[i8; 784]; 256], f32) =
        parse_weight_binary_const::<256, 784>(FC1_WEIGHT_DATA);
    const FC2_PARSED: ([[i8; 256]; 128], f32) =
        parse_weight_binary_const::<128, 256>(FC2_WEIGHT_DATA);
    const FC3_PARSED: ([[i8; 128]; 10], f32) =
        parse_weight_binary_const::<10, 128>(FC3_WEIGHT_DATA);

    let (fc1_weights, fc1_scale) = FC1_PARSED;
    let (fc2_weights, fc2_scale) = FC2_PARSED;
    let (fc3_weights, fc3_scale) = FC3_PARSED;

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

    // Run benchmarks based on mode
    if BENCHMARK_MODE {
        // Benchmark-only mode - skip accuracy testing
        println!("=== BENCHMARK-ONLY MODE ===");

        // Run detailed performance analysis
        detailed_performance_analysis(
            &fc1_weights,
            &fc2_weights,
            &fc3_weights,
            fc1_scale_q16,
            fc2_scale_q16,
            fc3_scale_q16,
        );
        println!();

        // Run full inference benchmark
        run_benchmark(
            &fc1_weights,
            &fc2_weights,
            &fc3_weights,
            fc1_scale_q16,
            fc2_scale_q16,
            fc3_scale_q16,
        );

        return; // Exit after benchmarks
    } else {
        // Normal mode - run quick benchmark then accuracy test
        println!("=== QUICK BENCHMARK ===");
        run_benchmark(
            &fc1_weights,
            &fc2_weights,
            &fc3_weights,
            fc1_scale_q16,
            fc2_scale_q16,
            fc3_scale_q16,
        );
        println!();
    }

    let test_images_data = vec![
        include_bytes!("../test_images/test_image_00000.bin"),
        include_bytes!("../test_images/test_image_00001.bin"),
        include_bytes!("../test_images/test_image_00002.bin"),
        include_bytes!("../test_images/test_image_00003.bin"),
        include_bytes!("../test_images/test_image_00004.bin"),
        include_bytes!("../test_images/test_image_00005.bin"),
        include_bytes!("../test_images/test_image_00006.bin"),
        include_bytes!("../test_images/test_image_00007.bin"),
        include_bytes!("../test_images/test_image_00008.bin"),
        include_bytes!("../test_images/test_image_00009.bin"),
    ];

    let total_images = test_images_data.len();
    let mut correct_predictions = 0;

    for (img_idx, image_data_bytes) in test_images_data.iter().enumerate() {
        println!("=== Test Image {} ===", img_idx + 1);

        let (image_data, true_label) = parse_image_binary(*image_data_bytes);
        println!("True label: {}", true_label);

        // Run pure INT8 inference with embedded weights
        let predicted_digit = mnist_inference_pure_int8(
            &fc1_weights,
            &fc2_weights,
            &fc3_weights,
            &image_data,
            fc1_scale_q16,
            fc2_scale_q16,
            fc3_scale_q16,
        );

        println!("Predicted:  {}", predicted_digit);

        if predicted_digit == true_label as usize {
            println!("✓ CORRECT PREDICTION!");
            correct_predictions += 1;
        } else {
            println!("❌ WRONG PREDICTION!");
        }

        println!();
    }

    // Summary
    println!("=== FINAL RESULTS ===");
    println!("Total images: {}", total_images);
    println!("Correct predictions: {}", correct_predictions);
    println!(
        "Accuracy: {:.2}%",
        (correct_predictions as f32 / total_images as f32) * 100.0
    );
}

/// Benchmark function to measure inference performance
fn run_benchmark(
    fc1_weights: &[[i8; 784]; 256],
    fc2_weights: &[[i8; 256]; 128],
    fc3_weights: &[[i8; 128]; 10],
    fc1_scale_q16: i32,
    fc2_scale_q16: i32,
    fc3_scale_q16: i32,
) {
    println!("=== BENCHMARK MODE ===");
    println!("Warmup iterations: {}", WARMUP_ITERATIONS);
    println!("Benchmark iterations: {}", BENCHMARK_ITERATIONS);

    // Use a representative test image for benchmarking
    let benchmark_image_data = include_bytes!("../test_images/test_image_00000.bin");
    let (image_data, _) = parse_image_binary(benchmark_image_data);

    // Warmup phase
    println!("Running warmup...");
    for _ in 0..WARMUP_ITERATIONS {
        let _ = mnist_inference_pure_int8(
            fc1_weights,
            fc2_weights,
            fc3_weights,
            &image_data,
            fc1_scale_q16,
            fc2_scale_q16,
            fc3_scale_q16,
        );
    }

    // Benchmark phase with cycle counting
    println!("Running benchmark with cycle counting...");

    let start_cycles = read_cycle_counter();

    for _ in 0..BENCHMARK_ITERATIONS {
        let _ = mnist_inference_pure_int8(
            fc1_weights,
            fc2_weights,
            fc3_weights,
            &image_data,
            fc1_scale_q16,
            fc2_scale_q16,
            fc3_scale_q16,
        );
    }

    let end_cycles = read_cycle_counter();
    let total_cycles = (end_cycles - start_cycles) as u64;

    // Calculate metrics
    let cycles_per_inference = total_cycles / BENCHMARK_ITERATIONS as u64;
    let inferences_per_second = if total_cycles > 0 {
        // Assuming 1GHz clock for calculation
        (1_000_000_000u64 * BENCHMARK_ITERATIONS as u64) / total_cycles
    } else {
        0
    };

    println!("=== BENCHMARK RESULTS ===");
    println!("Total cycles measured: {}", total_cycles);
    println!("Iterations completed: {}", BENCHMARK_ITERATIONS);
    println!("Cycles per inference: {}", cycles_per_inference);
    println!("Inferences per second (1GHz): {}", inferences_per_second);

    // Performance classification
    println!("Performance classification:");
    if cycles_per_inference < 100_000 {
        println!("Excellent performance");
    } else if cycles_per_inference < 500_000 {
        println!("Good performance");
    } else if cycles_per_inference < 2_000_000 {
        println!("Moderate performance");
    } else {
        println!("Needs optimization");
    }

    // Performance analysis
    let total_mac_operations =
        BENCHMARK_ITERATIONS as u64 * ((784 * 256) + (256 * 128) + (128 * 10)) as u64;
    let macs_per_cycle = if total_cycles > 0 {
        total_mac_operations as f64 / total_cycles as f64
    } else {
        0.0
    };

    println!("Total MAC operations: {}", total_mac_operations);
    println!("MACs per cycle: {:.4}", macs_per_cycle);
    println!("Note: Higher MACs/cycle indicates better vectorization");

    if BENCHMARK_ITERATIONS > 0 {
        println!("Benchmark completed successfully");
    }

    // Save baseline for comparison
    println!("Use this as baseline for optimization comparisons");
}

/// Quick benchmark for development iterations
#[allow(dead_code)]
fn quick_benchmark(
    fc1_weights: &[[i8; 784]; 256],
    fc2_weights: &[[i8; 256]; 128],
    fc3_weights: &[[i8; 128]; 10],
    fc1_scale_q16: i32,
    fc2_scale_q16: i32,
    fc3_scale_q16: i32,
) -> u32 {
    let benchmark_image_data = include_bytes!("../test_images/test_image_00000.bin");
    let (image_data, _) = parse_image_binary(benchmark_image_data);

    let start_cycles = read_cycle_counter();

    for _ in 0..10 {
        let _ = mnist_inference_pure_int8(
            fc1_weights,
            fc2_weights,
            fc3_weights,
            &image_data,
            fc1_scale_q16,
            fc2_scale_q16,
            fc3_scale_q16,
        );
    }

    let end_cycles = read_cycle_counter();
    (end_cycles - start_cycles) / 10
}
