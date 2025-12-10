//! Attention System Integration
//!
//! Integrates omega-attention mechanisms into the brain.

use crate::{BrainConfig, BrainError, Result};
use omega_attention::{AttentionController, AttentionMechanism, AttentionType, WorkingMemory};

/// Attention system wrapping multiple attention mechanisms
pub struct AttentionSystem {
    /// Main attention controller
    controller: AttentionController,
    /// Working memory
    working_memory: WorkingMemory,
    /// Current focus
    current_focus: Vec<f64>,
    /// Attention history
    history: Vec<Vec<f64>>,
    /// Maximum history
    max_history: usize,
    /// Dimension
    dim: usize,
}

impl AttentionSystem {
    /// Create new attention system
    pub fn new(config: &BrainConfig) -> Self {
        let controller = AttentionController::new(
            config.attention_heads,
            config.attention_dim,
            config.top_down_strength,
            config.bottom_up_strength,
        );

        let working_memory = WorkingMemory::new(config.workspace_capacity, config.attention_dim);

        Self {
            controller,
            working_memory,
            current_focus: vec![0.0; config.attention_dim],
            history: Vec::new(),
            max_history: 100,
            dim: config.attention_dim,
        }
    }

    /// Attend to input
    pub fn attend(&mut self, input: &[f64]) -> Result<Vec<f64>> {
        // Pad/truncate input to expected dimension
        let normalized = self.normalize_input(input);

        // Apply attention controller
        let attended = self.controller.attend(&normalized);

        // Gate through working memory
        let gated = self.working_memory.gate(&attended);

        // Update current focus
        self.update_focus(&gated);

        // Store in history
        self.history.push(gated.clone());
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }

        Ok(gated)
    }

    /// Focus attention on specific target
    pub fn focus_on(&mut self, target: &[f64]) -> Result<()> {
        let normalized = self.normalize_input(target);
        self.controller.set_top_down_bias(&normalized);
        self.current_focus = normalized;
        Ok(())
    }

    /// Get current focus
    pub fn current_focus(&self) -> Vec<f64> {
        self.current_focus.clone()
    }

    /// Update focus based on attended content
    fn update_focus(&mut self, attended: &[f64]) {
        for (i, &v) in attended.iter().enumerate() {
            if i < self.current_focus.len() {
                // Exponential moving average
                self.current_focus[i] = 0.8 * self.current_focus[i] + 0.2 * v;
            }
        }
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

    /// Add item to working memory
    pub fn remember(&mut self, item: &[f64]) {
        let normalized = self.normalize_input(item);
        self.working_memory.store(&normalized);
    }

    /// Get working memory contents
    pub fn working_memory_contents(&self) -> Vec<Vec<f64>> {
        self.working_memory.contents()
    }

    /// Clear working memory
    pub fn clear_working_memory(&mut self) {
        self.working_memory.clear();
    }

    /// Get attention strength
    pub fn attention_strength(&self) -> f64 {
        let max = self
            .current_focus
            .iter()
            .map(|x| x.abs())
            .fold(0.0, f64::max);
        let mean = self.current_focus.iter().map(|x| x.abs()).sum::<f64>()
            / self.current_focus.len().max(1) as f64;

        if mean > 0.0 {
            (max / mean).min(2.0) / 2.0
        } else {
            0.0
        }
    }

    /// Switch attention mechanism
    pub fn switch_mechanism(&mut self, mechanism: AttentionType) {
        self.controller.set_mechanism(mechanism);
    }

    /// Get recent attention history
    pub fn recent_history(&self, n: usize) -> Vec<&Vec<f64>> {
        self.history.iter().rev().take(n).collect()
    }

    /// Reset the attention system
    pub fn reset(&mut self) {
        self.controller = AttentionController::new(
            self.controller.num_heads(),
            self.dim,
            self.controller.top_down_strength(),
            self.controller.bottom_up_strength(),
        );
        self.working_memory.clear();
        self.current_focus = vec![0.0; self.dim];
        self.history.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attention_system_creation() {
        let config = BrainConfig::default();
        let system = AttentionSystem::new(&config);

        assert_eq!(system.current_focus().len(), config.attention_dim);
    }

    #[test]
    fn test_attend() {
        let config = BrainConfig::minimal();
        let mut system = AttentionSystem::new(&config);

        let input = vec![0.5; config.attention_dim];
        let attended = system.attend(&input).unwrap();

        assert_eq!(attended.len(), config.attention_dim);
    }

    #[test]
    fn test_focus_on() {
        let config = BrainConfig::minimal();
        let mut system = AttentionSystem::new(&config);

        let target = vec![1.0; config.attention_dim];
        system.focus_on(&target).unwrap();

        let focus = system.current_focus();
        assert_eq!(focus.len(), config.attention_dim);
    }

    #[test]
    fn test_working_memory() {
        let config = BrainConfig::minimal();
        let mut system = AttentionSystem::new(&config);

        let item = vec![0.5; config.attention_dim];
        system.remember(&item);

        let contents = system.working_memory_contents();
        assert!(!contents.is_empty());
    }
}
