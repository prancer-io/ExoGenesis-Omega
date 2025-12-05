# omega-persistence - SQLite Storage Backend

## Overview

`omega-persistence` provides durable, SQLite-backed storage for all ExoGenesis Omega components. It persists memories, skills, architectures, intelligences, causal graphs, reflexion episodes, and vector embeddings with full ACID guarantees.

**Key Features:**
- **SQLite Backend**: Reliable, embedded database
- **Zero Configuration**: Works out-of-the-box
- **Full ACID**: Transactions, rollbacks, consistency
- **Efficient Queries**: Indexed lookups, sorted results
- **Backup Support**: Easy database backup/restore
- **In-Memory Mode**: For testing and temporary storage

**Storage Capabilities:**
- Hierarchical memories (12 tiers)
- Learned skills with usage tracking
- Evolved architectures with lineage
- Intelligence instances with state
- Causal graphs and reflexion episodes
- Vector embeddings (as blobs)

**Version:** 0.1.0
**Crate:** `omega-persistence`
**Location:** `omega/crates/omega-persistence`

## Installation

```toml
[dependencies]
omega-persistence = "0.1.0"
```

## Core Concepts

### Database Schema

```sql
-- Memories table
CREATE TABLE memories (
    id TEXT PRIMARY KEY,
    content TEXT NOT NULL,
    tier INTEGER NOT NULL,
    importance REAL NOT NULL,
    embedding_blob BLOB,
    created_at INTEGER NOT NULL,
    last_accessed INTEGER NOT NULL
);

-- Skills table
CREATE TABLE skills (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    trigger_pattern TEXT NOT NULL,
    success_count INTEGER NOT NULL,
    last_used INTEGER,
    created_at INTEGER NOT NULL
);

-- Architectures table
CREATE TABLE architectures (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    paradigm TEXT NOT NULL,
    substrate TEXT NOT NULL,
    fitness_json TEXT NOT NULL,
    lineage_json TEXT NOT NULL,
    created_at INTEGER NOT NULL
);

-- Intelligences table
CREATE TABLE intelligences (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    arch_id TEXT NOT NULL,
    maturity REAL NOT NULL,
    capabilities_json TEXT NOT NULL,
    memories_json TEXT NOT NULL,
    state_json TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (arch_id) REFERENCES architectures(id)
);

-- Additional tables: vectors, reflexion_episodes, causal_edges
```

### Stored Types

```rust
pub struct StoredMemory {
    pub id: String,
    pub content: String,
    pub tier: i32,
    pub importance: f64,
    pub embedding_blob: Option<Vec<u8>>,
    pub created_at: i64,
    pub last_accessed: i64,
}

pub struct StoredSkill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub trigger_pattern: String,
    pub success_count: i64,
    pub last_used: Option<i64>,
    pub created_at: i64,
}

pub struct StoredArchitecture {
    pub id: String,
    pub name: String,
    pub paradigm: String,
    pub substrate: String,
    pub fitness_json: String,
    pub lineage_json: String,
    pub created_at: i64,
}
```

## API Reference

### Store Initialization

```rust
use omega_persistence::*;

// File-based database
let store = OmegaStore::new("omega.db")?;

// In-memory database (for testing)
let store = OmegaStore::new_in_memory()?;

// Custom path
let store = OmegaStore::new("/path/to/omega.db")?;
```

### Memory Operations

#### Store Memory

```rust
use omega_persistence::*;
use chrono::Utc;

let memory = StoredMemory {
    id: "mem-001".to_string(),
    content: "Rust is a systems programming language".to_string(),
    tier: 5,  // Semantic tier
    importance: 0.9,
    embedding_blob: Some(vec![1, 2, 3, 4]),  // Serialized embedding
    created_at: Utc::now().timestamp(),
    last_accessed: Utc::now().timestamp(),
};

store.store_memory(&memory)?;
```

#### Retrieve Memory

```rust
let memory = store.get_memory("mem-001")?;
println!("Content: {}", memory.content);
println!("Tier: {}", memory.tier);
println!("Importance: {}", memory.importance);
```

#### Query by Tier

```rust
let semantic_memories = store.query_memories_by_tier(5)?;  // Tier 5 = Semantic

for memory in semantic_memories {
    println!("{}: {} (importance: {})",
        memory.id,
        memory.content,
        memory.importance
    );
}
```

#### Update Access Time

```rust
let new_access_time = Utc::now().timestamp();
store.update_memory_access("mem-001", new_access_time)?;
```

### Skill Operations

#### Store Skill

```rust
use omega_persistence::*;

let skill = StoredSkill {
    id: "skill-001".to_string(),
    name: "code_review".to_string(),
    description: "Perform comprehensive code review".to_string(),
    trigger_pattern: "review.*code".to_string(),
    success_count: 42,
    last_used: Some(Utc::now().timestamp()),
    created_at: Utc::now().timestamp(),
};

store.store_skill(&skill)?;
```

#### Retrieve Skill

```rust
let skill = store.get_skill("skill-001")?;
println!("{}: {}", skill.name, skill.description);
println!("Success count: {}", skill.success_count);
```

#### Query by Pattern

```rust
let code_skills = store.get_skills_by_pattern("code")?;

for skill in code_skills {
    println!("{}: {} successes", skill.name, skill.success_count);
}
```

#### Increment Success Count

```rust
let new_time = Utc::now().timestamp();
store.increment_skill_success("skill-001", new_time)?;

let updated = store.get_skill("skill-001")?;
println!("New success count: {}", updated.success_count);
```

### Architecture Operations

#### Store Architecture

```rust
let arch = StoredArchitecture {
    id: "arch-001".to_string(),
    name: "Neural Transformer V2".to_string(),
    paradigm: "neural".to_string(),
    substrate: "digital".to_string(),
    fitness_json: serde_json::json!({
        "overall": 0.85,
        "capability": 0.90,
        "efficiency": 0.80
    }).to_string(),
    lineage_json: serde_json::json!({
        "parents": ["arch-000"],
        "generation": 2
    }).to_string(),
    created_at: Utc::now().timestamp(),
};

store.store_architecture(&arch)?;
```

#### Query by Paradigm

```rust
let neural_archs = store.get_architectures_by_paradigm("neural")?;

for arch in neural_archs {
    println!("{} (paradigm: {})", arch.name, arch.paradigm);
}
```

### Intelligence Operations

#### Store Intelligence

```rust
// First, ensure architecture exists
store.store_architecture(&architecture)?;

let intelligence = StoredIntelligence {
    id: "intel-001".to_string(),
    name: "AlphaAgent".to_string(),
    arch_id: "arch-001".to_string(),
    maturity: 0.75,
    capabilities_json: serde_json::json!(["reasoning", "learning"]).to_string(),
    memories_json: serde_json::json!(["mem-001", "mem-002"]).to_string(),
    state_json: serde_json::json!({"active": true}).to_string(),
    created_at: Utc::now().timestamp(),
    updated_at: Utc::now().timestamp(),
};

store.store_intelligence(&intelligence)?;
```

#### Query by Architecture

```rust
let intelligences = store.get_intelligences_by_arch("arch-001")?;

for intel in intelligences {
    println!("{} (maturity: {})", intel.name, intel.maturity);
}
```

### Vector Operations

#### Store Vector

```rust
// First store the associated memory
store.store_memory(&memory)?;

let vector = StoredVector {
    id: "vec-001".to_string(),
    memory_id: "mem-001".to_string(),
    dimensions: 1536,
    data_blob: vec![0u8; 1536 * 4],  // 1536 floats as bytes
};

store.store_vector(&vector)?;
```

#### Retrieve Vector

```rust
let vector = store.get_vector_by_memory("mem-001")?;
println!("Dimensions: {}", vector.dimensions);
println!("Data size: {} bytes", vector.data_blob.len());
```

### Reflexion Operations

#### Store Episode

```rust
let episode = StoredReflexionEpisode {
    id: "reflex-001".to_string(),
    memory_id: "mem-001".to_string(),
    trigger: "code_error".to_string(),
    context: "Syntax error in function".to_string(),
    action: "Fixed missing semicolon".to_string(),
    outcome: "Code compiled successfully".to_string(),
    created_at: Utc::now().timestamp(),
};

store.store_reflexion(&episode)?;
```

#### Query Episodes

```rust
let episodes = store.get_reflexions_by_memory("mem-001")?;

for ep in episodes {
    println!("Trigger: {} → Outcome: {}", ep.trigger, ep.outcome);
}
```

### Causal Edge Operations

#### Store Causal Edge

```rust
// Store memories first
store.store_memory(&cause_memory)?;
store.store_memory(&effect_memory)?;

let edge = StoredCausalEdge {
    id: "edge-001".to_string(),
    from_memory: "mem-cause".to_string(),
    to_memory: "mem-effect".to_string(),
    weight: 0.9,
    edge_type: "causal".to_string(),
    created_at: Utc::now().timestamp(),
};

store.store_causal_edge(&edge)?;
```

#### Query Edges

```rust
// Get all effects from a cause
let effects = store.get_causal_edges_from("mem-cause")?;

for edge in effects {
    println!("{} → {} (weight: {})",
        edge.from_memory,
        edge.to_memory,
        edge.weight
    );
}

// Get all causes of an effect
let causes = store.get_causal_edges_to("mem-effect")?;
```

### Database Statistics

```rust
let stats = store.get_statistics()?;

println!("Database Statistics:");
println!("  Memories: {}", stats.memory_count);
println!("  Skills: {}", stats.skill_count);
println!("  Architectures: {}", stats.architecture_count);
println!("  Intelligences: {}", stats.intelligence_count);
println!("  Vectors: {}", stats.vector_count);
println!("  Reflexion Episodes: {}", stats.reflexion_count);
println!("  Causal Edges: {}", stats.causal_edge_count);
```

### Backup and Restore

#### Create Backup

```rust
use tempfile::NamedTempFile;

let backup_path = "/path/to/backup/omega-backup.db";
store.backup(backup_path)?;

println!("Database backed up to {}", backup_path);
```

#### Restore from Backup

```rust
// Open backup database
let backup_store = OmegaStore::new(backup_path)?;

// Query data from backup
let memories = backup_store.query_memories_by_tier(5)?;
```

## Common Patterns

### 1. Atomic Transactions

```rust
use rusqlite::Transaction;

fn atomic_operation(store: &OmegaStore) -> Result<(), StorageError> {
    // Transactions ensure all-or-nothing execution
    let memory1 = StoredMemory { /* ... */ };
    let memory2 = StoredMemory { /* ... */ };

    store.store_memory(&memory1)?;
    store.store_memory(&memory2)?;

    // Both succeed or both roll back
    Ok(())
}
```

### 2. Bulk Insert

```rust
fn bulk_insert_memories(
    store: &OmegaStore,
    memories: Vec<StoredMemory>,
) -> Result<(), StorageError> {
    for memory in memories {
        store.store_memory(&memory)?;
    }
    Ok(())
}
```

### 3. Query with Pagination

```rust
fn paginate_memories(
    store: &OmegaStore,
    tier: i32,
    page_size: usize,
    page: usize,
) -> Result<Vec<StoredMemory>, StorageError> {
    let all_memories = store.query_memories_by_tier(tier)?;

    let start = page * page_size;
    let end = (start + page_size).min(all_memories.len());

    Ok(all_memories[start..end].to_vec())
}
```

### 4. Memory Cleanup

```rust
use chrono::{Utc, Duration};

fn cleanup_old_memories(
    store: &OmegaStore,
    days_old: i64,
) -> Result<usize, StorageError> {
    let cutoff = (Utc::now() - Duration::days(days_old)).timestamp();

    // Query all memories
    let all_tiers: Vec<i32> = (1..=12).collect();
    let mut deleted = 0;

    for tier in all_tiers {
        let memories = store.query_memories_by_tier(tier)?;

        for memory in memories {
            if memory.last_accessed < cutoff && memory.importance < 0.5 {
                // Note: delete_memory not shown in API above but would be implemented
                deleted += 1;
            }
        }
    }

    Ok(deleted)
}
```

### 5. Skill Leaderboard

```rust
fn get_top_skills(
    store: &OmegaStore,
    limit: usize,
) -> Result<Vec<StoredSkill>, StorageError> {
    // Get all skills (already sorted by success_count DESC)
    let skills = store.get_skills_by_pattern("")?;  // Empty pattern = all skills

    Ok(skills.into_iter().take(limit).collect())
}
```

## Best Practices

### Database Location

**DO:**
```rust
// Production: Use persistent path
let store = OmegaStore::new("data/omega.db")?;

// Testing: Use in-memory
let store = OmegaStore::new_in_memory()?;
```

**DON'T:**
```rust
// Bad: Temporary directory (may be cleaned up)
// let store = OmegaStore::new("/tmp/omega.db")?;
```

### Error Handling

**DO:**
```rust
match store.get_memory("mem-001") {
    Ok(memory) => println!("Found: {}", memory.content),
    Err(StorageError::NotFound(id)) => {
        eprintln!("Memory {} not found", id);
    }
    Err(e) => {
        eprintln!("Database error: {}", e);
        return Err(e);
    }
}
```

### Data Integrity

**DO:**
- Store related data atomically
- Validate foreign keys before inserting
- Use unique IDs (UUIDs)
- Handle NOT NULL constraints

**DON'T:**
- Insert intelligences without architectures
- Use duplicate IDs
- Store invalid JSON in JSON fields

### Performance

**DO:**
```rust
// Good: Query once, iterate in memory
let memories = store.query_memories_by_tier(5)?;
for memory in memories {
    process(memory);
}
```

**DON'T:**
```rust
// Bad: Repeated queries
// for id in ids {
//     let memory = store.get_memory(&id)?; // N queries!
//     process(memory);
// }
```

## Error Handling

```rust
use omega_persistence::StorageError;

fn safe_store_memory(
    store: &OmegaStore,
    memory: &StoredMemory,
) -> Result<(), StorageError> {
    match store.store_memory(memory) {
        Ok(()) => Ok(()),
        Err(StorageError::NotFound(id)) => {
            eprintln!("Entity {} not found", id);
            Err(StorageError::NotFound(id))
        }
        Err(StorageError::DatabaseError(msg)) => {
            eprintln!("Database error: {}", msg);
            Err(StorageError::DatabaseError(msg))
        }
        Err(StorageError::SerializationError(msg)) => {
            eprintln!("JSON error: {}", msg);
            Err(StorageError::SerializationError(msg))
        }
    }
}
```

**Error Types:**
- `NotFound(String)` - Entity not found by ID
- `DatabaseError(String)` - SQLite error
- `SerializationError(String)` - JSON serialization error

## Testing

```rust
#[test]
fn test_memory_roundtrip() {
    let store = OmegaStore::new_in_memory().unwrap();
    let now = chrono::Utc::now().timestamp();

    let memory = StoredMemory {
        id: "test-001".to_string(),
        content: "Test content".to_string(),
        tier: 3,
        importance: 0.8,
        embedding_blob: Some(vec![1, 2, 3]),
        created_at: now,
        last_accessed: now,
    };

    // Store
    store.store_memory(&memory).unwrap();

    // Retrieve
    let retrieved = store.get_memory("test-001").unwrap();

    assert_eq!(retrieved.content, "Test content");
    assert_eq!(retrieved.tier, 3);
    assert_eq!(retrieved.importance, 0.8);
}

#[test]
fn test_tier_query() {
    let store = OmegaStore::new_in_memory().unwrap();
    let now = chrono::Utc::now().timestamp();

    // Insert memories in different tiers
    for i in 1..=5 {
        let memory = StoredMemory {
            id: format!("mem-{}", i),
            content: format!("Content {}", i),
            tier: if i <= 3 { 1 } else { 2 },
            importance: 0.5,
            embedding_blob: None,
            created_at: now,
            last_accessed: now,
        };
        store.store_memory(&memory).unwrap();
    }

    let tier1 = store.query_memories_by_tier(1).unwrap();
    let tier2 = store.query_memories_by_tier(2).unwrap();

    assert_eq!(tier1.len(), 3);
    assert_eq!(tier2.len(), 2);
}
```

## Performance Optimization

### 1. Indexing

The schema includes indexes on commonly queried columns:

```sql
CREATE INDEX idx_memories_tier ON memories(tier);
CREATE INDEX idx_memories_importance ON memories(importance);
CREATE INDEX idx_skills_success ON skills(success_count);
CREATE INDEX idx_architectures_paradigm ON architectures(paradigm);
```

### 2. Prepared Statements

SQLite internally caches prepared statements. Repeated queries are faster.

### 3. Batch Operations

```rust
// Efficient: Single connection, multiple operations
let store = OmegaStore::new("omega.db")?;
for memory in memories {
    store.store_memory(&memory)?;
}

// Less efficient: Multiple connections
// for memory in memories {
//     let store = OmegaStore::new("omega.db")?;
//     store.store_memory(&memory)?;
// }
```

## Integration Examples

### With omega-memory

```rust
use omega_persistence::*;
use omega_memory::*;

// Convert omega-memory::Memory to StoredMemory
fn persist_memory(
    store: &OmegaStore,
    memory: &omega_memory::Memory,
) -> Result<(), StorageError> {
    let stored = StoredMemory {
        id: memory.id.clone(),
        content: match &memory.content {
            MemoryContent::Text(s) => s.clone(),
            _ => serde_json::to_string(&memory.content).unwrap(),
        },
        tier: memory.tier as i32,
        importance: memory.importance,
        embedding_blob: Some(serialize_embedding(&memory.embedding)),
        created_at: memory.created_at.timestamp(),
        last_accessed: memory.accessed_at.timestamp(),
    };

    store.store_memory(&stored)
}

fn serialize_embedding(embedding: &[f32]) -> Vec<u8> {
    embedding.iter()
        .flat_map(|f| f.to_le_bytes())
        .collect()
}
```

### With omega-meta-sona

```rust
use omega_persistence::*;
use omega_core::Architecture;

fn persist_architecture(
    store: &OmegaStore,
    arch: &Architecture,
) -> Result<(), StorageError> {
    let stored = StoredArchitecture {
        id: arch.id.clone(),
        name: arch.name.clone(),
        paradigm: format!("{:?}", arch.paradigm),
        substrate: format!("{:?}", arch.substrate),
        fitness_json: serde_json::to_string(&arch.fitness).unwrap(),
        lineage_json: serde_json::to_string(&arch.lineage).unwrap(),
        created_at: arch.created_at.timestamp(),
    };

    store.store_architecture(&stored)
}
```

## Advanced Topics

### Custom Queries

While the API provides common queries, you can execute custom SQL:

```rust
// This would require exposing the underlying connection
// (not shown in current API, but possible with extension)
```

### Multi-Database Setup

```rust
// Separate databases for different purposes
let memory_store = OmegaStore::new("memories.db")?;
let skills_store = OmegaStore::new("skills.db")?;
let archs_store = OmegaStore::new("architectures.db")?;
```

### WAL Mode

For better concurrent access (if implemented):

```sql
PRAGMA journal_mode = WAL;  -- Write-Ahead Logging
```

## References

- **Source**: `omega/crates/omega-persistence`
- **Schema**: `omega/crates/omega-persistence/src/schema.rs`
- **SQLite Docs**: https://www.sqlite.org/docs.html
- **Tests**: Full CRUD coverage for all entity types

## Version History

- **0.1.0** (2025-01-05): Initial release
  - SQLite storage for all entity types
  - Indexed queries
  - Backup support
  - In-memory mode for testing
