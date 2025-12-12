//! Meta-Cognition
//!
//! Thinking about thinking:
//! - Monitoring own cognitive processes
//! - Evaluating reasoning quality
//! - Controlling cognitive strategies
//! - Self-regulation

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use crate::{Result, StrangeLoopError};

/// A meta-level of cognition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaLevel {
    /// Level index (0 = base, higher = more meta)
    pub level: usize,
    /// Current content
    pub content: Vec<f64>,
    /// Monitoring of level below
    pub monitoring: MonitoringState,
    /// Control signals to level below
    pub control: ControlSignal,
}

impl MetaLevel {
    pub fn new(level: usize, dim: usize) -> Self {
        Self {
            level,
            content: vec![0.0; dim],
            monitoring: MonitoringState::new(),
            control: ControlSignal::new(),
        }
    }

    /// Update content
    pub fn update(&mut self, new_content: &[f64]) {
        for (i, &v) in new_content.iter().enumerate() {
            if i < self.content.len() {
                self.content[i] = 0.8 * self.content[i] + 0.2 * v;
            }
        }
    }

    /// Monitor input from lower level
    pub fn monitor(&mut self, lower_content: &[f64]) {
        // Compute statistics about lower level
        let mean: f64 = lower_content.iter().sum::<f64>() / lower_content.len().max(1) as f64;
        let variance: f64 = lower_content
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>()
            / lower_content.len().max(1) as f64;

        self.monitoring.activity_level = mean.abs();
        self.monitoring.variability = variance.sqrt();
        self.monitoring.update_count += 1;

        // Assess quality
        self.monitoring.quality = 1.0 - variance.min(1.0);
    }

    /// Generate control signal
    pub fn generate_control(&mut self) -> &ControlSignal {
        // Adjust based on monitoring
        if self.monitoring.quality < 0.5 {
            self.control.adjustment = 0.2; // Increase regulation
        } else {
            self.control.adjustment = -0.1; // Relax
        }

        self.control.intensity = self.monitoring.activity_level;
        &self.control
    }
}

/// Monitoring state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringState {
    /// Activity level of monitored process
    pub activity_level: f64,
    /// Variability of process
    pub variability: f64,
    /// Quality assessment
    pub quality: f64,
    /// Update count
    pub update_count: usize,
}

impl MonitoringState {
    pub fn new() -> Self {
        Self {
            activity_level: 0.0,
            variability: 0.0,
            quality: 0.5,
            update_count: 0,
        }
    }
}

impl Default for MonitoringState {
    fn default() -> Self {
        Self::new()
    }
}

/// Control signal to lower level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlSignal {
    /// Adjustment magnitude
    pub adjustment: f64,
    /// Control intensity
    pub intensity: f64,
    /// Strategy to use
    pub strategy: ControlStrategy,
}

impl ControlSignal {
    pub fn new() -> Self {
        Self {
            adjustment: 0.0,
            intensity: 0.5,
            strategy: ControlStrategy::Maintain,
        }
    }
}

impl Default for ControlSignal {
    fn default() -> Self {
        Self::new()
    }
}

/// Control strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlStrategy {
    /// Maintain current processing
    Maintain,
    /// Increase focus/intensity
    Intensify,
    /// Decrease/relax
    Relax,
    /// Switch strategy
    Switch,
    /// Inhibit processing
    Inhibit,
}

/// A thought about a thought
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtAboutThought {
    /// Original thought
    pub original: Vec<f64>,
    /// Meta-thought
    pub meta: Vec<f64>,
    /// Evaluation of original
    pub evaluation: ThoughtEvaluation,
    /// Level of reflection
    pub level: usize,
}

/// Evaluation of a thought
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtEvaluation {
    /// Clarity (0-1)
    pub clarity: f64,
    /// Relevance (0-1)
    pub relevance: f64,
    /// Confidence (0-1)
    pub confidence: f64,
    /// Should continue this line of thought?
    pub continue_thinking: bool,
}

impl ThoughtEvaluation {
    pub fn new() -> Self {
        Self {
            clarity: 0.5,
            relevance: 0.5,
            confidence: 0.5,
            continue_thinking: true,
        }
    }
}

impl Default for ThoughtEvaluation {
    fn default() -> Self {
        Self::new()
    }
}

/// Meta-cognition system
pub struct MetaCognition {
    /// Meta-levels
    levels: Vec<MetaLevel>,
    /// Current active level
    current_level: usize,
    /// Thought history
    thought_history: VecDeque<ThoughtAboutThought>,
    /// Maximum history
    max_history: usize,
    /// Dimension
    dim: usize,
}

impl MetaCognition {
    /// Create new meta-cognition system
    pub fn new(num_levels: usize) -> Self {
        let dim = 32;
        let levels = (0..num_levels).map(|i| MetaLevel::new(i, dim)).collect();

        Self {
            levels,
            current_level: 0,
            thought_history: VecDeque::with_capacity(100),
            max_history: 100,
            dim,
        }
    }

    /// Process input through meta-cognitive levels
    pub fn process(&mut self, input: &[f64]) -> Result<Vec<f64>> {
        if self.levels.is_empty() {
            return Err(StrangeLoopError::LevelMismatch {
                expected: 1,
                got: 0,
            });
        }

        // Process through each level
        let mut current = input.to_vec();

        for level_idx in 0..self.levels.len() {
            // Update level
            self.levels[level_idx].update(&current);

            // Monitor lower level
            if level_idx > 0 {
                let lower_content = self.levels[level_idx - 1].content.clone();
                self.levels[level_idx].monitor(&lower_content);

                // Apply control
                let control = self.levels[level_idx].generate_control();
                for v in &mut current {
                    *v *= 1.0 + control.adjustment;
                }
            }

            // Transform for next level (abstraction)
            if level_idx < self.levels.len() - 1 {
                current = current
                    .chunks(2)
                    .map(|chunk| chunk.iter().sum::<f64>() / chunk.len() as f64)
                    .collect();

                // Pad if needed
                while current.len() < self.dim {
                    current.push(0.0);
                }
            }
        }

        self.current_level = self.levels.len() - 1;
        Ok(current)
    }

    /// Think about a thought
    pub fn think_about(&mut self, thought: &[f64]) -> Result<ThoughtAboutThought> {
        // Evaluate the thought
        let mean: f64 = thought.iter().sum::<f64>() / thought.len().max(1) as f64;
        let variance: f64 = thought
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>()
            / thought.len().max(1) as f64;

        let evaluation = ThoughtEvaluation {
            clarity: 1.0 - variance.min(1.0),
            relevance: mean.abs().min(1.0),
            confidence: (1.0 - variance).clamp(0.0, 1.0),
            continue_thinking: variance < 0.5,
        };

        // Generate meta-thought
        let meta: Vec<f64> = thought
            .iter()
            .map(|&x| x * evaluation.confidence)
            .collect();

        let tat = ThoughtAboutThought {
            original: thought.to_vec(),
            meta,
            evaluation,
            level: self.current_level,
        };

        // Store in history
        self.thought_history.push_back(tat.clone());
        if self.thought_history.len() > self.max_history {
            self.thought_history.pop_front();
        }

        Ok(tat)
    }

    /// Get current level
    pub fn current_level(&self) -> usize {
        self.current_level
    }

    /// Get level
    pub fn level(&self, idx: usize) -> Option<&MetaLevel> {
        self.levels.get(idx)
    }

    /// Get recent thoughts
    pub fn recent_thoughts(&self, n: usize) -> Vec<&ThoughtAboutThought> {
        self.thought_history.iter().rev().take(n).collect()
    }

    /// Get monitoring at level
    pub fn monitoring(&self, level: usize) -> Option<&MonitoringState> {
        self.levels.get(level).map(|l| &l.monitoring)
    }

    /// Get overall cognitive quality
    pub fn cognitive_quality(&self) -> f64 {
        if self.levels.is_empty() {
            return 0.0;
        }
        self.levels.iter().map(|l| l.monitoring.quality).sum::<f64>() / self.levels.len() as f64
    }

    /// Reset
    pub fn reset(&mut self) {
        for level in &mut self.levels {
            *level = MetaLevel::new(level.level, self.dim);
        }
        self.current_level = 0;
        self.thought_history.clear();
    }
}

impl Default for MetaCognition {
    fn default() -> Self {
        Self::new(5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta_level() {
        let mut level = MetaLevel::new(1, 10);

        let content = vec![0.5; 10];
        level.update(&content);
        level.monitor(&content);

        assert!(level.monitoring.quality > 0.0);
    }

    #[test]
    fn test_meta_cognition() {
        let mut mc = MetaCognition::new(3);

        let input = vec![0.5; 32];
        let output = mc.process(&input).unwrap();

        assert!(!output.is_empty());
    }

    #[test]
    fn test_think_about() {
        let mut mc = MetaCognition::new(3);

        let thought = vec![0.5; 10];
        let tat = mc.think_about(&thought).unwrap();

        assert_eq!(tat.original.len(), 10);
        assert!(tat.evaluation.clarity > 0.0);
    }

    #[test]
    fn test_cognitive_quality() {
        let mut mc = MetaCognition::new(3);

        // Process some input
        let input = vec![0.5; 32];
        mc.process(&input).unwrap();

        let quality = mc.cognitive_quality();
        assert!(quality >= 0.0 && quality <= 1.0);
    }

    #[test]
    fn test_monitoring() {
        let mut mc = MetaCognition::new(3);

        let input = vec![0.5; 32];
        mc.process(&input).unwrap();

        let monitoring = mc.monitoring(1);
        assert!(monitoring.is_some());
    }
}
