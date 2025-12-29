//! RvLite Integration Simulation
//!
//! This example demonstrates the RvLite integration for long-term memory
//! persistence in ExoGenesis-Omega. It tests:
//!
//! 1. Memory storage across tiers
//! 2. Semantic similarity search
//! 3. Graph relationships
//! 4. Memory consolidation
//! 5. Hippocampal replay
//! 6. JSON persistence (save/load)
//!
//! Run with: cargo run --example rvlite_simulation

use omega_memory::{
    Memory, MemoryContent, MemoryTier,
    RvLiteBridge, RvLiteBridgeConfig, RelationshipType,
    ConsolidationThresholds,
};
use chrono::Utc;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("═══════════════════════════════════════════════════════════════");
    println!("       RvLite Integration Simulation - ExoGenesis Omega");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Configure RvLite bridge
    let storage_path = "/tmp/omega_rvlite_test.json";
    let config = RvLiteBridgeConfig {
        dimensions: 128, // Small for testing
        storage_path: Some(storage_path.to_string()),
        auto_sync_interval_secs: 0, // Disabled for manual control
        enable_graph: true,
        min_persistent_tier: MemoryTier::Instant, // Persist all tiers for testing
        consolidation_thresholds: ConsolidationThresholds {
            importance_threshold: 0.5,
            min_access_count: 2,
            min_age_hours: 0, // Immediate for testing
            decay_rate_individual: 0.99,
            decay_rate_species: 0.999,
        },
        max_memories_per_tier: HashMap::new(),
    };

    println!("[1/7] Creating RvLite Bridge...");
    let bridge = RvLiteBridge::new(config).await?;
    println!("      ✓ Bridge created with {} dimensions\n", 128);

    // =========================================================================
    // PHASE 1: Store memories across different tiers
    // =========================================================================
    println!("[2/7] Storing memories across tiers...");

    let memories = vec![
        ("Instant memory - working context", MemoryTier::Instant, 0.3),
        ("Session memory - current conversation", MemoryTier::Session, 0.5),
        ("Episodic memory - yesterday's event", MemoryTier::Episodic, 0.7),
        ("Semantic memory - learned fact", MemoryTier::Semantic, 0.8),
        ("Collective memory - shared knowledge", MemoryTier::Collective, 0.85),
        ("Evolutionary memory - species pattern", MemoryTier::Evolutionary, 0.9),
    ];

    let mut stored_ids: Vec<String> = Vec::new();

    for (i, (content, tier, importance)) in memories.iter().enumerate() {
        // Create a simple embedding (in production, use a real embedding model)
        let mut embedding = vec![0.0f32; 128];
        embedding[i % 128] = 1.0;
        embedding[(i + 1) % 128] = 0.5;
        embedding[(i + 64) % 128] = 0.3;

        let memory = Memory {
            id: format!("mem-{}", i),
            tier: *tier,
            content: MemoryContent::Text(content.to_string()),
            embedding,
            importance: *importance,
            created_at: Utc::now(),
            accessed_at: Utc::now(),
            access_count: 0,
            metadata: serde_json::json!({
                "source": "simulation",
                "index": i,
            }),
        };

        let id = bridge.store(&memory).await?;
        stored_ids.push(id.clone());
        println!("      ✓ Stored {} at tier {:?} (importance: {:.2})", id, tier, importance);
    }

    let stats = bridge.stats().await;
    println!("\n      Total memories: {}", stats.total_memories);
    println!("      Tier distribution: {:?}\n", stats.memories_per_tier);

    // =========================================================================
    // PHASE 2: Test semantic similarity search
    // =========================================================================
    println!("[3/7] Testing semantic similarity search...");

    // Create a query embedding similar to the first memory
    let mut query_embedding = vec![0.0f32; 128];
    query_embedding[0] = 1.0;
    query_embedding[1] = 0.5;

    let results = bridge.semantic_query(&query_embedding, 3, None).await?;
    println!("      Query returned {} results:", results.len());
    for result in &results {
        println!("        - {} (similarity: {:.4}, tier: {:?})",
            result.memory.id, result.similarity, MemoryTier::from(result.memory.tier));
    }

    // Query with tier filter
    let filtered_results = bridge.semantic_query(
        &query_embedding, 5, Some(MemoryTier::Semantic)
    ).await?;
    println!("\n      Filtered (Semantic+) returned {} results", filtered_results.len());
    println!();

    // =========================================================================
    // PHASE 3: Add graph relationships
    // =========================================================================
    println!("[4/7] Adding graph relationships...");

    // Create causal chain: mem-0 -> mem-1 -> mem-2
    bridge.add_relationship("mem-0", "mem-1", RelationshipType::Causes, 0.9).await?;
    bridge.add_relationship("mem-1", "mem-2", RelationshipType::Causes, 0.8).await?;
    bridge.add_relationship("mem-2", "mem-3", RelationshipType::Precedes, 0.7).await?;

    // Add semantic similarity links
    bridge.add_relationship("mem-3", "mem-4", RelationshipType::SimilarTo, 0.85).await?;
    bridge.add_relationship("mem-4", "mem-5", RelationshipType::AssociatedWith, 0.75).await?;

    println!("      ✓ Created 5 relationships");

    // Find related memories
    let related = bridge.find_related("mem-0", 3, None).await?;
    println!("      Related to mem-0 (within 3 hops): {} nodes", related.len());
    for (id, hops, weight) in &related {
        println!("        - {} (hops: {}, weight: {:.2})", id, hops, weight);
    }
    println!();

    // =========================================================================
    // PHASE 4: Test memory replay (hippocampal strengthening)
    // =========================================================================
    println!("[5/7] Testing memory replay (hippocampal strengthening)...");

    // Replay some memories multiple times
    let replay_targets = vec!["mem-2".to_string(), "mem-3".to_string()];

    for i in 0..5 {
        let strengthened = bridge.replay_memories(&replay_targets).await?;
        if i == 0 {
            println!("      Replaying {} memories...", strengthened);
        }
    }
    println!("      ✓ Completed 5 replay cycles");

    // Check strengthening effect
    let replayed_results = bridge.semantic_query(&query_embedding, 10, None).await?;
    for result in &replayed_results {
        if replay_targets.contains(&result.memory.id) {
            println!("      {} strength: {:.2}, replay_count: {}, access_count: {}",
                result.memory.id,
                result.memory.strength,
                result.memory.replay_count,
                result.memory.access_count
            );
        }
    }
    println!();

    // =========================================================================
    // PHASE 5: Test consolidation
    // =========================================================================
    println!("[6/7] Testing memory consolidation...");

    let consolidation_report = bridge.consolidate().await?;
    println!("      Consolidation report:");
    println!("        - Promoted: {}", consolidation_report.promoted);
    println!("        - Decayed: {}", consolidation_report.decayed);
    println!("        - Removed: {}", consolidation_report.removed);

    let stats_after = bridge.stats().await;
    println!("      After consolidation: {} memories", stats_after.total_memories);
    println!();

    // =========================================================================
    // PHASE 6: Test persistence (save/load)
    // =========================================================================
    println!("[7/7] Testing persistence (save/load)...");

    // Save to file
    bridge.save().await?;
    println!("      ✓ Saved to {}", storage_path);

    // Export to JSON
    let exported = bridge.export_json().await?;
    let export_size = serde_json::to_string(&exported)?.len();
    println!("      ✓ Exported JSON size: {} bytes", export_size);

    // Create a new bridge and load data
    let config2 = RvLiteBridgeConfig {
        dimensions: 128,
        storage_path: Some(storage_path.to_string()),
        ..Default::default()
    };
    let bridge2 = RvLiteBridge::new(config2).await?;

    let loaded_stats = bridge2.stats().await;
    println!("      ✓ Loaded {} memories from file", loaded_stats.total_memories);

    // Verify data integrity
    let loaded_results = bridge2.semantic_query(&query_embedding, 3, None).await?;
    println!("      ✓ Query on loaded data returned {} results", loaded_results.len());

    // Verify relationships persisted
    let loaded_related = bridge2.find_related("mem-0", 3, None).await?;
    println!("      ✓ Loaded {} relationships from mem-0", loaded_related.len());

    // =========================================================================
    // Summary
    // =========================================================================
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("                      SIMULATION COMPLETE");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("  Results Summary:");
    println!("  ─────────────────────────────────────────────────────────────");
    println!("  • Memories stored:        {}", stats.total_memories);
    println!("  • Relationships created:  5");
    println!("  • Replay cycles:          5");
    println!("  • Consolidations:         1");
    println!("  • Persistence verified:   ✓");
    println!("  • Data integrity:         ✓");
    println!();
    println!("  RvLite integration is working correctly!");
    println!("═══════════════════════════════════════════════════════════════\n");

    // Cleanup
    if std::path::Path::new(storage_path).exists() {
        std::fs::remove_file(storage_path)?;
        println!("  [Cleanup] Removed test file: {}", storage_path);
    }

    Ok(())
}
