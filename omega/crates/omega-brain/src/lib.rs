//! Omega Brain
//!
//! Unified brain-like cognitive architecture that integrates all Omega components
//! into a coherent cognitive system resembling biological brain function:
//!
//! - **Neural Substrate** (omega-snn): Spiking neurons with STDP learning
//! - **Attention System** (omega-attention): 39 attention mechanisms
//! - **Consciousness Core** (omega-consciousness): IIT, GWT, Free Energy
//! - **Memory System** (omega-hippocampus): Pattern separation/completion, replay
//! - **Sleep System** (omega-sleep): Consolidation during SWS/REM
//! - **Self-Awareness** (omega-strange-loops): Meta-cognition, self-model
//!
//! The cognitive cycle follows: Perception → Attention → Processing → Memory → Action

pub mod attention_system;
pub mod cognitive_cycle;
pub mod config;
pub mod consciousness_core;
pub mod memory_system;
pub mod neural_substrate;
pub mod self_awareness;
pub mod sleep_system;

pub use attention_system::AttentionSystem;
pub use cognitive_cycle::{CognitiveCycle, CognitiveState, ProcessingResult};
pub use config::{BrainConfig, BrainMode};
pub use consciousness_core::ConsciousnessCore;
pub use memory_system::MemorySystem;
pub use neural_substrate::NeuralSubstrate;
pub use self_awareness::SelfAwarenessSystem;
pub use sleep_system::SleepSystem;

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

/// Errors in brain processing
#[derive(Debug, Error)]
pub enum BrainError {
    #[error("Neural processing failed: {0}")]
    NeuralError(String),

    #[error("Attention allocation failed: {0}")]
    AttentionError(String),

    #[error("Consciousness integration failed: {0}")]
    ConsciousnessError(String),

    #[error("Memory operation failed: {0}")]
    MemoryError(String),

    #[error("Sleep cycle interrupted: {0}")]
    SleepError(String),

    #[error("Self-model inconsistency: {0}")]
    SelfModelError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

pub type Result<T> = std::result::Result<T, BrainError>;

/// Brain state snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainState {
    /// Current cognitive state
    pub cognitive_state: CognitiveStateSnapshot,
    /// Consciousness level (0-1)
    pub consciousness_level: f64,
    /// Attention focus
    pub attention_focus: Vec<f64>,
    /// Self-reference strength
    pub self_reference: f64,
    /// Current sleep stage (if sleeping)
    pub sleep_stage: Option<String>,
    /// Processing cycle count
    pub cycle_count: u64,
    /// Timestamp
    pub timestamp: u64,
}

/// Simplified cognitive state for serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveStateSnapshot {
    pub mode: String,
    pub activity_level: f64,
    pub integration: f64,
}

/// Brain metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainMetrics {
    /// Total processing cycles
    pub cycles: u64,
    /// Average processing time (ms)
    pub avg_processing_time: f64,
    /// IIT Phi value
    pub phi: f64,
    /// Free energy
    pub free_energy: f64,
    /// Memory consolidation ratio
    pub consolidation_ratio: f64,
    /// Strange loop count
    pub strange_loop_count: usize,
    /// Neural spike rate
    pub spike_rate: f64,
}

/// The unified Omega Brain
///
/// Integrates all cognitive components into a single coherent system
/// that processes information in a brain-like manner.
pub struct OmegaBrain {
    /// Configuration
    config: BrainConfig,
    /// Neural substrate (spiking networks)
    neural: Arc<RwLock<NeuralSubstrate>>,
    /// Attention system
    attention: Arc<RwLock<AttentionSystem>>,
    /// Consciousness core (IIT, GWT, FEP)
    consciousness: Arc<RwLock<ConsciousnessCore>>,
    /// Memory system (hippocampus)
    memory: Arc<RwLock<MemorySystem>>,
    /// Sleep system
    sleep: Arc<RwLock<SleepSystem>>,
    /// Self-awareness (strange loops)
    self_awareness: Arc<RwLock<SelfAwarenessSystem>>,
    /// Cognitive cycle manager
    cognitive_cycle: Arc<RwLock<CognitiveCycle>>,
    /// Processing cycle count
    cycle_count: Arc<RwLock<u64>>,
    /// Is brain active
    is_active: Arc<RwLock<bool>>,
}

impl OmegaBrain {
    /// Create a new Omega Brain with default configuration
    pub fn new() -> Self {
        Self::with_config(BrainConfig::default())
    }

    /// Create with custom configuration
    pub fn with_config(config: BrainConfig) -> Self {
        let neural = Arc::new(RwLock::new(NeuralSubstrate::new(&config)));
        let attention = Arc::new(RwLock::new(AttentionSystem::new(&config)));
        let consciousness = Arc::new(RwLock::new(ConsciousnessCore::new(&config)));
        let memory = Arc::new(RwLock::new(MemorySystem::new(&config)));
        let sleep = Arc::new(RwLock::new(SleepSystem::new(&config)));
        let self_awareness = Arc::new(RwLock::new(SelfAwarenessSystem::new(&config)));
        let cognitive_cycle = Arc::new(RwLock::new(CognitiveCycle::new(&config)));

        Self {
            config,
            neural,
            attention,
            consciousness,
            memory,
            sleep,
            self_awareness,
            cognitive_cycle,
            cycle_count: Arc::new(RwLock::new(0)),
            is_active: Arc::new(RwLock::new(true)),
        }
    }

    /// Process input through the brain
    ///
    /// This is the main entry point for cognitive processing.
    /// Input flows through: Neural → Attention → Consciousness → Memory → Output
    pub fn process(&self, input: &[f64]) -> Result<ProcessingResult> {
        if !*self.is_active.read() {
            return Err(BrainError::NeuralError("Brain is not active".to_string()));
        }

        // Check if we should be sleeping
        {
            let sleep = self.sleep.read();
            if sleep.should_sleep() {
                // Reduced processing during sleep
                return self.process_during_sleep(input);
            }
        }

        // Full cognitive cycle
        self.run_cognitive_cycle(input)
    }

    /// Run a full cognitive cycle
    fn run_cognitive_cycle(&self, input: &[f64]) -> Result<ProcessingResult> {
        // 1. Neural processing (spiking network)
        let neural_output = {
            let mut neural = self.neural.write();
            neural.process(input)?
        };

        // 2. Attention allocation
        let attended = {
            let mut attention = self.attention.write();
            attention.attend(&neural_output)?
        };

        // 3. Consciousness integration (IIT + GWT + FEP)
        let conscious_content = {
            let mut consciousness = self.consciousness.write();
            consciousness.integrate(&attended)?
        };

        // 4. Memory encoding/retrieval
        let memory_output = {
            let mut memory = self.memory.write();
            memory.process(&conscious_content)?
        };

        // 5. Self-awareness update
        let self_aware_output = {
            let mut self_awareness = self.self_awareness.write();
            self_awareness.reflect(&memory_output)?
        };

        // 6. Update cognitive cycle state
        let result = {
            let mut cycle = self.cognitive_cycle.write();
            cycle.complete_cycle(
                input,
                &neural_output,
                &attended,
                &conscious_content,
                &memory_output,
                &self_aware_output,
            )?
        };

        // Increment cycle counter
        {
            let mut count = self.cycle_count.write();
            *count += 1;
        }

        Ok(result)
    }

    /// Process during sleep (reduced/modified processing)
    fn process_during_sleep(&self, input: &[f64]) -> Result<ProcessingResult> {
        // During sleep, run consolidation instead of full processing
        let mut sleep = self.sleep.write();
        let mut memory = self.memory.write();

        // Run sleep stage processing
        let sleep_output = sleep.process_cycle()?;

        // Consolidate memories based on sleep stage
        if sleep_output.is_sws {
            memory.consolidate_slow_wave(&sleep_output)?;
        } else if sleep_output.is_rem {
            memory.consolidate_rem(&sleep_output)?;
        }

        // Return minimal processing result
        Ok(ProcessingResult {
            output: input.to_vec(),
            consciousness_level: 0.1, // Low during sleep
            attention_strength: 0.0,
            memory_encoded: false,
            strange_loop_detected: false,
            processing_time_ms: 0,
        })
    }

    /// Get current brain state
    pub fn state(&self) -> BrainState {
        let consciousness = self.consciousness.read();
        let attention = self.attention.read();
        let self_awareness = self.self_awareness.read();
        let sleep = self.sleep.read();
        let cycle = self.cognitive_cycle.read();

        BrainState {
            cognitive_state: CognitiveStateSnapshot {
                mode: cycle.current_mode().to_string(),
                activity_level: cycle.activity_level(),
                integration: consciousness.integration_level(),
            },
            consciousness_level: consciousness.consciousness_level(),
            attention_focus: attention.current_focus(),
            self_reference: self_awareness.self_reference_strength(),
            sleep_stage: sleep.current_stage_name(),
            cycle_count: *self.cycle_count.read(),
            timestamp: self.now(),
        }
    }

    /// Get brain metrics
    pub fn metrics(&self) -> BrainMetrics {
        let consciousness = self.consciousness.read();
        let memory = self.memory.read();
        let self_awareness = self.self_awareness.read();
        let neural = self.neural.read();
        let cycle = self.cognitive_cycle.read();

        BrainMetrics {
            cycles: *self.cycle_count.read(),
            avg_processing_time: cycle.avg_processing_time(),
            phi: consciousness.phi(),
            free_energy: consciousness.free_energy(),
            consolidation_ratio: memory.consolidation_ratio(),
            strange_loop_count: self_awareness.loop_count(),
            spike_rate: neural.spike_rate(),
        }
    }

    /// Enter sleep mode
    pub fn sleep(&self) -> Result<()> {
        let mut sleep = self.sleep.write();
        sleep.initiate_sleep()?;
        Ok(())
    }

    /// Wake up from sleep
    pub fn wake(&self) -> Result<()> {
        let mut sleep = self.sleep.write();
        sleep.wake_up()?;
        Ok(())
    }

    /// Force memory consolidation
    pub fn consolidate_memories(&self) -> Result<usize> {
        let mut memory = self.memory.write();
        memory.force_consolidation()
    }

    /// Think about a specific topic (directed cognition)
    pub fn think_about(&self, topic: &[f64]) -> Result<Vec<f64>> {
        // Direct attention to topic
        {
            let mut attention = self.attention.write();
            attention.focus_on(topic)?;
        }

        // Process through consciousness
        let conscious = {
            let mut consciousness = self.consciousness.write();
            consciousness.deliberate(topic)?
        };

        // Meta-cognitive reflection
        let reflected = {
            let mut self_awareness = self.self_awareness.write();
            self_awareness.think_about(&conscious)?
        };

        Ok(reflected)
    }

    /// Recall a memory
    pub fn recall(&self, cue: &[f64]) -> Result<Option<Vec<f64>>> {
        let memory = self.memory.read();
        memory.retrieve(cue)
    }

    /// Store a memory
    pub fn remember(&self, content: &[f64], importance: f64) -> Result<()> {
        let mut memory = self.memory.write();
        memory.encode(content, importance)
    }

    /// Get current consciousness level
    pub fn consciousness_level(&self) -> f64 {
        let consciousness = self.consciousness.read();
        consciousness.consciousness_level()
    }

    /// Get IIT Phi value
    pub fn phi(&self) -> f64 {
        let consciousness = self.consciousness.read();
        consciousness.phi()
    }

    /// Check if brain is dreaming
    pub fn is_dreaming(&self) -> bool {
        let sleep = self.sleep.read();
        sleep.is_rem()
    }

    /// Get self-model state
    pub fn self_state(&self) -> Vec<f64> {
        let self_awareness = self.self_awareness.read();
        self_awareness.current_self_state()
    }

    /// Activate the brain
    pub fn activate(&self) {
        let mut is_active = self.is_active.write();
        *is_active = true;
    }

    /// Deactivate the brain
    pub fn deactivate(&self) {
        let mut is_active = self.is_active.write();
        *is_active = false;
    }

    /// Check if brain is active
    pub fn is_active(&self) -> bool {
        *self.is_active.read()
    }

    /// Reset the brain to initial state
    pub fn reset(&self) {
        {
            let mut neural = self.neural.write();
            neural.reset();
        }
        {
            let mut attention = self.attention.write();
            attention.reset();
        }
        {
            let mut consciousness = self.consciousness.write();
            consciousness.reset();
        }
        {
            let mut memory = self.memory.write();
            memory.reset();
        }
        {
            let mut sleep = self.sleep.write();
            sleep.reset();
        }
        {
            let mut self_awareness = self.self_awareness.write();
            self_awareness.reset();
        }
        {
            let mut cycle = self.cognitive_cycle.write();
            cycle.reset();
        }
        {
            let mut count = self.cycle_count.write();
            *count = 0;
        }
    }

    fn now(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }
}

impl Default for OmegaBrain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brain_creation() {
        let brain = OmegaBrain::new();
        assert!(brain.is_active());
        assert_eq!(*brain.cycle_count.read(), 0);
    }

    #[test]
    fn test_brain_processing() {
        let brain = OmegaBrain::new();
        let input = vec![0.5; 32];

        let result = brain.process(&input);
        assert!(result.is_ok());

        let metrics = brain.metrics();
        assert_eq!(metrics.cycles, 1);
    }

    #[test]
    fn test_brain_state() {
        let brain = OmegaBrain::new();
        let state = brain.state();

        assert!(state.consciousness_level >= 0.0 && state.consciousness_level <= 1.0);
    }

    #[test]
    fn test_brain_reset() {
        let brain = OmegaBrain::new();

        // Process some input
        let input = vec![0.5; 32];
        brain.process(&input).unwrap();

        // Reset
        brain.reset();

        let metrics = brain.metrics();
        assert_eq!(metrics.cycles, 0);
    }
}
