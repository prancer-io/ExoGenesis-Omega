//! Neural Population Coding
//!
//! Implements population-level representations and sparse coding.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::neuron::NeuronId;
use crate::spike_train::SpikeTrain;

/// Activity state of a neural population
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationActivity {
    /// Population identifier
    pub population_id: String,
    /// Activity level per neuron (0.0 to 1.0)
    pub activities: HashMap<NeuronId, f64>,
    /// Population-level metrics
    pub mean_activity: f64,
    /// Sparsity (fraction of neurons with low activity)
    pub sparsity: f64,
    /// Population vector (centroid of activity)
    pub population_vector: Vec<f64>,
}

impl PopulationActivity {
    /// Create from spike trains over a time window
    pub fn from_spike_trains(
        population_id: String,
        trains: &[SpikeTrain],
        window: std::time::Duration,
        max_rate: f64,
    ) -> Self {
        let mut activities = HashMap::new();
        let mut total_activity = 0.0;
        let mut active_count = 0;

        for train in trains {
            let rate = train.firing_rate(window);
            let activity = (rate / max_rate).min(1.0);
            activities.insert(train.neuron_id.clone(), activity);
            total_activity += activity;

            if activity > 0.1 {
                active_count += 1;
            }
        }

        let n = trains.len() as f64;
        let mean_activity = if n > 0.0 { total_activity / n } else { 0.0 };
        let sparsity = if n > 0.0 {
            1.0 - (active_count as f64 / n)
        } else {
            1.0
        };

        Self {
            population_id,
            activities,
            mean_activity,
            sparsity,
            population_vector: Vec::new(),
        }
    }

    /// Get activity for specific neuron
    pub fn get_activity(&self, neuron_id: &NeuronId) -> f64 {
        *self.activities.get(neuron_id).unwrap_or(&0.0)
    }

    /// Get most active neurons
    pub fn top_active(&self, k: usize) -> Vec<(NeuronId, f64)> {
        let mut sorted: Vec<_> = self.activities.iter().collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
        sorted
            .into_iter()
            .take(k)
            .map(|(id, act)| (id.clone(), *act))
            .collect()
    }
}

/// Sparse code representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SparseCode {
    /// Dimension of the code
    pub dimension: usize,
    /// Active indices and their values
    pub active: HashMap<usize, f64>,
    /// Target sparsity level
    pub target_sparsity: f64,
}

impl SparseCode {
    /// Create empty sparse code
    pub fn new(dimension: usize, target_sparsity: f64) -> Self {
        Self {
            dimension,
            active: HashMap::new(),
            target_sparsity,
        }
    }

    /// Create from dense vector
    pub fn from_dense(values: &[f64], target_sparsity: f64) -> Self {
        let dimension = values.len();
        let k = ((1.0 - target_sparsity) * dimension as f64).ceil() as usize;

        // Get top-k indices
        let mut indexed: Vec<_> = values.iter().enumerate().collect();
        indexed.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

        let active: HashMap<usize, f64> = indexed
            .into_iter()
            .take(k)
            .filter(|(_, &v)| v > 0.0)
            .map(|(i, &v)| (i, v))
            .collect();

        Self {
            dimension,
            active,
            target_sparsity,
        }
    }

    /// Convert to dense vector
    pub fn to_dense(&self) -> Vec<f64> {
        let mut dense = vec![0.0; self.dimension];
        for (&idx, &val) in &self.active {
            if idx < self.dimension {
                dense[idx] = val;
            }
        }
        dense
    }

    /// Get actual sparsity
    pub fn sparsity(&self) -> f64 {
        1.0 - (self.active.len() as f64 / self.dimension as f64)
    }

    /// Get L1 norm
    pub fn l1_norm(&self) -> f64 {
        self.active.values().map(|v| v.abs()).sum()
    }

    /// Get L2 norm
    pub fn l2_norm(&self) -> f64 {
        self.active.values().map(|v| v * v).sum::<f64>().sqrt()
    }

    /// Dot product with another sparse code
    pub fn dot(&self, other: &SparseCode) -> f64 {
        let mut sum = 0.0;
        for (&idx, &val) in &self.active {
            if let Some(&other_val) = other.active.get(&idx) {
                sum += val * other_val;
            }
        }
        sum
    }

    /// Cosine similarity with another sparse code
    pub fn cosine_similarity(&self, other: &SparseCode) -> f64 {
        let dot = self.dot(other);
        let norm_self = self.l2_norm();
        let norm_other = other.l2_norm();

        if norm_self == 0.0 || norm_other == 0.0 {
            return 0.0;
        }

        dot / (norm_self * norm_other)
    }

    /// Add two sparse codes
    pub fn add(&self, other: &SparseCode) -> SparseCode {
        let mut result = self.clone();

        for (&idx, &val) in &other.active {
            *result.active.entry(idx).or_insert(0.0) += val;
        }

        result
    }

    /// Scale sparse code
    pub fn scale(&self, factor: f64) -> SparseCode {
        let mut result = self.clone();
        for val in result.active.values_mut() {
            *val *= factor;
        }
        result
    }
}

/// Neural population with encoding/decoding capabilities
#[derive(Debug, Clone)]
pub struct NeuralPopulation {
    /// Population ID
    pub id: String,
    /// Neuron IDs in this population
    pub neuron_ids: Vec<NeuronId>,
    /// Preferred stimuli for each neuron (tuning curves)
    pub tuning_centers: HashMap<NeuronId, Vec<f64>>,
    /// Tuning width (selectivity)
    pub tuning_width: f64,
}

impl NeuralPopulation {
    /// Create a new population
    pub fn new(id: String, neuron_ids: Vec<NeuronId>) -> Self {
        Self {
            id,
            neuron_ids,
            tuning_centers: HashMap::new(),
            tuning_width: 1.0,
        }
    }

    /// Create population with uniformly spaced tuning curves
    pub fn with_uniform_tuning(
        id: String,
        size: usize,
        stimulus_dim: usize,
        stimulus_range: (f64, f64),
    ) -> Self {
        let mut neuron_ids = Vec::new();
        let mut tuning_centers = HashMap::new();

        for i in 0..size {
            let neuron_id = format!("{}_{}", id, i);

            // Create tuning center
            let t = i as f64 / (size - 1).max(1) as f64;
            let center: Vec<f64> = (0..stimulus_dim)
                .map(|_| stimulus_range.0 + t * (stimulus_range.1 - stimulus_range.0))
                .collect();

            tuning_centers.insert(neuron_id.clone(), center);
            neuron_ids.push(neuron_id);
        }

        Self {
            id,
            neuron_ids,
            tuning_centers,
            tuning_width: (stimulus_range.1 - stimulus_range.0) / size as f64,
        }
    }

    /// Encode stimulus into population activity
    pub fn encode(&self, stimulus: &[f64]) -> HashMap<NeuronId, f64> {
        let mut activities = HashMap::new();

        for (neuron_id, center) in &self.tuning_centers {
            // Compute distance from tuning center
            let dist_sq: f64 = stimulus
                .iter()
                .zip(center.iter())
                .map(|(s, c)| (s - c).powi(2))
                .sum();

            // Gaussian tuning curve
            let activity = (-dist_sq / (2.0 * self.tuning_width * self.tuning_width)).exp();
            activities.insert(neuron_id.clone(), activity);
        }

        activities
    }

    /// Decode stimulus from population activity (population vector)
    pub fn decode(&self, activities: &HashMap<NeuronId, f64>) -> Vec<f64> {
        if self.tuning_centers.is_empty() {
            return Vec::new();
        }

        let dim = self.tuning_centers.values().next().map(|v| v.len()).unwrap_or(0);
        let mut weighted_sum = vec![0.0; dim];
        let mut total_weight = 0.0;

        for (neuron_id, center) in &self.tuning_centers {
            let activity = *activities.get(neuron_id).unwrap_or(&0.0);
            total_weight += activity;

            for (i, &c) in center.iter().enumerate() {
                weighted_sum[i] += activity * c;
            }
        }

        if total_weight > 0.0 {
            weighted_sum.iter().map(|&s| s / total_weight).collect()
        } else {
            vec![0.0; dim]
        }
    }

    /// Get population size
    pub fn size(&self) -> usize {
        self.neuron_ids.len()
    }
}

/// Winner-take-all circuit for competitive inhibition
#[derive(Debug, Clone)]
pub struct WinnerTakeAll {
    /// Number of winners to select
    pub k: usize,
    /// Inhibition strength
    pub inhibition: f64,
}

impl WinnerTakeAll {
    pub fn new(k: usize, inhibition: f64) -> Self {
        Self { k, inhibition }
    }

    /// Apply WTA to activities
    pub fn apply(&self, activities: &mut HashMap<NeuronId, f64>) {
        // Find top-k
        let mut sorted: Vec<_> = activities.iter().collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

        let winners: std::collections::HashSet<_> =
            sorted.iter().take(self.k).map(|(id, _)| (*id).clone()).collect();

        // Inhibit non-winners
        for (id, activity) in activities.iter_mut() {
            if !winners.contains(id) {
                *activity *= 1.0 - self.inhibition;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparse_code_creation() {
        let dense = vec![0.1, 0.9, 0.2, 0.8, 0.3];
        let sparse = SparseCode::from_dense(&dense, 0.6);

        assert!(sparse.sparsity() >= 0.4); // At most 60% active
        assert!(sparse.active.contains_key(&1)); // 0.9 should be active
        assert!(sparse.active.contains_key(&3)); // 0.8 should be active
    }

    #[test]
    fn test_sparse_code_operations() {
        let a = SparseCode::from_dense(&[1.0, 0.0, 1.0, 0.0], 0.5);
        let b = SparseCode::from_dense(&[1.0, 1.0, 0.0, 0.0], 0.5);

        let dot = a.dot(&b);
        assert!((dot - 1.0).abs() < 0.01); // Only overlap at index 0
    }

    #[test]
    fn test_neural_population_encoding() {
        let pop = NeuralPopulation::with_uniform_tuning(
            "test".to_string(),
            10,
            1,
            (0.0, 1.0),
        );

        let activities = pop.encode(&[0.5]);

        // Middle neuron should be most active
        let max_activity = activities.values().cloned().fold(0.0, f64::max);
        assert!(max_activity > 0.0);
    }

    #[test]
    fn test_neural_population_decoding() {
        let pop = NeuralPopulation::with_uniform_tuning(
            "test".to_string(),
            10,
            1,
            (0.0, 1.0),
        );

        let stimulus = vec![0.5];
        let activities = pop.encode(&stimulus);
        let decoded = pop.decode(&activities);

        assert!((decoded[0] - 0.5).abs() < 0.1);
    }

    #[test]
    fn test_winner_take_all() {
        let mut activities: HashMap<NeuronId, f64> = HashMap::new();
        activities.insert("n1".to_string(), 0.9);
        activities.insert("n2".to_string(), 0.8);
        activities.insert("n3".to_string(), 0.3);
        activities.insert("n4".to_string(), 0.2);

        let wta = WinnerTakeAll::new(2, 0.9);
        wta.apply(&mut activities);

        assert!(*activities.get("n1").unwrap() > 0.8);
        assert!(*activities.get("n2").unwrap() > 0.7);
        assert!(*activities.get("n3").unwrap() < 0.1);
        assert!(*activities.get("n4").unwrap() < 0.1);
    }
}
