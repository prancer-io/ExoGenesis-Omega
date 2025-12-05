# omega-memory - 12-Tier Cosmic Memory System

## Overview

`omega-memory` implements a hierarchical memory system spanning 12 temporal/spatial scales, from milliseconds (Immediate) to the age of the universe (Cosmic). It provides automatic consolidation, multi-tier querying, and integration with AgentDB for vector-based retrieval.

**Key Features:**
- **12 Memory Tiers**: Instant → Short-term → Session → ... → Cosmic
- **Automatic Consolidation**: Promote important memories between tiers
- **Vector-Based Retrieval**: Semantic search across all tiers
- **Time-Based Decay**: Automatic expiration and importance weighting
- **Structured Storage**: Text, Vector, Graph, and Hybrid content types

**Architecture:**
- Individual Memory (Tiers 1-4): Milliseconds to days
- Species Memory (Tiers 5-8): Weeks to centuries
- Cosmic Memory (Tiers 9-12): Millennia to universe age

**Version:** 0.1.0
**Crate:** `omega-memory`
**Location:** `omega/crates/omega-memory`

## Installation

```toml
[dependencies]
omega-memory = "0.1.0"
omega-core = "0.1.0"      # For Memory types
omega-agentdb = "0.1.0"   # For vector storage
```

## Core Concepts

### Memory Tier System

| Tier | Name | Timescale | Retention | Category |
|------|------|-----------|-----------|----------|
| **Individual Memory (T1-T4)** |
| 1 | Instant | 1ms-1s | 1 second | Working memory |
| 2 | Session | 1s-60s | 10 minutes | Recent context |
| 3 | Episodic | 1m-24h | 24 hours | Session data |
| 4 | Semantic | Days | 30 days | Events |
| **Species Memory (T5-T8)** |
| 5 | Collective | Weeks | 1 year | Shared knowledge |
| 6 | Evolutionary | Months | 5 years | Adaptations |
| 7 | Architectural | Years | 50 years | Structures |
| 8 | Substrate | Decades | 500 years | Foundations |
| **Cosmic Memory (T9-T12)** |
| 9 | Civilizational | Centuries | 10K years | Cultural |
| 10 | Temporal | Millennia | Permanent | Historical |
| 11 | Physical | Millions of years | Permanent | Natural laws |
| 12 | Omega | Age of universe | Permanent | Universal |

### Memory Content Types

```rust
pub enum MemoryContent {
    // Simple text
    Sensory(Vec<u8>),
    Text(String),

    // Structured data
    Structured(serde_json::Value),

    // Vector embeddings
    Embedding(Vec<f32>),

    // Multi-modal combination
    MultiModal {
        text: Option<String>,
        embedding: Vec<f32>,
        metadata: serde_json::Value,
    },
}
```

### Memory Metadata

```rust
pub struct Memory {
    pub id: String,
    pub tier: MemoryTier,
    pub content: MemoryContent,
    pub embedding: Vec<f32>,          // For semantic search
    pub importance: f64,              // 0.0 to 1.0
    pub created_at: DateTime<Utc>,
    pub accessed_at: DateTime<Utc>,
    pub access_count: u64,
    pub metadata: serde_json::Value,
}
```

### Relevance Scoring

Memories are ranked by relevance:

```
relevance = importance × time_decay + ln(1 + access_count) × 0.1
```

**Time Decay:**
- Instant (T1): exp(-hours / 0.01)
- Session (T3): exp(-hours / 24)
- Episodic (T4): exp(-hours / 168)  # 1 week
- Semantic (T5+): No decay (permanent)

## API Reference

### Initialization

```rust
use omega_memory::*;

#[tokio::main]
async fn main() -> Result<(), MemoryError> {
    // Create cosmic memory system
    let memory = CosmicMemory::new().await?;

    Ok(())
}
```

### Storing Memories

#### Simple Text Memory

```rust
use omega_memory::*;
use omega_core::MemoryTier;

let memory = Memory::new(
    MemoryTier::Semantic,
    MemoryContent::Text("Rust uses ownership for memory safety".to_string()),
    vec![0.1; 1536], // Embedding from text-embedding-ada-002
    0.9,             // High importance
);

let memory_id = memory_system.store(memory).await?;
```

#### Multi-Modal Memory

```rust
let memory = Memory::new(
    MemoryTier::Episodic,
    MemoryContent::MultiModal {
        text: Some("User asked about async Rust".to_string()),
        embedding: vec![0.1; 1536],
        metadata: serde_json::json!({
            "session_id": "sess-001",
            "user_id": "user-123",
            "timestamp": "2025-01-05T10:00:00Z"
        }),
    },
    vec![0.1; 1536],
    0.7,
);

memory_system.store(memory).await?;
```

#### Structured Memory

```rust
use serde_json::json;

let memory = Memory::new(
    MemoryTier::Collective,
    MemoryContent::Structured(json!({
        "type": "pattern",
        "name": "Builder Pattern",
        "language": "Rust",
        "usage_count": 42,
        "examples": ["TypeBuilder", "ConfigBuilder"]
    })),
    vec![0.1; 1536],
    0.8,
);

memory_system.store(memory).await?;
```

### Querying Memories

#### Simple Query

```rust
use omega_memory::*;

// Query specific tiers
let tiers = vec![
    MemoryTier::Semantic,
    MemoryTier::Episodic,
];

let query = QueryBuilder::new()
    .tiers(tiers)
    .limit(10)
    .build();

let results = memory_system.recall(&query, &query.tiers).await?;

for memory in results {
    println!("Memory: {} (importance: {:.2})",
        memory.id,
        memory.importance
    );
}
```

#### Vector-Based Semantic Query

```rust
let query_embedding = vec![0.1; 1536]; // From user query

let query = QueryBuilder::new()
    .embedding(query_embedding)
    .tiers(vec![MemoryTier::Semantic])
    .min_similarity(0.8)
    .limit(5)
    .build();

let results = memory_system.recall(&query, &query.tiers).await?;
```

#### Advanced Filtering

```rust
use chrono::{Utc, Duration};

let one_week_ago = Utc::now() - Duration::days(7);

let query = QueryBuilder::new()
    .tiers(vec![MemoryTier::Episodic, MemoryTier::Semantic])
    .min_importance(0.7)
    .time_range(one_week_ago, Utc::now())
    .tags(vec!["coding".to_string(), "rust".to_string()])
    .limit(20)
    .build();

let results = memory_system.recall(&query, &query.tiers).await?;
```

### Memory Consolidation

#### Manual Consolidation

```rust
// Promote from Episodic to Semantic
memory_system.consolidate(
    MemoryTier::Episodic,
    MemoryTier::Semantic,
).await?;
```

#### Automatic Consolidation

```rust
// Auto-consolidate based on importance and age
memory_system.auto_consolidate().await?;
```

**Consolidation Rules:**
1. **Importance-based**: importance > 0.8 → promote to higher tier
2. **Access-based**: access_count > 10 → promote
3. **Age-based**: memories near expiration with high importance → promote
4. **Relevance-based**: high relevance_score() → keep, promote

### Memory Statistics

```rust
let stats = memory_system.stats().await;

println!("Individual Memories: {}", stats.individual.total);
println!("Species Memories: {}", stats.species.total);
println!("Cosmic Memories: {}", stats.cosmic.total);
println!("Total: {}", stats.total_memories);

// Per-tier breakdown
println!("Semantic (T5): {}", stats.individual.semantic_count);
println!("Collective (T6): {}", stats.species.collective_count);
```

## Common Patterns

### 1. Session-Based Memory

```rust
async fn store_session_memory(
    memory_system: &CosmicMemory,
    session_id: &str,
    content: String,
    embedding: Vec<f32>,
) -> Result<String, MemoryError> {
    let memory = Memory::new(
        MemoryTier::Session,
        MemoryContent::MultiModal {
            text: Some(content),
            embedding: embedding.clone(),
            metadata: serde_json::json!({
                "session_id": session_id,
                "type": "conversation"
            }),
        },
        embedding,
        0.5, // Medium importance for session data
    );

    memory_system.store(memory).await
}
```

### 2. Importance-Based Tiering

```rust
fn select_tier(importance: f64) -> MemoryTier {
    match importance {
        i if i >= 0.9 => MemoryTier::Semantic,      // Critical facts
        i if i >= 0.7 => MemoryTier::Episodic,      // Important events
        i if i >= 0.5 => MemoryTier::Session,       // Useful context
        _ => MemoryTier::Instant,                   // Temporary data
    }
}
```

### 3. Memory Hierarchy Traversal

```rust
async fn search_all_tiers(
    memory_system: &CosmicMemory,
    query_embedding: Vec<f32>,
) -> Result<Vec<Memory>, MemoryError> {
    let all_tiers = vec![
        // Start with most important
        MemoryTier::Cosmic,
        MemoryTier::Physical,
        MemoryTier::Temporal,
        MemoryTier::Civilizational,
        MemoryTier::Substrate,
        MemoryTier::Architectural,
        MemoryTier::Evolutionary,
        MemoryTier::Collective,
        MemoryTier::Semantic,
        MemoryTier::Episodic,
        MemoryTier::Session,
        MemoryTier::Instant,
    ];

    let query = QueryBuilder::new()
        .embedding(query_embedding)
        .tiers(all_tiers.clone())
        .limit(50)
        .build();

    memory_system.recall(&query, &all_tiers).await
}
```

### 4. Temporal Window Query

```rust
use chrono::{Utc, Duration};

async fn query_recent_memories(
    memory_system: &CosmicMemory,
    hours: i64,
) -> Result<Vec<Memory>, MemoryError> {
    let start = Utc::now() - Duration::hours(hours);
    let end = Utc::now();

    let query = QueryBuilder::new()
        .tiers(vec![MemoryTier::Session, MemoryTier::Episodic])
        .time_range(start, end)
        .limit(100)
        .build();

    memory_system.recall(&query, &query.tiers).await
}
```

### 5. Consolidation Pipeline

```rust
async fn consolidation_pipeline(
    memory_system: &CosmicMemory,
) -> Result<(), MemoryError> {
    // Stage 1: Session → Episodic (high importance)
    memory_system.consolidate(
        MemoryTier::Session,
        MemoryTier::Episodic,
    ).await?;

    // Stage 2: Episodic → Semantic (very high importance)
    memory_system.consolidate(
        MemoryTier::Episodic,
        MemoryTier::Semantic,
    ).await?;

    // Stage 3: Semantic → Collective (critical knowledge)
    memory_system.consolidate(
        MemoryTier::Semantic,
        MemoryTier::Collective,
    ).await?;

    Ok(())
}
```

## Best Practices

### Tier Selection Guidelines

**Instant (T1):**
- Current cursor position
- Temporary calculations
- UI state

**Session (T2-T3):**
- Conversation context
- Current task state
- Working set data

**Episodic (T4):**
- Completed tasks
- User interactions
- Daily events

**Semantic (T5):**
- Learned facts
- Domain knowledge
- Persistent skills

**Collective (T6):**
- Shared knowledge
- Team learnings
- Organizational memory

**Higher Tiers (T7-T12):**
- Use sparingly
- Foundational knowledge only
- Universal constants

### Importance Scoring

**DO:**
- Set importance based on actual value
- Use 0.9-1.0 for critical facts
- Use 0.5-0.7 for normal data
- Use 0.1-0.3 for low-priority data

**DON'T:**
- Default to 1.0 for everything
- Ignore importance (affects consolidation)
- Change importance arbitrarily

### Consolidation Strategy

**DO:**
- Run auto-consolidate periodically (hourly/daily)
- Monitor tier sizes
- Consolidate high-access memories
- Keep consolidation thresholds consistent

**DON'T:**
- Consolidate every memory
- Skip auto-consolidation (memory bloat)
- Promote low-importance memories

### Query Optimization

**DO:**
- Limit tier scope when possible
- Use embeddings for semantic search
- Set appropriate limits (10-50 results)
- Filter by importance when needed

**DON'T:**
- Query all 12 tiers unnecessarily
- Return thousands of results
- Ignore time ranges

## Error Handling

```rust
use omega_memory::MemoryError;

async fn safe_store(
    memory_system: &CosmicMemory,
    memory: Memory,
) -> Result<String, MemoryError> {
    match memory_system.store(memory).await {
        Ok(id) => Ok(id),
        Err(MemoryError::Storage(msg)) => {
            eprintln!("Storage failed: {}", msg);
            Err(MemoryError::Storage(msg))
        }
        Err(e) => Err(e),
    }
}
```

**Error Types:**
- `Storage(String)` - Database storage error
- `Query(String)` - Invalid query
- `Consolidation(String)` - Consolidation failure
- `AgentDB(String)` - Vector DB error

## Performance Considerations

### 1. Batch Operations

```rust
// Good: Batch store
let mut ids = Vec::new();
for memory in memories {
    ids.push(memory_system.store(memory).await?);
}
```

### 2. Tier-Specific Queries

```rust
// Good: Query specific tier
let query = QueryBuilder::new()
    .tiers(vec![MemoryTier::Semantic]) // Only one tier
    .build();

// Avoid: Query all tiers
// let all_tiers = MemoryTier::all_tiers(); // Expensive!
```

### 3. Embedding Cache

```rust
use std::collections::HashMap;

struct EmbeddingCache {
    cache: HashMap<String, Vec<f32>>,
}

impl EmbeddingCache {
    fn get_or_compute(&mut self, text: &str) -> Vec<f32> {
        if let Some(embedding) = self.cache.get(text) {
            return embedding.clone();
        }

        let embedding = compute_embedding(text);
        self.cache.insert(text.to_string(), embedding.clone());
        embedding
    }
}
```

## Integration Examples

### With omega-agentdb

```rust
use omega_agentdb::*;
use omega_memory::*;

// Memory system uses AgentDB internally
let memory = CosmicMemory::new().await?;

// Store memory (automatically indexed in AgentDB)
let memory = Memory::new(/* ... */);
memory.store(memory).await?;
```

### With omega-loops

```rust
use omega_loops::*;
use omega_memory::*;

async fn loop_with_memory(
    loop_engine: &mut LoopEngine,
    memory_system: &CosmicMemory,
) -> Result<(), Box<dyn std::error::Error>> {
    // Store loop outputs in memory
    let input = CycleInput { /* ... */ };
    let output = loop_engine.execute_cycle(LoopType::Adaptive, input).await?;

    let memory = Memory::new(
        MemoryTier::Episodic,
        MemoryContent::Structured(serde_json::to_value(&output)?),
        vec![0.1; 1536],
        0.7,
    );

    memory_system.store(memory).await?;
    Ok(())
}
```

### With omega-runtime

```rust
use omega_runtime::*;

let config = OmegaConfig::default();
let runtime = OmegaRuntime::new(config).await?;

// Memory system is part of runtime
let api = OmegaAPI::new(runtime);
// Access memory via api.store_memory(), api.query_memories(), etc.
```

## Testing

```rust
#[tokio::test]
async fn test_memory_lifecycle() {
    let memory_system = CosmicMemory::new().await.unwrap();

    // Create memory
    let memory = Memory::new(
        MemoryTier::Session,
        MemoryContent::Text("test".to_string()),
        vec![0.1; 128],
        0.5,
    );

    // Store
    let id = memory_system.store(memory).await.unwrap();

    // Query
    let query = QueryBuilder::new()
        .tiers(vec![MemoryTier::Session])
        .build();

    let results = memory_system.recall(&query, &query.tiers).await.unwrap();
    assert!(!results.is_empty());

    // Consolidate
    memory_system.auto_consolidate().await.unwrap();
}
```

## Advanced Topics

### Custom Consolidation Logic

```rust
use omega_memory::*;

async fn custom_consolidation(
    memory_system: &CosmicMemory,
    min_importance: f64,
    min_access_count: u64,
) -> Result<(), MemoryError> {
    // Get all Episodic memories
    let query = QueryBuilder::new()
        .tiers(vec![MemoryTier::Episodic])
        .min_importance(min_importance)
        .build();

    let memories = memory_system.recall(&query, &query.tiers).await?;

    // Promote high-access memories
    for mut memory in memories {
        if memory.access_count > min_access_count {
            memory.tier = MemoryTier::Semantic;
            memory_system.store(memory).await?;
        }
    }

    Ok(())
}
```

### Memory Decay Simulation

```rust
impl Memory {
    pub fn simulate_decay(&self, hours: f64) -> f64 {
        let age_hours = (chrono::Utc::now() - self.created_at).num_hours() as f64;
        let total_hours = age_hours + hours;

        match self.tier {
            MemoryTier::Instant => (-total_hours / 0.01).exp(),
            MemoryTier::Session => (-total_hours / 24.0).exp(),
            MemoryTier::Episodic => (-total_hours / 168.0).exp(),
            _ => 1.0, // No decay for higher tiers
        }
    }
}
```

## References

- **Source**: `omega/crates/omega-memory`
- **Examples**: `omega/crates/omega-memory/examples/`
- **Simulation Results**: `docs/COMPREHENSIVE-SIMULATION-RESULTS.md`
- **Tests**: 228 tests passing, full tier coverage

## Version History

- **0.1.0** (2025-01-05): Initial release
  - 12-tier memory system
  - Automatic consolidation
  - Vector-based querying
  - AgentDB integration
