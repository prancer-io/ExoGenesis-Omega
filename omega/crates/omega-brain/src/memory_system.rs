//! Memory System Integration
//!
//! Integrates omega-hippocampus for episodic memory:
//! - Pattern separation (Dentate Gyrus)
//! - Pattern completion (CA3)
//! - Memory output (CA1)
//! - Replay during sleep

use crate::{BrainConfig, BrainError, Result};
use crate::sleep_system::SleepOutput;
use omega_hippocampus::{Hippocampus, Memory, MemoryType, ReplayBuffer};

/// Memory system wrapping hippocampal circuits
pub struct MemorySystem {
    /// Hippocampus
    hippocampus: Hippocampus,
    /// Replay buffer for consolidation
    replay_buffer: ReplayBuffer,
    /// Consolidation count
    consolidation_count: usize,
    /// Memory count
    memory_count: usize,
    /// Dimension
    dim: usize,
}

impl MemorySystem {
    /// Create new memory system
    pub fn new(config: &BrainConfig) -> Self {
        let hippocampus = Hippocampus::new(
            config.pattern_dim,
            config.ca3_size,
            config.consolidation_threshold,
        );

        let replay_buffer = ReplayBuffer::new(config.replay_buffer_size);

        Self {
            hippocampus,
            replay_buffer,
            consolidation_count: 0,
            memory_count: 0,
            dim: config.pattern_dim,
        }
    }

    /// Process conscious content through memory
    pub fn process(&mut self, conscious_content: &[f64]) -> Result<Vec<f64>> {
        // Normalize input
        let normalized = self.normalize_input(conscious_content);

        // Try to retrieve similar memory
        if let Some(retrieved) = self.hippocampus.retrieve(&normalized) {
            // Blend retrieved with current
            let blended = self.blend(&normalized, &retrieved.pattern);
            Ok(blended)
        } else {
            // Encode as new memory if significant
            let significance = self.compute_significance(&normalized);
            if significance > 0.3 {
                self.encode(&normalized, significance)?;
            }
            Ok(normalized)
        }
    }

    /// Encode a new memory
    pub fn encode(&mut self, content: &[f64], importance: f64) -> Result<()> {
        let normalized = self.normalize_input(content);
        self.hippocampus.encode(&normalized, importance);
        self.memory_count += 1;

        // Add to replay buffer
        self.replay_buffer.add(normalized, importance);

        Ok(())
    }

    /// Retrieve a memory by cue
    pub fn retrieve(&self, cue: &[f64]) -> Result<Option<Vec<f64>>> {
        let normalized = self.normalize_input(cue);
        if let Some(memory) = self.hippocampus.retrieve(&normalized) {
            Ok(Some(memory.pattern))
        } else {
            Ok(None)
        }
    }

    /// Consolidate during slow wave sleep
    pub fn consolidate_slow_wave(&mut self, sleep_output: &SleepOutput) -> Result<()> {
        // Replay memories with slow oscillations
        let memories_to_replay = self.replay_buffer.sample(sleep_output.replay_count);

        for memory in memories_to_replay {
            // Strengthen memory through replay
            self.hippocampus.strengthen(&memory.pattern, 0.1);
        }

        self.consolidation_count += 1;
        Ok(())
    }

    /// Consolidate during REM sleep
    pub fn consolidate_rem(&mut self, sleep_output: &SleepOutput) -> Result<()> {
        // Creative recombination during REM
        let memories = self.replay_buffer.sample(sleep_output.replay_count);

        if memories.len() >= 2 {
            // Create novel combinations
            for i in 0..memories.len() - 1 {
                let combined = self.recombine(&memories[i].pattern, &memories[i + 1].pattern);
                // Don't actually store - just strengthen associations
                self.hippocampus.strengthen_association(
                    &memories[i].pattern,
                    &memories[i + 1].pattern,
                    0.05,
                );
            }
        }

        Ok(())
    }

    /// Force consolidation
    pub fn force_consolidation(&mut self) -> Result<usize> {
        let count = self.hippocampus.consolidate_all();
        self.consolidation_count += 1;
        Ok(count)
    }

    /// Blend two patterns
    fn blend(&self, a: &[f64], b: &[f64]) -> Vec<f64> {
        a.iter()
            .zip(b.iter())
            .map(|(&x, &y)| 0.6 * x + 0.4 * y)
            .collect()
    }

    /// Recombine patterns (creative mixing)
    fn recombine(&self, a: &[f64], b: &[f64]) -> Vec<f64> {
        a.iter()
            .zip(b.iter())
            .enumerate()
            .map(|(i, (&x, &y))| {
                if i % 2 == 0 {
                    x
                } else {
                    y
                }
            })
            .collect()
    }

    /// Compute significance of content
    fn compute_significance(&self, content: &[f64]) -> f64 {
        let mean = content.iter().sum::<f64>() / content.len().max(1) as f64;
        let variance = content.iter().map(|x| (x - mean).powi(2)).sum::<f64>()
            / content.len().max(1) as f64;

        // High mean and moderate variance = significant
        mean.abs() * (1.0 - (variance - 0.3).abs().min(1.0))
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

    /// Get consolidation ratio
    pub fn consolidation_ratio(&self) -> f64 {
        if self.memory_count == 0 {
            0.0
        } else {
            self.consolidation_count as f64 / self.memory_count as f64
        }
    }

    /// Get memory count
    pub fn memory_count(&self) -> usize {
        self.memory_count
    }

    /// Get consolidation count
    pub fn consolidation_count(&self) -> usize {
        self.consolidation_count
    }

    /// Reset the memory system
    pub fn reset(&mut self) {
        self.hippocampus = Hippocampus::new(self.dim, 500, 0.7);
        self.replay_buffer.clear();
        self.consolidation_count = 0;
        self.memory_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_system_creation() {
        let config = BrainConfig::default();
        let system = MemorySystem::new(&config);

        assert_eq!(system.memory_count(), 0);
    }

    #[test]
    fn test_encode_retrieve() {
        let config = BrainConfig::minimal();
        let mut system = MemorySystem::new(&config);

        let content = vec![0.5; config.pattern_dim];
        system.encode(&content, 1.0).unwrap();

        let retrieved = system.retrieve(&content).unwrap();
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_process() {
        let config = BrainConfig::minimal();
        let mut system = MemorySystem::new(&config);

        let content = vec![0.5; config.pattern_dim];
        let result = system.process(&content).unwrap();

        assert_eq!(result.len(), config.pattern_dim);
    }

    #[test]
    fn test_consolidation_ratio() {
        let config = BrainConfig::minimal();
        let mut system = MemorySystem::new(&config);

        // Encode some memories
        for i in 0..5 {
            let content: Vec<f64> = (0..config.pattern_dim).map(|j| (i + j) as f64 / 100.0).collect();
            system.encode(&content, 0.8).unwrap();
        }

        // Force consolidation
        system.force_consolidation().unwrap();

        let ratio = system.consolidation_ratio();
        assert!(ratio > 0.0);
    }
}
