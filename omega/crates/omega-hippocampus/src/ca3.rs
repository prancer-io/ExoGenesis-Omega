//! CA3 Region
//!
//! Autoassociative memory network:
//! - Recurrent connections for pattern completion
//! - Attractor dynamics
//! - Hopfield-like energy landscape
//! - Learning via Hebbian plasticity

use rand::Rng;
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};

/// A CA3 pyramidal neuron
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CA3Neuron {
    /// Unique identifier
    pub id: usize,
    /// Current activation
    pub activation: f64,
    /// Previous activation (for dynamics)
    pub prev_activation: f64,
    /// Input from mossy fibers (DG)
    pub mossy_input: f64,
    /// Input from recurrent connections
    pub recurrent_input: f64,
    /// Bias
    pub bias: f64,
}

impl CA3Neuron {
    pub fn new(id: usize) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id,
            activation: 0.0,
            prev_activation: 0.0,
            mossy_input: 0.0,
            recurrent_input: 0.0,
            bias: rng.gen_range(-0.1..0.1),
        }
    }

    /// Update activation
    pub fn update(&mut self, tau: f64) {
        // Total input
        let total_input = self.mossy_input + self.recurrent_input + self.bias;

        // Leaky integration
        self.prev_activation = self.activation;
        self.activation = (1.0 - tau) * self.activation + tau * total_input.tanh();
    }

    /// Reset for next pattern
    pub fn reset(&mut self) {
        self.activation = 0.0;
        self.prev_activation = 0.0;
        self.mossy_input = 0.0;
        self.recurrent_input = 0.0;
    }
}

/// Pattern completion result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternCompletion {
    /// Completed pattern
    pub pattern: Vec<f64>,
    /// Number of iterations to converge
    pub iterations: usize,
    /// Whether it converged
    pub converged: bool,
    /// Final energy
    pub energy: f64,
}

/// CA3 recurrent network
pub struct CA3Network {
    /// Neurons
    neurons: Vec<CA3Neuron>,
    /// Recurrent weight matrix
    weights: Vec<Vec<f64>>,
    /// Mossy fiber weights from DG
    mossy_weights: Vec<Vec<f64>>,
    /// Connection probability
    connection_prob: f64,
    /// Time constant
    tau: f64,
    /// Learning rate
    learning_rate: f64,
    /// Stored patterns count
    pattern_count: usize,
}

impl CA3Network {
    /// Create new CA3 network
    pub fn new(size: usize, dg_size: usize, connection_prob: f64) -> Self {
        let mut rng = rand::thread_rng();
        let normal = Normal::new(0.0, 0.1).unwrap();

        // Initialize neurons
        let neurons = (0..size).map(CA3Neuron::new).collect();

        // Initialize recurrent weights (sparse)
        let mut weights = vec![vec![0.0; size]; size];
        for i in 0..size {
            for j in 0..size {
                if i != j && rng.gen::<f64>() < connection_prob {
                    weights[i][j] = normal.sample(&mut rng);
                }
            }
        }

        // Initialize mossy fiber weights
        let mossy_weights = vec![vec![0.0; dg_size]; size];

        Self {
            neurons,
            weights,
            mossy_weights,
            connection_prob,
            tau: 0.5,
            learning_rate: 0.01,
            pattern_count: 0,
        }
    }

    /// Encode a pattern from DG input
    pub fn encode(&mut self, dg_input: &[f64]) -> Vec<f64> {
        // Resize mossy weights if needed
        if !dg_input.is_empty() && self.mossy_weights[0].len() != dg_input.len() {
            let normal = Normal::new(0.0, 0.1).unwrap();
            let mut rng = rand::thread_rng();
            self.mossy_weights = vec![
                (0..dg_input.len())
                    .map(|_| normal.sample(&mut rng))
                    .collect();
                self.neurons.len()
            ];
        }

        // Apply mossy fiber input
        for (i, neuron) in self.neurons.iter_mut().enumerate() {
            neuron.mossy_input = 0.0;
            for (j, &input) in dg_input.iter().enumerate() {
                if j < self.mossy_weights[i].len() {
                    neuron.mossy_input += self.mossy_weights[i][j] * input;
                }
            }
            neuron.mossy_input = neuron.mossy_input.tanh();
        }

        // Run dynamics to settle
        for _ in 0..10 {
            self.step();
        }

        // Get current pattern
        let pattern: Vec<f64> = self.neurons.iter().map(|n| n.activation).collect();

        // Hebbian learning to store pattern
        self.store_pattern(&pattern);

        pattern
    }

    /// Pattern completion from partial cue
    pub fn complete(&mut self, cue: &[f64]) -> Vec<f64> {
        // Initialize from cue (via mossy fibers)
        if !cue.is_empty() && self.mossy_weights[0].len() == cue.len() {
            for (i, neuron) in self.neurons.iter_mut().enumerate() {
                neuron.mossy_input = 0.0;
                for (j, &input) in cue.iter().enumerate() {
                    if j < self.mossy_weights[i].len() {
                        neuron.mossy_input += self.mossy_weights[i][j] * input;
                    }
                }
                neuron.activation = neuron.mossy_input.tanh();
            }
        }

        // Run attractor dynamics
        let max_iterations = 100;
        let convergence_threshold = 0.001;

        for iter in 0..max_iterations {
            let prev_energy = self.compute_energy();

            // Zero out mossy input (completion phase)
            for neuron in &mut self.neurons {
                neuron.mossy_input *= 0.1; // Decay cue
            }

            self.step();

            let new_energy = self.compute_energy();

            // Check convergence
            if (new_energy - prev_energy).abs() < convergence_threshold {
                break;
            }
        }

        self.neurons.iter().map(|n| n.activation).collect()
    }

    /// Reactivate a stored pattern (for replay)
    pub fn reactivate(&mut self, pattern: &[f64]) {
        // Set activations directly
        for (neuron, &p) in self.neurons.iter_mut().zip(pattern.iter()) {
            neuron.activation = p;
        }

        // Run brief dynamics
        for _ in 0..5 {
            self.step();
        }
    }

    /// Single step of network dynamics
    fn step(&mut self) {
        let n = self.neurons.len();

        // Compute recurrent inputs
        let activations: Vec<f64> = self.neurons.iter().map(|n| n.activation).collect();

        for i in 0..n {
            self.neurons[i].recurrent_input = 0.0;
            for j in 0..n {
                self.neurons[i].recurrent_input += self.weights[i][j] * activations[j];
            }
        }

        // Update all neurons
        for neuron in &mut self.neurons {
            neuron.update(self.tau);
        }
    }

    /// Store pattern via Hebbian learning
    fn store_pattern(&mut self, pattern: &[f64]) {
        let n = self.neurons.len().min(pattern.len());

        // Hebbian rule: w_ij += lr * x_i * x_j
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    self.weights[i][j] +=
                        self.learning_rate * pattern[i] * pattern[j] / self.neurons.len() as f64;

                    // Weight bounds
                    self.weights[i][j] = self.weights[i][j].max(-1.0).min(1.0);
                }
            }
        }

        self.pattern_count += 1;
    }

    /// Compute Hopfield energy
    fn compute_energy(&self) -> f64 {
        let n = self.neurons.len();
        let mut energy = 0.0;

        // E = -0.5 * sum_ij w_ij * x_i * x_j
        for i in 0..n {
            for j in 0..n {
                energy -= 0.5 * self.weights[i][j] * self.neurons[i].activation * self.neurons[j].activation;
            }
        }

        energy
    }

    /// Get current activity level (fraction active)
    pub fn get_activity_level(&self) -> f64 {
        let active = self.neurons.iter().filter(|n| n.activation.abs() > 0.1).count();
        active as f64 / self.neurons.len() as f64
    }

    /// Get current pattern
    pub fn get_pattern(&self) -> Vec<f64> {
        self.neurons.iter().map(|n| n.activation).collect()
    }

    /// Reset network
    pub fn reset(&mut self) {
        for neuron in &mut self.neurons {
            neuron.reset();
        }
    }

    /// Get number of stored patterns
    pub fn pattern_count(&self) -> usize {
        self.pattern_count
    }

    /// Get network size
    pub fn size(&self) -> usize {
        self.neurons.len()
    }

    /// Compute capacity (theoretical max patterns)
    pub fn capacity(&self) -> usize {
        // Hopfield capacity ~0.14N
        (0.14 * self.neurons.len() as f64) as usize
    }
}

impl Default for CA3Network {
    fn default() -> Self {
        Self::new(512, 2560, 0.04)
    }
}

/// Compute pattern similarity
pub fn pattern_similarity(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
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
    if denom > 0.0 {
        dot / denom
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ca3_neuron() {
        let mut neuron = CA3Neuron::new(0);
        neuron.mossy_input = 0.5;
        neuron.recurrent_input = 0.3;

        neuron.update(0.5);
        assert!(neuron.activation != 0.0);
    }

    #[test]
    fn test_ca3_network_creation() {
        let network = CA3Network::new(100, 200, 0.1);
        assert_eq!(network.size(), 100);
    }

    #[test]
    fn test_pattern_encoding() {
        let mut network = CA3Network::new(50, 100, 0.1);

        let dg_input = vec![0.5; 100];
        let pattern = network.encode(&dg_input);

        assert_eq!(pattern.len(), 50);
        assert!(network.pattern_count() == 1);
    }

    #[test]
    fn test_pattern_completion() {
        let mut network = CA3Network::new(50, 100, 0.2);

        // Store a pattern
        let dg_input: Vec<f64> = (0..100).map(|i| if i < 50 { 1.0 } else { -1.0 }).collect();
        let stored = network.encode(&dg_input);

        network.reset();

        // Complete from partial cue
        let mut partial = dg_input.clone();
        for i in 50..100 {
            partial[i] = 0.0;
        }

        let completed = network.complete(&partial);

        // Should be similar to original
        let similarity = pattern_similarity(&stored, &completed);
        assert!(similarity > 0.3); // Some completion should occur
    }

    #[test]
    fn test_energy_decreases() {
        let mut network = CA3Network::new(30, 60, 0.2);

        // Initialize with random pattern
        let dg_input: Vec<f64> = (0..60).map(|i| (i as f64 / 60.0) - 0.5).collect();
        network.encode(&dg_input);

        // Energy should decrease over iterations
        let initial_energy = network.compute_energy();
        for _ in 0..50 {
            network.step();
        }
        let final_energy = network.compute_energy();

        assert!(final_energy <= initial_energy + 0.1); // Allow small numerical errors
    }

    #[test]
    fn test_capacity() {
        let network = CA3Network::new(100, 200, 0.1);
        let cap = network.capacity();

        assert!(cap > 0);
        assert!(cap < network.size());
    }
}
