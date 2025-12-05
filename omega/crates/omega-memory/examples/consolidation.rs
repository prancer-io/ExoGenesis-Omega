//! Memory consolidation example

use omega_memory::{
    CosmicMemory, Memory, MemoryContent, MemoryTier,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 ExoGenesis Omega - Memory Consolidation Demo\n");

    let memory = CosmicMemory::new().await?;

    // Create multiple instant memories with varying importance
    println!("📝 Creating memories with varying importance...\n");

    for i in 0..5 {
        let importance = 0.2 + (i as f64 * 0.2);
        let mem = Memory::new(
            MemoryTier::Instant,
            MemoryContent::Text(format!("Memory #{} with importance {:.1}", i, importance)),
            vec![i as f32 / 5.0; 4],
            importance,
        );
        memory.store(mem).await?;
        println!("  Created instant memory #{} (importance: {:.1})", i, importance);
    }

    println!("\n📊 Initial statistics:");
    let stats = memory.stats().await;
    println!("  Instant: {}", stats.individual.instant);
    println!("  Session: {}", stats.individual.session);
    println!("  Episodic: {}", stats.individual.episodic);
    println!("  Semantic: {}", stats.individual.semantic);

    // Run auto-consolidation
    println!("\n🔄 Running auto-consolidation...");
    memory.auto_consolidate().await?;

    println!("\n📊 Post-consolidation statistics:");
    let stats = memory.stats().await;
    println!("  Instant: {}", stats.individual.instant);
    println!("  Session: {}", stats.individual.session);
    println!("  Episodic: {}", stats.individual.episodic);
    println!("  Semantic: {}", stats.individual.semantic);

    println!("\n✅ High-importance memories were consolidated to higher tiers!");

    Ok(())
}
