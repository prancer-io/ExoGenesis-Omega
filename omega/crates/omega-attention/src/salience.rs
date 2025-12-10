//! Salience Computation
//!
//! Bottom-up attention based on stimulus properties:
//! - Novelty: How different from recent input
//! - Contrast: Local contrast with surroundings
//! - Change: Temporal change detection
//! - Intensity: Raw signal strength

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Types of salience features
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SalienceFeature {
    /// Novelty relative to recent history
    Novelty,
    /// Local contrast
    Contrast,
    /// Temporal change
    Change,
    /// Raw intensity
    Intensity,
    /// Spatial discontinuity
    EdgeSalience,
    /// Motion (change over time)
    Motion,
}

impl SalienceFeature {
    pub fn all() -> Vec<SalienceFeature> {
        vec![
            Self::Novelty,
            Self::Contrast,
            Self::Change,
            Self::Intensity,
            Self::EdgeSalience,
            Self::Motion,
        ]
    }
}

/// Salience map for a set of inputs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalienceMap {
    /// Per-item salience scores
    pub scores: Vec<f64>,
    /// Feature-specific contributions
    pub feature_scores: Vec<Vec<f64>>,
    /// Maximum salience value
    pub max_salience: f64,
    /// Index of most salient item
    pub max_index: usize,
}

impl SalienceMap {
    pub fn new(size: usize) -> Self {
        Self {
            scores: vec![0.0; size],
            feature_scores: vec![vec![0.0; size]; SalienceFeature::all().len()],
            max_salience: 0.0,
            max_index: 0,
        }
    }

    /// Get top-k salient items
    pub fn top_k(&self, k: usize) -> Vec<(usize, f64)> {
        let mut indexed: Vec<_> = self.scores.iter().enumerate().collect();
        indexed.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
        indexed
            .into_iter()
            .take(k)
            .map(|(i, &s)| (i, s))
            .collect()
    }

    /// Normalize scores
    pub fn normalize(&mut self) {
        let sum: f64 = self.scores.iter().sum();
        if sum > 0.0 {
            for s in &mut self.scores {
                *s /= sum;
            }
        }
    }
}

/// Configuration for salience computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalienceConfig {
    /// Weight for novelty
    pub novelty_weight: f64,
    /// Weight for contrast
    pub contrast_weight: f64,
    /// Weight for change
    pub change_weight: f64,
    /// Weight for intensity
    pub intensity_weight: f64,
    /// History length for novelty computation
    pub history_length: usize,
}

impl Default for SalienceConfig {
    fn default() -> Self {
        Self {
            novelty_weight: 0.3,
            contrast_weight: 0.25,
            change_weight: 0.25,
            intensity_weight: 0.2,
            history_length: 10,
        }
    }
}

/// Salience computation engine
pub struct SalienceComputer {
    config: SalienceConfig,
    /// History of recent inputs
    history: VecDeque<Vec<f64>>,
    /// Running mean for novelty
    running_mean: Vec<f64>,
    /// Running variance for novelty
    running_var: Vec<f64>,
    /// Previous input for change detection
    prev_input: Option<Vec<f64>>,
}

impl SalienceComputer {
    pub fn new() -> Self {
        Self::with_config(SalienceConfig::default())
    }

    pub fn with_config(config: SalienceConfig) -> Self {
        Self {
            config,
            history: VecDeque::with_capacity(10),
            running_mean: Vec::new(),
            running_var: Vec::new(),
            prev_input: None,
        }
    }

    /// Compute salience for input
    pub fn compute(&mut self, input: &[f64]) -> Vec<f64> {
        let n = input.len();
        if n == 0 {
            return vec![];
        }

        let mut salience = vec![0.0; n];

        // Compute novelty
        let novelty = self.compute_novelty(input);

        // Compute contrast
        let contrast = self.compute_contrast(input);

        // Compute change
        let change = self.compute_change(input);

        // Compute intensity
        let intensity = self.compute_intensity(input);

        // Combine features
        for i in 0..n {
            salience[i] = self.config.novelty_weight * novelty.get(i).copied().unwrap_or(0.0)
                + self.config.contrast_weight * contrast.get(i).copied().unwrap_or(0.0)
                + self.config.change_weight * change.get(i).copied().unwrap_or(0.0)
                + self.config.intensity_weight * intensity.get(i).copied().unwrap_or(0.0);
        }

        // Update history
        self.update_history(input);

        // Normalize
        let max = salience.iter().cloned().fold(0.0, f64::max);
        if max > 0.0 {
            for s in &mut salience {
                *s /= max;
            }
        }

        salience
    }

    /// Compute full salience map with feature breakdown
    pub fn compute_map(&mut self, input: &[f64]) -> SalienceMap {
        let n = input.len();
        let mut map = SalienceMap::new(n);

        if n == 0 {
            return map;
        }

        // Compute individual features
        let novelty = self.compute_novelty(input);
        let contrast = self.compute_contrast(input);
        let change = self.compute_change(input);
        let intensity = self.compute_intensity(input);

        // Store feature scores
        map.feature_scores[0] = novelty.clone();
        map.feature_scores[1] = contrast.clone();
        map.feature_scores[2] = change.clone();
        map.feature_scores[3] = intensity.clone();

        // Combine
        for i in 0..n {
            map.scores[i] = self.config.novelty_weight * novelty[i]
                + self.config.contrast_weight * contrast[i]
                + self.config.change_weight * change[i]
                + self.config.intensity_weight * intensity[i];
        }

        // Find max
        map.max_salience = 0.0;
        map.max_index = 0;
        for (i, &s) in map.scores.iter().enumerate() {
            if s > map.max_salience {
                map.max_salience = s;
                map.max_index = i;
            }
        }

        // Update history
        self.update_history(input);

        map
    }

    /// Compute novelty (deviation from running statistics)
    fn compute_novelty(&self, input: &[f64]) -> Vec<f64> {
        let n = input.len();

        if self.running_mean.is_empty() || self.running_mean.len() != n {
            // No history yet, everything is equally novel
            return vec![0.5; n];
        }

        let mut novelty = Vec::with_capacity(n);
        for i in 0..n {
            let mean = self.running_mean[i];
            let var = self.running_var[i].max(0.01); // Avoid division by zero
            let std = var.sqrt();

            // Z-score as novelty measure
            let z = (input[i] - mean).abs() / std;
            novelty.push((z / 3.0).min(1.0)); // Normalize, cap at 3 std
        }

        novelty
    }

    /// Compute local contrast
    fn compute_contrast(&self, input: &[f64]) -> Vec<f64> {
        let n = input.len();
        let mut contrast = vec![0.0; n];

        if n < 3 {
            return contrast;
        }

        // Compute local contrast (difference from neighbors)
        for i in 0..n {
            let mut neighbor_sum = 0.0;
            let mut neighbor_count = 0;

            // Look at neighbors (window of 3)
            for j in i.saturating_sub(1)..=(i + 1).min(n - 1) {
                if j != i {
                    neighbor_sum += input[j];
                    neighbor_count += 1;
                }
            }

            if neighbor_count > 0 {
                let neighbor_mean = neighbor_sum / neighbor_count as f64;
                contrast[i] = (input[i] - neighbor_mean).abs();
            }
        }

        // Normalize
        let max = contrast.iter().cloned().fold(0.0, f64::max);
        if max > 0.0 {
            for c in &mut contrast {
                *c /= max;
            }
        }

        contrast
    }

    /// Compute temporal change
    fn compute_change(&self, input: &[f64]) -> Vec<f64> {
        let n = input.len();

        let Some(ref prev) = self.prev_input else {
            return vec![0.0; n]; // No previous input
        };

        if prev.len() != n {
            return vec![0.0; n];
        }

        let mut change = Vec::with_capacity(n);
        for i in 0..n {
            change.push((input[i] - prev[i]).abs());
        }

        // Normalize
        let max = change.iter().cloned().fold(0.0, f64::max);
        if max > 0.0 {
            for c in &mut change {
                *c /= max;
            }
        }

        change
    }

    /// Compute raw intensity
    fn compute_intensity(&self, input: &[f64]) -> Vec<f64> {
        let max = input.iter().cloned().fold(0.0, f64::max).max(0.01);
        input.iter().map(|&x| x.abs() / max).collect()
    }

    /// Update running statistics
    fn update_history(&mut self, input: &[f64]) {
        let n = input.len();

        // Initialize if needed
        if self.running_mean.len() != n {
            self.running_mean = input.to_vec();
            self.running_var = vec![0.1; n];
        } else {
            // Exponential moving average
            let alpha = 0.1;
            for i in 0..n {
                let old_mean = self.running_mean[i];
                let new_mean = alpha * input[i] + (1.0 - alpha) * old_mean;
                let delta = input[i] - old_mean;
                let new_var = (1.0 - alpha) * (self.running_var[i] + alpha * delta * delta);

                self.running_mean[i] = new_mean;
                self.running_var[i] = new_var;
            }
        }

        // Update prev_input
        self.prev_input = Some(input.to_vec());

        // Update history buffer
        self.history.push_back(input.to_vec());
        while self.history.len() > self.config.history_length {
            self.history.pop_front();
        }
    }

    /// Reset state
    pub fn reset(&mut self) {
        self.history.clear();
        self.running_mean.clear();
        self.running_var.clear();
        self.prev_input = None;
    }
}

impl Default for SalienceComputer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_salience_computer_creation() {
        let computer = SalienceComputer::new();
        assert!(computer.history.is_empty());
    }

    #[test]
    fn test_salience_computation() {
        let mut computer = SalienceComputer::new();

        let input = vec![0.1, 0.5, 0.2, 0.8, 0.3];
        let salience = computer.compute(&input);

        assert_eq!(salience.len(), 5);
        assert!(salience.iter().all(|&s| s >= 0.0 && s <= 1.0));
    }

    #[test]
    fn test_novelty_detection() {
        let mut computer = SalienceComputer::new();

        // Build up baseline
        for _ in 0..10 {
            computer.compute(&vec![0.5, 0.5, 0.5, 0.5, 0.5]);
        }

        // Novel input (high value at position 2)
        let novel_input = vec![0.5, 0.5, 0.95, 0.5, 0.5];
        let salience = computer.compute(&novel_input);

        // Position 2 should be most salient
        let max_idx = salience
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i)
            .unwrap();

        assert_eq!(max_idx, 2);
    }

    #[test]
    fn test_change_detection() {
        let mut computer = SalienceComputer::new();

        // First input
        computer.compute(&vec![0.5, 0.5, 0.5, 0.5, 0.5]);

        // Second input with change at position 3
        let changed_input = vec![0.5, 0.5, 0.5, 0.9, 0.5];
        let map = computer.compute_map(&changed_input);

        // Change feature should be high at position 3
        assert!(map.feature_scores[2][3] > 0.5);
    }

    #[test]
    fn test_salience_map() {
        let mut computer = SalienceComputer::new();

        let input = vec![0.1, 0.9, 0.2, 0.8, 0.3];
        let map = computer.compute_map(&input);

        assert_eq!(map.scores.len(), 5);
        assert!(map.max_salience > 0.0);

        let top2 = map.top_k(2);
        assert_eq!(top2.len(), 2);
    }

    #[test]
    fn test_contrast_computation() {
        let mut computer = SalienceComputer::new();

        // High contrast at position 2
        let input = vec![0.1, 0.1, 0.9, 0.1, 0.1];
        let salience = computer.compute(&input);

        // Position 2 should have high salience
        assert!(salience[2] > salience[0]);
        assert!(salience[2] > salience[4]);
    }
}
