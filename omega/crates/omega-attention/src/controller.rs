//! Attention Controller
//!
//! Implements brain-like attention control with:
//! - Top-down (goal-driven) attention
//! - Bottom-up (salience-driven) attention
//! - Priority map combining both signals

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use crate::mechanisms::{
    AttentionMechanism, AttentionOutput, AttentionType,
    ScaledDotProductAttention, LinearAttention, SparseAttention,
};
use crate::Result;

/// Configuration for attention controller
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionConfig {
    /// Dimension of attention vectors
    pub dim: usize,
    /// Weight for top-down attention (0.0 to 1.0)
    pub top_down_weight: f64,
    /// Weight for bottom-up attention (0.0 to 1.0)
    pub bottom_up_weight: f64,
    /// Number of attention heads
    pub num_heads: usize,
    /// Default attention mechanism type
    pub default_mechanism: AttentionType,
    /// Attention focus decay rate
    pub focus_decay: f64,
}

impl Default for AttentionConfig {
    fn default() -> Self {
        Self {
            dim: 64,
            top_down_weight: 0.6,
            bottom_up_weight: 0.4,
            num_heads: 4,
            default_mechanism: AttentionType::ScaledDotProduct,
            focus_decay: 0.1,
        }
    }
}

/// Priority map combining top-down and bottom-up signals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityMap {
    /// Priority values
    pub priorities: Vec<f64>,
    /// Top-down contribution
    pub top_down: Vec<f64>,
    /// Bottom-up contribution
    pub bottom_up: Vec<f64>,
    /// Combined priority
    pub combined: Vec<f64>,
}

impl PriorityMap {
    pub fn new(size: usize) -> Self {
        Self {
            priorities: vec![0.0; size],
            top_down: vec![0.0; size],
            bottom_up: vec![0.0; size],
            combined: vec![0.0; size],
        }
    }

    /// Get highest priority index
    pub fn argmax(&self) -> usize {
        self.combined
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    /// Get top-k priority indices
    pub fn top_k(&self, k: usize) -> Vec<usize> {
        let mut indexed: Vec<_> = self.combined.iter().enumerate().collect();
        indexed.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
        indexed.into_iter().take(k).map(|(i, _)| i).collect()
    }

    /// Normalize priorities
    pub fn normalize(&mut self) {
        let sum: f64 = self.combined.iter().sum();
        if sum > 0.0 {
            for p in &mut self.combined {
                *p /= sum;
            }
        }
    }
}

/// The attention controller
pub struct AttentionController {
    config: AttentionConfig,
    /// Current focus target (for top-down)
    focus: Option<Vec<f64>>,
    /// Focus history
    focus_history: VecDeque<Vec<f64>>,
    /// Attention mechanism
    mechanism: Box<dyn AttentionMechanism>,
    /// Recent attention outputs for inhibition of return
    recent_attended: VecDeque<usize>,
}

impl AttentionController {
    pub fn new(config: AttentionConfig) -> Self {
        let mechanism: Box<dyn AttentionMechanism> = match config.default_mechanism {
            AttentionType::Linear => Box::new(LinearAttention::new(config.dim)),
            AttentionType::Sparse => Box::new(SparseAttention::new(config.dim, 16, 4)),
            _ => Box::new(ScaledDotProductAttention::new(config.dim)),
        };

        Self {
            config,
            focus: None,
            focus_history: VecDeque::with_capacity(10),
            mechanism,
            recent_attended: VecDeque::with_capacity(5),
        }
    }

    /// Set top-down focus target
    pub fn set_focus(&mut self, target: &[f64]) {
        if let Some(old_focus) = self.focus.take() {
            self.focus_history.push_back(old_focus);
            if self.focus_history.len() > 10 {
                self.focus_history.pop_front();
            }
        }
        self.focus = Some(target.to_vec());
    }

    /// Clear focus
    pub fn clear_focus(&mut self) {
        self.focus = None;
    }

    /// Get current focus
    pub fn current_focus(&self) -> Option<Vec<f64>> {
        self.focus.clone()
    }

    /// Compute top-down relevance based on current goals
    pub fn compute_relevance(&self, input: &[f64], goals: &[f64]) -> Vec<f64> {
        let n = input.len() / self.config.dim;
        let mut relevance = vec![0.0; n];

        // Compute similarity to goals
        for (i, rel) in relevance.iter_mut().enumerate().take(n) {
            let start = i * self.config.dim;
            let end = (start + self.config.dim).min(input.len());
            let item = &input[start..end];

            // Cosine similarity to goals
            let mut dot = 0.0;
            let mut norm_item = 0.0;
            let mut norm_goals = 0.0;

            for (j, &x) in item.iter().enumerate() {
                if let Some(&g) = goals.get(j) {
                    dot += x * g;
                }
                norm_item += x * x;
            }
            for &g in goals.iter().take(self.config.dim) {
                norm_goals += g * g;
            }

            norm_item = norm_item.sqrt();
            norm_goals = norm_goals.sqrt();

            if norm_item > 0.0 && norm_goals > 0.0 {
                *rel = (dot / (norm_item * norm_goals) + 1.0) / 2.0; // Normalize to [0,1]
            }

            // Boost if matches current focus
            if let Some(ref focus) = self.focus {
                let focus_sim = Self::cosine_similarity(item, focus);
                *rel = (*rel + focus_sim) / 2.0;
            }
        }

        relevance
    }

    /// Combine top-down and bottom-up priorities
    pub fn combine_priorities(&self, salience: &[f64], relevance: &[f64]) -> PriorityMap {
        let n = salience.len().min(relevance.len());
        let mut map = PriorityMap::new(n);

        map.bottom_up = salience[..n].to_vec();
        map.top_down = relevance[..n].to_vec();

        // Weighted combination
        for i in 0..n {
            let td = self.config.top_down_weight * relevance[i];
            let bu = self.config.bottom_up_weight * salience[i];
            map.combined[i] = td + bu;

            // Inhibition of return: reduce priority of recently attended
            if self.recent_attended.contains(&i) {
                map.combined[i] *= 0.5;
            }
        }

        map.normalize();
        map
    }

    /// Apply attention mechanism with priorities
    pub fn apply_attention(
        &mut self,
        input: &[f64],
        priority: &PriorityMap,
        context: &[f64],
    ) -> Result<AttentionOutput> {
        // Use priorities as soft mask
        let n = priority.combined.len();
        let threshold = 0.1 / n as f64; // Only attend to above-average priorities
        let mask: Vec<bool> = priority.combined.iter().map(|&p| p > threshold).collect();

        // Query is combination of focus and context
        let query = if let Some(ref focus) = self.focus {
            // Blend focus with context
            let mut q = vec![0.0; self.config.dim];
            for (i, q_val) in q.iter_mut().enumerate().take(self.config.dim) {
                let f = focus.get(i).copied().unwrap_or(0.0);
                let c = context.get(i).copied().unwrap_or(0.0);
                *q_val = 0.7 * f + 0.3 * c; // Focus-weighted query
            }
            q
        } else {
            context.to_vec()
        };

        // Apply attention
        let output = self.mechanism.compute(&query, input, input, Some(&mask));

        // Update inhibition of return
        self.recent_attended.push_back(output.max_index);
        if self.recent_attended.len() > 5 {
            self.recent_attended.pop_front();
        }

        Ok(output)
    }

    /// Decay focus over time
    pub fn decay_focus(&mut self) {
        if let Some(ref mut focus) = self.focus {
            for f in focus.iter_mut() {
                *f *= 1.0 - self.config.focus_decay;
            }

            // Clear if decayed to near-zero
            let norm: f64 = focus.iter().map(|x| x * x).sum::<f64>().sqrt();
            if norm < 0.01 {
                self.focus = None;
            }
        }
    }

    /// Switch attention mechanism
    pub fn set_mechanism(&mut self, mechanism_type: AttentionType) {
        self.mechanism = match mechanism_type {
            AttentionType::Linear => Box::new(LinearAttention::new(self.config.dim)),
            AttentionType::Sparse => Box::new(SparseAttention::new(self.config.dim, 16, 4)),
            _ => Box::new(ScaledDotProductAttention::new(self.config.dim)),
        };
    }

    /// Helper: cosine similarity
    fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
        let mut dot = 0.0;
        let mut norm_a = 0.0;
        let mut norm_b = 0.0;

        for (&x, &y) in a.iter().zip(b.iter()) {
            dot += x * y;
            norm_a += x * x;
            norm_b += y * y;
        }

        norm_a = norm_a.sqrt();
        norm_b = norm_b.sqrt();

        if norm_a > 0.0 && norm_b > 0.0 {
            (dot / (norm_a * norm_b) + 1.0) / 2.0
        } else {
            0.5
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attention_controller_creation() {
        let config = AttentionConfig::default();
        let controller = AttentionController::new(config);

        assert!(controller.current_focus().is_none());
    }

    #[test]
    fn test_set_focus() {
        let config = AttentionConfig::default();
        let mut controller = AttentionController::new(config);

        let target = vec![1.0, 0.0, 0.0, 0.0];
        controller.set_focus(&target);

        assert!(controller.current_focus().is_some());
    }

    #[test]
    fn test_priority_map() {
        let mut map = PriorityMap::new(5);
        map.combined = vec![0.1, 0.3, 0.2, 0.1, 0.3];

        assert_eq!(map.argmax(), 4); // max_by returns last max in case of ties

        let top2 = map.top_k(2);
        assert_eq!(top2.len(), 2);
    }

    #[test]
    fn test_combine_priorities() {
        let config = AttentionConfig::default();
        let controller = AttentionController::new(config);

        let salience = vec![0.5, 0.3, 0.2];
        let relevance = vec![0.2, 0.5, 0.3];

        let map = controller.combine_priorities(&salience, &relevance);

        assert_eq!(map.combined.len(), 3);
        assert!((map.combined.iter().sum::<f64>() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_focus_decay() {
        let mut config = AttentionConfig::default();
        config.focus_decay = 0.5;

        let mut controller = AttentionController::new(config);
        controller.set_focus(&vec![1.0, 1.0, 1.0, 1.0]);

        controller.decay_focus();

        let focus = controller.current_focus().unwrap();
        assert!(focus[0] < 1.0);
    }
}
