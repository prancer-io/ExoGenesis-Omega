//! Integration tests for the Omega Temporal Loops system

#[cfg(test)]
mod integration_tests {
    use crate::{
        LoopCoordinator, OmegaInput, LoopId,
        loops::*,
    };

    #[tokio::test]
    async fn test_full_loop_hierarchy() {
        // Create coordinator
        let mut coordinator = LoopCoordinator::new();

        // Register all 7 loops
        coordinator.register_loop(Box::new(QuantumLoop::new()));
        coordinator.register_loop(Box::new(NeuralLoop::new()));
        coordinator.register_loop(Box::new(CognitiveLoop::new()));
        coordinator.register_loop(Box::new(LearningLoop::new()));
        coordinator.register_loop(Box::new(DevelopmentalLoop::new()));
        coordinator.register_loop(Box::new(EvolutionaryLoop::new()));
        coordinator.register_loop(Box::new(CosmicLoop::new()));

        // Start coordinator
        coordinator.start().await.unwrap();
        assert!(coordinator.is_running().await);

        // Process input
        let input = OmegaInput::new("Test sensory input");
        let output = coordinator.process(input).await.unwrap();

        // Should have results from multiple loops
        assert!(!output.results.is_empty());

        // Stop coordinator
        coordinator.stop().await.unwrap();
        assert!(!coordinator.is_running().await);
    }

    #[tokio::test]
    async fn test_tick_all_loops() {
        let mut coordinator = LoopCoordinator::new();

        // Register first 4 loops
        coordinator.register_loop(Box::new(QuantumLoop::new()));
        coordinator.register_loop(Box::new(NeuralLoop::new()));
        coordinator.register_loop(Box::new(CognitiveLoop::new()));
        coordinator.register_loop(Box::new(LearningLoop::new()));

        coordinator.start().await.unwrap();

        // Tick all loops
        let results = coordinator.tick_all().await.unwrap();

        // Should have tick results from all registered loops
        assert_eq!(results.len(), 4);

        coordinator.stop().await.unwrap();
    }

    #[tokio::test]
    async fn test_targeted_loop_processing() {
        let mut coordinator = LoopCoordinator::new();

        coordinator.register_loop(Box::new(QuantumLoop::new()));
        coordinator.register_loop(Box::new(NeuralLoop::new()));
        coordinator.register_loop(Box::new(CognitiveLoop::new()));

        coordinator.start().await.unwrap();

        // Target specific loop
        let input = OmegaInput::new("Direct cognitive input")
            .with_target(LoopId::Cognitive);

        let output = coordinator.process(input).await.unwrap();

        // Should have processed through cognitive loop
        assert!(!output.results.is_empty());

        coordinator.stop().await.unwrap();
    }
}
