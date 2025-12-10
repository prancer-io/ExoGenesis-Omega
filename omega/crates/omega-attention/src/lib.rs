//! # Omega Attention - Brain-Like Selective Processing
//!
//! Implements 39 attention mechanisms inspired by transformer architectures and
//! neuroscience research, plus brain-like attention control systems.
//!
//! ## Features
//!
//! - **39 Attention Mechanisms**: Flash, Linear, Sparse, Hyperbolic, Graph, Memory-augmented
//! - **Top-Down Attention**: Goal-driven, task-relevant selection
//! - **Bottom-Up Attention**: Stimulus-driven, salience-based capture
//! - **Working Memory Gating**: Input/output/forget gates for WM control
//! - **Attention Spotlight**: Winner-take-all competition
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                   ATTENTION SYSTEM                          │
//! ├─────────────────────────────────────────────────────────────┤
//! │                                                              │
//! │  ┌────────────────────┐    ┌────────────────────┐           │
//! │  │   TOP-DOWN         │    │   BOTTOM-UP        │           │
//! │  │   (Goal-driven)    │    │   (Salience)       │           │
//! │  │                    │    │                    │           │
//! │  │  • Task relevance  │    │  • Novelty         │           │
//! │  │  • Expected value  │    │  • Contrast        │           │
//! │  │  • Memory match    │    │  • Motion          │           │
//! │  └────────┬───────────┘    └────────┬───────────┘           │
//! │           │                         │                        │
//! │           └───────────┬─────────────┘                        │
//! │                       ▼                                      │
//! │           ┌───────────────────────┐                         │
//! │           │   ATTENTION CONTROL   │                         │
//! │           │   (Priority Map)      │                         │
//! │           └───────────┬───────────┘                         │
//! │                       ▼                                      │
//! │           ┌───────────────────────┐                         │
//! │           │   ATTENTION MECHANISMS│                         │
//! │           │   (39 types)          │                         │
//! │           └───────────┬───────────┘                         │
//! │                       ▼                                      │
//! │           ┌───────────────────────┐                         │
//! │           │   WORKING MEMORY      │                         │
//! │           │   (Gated Access)      │                         │
//! │           └───────────────────────┘                         │
//! │                                                              │
//! └─────────────────────────────────────────────────────────────┘
//! ```

pub mod mechanisms;
pub mod controller;
pub mod working_memory;
pub mod salience;

pub use mechanisms::{
    AttentionMechanism, AttentionType, AttentionOutput,
    ScaledDotProductAttention, FlashAttention, LinearAttention,
    SparseAttention, HyperbolicAttention, GraphAttention,
    MemoryAugmentedAttention, MultiHeadAttention,
};
pub use controller::{AttentionController, AttentionConfig, PriorityMap};
pub use working_memory::{WorkingMemory, WMGate, WorkingMemoryItem};
pub use salience::{SalienceMap, SalienceComputer, SalienceFeature};

use thiserror::Error;

/// Errors that can occur in the attention module
#[derive(Error, Debug)]
pub enum AttentionError {
    #[error("Invalid dimensions: expected {expected}, got {got}")]
    InvalidDimensions { expected: usize, got: usize },

    #[error("Empty input")]
    EmptyInput,

    #[error("Attention computation failed: {0}")]
    ComputationError(String),

    #[error("Working memory full")]
    WorkingMemoryFull,

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

pub type Result<T> = std::result::Result<T, AttentionError>;

/// Main attention system orchestrating all components
pub struct AttentionSystem {
    controller: AttentionController,
    working_memory: WorkingMemory,
    salience: SalienceComputer,
}

impl AttentionSystem {
    /// Create a new attention system
    pub fn new(config: AttentionConfig) -> Self {
        Self {
            controller: AttentionController::new(config),
            working_memory: WorkingMemory::new(7), // 7±2 capacity
            salience: SalienceComputer::new(),
        }
    }

    /// Process input through attention system
    pub fn attend(
        &mut self,
        input: &[f64],
        goals: &[f64],
        context: &[f64],
    ) -> Result<AttentionOutput> {
        // 1. Compute bottom-up salience
        let salience = self.salience.compute(input);

        // 2. Compute top-down relevance based on goals
        let relevance = self.controller.compute_relevance(input, goals);

        // 3. Combine into priority map
        let priority = self.controller.combine_priorities(&salience, &relevance);

        // 4. Apply attention mechanism
        let output = self.controller.apply_attention(input, &priority, context)?;

        // 5. Update working memory if high priority
        if output.max_attention > 0.5 {
            let item = WorkingMemoryItem::new(
                output.attended_values.clone(),
                output.max_attention,
            );
            self.working_memory.try_store(item);
        }

        Ok(output)
    }

    /// Get current working memory contents
    pub fn working_memory(&self) -> &WorkingMemory {
        &self.working_memory
    }

    /// Get mutable access to working memory
    pub fn working_memory_mut(&mut self) -> &mut WorkingMemory {
        &mut self.working_memory
    }

    /// Focus attention on specific item (top-down control)
    pub fn focus(&mut self, target: &[f64]) {
        self.controller.set_focus(target);
    }

    /// Get current attention state
    pub fn state(&self) -> AttentionState {
        AttentionState {
            wm_items: self.working_memory.len(),
            wm_capacity: self.working_memory.capacity(),
            current_focus: self.controller.current_focus(),
        }
    }
}

/// Current state of attention system
#[derive(Debug, Clone)]
pub struct AttentionState {
    pub wm_items: usize,
    pub wm_capacity: usize,
    pub current_focus: Option<Vec<f64>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attention_system_creation() {
        let config = AttentionConfig::default();
        let system = AttentionSystem::new(config);

        let state = system.state();
        assert_eq!(state.wm_items, 0);
        assert_eq!(state.wm_capacity, 7);
    }
}
