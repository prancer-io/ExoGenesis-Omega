# omega-persistence

[![Crates.io](https://img.shields.io/crates/v/omega-persistence)](https://crates.io/crates/omega-persistence)
[![Documentation](https://docs.rs/omega-persistence/badge.svg)](https://docs.rs/omega-persistence)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

SQLite-based persistence layer for ExoGenesis Omega with schema migrations and transactional storage.

## Overview

`omega-persistence` provides durable, high-performance storage for all ExoGenesis Omega components. Built on SQLite with bundled native library support, it offers zero-dependency deployment with ACID guarantees for memories, skills, architectures, intelligences, causal graphs, and vector embeddings.

The crate implements a carefully designed schema with proper foreign key constraints, indexes for common queries, and support for backup/restore operations. All operations are transactional and type-safe.

## Features

- **Hierarchical Memory Storage**: Store memories across all 12 tiers with importance tracking
- **Skill Management**: Track learned skills with usage statistics and pattern matching
- **Architecture Versioning**: Store evolved architectures with full lineage tracking
- **Intelligence Lifecycle**: Persist intelligence instances with state and capabilities
- **Causal Graph Storage**: Build and query causal relationships between memories
- **Vector Embeddings**: Store high-dimensional vectors with memory associations
- **Reflexion Episodes**: Capture agent learning episodes with context
- **Transactional Operations**: Full ACID guarantees for all mutations
- **Backup & Restore**: Built-in support for database snapshots

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
omega-persistence = "0.1.0"
```

## Quick Start

```rust
use omega_persistence::{OmegaStore, StoredMemory};
use chrono::Utc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create database (file-based or in-memory)
    let store = OmegaStore::new("omega.db")?;
    // or for testing:
    // let store = OmegaStore::new_in_memory()?;

    // Store a memory
    let memory = StoredMemory {
        id: "mem-001".to_string(),
        content: "Important discovery about neural architectures".to_string(),
        tier: 4, // Semantic tier
        importance: 0.95,
        embedding_blob: None,
        created_at: Utc::now().timestamp(),
        last_accessed: Utc::now().timestamp(),
    };

    store.store_memory(&memory)?;

    // Retrieve it
    let retrieved = store.get_memory("mem-001")?;
    println!("Retrieved: {}", retrieved.content);

    // Query by tier
    let semantic_memories = store.query_memories_by_tier(4)?;
    println!("Found {} semantic memories", semantic_memories.len());

    Ok(())
}
```

## Core Concepts

### OmegaStore

The main interface to the persistence layer. Provides methods for:

- Creating/opening databases
- Storing and retrieving all entity types
- Querying with filters
- Backup and restore operations
- Database statistics

### Stored Entity Types

All entities are strongly typed with validation:

- **StoredMemory**: Memory with tier, importance, and optional embedding
- **StoredSkill**: Learned skill with trigger pattern and success tracking
- **StoredArchitecture**: Cognitive architecture with paradigm and fitness
- **StoredIntelligence**: Intelligence instance with capabilities and state
- **StoredVector**: High-dimensional embedding with dimension tracking
- **StoredReflexionEpisode**: Learning episode with trigger/action/outcome
- **StoredCausalEdge**: Directed causal relationship with weight

### Schema Design

The database schema uses proper normalization with:

- Foreign key constraints for referential integrity
- Indexes on commonly queried columns
- JSON storage for flexible metadata
- BLOB storage for vector embeddings
- Timestamp tracking for all entities

## Use Cases

### 1. Memory Management

```rust
use omega_persistence::{OmegaStore, StoredMemory};
use chrono::Utc;

let store = OmegaStore::new("memory.db")?;
let now = Utc::now().timestamp();

// Store memories across tiers
for tier in 1..=12 {
    let memory = StoredMemory {
        id: format!("mem-tier-{}", tier),
        content: format!("Memory at tier {}", tier),
        tier,
        importance: 0.5 + (tier as f64 * 0.04),
        embedding_blob: None,
        created_at: now,
        last_accessed: now,
    };
    store.store_memory(&memory)?;
}

// Query by tier with automatic importance ordering
let important_semantic = store.query_memories_by_tier(4)?;
for mem in important_semantic {
    println!("Importance {}: {}", mem.importance, mem.content);
}
```

### 2. Skill Tracking

```rust
use omega_persistence::{OmegaStore, StoredSkill};
use chrono::Utc;

let store = OmegaStore::new("skills.db")?;
let now = Utc::now().timestamp();

// Store a learned skill
let skill = StoredSkill {
    id: "skill-001".to_string(),
    name: "code_review".to_string(),
    description: "Perform thorough code review with best practices".to_string(),
    trigger_pattern: "review.*code|code.*review".to_string(),
    success_count: 0,
    last_used: None,
    created_at: now,
};

store.store_skill(&skill)?;

// Increment success on use
store.increment_skill_success("skill-001", Utc::now().timestamp())?;

// Find skills by pattern
let code_skills = store.get_skills_by_pattern("code")?;
println!("Found {} code-related skills", code_skills.len());
```

### 3. Architecture Evolution

```rust
use omega_persistence::{OmegaStore, StoredArchitecture};
use chrono::Utc;

let store = OmegaStore::new("architectures.db")?;
let now = Utc::now().timestamp();

// Store base architecture
let base = StoredArchitecture {
    id: "arch-gen-0".to_string(),
    name: "Transformer".to_string(),
    paradigm: "neural".to_string(),
    substrate: "pytorch".to_string(),
    fitness_json: r#"{"overall": 0.75, "capability": 0.80}"#.to_string(),
    lineage_json: r#"{"generation": 0, "parent": null}"#.to_string(),
    created_at: now,
};

store.store_architecture(&base)?;

// Store evolved version
let evolved = StoredArchitecture {
    id: "arch-gen-1".to_string(),
    name: "Transformer-v2".to_string(),
    paradigm: "neural".to_string(),
    substrate: "pytorch".to_string(),
    fitness_json: r#"{"overall": 0.85, "capability": 0.90}"#.to_string(),
    lineage_json: r#"{"generation": 1, "parent": "arch-gen-0"}"#.to_string(),
    created_at: now + 3600,
};

store.store_architecture(&evolved)?;

// Query by paradigm
let neural_archs = store.get_architectures_by_paradigm("neural")?;
```

### 4. Causal Graph Construction

```rust
use omega_persistence::{OmegaStore, StoredMemory, StoredCausalEdge};
use chrono::Utc;

let store = OmegaStore::new("causal.db")?;
let now = Utc::now().timestamp();

// Store cause and effect memories
let cause = StoredMemory {
    id: "mem-cause".to_string(),
    content: "Implemented caching layer".to_string(),
    tier: 4,
    importance: 0.8,
    embedding_blob: None,
    created_at: now,
    last_accessed: now,
};

let effect = StoredMemory {
    id: "mem-effect".to_string(),
    content: "Response time improved 10x".to_string(),
    tier: 4,
    importance: 0.9,
    embedding_blob: None,
    created_at: now + 3600,
    last_accessed: now + 3600,
};

store.store_memory(&cause)?;
store.store_memory(&effect)?;

// Create causal edge
let edge = StoredCausalEdge {
    id: "edge-001".to_string(),
    from_memory: "mem-cause".to_string(),
    to_memory: "mem-effect".to_string(),
    weight: 0.95,
    edge_type: "improvement".to_string(),
    created_at: now + 3600,
};

store.store_causal_edge(&edge)?;

// Query causal relationships
let effects = store.get_causal_edges_from("mem-cause")?;
```

### 5. Intelligence Persistence

```rust
use omega_persistence::{OmegaStore, StoredArchitecture, StoredIntelligence};
use chrono::Utc;

let store = OmegaStore::new("intelligence.db")?;
let now = Utc::now().timestamp();

// First create architecture
let arch = StoredArchitecture {
    id: "arch-hybrid".to_string(),
    name: "HybridCognitive".to_string(),
    paradigm: "hybrid".to_string(),
    substrate: "rust".to_string(),
    fitness_json: "{}".to_string(),
    lineage_json: "{}".to_string(),
    created_at: now,
};

store.store_architecture(&arch)?;

// Create intelligence instance
let intel = StoredIntelligence {
    id: "intel-001".to_string(),
    name: "Alpha".to_string(),
    arch_id: "arch-hybrid".to_string(),
    maturity: 0.75,
    capabilities_json: r#"["reasoning", "learning", "planning"]"#.to_string(),
    memories_json: r#"["mem-001", "mem-002"]"#.to_string(),
    state_json: r#"{"status": "active", "mode": "production"}"#.to_string(),
    created_at: now,
    updated_at: now,
};

store.store_intelligence(&intel)?;

// Query intelligences by architecture
let instances = store.get_intelligences_by_arch("arch-hybrid")?;
```

## Examples

### Database Backup and Restore

```rust
use omega_persistence::OmegaStore;

let store = OmegaStore::new("production.db")?;

// ... store data ...

// Create backup
store.backup("backup-2024-01-15.db")?;

// Later, open backup to verify
let backup_store = OmegaStore::new("backup-2024-01-15.db")?;
let stats = backup_store.get_statistics()?;
println!("Backup contains {} memories", stats.memory_count);
```

### Database Statistics

```rust
use omega_persistence::OmegaStore;

let store = OmegaStore::new("omega.db")?;
let stats = store.get_statistics()?;

println!("Database Statistics:");
println!("  Memories: {}", stats.memory_count);
println!("  Skills: {}", stats.skill_count);
println!("  Architectures: {}", stats.architecture_count);
println!("  Intelligences: {}", stats.intelligence_count);
println!("  Vectors: {}", stats.vector_count);
println!("  Reflexions: {}", stats.reflexion_count);
println!("  Causal Edges: {}", stats.causal_edge_count);
```

## Architecture

`omega-persistence` provides the storage foundation for the Omega ecosystem:

```
┌──────────────────────────────────────┐
│       Higher-level Crates            │
│  (omega-memory, omega-agentdb, etc)  │
└────────────────┬─────────────────────┘
                 │
                 ▼
┌──────────────────────────────────────┐
│       omega-persistence               │
│  - OmegaStore                         │
│  - Schema Management                  │
│  - Transaction Handling               │
└────────────────┬─────────────────────┘
                 │
                 ▼
┌──────────────────────────────────────┐
│         rusqlite (SQLite)             │
│  - ACID transactions                  │
│  - Bundled library                    │
│  - Zero external dependencies         │
└──────────────────────────────────────┘
```

## Performance

- **Fast Queries**: Indexed columns for common access patterns
- **Batch Operations**: Transactional bulk inserts
- **Bundled SQLite**: No external database setup required
- **Memory Efficient**: Streaming results for large datasets
- **Optimized Schema**: Proper normalization and denormalization balance

Key performance characteristics:

- Memory lookup by ID: O(1) with index
- Query by tier: O(n log n) with index + sort
- Skill pattern search: O(n) with full-text capabilities
- Causal graph queries: O(k) where k = edge count

## Related Crates

- **[omega-core](../omega-core)** - Core types and traits
- **[omega-agentdb](../omega-agentdb)** - Vector database for semantic search
- **[omega-memory](../omega-memory)** - 12-tier memory system (uses persistence)
- **[omega-loops](../omega-loops)** - Temporal loop orchestration
- **[omega-meta-sona](../omega-meta-sona)** - Architecture evolution
- **[omega-runtime](../omega-runtime)** - Production runtime

## License

Licensed under the MIT License. See [LICENSE](../../LICENSE) for details.
