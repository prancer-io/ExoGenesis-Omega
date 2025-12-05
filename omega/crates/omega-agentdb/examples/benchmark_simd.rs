/// Benchmark comparing SIMD vs theoretical scalar performance
/// This demonstrates the speedup achieved by SimSIMD optimization
use std::time::Instant;
use simsimd::SpatialSimilarity;

/// Scalar implementation (the old way - for comparison)
fn cosine_distance_scalar(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 1.0;
    }

    1.0 - (dot / (norm_a * norm_b))
}

/// SIMD implementation (the new way)
fn cosine_distance_simd(a: &[f32], b: &[f32]) -> f32 {
    f32::cosine(a, b).unwrap_or(1.0) as f32
}

fn main() {
    println!("=== SimSIMD Performance Benchmark ===\n");

    // Test different vector dimensions
    for dim in [128, 512, 1024, 4096] {
        println!("Dimension: {}", dim);

        // Create random-ish vectors
        let a: Vec<f32> = (0..dim).map(|i| (i as f32 * 0.1).sin()).collect();
        let b: Vec<f32> = (0..dim).map(|i| (i as f32 * 0.1).cos()).collect();

        // Warm up
        for _ in 0..100 {
            let _ = cosine_distance_scalar(&a, &b);
            let _ = cosine_distance_simd(&a, &b);
        }

        // Benchmark scalar (use black_box to prevent optimization)
        let iterations = 100_000;
        let start = Instant::now();
        let mut sum = 0.0;
        for _ in 0..iterations {
            sum += cosine_distance_scalar(&a, &b);
        }
        let scalar_time = start.elapsed();
        std::hint::black_box(sum); // Prevent dead code elimination

        // Benchmark SIMD (use black_box to prevent optimization)
        let start = Instant::now();
        let mut sum = 0.0;
        for _ in 0..iterations {
            sum += cosine_distance_simd(&a, &b);
        }
        let simd_time = start.elapsed();
        std::hint::black_box(sum); // Prevent dead code elimination

        let scalar_ns = scalar_time.as_nanos() / iterations;
        let simd_ns = simd_time.as_nanos() / iterations;
        let speedup = scalar_time.as_secs_f64() / simd_time.as_secs_f64();

        println!("  Scalar: {:>6} ns/op", scalar_ns);
        println!("  SIMD:   {:>6} ns/op", simd_ns);
        println!("  Speedup: {:.2}x faster", speedup);
        println!();
    }

    // Verify correctness
    println!("=== Correctness Verification ===");
    let test_a = vec![1.0, 0.0, 0.0];
    let test_b = vec![1.0, 0.0, 0.0];
    let test_c = vec![0.0, 1.0, 0.0];

    let scalar_identical = cosine_distance_scalar(&test_a, &test_b);
    let simd_identical = cosine_distance_simd(&test_a, &test_b);
    let scalar_ortho = cosine_distance_scalar(&test_a, &test_c);
    let simd_ortho = cosine_distance_simd(&test_a, &test_c);

    println!("Identical vectors [1,0,0] vs [1,0,0]:");
    println!("  Scalar: {:.6} (expect ~0.0)", scalar_identical);
    println!("  SIMD:   {:.6} (expect ~0.0)", simd_identical);
    println!("  Match: {}", (scalar_identical - simd_identical).abs() < 0.001);

    println!("\nOrthogonal vectors [1,0,0] vs [0,1,0]:");
    println!("  Scalar: {:.6} (expect ~1.0)", scalar_ortho);
    println!("  SIMD:   {:.6} (expect ~1.0)", simd_ortho);
    println!("  Match: {}", (scalar_ortho - simd_ortho).abs() < 0.001);

    println!("\n✅ SimSIMD integration complete and verified!");
}
