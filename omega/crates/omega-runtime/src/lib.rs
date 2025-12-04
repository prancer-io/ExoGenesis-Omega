//! # Omega Runtime
//!
//! Integrated runtime environment for the ExoGenesis Omega system.
//! Coordinates all subsystems including AgentDB, Memory, Loops, and Meta-SONA.

use omega_core::OmegaComponent;
use omega_agentdb::AgentDB;
use omega_memory::MemorySystem;
use omega_loops::LoopEngine;
use omega_meta_sona::MetaSONA;

/// Main runtime orchestrator
pub struct OmegaRuntime {
    pub agentdb: AgentDB,
    pub memory: MemorySystem,
    pub loops: LoopEngine,
    pub meta_sona: MetaSONA,
}

impl OmegaRuntime {
    /// Create a new OmegaRuntime instance with all subsystems
    pub fn new() -> Self {
        Self {
            agentdb: AgentDB::new(),
            memory: MemorySystem::new(),
            loops: LoopEngine::new(),
            meta_sona: MetaSONA::new(),
        }
    }

    /// Initialize all subsystems
    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.agentdb.initialize().await?;
        self.memory.initialize().await?;
        self.loops.initialize().await?;
        self.meta_sona.initialize().await?;
        Ok(())
    }

    /// Shutdown all subsystems gracefully
    pub async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.meta_sona.shutdown().await?;
        self.loops.shutdown().await?;
        self.memory.shutdown().await?;
        self.agentdb.shutdown().await?;
        Ok(())
    }
}
