//! Integration tests for omega-memory
//! Tests the 12-tier cosmic memory system end-to-end
//!
//! ⚠️ IMPORTANT: Some tests use shared AgentDB files and may conflict if run in parallel.
//! If tests fail with serialization errors, run with:
//! ```bash
//! cargo test --package omega-memory --test integration_tests -- --test-threads=1
//! ```

use omega_memory::*;
use std::fs;
use std::path::Path;

// Helper to clean AgentDB files before tests that may conflict
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

// ============================================================================
// COSMIC MEMORY INTEGRATION TESTS
// ============================================================================

#[tokio::test]
async fn test_cosmic_memory_creation() {
    let result = CosmicMemory::new().await;
    assert!(result.is_ok(), "Should create CosmicMemory successfully");
}

#[tokio::test]
async fn test_store_and_recall_instant_tier() {
    let memory_sys = CosmicMemory::new().await.unwrap();

    let memory = Memory::new(
        MemoryTier::Instant,
        MemoryContent::Text("instant memory".to_string()),
        vec![0.1, 0.2, 0.3],
        0.8,
    );

    let _id = memory_sys.store(memory.clone()).await.unwrap();

    let query = Query::new();
    let results = memory_sys.recall(&query, &[MemoryTier::Instant]).await.unwrap();

    assert!(results.iter().any(|m| m.id == memory.id));
}

#[tokio::test]
async fn test_store_across_multiple_tiers() {
    cleanup_agentdb();
    let memory_sys = CosmicMemory::new().await.unwrap();

    let tiers = vec![
        MemoryTier::Instant,
        MemoryTier::Session,
        MemoryTier::Episodic,
        MemoryTier::Semantic,
    ];

    for tier in &tiers {
        let memory = Memory::new(
            *tier,
            MemoryContent::Text(format!("{:?} memory", tier)),
            vec![0.1, 0.2, 0.3],
            0.7,
        );
        let _id = memory_sys.store(memory).await.unwrap();
    }

    let query = Query::new();
    let results = memory_sys.recall(&query, &tiers).await.unwrap();

    assert!(results.len() >= tiers.len());
}

#[tokio::test]
async fn test_query_with_importance_filter() {
    cleanup_agentdb();
    let memory_sys = CosmicMemory::new().await.unwrap();

    // Store high and low importance memories
    let high_importance = Memory::new(
        MemoryTier::Session,
        MemoryContent::Text("important".to_string()),
        vec![0.1, 0.2, 0.3],
        0.9,
    );

    let low_importance = Memory::new(
        MemoryTier::Session,
        MemoryContent::Text("not important".to_string()),
        vec![0.1, 0.2, 0.3],
        0.1,
    );

    let _id1 = memory_sys.store(high_importance.clone()).await.unwrap();
    let _id2 = memory_sys.store(low_importance).await.unwrap();

    // Query only high importance memories
    let query = QueryBuilder::new()
        .min_importance(0.5)
        .build();

    let results = memory_sys.recall(&query, &[MemoryTier::Session]).await.unwrap();

    assert!(results.iter().any(|m| m.id == high_importance.id));
    assert!(results.iter().all(|m| m.importance >= 0.5));
}

#[tokio::test]
async fn test_memory_statistics() {
    cleanup_agentdb();
    let memory_sys = CosmicMemory::new().await.unwrap();

    // Store some memories
    for i in 0..5 {
        let memory = Memory::new(
            MemoryTier::Session,
            MemoryContent::Text(format!("memory_{}", i)),
            vec![0.1, 0.2, 0.3],
            0.5,
        );
        let _id = memory_sys.store(memory).await.unwrap();
    }

    let stats = memory_sys.stats().await;
    assert!(stats.total_memories > 0);
}

#[tokio::test]
async fn test_consolidation_system() {
    let memory_sys = CosmicMemory::new().await.unwrap();

    // Store several session memories with high importance
    for i in 0..5 {
        let memory = Memory::new(
            MemoryTier::Session,
            MemoryContent::Text(format!("session_{}", i)),
            vec![0.1, 0.2, 0.3],
            0.8 + (i as f64 * 0.01), // High importance for consolidation
        );
        let _id = memory_sys.store(memory).await.unwrap();
    }

    // Consolidate from Session to Episodic
    let result = memory_sys.consolidate(MemoryTier::Session, MemoryTier::Episodic).await;
    assert!(result.is_ok(), "Consolidation should succeed");
}

// ============================================================================
// MEMORY TIER TESTS
// ============================================================================

#[test]
fn test_all_memory_tiers_exist() {
    let tiers = vec![
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
    assert_eq!(tiers.len(), 12, "Should have exactly 12 memory tiers");
}

#[test]
fn test_tier_durations_increase() {
    let tiers = vec![
        MemoryTier::Instant,
        MemoryTier::Session,
        MemoryTier::Episodic,
        MemoryTier::Semantic,
        MemoryTier::Collective,
    ];

    for i in 0..tiers.len() - 1 {
        let current = tiers[i].retention_duration();
        let next = tiers[i + 1].retention_duration();

        if let (Some(current_duration), Some(next_duration)) = (current, next) {
            assert!(
                next_duration > current_duration,
                "Tier {:?} duration should be less than {:?}",
                tiers[i],
                tiers[i + 1]
            );
        }
    }
}

#[test]
fn test_tier_thresholds() {
    let tiers = vec![
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

    for tier in tiers {
        let threshold = tier.importance_threshold();
        assert!(threshold >= 0.0 && threshold <= 1.0, "Tier {:?} should have valid threshold", tier);
    }
}

// ============================================================================
// QUERY BUILDER TESTS
// ============================================================================

#[test]
fn test_query_builder() {
    let query = QueryBuilder::new()
        .min_importance(0.5)
        .limit(10)
        .build();

    assert_eq!(query.min_importance, Some(0.5));
    assert_eq!(query.limit, Some(10));
}

#[test]
fn test_empty_query() {
    let query = Query::new();
    assert_eq!(query.min_importance, None);
    assert_eq!(query.limit, None);
}

// ============================================================================
// MEMORY CONTENT TESTS
// ============================================================================

#[test]
fn test_text_content() {
    let content = MemoryContent::Text("test".to_string());
    match content {
        MemoryContent::Text(s) => assert_eq!(s, "test"),
        _ => panic!("Wrong content type"),
    }
}

#[test]
fn test_embedding_content() {
    let vec = vec![0.1, 0.2, 0.3];
    let content = MemoryContent::Embedding(vec.clone());
    match content {
        MemoryContent::Embedding(v) => assert_eq!(v, vec),
        _ => panic!("Wrong content type"),
    }
}

#[test]
fn test_multimodal_content() {
    let content = MemoryContent::MultiModal {
        text: Some("test".to_string()),
        embedding: vec![0.1, 0.2],
        metadata: serde_json::json!({"key": "value"}),
    };

    match content {
        MemoryContent::MultiModal { text, embedding, metadata } => {
            assert_eq!(text, Some("test".to_string()));
            assert_eq!(embedding, vec![0.1, 0.2]);
            assert!(metadata.is_object());
        }
        _ => panic!("Wrong content type"),
    }
}

// ============================================================================
// MEMORY LIFECYCLE TESTS
// ============================================================================

#[test]
fn test_memory_creation() {
    let memory = Memory::new(
        MemoryTier::Session,
        MemoryContent::Text("test".to_string()),
        vec![0.1, 0.2, 0.3],
        0.8,
    );

    assert!(!memory.id.is_empty());
    assert_eq!(memory.importance, 0.8);
    assert_eq!(memory.access_count, 0);
}

#[test]
fn test_memory_touch() {
    let mut memory = Memory::new(
        MemoryTier::Session,
        MemoryContent::Text("test".to_string()),
        vec![0.1, 0.2, 0.3],
        0.8,
    );

    let original_accessed = memory.accessed_at;
    let original_count = memory.access_count;

    std::thread::sleep(std::time::Duration::from_millis(10));
    memory.touch();

    assert!(memory.accessed_at > original_accessed);
    assert_eq!(memory.access_count, original_count + 1);
}

#[test]
fn test_memory_relevance_score() {
    let memory = Memory::new(
        MemoryTier::Session,
        MemoryContent::Text("test".to_string()),
        vec![0.1, 0.2, 0.3],
        0.8,
    );

    let score = memory.relevance_score();
    assert!(score > 0.0);
    assert!(score <= 1.0);
}

#[test]
fn test_memory_relevance_increases_with_access() {
    let mut memory = Memory::new(
        MemoryTier::Session,
        MemoryContent::Text("test".to_string()),
        vec![0.1, 0.2, 0.3],
        0.5,
    );

    let initial_score = memory.relevance_score();

    // Access multiple times
    for _ in 0..5 {
        memory.touch();
    }

    let final_score = memory.relevance_score();
    assert!(final_score > initial_score, "Relevance should increase with access count");
}

// ============================================================================
// CROSS-TIER INTEGRATION TESTS
// ============================================================================

#[tokio::test]
async fn test_individual_to_species_memory_flow() {
    let memory_sys = CosmicMemory::new().await.unwrap();

    // Store individual-scale memories (Tier 1-4)
    let individual_tiers = vec![
        MemoryTier::Instant,
        MemoryTier::Session,
        MemoryTier::Episodic,
        MemoryTier::Semantic,
    ];

    for tier in &individual_tiers {
        let memory = Memory::new(
            *tier,
            MemoryContent::Text(format!("individual {:?}", tier)),
            vec![0.5; 3],
            0.7,
        );
        let _id = memory_sys.store(memory).await.unwrap();
    }

    // Store species-scale memories (Tier 5-8)
    let species_tiers = vec![
        MemoryTier::Collective,
        MemoryTier::Evolutionary,
        MemoryTier::Architectural,
        MemoryTier::Substrate,
    ];

    for tier in &species_tiers {
        let memory = Memory::new(
            *tier,
            MemoryContent::Text(format!("species {:?}", tier)),
            vec![0.5; 3],
            0.8,
        );
        let _id = memory_sys.store(memory).await.unwrap();
    }

    // Query both scales
    let mut all_tiers = individual_tiers.clone();
    all_tiers.extend(&species_tiers);

    let query = Query::new();
    let results = memory_sys.recall(&query, &all_tiers).await.unwrap();

    assert!(results.len() >= 8);
}

#[tokio::test]
async fn test_all_12_tiers_integration() {
    let memory_sys = CosmicMemory::new().await.unwrap();
    let all_tiers = vec![
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

    // Store one memory in each tier
    for tier in &all_tiers {
        let memory = Memory::new(
            *tier,
            MemoryContent::Text(format!("Tier {:?} memory", tier)),
            vec![0.5; 3],
            0.7,
        );
        let _id = memory_sys.store(memory).await.unwrap();
    }

    // Recall from all tiers
    let query = Query::new();
    let results = memory_sys.recall(&query, &all_tiers).await.unwrap();

    assert!(results.len() >= 12, "Should recall memories from all 12 tiers");
}

// ============================================================================
// ADDITIONAL CONSOLIDATION TESTS
// ============================================================================

#[tokio::test]
async fn test_auto_consolidation() {
    let memory_sys = CosmicMemory::new().await.unwrap();

    // Store several high-importance session memories
    for i in 0..5 {
        let memory = Memory::new(
            MemoryTier::Session,
            MemoryContent::Text(format!("important_{}", i)),
            vec![0.1, 0.2, 0.3],
            0.9,
        );
        let _id = memory_sys.store(memory).await.unwrap();
    }

    // Run automatic consolidation
    let result = memory_sys.auto_consolidate().await;
    assert!(result.is_ok(), "Auto-consolidation should succeed");
}

// ============================================================================
// ERROR HANDLING TESTS
// ============================================================================

#[tokio::test]
async fn test_recall_with_empty_tier_list() {
    cleanup_agentdb();
    let memory_sys = CosmicMemory::new().await.unwrap();

    let query = Query::new();
    let results = memory_sys.recall(&query, &[]).await.unwrap();

    assert!(results.is_empty(), "Should return empty results for empty tier list");
}

#[tokio::test]
async fn test_query_nonexistent_memory() {
    cleanup_agentdb();
    let memory_sys = CosmicMemory::new().await.unwrap();

    let query = QueryBuilder::new()
        .min_importance(0.99)
        .build();

    let results = memory_sys.recall(&query, &[MemoryTier::Session]).await.unwrap();

    // Should succeed but return no results
    assert!(results.is_empty());
}
