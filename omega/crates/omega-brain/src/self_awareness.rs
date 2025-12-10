//! Self-Awareness System Integration
//!
//! Integrates omega-strange-loops for:
//! - Self-model maintenance
//! - Meta-cognition
//! - Strange loop detection
//! - Self-reference

use crate::{BrainConfig, BrainError, Result};
use omega_strange_loops::{
    MetaCognition, SelfModel, StrangeLoopEngine, ThoughtAboutThought,
};

/// Self-awareness system wrapping strange loop components
pub struct SelfAwarenessSystem {
    /// Strange loop engine
    engine: StrangeLoopEngine,
    /// Self-model
    self_model: SelfModel,
    /// Meta-cognition system
    meta_cognition: MetaCognition,
    /// Strange loop count
    loop_count: usize,
    /// Self-reference strength
    self_ref_strength: f64,
    /// Dimension
    dim: usize,
}

impl SelfAwarenessSystem {
    /// Create new self-awareness system
    pub fn new(config: &BrainConfig) -> Self {
        let engine = StrangeLoopEngine::with_config(omega_strange_loops::StrangeLoopConfig {
            max_depth: config.max_recursion,
            meta_levels: config.meta_levels,
            update_rate: config.self_update_rate,
            mirror_depth: config.mirror_depth,
            detect_paradoxes: true,
        });

        let self_model = SelfModel::new(config.meta_levels);
        let meta_cognition = MetaCognition::new(config.meta_levels);

        Self {
            engine,
            self_model,
            meta_cognition,
            loop_count: 0,
            self_ref_strength: 0.0,
            dim: config.attention_dim,
        }
    }

    /// Reflect on input (self-referential processing)
    pub fn reflect(&mut self, input: &[f64]) -> Result<Vec<f64>> {
        // Normalize input
        let normalized = self.normalize_input(input);

        // Process through strange loop engine
        let output = self
            .engine
            .process(&normalized)
            .map_err(|e| BrainError::SelfModelError(e.to_string()))?;

        // Update self-model
        self.self_model.observe(&normalized);
        self.self_model.update(&output);

        // Update metrics
        self.self_ref_strength = self.self_model.self_reference_strength();
        self.loop_count = self.engine.loop_count();

        Ok(output)
    }

    /// Think about a thought (meta-cognition)
    pub fn think_about(&mut self, thought: &[f64]) -> Result<Vec<f64>> {
        let normalized = self.normalize_input(thought);

        // Meta-cognitive processing
        let tat = self
            .meta_cognition
            .think_about(&normalized)
            .map_err(|e| BrainError::SelfModelError(e.to_string()))?;

        // Process through engine for self-reference
        let output = self
            .engine
            .process(&tat.meta)
            .map_err(|e| BrainError::SelfModelError(e.to_string()))?;

        Ok(output)
    }

    /// Get current self-state
    pub fn current_self_state(&self) -> Vec<f64> {
        self.self_model.current_state().state
    }

    /// Get self-reference strength
    pub fn self_reference_strength(&self) -> f64 {
        self.self_ref_strength
    }

    /// Get loop count
    pub fn loop_count(&self) -> usize {
        self.loop_count
    }

    /// Get self-model confidence
    pub fn confidence(&self) -> f64 {
        self.self_model.confidence()
    }

    /// Get cognitive quality
    pub fn cognitive_quality(&self) -> f64 {
        self.meta_cognition.cognitive_quality()
    }

    /// Normalize input to expected dimension
    fn normalize_input(&self, input: &[f64]) -> Vec<f64> {
        let mut result = vec![0.0; self.dim];
        for (i, &v) in input.iter().enumerate() {
            if i < self.dim {
                result[i] = v;
            }
        }
        result
    }

    /// Check for self-reference paradox
    pub fn has_paradox(&self) -> bool {
        // Detect if self-reference creates logical issues
        self.self_ref_strength > 0.95 && self.loop_count > 10
    }

    /// Get meta-level count
    pub fn meta_level_count(&self) -> usize {
        self.meta_cognition.current_level()
    }

    /// Reset the self-awareness system
    pub fn reset(&mut self) {
        self.engine.reset();
        self.self_model.reset();
        self.meta_cognition.reset();
        self.loop_count = 0;
        self.self_ref_strength = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_awareness_creation() {
        let config = BrainConfig::default();
        let system = SelfAwarenessSystem::new(&config);

        assert_eq!(system.loop_count(), 0);
    }

    #[test]
    fn test_reflect() {
        let config = BrainConfig::minimal();
        let mut system = SelfAwarenessSystem::new(&config);

        let input = vec![0.5; config.attention_dim];
        let output = system.reflect(&input).unwrap();

        assert_eq!(output.len(), config.attention_dim);
    }

    #[test]
    fn test_think_about() {
        let config = BrainConfig::minimal();
        let mut system = SelfAwarenessSystem::new(&config);

        let thought = vec![0.5; config.attention_dim];
        let output = system.think_about(&thought).unwrap();

        assert_eq!(output.len(), config.attention_dim);
    }

    #[test]
    fn test_self_reference_builds() {
        let config = BrainConfig::minimal();
        let mut system = SelfAwarenessSystem::new(&config);

        // Repeatedly process similar input
        let input = vec![0.5; config.attention_dim];
        for _ in 0..10 {
            system.reflect(&input).unwrap();
        }

        // Should build self-reference
        assert!(system.self_reference_strength() > 0.0);
    }

    #[test]
    fn test_current_self_state() {
        let config = BrainConfig::minimal();
        let mut system = SelfAwarenessSystem::new(&config);

        let input = vec![0.5; config.attention_dim];
        system.reflect(&input).unwrap();

        let state = system.current_self_state();
        assert!(!state.is_empty());
    }
}
