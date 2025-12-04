//! # Omega Loops
//!
//! Temporal loop system for ExoGenesis Omega implementing the 7-layer
//! temporal hierarchy from milliseconds (Reflexive) to decades (Transcendent).
//!
//! ## Architecture
//!
//! The system consists of 7 nested temporal loops operating at different timescales:
//!
//! - Loop 1: Reflexive (100ms) - Immediate sensory-motor feedback
//! - Loop 2: Reactive (5s) - Quick decision-making
//! - Loop 3: Adaptive (30min) - Learning from recent experiences
//! - Loop 4: Deliberative (24h) - Strategic planning
//! - Loop 5: Evolutionary (7d) - Systematic improvement
//! - Loop 6: Transformative (1y) - Fundamental changes
//! - Loop 7: Transcendent (10y) - Paradigm shifts
//!
//! ## Example
//!
//! ```rust,no_run
//! use omega_loops::LoopEngine;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut engine = LoopEngine::new();
//!     engine.initialize().await?;
//!
//!     // Engine coordinates all 7 temporal loops
//!
//!     engine.shutdown().await?;
//!     Ok(())
//! }
//! ```

pub mod coordinator;
pub mod executor;
pub mod processors;

use omega_core::{
    LoopType, CycleInput, CycleOutput, LoopManager,
};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug};

pub use coordinator::LoopCoordinator;
pub use executor::LoopExecutor;

/// Main loop engine that coordinates all 7 temporal loops
pub struct LoopEngine {
    coordinator: Arc<RwLock<LoopCoordinator>>,
    executors: HashMap<LoopType, LoopExecutor>,
    running: Arc<RwLock<bool>>,
}

impl LoopEngine {
    /// Create a new loop engine instance
    pub fn new() -> Self {
        info!("Initializing Omega Loop Engine");

        let coordinator = Arc::new(RwLock::new(LoopCoordinator::new()));

        // Create executors for all 7 loop types
        let mut executors = HashMap::new();
        for loop_type in LoopType::all_loops() {
            executors.insert(loop_type, LoopExecutor::new(loop_type));
        }

        Self {
            coordinator,
            executors,
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Initialize the loop engine and all temporal loops
    pub async fn initialize(&mut self) -> Result<(), Box<dyn Error>> {
        info!("Starting Loop Engine initialization");

        let mut coord = self.coordinator.write().await;

        // Create all 7 temporal loops
        for loop_type in LoopType::all_loops() {
            let name = format!("{:?} Loop", loop_type);
            let description = loop_type.description().to_string();

            coord.create_loop(loop_type, name, description).await?;
            debug!("Created {:?} loop", loop_type);
        }

        *self.running.write().await = true;
        info!("Loop Engine initialized with {} loops", LoopType::all_loops().len());

        Ok(())
    }

    /// Shutdown the loop engine gracefully
    pub async fn shutdown(&mut self) -> Result<(), Box<dyn Error>> {
        info!("Shutting down Loop Engine");

        *self.running.write().await = false;

        // Stop all executors
        for (loop_type, executor) in &mut self.executors {
            executor.stop().await?;
            debug!("Stopped {:?} executor", loop_type);
        }

        info!("Loop Engine shutdown complete");
        Ok(())
    }

    /// Check if the engine is running
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }

    /// Execute a cycle in a specific loop
    pub async fn execute_cycle(
        &mut self,
        loop_type: LoopType,
        input: CycleInput,
    ) -> Result<CycleOutput, Box<dyn Error>> {
        if let Some(executor) = self.executors.get_mut(&loop_type) {
            let coord = self.coordinator.clone();
            executor.execute_cycle(coord, input).await
        } else {
            Err(format!("No executor found for loop type {:?}", loop_type).into())
        }
    }

    /// Get the coordinator for direct loop management
    pub fn coordinator(&self) -> Arc<RwLock<LoopCoordinator>> {
        self.coordinator.clone()
    }

    /// Get statistics for all loops
    pub async fn get_stats(&self) -> HashMap<LoopType, LoopStats> {
        let mut stats = HashMap::new();
        let coord = self.coordinator.read().await;

        for loop_type in LoopType::all_loops() {
            if let Ok(Some(loop_data)) = coord.get_loop_by_type(loop_type).await {
                stats.insert(loop_type, LoopStats {
                    cycles_completed: loop_data.metrics.cycles_completed,
                    success_rate: loop_data.metrics.success_rate,
                    average_cycle_time: loop_data.metrics.average_cycle_time,
                });
            }
        }

        stats
    }
}

impl Default for LoopEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics for a temporal loop
#[derive(Debug, Clone)]
pub struct LoopStats {
    pub cycles_completed: u64,
    pub success_rate: f64,
    pub average_cycle_time: chrono::Duration,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_loop_engine_initialization() {
        let mut engine = LoopEngine::new();
        assert!(!engine.is_running().await);

        engine.initialize().await.unwrap();
        assert!(engine.is_running().await);

        engine.shutdown().await.unwrap();
        assert!(!engine.is_running().await);
    }

    #[tokio::test]
    async fn test_all_loops_created() {
        let mut engine = LoopEngine::new();
        engine.initialize().await.unwrap();

        let stats = engine.get_stats().await;
        assert_eq!(stats.len(), 7);

        engine.shutdown().await.unwrap();
    }
}
