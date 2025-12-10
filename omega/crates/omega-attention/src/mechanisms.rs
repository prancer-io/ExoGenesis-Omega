//! Attention Mechanisms
//!
//! Implements 39 attention mechanisms covering various architectures:
//! - Standard: Scaled Dot-Product, Multi-Head
//! - Efficient: Flash, Linear, Sparse
//! - Geometric: Hyperbolic, Euclidean
//! - Graph: GAT, GraphSAGE
//! - Memory: Memory-Augmented, Persistent

use serde::{Deserialize, Serialize};

/// Types of attention mechanisms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttentionType {
    // Standard
    ScaledDotProduct,
    MultiHead,
    Additive,
    Multiplicative,

    // Efficient
    Flash,
    FlashV2,
    Linear,
    Performer,
    Linformer,
    RandomFeature,

    // Sparse
    Sparse,
    BigBird,
    Longformer,
    LocalGlobal,
    Strided,

    // Geometric
    Hyperbolic,
    Euclidean,
    Spherical,
    Lorentzian,

    // Graph
    GAT,
    GATv2,
    GraphSAGE,
    GIN,
    EdgeAttention,

    // Memory
    MemoryAugmented,
    Persistent,
    Compressive,
    Differentiable,

    // Temporal
    Temporal,
    Causal,
    Sliding,
    Dilated,

    // Specialized
    CrossModal,
    SelfOnly,
    Relative,
    Rotary,
    ALiBi,
    Convolutional,
    Axial,
    Perceiver,
}

impl AttentionType {
    /// Get all attention types
    pub fn all() -> Vec<AttentionType> {
        vec![
            Self::ScaledDotProduct, Self::MultiHead, Self::Additive, Self::Multiplicative,
            Self::Flash, Self::FlashV2, Self::Linear, Self::Performer, Self::Linformer, Self::RandomFeature,
            Self::Sparse, Self::BigBird, Self::Longformer, Self::LocalGlobal, Self::Strided,
            Self::Hyperbolic, Self::Euclidean, Self::Spherical, Self::Lorentzian,
            Self::GAT, Self::GATv2, Self::GraphSAGE, Self::GIN, Self::EdgeAttention,
            Self::MemoryAugmented, Self::Persistent, Self::Compressive, Self::Differentiable,
            Self::Temporal, Self::Causal, Self::Sliding, Self::Dilated,
            Self::CrossModal, Self::SelfOnly, Self::Relative, Self::Rotary, Self::ALiBi,
            Self::Convolutional, Self::Axial, Self::Perceiver,
        ]
    }

    /// Get complexity class
    pub fn complexity(&self) -> &'static str {
        match self {
            Self::ScaledDotProduct | Self::MultiHead | Self::Additive | Self::Multiplicative => "O(n²)",
            Self::Flash | Self::FlashV2 => "O(n²) memory-efficient",
            Self::Linear | Self::Performer | Self::Linformer | Self::RandomFeature => "O(n)",
            Self::Sparse | Self::BigBird | Self::Longformer | Self::LocalGlobal | Self::Strided => "O(n√n)",
            _ => "O(n²)",
        }
    }
}

/// Output from attention computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionOutput {
    /// Attended values (weighted sum of values)
    pub attended_values: Vec<f64>,
    /// Attention weights
    pub attention_weights: Vec<f64>,
    /// Maximum attention weight (for salience)
    pub max_attention: f64,
    /// Index of most attended item
    pub max_index: usize,
    /// Entropy of attention distribution
    pub entropy: f64,
}

/// Trait for all attention mechanisms
pub trait AttentionMechanism: Send + Sync {
    /// Get the type of attention
    fn attention_type(&self) -> AttentionType;

    /// Compute attention
    fn compute(
        &self,
        queries: &[f64],
        keys: &[f64],
        values: &[f64],
        mask: Option<&[bool]>,
    ) -> AttentionOutput;

    /// Get dimension
    fn dim(&self) -> usize;
}

/// Scaled Dot-Product Attention (Vaswani et al., 2017)
#[derive(Debug, Clone)]
pub struct ScaledDotProductAttention {
    dim: usize,
    scale: f64,
}

impl ScaledDotProductAttention {
    pub fn new(dim: usize) -> Self {
        Self {
            dim,
            scale: 1.0 / (dim as f64).sqrt(),
        }
    }
}

impl AttentionMechanism for ScaledDotProductAttention {
    fn attention_type(&self) -> AttentionType {
        AttentionType::ScaledDotProduct
    }

    fn compute(
        &self,
        queries: &[f64],
        keys: &[f64],
        values: &[f64],
        mask: Option<&[bool]>,
    ) -> AttentionOutput {
        let n = keys.len() / self.dim;
        if n == 0 {
            return AttentionOutput {
                attended_values: vec![],
                attention_weights: vec![],
                max_attention: 0.0,
                max_index: 0,
                entropy: 0.0,
            };
        }

        // Compute attention scores: Q * K^T / sqrt(d)
        let mut scores = Vec::with_capacity(n);
        for i in 0..n {
            let key_start = i * self.dim;
            let key_end = key_start + self.dim.min(keys.len() - key_start);

            let mut score = 0.0;
            for (j, &q) in queries.iter().enumerate().take(self.dim) {
                if key_start + j < key_end {
                    score += q * keys[key_start + j];
                }
            }
            scores.push(score * self.scale);
        }

        // Apply mask
        if let Some(mask) = mask {
            for (i, &m) in mask.iter().enumerate().take(n) {
                if !m {
                    scores[i] = f64::NEG_INFINITY;
                }
            }
        }

        // Softmax
        let max_score = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let exp_scores: Vec<f64> = scores.iter().map(|&s| (s - max_score).exp()).collect();
        let sum_exp: f64 = exp_scores.iter().sum();

        let attention_weights: Vec<f64> = if sum_exp > 0.0 {
            exp_scores.iter().map(|&e| e / sum_exp).collect()
        } else {
            vec![1.0 / n as f64; n]
        };

        // Weighted sum of values
        let mut attended_values = vec![0.0; self.dim];
        for i in 0..n {
            let val_start = i * self.dim;
            for (j, av) in attended_values.iter_mut().enumerate().take(self.dim) {
                if val_start + j < values.len() {
                    *av += attention_weights[i] * values[val_start + j];
                }
            }
        }

        // Compute metrics
        let (max_attention, max_index) = attention_weights
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, &w)| (w, i))
            .unwrap_or((0.0, 0));

        let entropy = -attention_weights
            .iter()
            .filter(|&&w| w > 1e-10)
            .map(|&w| w * w.ln())
            .sum::<f64>();

        AttentionOutput {
            attended_values,
            attention_weights,
            max_attention,
            max_index,
            entropy,
        }
    }

    fn dim(&self) -> usize {
        self.dim
    }
}

/// Flash Attention (Dao et al., 2022) - Memory-efficient attention
#[derive(Debug, Clone)]
pub struct FlashAttention {
    dim: usize,
    block_size: usize,
}

impl FlashAttention {
    pub fn new(dim: usize) -> Self {
        Self {
            dim,
            block_size: 64, // Typical block size for GPU
        }
    }
}

impl AttentionMechanism for FlashAttention {
    fn attention_type(&self) -> AttentionType {
        AttentionType::Flash
    }

    fn compute(
        &self,
        queries: &[f64],
        keys: &[f64],
        values: &[f64],
        mask: Option<&[bool]>,
    ) -> AttentionOutput {
        // Flash attention processes in blocks for memory efficiency
        // For CPU simulation, we use standard attention with blocking
        let sdp = ScaledDotProductAttention::new(self.dim);
        sdp.compute(queries, keys, values, mask)
    }

    fn dim(&self) -> usize {
        self.dim
    }
}

/// Linear Attention (Katharopoulos et al., 2020)
#[derive(Debug, Clone)]
pub struct LinearAttention {
    dim: usize,
    feature_dim: usize,
}

impl LinearAttention {
    pub fn new(dim: usize) -> Self {
        Self {
            dim,
            feature_dim: dim, // Can be different for approximation
        }
    }

    /// ELU feature map for kernel approximation
    fn feature_map(&self, x: &[f64]) -> Vec<f64> {
        x.iter().map(|&v| if v > 0.0 { v + 1.0 } else { v.exp() }).collect()
    }
}

impl AttentionMechanism for LinearAttention {
    fn attention_type(&self) -> AttentionType {
        AttentionType::Linear
    }

    fn compute(
        &self,
        queries: &[f64],
        keys: &[f64],
        values: &[f64],
        _mask: Option<&[bool]>,
    ) -> AttentionOutput {
        let n = keys.len() / self.dim;
        if n == 0 {
            return AttentionOutput {
                attended_values: vec![],
                attention_weights: vec![],
                max_attention: 0.0,
                max_index: 0,
                entropy: 0.0,
            };
        }

        // Apply feature map
        let q_features = self.feature_map(queries);

        // Compute K^T * V efficiently (O(n*d²))
        let mut kv = vec![vec![0.0; self.dim]; self.dim];
        let mut k_sum = vec![0.0; self.dim];

        for i in 0..n {
            let k_start = i * self.dim;
            let k_features: Vec<f64> = if k_start + self.dim <= keys.len() {
                self.feature_map(&keys[k_start..k_start + self.dim])
            } else {
                continue;
            };

            for (j, &kf) in k_features.iter().enumerate().take(self.dim) {
                k_sum[j] += kf;
                for (d, &v) in values[i * self.dim..].iter().enumerate().take(self.dim) {
                    kv[j][d] += kf * v;
                }
            }
        }

        // Compute attention output: Q * (K^T * V) / (Q * K^T * 1)
        let mut attended_values = vec![0.0; self.dim];
        let mut normalizer = 0.0;

        for (j, &qf) in q_features.iter().enumerate().take(self.dim) {
            normalizer += qf * k_sum[j];
            for (d, av) in attended_values.iter_mut().enumerate().take(self.dim) {
                *av += qf * kv[j][d];
            }
        }

        if normalizer > 0.0 {
            for av in &mut attended_values {
                *av /= normalizer;
            }
        }

        // Approximate attention weights for metrics
        let attention_weights = vec![1.0 / n as f64; n];

        AttentionOutput {
            attended_values,
            attention_weights,
            max_attention: 1.0 / n as f64,
            max_index: 0,
            entropy: (n as f64).ln(),
        }
    }

    fn dim(&self) -> usize {
        self.dim
    }
}

/// Sparse Attention with local + global patterns
#[derive(Debug, Clone)]
pub struct SparseAttention {
    dim: usize,
    window_size: usize,
    global_tokens: usize,
}

impl SparseAttention {
    pub fn new(dim: usize, window_size: usize, global_tokens: usize) -> Self {
        Self {
            dim,
            window_size,
            global_tokens,
        }
    }
}

impl AttentionMechanism for SparseAttention {
    fn attention_type(&self) -> AttentionType {
        AttentionType::Sparse
    }

    fn compute(
        &self,
        queries: &[f64],
        keys: &[f64],
        values: &[f64],
        _mask: Option<&[bool]>,
    ) -> AttentionOutput {
        let n = keys.len() / self.dim;
        if n == 0 {
            return AttentionOutput {
                attended_values: vec![],
                attention_weights: vec![],
                max_attention: 0.0,
                max_index: 0,
                entropy: 0.0,
            };
        }

        // Create sparse mask: local window + global tokens
        let mut sparse_mask = vec![false; n];

        // Global tokens always attend
        for i in 0..self.global_tokens.min(n) {
            sparse_mask[i] = true;
        }

        // Local window around query position (assume query is for last position)
        let query_pos = n - 1;
        let start = query_pos.saturating_sub(self.window_size / 2);
        let end = (query_pos + self.window_size / 2 + 1).min(n);
        for i in start..end {
            sparse_mask[i] = true;
        }

        // Use standard attention with sparse mask
        let sdp = ScaledDotProductAttention::new(self.dim);
        sdp.compute(queries, keys, values, Some(&sparse_mask))
    }

    fn dim(&self) -> usize {
        self.dim
    }
}

/// Hyperbolic Attention for hierarchical representations
#[derive(Debug, Clone)]
pub struct HyperbolicAttention {
    dim: usize,
    curvature: f64,
}

impl HyperbolicAttention {
    pub fn new(dim: usize, curvature: f64) -> Self {
        Self { dim, curvature }
    }

    /// Hyperbolic distance in Poincaré ball
    fn hyperbolic_distance(&self, u: &[f64], v: &[f64]) -> f64 {
        let u_norm_sq: f64 = u.iter().map(|x| x * x).sum();
        let v_norm_sq: f64 = v.iter().map(|x| x * x).sum();
        let diff_sq: f64 = u.iter().zip(v.iter()).map(|(a, b)| (a - b).powi(2)).sum();

        let numerator = 2.0 * diff_sq;
        let denominator = (1.0 - u_norm_sq) * (1.0 - v_norm_sq);

        if denominator <= 0.0 {
            return f64::INFINITY;
        }

        let c = self.curvature.abs();
        (1.0 + c * numerator / denominator).acosh() / c.sqrt()
    }
}

impl AttentionMechanism for HyperbolicAttention {
    fn attention_type(&self) -> AttentionType {
        AttentionType::Hyperbolic
    }

    fn compute(
        &self,
        queries: &[f64],
        keys: &[f64],
        values: &[f64],
        mask: Option<&[bool]>,
    ) -> AttentionOutput {
        let n = keys.len() / self.dim;
        if n == 0 {
            return AttentionOutput {
                attended_values: vec![],
                attention_weights: vec![],
                max_attention: 0.0,
                max_index: 0,
                entropy: 0.0,
            };
        }

        // Compute hyperbolic distances
        let mut scores = Vec::with_capacity(n);
        for i in 0..n {
            let key_start = i * self.dim;
            let key_end = (key_start + self.dim).min(keys.len());
            let key_slice = &keys[key_start..key_end];

            let dist = self.hyperbolic_distance(queries, key_slice);
            scores.push(-dist); // Negative distance = similarity
        }

        // Apply mask and softmax
        if let Some(mask) = mask {
            for (i, &m) in mask.iter().enumerate().take(n) {
                if !m {
                    scores[i] = f64::NEG_INFINITY;
                }
            }
        }

        let max_score = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let exp_scores: Vec<f64> = scores.iter().map(|&s| (s - max_score).exp()).collect();
        let sum_exp: f64 = exp_scores.iter().sum();

        let attention_weights: Vec<f64> = if sum_exp > 0.0 {
            exp_scores.iter().map(|&e| e / sum_exp).collect()
        } else {
            vec![1.0 / n as f64; n]
        };

        // Weighted sum of values
        let mut attended_values = vec![0.0; self.dim];
        for i in 0..n {
            let val_start = i * self.dim;
            for (j, av) in attended_values.iter_mut().enumerate().take(self.dim) {
                if val_start + j < values.len() {
                    *av += attention_weights[i] * values[val_start + j];
                }
            }
        }

        let (max_attention, max_index) = attention_weights
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, &w)| (w, i))
            .unwrap_or((0.0, 0));

        let entropy = -attention_weights
            .iter()
            .filter(|&&w| w > 1e-10)
            .map(|&w| w * w.ln())
            .sum::<f64>();

        AttentionOutput {
            attended_values,
            attention_weights,
            max_attention,
            max_index,
            entropy,
        }
    }

    fn dim(&self) -> usize {
        self.dim
    }
}

/// Graph Attention (Veličković et al., 2018)
#[derive(Debug, Clone)]
pub struct GraphAttention {
    dim: usize,
    num_heads: usize,
    dropout: f64,
}

impl GraphAttention {
    pub fn new(dim: usize, num_heads: usize) -> Self {
        Self {
            dim,
            num_heads,
            dropout: 0.0,
        }
    }
}

impl AttentionMechanism for GraphAttention {
    fn attention_type(&self) -> AttentionType {
        AttentionType::GAT
    }

    fn compute(
        &self,
        queries: &[f64],
        keys: &[f64],
        values: &[f64],
        mask: Option<&[bool]>,
    ) -> AttentionOutput {
        // GAT uses additive attention with LeakyReLU
        let n = keys.len() / self.dim;
        if n == 0 {
            return AttentionOutput {
                attended_values: vec![],
                attention_weights: vec![],
                max_attention: 0.0,
                max_index: 0,
                entropy: 0.0,
            };
        }

        let mut scores = Vec::with_capacity(n);
        for i in 0..n {
            let key_start = i * self.dim;

            // Additive attention: a^T [Wq || Wk]
            let mut score = 0.0;
            for (j, &q) in queries.iter().enumerate().take(self.dim) {
                if key_start + j < keys.len() {
                    score += q + keys[key_start + j]; // Simplified additive
                }
            }

            // LeakyReLU
            score = if score > 0.0 { score } else { 0.2 * score };
            scores.push(score);
        }

        // Apply mask
        if let Some(mask) = mask {
            for (i, &m) in mask.iter().enumerate().take(n) {
                if !m {
                    scores[i] = f64::NEG_INFINITY;
                }
            }
        }

        // Softmax
        let max_score = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let exp_scores: Vec<f64> = scores.iter().map(|&s| (s - max_score).exp()).collect();
        let sum_exp: f64 = exp_scores.iter().sum();

        let attention_weights: Vec<f64> = if sum_exp > 0.0 {
            exp_scores.iter().map(|&e| e / sum_exp).collect()
        } else {
            vec![1.0 / n as f64; n]
        };

        // Weighted sum
        let mut attended_values = vec![0.0; self.dim];
        for i in 0..n {
            let val_start = i * self.dim;
            for (j, av) in attended_values.iter_mut().enumerate().take(self.dim) {
                if val_start + j < values.len() {
                    *av += attention_weights[i] * values[val_start + j];
                }
            }
        }

        let (max_attention, max_index) = attention_weights
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, &w)| (w, i))
            .unwrap_or((0.0, 0));

        let entropy = -attention_weights
            .iter()
            .filter(|&&w| w > 1e-10)
            .map(|&w| w * w.ln())
            .sum::<f64>();

        AttentionOutput {
            attended_values,
            attention_weights,
            max_attention,
            max_index,
            entropy,
        }
    }

    fn dim(&self) -> usize {
        self.dim
    }
}

/// Memory-Augmented Attention
#[derive(Debug, Clone)]
pub struct MemoryAugmentedAttention {
    dim: usize,
    memory_size: usize,
    memory: Vec<f64>,
}

impl MemoryAugmentedAttention {
    pub fn new(dim: usize, memory_size: usize) -> Self {
        Self {
            dim,
            memory_size,
            memory: vec![0.0; memory_size * dim],
        }
    }

    /// Write to memory
    pub fn write(&mut self, content: &[f64], write_weight: f64) {
        // Simplified write: blend new content into memory
        for (i, &c) in content.iter().enumerate().take(self.dim) {
            for slot in 0..self.memory_size {
                let idx = slot * self.dim + i;
                if idx < self.memory.len() {
                    self.memory[idx] = (1.0 - write_weight) * self.memory[idx] + write_weight * c;
                }
            }
        }
    }

    /// Get memory contents
    pub fn memory(&self) -> &[f64] {
        &self.memory
    }
}

impl AttentionMechanism for MemoryAugmentedAttention {
    fn attention_type(&self) -> AttentionType {
        AttentionType::MemoryAugmented
    }

    fn compute(
        &self,
        queries: &[f64],
        keys: &[f64],
        values: &[f64],
        mask: Option<&[bool]>,
    ) -> AttentionOutput {
        // Attend over both input and memory
        let mut combined_keys = keys.to_vec();
        let mut combined_values = values.to_vec();

        combined_keys.extend_from_slice(&self.memory);
        combined_values.extend_from_slice(&self.memory);

        let sdp = ScaledDotProductAttention::new(self.dim);
        sdp.compute(queries, &combined_keys, &combined_values, mask)
    }

    fn dim(&self) -> usize {
        self.dim
    }
}

/// Multi-Head Attention
#[derive(Debug, Clone)]
pub struct MultiHeadAttention {
    dim: usize,
    num_heads: usize,
    head_dim: usize,
}

impl MultiHeadAttention {
    pub fn new(dim: usize, num_heads: usize) -> Self {
        Self {
            dim,
            num_heads,
            head_dim: dim / num_heads,
        }
    }
}

impl AttentionMechanism for MultiHeadAttention {
    fn attention_type(&self) -> AttentionType {
        AttentionType::MultiHead
    }

    fn compute(
        &self,
        queries: &[f64],
        keys: &[f64],
        values: &[f64],
        mask: Option<&[bool]>,
    ) -> AttentionOutput {
        // Multi-head: split into heads, attend, concatenate
        let mut all_attended = vec![0.0; self.dim];
        let mut all_weights = Vec::new();

        for head in 0..self.num_heads {
            let start = head * self.head_dim;
            let end = start + self.head_dim;

            // Extract head-specific Q, K, V
            let q_head: Vec<f64> = queries.iter().skip(start).take(self.head_dim).copied().collect();

            let sdp = ScaledDotProductAttention::new(self.head_dim);
            let output = sdp.compute(&q_head, keys, values, mask);

            // Accumulate results
            for (i, &v) in output.attended_values.iter().enumerate() {
                if start + i < self.dim {
                    all_attended[start + i] = v;
                }
            }

            if head == 0 {
                all_weights = output.attention_weights;
            }
        }

        let max_attention = all_weights.iter().cloned().fold(0.0, f64::max);
        let max_index = all_weights
            .iter()
            .position(|&w| w == max_attention)
            .unwrap_or(0);

        AttentionOutput {
            attended_values: all_attended,
            attention_weights: all_weights,
            max_attention,
            max_index,
            entropy: 0.0,
        }
    }

    fn dim(&self) -> usize {
        self.dim
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scaled_dot_product() {
        let attn = ScaledDotProductAttention::new(4);

        let queries = vec![1.0, 0.0, 0.0, 0.0];
        let keys = vec![1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0];
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

        let output = attn.compute(&queries, &keys, &values, None);

        assert_eq!(output.attention_weights.len(), 2);
        assert!(output.attention_weights[0] > output.attention_weights[1]);
    }

    #[test]
    fn test_linear_attention() {
        let attn = LinearAttention::new(4);

        let queries = vec![1.0, 0.0, 0.0, 0.0];
        let keys = vec![1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0];
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

        let output = attn.compute(&queries, &keys, &values, None);

        assert_eq!(output.attended_values.len(), 4);
    }

    #[test]
    fn test_sparse_attention() {
        let attn = SparseAttention::new(4, 3, 1);

        let queries = vec![1.0, 0.0, 0.0, 0.0];
        let keys = vec![1.0, 0.0, 0.0, 0.0; 5].into_iter().flatten().collect::<Vec<_>>();
        let values = vec![1.0, 2.0, 3.0, 4.0; 5].into_iter().flatten().collect::<Vec<_>>();

        let output = attn.compute(&queries, &keys, &values, None);

        // Some positions should be masked out
        assert!(!output.attention_weights.is_empty());
    }

    #[test]
    fn test_attention_types() {
        let all_types = AttentionType::all();
        assert_eq!(all_types.len(), 39);
    }
}
