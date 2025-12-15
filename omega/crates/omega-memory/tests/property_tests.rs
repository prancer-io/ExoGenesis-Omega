//! Property-based tests for omega-memory
//! Tests mathematical invariants and properties of the 12-tier memory system
//!
//! ⚠️ IMPORTANT: These tests use shared AgentDB files and MUST run sequentially:
//! ```bash
//! cargo test --package omega-memory --test property_tests -- --test-threads=1
//! ```

use omega_memory::{CosmicMemory, Memory, MemoryContent, MemoryTier, Query};
use proptest::prelude::*;
use std::fs;
use std::path::Path;

// Helper to clean AgentDB files before tests
fn cleanup_agentdb() {
    // Ensure directory exists
    let _ = fs::create_dir_all("/tmp/omega/memory");

    let paths = vec![
        "/tmp/omega/memory/episodic.agentdb",
        "/tmp/omega/memory/semantic.agentdb",
    ];

    for path in paths {
        if Path::new(path).exists() {
            let _ = fs::remove_file(path);
        }
    }
}

// Helper to create test memory
fn create_test_memory(tier: MemoryTier, text: String, importance: f64) -> Memory {
    Memory::new(
        tier,
        MemoryContent::Text(text),
        vec![0.1, 0.2, 0.3],  // Simple embedding
        importance,
    )
}

// Property 1: Store and recall should be idempotent
proptest! {
    #[test]
    fn prop_store_recall_idempotent(
        text in "[a-zA-Z0-9 ]{5,20}",
        importance in 0.0f64..1.0f64,
    ) {
        cleanup_agentdb();
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let memory_sys = CosmicMemory::new().await.unwrap();
            let memory = create_test_memory(MemoryTier::Session, text.clone(), importance);
            let id = memory.id.clone();

            // Store
            memory_sys.store(memory).await.unwrap();

            // Recall all from session tier
            let query = Query::new();
            let results = memory_sys.recall(&query, &[MemoryTier::Session]).await.unwrap();

            // Should find our memory
            prop_assert!(results.iter().any(|m| m.id == id));

            if let Some(recalled) = results.iter().find(|m| m.id == id) {
                match &recalled.content {
                    MemoryContent::Text(t) => prop_assert_eq!(t, &text),
                    _ => prop_assert!(false, "Wrong content type"),
                }
                prop_assert_eq!(recalled.importance, importance);
            }
            Ok(())
        }).unwrap();
    }
}

// Property 2: Importance should affect relevance score
proptest! {
    #[test]
    fn prop_importance_affects_relevance(
        low_imp in 0.0f64..0.3f64,
        high_imp in 0.7f64..1.0f64,
    ) {
        let low_memory = create_test_memory(
            MemoryTier::Semantic,
            "low importance".to_string(),
            low_imp,
        );
        let high_memory = create_test_memory(
            MemoryTier::Semantic,
            "high importance".to_string(),
            high_imp,
        );

        let low_score = low_memory.relevance_score();
        let high_score = high_memory.relevance_score();

        prop_assert!(
            high_score > low_score,
            "Higher importance ({}) should have higher relevance score ({}) than lower importance ({}) with score ({})",
            high_imp, high_score, low_imp, low_score
        );
    }
}

// Property 3: Access count should increase relevance
proptest! {
    #[test]
    fn prop_access_boosts_relevance(access_count in 1u64..100u64) {
        let mut memory = create_test_memory(
            MemoryTier::Session,
            "test".to_string(),
            0.5,
        );

        let initial_score = memory.relevance_score();

        // Simulate accesses
        for _ in 0..access_count {
            memory.touch();
        }

        let final_score = memory.relevance_score();

        prop_assert!(
            final_score > initial_score,
            "Accessing memory {} times should increase relevance from {} to {}",
            access_count, initial_score, final_score
        );
    }
}

// Property 4: All tiers should be storable
proptest! {
    #[test]
    fn prop_all_tiers_storable(tier_idx in 0usize..12usize) {
        let tiers = [
            MemoryTier::Instant,
            MemoryTier::Session,
            MemoryTier::Episodic,
            MemoryTier::Semantic,
            MemoryTier::Collective,
            MemoryTier::Evolutionary,
            MemoryTier::Architectural,
            MemoryTier::Substrate,
            MemoryTier::Civilizational,
            MemoryTier::Temporal,
            MemoryTier::Physical,
            MemoryTier::Omega,
        ];

        let tier = tiers[tier_idx];

        cleanup_agentdb();
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let memory_sys = CosmicMemory::new().await.unwrap();
            let memory = create_test_memory(tier, format!("tier_{:?}", tier), 0.5);

            let result = memory_sys.store(memory).await;
            prop_assert!(result.is_ok(), "Tier {:?} should be storable", tier);
            Ok(())
        }).unwrap();
    }
}

// Property 5: Multiple stores should all succeed
proptest! {
    #[test]
    fn prop_concurrent_stores_succeed(count in 5usize..50usize) {
        cleanup_agentdb();
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let memory_sys = CosmicMemory::new().await.unwrap();

            // Store memories sequentially (CosmicMemory doesn't implement Clone)
            for i in 0..count {
                let memory = create_test_memory(
                    MemoryTier::Session,
                    format!("concurrent_{}", i),
                    0.5,
                );
                memory_sys.store(memory).await.unwrap();
            }

            // Verify all were stored
            let query = Query::new();
            let results = memory_sys.recall(&query, &[MemoryTier::Session]).await.unwrap();
            prop_assert!(results.len() >= count, "All stores should succeed");
            Ok(())
        }).unwrap();
    }
}

// Property 6: Recall should respect tier boundaries
proptest! {
    #[test]
    fn prop_recall_respects_tiers(text in "[a-z]{5,15}") {
        cleanup_agentdb();
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let memory_sys = CosmicMemory::new().await.unwrap();

            // Store in session tier
            let session_mem = create_test_memory(MemoryTier::Session, text.clone(), 0.5);
            let session_id = session_mem.id.clone();
            memory_sys.store(session_mem).await.unwrap();

            // Store in episodic tier
            let episodic_mem = create_test_memory(MemoryTier::Episodic, text.clone(), 0.5);
            let episodic_id = episodic_mem.id.clone();
            memory_sys.store(episodic_mem).await.unwrap();

            // Query only session tier
            let query = Query::new();
            let session_results = memory_sys.recall(&query, &[MemoryTier::Session]).await.unwrap();

            // Should find session memory
            prop_assert!(session_results.iter().any(|m| m.id == session_id));
            // Should NOT find episodic memory (it's in a different tier)
            prop_assert!(!session_results.iter().any(|m| m.id == episodic_id));

            Ok(())
        }).unwrap();
    }
}

// Property 7: Consolidation should be safe (no data loss)
proptest! {
    #[test]
    fn prop_consolidation_preserves_data(importance in 0.5f64..1.0f64) {
        cleanup_agentdb();
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let memory_sys = CosmicMemory::new().await.unwrap();

            let memory = create_test_memory(
                MemoryTier::Instant,
                "important_data".to_string(),
                importance,
            );
            let id = memory.id.clone();

            memory_sys.store(memory).await.unwrap();

            // Run consolidation
            let result = memory_sys.consolidate(MemoryTier::Instant, MemoryTier::Session).await;
            prop_assert!(result.is_ok(), "Consolidation should succeed");

            // Data should still be accessible (either in instant or session tier)
            let query = Query::new();
            let all_results = memory_sys.recall(
                &query,
                &[MemoryTier::Instant, MemoryTier::Session]
            ).await.unwrap();

            prop_assert!(
                all_results.iter().any(|m| m.id == id),
                "Memory should be accessible after consolidation"
            );
            Ok(())
        }).unwrap();
    }
}

// Property 8: Statistics should be consistent
proptest! {
    #[test]
    fn prop_stats_consistent(store_count in 1usize..30usize) {
        cleanup_agentdb();
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let memory_sys = CosmicMemory::new().await.unwrap();

            // Store memories
            for i in 0..store_count {
                let memory = create_test_memory(
                    MemoryTier::Session,
                    format!("mem_{}", i),
                    0.5,
                );
                memory_sys.store(memory).await.unwrap();
            }

            let stats = memory_sys.stats().await;

            prop_assert!(
                stats.total_memories >= store_count,
                "Stats should reflect at least {} stored memories, got {}",
                store_count,
                stats.total_memories
            );
            Ok(())
        }).unwrap();
    }
}

// Property 9: Memory content types should be preserved
proptest! {
    #[test]
    fn prop_content_type_preserved(
        text in "[a-zA-Z0-9 ]{10,50}",
        embedding_len in 5usize..20usize,
    ) {
        cleanup_agentdb();
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let memory_sys = CosmicMemory::new().await.unwrap();

            // Test Text content
            let text_memory = Memory::new(
                MemoryTier::Session,
                MemoryContent::Text(text.clone()),
                vec![0.1; embedding_len],
                0.5,
            );
            let text_id = text_memory.id.clone();
            memory_sys.store(text_memory).await.unwrap();

            let query = Query::new();
            let results = memory_sys.recall(&query, &[MemoryTier::Session]).await.unwrap();
            let recalled = results.iter().find(|m| m.id == text_id).unwrap();

            match &recalled.content {
                MemoryContent::Text(t) => prop_assert_eq!(t, &text),
                _ => prop_assert!(false, "Content type should be preserved as Text"),
            }

            Ok(())
        }).unwrap();
    }
}

// Property 10: Tier hierarchy should be maintained
proptest! {
    #[test]
    fn prop_tier_hierarchy_maintained(_dummy in 0u8..1u8) {
        // Individual scale tiers (0-3)
        let individual_tiers = [
            MemoryTier::Instant,
            MemoryTier::Session,
            MemoryTier::Episodic,
            MemoryTier::Semantic,
        ];

        // Species scale tiers (4-7)
        let species_tiers = [
            MemoryTier::Collective,
            MemoryTier::Evolutionary,
            MemoryTier::Architectural,
            MemoryTier::Substrate,
        ];

        // Cosmic scale tiers (8-11)
        let cosmic_tiers = [
            MemoryTier::Civilizational,
            MemoryTier::Temporal,
            MemoryTier::Physical,
            MemoryTier::Omega,
        ];

        cleanup_agentdb();
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let memory_sys = CosmicMemory::new().await.unwrap();

            // Store one memory in each scale
            for tier in &individual_tiers {
                let mem = create_test_memory(*tier, format!("{:?}", tier), 0.5);
                memory_sys.store(mem).await.unwrap();
            }

            for tier in &species_tiers {
                let mem = create_test_memory(*tier, format!("{:?}", tier), 0.5);
                memory_sys.store(mem).await.unwrap();
            }

            for tier in &cosmic_tiers {
                let mem = create_test_memory(*tier, format!("{:?}", tier), 0.5);
                memory_sys.store(mem).await.unwrap();
            }

            // All should be queryable in their respective scales
            let query = Query::new();
            let ind_results = memory_sys.recall(&query, &individual_tiers).await.unwrap();
            let spe_results = memory_sys.recall(&query, &species_tiers).await.unwrap();
            let cos_results = memory_sys.recall(&query, &cosmic_tiers).await.unwrap();

            prop_assert!(ind_results.len() >= 4, "Individual scale should have memories");
            prop_assert!(spe_results.len() >= 4, "Species scale should have memories");
            prop_assert!(cos_results.len() >= 4, "Cosmic scale should have memories");

            Ok(())
        }).unwrap();
    }
}
