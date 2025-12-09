//! # Digital Twin Social Media Platform
//!
//! This example demonstrates how to use ExoGenesis Omega to build
//! a sophisticated digital twin system for social applications.
//!
//! ## Core Components
//!
//! - **Personality Engine**: 4096-dimensional personality vectors using AgentDB
//! - **Emotional Loops**: 7 temporal loops for processing emotional states
//! - **Relationship Graph**: Causal reasoning for compatibility prediction
//! - **Multi-Agent ARIA**: Coherent AI presence through agent orchestration
//!
//! ## Architecture Overview
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    DIGITAL TWIN PLATFORM                     │
//! ├─────────────────────────────────────────────────────────────┤
//! │                                                              │
//! │  User Input ──► Emotional Loops ──► Personality Update      │
//! │       │              │                    │                  │
//! │       ▼              ▼                    ▼                  │
//! │  Sensors ───► AgentDB Vectors ───► Matching Engine          │
//! │       │              │                    │                  │
//! │       ▼              ▼                    ▼                  │
//! │  Wearables ─► 12-Tier Memory ───► ARIA Response             │
//! │                                                              │
//! └─────────────────────────────────────────────────────────────┘
//! ```

pub mod types;
pub mod personality;
pub mod emotional;
pub mod matching;
pub mod aria;
pub mod privacy;
pub mod sensors;

// Re-exports for convenience
pub use types::*;
pub use personality::PersonalityEngine;
pub use emotional::EmotionalLoopProcessor;
pub use matching::MatchingEngine;
pub use aria::ARIASwarm;
pub use privacy::ZeroKnowledgeLayer;
pub use sensors::{KeyboardSensor, WearableSensor};
