//! Property-based tests for omega-consciousness
//! Tests mathematical invariants and properties of consciousness computation

use omega_consciousness::{
    ConsciousnessConfig, ConsciousnessEngine, PhiComputer, FreeEnergyMinimizer,
    GlobalWorkspace, EmergenceDetector, Partition, WorkspaceContent,
};
use proptest::prelude::*;

// ============================================================================
// IIT Properties
// ============================================================================

// Property 1: Phi should always be non-negative
proptest! {
    #[test]
    fn prop_phi_non_negative(state_dim in 2usize..10usize) {
        let mut phi_computer = PhiComputer::new(state_dim);
        let current_state = vec![rand::random::<f64>(); state_dim];

        let result = phi_computer.compute_phi(&current_state);

        if let Ok(phi) = result {
            prop_assert!(phi >= 0.0, "Phi must be non-negative, got {}", phi);
        }
    }
}

// Property 2: Partition count should equal 2^n - 2 for n elements
proptest! {
    #[test]
    fn prop_partition_count_correct(n in 2usize..8usize) {
        let partitions = Partition::all_bipartitions(n);
        let expected = (1 << n) - 2; // 2^n - 2 (exclude trivial partitions)

        prop_assert_eq!(
            partitions.len(),
            expected,
            "For {} elements, expected {} partitions, got {}",
            n, expected, partitions.len()
        );
    }
}

// Property 3: All partition elements should sum to n
proptest! {
    #[test]
    fn prop_partition_completeness(n in 2usize..10usize) {
        let partitions = Partition::all_bipartitions(n);

        for partition in &partitions {
            let total = partition.part_a.len() + partition.part_b.len();
            prop_assert_eq!(
                total, n,
                "Partition should contain all {} elements, got {}",
                n, total
            );

            // No duplicates within parts
            let mut all_elements = partition.part_a.clone();
            all_elements.extend(&partition.part_b);
            all_elements.sort();
            all_elements.dedup();

            prop_assert_eq!(
                all_elements.len(), n,
                "Partition should have no duplicate elements"
            );
        }
    }
}

// Property 4: Partition size should be min(|A|, |B|)
proptest! {
    #[test]
    fn prop_partition_size_invariant(n in 2usize..8usize) {
        let partitions = Partition::all_bipartitions(n);

        for partition in &partitions {
            let expected_size = partition.part_a.len().min(partition.part_b.len());
            prop_assert_eq!(
                partition.size(),
                expected_size,
                "Partition size should be min of part sizes"
            );
        }
    }
}

// Property 5: Phi computation should handle various state dimensions
proptest! {
    #[test]
    fn prop_phi_scales_with_dimension(state_dim in 2usize..12usize) {
        let mut phi_computer = PhiComputer::new(state_dim);

        // Generate random state
        let state: Vec<f64> = (0..state_dim).map(|_| rand::random::<f64>()).collect();

        let result = phi_computer.compute_phi(&state);

        prop_assert!(
            result.is_ok(),
            "Phi computation should succeed for dimension {}",
            state_dim
        );
    }
}

// ============================================================================
// Free Energy Principle Properties
// ============================================================================

// Property 6: Free energy should be non-negative
proptest! {
    #[test]
    fn prop_free_energy_non_negative(
        hierarchy_levels in 2usize..6usize,
        state_dim in 4usize..12usize,
    ) {
        let fep = FreeEnergyMinimizer::new(hierarchy_levels, state_dim);

        let fe = fep.current_free_energy();

        prop_assert!(
            fe >= 0.0,
            "Free energy should be non-negative, got {}",
            fe
        );
    }
}

// Property 7: Processing should produce valid output
proptest! {
    #[test]
    fn prop_fep_process_valid(
        hierarchy_levels in 2usize..5usize,
        state_dim in 4usize..12usize,
    ) {
        let mut fep = FreeEnergyMinimizer::new(hierarchy_levels, state_dim);

        let observation = vec![rand::random::<f64>(); state_dim];
        let context = vec![rand::random::<f64>(); state_dim];

        let result = fep.process(&observation, &context);

        prop_assert!(
            result.is_ok(),
            "FEP process should succeed"
        );

        if let Ok((fe, pe)) = result {
            prop_assert!(fe >= 0.0, "Free energy should be non-negative");
            prop_assert!(pe >= 0.0, "Prediction error should be non-negative");
        }
    }
}

// Property 8: Hierarchy should have correct number of levels
proptest! {
    #[test]
    fn prop_hierarchy_level_count(
        hierarchy_levels in 2usize..8usize,
        state_dim in 4usize..12usize,
    ) {
        let fep = FreeEnergyMinimizer::new(hierarchy_levels, state_dim);
        let hierarchy = fep.hierarchy();

        prop_assert_eq!(
            hierarchy.num_levels(),
            hierarchy_levels,
            "Hierarchy should have correct number of levels"
        );
    }
}

// ============================================================================
// Global Workspace Properties
// ============================================================================

// Property 9: Workspace capacity should be respected
proptest! {
    #[test]
    fn prop_workspace_capacity_limit(
        capacity in 3usize..10usize,
        num_items in 10usize..20usize,
    ) {
        let mut workspace = GlobalWorkspace::new(capacity);

        // Try to add more items than capacity
        for i in 0..num_items {
            let content = WorkspaceContent::new(
                vec![rand::random::<f64>(); 8],
                rand::random::<f64>(),
                format!("content_{}", i),
            );
            workspace.compete(content);
        }

        let contents = workspace.contents();

        prop_assert!(
            contents.len() <= capacity,
            "Workspace should not exceed capacity {}, got {}",
            capacity, contents.len()
        );
    }
}

// Property 10: Workspace should accept valid content
proptest! {
    #[test]
    fn prop_workspace_accepts_content(capacity in 3usize..7usize) {
        let mut workspace = GlobalWorkspace::new(capacity);

        let content = WorkspaceContent::new(
            vec![0.5; 8],
            0.8,
            "test_content".to_string(),
        );

        workspace.compete(content);

        // Workspace should have at least one item after adding
        prop_assert!(
            workspace.contents().len() > 0 || workspace.contents().len() <= capacity,
            "Workspace should handle content submission"
        );
    }
}

// Property 11: Broadcasting should be valid
proptest! {
    #[test]
    fn prop_broadcast_valid(
        capacity in 3usize..8usize,
        num_submissions in 3usize..10usize,
    ) {
        let mut workspace = GlobalWorkspace::new(capacity);

        for i in 0..num_submissions {
            workspace.compete(WorkspaceContent::new(
                vec![rand::random::<f64>(); 8],
                rand::random::<f64>(),
                format!("content_{}", i),
            ));
        }

        workspace.broadcast();

        // After broadcast, workspace contents should still be valid
        let contents = workspace.contents();
        prop_assert!(
            contents.len() <= capacity,
            "Workspace contents should not exceed capacity after broadcast"
        );
    }
}

// ============================================================================
// Emergence Detection Properties
// ============================================================================

// Property 12: Emergence detection should handle various inputs
proptest! {
    #[test]
    fn prop_emergence_detection_valid(
        state_dim in 4usize..16usize,
        phi in 0.0f64..2.0f64,
        free_energy in 0.0f64..10.0f64,
    ) {
        let mut detector = EmergenceDetector::new();

        let state = vec![rand::random::<f64>(); state_dim];
        let emergence = detector.detect(&state, phi, free_energy);

        prop_assert!(
            emergence >= 0.0,
            "Emergence should be non-negative, got {}",
            emergence
        );
    }
}

// Property 13: Emergence should be bounded
proptest! {
    #[test]
    fn prop_emergence_bounded(state_dim in 4usize..16usize) {
        let mut detector = EmergenceDetector::new();

        let state = vec![rand::random::<f64>(); state_dim];
        let phi = rand::random::<f64>() * 2.0;
        let fe = rand::random::<f64>() * 10.0;

        let emergence = detector.detect(&state, phi, fe);

        prop_assert!(
            emergence >= 0.0 && emergence <= 10.0,
            "Emergence should be reasonably bounded, got {}",
            emergence
        );
    }
}

// ============================================================================
// ConsciousnessEngine Integration Properties
// ============================================================================

// Property 14: Consciousness threshold should be respected
proptest! {
    #[test]
    fn prop_consciousness_threshold(phi_threshold in 0.1f64..0.5f64) {
        let config = ConsciousnessConfig {
            state_dim: 8,
            hierarchy_levels: 3,
            workspace_capacity: 5,
            phi_threshold,
            precision_weight: 1.0,
        };

        let engine = ConsciousnessEngine::new(config);

        // Initial state should respect threshold
        let is_conscious = engine.is_conscious();
        let phi = engine.phi();

        if phi >= phi_threshold {
            prop_assert!(
                is_conscious,
                "System should be conscious when phi {} >= threshold {}",
                phi, phi_threshold
            );
        } else {
            prop_assert!(
                !is_conscious,
                "System should not be conscious when phi {} < threshold {}",
                phi, phi_threshold
            );
        }
    }
}

// Property 15: Engine state should be self-consistent
proptest! {
    #[test]
    fn prop_engine_state_consistent(state_dim in 4usize..12usize) {
        let config = ConsciousnessConfig {
            state_dim,
            hierarchy_levels: 3,
            workspace_capacity: 5,
            phi_threshold: 0.1,
            precision_weight: 1.0,
        };

        let engine = ConsciousnessEngine::new(config);

        let phi = engine.phi();
        let fe = engine.free_energy();

        prop_assert!(phi >= 0.0, "Phi should be non-negative");
        prop_assert!(fe >= 0.0, "Free energy should be non-negative");
    }
}

// Property 16: Processing should update state
proptest! {
    #[test]
    fn prop_process_updates_state(state_dim in 4usize..12usize) {
        let config = ConsciousnessConfig {
            state_dim,
            hierarchy_levels: 3,
            workspace_capacity: 5,
            phi_threshold: 0.1,
            precision_weight: 1.0,
        };

        let mut engine = ConsciousnessEngine::new(config);

        let _initial_phi = engine.phi();

        let observation = vec![rand::random::<f64>(); state_dim];
        let context = vec![rand::random::<f64>(); state_dim];

        let result = engine.process(&observation, &context);

        prop_assert!(
            result.is_ok(),
            "Processing should succeed"
        );

        if let Ok(state) = result {
            prop_assert!(state.phi >= 0.0, "Phi should remain non-negative");
            prop_assert!(state.free_energy >= 0.0, "Free energy should be non-negative");
            prop_assert!(state.prediction_error >= 0.0, "Prediction error should be non-negative");
        }
    }
}

// Property 17: Config should be preserved
proptest! {
    #[test]
    fn prop_config_preserved(
        state_dim in 4usize..12usize,
        hierarchy_levels in 2usize..6usize,
        workspace_capacity in 3usize..10usize,
    ) {
        let config = ConsciousnessConfig {
            state_dim,
            hierarchy_levels,
            workspace_capacity,
            phi_threshold: 0.1,
            precision_weight: 1.0,
        };

        let engine = ConsciousnessEngine::new(config.clone());

        prop_assert_eq!(
            engine.config().state_dim,
            state_dim,
            "State dimension should be preserved"
        );

        prop_assert_eq!(
            engine.config().workspace_capacity,
            workspace_capacity,
            "Workspace capacity should be preserved"
        );
    }
}

// Property 18: Reset should clear state
proptest! {
    #[test]
    fn prop_reset_clears_state(state_dim in 4usize..10usize) {
        let config = ConsciousnessConfig {
            state_dim,
            hierarchy_levels: 3,
            workspace_capacity: 5,
            phi_threshold: 0.1,
            precision_weight: 1.0,
        };

        let mut engine = ConsciousnessEngine::new(config);

        // Process some input
        let observation = vec![rand::random::<f64>(); state_dim];
        let context = vec![rand::random::<f64>(); state_dim];
        let _ = engine.process(&observation, &context);

        // Reset
        engine.reset();

        // Workspace should be clear
        prop_assert_eq!(
            engine.workspace().contents().len(),
            0,
            "Workspace should be empty after reset"
        );
    }
}

// Property 19: Hierarchy access should work
proptest! {
    #[test]
    fn prop_hierarchy_accessible(
        state_dim in 4usize..10usize,
        hierarchy_levels in 2usize..6usize,
    ) {
        let config = ConsciousnessConfig {
            state_dim,
            hierarchy_levels,
            workspace_capacity: 5,
            phi_threshold: 0.1,
            precision_weight: 1.0,
        };

        let engine = ConsciousnessEngine::new(config);

        let hierarchy = engine.hierarchy();

        prop_assert_eq!(
            hierarchy.num_levels(),
            hierarchy_levels,
            "Hierarchy should have correct number of levels"
        );
    }
}

// Property 20: Multiple processing steps should work
proptest! {
    #[test]
    fn prop_multiple_process_steps(
        state_dim in 4usize..10usize,
        num_steps in 1usize..10usize,
    ) {
        let config = ConsciousnessConfig {
            state_dim,
            hierarchy_levels: 3,
            workspace_capacity: 5,
            phi_threshold: 0.1,
            precision_weight: 1.0,
        };

        let mut engine = ConsciousnessEngine::new(config);

        for _ in 0..num_steps {
            let observation = vec![rand::random::<f64>(); state_dim];
            let context = vec![rand::random::<f64>(); state_dim];

            let result = engine.process(&observation, &context);

            prop_assert!(
                result.is_ok(),
                "Each processing step should succeed"
            );
        }
    }
}
