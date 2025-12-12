//! CA1 Region
//!
//! Output layer of hippocampus:
//! - Receives from CA3 (Schaffer collaterals)
//! - Receives from EC layer III (temporoammonic path)
//! - Projects to subiculum and EC
//! - Involved in memory consolidation

use rand::Rng;
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};

/// A CA1 pyramidal neuron
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CA1Neuron {
    /// Unique identifier
    pub id: usize,
    /// Current activation
    pub activation: f64,
    /// Weights from CA3 (Schaffer collaterals)
    pub schaffer_weights: Vec<f64>,
    /// Weights from EC (temporoammonic)
    pub ta_weights: Vec<f64>,
    /// Bias
    pub bias: f64,
    /// Plasticity state
    pub eligibility_trace: f64,
}

impl CA1Neuron {
    pub fn new(id: usize, ca3_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let normal = Normal::new(0.0, 0.1).unwrap();

        Self {
            id,
            activation: 0.0,
            schaffer_weights: (0..ca3_size).map(|_| normal.sample(&mut rng)).collect(),
            ta_weights: Vec::new(),
            bias: rng.gen_range(-0.1..0.1),
            eligibility_trace: 0.0,
        }
    }

    /// Compute activation from CA3 input
    pub fn activate(&mut self, ca3_input: &[f64]) -> f64 {
        let mut sum = self.bias;

        // Schaffer collateral input
        for (w, &x) in self.schaffer_weights.iter().zip(ca3_input.iter()) {
            sum += w * x;
        }

        self.activation = sum.tanh();
        self.activation
    }

    /// Compute with both CA3 and EC input
    pub fn activate_with_ec(&mut self, ca3_input: &[f64], ec_input: &[f64]) -> f64 {
        let mut sum = self.bias;

        // Schaffer collateral input
        for (w, &x) in self.schaffer_weights.iter().zip(ca3_input.iter()) {
            sum += w * x;
        }

        // Temporoammonic input
        for (w, &x) in self.ta_weights.iter().zip(ec_input.iter()) {
            sum += w * x * 0.5; // Weaker than Schaffer
        }

        self.activation = sum.tanh();
        self.activation
    }

    /// Update eligibility trace
    pub fn update_trace(&mut self, decay: f64) {
        self.eligibility_trace = decay * self.eligibility_trace + (1.0 - decay) * self.activation;
    }

    /// Learn from CA3 input
    pub fn learn(&mut self, ca3_input: &[f64], target: f64, learning_rate: f64) {
        let error = target - self.activation;
        for (w, &x) in self.schaffer_weights.iter_mut().zip(ca3_input.iter()) {
            *w += learning_rate * error * x;
            *w = w.clamp(-2.0, 2.0);
        }
    }

    /// Reset activation
    pub fn reset(&mut self) {
        self.activation = 0.0;
        self.eligibility_trace = 0.0;
    }
}

/// Output from CA1 layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CA1Output {
    /// Activation pattern
    pub pattern: Vec<f64>,
    /// Population activity level
    pub activity: f64,
    /// Temporal context signal
    pub temporal_context: Vec<f64>,
}

/// CA1 layer
pub struct CA1Layer {
    /// Neurons
    neurons: Vec<CA1Neuron>,
    /// Input dimension (CA3 size)
    ca3_size: usize,
    /// Temporal context buffer
    temporal_buffer: Vec<Vec<f64>>,
    /// Buffer size
    buffer_size: usize,
}

impl CA1Layer {
    /// Create new CA1 layer
    pub fn new(size: usize, ca3_size: usize) -> Self {
        let neurons = (0..size).map(|i| CA1Neuron::new(i, ca3_size)).collect();

        Self {
            neurons,
            ca3_size,
            temporal_buffer: Vec::with_capacity(10),
            buffer_size: 10,
        }
    }

    /// Process CA3 input
    pub fn process(&mut self, ca3_input: &[f64]) -> Vec<f64> {
        // Resize weights if needed
        if !ca3_input.is_empty() && self.neurons[0].schaffer_weights.len() != ca3_input.len() {
            let normal = Normal::new(0.0, 0.1).unwrap();
            let mut rng = rand::thread_rng();
            for neuron in &mut self.neurons {
                neuron.schaffer_weights = (0..ca3_input.len())
                    .map(|_| normal.sample(&mut rng))
                    .collect();
            }
            self.ca3_size = ca3_input.len();
        }

        // Activate all neurons
        let output: Vec<f64> = self
            .neurons
            .iter_mut()
            .map(|n| n.activate(ca3_input))
            .collect();

        // Update temporal buffer
        self.temporal_buffer.push(output.clone());
        if self.temporal_buffer.len() > self.buffer_size {
            self.temporal_buffer.remove(0);
        }

        output
    }

    /// Process with temporal integration
    pub fn process_temporal(&mut self, ca3_input: &[f64]) -> CA1Output {
        let pattern = self.process(ca3_input);

        // Compute activity level
        let activity = pattern.iter().map(|x| x.abs()).sum::<f64>() / pattern.len() as f64;

        // Compute temporal context (weighted average of recent patterns)
        let temporal_context = self.compute_temporal_context();

        CA1Output {
            pattern,
            activity,
            temporal_context,
        }
    }

    /// Compute temporal context from buffer
    fn compute_temporal_context(&self) -> Vec<f64> {
        if self.temporal_buffer.is_empty() {
            return vec![0.0; self.neurons.len()];
        }

        let mut context = vec![0.0; self.neurons.len()];
        let mut total_weight = 0.0;

        for (i, pattern) in self.temporal_buffer.iter().enumerate() {
            let weight = (i + 1) as f64; // More recent = higher weight
            total_weight += weight;

            for (j, &p) in pattern.iter().enumerate() {
                if j < context.len() {
                    context[j] += weight * p;
                }
            }
        }

        if total_weight > 0.0 {
            for c in &mut context {
                *c /= total_weight;
            }
        }

        context
    }

    /// Learn from target output
    pub fn learn(&mut self, ca3_input: &[f64], target: &[f64], learning_rate: f64) {
        for (neuron, &t) in self.neurons.iter_mut().zip(target.iter()) {
            neuron.learn(ca3_input, t, learning_rate);
        }
    }

    /// Update eligibility traces
    pub fn update_traces(&mut self, decay: f64) {
        for neuron in &mut self.neurons {
            neuron.update_trace(decay);
        }
    }

    /// Get current output pattern
    pub fn get_pattern(&self) -> Vec<f64> {
        self.neurons.iter().map(|n| n.activation).collect()
    }

    /// Get activity level
    pub fn get_activity(&self) -> f64 {
        let sum: f64 = self.neurons.iter().map(|n| n.activation.abs()).sum();
        sum / self.neurons.len() as f64
    }

    /// Reset layer
    pub fn reset(&mut self) {
        for neuron in &mut self.neurons {
            neuron.reset();
        }
        self.temporal_buffer.clear();
    }

    /// Get layer size
    pub fn size(&self) -> usize {
        self.neurons.len()
    }
}

impl Default for CA1Layer {
    fn default() -> Self {
        Self::new(256, 512)
    }
}

/// Compute population vector from CA1 activity
pub fn compute_population_vector(activations: &[f64], preferred_directions: &[f64]) -> f64 {
    if activations.len() != preferred_directions.len() || activations.is_empty() {
        return 0.0;
    }

    let mut sum_x = 0.0;
    let mut sum_y = 0.0;

    for (&a, &dir) in activations.iter().zip(preferred_directions.iter()) {
        sum_x += a * dir.cos();
        sum_y += a * dir.sin();
    }

    sum_y.atan2(sum_x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ca1_neuron() {
        let mut neuron = CA1Neuron::new(0, 10);
        let input = vec![0.5; 10];

        let activation = neuron.activate(&input);
        assert!(activation >= -1.0 && activation <= 1.0);
    }

    #[test]
    fn test_ca1_layer() {
        let mut layer = CA1Layer::new(20, 50);
        let ca3_input = vec![0.3; 50];

        let output = layer.process(&ca3_input);
        assert_eq!(output.len(), 20);
    }

    #[test]
    fn test_temporal_processing() {
        let mut layer = CA1Layer::new(10, 20);

        // Process multiple inputs
        for i in 0..5 {
            let input: Vec<f64> = (0..20).map(|j| ((i + j) % 20) as f64 / 20.0).collect();
            layer.process(&input);
        }

        let output = layer.process_temporal(&vec![0.5; 20]);
        assert!(!output.temporal_context.is_empty());
    }

    #[test]
    fn test_learning() {
        let mut layer = CA1Layer::new(5, 10);
        let input = vec![0.5; 10];
        let target = vec![0.8, -0.3, 0.5, 0.0, -0.7];

        layer.process(&input);
        let initial = layer.get_pattern();

        // Learn
        for _ in 0..100 {
            layer.process(&input);
            layer.learn(&input, &target, 0.1);
        }

        let learned = layer.get_pattern();

        // Should be closer to target
        let initial_error: f64 = initial
            .iter()
            .zip(target.iter())
            .map(|(a, t)| (a - t).powi(2))
            .sum();
        let learned_error: f64 = learned
            .iter()
            .zip(target.iter())
            .map(|(a, t)| (a - t).powi(2))
            .sum();

        assert!(learned_error < initial_error);
    }

    #[test]
    fn test_population_vector() {
        let activations = vec![1.0, 0.0, 0.0, 0.0];
        let directions = vec![0.0, std::f64::consts::FRAC_PI_2, std::f64::consts::PI, 3.0 * std::f64::consts::FRAC_PI_2];

        let pv = compute_population_vector(&activations, &directions);
        assert!((pv - 0.0).abs() < 0.1); // Should point to 0 radians
    }
}
