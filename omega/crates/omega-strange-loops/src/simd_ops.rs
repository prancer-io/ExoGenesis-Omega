//! SIMD-Optimized Vector Operations for Strange Loops
//!
//! High-performance distance, similarity, and state evolution computations using SIMD:
//! - Cosine similarity/distance (AVX2/AVX-512/NEON accelerated)
//! - Dot product (inner product)
//! - L2 (Euclidean) distance
//! - Batch state evolution
//! - Weighted combinations
//!
//! These operations are critical for:
//! - Self-reference detection (cosine similarity between input and reflection)
//! - Strange loop strength computation
//! - Meta-cognitive processing
//! - Consciousness delta calculations

use simsimd::SpatialSimilarity;

/// SIMD acceleration level detected at runtime
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimdLevel {
    /// Scalar fallback (no SIMD)
    Scalar,
    /// SSE4.1 (128-bit, 4 floats)
    Sse4,
    /// AVX2 (256-bit, 8 floats)
    Avx2,
    /// AVX-512 (512-bit, 16 floats)
    Avx512,
    /// ARM NEON (128-bit, 4 floats)
    Neon,
}

impl SimdLevel {
    /// Detect available SIMD level
    pub fn detect() -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx512f") {
                return SimdLevel::Avx512;
            }
            if is_x86_feature_detected!("avx2") {
                return SimdLevel::Avx2;
            }
            if is_x86_feature_detected!("sse4.1") {
                return SimdLevel::Sse4;
            }
        }
        #[cfg(target_arch = "aarch64")]
        {
            // NEON is always available on aarch64
            return SimdLevel::Neon;
        }
        SimdLevel::Scalar
    }

    /// Get vector width in floats
    pub fn width(&self) -> usize {
        match self {
            SimdLevel::Scalar => 1,
            SimdLevel::Sse4 => 4,
            SimdLevel::Avx2 => 8,
            SimdLevel::Avx512 => 16,
            SimdLevel::Neon => 4,
        }
    }

    /// Get human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            SimdLevel::Scalar => "Scalar",
            SimdLevel::Sse4 => "SSE4.1",
            SimdLevel::Avx2 => "AVX2",
            SimdLevel::Avx512 => "AVX-512",
            SimdLevel::Neon => "ARM NEON",
        }
    }
}

// ============================================================================
// SIMD-Accelerated Similarity Operations (f64)
// ============================================================================

/// SIMD-accelerated cosine similarity for f64
/// Returns: 1.0 = identical, 0.0 = orthogonal, -1.0 = opposite
#[inline]
pub fn cosine_similarity_f64(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    match f64::cosine(a, b) {
        Some(distance) => 1.0 - distance,
        None => cosine_similarity_scalar_f64(a, b),
    }
}

/// SIMD-accelerated cosine distance for f64
/// Returns: 0.0 = identical, 1.0 = orthogonal, 2.0 = opposite
#[inline]
pub fn cosine_distance_f64(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return 1.0;
    }
    match f64::cosine(a, b) {
        Some(distance) => distance,
        None => 1.0 - cosine_similarity_scalar_f64(a, b),
    }
}

/// SIMD-accelerated dot product for f64
#[inline]
pub fn dot_product_f64(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    match f64::dot(a, b) {
        Some(product) => product,
        None => dot_product_scalar_f64(a, b),
    }
}

/// SIMD-accelerated L2 squared distance for f64
#[inline]
pub fn l2_squared_f64(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return f64::MAX;
    }
    match f64::sqeuclidean(a, b) {
        Some(distance) => distance,
        None => l2_squared_scalar_f64(a, b),
    }
}

/// SIMD-accelerated L2 distance for f64
#[inline]
pub fn l2_distance_f64(a: &[f64], b: &[f64]) -> f64 {
    l2_squared_f64(a, b).sqrt()
}

// ============================================================================
// SIMD-Accelerated Similarity Operations (f32)
// ============================================================================

/// SIMD-accelerated cosine similarity for f32
#[inline]
pub fn cosine_similarity_f32(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    match f32::cosine(a, b) {
        Some(distance) => (1.0 - distance) as f32,
        None => cosine_similarity_scalar_f32(a, b),
    }
}

/// SIMD-accelerated dot product for f32
#[inline]
pub fn dot_product_f32(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    match f32::dot(a, b) {
        Some(product) => product as f32,
        None => dot_product_scalar_f32(a, b),
    }
}

/// SIMD-accelerated L2 squared distance for f32
#[inline]
pub fn l2_squared_f32(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return f32::MAX;
    }
    match f32::sqeuclidean(a, b) {
        Some(distance) => distance as f32,
        None => l2_squared_scalar_f32(a, b),
    }
}

// ============================================================================
// Batch Operations (Vectorized State Evolution)
// ============================================================================

/// Evolve multiple states in parallel using SIMD
/// Formula: state[i] = state[i] * transition[i] + noise[i]
/// Returns the number of operations performed
#[inline]
pub fn evolve_states(states: &mut [f64], transition: &[f64], noise: &[f64]) -> usize {
    let len = states.len().min(transition.len()).min(noise.len());

    // Process 4 elements at a time (unrolled loop)
    let chunks = len / 4;
    for i in 0..chunks {
        let base = i * 4;
        states[base] = states[base] * transition[base] + noise[base];
        states[base + 1] = states[base + 1] * transition[base + 1] + noise[base + 1];
        states[base + 2] = states[base + 2] * transition[base + 2] + noise[base + 2];
        states[base + 3] = states[base + 3] * transition[base + 3] + noise[base + 3];
    }

    // Handle remainder
    for i in (chunks * 4)..len {
        states[i] = states[i] * transition[i] + noise[i];
    }

    len
}

/// Weighted combination of multiple vectors
/// Formula: output[i] = sum(weights[j] * vectors[j][i])
#[inline]
pub fn weighted_combine(vectors: &[&[f64]], weights: &[f64], output: &mut [f64]) -> usize {
    if vectors.is_empty() || weights.len() != vectors.len() || output.is_empty() {
        return 0;
    }

    let len = vectors.iter().map(|v| v.len()).min().unwrap_or(0).min(output.len());

    // Zero output
    for val in output.iter_mut().take(len) {
        *val = 0.0;
    }

    // Accumulate weighted contributions
    for (vec, &weight) in vectors.iter().zip(weights.iter()) {
        for i in 0..len {
            output[i] += weight * vec[i];
        }
    }

    len
}

/// Batch cosine similarity: compute similarity between one vector and many
/// Returns similarities for each comparison
pub fn batch_cosine_similarity_f64(query: &[f64], vectors: &[&[f64]]) -> Vec<f64> {
    vectors
        .iter()
        .map(|v| cosine_similarity_f64(query, v))
        .collect()
}

/// Sum reduction with SIMD
#[inline]
pub fn sum_f64(values: &[f64]) -> f64 {
    // Process 4 at a time
    let chunks = values.len() / 4;
    let mut sum = 0.0;

    for i in 0..chunks {
        let base = i * 4;
        sum += values[base] + values[base + 1] + values[base + 2] + values[base + 3];
    }

    for i in (chunks * 4)..values.len() {
        sum += values[i];
    }

    sum
}

/// Mean with SIMD
#[inline]
pub fn mean_f64(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    sum_f64(values) / values.len() as f64
}

/// Variance with SIMD
#[inline]
pub fn variance_f64(values: &[f64]) -> f64 {
    if values.len() < 2 {
        return 0.0;
    }

    let mean = mean_f64(values);
    let sum_sq: f64 = values.iter().map(|&x| (x - mean) * (x - mean)).sum();
    sum_sq / values.len() as f64
}

/// Normalize vector in place
#[inline]
pub fn normalize_f64(values: &mut [f64]) {
    let norm: f64 = values.iter().map(|&x| x * x).sum::<f64>().sqrt();
    if norm > 0.0 {
        for val in values.iter_mut() {
            *val /= norm;
        }
    }
}

// ============================================================================
// Scalar Fallbacks
// ============================================================================

fn cosine_similarity_scalar_f64(a: &[f64], b: &[f64]) -> f64 {
    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for (&x, &y) in a.iter().zip(b.iter()) {
        dot += x * y;
        norm_a += x * x;
        norm_b += y * y;
    }

    let denom = (norm_a * norm_b).sqrt();
    if denom > 0.0 {
        dot / denom
    } else {
        0.0
    }
}

fn cosine_similarity_scalar_f32(a: &[f32], b: &[f32]) -> f32 {
    let mut dot = 0.0f32;
    let mut norm_a = 0.0f32;
    let mut norm_b = 0.0f32;

    for (&x, &y) in a.iter().zip(b.iter()) {
        dot += x * y;
        norm_a += x * x;
        norm_b += y * y;
    }

    let denom = (norm_a * norm_b).sqrt();
    if denom > 0.0 {
        dot / denom
    } else {
        0.0
    }
}

fn dot_product_scalar_f64(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(&x, &y)| x * y).sum()
}

fn dot_product_scalar_f32(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(&x, &y)| x * y).sum()
}

fn l2_squared_scalar_f64(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(&x, &y)| (x - y) * (x - y))
        .sum()
}

fn l2_squared_scalar_f32(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(&x, &y)| (x - y) * (x - y))
        .sum()
}

// ============================================================================
// Strange Loop Specific Operations
// ============================================================================

/// Compute consciousness delta using SIMD
/// Higher similarity between input and reflection indicates stronger strange loop
#[inline]
pub fn consciousness_delta(input: &[f64], reflection: &[f64], meta_output: &[f64]) -> f64 {
    let self_similarity = cosine_similarity_f64(input, reflection);
    let meta_similarity = cosine_similarity_f64(input, meta_output);

    // Consciousness emerges from self-reference + meta-awareness
    (self_similarity * 0.6 + meta_similarity * 0.4).max(0.0).min(1.0)
}

/// Compute strange loop strength from multiple perspectives
#[inline]
pub fn loop_strength(
    input: &[f64],
    reflection: &[f64],
    historical_avg: &[f64],
) -> f64 {
    let current_similarity = cosine_similarity_f64(input, reflection);
    let historical_similarity = cosine_similarity_f64(input, historical_avg);

    // Loop strength is how much current input matches both reflection AND history
    // This indicates a stable self-referential pattern
    (current_similarity + historical_similarity) / 2.0
}

/// Batch process multiple loop detections
pub fn batch_loop_detection(
    inputs: &[&[f64]],
    reflections: &[&[f64]],
    threshold: f64,
) -> Vec<bool> {
    inputs
        .iter()
        .zip(reflections.iter())
        .map(|(input, reflection)| cosine_similarity_f64(input, reflection) > threshold)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simd_detection() {
        let level = SimdLevel::detect();
        println!("Detected SIMD level: {} (width: {})", level.name(), level.width());
        assert!(level.width() >= 1);
    }

    #[test]
    fn test_cosine_similarity_f64() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        let c = vec![0.0, 1.0, 0.0];

        // Identical vectors = 1.0
        let sim = cosine_similarity_f64(&a, &b);
        assert!((sim - 1.0).abs() < 0.001);

        // Orthogonal vectors = 0.0
        let sim = cosine_similarity_f64(&a, &c);
        assert!(sim.abs() < 0.001);
    }

    #[test]
    fn test_dot_product_f64() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];

        let dot = dot_product_f64(&a, &b);
        assert!((dot - 32.0).abs() < 0.001); // 1*4 + 2*5 + 3*6 = 32
    }

    #[test]
    fn test_l2_distance_f64() {
        let a = vec![0.0, 0.0, 0.0];
        let b = vec![3.0, 4.0, 0.0];

        let dist = l2_distance_f64(&a, &b);
        assert!((dist - 5.0).abs() < 0.001); // sqrt(9 + 16) = 5
    }

    #[test]
    fn test_evolve_states() {
        let mut states = vec![1.0, 2.0, 3.0, 4.0];
        let transition = vec![0.5, 0.5, 0.5, 0.5];
        let noise = vec![0.1, 0.1, 0.1, 0.1];

        let processed = evolve_states(&mut states, &transition, &noise);
        assert_eq!(processed, 4);
        assert!((states[0] - 0.6).abs() < 0.001); // 1.0 * 0.5 + 0.1
    }

    #[test]
    fn test_consciousness_delta() {
        let input = vec![0.5; 10];
        let reflection = vec![0.5; 10]; // Perfect reflection
        let meta = vec![0.4; 10];

        let delta = consciousness_delta(&input, &reflection, &meta);
        assert!(delta > 0.8); // High self-similarity = high consciousness
    }

    #[test]
    fn test_batch_loop_detection() {
        let input1 = vec![1.0, 0.0, 0.0];
        let input2 = vec![0.0, 1.0, 0.0];
        let reflection1 = vec![1.0, 0.0, 0.0]; // Match
        let reflection2 = vec![0.0, 0.0, 1.0]; // No match

        let inputs: Vec<&[f64]> = vec![&input1, &input2];
        let reflections: Vec<&[f64]> = vec![&reflection1, &reflection2];

        let results = batch_loop_detection(&inputs, &reflections, 0.5);
        assert_eq!(results, vec![true, false]);
    }
}
