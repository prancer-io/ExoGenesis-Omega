//! Omega Hippocampus
//!
//! Biologically-inspired hippocampal memory system implementing:
//! - Dentate Gyrus (DG): Pattern separation via sparse coding
//! - CA3: Autoassociative memory for pattern completion
//! - CA1: Output layer for memory consolidation
//! - Entorhinal Cortex: Input/output interface
//! - Place cells: Spatial memory and navigation
//! - Sharp-wave ripples: Memory replay and consolidation
//!
//! Based on computational neuroscience models of hippocampal function.

pub mod ca1;
pub mod ca3;
pub mod dentate_gyrus;
pub mod entorhinal;
pub mod place_cells;
pub mod replay;

pub use ca1::{CA1Layer, CA1Neuron, CA1Output};
pub use ca3::{CA3Network, CA3Neuron, PatternCompletion};
pub use dentate_gyrus::{DentateGyrus, GranuleCell, MossyFiber};
pub use entorhinal::{EntorhinalCortex, GridCell, PerforantPath};
pub use place_cells::{PlaceCell, PlaceField, SpatialMap};
pub use replay::{ReplayBuffer, ReplayEvent, SharpWaveRipple};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Hippocampus errors
#[derive(Debug, Error)]
pub enum HippocampusError {
    #[error("Pattern size mismatch: expected {expected}, got {got}")]
    PatternSizeMismatch { expected: usize, got: usize },

    #[error("Memory capacity exceeded: {0}")]
    CapacityExceeded(String),

    #[error("Pattern not found: {0}")]
    PatternNotFound(String),

    #[error("Encoding failed: {0}")]
    EncodingFailed(String),

    #[error("Replay failed: {0}")]
    ReplayFailed(String),
}

pub type Result<T> = std::result::Result<T, HippocampusError>;

/// Configuration for the hippocampal system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HippocampusConfig {
    /// Dimension of input patterns
    pub input_dim: usize,
    /// Number of granule cells in DG (typically 10x input)
    pub dg_size: usize,
    /// Number of CA3 neurons
    pub ca3_size: usize,
    /// Number of CA1 neurons
    pub ca1_size: usize,
    /// Sparsity of DG representation (fraction active)
    pub dg_sparsity: f64,
    /// CA3 recurrent connection probability
    pub ca3_recurrence: f64,
    /// Learning rate for synaptic plasticity
    pub learning_rate: f64,
    /// Replay buffer size
    pub replay_buffer_size: usize,
    /// Sharp-wave ripple threshold
    pub ripple_threshold: f64,
}

impl Default for HippocampusConfig {
    fn default() -> Self {
        Self {
            input_dim: 256,
            dg_size: 2560, // 10x expansion
            ca3_size: 512,
            ca1_size: 256,
            dg_sparsity: 0.02, // 2% active (sparse coding)
            ca3_recurrence: 0.04,
            learning_rate: 0.01,
            replay_buffer_size: 1000,
            ripple_threshold: 0.7,
        }
    }
}

/// Memory trace stored in hippocampus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryTrace {
    /// Unique identifier
    pub id: String,
    /// Original input pattern
    pub input: Vec<f64>,
    /// DG sparse representation
    pub dg_code: Vec<f64>,
    /// CA3 representation
    pub ca3_code: Vec<f64>,
    /// CA1 output
    pub ca1_output: Vec<f64>,
    /// Timestamp of encoding
    pub timestamp: u64,
    /// Strength of memory (consolidation level)
    pub strength: f64,
    /// Number of times replayed
    pub replay_count: u32,
    /// Associated context/location
    pub context: Option<Vec<f64>>,
}

impl MemoryTrace {
    pub fn new(id: String, input: Vec<f64>) -> Self {
        Self {
            id,
            input,
            dg_code: Vec::new(),
            ca3_code: Vec::new(),
            ca1_output: Vec::new(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            strength: 1.0,
            replay_count: 0,
            context: None,
        }
    }
}

/// The complete hippocampal formation
pub struct Hippocampus {
    config: HippocampusConfig,
    /// Entorhinal cortex (input/output interface)
    entorhinal: EntorhinalCortex,
    /// Dentate gyrus (pattern separation)
    dentate_gyrus: DentateGyrus,
    /// CA3 (autoassociative memory)
    ca3: CA3Network,
    /// CA1 (output layer)
    ca1: CA1Layer,
    /// Place cell system
    place_cells: SpatialMap,
    /// Replay buffer
    replay: ReplayBuffer,
    /// Stored memory traces
    memories: HashMap<String, MemoryTrace>,
    /// Current theta phase (0 to 2π)
    theta_phase: f64,
    /// Theta frequency (Hz)
    theta_frequency: f64,
}

impl Hippocampus {
    /// Create new hippocampus with default configuration
    pub fn new() -> Self {
        Self::with_config(HippocampusConfig::default())
    }

    /// Create hippocampus with custom configuration
    pub fn with_config(config: HippocampusConfig) -> Self {
        let entorhinal = EntorhinalCortex::new(config.input_dim, config.dg_size);
        let dentate_gyrus = DentateGyrus::new(config.dg_size, config.dg_sparsity);
        let ca3 = CA3Network::new(config.ca3_size, config.dg_size, config.ca3_recurrence);
        let ca1 = CA1Layer::new(config.ca1_size, config.ca3_size);
        let place_cells = SpatialMap::new(64.0, 64.0, 100); // 64x64 environment, 100 place cells
        let replay = ReplayBuffer::new(config.replay_buffer_size);

        Self {
            config,
            entorhinal,
            dentate_gyrus,
            ca3,
            ca1,
            place_cells,
            replay,
            memories: HashMap::new(),
            theta_phase: 0.0,
            theta_frequency: 8.0, // 8 Hz theta
        }
    }

    /// Encode a new memory
    pub fn encode(&mut self, input: &[f64], context: Option<&[f64]>) -> Result<String> {
        if input.len() != self.config.input_dim {
            return Err(HippocampusError::PatternSizeMismatch {
                expected: self.config.input_dim,
                got: input.len(),
            });
        }

        // Generate memory ID
        let id = uuid::Uuid::now_v7().to_string();
        let mut trace = MemoryTrace::new(id.clone(), input.to_vec());

        // Step 1: Entorhinal cortex preprocessing
        let ec_output = self.entorhinal.process(input);

        // Step 2: Dentate gyrus pattern separation
        let dg_output = self.dentate_gyrus.separate(&ec_output);
        trace.dg_code = dg_output.clone();

        // Step 3: CA3 encoding (sparse DG → CA3)
        let ca3_output = self.ca3.encode(&dg_output);
        trace.ca3_code = ca3_output.clone();

        // Step 4: CA3 → CA1 transfer
        let ca1_output = self.ca1.process(&ca3_output);
        trace.ca1_output = ca1_output;

        // Store context if provided
        if let Some(ctx) = context {
            trace.context = Some(ctx.to_vec());
        }

        // Add to replay buffer
        self.replay.add(ReplayEvent {
            memory_id: id.clone(),
            pattern: trace.ca3_code.clone(),
            timestamp: trace.timestamp,
            priority: 1.0,
        });

        // Store memory trace
        self.memories.insert(id.clone(), trace);

        Ok(id)
    }

    /// Retrieve/complete a memory pattern
    pub fn retrieve(&mut self, cue: &[f64]) -> Result<Vec<f64>> {
        if cue.len() != self.config.input_dim {
            return Err(HippocampusError::PatternSizeMismatch {
                expected: self.config.input_dim,
                got: cue.len(),
            });
        }

        // Entorhinal preprocessing
        let ec_output = self.entorhinal.process(cue);

        // DG separation (creates retrieval cue)
        let dg_output = self.dentate_gyrus.separate(&ec_output);

        // CA3 pattern completion
        let completed = self.ca3.complete(&dg_output);

        // CA1 output
        let output = self.ca1.process(&completed);

        // Entorhinal decoding (back to input space)
        let retrieved = self.entorhinal.decode(&output);

        Ok(retrieved)
    }

    /// Retrieve by memory ID
    pub fn retrieve_by_id(&self, id: &str) -> Option<&MemoryTrace> {
        self.memories.get(id)
    }

    /// Perform replay during "offline" periods (sleep)
    pub fn replay(&mut self, num_events: usize) -> Vec<String> {
        let events = self.replay.sample(num_events);
        let mut replayed_ids = Vec::new();

        for event in events {
            if let Some(trace) = self.memories.get_mut(&event.memory_id) {
                // Replay strengthens memory
                trace.strength *= 1.1;
                trace.strength = trace.strength.min(10.0);
                trace.replay_count += 1;

                // Reactivate CA3 pattern
                self.ca3.reactivate(&event.pattern);

                // Update CA1
                let ca1_out = self.ca1.process(&event.pattern);
                trace.ca1_output = ca1_out;

                replayed_ids.push(event.memory_id.clone());
            }
        }

        replayed_ids
    }

    /// Generate sharp-wave ripple replay
    pub fn sharp_wave_ripple(&mut self) -> Option<SharpWaveRipple> {
        // Check if conditions are right for SWR
        let activity_level = self.ca3.get_activity_level();

        if activity_level > self.config.ripple_threshold {
            // Sample memories for replay
            let events = self.replay.sample_prioritized(5);

            if !events.is_empty() {
                let patterns: Vec<Vec<f64>> = events.iter().map(|e| e.pattern.clone()).collect();

                let ripple = SharpWaveRipple {
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis() as u64,
                    patterns,
                    duration_ms: 100,
                    frequency_hz: 150.0, // Ripple frequency
                };

                // Process replay
                for event in events {
                    if let Some(trace) = self.memories.get_mut(&event.memory_id) {
                        trace.strength *= 1.2;
                        trace.replay_count += 1;
                    }
                }

                return Some(ripple);
            }
        }

        None
    }

    /// Update spatial representation (place cells)
    pub fn update_location(&mut self, x: f64, y: f64) {
        self.place_cells.update_position(x, y);
    }

    /// Get current place cell activity
    pub fn get_place_activity(&self) -> Vec<f64> {
        self.place_cells.get_activity()
    }

    /// Advance theta oscillation
    pub fn step_theta(&mut self, dt: f64) {
        self.theta_phase += 2.0 * std::f64::consts::PI * self.theta_frequency * dt;
        if self.theta_phase > 2.0 * std::f64::consts::PI {
            self.theta_phase -= 2.0 * std::f64::consts::PI;
        }
    }

    /// Get current theta phase
    pub fn theta_phase(&self) -> f64 {
        self.theta_phase
    }

    /// Get memory count
    pub fn memory_count(&self) -> usize {
        self.memories.len()
    }

    /// Get all memory IDs
    pub fn memory_ids(&self) -> Vec<String> {
        self.memories.keys().cloned().collect()
    }

    /// Decay old memories
    pub fn decay(&mut self, factor: f64) {
        for trace in self.memories.values_mut() {
            trace.strength *= factor;
        }

        // Remove very weak memories
        self.memories.retain(|_, trace| trace.strength > 0.01);
    }

    /// Get statistics
    pub fn stats(&self) -> HippocampusStats {
        HippocampusStats {
            memory_count: self.memories.len(),
            replay_buffer_size: self.replay.len(),
            average_strength: if self.memories.is_empty() {
                0.0
            } else {
                self.memories.values().map(|m| m.strength).sum::<f64>()
                    / self.memories.len() as f64
            },
            ca3_activity: self.ca3.get_activity_level(),
            theta_phase: self.theta_phase,
        }
    }

    /// Clear all memories
    pub fn clear(&mut self) {
        self.memories.clear();
        self.replay.clear();
        self.ca3.reset();
        self.ca1.reset();
    }
}

impl Default for Hippocampus {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about hippocampus state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HippocampusStats {
    pub memory_count: usize,
    pub replay_buffer_size: usize,
    pub average_strength: f64,
    pub ca3_activity: f64,
    pub theta_phase: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hippocampus_creation() {
        let hippo = Hippocampus::new();
        assert_eq!(hippo.memory_count(), 0);
    }

    #[test]
    fn test_encode_retrieve() {
        let mut hippo = Hippocampus::with_config(HippocampusConfig {
            input_dim: 64,
            dg_size: 256,
            ca3_size: 128,
            ca1_size: 64,
            ..Default::default()
        });

        // Encode a pattern
        let input: Vec<f64> = (0..64).map(|i| (i as f64 / 64.0)).collect();
        let id = hippo.encode(&input, None).unwrap();

        assert_eq!(hippo.memory_count(), 1);

        // Retrieve with partial cue
        let mut cue = input.clone();
        for i in 32..64 {
            cue[i] = 0.0; // Zero out half
        }

        let retrieved = hippo.retrieve(&cue).unwrap();
        assert_eq!(retrieved.len(), 64);
    }

    #[test]
    fn test_replay() {
        let mut hippo = Hippocampus::with_config(HippocampusConfig {
            input_dim: 32,
            dg_size: 128,
            ca3_size: 64,
            ca1_size: 32,
            ..Default::default()
        });

        // Encode multiple patterns
        for i in 0..5 {
            let input: Vec<f64> = (0..32).map(|j| ((i + j) as f64 / 32.0)).collect();
            hippo.encode(&input, None).unwrap();
        }

        // Replay
        let replayed = hippo.replay(3);
        assert!(replayed.len() <= 3);
    }

    #[test]
    fn test_theta_oscillation() {
        let mut hippo = Hippocampus::new();

        hippo.step_theta(0.01);
        assert!(hippo.theta_phase() > 0.0);

        // Full cycle
        for _ in 0..1000 {
            hippo.step_theta(0.001);
        }
        assert!(hippo.theta_phase() < 2.0 * std::f64::consts::PI);
    }

    #[test]
    fn test_decay() {
        let mut hippo = Hippocampus::with_config(HippocampusConfig {
            input_dim: 16,
            dg_size: 64,
            ca3_size: 32,
            ca1_size: 16,
            ..Default::default()
        });

        let input = vec![0.5; 16];
        hippo.encode(&input, None).unwrap();

        let initial_strength = hippo.memories.values().next().unwrap().strength;

        hippo.decay(0.9);

        let new_strength = hippo.memories.values().next().unwrap().strength;
        assert!(new_strength < initial_strength);
    }
}
