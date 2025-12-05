//! Intelligence factory for creating and evolving intelligences

use serde::{Deserialize, Serialize};
use thiserror::Error;
use omega_core::{Architecture, Intelligence, Paradigm, SubstrateType};

use crate::{
    search::{MCTS, MCTSConfig},
    optimization::{PPOOptimizer, PPOConfig, Trajectory, Experience},
    fitness::FitnessEvaluator,
    architecture::ArchitectureState,
};
use ndarray::Array1;

#[derive(Error, Debug)]
pub enum FactoryError {
    #[error("Creation failed: {0}")]
    CreationFailed(String),

    #[error("Evolution failed: {0}")]
    EvolutionFailed(String),

    #[error("Invalid specification: {0}")]
    InvalidSpecification(String),
}

/// Specification for creating an intelligence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligenceSpec {
    pub name: String,
    pub paradigm: Option<Paradigm>,
    pub substrate: Option<SubstrateType>,
    pub min_capability: f64,
    pub max_generations: usize,
}

impl Default for IntelligenceSpec {
    fn default() -> Self {
        Self {
            name: "Generated Intelligence".to_string(),
            paradigm: Some(Paradigm::Neural),
            substrate: Some(SubstrateType::Digital),
            min_capability: 0.7,
            max_generations: 10,
        }
    }
}

/// Factory for creating and evolving intelligences
pub struct IntelligenceFactory {
    mcts: MCTS,
    ppo: PPOOptimizer,
    evaluator: FitnessEvaluator,
}

impl IntelligenceFactory {
    pub fn new() -> Self {
        let mcts_config = MCTSConfig::default();
        let ppo_config = PPOConfig::default();

        let mcts = MCTS::new(mcts_config);
        // PPO state_dim = 4096 (architecture encoding), action_dim = 128 (hyperparameter choices)
        let ppo = PPOOptimizer::new(ppo_config, 4096, 128);
        let evaluator = FitnessEvaluator::new();

        Self { mcts, ppo, evaluator }
    }

    pub fn with_configs(mcts_config: MCTSConfig, ppo_config: PPOConfig) -> Self {
        let mcts = MCTS::new(mcts_config);
        // PPO state_dim = 4096 (architecture encoding), action_dim = 128 (hyperparameter choices)
        let ppo = PPOOptimizer::new(ppo_config, 4096, 128);
        let evaluator = FitnessEvaluator::new();

        Self { mcts, ppo, evaluator }
    }

    /// Create a new intelligence from specification
    pub async fn create_intelligence(
        &mut self,
        specification: IntelligenceSpec,
    ) -> Result<Intelligence, FactoryError> {
        tracing::info!("Creating intelligence: {}", specification.name);

        // Phase 1: Use MCTS to search for architecture
        let initial_state = ArchitectureState::new();
        let mut architecture = self.mcts.search(initial_state).await
            .map_err(|e| FactoryError::CreationFailed(e.to_string()))?;

        // Apply specification constraints
        if let Some(paradigm) = specification.paradigm {
            architecture.paradigm = paradigm;
        }
        if let Some(substrate) = specification.substrate {
            architecture.substrate = substrate;
        }
        architecture.name = specification.name.clone();

        // Phase 2: Evaluate fitness
        let fitness = self.evaluator.evaluate(&architecture).await
            .map_err(|e| FactoryError::CreationFailed(e.to_string()))?;

        architecture.fitness = Some(fitness.clone());

        // Phase 3: Optimize if needed
        if fitness.capability < specification.min_capability {
            tracing::info!(
                "Initial capability {:.2} below threshold {:.2}, optimizing...",
                fitness.capability,
                specification.min_capability
            );

            architecture = self.optimize_architecture(architecture).await?;
        }

        // Create intelligence instance
        let intelligence = Intelligence::new(specification.name, architecture);

        tracing::info!(
            "Created intelligence {} with fitness {:.2}",
            intelligence.id,
            intelligence.architecture.fitness.as_ref().unwrap().overall
        );

        Ok(intelligence)
    }

    /// Evolve an existing architecture through multiple generations
    pub async fn evolve_architecture(
        &mut self,
        base: Architecture,
        generations: usize,
    ) -> Result<Architecture, FactoryError> {
        tracing::info!(
            "Evolving architecture {} for {} generations",
            base.id,
            generations
        );

        let mut current = base;

        for generation in 0..generations {
            tracing::debug!("Generation {}/{}", generation + 1, generations);

            // Optimize current architecture
            current = self.optimize_architecture(current).await?;

            // Evaluate fitness
            let fitness = self.evaluator.evaluate(&current).await
                .map_err(|e| FactoryError::EvolutionFailed(e.to_string()))?;

            current.fitness = Some(fitness);

            // Update lineage
            let parent_id = current.id.clone();
            current.id = uuid::Uuid::now_v7().to_string();
            current.lineage.push(parent_id);
        }

        tracing::info!(
            "Evolved architecture to {} with fitness {:.2}",
            current.id,
            current.fitness.as_ref().unwrap().overall
        );

        Ok(current)
    }

    /// Optimize architecture using PPO
    async fn optimize_architecture(
        &mut self,
        architecture: Architecture,
    ) -> Result<Architecture, FactoryError> {
        // Generate training trajectories
        let trajectories = self.generate_trajectories(&architecture, 10).await?;

        // Optimize using PPO
        let result = self.ppo.optimize(&trajectories).await
            .map_err(|e| FactoryError::EvolutionFailed(e.to_string()))?;

        tracing::debug!(
            "PPO optimization: policy_loss={:.4}, value_loss={:.4}, entropy={:.4}",
            result.policy_loss,
            result.value_loss,
            result.entropy
        );

        // Return optimized architecture (simplified - would apply learned changes)
        Ok(architecture)
    }

    /// Generate training trajectories for PPO
    async fn generate_trajectories(
        &self,
        _architecture: &Architecture,
        num_trajectories: usize,
    ) -> Result<Vec<Trajectory>, FactoryError> {
        let mut trajectories = Vec::new();

        for _ in 0..num_trajectories {
            let mut trajectory = Trajectory::new();

            // Generate random trajectory (simplified)
            for i in 0..20 {
                let state = Array1::from_vec(vec![i as f64 / 20.0; 4096]);
                let next_state = Array1::from_vec(vec![(i + 1) as f64 / 20.0; 4096]);

                trajectory.add(Experience {
                    state,
                    action: rand::random::<usize>() % 128,
                    reward: rand::random::<f64>(),
                    next_state,
                    done: i == 19,
                    log_prob: -1.0,
                    value: 0.5,
                });
            }

            trajectories.push(trajectory);
        }

        Ok(trajectories)
    }
}

impl Default for IntelligenceFactory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_intelligence() {
        let mut factory = IntelligenceFactory::new();
        let spec = IntelligenceSpec::default();

        let result = factory.create_intelligence(spec).await;
        assert!(result.is_ok());

        let intelligence = result.unwrap();
        assert!(intelligence.architecture.fitness.is_some());
    }

    #[tokio::test]
    async fn test_evolve_architecture() {
        let mut factory = IntelligenceFactory::new();

        let base_arch = Architecture {
            id: "base".to_string(),
            name: "Base Architecture".to_string(),
            paradigm: Paradigm::Neural,
            substrate: SubstrateType::Digital,
            fitness: None,
            lineage: vec![],
            created_at: chrono::Utc::now(),
        };

        let result = factory.evolve_architecture(base_arch, 3).await;
        assert!(result.is_ok());

        let evolved = result.unwrap();
        assert_eq!(evolved.lineage.len(), 3);
        assert!(evolved.fitness.is_some());
    }
}
