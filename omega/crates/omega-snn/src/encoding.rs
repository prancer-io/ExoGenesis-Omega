//! Spike Encoding Methods
//!
//! Provides various methods for converting analog signals to spike trains:
//! - Rate Coding: Higher values produce more frequent spikes
//! - Temporal Coding: Information encoded in spike timing
//! - Delta Modulation: Spikes encode changes only
//! - Population Coding: Distributed representation across neurons

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::neuron::NeuronId;
use crate::spike_train::Spike;

// ============================================================================
// RATE CODING
// ============================================================================

/// Rate coding encoder - converts values to spike probability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateEncoder {
    /// Maximum firing rate (Hz)
    pub max_rate: f64,
    /// Minimum value for mapping
    pub min_value: f64,
    /// Maximum value for mapping
    pub max_value: f64,
    /// Time step (ms)
    pub dt_ms: f64,
}

impl Default for RateEncoder {
    fn default() -> Self {
        Self {
            max_rate: 100.0,  // 100 Hz max
            min_value: 0.0,
            max_value: 1.0,
            dt_ms: 1.0,
        }
    }
}

impl RateEncoder {
    pub fn new(max_rate: f64, min_value: f64, max_value: f64, dt_ms: f64) -> Self {
        Self {
            max_rate,
            min_value,
            max_value,
            dt_ms,
        }
    }

    /// Convert a value to spike probability for this time step
    pub fn spike_probability(&self, value: f64) -> f64 {
        // Normalize to [0, 1]
        let normalized = (value - self.min_value) / (self.max_value - self.min_value);
        let normalized = normalized.clamp(0.0, 1.0);

        // Convert rate to probability per timestep
        // P(spike) = rate * dt (for small dt)
        let rate = normalized * self.max_rate;
        (rate * self.dt_ms / 1000.0).min(1.0)
    }

    /// Encode a value, returning true if a spike should be generated
    pub fn encode(&self, value: f64) -> bool {
        let prob = self.spike_probability(value);
        rand::thread_rng().gen::<f64>() < prob
    }

    /// Encode multiple values (e.g., for a population)
    pub fn encode_batch(&self, values: &[f64]) -> Vec<bool> {
        values.iter().map(|&v| self.encode(v)).collect()
    }
}

// ============================================================================
// TEMPORAL CODING
// ============================================================================

/// Temporal coding encoder - encodes values as spike timing
///
/// Higher values produce earlier spikes within a time window.
/// Uses inverse latency coding: value → latency → spike time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalEncoder {
    /// Maximum latency (ms) for minimum value
    pub max_latency_ms: f64,
    /// Minimum value for mapping
    pub min_value: f64,
    /// Maximum value for mapping
    pub max_value: f64,
    /// Encoding type
    pub encoding: TemporalEncodingType,
}

/// Types of temporal encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemporalEncodingType {
    /// Linear mapping: latency = max_latency * (1 - normalized_value)
    Linear,
    /// Logarithmic: better resolution for high values
    Logarithmic,
    /// Exponential: better resolution for low values
    Exponential,
}

impl Default for TemporalEncoder {
    fn default() -> Self {
        Self {
            max_latency_ms: 50.0,
            min_value: 0.0,
            max_value: 1.0,
            encoding: TemporalEncodingType::Linear,
        }
    }
}

impl TemporalEncoder {
    pub fn new(max_latency_ms: f64, min_value: f64, max_value: f64) -> Self {
        Self {
            max_latency_ms,
            min_value,
            max_value,
            encoding: TemporalEncodingType::Linear,
        }
    }

    /// Get spike latency for a value (ms from window start)
    pub fn latency(&self, value: f64) -> f64 {
        let normalized = (value - self.min_value) / (self.max_value - self.min_value);
        let normalized = normalized.clamp(0.0, 1.0);

        match self.encoding {
            TemporalEncodingType::Linear => {
                // High value = low latency (early spike)
                self.max_latency_ms * (1.0 - normalized)
            }
            TemporalEncodingType::Logarithmic => {
                // Logarithmic scaling
                let log_scale = (1.0 - normalized + 0.01).ln() / (0.01_f64.ln());
                self.max_latency_ms * log_scale
            }
            TemporalEncodingType::Exponential => {
                // Exponential scaling
                let exp_scale = (-normalized * 5.0).exp();
                self.max_latency_ms * exp_scale
            }
        }
    }

    /// Encode a value, returning the spike time as Duration
    pub fn encode(&self, value: f64) -> Duration {
        Duration::from_secs_f64(self.latency(value) / 1000.0)
    }

    /// Encode multiple values
    pub fn encode_batch(&self, values: &[f64]) -> Vec<Duration> {
        values.iter().map(|&v| self.encode(v)).collect()
    }
}

// ============================================================================
// DELTA MODULATION
// ============================================================================

/// Delta modulation encoder - spikes encode changes
///
/// Generates spikes only when the signal changes by more than a threshold.
/// Returns +1 for increase, -1 for decrease, 0 for no change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaEncoder {
    /// Threshold for triggering a spike
    pub threshold: f64,
    /// Previous value for comparison
    previous: f64,
    /// Whether the encoder has been initialized
    initialized: bool,
}

impl Default for DeltaEncoder {
    fn default() -> Self {
        Self {
            threshold: 0.1,
            previous: 0.0,
            initialized: false,
        }
    }
}

impl DeltaEncoder {
    pub fn new(threshold: f64) -> Self {
        Self {
            threshold,
            previous: 0.0,
            initialized: false,
        }
    }

    /// Encode a value, returning -1, 0, or +1
    pub fn encode(&mut self, value: f64) -> i8 {
        if !self.initialized {
            self.previous = value;
            self.initialized = true;
            return 0;
        }

        let delta = value - self.previous;

        if delta > self.threshold {
            self.previous = value;
            1
        } else if delta < -self.threshold {
            self.previous = value;
            -1
        } else {
            0
        }
    }

    /// Reset the encoder state
    pub fn reset(&mut self) {
        self.previous = 0.0;
        self.initialized = false;
    }

    /// Encode a sequence of values
    pub fn encode_sequence(&mut self, values: &[f64]) -> Vec<i8> {
        values.iter().map(|&v| self.encode(v)).collect()
    }
}

// ============================================================================
// POPULATION CODING
// ============================================================================

/// Population coding encoder - distributed representation across neurons
///
/// Uses Gaussian tuning curves centered at preferred values.
/// Each neuron responds maximally to its preferred value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationEncoder {
    /// Preferred values for each neuron in the population
    pub preferred_values: Vec<f64>,
    /// Width of tuning curves (sigma)
    pub tuning_width: f64,
    /// Maximum response rate
    pub max_rate: f64,
}

impl PopulationEncoder {
    /// Create a population encoder with uniformly distributed preferred values
    pub fn uniform(num_neurons: usize, min_value: f64, max_value: f64, tuning_width: f64) -> Self {
        let step = (max_value - min_value) / (num_neurons - 1).max(1) as f64;
        let preferred_values = (0..num_neurons)
            .map(|i| min_value + i as f64 * step)
            .collect();

        Self {
            preferred_values,
            tuning_width,
            max_rate: 100.0,
        }
    }

    /// Create with custom preferred values
    pub fn with_preferences(preferred_values: Vec<f64>, tuning_width: f64) -> Self {
        Self {
            preferred_values,
            tuning_width,
            max_rate: 100.0,
        }
    }

    /// Get response of a single neuron to a value
    pub fn neuron_response(&self, neuron_idx: usize, value: f64) -> f64 {
        if neuron_idx >= self.preferred_values.len() {
            return 0.0;
        }

        let pref = self.preferred_values[neuron_idx];
        let diff = value - pref;

        // Gaussian tuning curve
        let response = (-diff * diff / (2.0 * self.tuning_width * self.tuning_width)).exp();
        response * self.max_rate
    }

    /// Get responses of all neurons to a value
    pub fn encode(&self, value: f64) -> Vec<f64> {
        (0..self.preferred_values.len())
            .map(|i| self.neuron_response(i, value))
            .collect()
    }

    /// Decode population activity back to value estimate
    pub fn decode(&self, responses: &[f64]) -> f64 {
        if responses.len() != self.preferred_values.len() {
            return 0.0;
        }

        // Center-of-mass decoding
        let total: f64 = responses.iter().sum();
        if total == 0.0 {
            return 0.0;
        }

        let weighted_sum: f64 = responses
            .iter()
            .zip(self.preferred_values.iter())
            .map(|(r, &p)| r * p)
            .sum();

        weighted_sum / total
    }

    /// Get number of neurons in population
    pub fn population_size(&self) -> usize {
        self.preferred_values.len()
    }
}

// ============================================================================
// SPARSE SPIKES STRUCTURE
// ============================================================================

/// Efficient sparse representation of population spike activity
///
/// Only stores active (spiking) neurons rather than a dense matrix.
/// Dramatically reduces memory for sparse firing patterns.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SparseSpikes {
    /// Neuron IDs that spiked
    pub active_neurons: Vec<NeuronId>,
    /// Corresponding spike times (if tracking timing)
    pub spike_times: Vec<Duration>,
    /// Total number of neurons in population (for density calculation)
    pub population_size: usize,
}

impl SparseSpikes {
    pub fn new(population_size: usize) -> Self {
        Self {
            active_neurons: Vec::new(),
            spike_times: Vec::new(),
            population_size,
        }
    }

    /// Add a spike
    pub fn add_spike(&mut self, neuron_id: NeuronId, time: Duration) {
        self.active_neurons.push(neuron_id);
        self.spike_times.push(time);
    }

    /// Add multiple spikes at the same time
    pub fn add_spikes(&mut self, neuron_ids: &[NeuronId], time: Duration) {
        for id in neuron_ids {
            self.add_spike(id.clone(), time);
        }
    }

    /// Get sparsity ratio (fraction of silent neurons)
    pub fn sparsity(&self) -> f64 {
        if self.population_size == 0 {
            return 1.0;
        }
        1.0 - (self.active_neurons.len() as f64 / self.population_size as f64)
    }

    /// Get active count
    pub fn active_count(&self) -> usize {
        self.active_neurons.len()
    }

    /// Check if a neuron is active
    pub fn is_active(&self, neuron_id: &NeuronId) -> bool {
        self.active_neurons.contains(neuron_id)
    }

    /// Convert to dense vector (true = spiked)
    pub fn to_dense(&self, id_to_index: &std::collections::HashMap<NeuronId, usize>) -> Vec<bool> {
        let mut dense = vec![false; self.population_size];
        for id in &self.active_neurons {
            if let Some(&idx) = id_to_index.get(id) {
                if idx < dense.len() {
                    dense[idx] = true;
                }
            }
        }
        dense
    }

    /// Convert from dense vector
    pub fn from_dense(
        dense: &[bool],
        index_to_id: &[NeuronId],
        time: Duration,
    ) -> Self {
        let mut sparse = Self::new(dense.len());
        for (idx, &active) in dense.iter().enumerate() {
            if active && idx < index_to_id.len() {
                sparse.add_spike(index_to_id[idx].clone(), time);
            }
        }
        sparse
    }

    /// Clear all spikes
    pub fn clear(&mut self) {
        self.active_neurons.clear();
        self.spike_times.clear();
    }

    /// Convert to Spike events
    pub fn to_spikes(&self) -> Vec<Spike> {
        self.active_neurons
            .iter()
            .zip(self.spike_times.iter())
            .map(|(id, &time)| Spike::new(id.clone(), time))
            .collect()
    }
}

// ============================================================================
// MULTI-ENCODER
// ============================================================================

/// Combined encoder supporting multiple encoding strategies
#[derive(Debug, Clone)]
pub struct MultiEncoder {
    pub rate: RateEncoder,
    pub temporal: TemporalEncoder,
    pub delta: DeltaEncoder,
    pub population: Option<PopulationEncoder>,
}

impl Default for MultiEncoder {
    fn default() -> Self {
        Self {
            rate: RateEncoder::default(),
            temporal: TemporalEncoder::default(),
            delta: DeltaEncoder::default(),
            population: None,
        }
    }
}

impl MultiEncoder {
    pub fn with_population(num_neurons: usize, min_value: f64, max_value: f64) -> Self {
        Self {
            population: Some(PopulationEncoder::uniform(
                num_neurons,
                min_value,
                max_value,
                (max_value - min_value) / num_neurons as f64,
            )),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_encoder() {
        let encoder = RateEncoder::default();

        // Zero value should have low probability
        let prob_zero = encoder.spike_probability(0.0);
        assert!(prob_zero < 0.01);

        // Max value should have higher probability
        let prob_max = encoder.spike_probability(1.0);
        assert!(prob_max > prob_zero);
        assert!(prob_max <= 0.1); // max_rate * dt / 1000 = 100 * 1 / 1000 = 0.1
    }

    #[test]
    fn test_temporal_encoder() {
        let encoder = TemporalEncoder::default();

        // High value = low latency (early spike)
        let latency_high = encoder.latency(1.0);
        let latency_low = encoder.latency(0.0);

        assert!(latency_high < latency_low);
        assert!(latency_high >= 0.0);
        assert!(latency_low <= encoder.max_latency_ms);
    }

    #[test]
    fn test_delta_encoder() {
        let mut encoder = DeltaEncoder::new(0.1);

        // First value initializes
        assert_eq!(encoder.encode(0.5), 0);

        // Same value = no spike
        assert_eq!(encoder.encode(0.55), 0); // delta < threshold

        // Increase above threshold
        assert_eq!(encoder.encode(0.7), 1);

        // Decrease
        assert_eq!(encoder.encode(0.5), -1);
    }

    #[test]
    fn test_population_encoder() {
        let encoder = PopulationEncoder::uniform(5, 0.0, 1.0, 0.2);

        // Response to middle value
        let responses = encoder.encode(0.5);
        assert_eq!(responses.len(), 5);

        // Middle neuron should have highest response
        let max_idx = responses
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i)
            .unwrap();
        assert_eq!(max_idx, 2); // Middle neuron prefers 0.5
    }

    #[test]
    fn test_population_decode() {
        let encoder = PopulationEncoder::uniform(5, 0.0, 1.0, 0.2);

        let value = 0.75;
        let responses = encoder.encode(value);
        let decoded = encoder.decode(&responses);

        // Decoded should be close to original
        assert!((decoded - value).abs() < 0.15);
    }

    #[test]
    fn test_sparse_spikes() {
        let mut sparse = SparseSpikes::new(100);

        sparse.add_spike("n1".to_string(), Duration::from_millis(10));
        sparse.add_spike("n5".to_string(), Duration::from_millis(10));
        sparse.add_spike("n99".to_string(), Duration::from_millis(10));

        assert_eq!(sparse.active_count(), 3);
        assert!(sparse.sparsity() > 0.95); // 97% sparse

        assert!(sparse.is_active(&"n1".to_string()));
        assert!(!sparse.is_active(&"n50".to_string()));
    }

    #[test]
    fn test_sparse_to_dense() {
        let mut sparse = SparseSpikes::new(5);
        sparse.add_spike("n0".to_string(), Duration::ZERO);
        sparse.add_spike("n2".to_string(), Duration::ZERO);
        sparse.add_spike("n4".to_string(), Duration::ZERO);

        let id_map: std::collections::HashMap<String, usize> = [
            ("n0".to_string(), 0),
            ("n1".to_string(), 1),
            ("n2".to_string(), 2),
            ("n3".to_string(), 3),
            ("n4".to_string(), 4),
        ]
        .into_iter()
        .collect();

        let dense = sparse.to_dense(&id_map);
        assert_eq!(dense, vec![true, false, true, false, true]);
    }

    #[test]
    fn test_temporal_encoding_types() {
        let mut encoder = TemporalEncoder::default();

        // Test linear
        encoder.encoding = TemporalEncodingType::Linear;
        let lat_linear = encoder.latency(0.5);

        // Test logarithmic
        encoder.encoding = TemporalEncodingType::Logarithmic;
        let lat_log = encoder.latency(0.5);

        // Test exponential
        encoder.encoding = TemporalEncodingType::Exponential;
        let lat_exp = encoder.latency(0.5);

        // All should produce valid latencies
        assert!(lat_linear >= 0.0 && lat_linear <= encoder.max_latency_ms);
        assert!(lat_log >= 0.0 && lat_log <= encoder.max_latency_ms);
        assert!(lat_exp >= 0.0 && lat_exp <= encoder.max_latency_ms);
    }
}
