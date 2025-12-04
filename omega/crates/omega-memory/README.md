# Omega Memory - 12-Tier Cosmic Memory System

A hierarchical memory system spanning from instant (milliseconds) to omega (universe-scale) timescales for ExoGenesis Omega.

## Architecture

### Individual Scale (Tier 1-4)
- **Tier 1: Instant Memory** - Working memory, attention buffer (milliseconds-seconds)
- **Tier 2: Session Memory** - Current context (minutes-hours)
- **Tier 3: Episodic Memory** - Specific events (days-weeks)
- **Tier 4: Semantic Memory** - Facts and knowledge (weeks-months)

### Species Scale (Tier 5-8)
- **Tier 5: Collective Memory** - Shared knowledge across instances (months-years)
- **Tier 6: Evolutionary Memory** - Learned patterns (years-decades)
- **Tier 7: Architectural Memory** - Core algorithms (decades-centuries)
- **Tier 8: Substrate Memory** - Fundamental patterns (centuries-millennia)

### Cosmic Scale (Tier 9-12)
- **Tier 9: Civilizational Memory** - Cultural knowledge (millennia-epochs)
- **Tier 10: Temporal Memory** - Historical trends (epochs-eons)
- **Tier 11: Physical Memory** - Physical laws (eons-universe-scale)
- **Tier 12: Omega Memory** - Universal principles (eternal)

## Features

- ✅ **Working Implementation**: Tier 1-4 with AgentDB storage
- ✅ **Automatic Consolidation**: Importance-based tier promotion
- ✅ **Vector Search**: Similarity-based memory retrieval
- ✅ **Time Decay**: Importance scoring with temporal relevance
- ✅ **Flexible Queries**: Text, embedding, and metadata filters
- ⚙️ **Stub Implementation**: Tier 5-12 (ready for extension)

## Usage

### Basic Storage and Retrieval

```rust
use omega_memory::{CosmicMemory, Memory, MemoryContent, MemoryTier, QueryBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize system
    let memory = CosmicMemory::new().await?;

    // Store a memory
    let mem = Memory::new(
        MemoryTier::Session,
        MemoryContent::Text("Important conversation".to_string()),
        vec![0.1, 0.2, 0.3, 0.4],
        0.8, // importance
    );
    memory.store(mem).await?;

    // Query memories
    let query = QueryBuilder::new()
        .session()
        .min_importance(0.5)
        .limit(10)
        .build();

    let results = memory.recall(&query, &[MemoryTier::Session]).await?;

    Ok(())
}
```

### Automatic Consolidation

```rust
// Run automatic consolidation
memory.auto_consolidate().await?;

// Or consolidate specific tiers
memory.consolidate(MemoryTier::Instant, MemoryTier::Session).await?;
```

### Query Builder API

```rust
// Query individual scale (T1-T4)
let query = QueryBuilder::new()
    .individual()
    .text("search term")
    .min_importance(0.5)
    .limit(20)
    .build();

// Query specific tier
let query = QueryBuilder::new()
    .semantic()
    .embedding(vec![0.1, 0.2, 0.3])
    .build();

// Query all tiers
let query = QueryBuilder::new()
    .all_tiers()
    .min_importance(0.7)
    .build();
```

## Memory Lifecycle

1. **Store** - Memories enter at appropriate tier based on importance
2. **Access** - Retrieval updates access count and timestamp
3. **Decay** - Lower tiers experience time-based importance decay
4. **Consolidate** - High-importance memories promote to higher tiers
5. **Prune** - Lower tiers maintain size limits via LRU eviction

## Implementation Status

### ✅ Implemented (Tier 1-4)
- In-memory storage (Instant, Session)
- AgentDB persistence (Episodic, Semantic)
- Vector similarity search
- Automatic pruning and consolidation
- Time decay and relevance scoring

### ⚙️ Stub (Tier 5-12)
- Basic storage structure in place
- Ready for distributed backend integration
- Planned: Vector database, graph storage, knowledge graphs

## Examples

Run the included examples:

```bash
# Basic usage
cargo run --example basic_usage

# Consolidation demo
cargo run --example consolidation
```

## Testing

```bash
cargo test
```

## Integration with ExoGenesis Omega

The Cosmic Memory system integrates with:
- **AgentDB**: Persistent storage for individual tiers
- **Reasoning Engine**: Context retrieval for decision-making
- **Learning System**: Pattern extraction and consolidation
- **Distributed Runtime**: Shared memory across agent swarms

## Future Enhancements

- [ ] Distributed storage for species/cosmic tiers
- [ ] Graph-based knowledge representation
- [ ] Neural encoding/compression
- [ ] Cross-agent memory sharing protocols
- [ ] Quantum-inspired retrieval algorithms
- [ ] Semantic compression for higher tiers

## License

MIT OR Apache-2.0
