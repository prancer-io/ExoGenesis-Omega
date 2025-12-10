//! SIMD-Optimized Vector Operations
//!
//! High-performance distance and similarity computations:
//! - Cosine similarity/distance
//! - Euclidean (L2) distance
//! - Dot product (inner product)
//! - Manhattan (L1) distance
//! - Batch operations

use simsimd::SpatialSimilarity;

/// SIMD-accelerated cosine similarity
/// Returns: 1.0 = identical, 0.0 = orthogonal, -1.0 = opposite
#[inline]
pub fn cosine_similarity_f32(a: &[f32], b: &[f32]) -> f32 {
    match f32::cosine(a, b) {
        Some(distance) => 1.0 - distance,
        None => 0.0,
    }
}

/// SIMD-accelerated cosine distance
/// Returns: 0.0 = identical, 1.0 = orthogonal, 2.0 = opposite
#[inline]
pub fn cosine_distance_f32(a: &[f32], b: &[f32]) -> f32 {
    match f32::cosine(a, b) {
        Some(distance) => distance,
        None => 1.0,
    }
}

/// SIMD-accelerated L2 (Euclidean) squared distance
#[inline]
pub fn l2_squared_f32(a: &[f32], b: &[f32]) -> f32 {
    match f32::sqeuclidean(a, b) {
        Some(distance) => distance,
        None => f32::MAX,
    }
}

/// SIMD-accelerated L2 (Euclidean) distance
#[inline]
pub fn l2_distance_f32(a: &[f32], b: &[f32]) -> f32 {
    l2_squared_f32(a, b).sqrt()
}

/// SIMD-accelerated dot product (inner product)
#[inline]
pub fn dot_product_f32(a: &[f32], b: &[f32]) -> f32 {
    match f32::dot(a, b) {
        Some(product) => product,
        None => 0.0,
    }
}

/// Distance metric types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistanceMetric {
    /// Cosine distance (1 - cosine similarity)
    Cosine,
    /// L2 (Euclidean) distance
    L2,
    /// L2 squared (faster, avoids sqrt)
    L2Squared,
    /// Inner product (dot product)
    InnerProduct,
    /// Manhattan (L1) distance - scalar fallback
    Manhattan,
}

impl DistanceMetric {
    /// Compute distance between two vectors
    #[inline]
    pub fn compute(&self, a: &[f32], b: &[f32]) -> f32 {
        match self {
            Self::Cosine => cosine_distance_f32(a, b),
            Self::L2 => l2_distance_f32(a, b),
            Self::L2Squared => l2_squared_f32(a, b),
            Self::InnerProduct => -dot_product_f32(a, b), // Negative for min-search
            Self::Manhattan => manhattan_distance_f32(a, b),
        }
    }

    /// Convert distance to similarity (for ranking)
    #[inline]
    pub fn to_similarity(&self, distance: f32) -> f32 {
        match self {
            Self::Cosine => 1.0 - distance,
            Self::L2 | Self::L2Squared | Self::Manhattan => 1.0 / (1.0 + distance),
            Self::InnerProduct => -distance, // Already negated
        }
    }
}

/// Manhattan (L1) distance - scalar implementation
pub fn manhattan_distance_f32(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(&x, &y)| (x - y).abs())
        .sum()
}

/// Batch cosine similarities - compute similarity of query against multiple vectors
pub fn batch_cosine_similarity(query: &[f32], vectors: &[&[f32]]) -> Vec<f32> {
    vectors
        .iter()
        .map(|v| cosine_similarity_f32(query, v))
        .collect()
}

/// Batch L2 distances - compute L2 distance of query against multiple vectors
pub fn batch_l2_distance(query: &[f32], vectors: &[&[f32]]) -> Vec<f32> {
    vectors.iter().map(|v| l2_distance_f32(query, v)).collect()
}

/// Find top-k most similar vectors
pub fn top_k_similar(
    query: &[f32],
    vectors: &[(&str, &[f32])],
    k: usize,
    metric: DistanceMetric,
) -> Vec<(String, f32)> {
    let mut scored: Vec<(String, f32)> = vectors
        .iter()
        .map(|(id, v)| {
            let dist = metric.compute(query, v);
            let sim = metric.to_similarity(dist);
            (id.to_string(), sim)
        })
        .collect();

    // Sort by similarity descending
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    scored.truncate(k);
    scored
}

/// Vector normalization for cosine similarity
pub fn normalize_f32(v: &mut [f32]) {
    let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 1e-10 {
        for x in v.iter_mut() {
            *x /= norm;
        }
    }
}

/// Check if vector is normalized
pub fn is_normalized(v: &[f32], tolerance: f32) -> bool {
    let norm: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    (norm - 1.0).abs() < tolerance
}

/// Compute vector magnitude (L2 norm)
pub fn magnitude_f32(v: &[f32]) -> f32 {
    v.iter().map(|x| x * x).sum::<f32>().sqrt()
}

/// Compute centroid of multiple vectors
pub fn centroid(vectors: &[&[f32]]) -> Vec<f32> {
    if vectors.is_empty() {
        return Vec::new();
    }

    let dim = vectors[0].len();
    let mut result = vec![0.0f32; dim];
    let n = vectors.len() as f32;

    for v in vectors {
        for (i, &x) in v.iter().enumerate() {
            if i < dim {
                result[i] += x;
            }
        }
    }

    for x in &mut result {
        *x /= n;
    }

    result
}

/// Compute variance of vectors around their centroid
pub fn variance(vectors: &[&[f32]], centroid: &[f32]) -> f32 {
    if vectors.is_empty() {
        return 0.0;
    }

    vectors
        .iter()
        .map(|v| l2_squared_f32(v, centroid))
        .sum::<f32>()
        / vectors.len() as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let a = [1.0, 0.0, 0.0];
        let b = [1.0, 0.0, 0.0];
        assert!((cosine_similarity_f32(&a, &b) - 1.0).abs() < 0.001);

        let c = [1.0, 0.0, 0.0];
        let d = [0.0, 1.0, 0.0];
        assert!(cosine_similarity_f32(&c, &d).abs() < 0.001);
    }

    #[test]
    fn test_l2_distance() {
        let a = [0.0, 0.0, 0.0];
        let b = [3.0, 4.0, 0.0];
        assert!((l2_distance_f32(&a, &b) - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_dot_product() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
        assert!((dot_product_f32(&a, &b) - 32.0).abs() < 0.001);
    }

    #[test]
    fn test_manhattan_distance() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 6.0, 3.0];
        // |1-4| + |2-6| + |3-3| = 3 + 4 + 0 = 7
        assert!((manhattan_distance_f32(&a, &b) - 7.0).abs() < 0.001);
    }

    #[test]
    fn test_normalize() {
        let mut v = [3.0, 4.0, 0.0];
        normalize_f32(&mut v);
        assert!((magnitude_f32(&v) - 1.0).abs() < 0.001);
        assert!(is_normalized(&v, 0.001));
    }

    #[test]
    fn test_top_k() {
        let query = [1.0, 0.0, 0.0];
        let vectors: Vec<(&str, &[f32])> = vec![
            ("a", &[1.0, 0.0, 0.0][..]),
            ("b", &[0.0, 1.0, 0.0][..]),
            ("c", &[0.7, 0.7, 0.0][..]),
        ];

        let result = top_k_similar(&query, &vectors, 2, DistanceMetric::Cosine);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, "a"); // Most similar
    }

    #[test]
    fn test_centroid() {
        let v1 = [0.0, 0.0];
        let v2 = [2.0, 2.0];
        let vectors: Vec<&[f32]> = vec![&v1, &v2];

        let c = centroid(&vectors);
        assert!((c[0] - 1.0).abs() < 0.001);
        assert!((c[1] - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_distance_metrics() {
        let a = [1.0, 0.0, 0.0];
        let b = [0.0, 1.0, 0.0];

        let cosine_dist = DistanceMetric::Cosine.compute(&a, &b);
        assert!((cosine_dist - 1.0).abs() < 0.001);

        let l2_dist = DistanceMetric::L2.compute(&a, &b);
        assert!((l2_dist - std::f32::consts::SQRT_2).abs() < 0.001);
    }
}
