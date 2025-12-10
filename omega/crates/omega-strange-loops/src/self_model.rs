//! Self Model
//!
//! Internal representation of self:
//! - Current state
//! - Predicted states
//! - Historical states
//! - Self-attributes

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Current state of self
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfState {
    /// State vector
    pub state: Vec<f64>,
    /// Confidence in state estimate
    pub confidence: f64,
    /// Predicted next state
    pub predicted_next: Vec<f64>,
    /// Prediction error history
    pub prediction_error: f64,
    /// Timestamp
    pub timestamp: u64,
}

impl SelfState {
    pub fn new(dim: usize) -> Self {
        Self {
            state: vec![0.0; dim],
            confidence: 0.5,
            predicted_next: vec![0.0; dim],
            prediction_error: 0.0,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }

    /// Update state
    pub fn update(&mut self, new_state: &[f64]) {
        // Compute prediction error
        self.prediction_error = self
            .predicted_next
            .iter()
            .zip(new_state.iter())
            .map(|(p, n)| (p - n).powi(2))
            .sum::<f64>()
            .sqrt()
            / new_state.len() as f64;

        // Update confidence based on prediction error
        self.confidence = 0.9 * self.confidence + 0.1 * (1.0 - self.prediction_error.min(1.0));

        // Update state
        for (i, &n) in new_state.iter().enumerate() {
            if i < self.state.len() {
                self.state[i] = n;
            }
        }

        // Predict next (simple: slight change)
        for i in 0..self.predicted_next.len() {
            self.predicted_next[i] = self.state.get(i).copied().unwrap_or(0.0) * 1.01;
        }

        self.timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
    }
}

/// Update to self model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfModelUpdate {
    /// Type of update
    pub update_type: SelfUpdateType,
    /// Old values
    pub old_values: Vec<f64>,
    /// New values
    pub new_values: Vec<f64>,
    /// Confidence change
    pub confidence_delta: f64,
}

/// Types of self-model updates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelfUpdateType {
    /// Observation-based update
    Observation,
    /// Prediction correction
    Prediction,
    /// Meta-cognitive adjustment
    MetaCognitive,
    /// External feedback
    Feedback,
}

/// Self model with multiple levels
pub struct SelfModel {
    /// Number of meta-levels
    num_levels: usize,
    /// Current state at each level
    states: Vec<SelfState>,
    /// Attributes about self
    attributes: Vec<SelfAttribute>,
    /// History of states
    history: VecDeque<SelfState>,
    /// Maximum history length
    max_history: usize,
    /// Self-reference strength
    self_ref_strength: f64,
    /// Update count
    update_count: usize,
}

impl SelfModel {
    /// Create new self model
    pub fn new(num_levels: usize) -> Self {
        let dim = 32;
        let states = (0..num_levels).map(|_| SelfState::new(dim)).collect();

        Self {
            num_levels,
            states,
            attributes: Vec::new(),
            history: VecDeque::with_capacity(100),
            max_history: 100,
            self_ref_strength: 0.0,
            update_count: 0,
        }
    }

    /// Observe input and update self-model
    pub fn observe(&mut self, input: &[f64]) {
        // Update base level (level 0)
        if !self.states.is_empty() {
            self.states[0].update(input);

            // Propagate to higher levels (abstraction)
            for level in 1..self.num_levels {
                let prev_state = self.states[level - 1].state.clone();
                let abstracted: Vec<f64> = prev_state
                    .chunks(2)
                    .map(|chunk| chunk.iter().sum::<f64>() / chunk.len() as f64)
                    .collect();

                self.states[level].update(&abstracted);
            }
        }
    }

    /// Update self-model
    pub fn update(&mut self, output: &[f64]) {
        // Store current state in history
        if !self.states.is_empty() {
            let current = self.states[0].clone();
            self.history.push_back(current);
            if self.history.len() > self.max_history {
                self.history.pop_front();
            }
        }

        // Update base level
        if !self.states.is_empty() {
            let old = self.states[0].state.clone();
            self.states[0].update(output);

            // Compute self-reference strength
            let similarity = self.cosine_similarity(&old, output);
            self.self_ref_strength = 0.9 * self.self_ref_strength + 0.1 * similarity;
        }

        self.update_count += 1;
    }

    /// Get current state at level
    pub fn state_at_level(&self, level: usize) -> Option<&SelfState> {
        self.states.get(level)
    }

    /// Get current state (base level)
    pub fn current_state(&self) -> SelfState {
        self.states.first().cloned().unwrap_or_else(|| SelfState::new(32))
    }

    /// Get self-reference strength
    pub fn self_reference_strength(&self) -> f64 {
        self.self_ref_strength
    }

    /// Get overall confidence
    pub fn confidence(&self) -> f64 {
        if self.states.is_empty() {
            return 0.0;
        }
        self.states.iter().map(|s| s.confidence).sum::<f64>() / self.states.len() as f64
    }

    /// Get prediction error at level
    pub fn prediction_error(&self, level: usize) -> f64 {
        self.states
            .get(level)
            .map(|s| s.prediction_error)
            .unwrap_or(1.0)
    }

    /// Add self-attribute
    pub fn add_attribute(&mut self, attr: SelfAttribute) {
        self.attributes.push(attr);
    }

    /// Get attributes
    pub fn attributes(&self) -> &[SelfAttribute] {
        &self.attributes
    }

    /// Get history length
    pub fn history_len(&self) -> usize {
        self.history.len()
    }

    /// Get update count
    pub fn update_count(&self) -> usize {
        self.update_count
    }

    fn cosine_similarity(&self, a: &[f64], b: &[f64]) -> f64 {
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

    /// Reset self model
    pub fn reset(&mut self) {
        for state in &mut self.states {
            *state = SelfState::new(state.state.len());
        }
        self.history.clear();
        self.self_ref_strength = 0.0;
        self.update_count = 0;
    }
}

impl Default for SelfModel {
    fn default() -> Self {
        Self::new(5)
    }
}

/// An attribute about self
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfAttribute {
    /// Attribute name
    pub name: String,
    /// Attribute value
    pub value: f64,
    /// Confidence in this attribute
    pub confidence: f64,
    /// Source of attribute
    pub source: AttributeSource,
}

/// Source of self-attribute
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttributeSource {
    /// Observed from behavior
    Observed,
    /// Inferred from patterns
    Inferred,
    /// External feedback
    External,
    /// Self-reflection
    Reflected,
}

impl SelfAttribute {
    pub fn new(name: String, value: f64, source: AttributeSource) -> Self {
        Self {
            name,
            value,
            confidence: 0.5,
            source,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_state() {
        let mut state = SelfState::new(10);

        let input = vec![0.5; 10];
        state.update(&input);

        assert_eq!(state.state.len(), 10);
        assert!(state.confidence > 0.0);
    }

    #[test]
    fn test_self_model() {
        let mut model = SelfModel::new(3);

        let input = vec![0.5; 32];
        model.observe(&input);

        assert!(model.confidence() > 0.0);
    }

    #[test]
    fn test_self_reference() {
        let mut model = SelfModel::new(3);

        // Feed similar inputs multiple times
        let input = vec![0.5; 32];
        for _ in 0..10 {
            model.observe(&input);
            model.update(&input);
        }

        // Should build up self-reference
        assert!(model.self_reference_strength() > 0.5);
    }

    #[test]
    fn test_history() {
        let mut model = SelfModel::new(2);

        for i in 0..10 {
            let input: Vec<f64> = (0..32).map(|j| (i + j) as f64 / 100.0).collect();
            model.observe(&input);
            model.update(&input);
        }

        assert!(model.history_len() > 0);
    }

    #[test]
    fn test_attributes() {
        let mut model = SelfModel::new(2);

        model.add_attribute(SelfAttribute::new(
            "curiosity".to_string(),
            0.8,
            AttributeSource::Observed,
        ));

        assert_eq!(model.attributes().len(), 1);
    }
}
