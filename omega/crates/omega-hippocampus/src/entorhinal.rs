//! Entorhinal Cortex (EC)
//!
//! Gateway to hippocampus:
//! - Layer II → DG and CA3 (perforant path)
//! - Layer III → CA1 (temporoammonic path)
//! - Layer V/VI receives hippocampal output
//! - Grid cells for spatial representation

use rand::Rng;
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// A grid cell for spatial representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridCell {
    /// Unique identifier
    pub id: usize,
    /// Grid spacing (lambda)
    pub spacing: f64,
    /// Grid orientation (theta)
    pub orientation: f64,
    /// Phase offset (x, y)
    pub phase: (f64, f64),
    /// Current activation
    pub activation: f64,
}

impl GridCell {
    pub fn new(id: usize, spacing: f64, orientation: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id,
            spacing,
            orientation,
            phase: (rng.gen_range(0.0..spacing), rng.gen_range(0.0..spacing)),
            activation: 0.0,
        }
    }

    /// Compute grid cell activation for position (x, y)
    pub fn compute(&mut self, x: f64, y: f64) -> f64 {
        // Grid cell firing pattern is sum of 3 cosines at 60° angles
        let mut sum = 0.0;

        for i in 0..3 {
            let theta = self.orientation + (i as f64) * PI / 3.0;
            let projected = (x - self.phase.0) * theta.cos() + (y - self.phase.1) * theta.sin();
            sum += (2.0 * PI * projected / self.spacing).cos();
        }

        // Normalize to [0, 1]
        self.activation = (sum / 3.0 + 1.0) / 2.0;
        self.activation
    }
}

/// Perforant path connection from EC to hippocampus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerforantPath {
    /// Source EC neuron
    pub source: usize,
    /// Target DG/CA3 neuron
    pub target: usize,
    /// Connection weight
    pub weight: f64,
    /// Synaptic delay (ms)
    pub delay: f64,
}

/// Entorhinal Cortex
pub struct EntorhinalCortex {
    /// Input dimension
    input_dim: usize,
    /// Output dimension (to DG)
    output_dim: usize,
    /// Layer II neurons (→ DG, CA3)
    layer2_weights: Vec<Vec<f64>>,
    /// Layer III neurons (→ CA1)
    layer3_weights: Vec<Vec<f64>>,
    /// Grid cells for spatial representation
    grid_cells: Vec<GridCell>,
    /// Current layer II output
    layer2_output: Vec<f64>,
    /// Current layer III output
    layer3_output: Vec<f64>,
    /// Decoding weights (hippocampus → EC output)
    decode_weights: Vec<Vec<f64>>,
}

impl EntorhinalCortex {
    /// Create new entorhinal cortex
    pub fn new(input_dim: usize, output_dim: usize) -> Self {
        let mut rng = rand::thread_rng();
        let normal = Normal::new(0.0, 0.1).unwrap();

        // Layer II weights (perforant path to DG)
        let layer2_weights: Vec<Vec<f64>> = (0..output_dim)
            .map(|_| (0..input_dim).map(|_| normal.sample(&mut rng)).collect())
            .collect();

        // Layer III weights (temporoammonic to CA1)
        let layer3_weights: Vec<Vec<f64>> = (0..output_dim / 2)
            .map(|_| (0..input_dim).map(|_| normal.sample(&mut rng)).collect())
            .collect();

        // Decode weights
        let decode_weights: Vec<Vec<f64>> = (0..input_dim)
            .map(|_| (0..output_dim / 2).map(|_| normal.sample(&mut rng)).collect())
            .collect();

        // Create grid cells with various spacings
        let mut grid_cells = Vec::new();
        let spacings = [20.0, 35.0, 60.0, 100.0]; // Different spatial scales

        for (i, &spacing) in spacings.iter().enumerate() {
            for j in 0..10 {
                let orientation = rng.gen_range(0.0..PI / 3.0);
                grid_cells.push(GridCell::new(i * 10 + j, spacing, orientation));
            }
        }

        Self {
            input_dim,
            output_dim,
            layer2_weights,
            layer3_weights,
            grid_cells,
            layer2_output: vec![0.0; output_dim],
            layer3_output: vec![0.0; output_dim / 2],
            decode_weights,
        }
    }

    /// Process input through EC to produce perforant path output
    pub fn process(&mut self, input: &[f64]) -> Vec<f64> {
        // Resize weights if input dimension changed
        if input.len() != self.input_dim {
            self.resize_input(input.len());
        }

        // Layer II processing (→ DG)
        self.layer2_output = self
            .layer2_weights
            .iter()
            .map(|weights| {
                let sum: f64 = weights.iter().zip(input.iter()).map(|(w, &x)| w * x).sum();
                sum.tanh()
            })
            .collect();

        // Layer III processing (→ CA1)
        self.layer3_output = self
            .layer3_weights
            .iter()
            .map(|weights| {
                let sum: f64 = weights.iter().zip(input.iter()).map(|(w, &x)| w * x).sum();
                sum.tanh()
            })
            .collect();

        self.layer2_output.clone()
    }

    /// Process with spatial position (incorporates grid cells)
    pub fn process_spatial(&mut self, input: &[f64], x: f64, y: f64) -> Vec<f64> {
        let mut output = self.process(input);

        // Add grid cell contribution
        let grid_output: Vec<f64> = self
            .grid_cells
            .iter_mut()
            .map(|gc| gc.compute(x, y))
            .collect();

        // Combine nonspatial and spatial
        let grid_mean: f64 = grid_output.iter().sum::<f64>() / grid_output.len() as f64;

        for o in &mut output {
            *o = (*o + grid_mean) / 2.0;
        }

        output
    }

    /// Get layer III output (for CA1)
    pub fn get_layer3_output(&self) -> &[f64] {
        &self.layer3_output
    }

    /// Decode hippocampal output back to input space
    pub fn decode(&self, hippo_output: &[f64]) -> Vec<f64> {
        self.decode_weights
            .iter()
            .map(|weights| {
                let sum: f64 = weights
                    .iter()
                    .zip(hippo_output.iter())
                    .map(|(w, &x)| w * x)
                    .sum();
                sum.tanh()
            })
            .collect()
    }

    /// Learn encoding weights
    pub fn learn_encoding(&mut self, input: &[f64], target_output: &[f64], learning_rate: f64) {
        for (i, weights) in self.layer2_weights.iter_mut().enumerate() {
            if i < target_output.len() {
                let error = target_output[i] - self.layer2_output.get(i).copied().unwrap_or(0.0);
                for (w, &x) in weights.iter_mut().zip(input.iter()) {
                    *w += learning_rate * error * x;
                    *w = w.max(-2.0).min(2.0);
                }
            }
        }
    }

    /// Learn decoding weights
    pub fn learn_decoding(&mut self, hippo_output: &[f64], target_input: &[f64], learning_rate: f64) {
        let decoded = self.decode(hippo_output);

        for (i, weights) in self.decode_weights.iter_mut().enumerate() {
            if i < target_input.len() && i < decoded.len() {
                let error = target_input[i] - decoded[i];
                for (w, &h) in weights.iter_mut().zip(hippo_output.iter()) {
                    *w += learning_rate * error * h;
                    *w = w.max(-2.0).min(2.0);
                }
            }
        }
    }

    /// Resize for new input dimension
    fn resize_input(&mut self, new_dim: usize) {
        let mut rng = rand::thread_rng();
        let normal = Normal::new(0.0, 0.1).unwrap();

        self.input_dim = new_dim;

        // Resize layer 2 weights
        for weights in &mut self.layer2_weights {
            weights.resize_with(new_dim, || normal.sample(&mut rng));
        }

        // Resize layer 3 weights
        for weights in &mut self.layer3_weights {
            weights.resize_with(new_dim, || normal.sample(&mut rng));
        }
    }

    /// Get grid cell activity pattern
    pub fn get_grid_pattern(&self) -> Vec<f64> {
        self.grid_cells.iter().map(|gc| gc.activation).collect()
    }

    /// Get input dimension
    pub fn input_dim(&self) -> usize {
        self.input_dim
    }

    /// Get output dimension
    pub fn output_dim(&self) -> usize {
        self.output_dim
    }
}

impl Default for EntorhinalCortex {
    fn default() -> Self {
        Self::new(256, 2560)
    }
}

/// Compute grid cell population vector for position decoding
pub fn decode_position_from_grid(grid_cells: &[GridCell]) -> (f64, f64) {
    if grid_cells.is_empty() {
        return (0.0, 0.0);
    }

    // Use weighted average of grid cell phases
    let mut x_sum = 0.0;
    let mut y_sum = 0.0;
    let mut weight_sum = 0.0;

    for gc in grid_cells {
        let weight = gc.activation;
        x_sum += weight * gc.phase.0;
        y_sum += weight * gc.phase.1;
        weight_sum += weight;
    }

    if weight_sum > 0.0 {
        (x_sum / weight_sum, y_sum / weight_sum)
    } else {
        (0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_cell() {
        let mut gc = GridCell::new(0, 30.0, 0.0);

        // Test at different positions
        let a1 = gc.compute(0.0, 0.0);
        let a2 = gc.compute(15.0, 0.0);
        let a3 = gc.compute(30.0, 0.0);

        // Activation should be periodic
        assert!((a1 - a3).abs() < 0.1); // Same phase
        assert!((a1 - a2).abs() > 0.1); // Different phase
    }

    #[test]
    fn test_entorhinal_cortex() {
        let mut ec = EntorhinalCortex::new(64, 256);

        let input = vec![0.5; 64];
        let output = ec.process(&input);

        assert_eq!(output.len(), 256);
    }

    #[test]
    fn test_spatial_processing() {
        let mut ec = EntorhinalCortex::new(32, 128);

        let input = vec![0.5; 32];

        let out1 = ec.process_spatial(&input, 0.0, 0.0);
        let out2 = ec.process_spatial(&input, 50.0, 50.0);

        // Outputs should differ based on position
        let diff: f64 = out1
            .iter()
            .zip(out2.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();
        assert!(diff > 0.0);
    }

    #[test]
    fn test_decode() {
        let ec = EntorhinalCortex::new(32, 64);

        let hippo_output = vec![0.5; 32];
        let decoded = ec.decode(&hippo_output);

        assert_eq!(decoded.len(), 32);
    }

    #[test]
    fn test_learning() {
        let mut ec = EntorhinalCortex::new(16, 32);

        let input = vec![0.5; 16];
        let target = vec![0.8; 32];

        // Learn
        for _ in 0..50 {
            ec.process(&input);
            ec.learn_encoding(&input, &target, 0.1);
        }

        let output = ec.process(&input);
        let mean: f64 = output.iter().sum::<f64>() / output.len() as f64;

        // Output should be closer to target mean (0.8)
        assert!(mean > 0.3);
    }
}
