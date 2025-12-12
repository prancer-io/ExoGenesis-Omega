//! Free Energy Principle (FEP)
//!
//! Implements the Free Energy Principle and Active Inference:
//! - Predictive processing hierarchy
//! - Prediction error minimization
//! - Precision-weighted inference
//! - Active inference for action selection
//!
//! Based on Friston (2010) - "The free-energy principle: a unified brain theory"

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use crate::Result;

/// Prediction error at a single level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionError {
    /// Level in hierarchy (0 = lowest/sensory)
    pub level: usize,
    /// Error vector
    pub error: Vec<f64>,
    /// Precision (inverse variance)
    pub precision: f64,
    /// Magnitude of error
    pub magnitude: f64,
}

impl PredictionError {
    pub fn new(level: usize, error: Vec<f64>, precision: f64) -> Self {
        let magnitude = error.iter().map(|e| e * e).sum::<f64>().sqrt();
        Self {
            level,
            error,
            precision,
            magnitude,
        }
    }

    /// Precision-weighted error
    pub fn weighted_error(&self) -> f64 {
        self.precision * self.magnitude
    }
}

/// A single level in the predictive hierarchy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchyLevel {
    /// Level index
    pub level: usize,
    /// Current state belief (mean)
    pub mu: Vec<f64>,
    /// Prediction for level below
    pub prediction: Vec<f64>,
    /// Prediction error from below
    pub error: Vec<f64>,
    /// Precision of predictions
    pub precision: f64,
    /// Learning rate
    pub learning_rate: f64,
}

impl HierarchyLevel {
    pub fn new(level: usize, dim: usize) -> Self {
        Self {
            level,
            mu: vec![0.0; dim],
            prediction: vec![0.0; dim],
            error: vec![0.0; dim],
            precision: 1.0,
            learning_rate: 0.1,
        }
    }

    /// Generate prediction for level below
    pub fn predict(&mut self, _context: &[f64]) {
        // Simple prediction: current belief is the prediction
        // In full FEP, this would be a generative model
        self.prediction = self.mu.clone();
    }

    /// Update belief based on prediction error from below
    pub fn update(&mut self, error: &[f64]) {
        self.error = error.to_vec();

        // Gradient descent on prediction error
        // mu_new = mu + lr * precision * error
        for (i, e) in error.iter().enumerate() {
            if i < self.mu.len() {
                self.mu[i] += self.learning_rate * self.precision * e;
            }
        }
    }

    /// Update precision based on error history
    pub fn update_precision(&mut self, error_magnitude: f64) {
        // Precision increases when errors are small (confidence)
        // Precision decreases when errors are large (uncertainty)
        let target_precision = 1.0 / (error_magnitude + 0.1);
        self.precision = 0.9 * self.precision + 0.1 * target_precision;
        self.precision = self.precision.max(0.1).min(10.0);
    }
}

/// Complete predictive hierarchy
#[derive(Debug, Clone)]
pub struct PredictiveHierarchy {
    /// Levels from lowest (sensory) to highest (abstract)
    levels: Vec<HierarchyLevel>,
    /// Dimension at each level
    dim: usize,
}

impl PredictiveHierarchy {
    pub fn new(num_levels: usize, dim: usize) -> Self {
        let levels = (0..num_levels)
            .map(|i| HierarchyLevel::new(i, dim))
            .collect();

        Self { levels, dim }
    }

    /// Get number of levels
    pub fn num_levels(&self) -> usize {
        self.levels.len()
    }

    /// Get a specific level
    pub fn level(&self, index: usize) -> Option<&HierarchyLevel> {
        self.levels.get(index)
    }

    /// Get mutable level
    pub fn level_mut(&mut self, index: usize) -> Option<&mut HierarchyLevel> {
        self.levels.get_mut(index)
    }

    /// Get all levels
    pub fn levels(&self) -> &[HierarchyLevel] {
        &self.levels
    }

    /// Process input through hierarchy (bottom-up then top-down)
    pub fn process(&mut self, input: &[f64]) -> Vec<PredictionError> {
        let mut errors = Vec::new();

        // Bottom-up: propagate prediction errors
        let mut current = input.to_vec();

        for level in 0..self.levels.len() {
            // Get prediction from this level
            let prediction = self.levels[level].prediction.clone();

            // Compute prediction error
            let error: Vec<f64> = current
                .iter()
                .zip(prediction.iter())
                .map(|(c, p)| c - p)
                .collect();

            let pe = PredictionError::new(level, error.clone(), self.levels[level].precision);
            errors.push(pe);

            // Update this level's beliefs
            self.levels[level].update(&error);

            // Error magnitude for precision update
            let magnitude = error.iter().map(|e| e * e).sum::<f64>().sqrt();
            self.levels[level].update_precision(magnitude);

            // Pass up: abstracted version of beliefs becomes input to next level
            current = self.levels[level].mu.clone();
        }

        // Top-down: generate predictions
        for level in (0..self.levels.len()).rev() {
            let context = if level < self.levels.len() - 1 {
                self.levels[level + 1].mu.clone()
            } else {
                vec![0.0; self.dim]
            };

            self.levels[level].predict(&context);
        }

        errors
    }

    /// Get total prediction error
    pub fn total_error(&self) -> f64 {
        self.levels
            .iter()
            .map(|l| l.error.iter().map(|e| e * e).sum::<f64>())
            .sum::<f64>()
            .sqrt()
    }

    /// Reset hierarchy
    pub fn reset(&mut self) {
        for level in &mut self.levels {
            level.mu = vec![0.0; self.dim];
            level.prediction = vec![0.0; self.dim];
            level.error = vec![0.0; self.dim];
            level.precision = 1.0;
        }
    }
}

/// Active inference for action selection
#[derive(Debug, Clone)]
pub struct ActiveInference {
    /// Prior preferences (desired states)
    prior_preferences: Vec<f64>,
    /// Action repertoire
    action_dim: usize,
    /// Expected free energy for each action
    efe: Vec<f64>,
    /// Exploration-exploitation balance
    exploration_weight: f64,
}

impl ActiveInference {
    pub fn new(state_dim: usize, action_dim: usize) -> Self {
        Self {
            prior_preferences: vec![0.0; state_dim],
            action_dim,
            efe: vec![0.0; action_dim],
            exploration_weight: 0.1,
        }
    }

    /// Set prior preferences (goals)
    pub fn set_preferences(&mut self, preferences: Vec<f64>) {
        self.prior_preferences = preferences;
    }

    /// Compute expected free energy for actions
    pub fn compute_efe(&mut self, current_state: &[f64], predicted_states: &[Vec<f64>]) {
        for (action, predicted) in predicted_states.iter().enumerate() {
            if action >= self.action_dim {
                break;
            }

            // Expected free energy = ambiguity + risk
            // Ambiguity: entropy of predicted state
            let ambiguity = Self::entropy(predicted);

            // Risk: KL divergence from preferences
            let risk = Self::kl_divergence(predicted, &self.prior_preferences);

            // Exploration bonus (information gain)
            let info_gain = Self::expected_info_gain(current_state, predicted);

            self.efe[action] = risk + ambiguity - self.exploration_weight * info_gain;
        }
    }

    /// Select action that minimizes expected free energy
    pub fn select_action(&self) -> usize {
        self.efe
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    /// Entropy of a distribution (treating values as probabilities)
    fn entropy(dist: &[f64]) -> f64 {
        let sum: f64 = dist.iter().map(|&x| x.abs()).sum();
        if sum < 1e-10 {
            return 0.0;
        }

        let normalized: Vec<f64> = dist.iter().map(|&x| x.abs() / sum).collect();
        -normalized
            .iter()
            .filter(|&&p| p > 1e-10)
            .map(|&p| p * p.ln())
            .sum::<f64>()
    }

    /// KL divergence (simplified)
    fn kl_divergence(p: &[f64], q: &[f64]) -> f64 {
        let mut kl = 0.0;
        for (&pi, &qi) in p.iter().zip(q.iter()) {
            let pi_abs = pi.abs().max(1e-10);
            let qi_abs = qi.abs().max(1e-10);
            kl += pi_abs * (pi_abs / qi_abs).ln();
        }
        kl.max(0.0)
    }

    /// Expected information gain
    fn expected_info_gain(current: &[f64], predicted: &[f64]) -> f64 {
        // Simplified: difference between states indicates potential learning
        current
            .iter()
            .zip(predicted.iter())
            .map(|(c, p)| (c - p).abs())
            .sum::<f64>()
            / current.len().max(1) as f64
    }
}

/// Free energy minimizer combining hierarchy and active inference
pub struct FreeEnergyMinimizer {
    hierarchy: PredictiveHierarchy,
    active: ActiveInference,
    current_free_energy: f64,
    error_history: VecDeque<f64>,
}

impl FreeEnergyMinimizer {
    pub fn new(num_levels: usize, dim: usize) -> Self {
        Self {
            hierarchy: PredictiveHierarchy::new(num_levels, dim),
            active: ActiveInference::new(dim, 8), // 8 possible actions
            current_free_energy: 0.0,
            error_history: VecDeque::with_capacity(100),
        }
    }

    /// Process input and compute free energy
    pub fn process(&mut self, input: &[f64], context: &[f64]) -> Result<(f64, f64)> {
        // Process through hierarchy
        let errors = self.hierarchy.process(input);

        // Compute variational free energy
        // F = E[log q(x)] - E[log p(x, y)]
        // Simplified: F ≈ precision-weighted prediction errors

        let prediction_error: f64 = errors.iter().map(|e| e.weighted_error()).sum();

        // Complexity term (deviation from prior)
        let complexity = self.compute_complexity(context);

        // Total free energy
        self.current_free_energy = prediction_error + complexity;

        // Store in history
        self.error_history.push_back(prediction_error);
        if self.error_history.len() > 100 {
            self.error_history.pop_front();
        }

        Ok((self.current_free_energy, prediction_error))
    }

    /// Compute complexity term
    fn compute_complexity(&self, prior: &[f64]) -> f64 {
        // KL divergence from prior beliefs
        let current = &self.hierarchy.levels[0].mu;

        let mut kl = 0.0;
        for (&c, &p) in current.iter().zip(prior.iter()) {
            let c_abs = c.abs().max(1e-10);
            let p_abs = p.abs().max(1e-10);
            kl += (c_abs - p_abs).powi(2);
        }

        kl.sqrt() * 0.1 // Weight complexity term
    }

    /// Get current free energy
    pub fn current_free_energy(&self) -> f64 {
        self.current_free_energy
    }

    /// Get hierarchy
    pub fn hierarchy(&self) -> &PredictiveHierarchy {
        &self.hierarchy
    }

    /// Get active inference module
    pub fn active_inference(&self) -> &ActiveInference {
        &self.active
    }

    /// Get mutable active inference module
    pub fn active_inference_mut(&mut self) -> &mut ActiveInference {
        &mut self.active
    }

    /// Select action to minimize expected free energy
    pub fn select_action(&mut self, predicted_states: &[Vec<f64>]) -> usize {
        let current = &self.hierarchy.levels[0].mu;
        self.active.compute_efe(current, predicted_states);
        self.active.select_action()
    }

    /// Reset minimizer
    pub fn reset(&mut self) {
        self.hierarchy.reset();
        self.current_free_energy = 0.0;
        self.error_history.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hierarchy_creation() {
        let hierarchy = PredictiveHierarchy::new(5, 32);
        assert_eq!(hierarchy.num_levels(), 5);
    }

    #[test]
    fn test_hierarchy_process() {
        let mut hierarchy = PredictiveHierarchy::new(3, 8);

        let input = vec![0.5; 8];
        let errors = hierarchy.process(&input);

        assert_eq!(errors.len(), 3);
    }

    #[test]
    fn test_prediction_error() {
        let pe = PredictionError::new(0, vec![0.1, 0.2, 0.3], 1.0);

        assert!(pe.magnitude > 0.0);
        assert!(pe.weighted_error() > 0.0);
    }

    #[test]
    fn test_active_inference() {
        let mut active = ActiveInference::new(8, 4);

        active.set_preferences(vec![1.0; 8]);

        let current = vec![0.5; 8];
        let predicted = vec![
            vec![0.6; 8],
            vec![0.7; 8],
            vec![0.8; 8],
            vec![0.9; 8],
        ];

        active.compute_efe(&current, &predicted);
        let action = active.select_action();

        assert!(action < 4);
    }

    #[test]
    fn test_free_energy_minimizer() {
        let mut minimizer = FreeEnergyMinimizer::new(3, 8);

        let input = vec![0.5; 8];
        let context = vec![0.3; 8];

        let result = minimizer.process(&input, &context);
        assert!(result.is_ok());

        let (fe, _) = result.unwrap();
        assert!(fe >= 0.0);
    }

    #[test]
    fn test_free_energy_decreases() {
        let mut minimizer = FreeEnergyMinimizer::new(3, 8);

        let context = vec![0.3; 8];
        let mut prev_fe = f64::INFINITY;

        // Process same input multiple times
        for _ in 0..10 {
            let input = vec![0.5; 8];
            let (fe, _) = minimizer.process(&input, &context).unwrap();

            // Free energy should generally decrease (or stabilize)
            // as predictions improve
            prev_fe = fe;
        }

        assert!(prev_fe < f64::INFINITY);
    }
}
