//! # Omega Meta-SONA
//!
//! Self-Organizing Neural Architecture (META-SONA) - The intelligence design engine
//! for ExoGenesis Omega.
//!
//! ## Overview
//!
//! META-SONA is the component that enables ExoGenesis Omega to design new cognitive
//! architectures. While SONA optimizes weights within a fixed architecture, META-SONA
//! optimizes the architecture itself.
//!
//! ## Features
//!
//! - **Architecture Search**: Monte Carlo Tree Search (MCTS) for exploring architecture space
//! - **Hyperparameter Optimization**: Proximal Policy Optimization (PPO) for tuning
//! - **Multi-Objective Fitness**: Evaluation across capability, efficiency, alignment, and novelty
//! - **Intelligence Factory**: Create and evolve intelligences from specifications
//!
//! ## Architecture
//!
//! ```text
//!                    ┌─────────────────────┐
//!                    │   META-SONA         │
//!                    └──────────┬──────────┘
//!                               │
//!          ┌────────────────────┼────────────────────┐
//!          │                    │                    │
//!          ▼                    ▼                    ▼
//! ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
//! │ Architecture    │  │ Intelligence    │  │  Fitness        │
//! │ Search (MCTS)   │  │ Factory         │  │  Evaluation     │
//! └─────────────────┘  └─────────────────┘  └─────────────────┘
//!          │                    │                    │
//!          ▼                    ▼                    ▼
//! ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
//! │ PPO             │  │ Architecture    │  │  Benchmarks     │
//! │ Optimization    │  │ Space           │  │  & Metrics      │
//! └─────────────────┘  └─────────────────┘  └─────────────────┘
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use omega_meta_sona::{IntelligenceFactory, IntelligenceSpec};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create factory
//!     let mut factory = IntelligenceFactory::new();
//!
//!     // Define specification
//!     let spec = IntelligenceSpec {
//!         name: "My AI".to_string(),
//!         min_capability: 0.8,
//!         ..Default::default()
//!     };
//!
//!     // Create intelligence
//!     let intelligence = factory.create_intelligence(spec).await?;
//!
//!     println!("Created: {} with fitness {:.2}",
//!         intelligence.name,
//!         intelligence.architecture.fitness.unwrap().overall
//!     );
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Components
//!
//! ### Architecture Module
//!
//! Defines how architectures are represented and encoded:
//! - `ArchitectureSpace`: The space of all possible architectures
//! - `ArchitectureEncoding`: Vector representations for optimization
//! - `ComputationalGraph`: Graph structure of neural architectures
//!
//! ### Search Module
//!
//! Algorithms for discovering architectures:
//! - `MCTS`: Monte Carlo Tree Search with UCB1 selection
//! - Parallel simulations and rollouts
//! - Architecture-specific exploration bonuses
//!
//! ### Optimization Module
//!
//! Refinement of discovered architectures:
//! - `PPOOptimizer`: Proximal Policy Optimization
//! - Generalized Advantage Estimation (GAE)
//! - Clipped surrogate objectives
//!
//! ### Fitness Module
//!
//! Multi-objective evaluation:
//! - Capability evaluation (benchmarks)
//! - Efficiency evaluation (resource usage)
//! - Alignment evaluation (safety tests)
//! - Novelty evaluation (innovation metrics)
//!
//! ### Factory Module
//!
//! Intelligence creation and evolution:
//! - `IntelligenceFactory`: Main API for creating intelligences
//! - Specification-based creation
//! - Multi-generation evolution

pub mod architecture;
pub mod search;
pub mod optimization;
pub mod fitness;
pub mod factory;

// Re-export main types
pub use architecture::{
    ArchitectureSpace, ArchitectureEncoding, ArchitectureNode,
    NodeType, Parameters, Connection, ComputationalGraph,
};
pub use search::{MCTS, MCTSConfig, MCTSError};
pub use optimization::{PPOOptimizer, PPOConfig, Trajectory, OptimizationResult};
pub use fitness::{FitnessEvaluator, MetricWeight, EvaluationError};
pub use factory::{IntelligenceFactory, IntelligenceSpec, FactoryError};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Result type for META-SONA operations
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Meta-SONA orchestrator - main entry point
pub struct MetaSONA {
    factory: IntelligenceFactory,
}

impl MetaSONA {
    /// Create a new MetaSONA instance
    pub fn new() -> Self {
        Self {
            factory: IntelligenceFactory::new(),
        }
    }

    /// Create a new intelligence from specification
    pub async fn create_intelligence(&mut self, spec: IntelligenceSpec) -> Result<omega_core::Intelligence> {
        self.factory.create_intelligence(spec).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Evolve an architecture through multiple generations
    pub async fn evolve_architecture(
        &mut self,
        base: omega_core::Architecture,
        generations: usize,
    ) -> Result<omega_core::Architecture> {
        self.factory.evolve_architecture(base, generations).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Get the intelligence factory
    pub fn factory(&self) -> &IntelligenceFactory {
        &self.factory
    }
}

impl Default for MetaSONA {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_meta_sona_creation() {
        let mut meta_sona = MetaSONA::new();
        let spec = IntelligenceSpec::default();

        let result = meta_sona.create_intelligence(spec).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_meta_sona_evolution() {
        let mut meta_sona = MetaSONA::new();

        let base_arch = omega_core::Architecture {
            id: "base".to_string(),
            name: "Base".to_string(),
            paradigm: omega_core::Paradigm::Neural,
            substrate: omega_core::SubstrateType::Digital,
            fitness: None,
            lineage: vec![],
            created_at: chrono::Utc::now(),
        };

        let result = meta_sona.evolve_architecture(base_arch, 2).await;
        assert!(result.is_ok());
    }
}
