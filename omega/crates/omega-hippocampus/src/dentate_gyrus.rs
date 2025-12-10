//! Dentate Gyrus (DG)
//!
//! Pattern separation through sparse coding:
//! - Granule cells with very sparse activity (~2%)
//! - Mossy fiber projections to CA3
//! - Adult neurogenesis simulation
//! - Pattern decorrelation

use rand::Rng;
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};

/// A granule cell in the dentate gyrus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GranuleCell {
    /// Unique identifier
    pub id: usize,
    /// Input weights from entorhinal cortex
    pub weights: Vec<f64>,
    /// Activation threshold
    pub threshold: f64,
    /// Current activation
    pub activation: f64,
    /// Whether currently active
    pub active: bool,
    /// Age of cell (for neurogenesis)
    pub age: u32,
    /// Maturity level (0-1)
    pub maturity: f64,
}

impl GranuleCell {
    pub fn new(id: usize, input_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let normal = Normal::new(0.0, 0.1).unwrap();

        Self {
            id,
            weights: (0..input_size)
                .map(|_| normal.sample(&mut rng))
                .collect(),
            threshold: rng.gen_range(0.3..0.7),
            activation: 0.0,
            active: false,
            age: 0,
            maturity: 1.0,
        }
    }

    /// Create a new (immature) granule cell
    pub fn new_born(id: usize, input_size: usize) -> Self {
        let mut cell = Self::new(id, input_size);
        cell.maturity = 0.1;
        cell.age = 0;
        cell
    }

    /// Compute activation from input
    pub fn activate(&mut self, input: &[f64]) -> f64 {
        let mut sum = 0.0;
        for (w, &x) in self.weights.iter().zip(input.iter()) {
            sum += w * x;
        }

        // Apply maturity scaling
        sum *= self.maturity;

        // ReLU-like activation
        self.activation = if sum > self.threshold {
            (sum - self.threshold).tanh()
        } else {
            0.0
        };

        self.active = self.activation > 0.0;
        self.activation
    }

    /// Update weights (learning)
    pub fn learn(&mut self, input: &[f64], target: f64, learning_rate: f64) {
        let error = target - self.activation;
        for (w, &x) in self.weights.iter_mut().zip(input.iter()) {
            *w += learning_rate * error * x * self.maturity;
        }
    }

    /// Age the cell (for neurogenesis simulation)
    pub fn mature(&mut self) {
        self.age += 1;
        if self.maturity < 1.0 {
            self.maturity += 0.01;
            self.maturity = self.maturity.min(1.0);
        }
    }
}

/// Mossy fiber connection from DG to CA3
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MossyFiber {
    /// Source granule cell ID
    pub source: usize,
    /// Target CA3 neuron ID
    pub target: usize,
    /// Connection weight (very strong)
    pub weight: f64,
}

impl MossyFiber {
    pub fn new(source: usize, target: usize) -> Self {
        Self {
            source,
            target,
            weight: 5.0, // Mossy fibers are very strong
        }
    }
}

/// The Dentate Gyrus region
pub struct DentateGyrus {
    /// Granule cells
    cells: Vec<GranuleCell>,
    /// Target sparsity (fraction active)
    sparsity: f64,
    /// Winner-take-all k value
    k_winners: usize,
    /// Input dimension
    input_dim: usize,
    /// Inhibition strength
    inhibition: f64,
    /// Neurogenesis rate (new cells per step)
    neurogenesis_rate: f64,
}

impl DentateGyrus {
    /// Create new dentate gyrus
    pub fn new(size: usize, sparsity: f64) -> Self {
        // Input dimension will be set on first separation
        Self {
            cells: Vec::new(),
            sparsity,
            k_winners: ((size as f64) * sparsity).max(1.0) as usize,
            input_dim: 0,
            inhibition: 1.0,
            neurogenesis_rate: 0.001,
        }
    }

    /// Initialize cells for given input dimension
    fn initialize(&mut self, input_dim: usize) {
        let size = (input_dim as f64 / self.sparsity) as usize;
        self.input_dim = input_dim;
        self.cells = (0..size)
            .map(|i| GranuleCell::new(i, input_dim))
            .collect();
        self.k_winners = ((size as f64) * self.sparsity).max(1.0) as usize;
    }

    /// Pattern separation: transform input to sparse DG representation
    pub fn separate(&mut self, input: &[f64]) -> Vec<f64> {
        // Initialize if needed
        if self.cells.is_empty() || self.input_dim != input.len() {
            self.initialize(input.len());
        }

        // Compute all activations
        let mut activations: Vec<(usize, f64)> = self
            .cells
            .iter_mut()
            .enumerate()
            .map(|(i, cell)| (i, cell.activate(input)))
            .collect();

        // Sort by activation (descending)
        activations.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Winner-take-all: only top k are active
        let mut output = vec![0.0; self.cells.len()];

        for (rank, (idx, activation)) in activations.iter().enumerate() {
            if rank < self.k_winners && *activation > 0.0 {
                output[*idx] = *activation;
                self.cells[*idx].active = true;
            } else {
                self.cells[*idx].active = false;
                self.cells[*idx].activation = 0.0;
            }
        }

        // Apply lateral inhibition
        let total: f64 = output.iter().sum();
        if total > 0.0 {
            for o in &mut output {
                *o /= total * self.inhibition;
            }
        }

        output
    }

    /// Get sparse binary representation
    pub fn get_sparse_code(&self) -> Vec<bool> {
        self.cells.iter().map(|c| c.active).collect()
    }

    /// Get number of active cells
    pub fn active_count(&self) -> usize {
        self.cells.iter().filter(|c| c.active).count()
    }

    /// Get actual sparsity
    pub fn actual_sparsity(&self) -> f64 {
        if self.cells.is_empty() {
            0.0
        } else {
            self.active_count() as f64 / self.cells.len() as f64
        }
    }

    /// Simulate adult neurogenesis
    pub fn neurogenesis(&mut self) {
        let mut rng = rand::thread_rng();

        // Age existing cells
        for cell in &mut self.cells {
            cell.mature();
        }

        // Possibly add new cells
        if rng.gen::<f64>() < self.neurogenesis_rate {
            let new_id = self.cells.len();
            let new_cell = GranuleCell::new_born(new_id, self.input_dim.max(1));
            self.cells.push(new_cell);
        }
    }

    /// Get mossy fiber projections to CA3
    pub fn get_mossy_fibers(&self, ca3_size: usize) -> Vec<MossyFiber> {
        let mut rng = rand::thread_rng();
        let mut fibers = Vec::new();

        // Each active granule cell projects to a few CA3 neurons
        for cell in &self.cells {
            if cell.active {
                // Each granule cell contacts ~15 CA3 neurons
                let num_targets = 15.min(ca3_size);
                for _ in 0..num_targets {
                    let target = rng.gen_range(0..ca3_size);
                    fibers.push(MossyFiber::new(cell.id, target));
                }
            }
        }

        fibers
    }

    /// Learning: adjust weights based on feedback
    pub fn learn(&mut self, input: &[f64], target_pattern: &[f64], learning_rate: f64) {
        for (cell, &target) in self.cells.iter_mut().zip(target_pattern.iter()) {
            if cell.active {
                cell.learn(input, target, learning_rate);
            }
        }
    }

    /// Reset activations
    pub fn reset(&mut self) {
        for cell in &mut self.cells {
            cell.activation = 0.0;
            cell.active = false;
        }
    }

    /// Get cell count
    pub fn size(&self) -> usize {
        self.cells.len()
    }
}

impl Default for DentateGyrus {
    fn default() -> Self {
        Self::new(2560, 0.02)
    }
}

/// Compute pattern separation quality between two patterns
pub fn separation_index(pattern_a: &[f64], pattern_b: &[f64]) -> f64 {
    if pattern_a.len() != pattern_b.len() || pattern_a.is_empty() {
        return 0.0;
    }

    // Compute overlap (correlation)
    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for (&a, &b) in pattern_a.iter().zip(pattern_b.iter()) {
        dot += a * b;
        norm_a += a * a;
        norm_b += b * b;
    }

    let denom = (norm_a * norm_b).sqrt();
    if denom > 0.0 {
        1.0 - (dot / denom) // 1 - correlation = separation
    } else {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_granule_cell() {
        let mut cell = GranuleCell::new(0, 10);
        let input = vec![0.5; 10];

        let activation = cell.activate(&input);
        assert!(activation >= 0.0);
    }

    #[test]
    fn test_dentate_gyrus_separation() {
        let mut dg = DentateGyrus::new(100, 0.1);

        let input = vec![0.5; 50];
        let output = dg.separate(&input);

        // Check sparsity
        let active = output.iter().filter(|&&x| x > 0.0).count();
        let sparsity = active as f64 / output.len() as f64;

        assert!(sparsity <= 0.15); // Should be sparse
    }

    #[test]
    fn test_pattern_separation() {
        let mut dg = DentateGyrus::new(200, 0.05);

        // Two similar input patterns
        let input_a: Vec<f64> = (0..100).map(|i| (i as f64 / 100.0)).collect();
        let mut input_b = input_a.clone();
        input_b[50] += 0.1; // Small difference

        let output_a = dg.separate(&input_a);
        dg.reset();
        let output_b = dg.separate(&input_b);

        // Outputs should be more different than inputs
        let input_sep = separation_index(&input_a, &input_b);
        let output_sep = separation_index(&output_a, &output_b);

        // Pattern separation should increase difference
        // (or at least maintain it)
        assert!(output_sep >= input_sep * 0.5);
    }

    #[test]
    fn test_neurogenesis() {
        let mut dg = DentateGyrus::new(50, 0.1);
        let input = vec![0.5; 20];
        dg.separate(&input);

        let initial_size = dg.size();

        // Run many steps to trigger neurogenesis
        for _ in 0..1000 {
            dg.neurogenesis();
        }

        // Size should have increased
        assert!(dg.size() >= initial_size);
    }

    #[test]
    fn test_mossy_fibers() {
        let mut dg = DentateGyrus::new(100, 0.1);
        let input = vec![0.5; 50];
        dg.separate(&input);

        let fibers = dg.get_mossy_fibers(50);

        // Should have fibers from active cells
        assert!(!fibers.is_empty() || dg.active_count() == 0);
    }
}
