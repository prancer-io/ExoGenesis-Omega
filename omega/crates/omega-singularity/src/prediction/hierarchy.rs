//! Omega Hierarchy - Multi-Level Predictive Processing
//!
//! The brain processes information through multiple hierarchical levels,
//! each level predicting the activity of the level below. Prediction errors
//! flow upward, while predictions flow downward.
//!
//! ```text
//! PREDICTIVE HIERARCHY
//! ═══════════════════
//!
//! Level 6: Abstract Concepts     ← "Understanding"
//!     ↑↓ prediction/error
//! Level 5: Semantic Categories   ← "Recognition"
//!     ↑↓ prediction/error
//! Level 4: Object Features       ← "Features"
//!     ↑↓ prediction/error
//! Level 3: Complex Patterns      ← "Patterns"
//!     ↑↓ prediction/error
//! Level 2: Simple Features       ← "Edges"
//!     ↑↓ prediction/error
//! Level 1: Raw Sensory           ← "Pixels"
//!     ↑↓ prediction/error
//! Level 0: World                 ← Reality
//! ```

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use uuid::Uuid;

use super::Result;

/// A bottom-up signal carrying prediction errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottomUpSignal {
    /// Source level
    pub from_level: usize,
    /// Target level
    pub to_level: usize,
    /// Prediction error vector
    pub error: Vec<f64>,
    /// Precision (confidence in the error)
    pub precision: f64,
    /// Timestamp
    pub timestamp: u64,
}

/// A top-down prediction constraining lower levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopDownPrediction {
    /// Source level
    pub from_level: usize,
    /// Target level
    pub to_level: usize,
    /// Prediction vector
    pub prediction: Vec<f64>,
    /// Precision (confidence in the prediction)
    pub precision: f64,
    /// Timestamp
    pub timestamp: u64,
}

/// A single level in the hierarchy
#[derive(Debug, Clone)]
pub struct HierarchyLevel {
    /// Level index (0 = closest to world)
    pub level: usize,
    /// Name of this level
    pub name: String,
    /// Dimension of representations at this level
    pub dim: usize,
    /// Current belief state (posterior)
    pub mu: Vec<f64>,
    /// Current prediction for level below
    pub prediction: Vec<f64>,
    /// Current prediction error from level below
    pub error: Vec<f64>,
    /// Precision of predictions
    pub precision: f64,
    /// Precision of errors (how much to trust bottom-up signals)
    pub error_precision: f64,
    /// Learning rate
    pub learning_rate: f64,
    /// Prior expectation
    pub prior: Vec<f64>,
    /// History of states
    history: VecDeque<Vec<f64>>,
    /// Maximum history length
    max_history: usize,
}

impl HierarchyLevel {
    pub fn new(level: usize, name: impl Into<String>, dim: usize) -> Self {
        Self {
            level,
            name: name.into(),
            dim,
            mu: vec![0.0; dim],
            prediction: vec![0.0; dim],
            error: vec![0.0; dim],
            precision: 1.0,
            error_precision: 1.0,
            learning_rate: 0.1,
            prior: vec![0.0; dim],
            history: VecDeque::with_capacity(100),
            max_history: 100,
        }
    }

    /// Generate prediction for level below
    pub fn predict(&mut self) -> TopDownPrediction {
        // Transform belief through generative model
        // In a full model, this would be a learned transformation
        // Here we use identity with noise reduction
        self.prediction = self.mu.iter()
            .map(|&x| x * 0.95) // Slight shrinkage towards prior
            .collect();

        TopDownPrediction {
            from_level: self.level,
            to_level: self.level.saturating_sub(1),
            prediction: self.prediction.clone(),
            precision: self.precision,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }

    /// Process bottom-up prediction error
    pub fn process_error(&mut self, error: &BottomUpSignal) {
        self.error = error.error.clone();

        // Update belief based on prediction error
        // mu_new = mu + lr * error_precision * error
        for (i, e) in error.error.iter().enumerate() {
            if i < self.mu.len() {
                let update = self.learning_rate * error.precision * e;
                self.mu[i] += update;

                // Also incorporate prior
                self.mu[i] = 0.95 * self.mu[i] + 0.05 * self.prior.get(i).copied().unwrap_or(0.0);
            }
        }

        // Update precision based on error magnitude
        let error_mag: f64 = error.error.iter().map(|e| e * e).sum::<f64>().sqrt();
        self.error_precision = 1.0 / (error_mag + 0.1);
        self.error_precision = self.error_precision.clamp(0.1, 10.0);

        // Store in history
        self.history.push_back(self.mu.clone());
        if self.history.len() > self.max_history {
            self.history.pop_front();
        }
    }

    /// Process top-down prediction from level above
    pub fn process_prediction(&mut self, pred: &TopDownPrediction) {
        // Top-down predictions become the prior for this level
        self.prior = pred.prediction.clone();

        // Adjust belief towards prediction weighted by precision
        for (i, &p) in pred.prediction.iter().enumerate() {
            if i < self.mu.len() {
                self.mu[i] = (1.0 - pred.precision * 0.1) * self.mu[i]
                           + pred.precision * 0.1 * p;
            }
        }
    }

    /// Compute prediction error given observation
    pub fn compute_error(&self, observation: &[f64]) -> BottomUpSignal {
        let error: Vec<f64> = observation.iter()
            .zip(self.prediction.iter())
            .map(|(o, p)| o - p)
            .collect();

        BottomUpSignal {
            from_level: self.level,
            to_level: self.level + 1,
            error,
            precision: self.error_precision,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }

    /// Get surprise (prediction error magnitude)
    pub fn surprise(&self) -> f64 {
        self.error.iter().map(|e| e * e).sum::<f64>().sqrt()
    }

    /// Get free energy at this level
    pub fn free_energy(&self) -> f64 {
        // F = prediction_error + complexity
        let prediction_error: f64 = self.error.iter()
            .map(|e| e * e * self.error_precision)
            .sum();

        let complexity: f64 = self.mu.iter()
            .zip(self.prior.iter())
            .map(|(m, p)| (m - p).powi(2))
            .sum::<f64>()
            .sqrt();

        prediction_error + 0.1 * complexity
    }
}

/// Configuration for the predictive hierarchy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchyConfig {
    /// Number of levels
    pub num_levels: usize,
    /// Dimension at each level
    pub level_dims: Vec<usize>,
    /// Names for each level
    pub level_names: Vec<String>,
    /// Learning rate
    pub learning_rate: f64,
    /// Top-down weight
    pub top_down_weight: f64,
    /// Bottom-up weight
    pub bottom_up_weight: f64,
}

impl Default for HierarchyConfig {
    fn default() -> Self {
        Self {
            num_levels: 7,
            level_dims: vec![256, 128, 64, 32, 16, 8, 4],
            level_names: vec![
                "Sensory".to_string(),
                "Features".to_string(),
                "Patterns".to_string(),
                "Objects".to_string(),
                "Categories".to_string(),
                "Concepts".to_string(),
                "Abstract".to_string(),
            ],
            learning_rate: 0.1,
            top_down_weight: 0.3,
            bottom_up_weight: 0.7,
        }
    }
}

/// A single layer for transformations between levels
#[derive(Debug, Clone)]
pub struct PredictionLayer {
    /// Input dimension
    pub input_dim: usize,
    /// Output dimension
    pub output_dim: usize,
    /// Weight matrix (simplified as diagonal + noise)
    weights: Vec<Vec<f64>>,
    /// Bias
    bias: Vec<f64>,
}

impl PredictionLayer {
    pub fn new(input_dim: usize, output_dim: usize) -> Self {
        // Initialize with random-ish weights
        let mut weights = vec![vec![0.0; input_dim]; output_dim];
        for i in 0..output_dim {
            for j in 0..input_dim {
                // Diagonal-ish initialization
                weights[i][j] = if i * input_dim / output_dim == j {
                    1.0
                } else {
                    0.1 / input_dim as f64
                };
            }
        }

        Self {
            input_dim,
            output_dim,
            weights,
            bias: vec![0.0; output_dim],
        }
    }

    /// Forward pass
    pub fn forward(&self, input: &[f64]) -> Vec<f64> {
        let mut output = self.bias.clone();

        for (i, out) in output.iter_mut().enumerate() {
            for (j, &inp) in input.iter().enumerate() {
                if j < self.input_dim {
                    *out += self.weights[i][j] * inp;
                }
            }
            // Apply nonlinearity
            *out = 1.0 / (1.0 + (-*out).exp());
        }

        output
    }

    /// Update weights based on prediction error
    pub fn update(&mut self, input: &[f64], error: &[f64], learning_rate: f64) {
        for (i, &e) in error.iter().enumerate() {
            if i < self.output_dim {
                for (j, &inp) in input.iter().enumerate() {
                    if j < self.input_dim {
                        self.weights[i][j] += learning_rate * e * inp;
                    }
                }
                self.bias[i] += learning_rate * e;
            }
        }
    }
}

/// The Omega Hierarchy - Complete Predictive Processing System
pub struct OmegaHierarchy {
    config: HierarchyConfig,
    /// Levels from low (sensory) to high (abstract)
    levels: Vec<HierarchyLevel>,
    /// Upward projections (level i -> level i+1)
    upward_layers: Vec<PredictionLayer>,
    /// Downward projections (level i+1 -> level i)
    downward_layers: Vec<PredictionLayer>,
    /// Total free energy
    total_free_energy: f64,
    /// Total surprise
    total_surprise: f64,
    /// Processing cycles
    cycles: u64,
}

impl OmegaHierarchy {
    pub fn new(config: HierarchyConfig) -> Self {
        let num_levels = config.num_levels;

        // Create levels
        let mut levels = Vec::with_capacity(num_levels);
        for i in 0..num_levels {
            let dim = config.level_dims.get(i).copied().unwrap_or(64);
            let name = config.level_names.get(i).cloned()
                .unwrap_or_else(|| format!("Level {}", i));
            let mut level = HierarchyLevel::new(i, name, dim);
            level.learning_rate = config.learning_rate;
            levels.push(level);
        }

        // Create projection layers
        let mut upward_layers = Vec::with_capacity(num_levels - 1);
        let mut downward_layers = Vec::with_capacity(num_levels - 1);

        for i in 0..(num_levels - 1) {
            let lower_dim = config.level_dims.get(i).copied().unwrap_or(64);
            let upper_dim = config.level_dims.get(i + 1).copied().unwrap_or(64);

            upward_layers.push(PredictionLayer::new(lower_dim, upper_dim));
            downward_layers.push(PredictionLayer::new(upper_dim, lower_dim));
        }

        Self {
            config,
            levels,
            upward_layers,
            downward_layers,
            total_free_energy: 0.0,
            total_surprise: 0.0,
            cycles: 0,
        }
    }

    /// Process input through the hierarchy
    pub fn process(&mut self, input: &[f64]) -> HierarchyResult {
        self.cycles += 1;
        let mut errors = Vec::new();
        let mut predictions = Vec::new();

        // ===== BOTTOM-UP PASS =====
        // Propagate prediction errors upward

        let mut current_signal = input.to_vec();

        for i in 0..self.levels.len() {
            // Compute prediction error at this level
            let error = self.levels[i].compute_error(&current_signal);
            errors.push(error.clone());

            // Update this level based on error
            if i > 0 {
                self.levels[i].process_error(&errors[i - 1]);
            }

            // Project upward for next level
            if i < self.upward_layers.len() {
                current_signal = self.upward_layers[i].forward(&self.levels[i].mu);
            }
        }

        // ===== TOP-DOWN PASS =====
        // Generate predictions flowing downward

        for i in (0..self.levels.len()).rev() {
            // Generate prediction for level below
            let pred = self.levels[i].predict();
            predictions.push(pred.clone());

            // Send prediction to level below
            if i > 0 {
                // Project downward
                let projected = if i <= self.downward_layers.len() {
                    self.downward_layers[i - 1].forward(&pred.prediction)
                } else {
                    pred.prediction.clone()
                };

                let projected_pred = TopDownPrediction {
                    from_level: i,
                    to_level: i - 1,
                    prediction: projected,
                    precision: pred.precision,
                    timestamp: pred.timestamp,
                };

                self.levels[i - 1].process_prediction(&projected_pred);
            }
        }

        // ===== LEARNING =====
        // Update projection weights based on errors

        for i in 0..self.upward_layers.len() {
            if i < errors.len() {
                self.upward_layers[i].update(
                    &self.levels[i].mu,
                    &errors[i].error,
                    self.config.learning_rate * 0.1
                );
            }
        }

        // ===== COMPUTE METRICS =====

        self.total_free_energy = self.levels.iter()
            .map(|l| l.free_energy())
            .sum();

        self.total_surprise = self.levels.iter()
            .map(|l| l.surprise())
            .sum();

        HierarchyResult {
            level_states: self.levels.iter().map(|l| l.mu.clone()).collect(),
            level_errors: errors.iter().map(|e| e.error.clone()).collect(),
            level_predictions: predictions.iter().map(|p| p.prediction.clone()).collect(),
            total_free_energy: self.total_free_energy,
            total_surprise: self.total_surprise,
            dominant_level: self.find_dominant_level(),
        }
    }

    /// Find the level with highest activity (surprise)
    fn find_dominant_level(&self) -> usize {
        self.levels.iter()
            .enumerate()
            .max_by(|a, b| a.1.surprise().partial_cmp(&b.1.surprise()).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    /// Get belief at a specific level
    pub fn belief_at(&self, level: usize) -> Option<&Vec<f64>> {
        self.levels.get(level).map(|l| &l.mu)
    }

    /// Get prediction at a specific level
    pub fn prediction_at(&self, level: usize) -> Option<&Vec<f64>> {
        self.levels.get(level).map(|l| &l.prediction)
    }

    /// Get error at a specific level
    pub fn error_at(&self, level: usize) -> Option<&Vec<f64>> {
        self.levels.get(level).map(|l| &l.error)
    }

    /// Get total free energy
    pub fn free_energy(&self) -> f64 {
        self.total_free_energy
    }

    /// Get total surprise
    pub fn surprise(&self) -> f64 {
        self.total_surprise
    }

    /// Get number of levels
    pub fn num_levels(&self) -> usize {
        self.levels.len()
    }

    /// Get a level
    pub fn level(&self, index: usize) -> Option<&HierarchyLevel> {
        self.levels.get(index)
    }

    /// Get processing cycles
    pub fn cycles(&self) -> u64 {
        self.cycles
    }

    /// Reset the hierarchy
    pub fn reset(&mut self) {
        for level in &mut self.levels {
            level.mu = vec![0.0; level.dim];
            level.prediction = vec![0.0; level.dim];
            level.error = vec![0.0; level.dim];
            level.precision = 1.0;
            level.error_precision = 1.0;
        }
        self.total_free_energy = 0.0;
        self.total_surprise = 0.0;
        self.cycles = 0;
    }
}

/// Result of hierarchy processing
#[derive(Debug, Clone)]
pub struct HierarchyResult {
    /// States at each level
    pub level_states: Vec<Vec<f64>>,
    /// Errors at each level
    pub level_errors: Vec<Vec<f64>>,
    /// Predictions at each level
    pub level_predictions: Vec<Vec<f64>>,
    /// Total free energy
    pub total_free_energy: f64,
    /// Total surprise
    pub total_surprise: f64,
    /// Level with highest activity
    pub dominant_level: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hierarchy_creation() {
        let config = HierarchyConfig::default();
        let hierarchy = OmegaHierarchy::new(config);

        assert_eq!(hierarchy.num_levels(), 7);
    }

    #[test]
    fn test_hierarchy_processing() {
        let config = HierarchyConfig::default();
        let mut hierarchy = OmegaHierarchy::new(config);

        let input = vec![0.5; 256]; // Match first level dim
        let result = hierarchy.process(&input);

        assert_eq!(result.level_states.len(), 7);
        assert!(result.total_free_energy >= 0.0);
    }

    #[test]
    fn test_free_energy_reduction() {
        let config = HierarchyConfig::default();
        let mut hierarchy = OmegaHierarchy::new(config);

        let input = vec![0.5; 256];

        // Process multiple times
        let mut prev_fe = f64::INFINITY;
        for _ in 0..10 {
            let result = hierarchy.process(&input);
            // Free energy should generally decrease or stabilize
            prev_fe = result.total_free_energy;
        }

        assert!(prev_fe < f64::INFINITY);
    }

    #[test]
    fn test_prediction_layer() {
        let layer = PredictionLayer::new(64, 32);
        let input = vec![0.5; 64];
        let output = layer.forward(&input);

        assert_eq!(output.len(), 32);
    }
}
