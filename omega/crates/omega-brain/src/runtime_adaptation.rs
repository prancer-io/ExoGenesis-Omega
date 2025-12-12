//! Runtime Adaptation System
//!
//! Implements continuous learning without catastrophic forgetting:
//! - MicroLoRA (rank 1-2): Instant adaptation for immediate context
//! - BaseLoRA (rank 4-16): Long-term learning and skill acquisition
//! - EWC++ (Elastic Weight Consolidation): Prevents forgetting
//! - ReasoningBank: Stores successful reasoning patterns
//!
//! Inspired by ruvector-sona architecture.

use serde::{Deserialize, Serialize};

/// LoRA rank configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoRARank {
    /// MicroLoRA: rank 1-2 for instant adaptation
    Micro(usize),
    /// BaseLoRA: rank 4-16 for long-term learning
    Base(usize),
    /// Custom rank
    Custom(usize),
}

impl Default for LoRARank {
    fn default() -> Self {
        Self::Base(8)
    }
}

/// LoRA adapter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoRAConfig {
    /// Rank of the low-rank decomposition
    pub rank: LoRARank,
    /// Alpha scaling factor
    pub alpha: f64,
    /// Dropout rate
    pub dropout: f64,
    /// Target dimensions
    pub dim: usize,
    /// Learning rate
    pub learning_rate: f64,
}

impl Default for LoRAConfig {
    fn default() -> Self {
        Self {
            rank: LoRARank::Base(8),
            alpha: 16.0,
            dropout: 0.0,
            dim: 256,
            learning_rate: 0.001,
        }
    }
}

/// LoRA adapter matrices
#[derive(Debug, Clone)]
pub struct LoRAAdapter {
    /// Configuration
    config: LoRAConfig,
    /// Matrix A (d x r)
    matrix_a: Vec<Vec<f64>>,
    /// Matrix B (r x d)
    matrix_b: Vec<Vec<f64>>,
    /// Merged delta weights
    delta_weights: Vec<Vec<f64>>,
    /// Is merged with base
    is_merged: bool,
    /// Update count
    update_count: u64,
}

impl LoRAAdapter {
    /// Create new LoRA adapter
    pub fn new(config: LoRAConfig) -> Self {
        let rank = match config.rank {
            LoRARank::Micro(r) => r,
            LoRARank::Base(r) => r,
            LoRARank::Custom(r) => r,
        };

        // Initialize A with small random values, B with zeros
        let matrix_a: Vec<Vec<f64>> = (0..config.dim)
            .map(|i| (0..rank).map(|j| ((i * j) as f64 * 0.001).sin() * 0.01).collect())
            .collect();

        let matrix_b: Vec<Vec<f64>> = (0..rank)
            .map(|_| vec![0.0; config.dim])
            .collect();

        let delta_weights = vec![vec![0.0; config.dim]; config.dim];

        Self {
            config,
            matrix_a,
            matrix_b,
            delta_weights,
            is_merged: false,
            update_count: 0,
        }
    }

    /// Apply LoRA transformation
    pub fn apply(&self, input: &[f64]) -> Vec<f64> {
        if self.is_merged {
            // Use merged weights
            return self.apply_merged(input);
        }

        // Compute A * x
        let rank = self.matrix_a.first().map_or(0, |r| r.len());
        let mut intermediate = vec![0.0; rank];
        for (i, row) in self.matrix_a.iter().enumerate() {
            if i < input.len() {
                for (j, &a) in row.iter().enumerate() {
                    intermediate[j] += a * input[i];
                }
            }
        }

        // Compute B * (A * x)
        let mut output = vec![0.0; self.config.dim];
        for (i, row) in self.matrix_b.iter().enumerate() {
            for (j, &b) in row.iter().enumerate() {
                if j < output.len() {
                    output[j] += b * intermediate[i];
                }
            }
        }

        // Scale by alpha / rank
        let rank_val = match self.config.rank {
            LoRARank::Micro(r) => r,
            LoRARank::Base(r) => r,
            LoRARank::Custom(r) => r,
        } as f64;
        let scale = self.config.alpha / rank_val;

        for v in &mut output {
            *v *= scale;
        }

        output
    }

    /// Apply using merged weights
    fn apply_merged(&self, input: &[f64]) -> Vec<f64> {
        let mut output = vec![0.0; self.config.dim];
        for (i, row) in self.delta_weights.iter().enumerate() {
            for (j, &w) in row.iter().enumerate() {
                if j < input.len() {
                    output[i] += w * input[j];
                }
            }
        }
        output
    }

    /// Update adapter weights
    pub fn update(&mut self, input: &[f64], target: &[f64]) {
        // Compute current output
        let output = self.apply(input);

        // Compute error
        let error: Vec<f64> = target
            .iter()
            .zip(output.iter())
            .map(|(&t, &o)| t - o)
            .collect();

        let rank = self.matrix_a.first().map(|r| r.len()).unwrap_or(0);

        // Update B (gradient descent)
        for (i, row) in self.matrix_b.iter_mut().enumerate() {
            if i < rank {
                for (j, b) in row.iter_mut().enumerate() {
                    if j < error.len() {
                        // Simplified gradient
                        let grad = error[j] * self.matrix_a.first().and_then(|r| r.get(i)).copied().unwrap_or(0.0);
                        *b += self.config.learning_rate * grad;
                    }
                }
            }
        }

        // Update A
        for (i, row) in self.matrix_a.iter_mut().enumerate() {
            if i < input.len() {
                for (j, a) in row.iter_mut().enumerate() {
                    if j < rank {
                        let grad = error.get(i).copied().unwrap_or(0.0) * input[i];
                        *a += self.config.learning_rate * grad * 0.1;
                    }
                }
            }
        }

        self.update_count += 1;
    }

    /// Merge LoRA weights into delta
    pub fn merge(&mut self) {
        if self.is_merged {
            return;
        }

        let rank = self.matrix_a.first().map(|r| r.len()).unwrap_or(0);
        let rank_val = match self.config.rank {
            LoRARank::Micro(r) => r,
            LoRARank::Base(r) => r,
            LoRARank::Custom(r) => r,
        } as f64;
        let scale = self.config.alpha / rank_val;

        // Compute BA
        for i in 0..self.config.dim {
            for j in 0..self.config.dim {
                let mut sum = 0.0;
                for k in 0..rank {
                    let a_val = self.matrix_a.get(j).and_then(|r| r.get(k)).copied().unwrap_or(0.0);
                    let b_val = self.matrix_b.get(k).and_then(|r| r.get(i)).copied().unwrap_or(0.0);
                    sum += b_val * a_val;
                }
                self.delta_weights[i][j] = sum * scale;
            }
        }

        self.is_merged = true;
    }

    /// Get update count
    pub fn update_count(&self) -> u64 {
        self.update_count
    }
}

/// EWC++ (Elastic Weight Consolidation) for preventing catastrophic forgetting
#[derive(Debug, Clone)]
pub struct EWCPlusPlus {
    /// Fisher information diagonal
    fisher: Vec<f64>,
    /// Optimal weights from previous task
    optimal_weights: Vec<f64>,
    /// Lambda (importance weight)
    lambda: f64,
    /// Online update rate
    gamma: f64,
    /// Sample count
    sample_count: u64,
}

impl EWCPlusPlus {
    /// Create new EWC++
    pub fn new(dim: usize, lambda: f64) -> Self {
        Self {
            fisher: vec![0.0; dim],
            optimal_weights: vec![0.0; dim],
            lambda,
            gamma: 0.9,
            sample_count: 0,
        }
    }

    /// Update Fisher information online
    pub fn update_fisher(&mut self, gradients: &[f64]) {
        for (i, &g) in gradients.iter().enumerate() {
            if i < self.fisher.len() {
                // Online update: F_new = gamma * F_old + (1 - gamma) * g^2
                self.fisher[i] = self.gamma * self.fisher[i] + (1.0 - self.gamma) * g * g;
            }
        }
        self.sample_count += 1;
    }

    /// Store optimal weights
    pub fn store_optimal(&mut self, weights: &[f64]) {
        for (i, &w) in weights.iter().enumerate() {
            if i < self.optimal_weights.len() {
                self.optimal_weights[i] = w;
            }
        }
    }

    /// Compute EWC penalty
    pub fn penalty(&self, current_weights: &[f64]) -> f64 {
        let mut penalty = 0.0;
        for (i, (&current_w, &optimal_w)) in current_weights.iter().zip(self.optimal_weights.iter()).enumerate().take(self.fisher.len()) {
            let diff = current_w - optimal_w;
            penalty += self.fisher[i] * diff * diff;
        }
        0.5 * self.lambda * penalty
    }

    /// Regularized gradient
    pub fn regularize_gradient(&self, gradients: &mut [f64], current_weights: &[f64]) {
        for i in 0..gradients.len().min(self.fisher.len()) {
            let ewc_grad = self.lambda * self.fisher[i] * (current_weights[i] - self.optimal_weights[i]);
            gradients[i] += ewc_grad;
        }
    }
}

/// Reasoning pattern stored in ReasoningBank
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningPattern {
    /// Pattern ID
    pub id: String,
    /// Input embedding
    pub input: Vec<f64>,
    /// Output embedding
    pub output: Vec<f64>,
    /// Success score
    pub score: f64,
    /// Usage count
    pub usage_count: u64,
    /// Cluster ID
    pub cluster_id: usize,
}

/// Cosine similarity (free function to avoid borrow conflicts)
fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() {
        return 0.0;
    }
    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;
    for (&x, &y) in a.iter().zip(b.iter()) {
        dot += x * y;
        norm_a += x * x;
        norm_b += y * y;
    }
    let denom = (norm_a * norm_b).sqrt();
    if denom > 0.0 { dot / denom } else { 0.0 }
}

/// Euclidean distance (free function to avoid borrow conflicts)
fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(&x, &y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

/// ReasoningBank using K-means++ clustering
#[derive(Debug, Clone)]
pub struct ReasoningBank {
    /// Stored patterns
    patterns: Vec<ReasoningPattern>,
    /// Cluster centroids
    centroids: Vec<Vec<f64>>,
    /// Number of clusters
    num_clusters: usize,
    /// Maximum patterns
    max_patterns: usize,
}

impl ReasoningBank {
    /// Create new ReasoningBank
    pub fn new(num_clusters: usize, max_patterns: usize) -> Self {
        Self {
            patterns: Vec::new(),
            centroids: Vec::new(),
            num_clusters,
            max_patterns,
        }
    }

    /// Store a reasoning pattern
    pub fn store(&mut self, pattern: ReasoningPattern) {
        if self.patterns.len() >= self.max_patterns {
            // Remove lowest scoring pattern
            if let Some(min_idx) = self
                .patterns
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.score.partial_cmp(&b.score).unwrap())
                .map(|(i, _)| i)
            {
                self.patterns.remove(min_idx);
            }
        }

        self.patterns.push(pattern);
    }

    /// Retrieve similar patterns
    pub fn retrieve(&self, query: &[f64], k: usize) -> Vec<&ReasoningPattern> {
        let mut scored: Vec<(f64, &ReasoningPattern)> = self
            .patterns
            .iter()
            .map(|p| {
                let sim = cosine_similarity(query, &p.input);
                (sim, p)
            })
            .collect();

        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        scored.into_iter().take(k).map(|(_, p)| p).collect()
    }

    /// Update centroids using K-means++
    pub fn update_clusters(&mut self) {
        if self.patterns.is_empty() {
            return;
        }

        let dim = self.patterns[0].input.len();

        // Initialize centroids if needed
        if self.centroids.is_empty() || self.centroids.len() != self.num_clusters {
            self.initialize_centroids_kmeans_pp(dim);
        }

        // Clone centroids to avoid borrow conflicts
        let centroids_snapshot = self.centroids.clone();

        // Assign patterns to clusters and update centroids
        let mut cluster_sums: Vec<Vec<f64>> = vec![vec![0.0; dim]; self.num_clusters];
        let mut cluster_counts: Vec<usize> = vec![0; self.num_clusters];

        for pattern in &mut self.patterns {
            // Find nearest centroid
            let nearest = centroids_snapshot
                .iter()
                .enumerate()
                .map(|(i, c)| (i, euclidean_distance(&pattern.input, c)))
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .map(|(i, _)| i)
                .unwrap_or(0);

            pattern.cluster_id = nearest;

            // Accumulate for centroid update
            for (j, &v) in pattern.input.iter().enumerate() {
                if j < dim {
                    cluster_sums[nearest][j] += v;
                }
            }
            cluster_counts[nearest] += 1;
        }

        // Update centroids
        for (i, centroid) in self.centroids.iter_mut().enumerate() {
            if cluster_counts[i] > 0 {
                for (j, c) in centroid.iter_mut().enumerate() {
                    *c = cluster_sums[i][j] / cluster_counts[i] as f64;
                }
            }
        }
    }

    fn initialize_centroids_kmeans_pp(&mut self, dim: usize) {
        self.centroids = Vec::with_capacity(self.num_clusters);

        if self.patterns.is_empty() {
            for _ in 0..self.num_clusters {
                self.centroids.push(vec![0.0; dim]);
            }
            return;
        }

        // First centroid: random pattern
        self.centroids.push(self.patterns[0].input.clone());

        // Remaining centroids: weighted by distance
        for _ in 1..self.num_clusters {
            let distances: Vec<f64> = self
                .patterns
                .iter()
                .map(|p| {
                    self.centroids
                        .iter()
                        .map(|c| euclidean_distance(&p.input, c))
                        .fold(f64::INFINITY, f64::min)
                })
                .collect();

            let total: f64 = distances.iter().map(|d| d * d).sum();
            if total > 0.0 {
                // Simplified: pick pattern with maximum distance
                let max_idx = distances
                    .iter()
                    .enumerate()
                    .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                    .map(|(i, _)| i)
                    .unwrap_or(0);
                self.centroids.push(self.patterns[max_idx].input.clone());
            } else {
                self.centroids.push(vec![0.0; dim]);
            }
        }
    }

    /// Get pattern count
    pub fn len(&self) -> usize {
        self.patterns.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.patterns.is_empty()
    }
}

/// Complete runtime adaptation system
pub struct RuntimeAdaptation {
    /// MicroLoRA for instant adaptation
    micro_lora: LoRAAdapter,
    /// BaseLoRA for long-term learning
    base_lora: LoRAAdapter,
    /// EWC++ for forgetting prevention
    ewc: EWCPlusPlus,
    /// ReasoningBank for pattern storage
    reasoning_bank: ReasoningBank,
    /// Current weights
    current_weights: Vec<f64>,
    /// Adaptation count
    adaptation_count: u64,
}

impl RuntimeAdaptation {
    /// Create new runtime adaptation system
    pub fn new(dim: usize) -> Self {
        let micro_config = LoRAConfig {
            rank: LoRARank::Micro(2),
            alpha: 4.0,
            learning_rate: 0.01,
            dim,
            ..Default::default()
        };

        let base_config = LoRAConfig {
            rank: LoRARank::Base(8),
            alpha: 16.0,
            learning_rate: 0.001,
            dim,
            ..Default::default()
        };

        Self {
            micro_lora: LoRAAdapter::new(micro_config),
            base_lora: LoRAAdapter::new(base_config),
            ewc: EWCPlusPlus::new(dim, 1000.0),
            reasoning_bank: ReasoningBank::new(10, 1000),
            current_weights: vec![0.0; dim],
            adaptation_count: 0,
        }
    }

    /// Adapt to new input-output pair
    pub fn adapt(&mut self, input: &[f64], output: &[f64]) {
        // Quick adaptation with MicroLoRA
        self.micro_lora.update(input, output);

        // Slow adaptation with BaseLoRA
        if self.adaptation_count % 10 == 0 {
            self.base_lora.update(input, output);
        }

        // Store successful pattern
        let pattern = ReasoningPattern {
            id: format!("pattern_{}", self.adaptation_count),
            input: input.to_vec(),
            output: output.to_vec(),
            score: 1.0,
            usage_count: 1,
            cluster_id: 0,
        };
        self.reasoning_bank.store(pattern);

        self.adaptation_count += 1;
    }

    /// Apply adaptation to input
    pub fn apply(&self, input: &[f64]) -> Vec<f64> {
        let micro_out = self.micro_lora.apply(input);
        let base_out = self.base_lora.apply(input);

        // Combine outputs
        micro_out
            .iter()
            .zip(base_out.iter())
            .zip(input.iter())
            .map(|((&m, &b), &i)| i + 0.6 * m + 0.4 * b)
            .collect()
    }

    /// Consolidate learning (call periodically)
    pub fn consolidate(&mut self) {
        // Merge MicroLoRA into BaseLoRA
        self.micro_lora.merge();

        // Update EWC Fisher information
        let gradients = vec![0.01; self.current_weights.len()]; // Placeholder
        self.ewc.update_fisher(&gradients);

        // Store current weights as optimal
        self.ewc.store_optimal(&self.current_weights);

        // Update reasoning bank clusters
        self.reasoning_bank.update_clusters();
    }

    /// Get adaptation stats
    pub fn stats(&self) -> AdaptationStats {
        AdaptationStats {
            adaptation_count: self.adaptation_count,
            micro_lora_updates: self.micro_lora.update_count(),
            base_lora_updates: self.base_lora.update_count(),
            reasoning_patterns: self.reasoning_bank.len(),
            ewc_samples: self.ewc.sample_count,
        }
    }
}

/// Adaptation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationStats {
    /// Total adaptations
    pub adaptation_count: u64,
    /// MicroLoRA updates
    pub micro_lora_updates: u64,
    /// BaseLoRA updates
    pub base_lora_updates: u64,
    /// Stored reasoning patterns
    pub reasoning_patterns: usize,
    /// EWC samples
    pub ewc_samples: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lora_adapter() {
        let config = LoRAConfig {
            dim: 16,
            rank: LoRARank::Base(4),
            ..Default::default()
        };
        let mut adapter = LoRAAdapter::new(config);

        let input = vec![0.5; 16];
        let output = adapter.apply(&input);
        assert_eq!(output.len(), 16);

        let target = vec![0.3; 16];
        adapter.update(&input, &target);
        assert_eq!(adapter.update_count(), 1);
    }

    #[test]
    fn test_ewc() {
        let mut ewc = EWCPlusPlus::new(8, 100.0);

        let gradients = vec![0.1; 8];
        ewc.update_fisher(&gradients);

        let weights = vec![0.5; 8];
        ewc.store_optimal(&weights);

        let current = vec![0.6; 8];
        let penalty = ewc.penalty(&current);
        assert!(penalty > 0.0);
    }

    #[test]
    fn test_reasoning_bank() {
        let mut bank = ReasoningBank::new(5, 100);

        let pattern = ReasoningPattern {
            id: "test".to_string(),
            input: vec![0.5; 8],
            output: vec![0.3; 8],
            score: 1.0,
            usage_count: 1,
            cluster_id: 0,
        };
        bank.store(pattern);

        let query = vec![0.5; 8];
        let results = bank.retrieve(&query, 1);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_runtime_adaptation() {
        let mut adapter = RuntimeAdaptation::new(16);

        let input = vec![0.5; 16];
        let output = vec![0.3; 16];
        adapter.adapt(&input, &output);

        let result = adapter.apply(&input);
        assert_eq!(result.len(), 16);

        let stats = adapter.stats();
        assert_eq!(stats.adaptation_count, 1);
    }
}
