//! # Omega Runtime
//!
//! Integrated runtime environment for the ExoGenesis Omega system.
//! Coordinates all subsystems including AgentDB, Memory, Loops, and Meta-SONA.
//!
//! ## Overview
//!
//! The Omega Runtime provides a unified orchestration layer for all Omega subsystems:
//!
//! - **AgentDB**: Agent lifecycle and persistence management
//! - **Memory System**: Multi-tier memory with working, short-term, and long-term storage
//! - **Loop Engine**: Cognitive loops (conscious, subconscious, meta, unconscious)
//! - **Meta-SONA**: Self-organizing neural architectures and evolution
//!
//! ## Features
//!
//! - **Event System**: Comprehensive event bus for system-wide notifications
//! - **State Management**: Robust state machine with lifecycle management
//! - **Configuration**: Flexible, validated configuration system
//! - **API**: High-level API for all subsystem operations
//! - **Health Monitoring**: Built-in health checks for all components
//!
//! ## Example
//!
//! ```rust,no_run
//! use omega_runtime::{OmegaRuntime, OmegaConfig, OmegaAPI};
//! use std::sync::Arc;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create runtime with default configuration
//!     let config = OmegaConfig::default();
//!     let runtime = Arc::new(OmegaRuntime::new(config).await?);
//!
//!     // Start all subsystems
//!     runtime.start().await?;
//!
//!     // Create API for high-level operations
//!     let api = OmegaAPI::new(runtime.clone());
//!
//!     // Use the API...
//!
//!     // Shutdown gracefully
//!     runtime.stop().await?;
//!     Ok(())
//! }
//! ```

pub mod api;
pub mod config;
pub mod error;
pub mod events;
pub mod runtime;

#[cfg(test)]
mod tests;

// Re-export main types
pub use api::{
    Architecture, CycleInput, CycleOutput, Intelligence, IntelligenceSpec,
    LoopInfo, LoopStatus, Memory, OmegaAPI, RuntimeMetrics,
};
pub use config::{
    AgentDBConfig, LoopsConfig, MemoryConfig, MetaSONAConfig, OmegaConfig,
};
pub use error::{APIError, APIResult, ConfigError, ConfigResult, RuntimeError, RuntimeResult};
pub use events::{
    ArchitectureId, EventBus, EventHandler, IntelligenceId, LoopType,
    MemoryTier, OmegaEvent,
};
pub use runtime::{OmegaRuntime, RuntimeHealth, RuntimeState};
