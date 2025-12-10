//! Brain Configuration
//!
//! Unified configuration for all brain components.

use serde::{Deserialize, Serialize};

/// Brain operating mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrainMode {
    /// Normal waking cognition
    Awake,
    /// Focused attention mode
    Focused,
    /// Creative/divergent thinking
    Creative,
    /// Light sleep (N1-N2)
    LightSleep,
    /// Deep sleep (N3 SWS)
    DeepSleep,
    /// REM dreaming
    Dreaming,
    /// Low power mode
    Idle,
}

impl Default for BrainMode {
    fn default() -> Self {
        Self::Awake
    }
}

impl std::fmt::Display for BrainMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BrainMode::Awake => write!(f, "Awake"),
            BrainMode::Focused => write!(f, "Focused"),
            BrainMode::Creative => write!(f, "Creative"),
            BrainMode::LightSleep => write!(f, "LightSleep"),
            BrainMode::DeepSleep => write!(f, "DeepSleep"),
            BrainMode::Dreaming => write!(f, "Dreaming"),
            BrainMode::Idle => write!(f, "Idle"),
        }
    }
}

/// Configuration for the Omega Brain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainConfig {
    // === Neural Substrate ===
    /// Number of neurons in the spiking network
    pub neuron_count: usize,
    /// Input dimension
    pub input_dim: usize,
    /// Hidden dimension
    pub hidden_dim: usize,
    /// Output dimension
    pub output_dim: usize,
    /// STDP learning rate
    pub stdp_learning_rate: f64,
    /// Enable neuromodulation
    pub neuromodulation_enabled: bool,

    // === Attention ===
    /// Number of attention heads
    pub attention_heads: usize,
    /// Attention dimension
    pub attention_dim: usize,
    /// Top-down attention strength
    pub top_down_strength: f64,
    /// Bottom-up attention strength
    pub bottom_up_strength: f64,

    // === Consciousness ===
    /// IIT integration threshold
    pub phi_threshold: f64,
    /// GWT broadcast threshold
    pub broadcast_threshold: f64,
    /// Free energy precision
    pub precision: f64,
    /// Number of workspace slots
    pub workspace_capacity: usize,

    // === Memory ===
    /// Hippocampal pattern dimension
    pub pattern_dim: usize,
    /// CA3 network size
    pub ca3_size: usize,
    /// Memory consolidation threshold
    pub consolidation_threshold: f64,
    /// Maximum memories to retain
    pub max_memories: usize,
    /// Replay buffer size
    pub replay_buffer_size: usize,

    // === Sleep ===
    /// Sleep cycle duration (in processing cycles)
    pub sleep_cycle_length: usize,
    /// SWS duration ratio (0-1)
    pub sws_ratio: f64,
    /// REM duration ratio (0-1)
    pub rem_ratio: f64,
    /// Enable automatic sleep
    pub auto_sleep_enabled: bool,
    /// Cycles before auto-sleep
    pub cycles_before_sleep: u64,

    // === Self-Awareness ===
    /// Number of meta-cognitive levels
    pub meta_levels: usize,
    /// Maximum recursion depth
    pub max_recursion: usize,
    /// Self-model update rate
    pub self_update_rate: f64,
    /// Mirror reflection depth
    pub mirror_depth: usize,

    // === Processing ===
    /// Processing batch size
    pub batch_size: usize,
    /// Enable parallel processing
    pub parallel_enabled: bool,
    /// Default brain mode
    pub default_mode: BrainMode,
}

impl Default for BrainConfig {
    fn default() -> Self {
        Self {
            // Neural
            neuron_count: 1000,
            input_dim: 64,
            hidden_dim: 256,
            output_dim: 64,
            stdp_learning_rate: 0.01,
            neuromodulation_enabled: true,

            // Attention
            attention_heads: 8,
            attention_dim: 64,
            top_down_strength: 0.6,
            bottom_up_strength: 0.4,

            // Consciousness
            phi_threshold: 0.3,
            broadcast_threshold: 0.5,
            precision: 1.0,
            workspace_capacity: 7,

            // Memory
            pattern_dim: 128,
            ca3_size: 500,
            consolidation_threshold: 0.7,
            max_memories: 10000,
            replay_buffer_size: 1000,

            // Sleep
            sleep_cycle_length: 1000,
            sws_ratio: 0.6,
            rem_ratio: 0.25,
            auto_sleep_enabled: true,
            cycles_before_sleep: 10000,

            // Self-awareness
            meta_levels: 5,
            max_recursion: 7,
            self_update_rate: 0.1,
            mirror_depth: 3,

            // Processing
            batch_size: 32,
            parallel_enabled: true,
            default_mode: BrainMode::Awake,
        }
    }
}

impl BrainConfig {
    /// Create a minimal configuration (for testing)
    pub fn minimal() -> Self {
        Self {
            neuron_count: 100,
            input_dim: 16,
            hidden_dim: 32,
            output_dim: 16,
            attention_heads: 2,
            attention_dim: 16,
            pattern_dim: 32,
            ca3_size: 50,
            meta_levels: 3,
            max_recursion: 3,
            mirror_depth: 2,
            batch_size: 8,
            max_memories: 100,
            replay_buffer_size: 50,
            ..Default::default()
        }
    }

    /// Create a high-capacity configuration
    pub fn high_capacity() -> Self {
        Self {
            neuron_count: 10000,
            input_dim: 256,
            hidden_dim: 1024,
            output_dim: 256,
            attention_heads: 16,
            attention_dim: 256,
            pattern_dim: 512,
            ca3_size: 2000,
            meta_levels: 7,
            max_recursion: 10,
            mirror_depth: 5,
            max_memories: 100000,
            replay_buffer_size: 10000,
            ..Default::default()
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.input_dim == 0 {
            return Err("input_dim must be > 0".to_string());
        }
        if self.neuron_count == 0 {
            return Err("neuron_count must be > 0".to_string());
        }
        if self.meta_levels == 0 {
            return Err("meta_levels must be > 0".to_string());
        }
        if self.phi_threshold < 0.0 || self.phi_threshold > 1.0 {
            return Err("phi_threshold must be in [0, 1]".to_string());
        }
        if self.sws_ratio + self.rem_ratio > 1.0 {
            return Err("sws_ratio + rem_ratio must be <= 1".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = BrainConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_minimal_config() {
        let config = BrainConfig::minimal();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_brain_mode_display() {
        assert_eq!(BrainMode::Awake.to_string(), "Awake");
        assert_eq!(BrainMode::Dreaming.to_string(), "Dreaming");
    }
}
