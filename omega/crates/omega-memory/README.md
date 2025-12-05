# omega-memory

[![Crates.io](https://img.shields.io/crates/v/omega-memory)](https://crates.io/crates/omega-memory)
[![Documentation](https://docs.rs/omega-memory/badge.svg)](https://docs.rs/omega-memory)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

12-tier cosmic memory system with automatic consolidation spanning from instant (milliseconds) to omega (universal) timescales.

## Overview

`omega-memory` implements a revolutionary hierarchical memory architecture that spans 12 temporal tiers—from immediate sensory memory lasting milliseconds to cosmic memory operating at the scale of the universe's lifetime. The system automatically consolidates memories between tiers based on importance, recency, and access patterns, mimicking biological memory consolidation.

This architecture enables AI systems to operate coherently across vastly different timescales, from real-time interactions to multi-generational learning, while efficiently managing memory resources.

## Features

- **12 Memory Tiers**: Instant, Session, Episodic, Semantic, Collective, Evolutionary, Architectural, Substrate, Civilizational, Temporal, Physical, Omega
- **Automatic Consolidation**: Intelligent migration of memories between tiers
- **Time Decay**: Tier-appropriate decay functions for memory relevance
- **Multi-Modal Storage**: Text, embeddings, structured data, sensory data
- **Access Tracking**: Automatic tracking of memory access patterns
- **Importance Scoring**: Combined importance + recency + frequency scoring
- **Async-First**: Full Tokio support for concurrent memory operations
- **Type-Safe Queries**: Strongly typed query builder with filters

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
omega-memory = "0.1.0"
```

## Quick Start

```rust
use omega_memory::{CosmicMemory, Memory, MemoryTier, MemoryContent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the cosmic memory system
    let memory = CosmicMemory::new().await?;

    // Store a semantic memory (knowledge)
    let mem = Memory::new(
        MemoryTier::Semantic,
        MemoryContent::Text("Rust uses ownership for memory safety".to_string()),
        vec![0.1; 768], // embedding vector
        0.9, // high importance
    );

    let id = memory.store(mem).await?;

    // Query memories
    let query = QueryBuilder::new()
        .with_embedding(vec![0.1; 768])
        .with_min_importance(0.8)
        .build();

    let results = memory.recall(&query, &[MemoryTier::Semantic]).await?;

    println!("Found {} relevant memories", results.len());

    // Automatic consolidation
    memory.auto_consolidate().await?;

    Ok(())
}
```

## Core Concepts

### 12-Tier Memory Hierarchy

The memory system is organized into three scales:

**Individual Scale (Tiers 1-4)**:
1. **Instant** (milliseconds) - Sensory buffers, reflexive responses
2. **Session** (hours) - Working memory, current context
3. **Episodic** (days) - Event memories, experiences
4. **Semantic** (weeks) - Factual knowledge, concepts

**Species Scale (Tiers 5-8)**:
5. **Collective** (months) - Shared knowledge, culture
6. **Evolutionary** (years) - Learned behaviors, adaptations
7. **Architectural** (decades) - Structural patterns, designs
8. **Substrate** (centuries) - Fundamental principles

**Cosmic Scale (Tiers 9-12)**:
9. **Civilizational** (millennia) - Cultural knowledge
10. **Temporal** (millions of years) - Temporal patterns
11. **Physical** (billions of years) - Physical laws
12. **Omega** (age of universe) - Universal constants

### Memory Consolidation

Memories automatically migrate between tiers based on:

- **Importance**: High-importance memories consolidate faster
- **Access Frequency**: Frequently accessed memories are retained
- **Recency**: Recent access prevents decay
- **Tier Policies**: Each tier has specific consolidation rules

### Memory Content Types

```rust
pub enum MemoryContent {
    Sensory(Vec<u8>),              // Raw sensory data
    Text(String),                   // Text-based memory
    Structured(serde_json::Value),  // JSON structures
    Embedding(Vec<f32>),            // Vector embeddings
    MultiModal {                    // Combined modalities
        text: Option<String>,
        embedding: Vec<f32>,
        metadata: serde_json::Value,
    },
}
```

### Relevance Scoring

Memory relevance is computed as:

```
relevance = (importance × time_decay) + access_boost
```

where:
- `importance`: Base importance score (0.0-1.0)
- `time_decay`: Tier-specific exponential decay
- `access_boost`: Logarithmic boost from access frequency

## Use Cases

### 1. Multi-Scale Agent Memory

```rust
use omega_memory::{CosmicMemory, Memory, MemoryTier, MemoryContent};

let memory = CosmicMemory::new().await?;

// Store immediate sensory data
let sensory = Memory::new(
    MemoryTier::Instant,
    MemoryContent::Sensory(vec![255, 128, 64]),
    vec![],
    0.3,
);
memory.store(sensory).await?;

// Store episodic event
let event = Memory::new(
    MemoryTier::Episodic,
    MemoryContent::Text("User requested feature X at 10:30 AM".to_string()),
    get_embedding("user feature request"),
    0.7,
);
memory.store(event).await?;

// Store semantic knowledge
let knowledge = Memory::new(
    MemoryTier::Semantic,
    MemoryContent::Structured(serde_json::json!({
        "concept": "REST API",
        "definition": "Representational State Transfer interface",
        "examples": ["GET /users", "POST /items"]
    })),
    get_embedding("REST API concept"),
    0.9,
);
memory.store(knowledge).await?;
```

### 2. Knowledge Base with Automatic Pruning

```rust
use omega_memory::{CosmicMemory, Memory, MemoryTier};

let memory = CosmicMemory::new().await?;

// Store many facts
for fact in facts {
    let mem = Memory::new(
        MemoryTier::Semantic,
        MemoryContent::Text(fact.clone()),
        fact.embedding.clone(),
        fact.importance,
    );
    memory.store(mem).await?;
}

// Low-importance, rarely accessed memories automatically decay
// High-importance, frequently accessed memories consolidate to higher tiers
memory.auto_consolidate().await?;

// Query returns only relevant, non-expired memories
let query = QueryBuilder::new()
    .with_text("machine learning")
    .with_min_importance(0.5)
    .build();

let results = memory.recall(&query, &[MemoryTier::Semantic]).await?;
```

### 3. Long-Term Learning System

```rust
use omega_memory::{CosmicMemory, MemoryTier};

let memory = CosmicMemory::new().await?;

// Short-term learning (session tier)
for observation in recent_observations {
    memory.store(observation.to_memory(MemoryTier::Session)).await?;
}

// Consolidate important patterns to semantic tier
memory.consolidate(
    MemoryTier::Session,
    MemoryTier::Semantic
).await?;

// Over time, foundational knowledge reaches evolutionary tier
memory.consolidate(
    MemoryTier::Semantic,
    MemoryTier::Evolutionary
).await?;

// Architectural patterns emerge at higher tiers
let architectural_memories = memory.recall(
    &Query::all(),
    &[MemoryTier::Architectural]
).await?;
```

### 4. Multi-Agent Shared Memory

```rust
use omega_memory::{CosmicMemory, Memory, MemoryTier, MemoryContent};

let collective_memory = CosmicMemory::new().await?;

// Individual agent stores to collective tier
async fn agent_contribute(memory: &CosmicMemory, knowledge: String) {
    let mem = Memory::new(
        MemoryTier::Collective,
        MemoryContent::Text(knowledge),
        get_embedding(&knowledge),
        0.8,
    );
    memory.store(mem).await.unwrap();
}

// All agents can query collective knowledge
let query = QueryBuilder::new()
    .with_text("best practices")
    .build();

let shared_knowledge = collective_memory.recall(
    &query,
    &[MemoryTier::Collective]
).await?;
```

### 5. Hierarchical Recall Across Tiers

```rust
use omega_memory::{CosmicMemory, QueryBuilder, MemoryTier};

let memory = CosmicMemory::new().await?;

// Build query with embedding
let query = QueryBuilder::new()
    .with_embedding(query_vector)
    .with_min_importance(0.7)
    .with_max_results(20)
    .build();

// Search across multiple tiers simultaneously
let results = memory.recall(
    &query,
    &[
        MemoryTier::Session,
        MemoryTier::Episodic,
        MemoryTier::Semantic,
        MemoryTier::Collective,
    ]
).await?;

// Results are automatically sorted by relevance
for mem in results {
    println!("Tier {:?}: {} (relevance: {:.3})",
        mem.tier,
        mem.content,
        mem.relevance_score()
    );
}
```

## Examples

### Memory Statistics

```rust
let memory = CosmicMemory::new().await?;

// Store various memories...

let stats = memory.stats().await;
println!("Memory Statistics:");
println!("  Individual tiers: {} memories", stats.individual.total);
println!("  Species tiers: {} memories", stats.species.total);
println!("  Cosmic tiers: {} memories", stats.cosmic.total);
println!("  Total: {} memories", stats.total_memories);
```

### Custom Consolidation Logic

```rust
use omega_memory::{CosmicMemory, MemoryTier};

let memory = CosmicMemory::new().await?;

// Consolidate session → episodic for memories > 1 hour old
memory.consolidate(
    MemoryTier::Session,
    MemoryTier::Episodic
).await?;

// Consolidate episodic → semantic for important, old memories
memory.consolidate(
    MemoryTier::Episodic,
    MemoryTier::Semantic
).await?;

// Or use automatic consolidation with built-in heuristics
memory.auto_consolidate().await?;
```

## Architecture

The memory system is structured in three layers:

```
┌─────────────────────────────────────────┐
│          CosmicMemory (API)              │
│  - Unified interface                     │
│  - Query routing                         │
│  - Consolidation orchestration           │
└────────┬────────────┬────────────┬───────┘
         │            │            │
         ▼            ▼            ▼
┌────────────┐┌────────────┐┌────────────┐
│ Individual ││  Species   ││  Cosmic    │
│ Memory     ││  Memory    ││  Memory    │
│ (1-4)      ││  (5-8)     ││  (9-12)    │
└────────┬───┘└─────┬──────┘└──────┬─────┘
         │          │              │
         ▼          ▼              ▼
┌─────────────────────────────────────────┐
│        Memory Consolidator               │
│  - Importance-based migration            │
│  - Time decay application                │
│  - Access pattern analysis               │
└─────────────────────────────────────────┘
```

## Performance

Memory system performance characteristics:

- **Store**: O(1) - Constant time insertion
- **Recall**: O(log n) - Logarithmic search with indexes
- **Consolidation**: O(n) - Linear scan with filtering
- **Memory Usage**: ~200 bytes per memory + embedding size

### Optimization Strategies

1. **Tier Separation**: Keeps hot (recent) and cold (old) data separate
2. **Lazy Consolidation**: Only consolidates when needed
3. **Access Tracking**: Minimal overhead with atomic counters
4. **Embedding Compression**: Optional quantization for large datasets

## Related Crates

- **[omega-core](../omega-core)** - Core memory types and traits
- **[omega-agentdb](../omega-agentdb)** - Vector search backend
- **[omega-persistence](../omega-persistence)** - SQLite storage layer
- **[omega-loops](../omega-loops)** - Temporal loop integration
- **[omega-meta-sona](../omega-meta-sona)** - Architecture evolution
- **[omega-runtime](../omega-runtime)** - Runtime orchestration

## License

Licensed under the MIT License. See [LICENSE](../../LICENSE) for details.
