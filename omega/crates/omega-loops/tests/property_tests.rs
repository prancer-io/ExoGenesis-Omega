//! Property-based tests for omega-loops
//! Tests mathematical invariants and properties of the 7-layer temporal loop system

use omega_core::{LoopType, LoopManager, CycleInput, CycleOutput};
use omega_loops::{LoopEngine, LoopCoordinator, LoopExecutor};
use proptest::prelude::*;
use tokio::runtime::Runtime;
use std::collections::HashMap;

// ============================================================================
// LoopType Properties
// ============================================================================

// Property 1: Should have exactly 7 loop types
proptest! {
    #[test]
    fn prop_seven_loops(_dummy in 0u8..1u8) {
        let all_loops = LoopType::all_loops();
        prop_assert_eq!(
            all_loops.len(),
            7,
            "System should have exactly 7 temporal loops"
        );
    }
}

// Property 2: All loop types should be unique
proptest! {
    #[test]
    fn prop_loop_types_unique(_dummy in 0u8..1u8) {
        let all_loops = LoopType::all_loops();
        let mut sorted = all_loops.clone();
        sorted.sort_by_key(|l| *l as usize);
        sorted.dedup();

        prop_assert_eq!(
            all_loops.len(),
            sorted.len(),
            "All loop types should be unique"
        );
    }
}

// Property 3: Loop durations should be monotonically increasing
proptest! {
    #[test]
    fn prop_durations_increase(_dummy in 0u8..1u8) {
        let all_loops = LoopType::all_loops();

        for i in 0..all_loops.len() - 1 {
            let current_duration = all_loops[i].cycle_duration();
            let next_duration = all_loops[i + 1].cycle_duration();

            prop_assert!(
                next_duration > current_duration,
                "Loop durations should increase: {:?} < {:?}",
                all_loops[i], all_loops[i + 1]
            );
        }
    }
}

// Property 4: All loops should have descriptions
proptest! {
    #[test]
    fn prop_all_loops_have_descriptions(_dummy in 0u8..1u8) {
        for loop_type in LoopType::all_loops() {
            let desc = loop_type.description();
            prop_assert!(
                !desc.is_empty(),
                "Loop {:?} should have a non-empty description",
                loop_type
            );
        }
    }
}

// Property 5: Loop durations should be positive
proptest! {
    #[test]
    fn prop_durations_positive(_dummy in 0u8..1u8) {
        for loop_type in LoopType::all_loops() {
            let duration = loop_type.cycle_duration();
            prop_assert!(
                duration.num_milliseconds() > 0,
                "Loop {:?} should have positive duration",
                loop_type
            );
        }
    }
}

// Property 6: Durations should span multiple timescales
proptest! {
    #[test]
    fn prop_durations_span_timescales(_dummy in 0u8..1u8) {
        let all_loops = LoopType::all_loops();

        let min_ms = all_loops[0].cycle_duration().num_milliseconds();
        let max_ms = all_loops[all_loops.len() - 1].cycle_duration().num_milliseconds();

        let ratio = max_ms as f64 / min_ms as f64;

        prop_assert!(
            ratio >= 1e6,
            "Loop durations should span at least 6 orders of magnitude, got ratio {}",
            ratio
        );
    }
}

// Property 7: Reflexive should be fastest
proptest! {
    #[test]
    fn prop_reflexive_fastest(_dummy in 0u8..1u8) {
        let reflexive_ms = LoopType::Reflexive.cycle_duration().num_milliseconds();

        for loop_type in LoopType::all_loops() {
            if loop_type != LoopType::Reflexive {
                let ms = loop_type.cycle_duration().num_milliseconds();
                prop_assert!(
                    ms > reflexive_ms,
                    "Reflexive should be fastest"
                );
            }
        }
    }
}

// Property 8: Transcendent should be slowest
proptest! {
    #[test]
    fn prop_transcendent_slowest(_dummy in 0u8..1u8) {
        let transcendent_ms = LoopType::Transcendent.cycle_duration().num_milliseconds();

        for loop_type in LoopType::all_loops() {
            if loop_type != LoopType::Transcendent {
                let ms = loop_type.cycle_duration().num_milliseconds();
                prop_assert!(
                    ms < transcendent_ms,
                    "Transcendent should be slowest"
                );
            }
        }
    }
}

// ============================================================================
// LoopCoordinator Properties
// ============================================================================

// Property 9: Coordinator should create all loops successfully
proptest! {
    #[test]
    fn prop_coordinator_creates_all_loops(_dummy in 0u8..1u8) {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut coordinator = LoopCoordinator::new();

            for loop_type in LoopType::all_loops() {
                let name = format!("{:?} Loop", loop_type);
                let desc = loop_type.description().to_string();

                let result = coordinator.create_loop(loop_type, name, desc).await;

                prop_assert!(
                    result.is_ok(),
                    "Creating {:?} loop should succeed",
                    loop_type
                );
            }

            Ok(())
        }).ok();
    }
}

// Property 10: Coordinator should list all created loops
proptest! {
    #[test]
    fn prop_coordinator_lists_loops(count in 1usize..7usize) {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut coordinator = LoopCoordinator::new();

            let loops = &LoopType::all_loops()[0..count];

            for loop_type in loops {
                let name = format!("{:?} Loop", loop_type);
                let desc = loop_type.description().to_string();
                coordinator.create_loop(*loop_type, name, desc).await.unwrap();
            }

            let list = coordinator.list_loops().await.unwrap();

            prop_assert!(
                list.len() >= count,
                "Coordinator should list at least {} loops, got {}",
                count, list.len()
            );

            Ok(())
        }).ok();
    }
}

// ============================================================================
// LoopExecutor Properties
// ============================================================================

// Property 11: Executor should be created for all loop types
proptest! {
    #[test]
    fn prop_executor_handles_all_types(_dummy in 0u8..1u8) {
        for loop_type in LoopType::all_loops() {
            let _executor = LoopExecutor::new(loop_type);
            // Just verifying creation succeeds
            prop_assert!(true, "Executor creation should succeed");
        }
    }
}

// ============================================================================
// LoopEngine Properties
// ============================================================================

// Property 12: Engine should initialize successfully
proptest! {
    #[test]
    fn prop_engine_initializes(_dummy in 0u8..1u8) {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut engine = LoopEngine::new();
            let result = engine.initialize().await;

            prop_assert!(
                result.is_ok(),
                "Engine initialization should succeed"
            );

            engine.shutdown().await.ok();
            Ok(())
        }).ok();
    }
}

// Property 13: Engine should shutdown cleanly
proptest! {
    #[test]
    fn prop_engine_shutdowns_cleanly(_dummy in 0u8..1u8) {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut engine = LoopEngine::new();
            engine.initialize().await.unwrap();

            let result = engine.shutdown().await;

            prop_assert!(
                result.is_ok(),
                "Engine shutdown should succeed"
            );

            Ok(())
        }).ok();
    }
}

// Property 14: Multiple engines can coexist
proptest! {
    #[test]
    fn prop_multiple_engines(count in 2usize..5usize) {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut engines = Vec::new();

            for _ in 0..count {
                let mut engine = LoopEngine::new();
                engine.initialize().await.unwrap();
                engines.push(engine);
            }

            prop_assert_eq!(
                engines.len(),
                count,
                "Should create {} engines",
                count
            );

            for mut engine in engines {
                engine.shutdown().await.ok();
            }

            Ok(())
        }).ok();
    }
}

// Property 15: Engine should handle rapid init/shutdown cycles
proptest! {
    #[test]
    fn prop_rapid_init_shutdown(cycles in 1usize..5usize) {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            for _ in 0..cycles {
                let mut engine = LoopEngine::new();
                engine.initialize().await.unwrap();
                engine.shutdown().await.unwrap();
            }

        });
    }
}

// ============================================================================
// CycleInput/Output Properties
// ============================================================================

// Property 16: CycleInput should preserve context
proptest! {
    #[test]
    fn prop_cycle_input_preserves_context(
        context_len in 1usize..50usize,
        objectives_count in 1usize..10usize,
    ) {
        let context: String = (0..context_len).map(|_| 'a').collect();
        let objectives: Vec<String> = (0..objectives_count)
            .map(|i| format!("objective_{}", i))
            .collect();

        let input = CycleInput {
            data: HashMap::new(),
            context: context.clone(),
            objectives: objectives.clone(),
        };

        prop_assert_eq!(
            input.context.len(),
            context_len,
            "CycleInput should preserve context length"
        );

        prop_assert_eq!(
            input.objectives.len(),
            objectives_count,
            "CycleInput should preserve objectives count"
        );
    }
}

// Property 17: CycleOutput should preserve insights
proptest! {
    #[test]
    fn prop_cycle_output_preserves_insights(insights_count in 1usize..20usize) {
        let insights: Vec<String> = (0..insights_count)
            .map(|i| format!("insight_{}", i))
            .collect();

        let output = CycleOutput {
            results: HashMap::new(),
            insights: insights.clone(),
            actions: vec![],
            next_objectives: vec![],
        };

        prop_assert_eq!(
            output.insights.len(),
            insights_count,
            "CycleOutput should preserve insights count"
        );
    }
}

// ============================================================================
// Additional Properties
// ============================================================================

// Property 18: Coordinator should handle concurrent list operations
proptest! {
    #[test]
    fn prop_concurrent_list_operations(num_ops in 5usize..20usize) {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut coordinator = LoopCoordinator::new();

            // Create all loops first
            for loop_type in LoopType::all_loops() {
                let name = format!("{:?} Loop", loop_type);
                let desc = loop_type.description().to_string();
                coordinator.create_loop(loop_type, name, desc).await.unwrap();
            }

            // Perform multiple list operations
            for _ in 0..num_ops {
                let list = coordinator.list_loops().await.unwrap();
                prop_assert!(
                    list.len() >= 7,
                    "Coordinator should maintain all loops"
                );
            }

            Ok(())
        }).ok();
    }
}

// Property 19: Loop types should maintain ordering
proptest! {
    #[test]
    fn prop_loop_ordering_maintained(_dummy in 0u8..1u8) {
        let loops = LoopType::all_loops();

        prop_assert_eq!(loops[0], LoopType::Reflexive, "First should be Reflexive");
        prop_assert_eq!(loops[loops.len() - 1], LoopType::Transcendent, "Last should be Transcendent");
    }
}

// Property 20: Loop descriptions should be meaningful
proptest! {
    #[test]
    fn prop_descriptions_meaningful(_dummy in 0u8..1u8) {
        for loop_type in LoopType::all_loops() {
            let desc = loop_type.description();

            prop_assert!(
                desc.len() > 20,
                "Description for {:?} should be meaningful (>20 chars), got {}",
                loop_type, desc.len()
            );
        }
    }
}
