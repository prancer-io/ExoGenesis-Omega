//! # ExoGenesis Omega - Core Types and Traits
//!
//! This crate provides the foundational types and traits for the ExoGenesis Omega
//! universal intelligence orchestration system.
//!
//! ## Overview
//!
//! ExoGenesis Omega enables the orchestration of intelligence at all scales, from
//! milliseconds to cosmic timescales, through:
//!
//! - **12-Tier Cosmic Memory**: Memory systems spanning from immediate (milliseconds)
//!   to cosmic (billions of years) timescales
//! - **7 Temporal Loops**: Multi-scale feedback and learning cycles from reflexive
//!   to transcendent
//! - **Universal Intelligence**: Support for any type of intelligence substrate from
//!   digital to cosmic
//!
//! ## Architecture
//!
//! ### Intelligence Types
//!
//! The [`Intelligence`] type represents any form of intelligence, with support for:
//! - Multiple paradigms (Neural, Symbolic, Quantum, Biological, etc.)
//! - Various substrates (Digital, Biological, Social, Cosmic, etc.)
//! - Dynamic capabilities and evolution
//!
//! ### Memory System
//!
//! The memory system provides 12 tiers of storage:
//! 1. Immediate (milliseconds)
//! 2. Short-term (seconds to minutes)
//! 3. Session (hours)
//! 4. Episodic (days)
//! 5. Semantic (weeks)
//! 6. Procedural (months)
//! 7. Strategic (years)
//! 8. Civilizational (decades to centuries)
//! 9. Evolutionary (millennia)
//! 10. Planetary (millions of years)
//! 11. Galactic (billions of years)
//! 12. Cosmic (age of universe)
//!
//! ### Temporal Loops
//!
//! Seven nested feedback loops enable learning and adaptation:
//! 1. Reflexive (milliseconds)
//! 2. Reactive (seconds)
//! 3. Adaptive (minutes to hours)
//! 4. Deliberative (days)
//! 5. Evolutionary (weeks to months)
//! 6. Transformative (years)
//! 7. Transcendent (decades+)
//!
//! ## Usage
//!
//! ```rust
//! use omega_core::*;
//! use chrono::Utc;
//!
//! // Create an intelligence
//! let architecture = Architecture {
//!     id: "arch-1".to_string(),
//!     name: "Neural Network".to_string(),
//!     paradigm: Paradigm::Neural,
//!     substrate: SubstrateType::Digital,
//!     fitness: None,
//!     lineage: vec![],
//!     created_at: Utc::now(),
//! };
//!
//! let intelligence = Intelligence::new(
//!     "My AI".to_string(),
//!     architecture,
//! );
//!
//! // Create a memory
//! let memory = Memory::new(
//!     MemoryTier::Semantic,
//!     MemoryType::Knowledge,
//!     MemoryContent::Text("Important fact".to_string()),
//!     0.9, // importance
//! );
//!
//! // Create a temporal loop
//! let mut loop_instance = TemporalLoop::new(
//!     LoopType::Adaptive,
//!     "Learning Loop".to_string(),
//!     "Continuous learning from experience".to_string(),
//! );
//! ```

pub mod types;
pub mod traits;

// Re-export all public types and traits
pub use types::*;
pub use traits::*;

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Error types for omega-core
pub mod error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum OmegaError {
        #[error("Intelligence not found: {0}")]
        IntelligenceNotFound(String),

        #[error("Memory not found: {0}")]
        MemoryNotFound(String),

        #[error("Loop not found: {0}")]
        LoopNotFound(String),

        #[error("Invalid tier: {0}")]
        InvalidTier(String),

        #[error("Invalid loop type: {0}")]
        InvalidLoopType(String),

        #[error("Operation failed: {0}")]
        OperationFailed(String),

        #[error("Serialization error: {0}")]
        SerializationError(#[from] serde_json::Error),

        #[error("Unknown error: {0}")]
        Unknown(String),
    }
}

pub use error::OmegaError;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_create_intelligence() {
        let architecture = Architecture {
            id: "test-arch".to_string(),
            name: "Test Architecture".to_string(),
            paradigm: Paradigm::Neural,
            substrate: SubstrateType::Digital,
            fitness: None,
            lineage: vec![],
            created_at: Utc::now(),
        };

        let intelligence = Intelligence::new("Test AI".to_string(), architecture);
        assert_eq!(intelligence.name, "Test AI");
        assert_eq!(intelligence.generation, 0);
        assert_eq!(intelligence.status, IntelligenceStatus::Initializing);
    }

    #[test]
    fn test_memory_tiers() {
        let tiers = MemoryTier::all_tiers();
        assert_eq!(tiers.len(), 12);
        assert_eq!(tiers[0], MemoryTier::Immediate);
        assert_eq!(tiers[11], MemoryTier::Cosmic);
    }

    #[test]
    fn test_create_memory() {
        let memory = Memory::new(
            MemoryTier::Semantic,
            MemoryType::Knowledge,
            MemoryContent::Text("Test memory".to_string()),
            0.8,
        );

        assert_eq!(memory.tier, MemoryTier::Semantic);
        assert!(!memory.is_expired());
        assert_eq!(memory.metadata.importance, 0.8);
    }

    #[test]
    fn test_temporal_loops() {
        let loops = LoopType::all_loops();
        assert_eq!(loops.len(), 7);
        assert_eq!(loops[0], LoopType::Reflexive);
        assert_eq!(loops[6], LoopType::Transcendent);
    }

    #[test]
    fn test_loop_cycle() {
        let mut temporal_loop = TemporalLoop::new(
            LoopType::Adaptive,
            "Test Loop".to_string(),
            "Test Description".to_string(),
        );

        let input = CycleInput {
            data: std::collections::HashMap::new(),
            context: "test".to_string(),
            objectives: vec!["learn".to_string()],
        };

        let cycle_id = temporal_loop.start_cycle(input);
        assert!(!cycle_id.is_empty());
        assert!(temporal_loop.current_cycle.is_some());
    }
}
