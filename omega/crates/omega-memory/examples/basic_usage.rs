//! Basic usage example for Omega Memory system

use omega_memory::{
    CosmicMemory, Memory, MemoryContent, MemoryTier, QueryBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧠 ExoGenesis Omega - Cosmic Memory System Demo\n");

    // Initialize the cosmic memory system
    let memory = CosmicMemory::new().await?;
    println!("✅ Initialized 12-tier cosmic memory system\n");

    // Store memories in different tiers
    println!("📝 Storing memories across tiers...\n");

    // Tier 1: Instant memory
    let instant_mem = Memory::new(
        MemoryTier::Instant,
        MemoryContent::Text("Current thought: Processing user input".to_string()),
        vec![0.1, 0.2, 0.3, 0.4],
        0.3,
    );
    memory.store(instant_mem).await?;
    println!("  [T1] Instant: Working memory stored");

    // Tier 2: Session memory
    let session_mem = Memory::new(
        MemoryTier::Session,
        MemoryContent::Text("Conversation context about Rust programming".to_string()),
        vec![0.5, 0.6, 0.7, 0.8],
        0.5,
    );
    memory.store(session_mem).await?;
    println!("  [T2] Session: Conversation context stored");

    // Tier 3: Episodic memory
    let episodic_mem = Memory::new(
        MemoryTier::Episodic,
        MemoryContent::Text("Completed implementation of memory system on 2025-12-04".to_string()),
        vec![0.2, 0.4, 0.6, 0.8],
        0.7,
    );
    memory.store(episodic_mem).await?;
    println!("  [T3] Episodic: Event memory stored");

    // Tier 4: Semantic memory
    let semantic_mem = Memory::new(
        MemoryTier::Semantic,
        MemoryContent::Text("Knowledge: Rust uses ownership for memory safety".to_string()),
        vec![0.9, 0.8, 0.7, 0.6],
        0.8,
    );
    memory.store(semantic_mem).await?;
    println!("  [T4] Semantic: Knowledge stored");

    // Tier 5: Collective memory
    let collective_mem = Memory::new(
        MemoryTier::Collective,
        MemoryContent::Text("Shared pattern: Async/await improves concurrency".to_string()),
        vec![0.7, 0.7, 0.7, 0.7],
        0.75,
    );
    memory.store(collective_mem).await?;
    println!("  [T5] Collective: Shared knowledge stored");

    // Tier 12: Omega memory
    let omega_mem = Memory::new(
        MemoryTier::Omega,
        MemoryContent::Text("Universal principle: Information cannot be destroyed".to_string()),
        vec![1.0, 1.0, 1.0, 1.0],
        0.99,
    );
    memory.store(omega_mem).await?;
    println!("  [T12] Omega: Universal truth stored\n");

    // Query memories
    println!("🔍 Querying memories...\n");

    // Query individual scale memories
    let query = QueryBuilder::new()
        .individual()
        .min_importance(0.5)
        .build();

    let results = memory.recall(&query, &[
        MemoryTier::Instant,
        MemoryTier::Session,
        MemoryTier::Episodic,
        MemoryTier::Semantic,
    ]).await?;

    println!("  Found {} memories in individual scale (T1-T4):", results.len());
    for mem in &results {
        if let MemoryContent::Text(text) = &mem.content {
            println!("    - {}: {}", mem.tier, text);
        }
    }

    // Memory statistics
    println!("\n📊 Memory Statistics:\n");
    let stats = memory.stats().await;
    println!("  Individual Scale (T1-T4):");
    println!("    - Instant: {}", stats.individual.instant);
    println!("    - Session: {}", stats.individual.session);
    println!("    - Episodic: {}", stats.individual.episodic);
    println!("    - Semantic: {}", stats.individual.semantic);
    println!("  Species Scale (T5-T8):");
    println!("    - Collective: {}", stats.species.collective);
    println!("    - Evolutionary: {}", stats.species.evolutionary);
    println!("    - Architectural: {}", stats.species.architectural);
    println!("    - Substrate: {}", stats.species.substrate);
    println!("  Cosmic Scale (T9-T12):");
    println!("    - Civilizational: {}", stats.cosmic.civilizational);
    println!("    - Temporal: {}", stats.cosmic.temporal);
    println!("    - Physical: {}", stats.cosmic.physical);
    println!("    - Omega: {}", stats.cosmic.omega);
    println!("\n  Total memories: {}", stats.total_memories);

    // Run consolidation
    println!("\n🔄 Running automatic consolidation...");
    memory.auto_consolidate().await?;
    println!("✅ Consolidation complete\n");

    println!("🎉 Demo complete!");

    Ok(())
}
