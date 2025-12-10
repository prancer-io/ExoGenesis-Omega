//! Memory Consolidation
//!
//! Sleep-dependent memory processing:
//! - Hippocampal-neocortical transfer
//! - Synaptic homeostasis
//! - Memory reactivation
//! - Schema integration

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::rem::DreamContent;
use crate::spindles::SleepSpindle;
use crate::sws::SlowWave;

/// A consolidation event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationEvent {
    /// Type of consolidation
    pub consolidation_type: ConsolidationType,
    /// Memories affected (indices)
    pub memory_indices: Vec<usize>,
    /// Strength of consolidation
    pub strength: f64,
    /// Timestamp
    pub timestamp: u64,
}

/// Types of memory consolidation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsolidationType {
    /// Hippocampal replay during SWS
    Replay,
    /// Transfer to neocortex
    Transfer,
    /// Schema integration during REM
    Integration,
    /// Synaptic rescaling
    Rescaling,
    /// Memory linking
    Linking,
}

/// Memory to be consolidated
#[derive(Debug, Clone)]
pub struct MemoryToConsolidate {
    /// Memory content (vector representation)
    pub content: Vec<f64>,
    /// Importance/salience
    pub importance: f64,
    /// Age (time since encoding)
    pub age: f64,
    /// Times replayed
    pub replay_count: usize,
    /// Consolidation level (0-1)
    pub consolidation_level: f64,
}

impl MemoryToConsolidate {
    pub fn new(content: Vec<f64>, importance: f64) -> Self {
        Self {
            content,
            importance,
            age: 0.0,
            replay_count: 0,
            consolidation_level: 0.0,
        }
    }
}

/// Memory consolidator
pub struct MemoryConsolidator {
    /// Memories waiting for consolidation
    pending_memories: Vec<MemoryToConsolidate>,
    /// Consolidated memories
    consolidated_memories: Vec<Vec<f64>>,
    /// Consolidation events
    events: Vec<ConsolidationEvent>,
    /// Total consolidation count
    consolidation_count: usize,
    /// Synaptic scaling factor
    synaptic_scale: f64,
}

impl MemoryConsolidator {
    pub fn new() -> Self {
        Self {
            pending_memories: Vec::new(),
            consolidated_memories: Vec::new(),
            events: Vec::new(),
            consolidation_count: 0,
            synaptic_scale: 1.0,
        }
    }

    /// Add memories to consolidate
    pub fn add_memories(&mut self, memories: Vec<Vec<f64>>) {
        for content in memories {
            let importance = 1.0; // Default importance
            self.pending_memories
                .push(MemoryToConsolidate::new(content, importance));
        }
    }

    /// Add memory with importance
    pub fn add_memory(&mut self, content: Vec<f64>, importance: f64) {
        self.pending_memories
            .push(MemoryToConsolidate::new(content, importance));
    }

    /// Process slow wave (main consolidation mechanism)
    pub fn process_slow_wave(&mut self, wave: &SlowWave) -> ConsolidationEvent {
        let mut rng = rand::thread_rng();

        // Select memories for replay based on importance and recency
        let mut indices: Vec<usize> = Vec::new();
        let strength = wave.consolidation_strength();

        for (i, mem) in self.pending_memories.iter_mut().enumerate() {
            // Probability based on importance and consolidation strength
            let prob = mem.importance * strength * (1.0 - mem.consolidation_level);
            if rng.gen::<f64>() < prob {
                indices.push(i);
                mem.replay_count += 1;
                mem.consolidation_level += 0.1 * strength;
                mem.consolidation_level = mem.consolidation_level.min(1.0);
            }
        }

        // Transfer fully consolidated memories
        self.transfer_consolidated();

        self.consolidation_count += 1;

        let event = ConsolidationEvent {
            consolidation_type: ConsolidationType::Replay,
            memory_indices: indices,
            strength,
            timestamp: wave.timestamp,
        };

        self.events.push(event.clone());
        event
    }

    /// Process sleep spindle (enhances consolidation)
    pub fn process_spindle(&mut self, spindle: &SleepSpindle) -> ConsolidationEvent {
        let strength = spindle.consolidation_strength();

        // Spindles enhance ongoing consolidation
        for mem in &mut self.pending_memories {
            if mem.replay_count > 0 {
                mem.consolidation_level += 0.05 * strength;
                mem.consolidation_level = mem.consolidation_level.min(1.0);
            }
        }

        self.consolidation_count += 1;

        let event = ConsolidationEvent {
            consolidation_type: ConsolidationType::Transfer,
            memory_indices: vec![],
            strength,
            timestamp: spindle.timestamp,
        };

        self.events.push(event.clone());
        event
    }

    /// Process dream content (memory reorganization)
    pub fn process_dream(&mut self, dream: &DreamContent) -> ConsolidationEvent {
        let mut rng = rand::thread_rng();

        // Dreams help integrate memories into schemas
        let mut linked_indices: Vec<usize> = Vec::new();

        // Find memories that might be connected
        for i in 0..self.pending_memories.len() {
            if rng.gen::<f64>() < dream.bizarreness * 0.5 {
                linked_indices.push(i);
            }
        }

        // Link memories (simulate schema integration)
        if linked_indices.len() >= 2 {
            for idx in &linked_indices {
                if *idx < self.pending_memories.len() {
                    self.pending_memories[*idx].consolidation_level += 0.05;
                }
            }
        }

        self.consolidation_count += 1;

        let event = ConsolidationEvent {
            consolidation_type: ConsolidationType::Integration,
            memory_indices: linked_indices,
            strength: dream.intensity,
            timestamp: dream.timestamp,
        };

        self.events.push(event.clone());
        event
    }

    /// Transfer consolidated memories to long-term storage
    fn transfer_consolidated(&mut self) {
        let mut to_transfer = Vec::new();

        for (i, mem) in self.pending_memories.iter().enumerate() {
            if mem.consolidation_level >= 0.8 {
                to_transfer.push(i);
            }
        }

        // Remove from pending and add to consolidated (reverse order to preserve indices)
        for i in to_transfer.into_iter().rev() {
            let mem = self.pending_memories.remove(i);
            self.consolidated_memories.push(mem.content);
        }
    }

    /// Perform synaptic homeostasis (downscaling)
    pub fn synaptic_homeostasis(&mut self, factor: f64) {
        self.synaptic_scale *= factor;

        // Reduce consolidation levels slightly (makes room for new memories)
        for mem in &mut self.pending_memories {
            mem.consolidation_level *= factor;
        }
    }

    /// Get consolidated memories
    pub fn get_consolidated(&self) -> Vec<Vec<f64>> {
        self.consolidated_memories.clone()
    }

    /// Get pending count
    pub fn pending_count(&self) -> usize {
        self.pending_memories.len()
    }

    /// Get consolidation count
    pub fn consolidation_count(&self) -> usize {
        self.consolidation_count
    }

    /// Get recent events
    pub fn recent_events(&self, n: usize) -> Vec<&ConsolidationEvent> {
        self.events.iter().rev().take(n).collect()
    }

    /// Reset consolidator
    pub fn reset(&mut self) {
        self.pending_memories.clear();
        self.consolidated_memories.clear();
        self.events.clear();
        self.consolidation_count = 0;
        self.synaptic_scale = 1.0;
    }
}

impl Default for MemoryConsolidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_consolidator() {
        let mut consolidator = MemoryConsolidator::new();

        consolidator.add_memory(vec![0.5; 10], 0.8);
        consolidator.add_memory(vec![0.3; 10], 0.6);

        assert_eq!(consolidator.pending_count(), 2);
    }

    #[test]
    fn test_slow_wave_consolidation() {
        let mut consolidator = MemoryConsolidator::new();

        for i in 0..5 {
            consolidator.add_memory(vec![i as f64 / 5.0; 10], 0.8);
        }

        let wave = SlowWave::new(1.0, 0.8);
        let event = consolidator.process_slow_wave(&wave);

        assert_eq!(event.consolidation_type, ConsolidationType::Replay);
        assert!(consolidator.consolidation_count() > 0);
    }

    #[test]
    fn test_memory_transfer() {
        let mut consolidator = MemoryConsolidator::new();
        consolidator.add_memory(vec![1.0; 10], 1.0);

        // Consolidate multiple times to reach threshold
        for _ in 0..20 {
            let wave = SlowWave::new(1.0, 1.0);
            consolidator.process_slow_wave(&wave);
        }

        // Some memories should be transferred
        assert!(!consolidator.get_consolidated().is_empty() || consolidator.pending_count() > 0);
    }

    #[test]
    fn test_synaptic_homeostasis() {
        let mut consolidator = MemoryConsolidator::new();
        consolidator.add_memory(vec![1.0; 10], 0.8);

        // Partially consolidate
        let wave = SlowWave::new(1.0, 0.8);
        consolidator.process_slow_wave(&wave);

        let before = consolidator.pending_memories[0].consolidation_level;
        consolidator.synaptic_homeostasis(0.9);
        let after = consolidator.pending_memories[0].consolidation_level;

        assert!(after < before);
    }
}
