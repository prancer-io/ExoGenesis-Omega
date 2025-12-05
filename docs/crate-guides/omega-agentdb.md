# omega-agentdb - SIMD-Optimized Vector Database

## Overview

`omega-agentdb` provides a high-performance, in-memory vector database with SIMD optimization for ExoGenesis Omega. It implements four core subsystems: vector storage with HNSW indexing, Reflexion memory, Causal reasoning, and Skill management.

**Key Features:**
- **SIMD-Optimized**: 13-41x speedup using SimSIMD library (AVX2/AVX-512)
- **HNSW Index**: O(log n) approximate nearest neighbor search
- **Reflexion System**: Learning from experience with critique
- **Causal Graphs**: Track cause-effect relationships
- **Skill Library**: Semantic skill storage with usage tracking
- **Zero-Copy**: Efficient memory usage with Arc/RwLock

**Performance:**
- Vector search: 346ns (4096-dim, SIMD) vs 14,481ns (scalar)
- Cosine similarity: **41.74x faster** at 4096 dimensions
- Scales to millions of vectors with sub-millisecond search

**Version:** 0.1.0
**Crate:** `omega-agentdb`
**Location:** `omega/crates/omega-agentdb`

## Installation

```toml
[dependencies]
omega-agentdb = "0.1.0"
```

For SIMD optimization (enabled by default):

```toml
[dependencies]
omega-agentdb = { version = "0.1.0", features = ["simd"] }
simsimd = "5.9"  # Included automatically
```

## Core Concepts

### 1. Vector Storage & HNSW

**Hierarchical Navigable Small World (HNSW)** provides fast approximate nearest neighbor search:

- **Time Complexity**: O(log n) search
- **Space Complexity**: O(n) storage
- **Parameters**:
  - `m`: Number of connections per node (default: 32)
  - `ef_construction`: Exploration factor during build (default: 100)
  - `ef_search`: Exploration factor during search (default: 100)

**SIMD Acceleration:**
- Uses SimSIMD library v5.9
- Automatically selects best instruction set (AVX-512, AVX2, NEON)
- Hardware-accelerated cosine distance computation

### 2. Reflexion Episodes

**Reflexion** enables learning from experience:

```rust
pub struct ReflexionEpisode {
    pub id: Option<ReflexionId>,
    pub session_id: String,
    pub task: String,
    pub input: serde_json::Value,
    pub output: serde_json::Value,
    pub reward: f64,          // -1.0 to 1.0
    pub success: bool,
    pub critique: String,      // Self-critique
    pub latency_ms: u64,
    pub tokens: u64,
    pub timestamp: DateTime<Utc>,
    pub embedding: Option<Embedding>,
}
```

**Use Cases:**
- Learning from mistakes
- Performance optimization
- Pattern recognition
- Adaptive behavior

### 3. Causal Reasoning

**Causal edges** represent cause-effect relationships:

```rust
pub struct CausalEdge {
    pub cause: String,
    pub effect: String,
    pub uplift: f64,           // Effect magnitude
    pub confidence: f64,       // 0.0 to 1.0
    pub sample_size: u64,      // Observations
    pub first_observed: DateTime<Utc>,
    pub last_observed: DateTime<Utc>,
}
```

**Capabilities:**
- Find effects of a cause
- Find causes of an effect
- Path finding (multi-hop causality)
- Weighted graph traversal

### 4. Skill Management

**Skills** are learned capabilities with semantic embeddings:

```rust
pub struct Skill {
    pub id: Option<SkillId>,
    pub name: String,
    pub description: String,
    pub embedding: Embedding,  // Semantic vector
    pub usage_count: u64,
    pub success_rate: f64,
    pub created_at: DateTime<Utc>,
}
```

**Features:**
- Semantic search by description
- Usage tracking
- Success rate computation
- Embedding-based similarity

## API Reference

### Database Initialization

```rust
use omega_agentdb::*;

#[tokio::main]
async fn main() -> Result<(), AgentDBError> {
    // Default configuration (4096-dim vectors)
    let db = AgentDB::new(AgentDBConfig::default()).await?;

    // Custom configuration
    let db = AgentDB::new(AgentDBConfig {
        dimension: 1024,      // Vector dimension
        hnsw_m: 16,          // HNSW connections
        hnsw_ef: 100,        // Search quality
        cache_size: 100_000, // Max vectors
    }).await?;

    Ok(())
}
```

### Vector Operations

#### Store Vectors

```rust
use omega_agentdb::*;

let embedding: Vec<f32> = vec![0.1; 4096]; // 4096-dim vector
let metadata = serde_json::json!({
    "type": "document",
    "title": "Rust Programming Guide",
    "chapter": 5
});

let vector_id = db.vector_store(embedding, metadata).await?;
println!("Stored vector: {}", vector_id);
```

#### Search Vectors (SIMD-Accelerated)

```rust
let query: Vec<f32> = vec![0.1; 4096];
let k = 10; // Top 10 results

let results = db.vector_search(&query, k).await?;

for result in results {
    println!("ID: {}, Similarity: {:.4}, Metadata: {}",
        result.id,
        result.similarity,
        result.metadata
    );
}
```

#### Retrieve & Delete

```rust
// Get specific vector
let (embedding, metadata) = db.vector_get("vec-id").await?;

// Delete vector
db.vector_delete("vec-id").await?;
```

### Reflexion Operations

#### Store Episode

```rust
use omega_agentdb::*;
use chrono::Utc;

let episode = ReflexionEpisode {
    id: None, // Auto-generated
    session_id: "session-001".to_string(),
    task: "code_generation".to_string(),
    input: serde_json::json!({"prompt": "Write factorial function"}),
    output: serde_json::json!({"code": "fn factorial(n: u64) -> u64 {...}"}),
    reward: 0.9,
    success: true,
    critique: "Good implementation, could add error handling".to_string(),
    latency_ms: 150,
    tokens: 120,
    timestamp: Utc::now(),
    embedding: None,
};

let episode_id = db.reflexion_store(episode).await?;
```

#### Retrieve Episodes

```rust
// Search by task
let episodes = db.reflexion_retrieve("code_generation", 10).await?;

// Get by session
let session_episodes = db.reflexion_by_session("session-001").await?;

// Analyze performance
let stats = db.reflexion_analyze("code").await?;
println!("Success rate: {:.2}%", stats.success_rate * 100.0);
println!("Avg reward: {:.2}", stats.avg_reward);
println!("Avg latency: {}ms", stats.avg_latency_ms);
```

### Causal Operations

#### Add Causal Edge

```rust
use omega_agentdb::*;
use chrono::Utc;

let edge = CausalEdge {
    cause: "use_cache".to_string(),
    effect: "faster_response".to_string(),
    uplift: 0.6,           // 60% improvement
    confidence: 0.95,      // 95% confidence
    sample_size: 100,      // 100 observations
    first_observed: Utc::now(),
    last_observed: Utc::now(),
};

db.causal_add_edge(edge).await?;
```

#### Query Causality

```rust
// Find effects of a cause
let effects = db.causal_query_effects("use_cache").await?;
for edge in effects {
    println!("{} -> {} (uplift: {:.2})",
        edge.cause, edge.effect, edge.uplift
    );
}

// Find causes of an effect
let causes = db.causal_query_causes("faster_response").await?;

// Find causal paths
let paths = db.causal_find_path("action_a", "outcome_z", 5).await?;
for path in paths {
    println!("Path: {:?}", path);
}
```

### Skill Operations

#### Create Skill

```rust
use omega_agentdb::*;

let embedding: Vec<f32> = vec![0.1; 4096]; // Semantic embedding

let skill = Skill {
    id: None,
    name: "code_review".to_string(),
    description: "Perform comprehensive code review with best practices".to_string(),
    embedding,
    usage_count: 0,
    success_rate: 0.0,
    created_at: Utc::now(),
};

let skill_id = db.skill_create(skill).await?;
```

#### Search Skills

```rust
// Text-based search
let skills = db.skill_search("code review", 5).await?;

// Embedding-based search (SIMD-accelerated)
let query_embedding: Vec<f32> = vec![0.1; 4096];
let results = db.skill_search_by_embedding(&query_embedding, 5).await?;

for (skill, similarity) in results {
    println!("{}: {:.4} (used {} times, {:.1}% success)",
        skill.name,
        similarity,
        skill.usage_count,
        skill.success_rate * 100.0
    );
}
```

#### Update Skill Stats

```rust
// Record skill usage
db.skill_update_stats("skill-id", true).await?;  // Success
db.skill_update_stats("skill-id", false).await?; // Failure

// Get skill
let skill = db.skill_get("skill-id").await?;
println!("Success rate: {:.2}%", skill.success_rate * 100.0);
```

## SIMD Optimization Details

### Performance Benchmarks

**Cosine Distance Computation:**

| Dimension | Scalar (ns) | SIMD (ns) | Speedup |
|-----------|-------------|-----------|---------|
| 128 | 1,039 | 75 | **13.94x** |
| 512 | 3,825 | 180 | **21.25x** |
| 1024 | 7,409 | 298 | **24.86x** |
| 4096 | 14,481 | 346 | **41.74x** |

**Hardware**: AVX2 (256-bit SIMD), Intel/AMD x86_64

### SIMD Implementation

```rust
// Internal SIMD-optimized distance function
use simsimd::SpatialSimilarity;

fn cosine_distance_simd(a: &[f32], b: &[f32]) -> f32 {
    f32::cosine(a, b).unwrap_or(1.0) as f32
}
```

**How it works:**
1. SimSIMD detects CPU capabilities at runtime
2. Selects optimal instruction set (AVX-512 > AVX2 > SSE > scalar)
3. Processes multiple vector elements in parallel
4. Returns distance: 0 (identical) to 2 (opposite)

### Enabling/Disabling SIMD

SIMD is enabled by default. To disable:

```toml
[dependencies]
omega-agentdb = { version = "0.1.0", default-features = false }
```

## Common Patterns

### 1. Agent Memory System

```rust
use omega_agentdb::*;

async fn agent_memory_system() -> Result<(), AgentDBError> {
    let db = AgentDB::new(AgentDBConfig::default()).await?;

    // Store experience
    let episode = ReflexionEpisode {
        task: "planning".to_string(),
        reward: 0.8,
        success: true,
        critique: "Good plan, consider edge cases".to_string(),
        // ... other fields
    };
    db.reflexion_store(episode).await?;

    // Learn from experience
    let past_attempts = db.reflexion_retrieve("planning", 5).await?;
    let stats = db.reflexion_analyze("planning").await?;

    if stats.success_rate > 0.7 {
        println!("Agent is proficient at planning");
    }

    Ok(())
}
```

### 2. Semantic Skill Matching

```rust
async fn find_relevant_skill(
    db: &AgentDB,
    task_description: &str,
    task_embedding: &[f32],
) -> Result<Option<Skill>, AgentDBError> {
    let results = db.skill_search_by_embedding(task_embedding, 1).await?;

    if let Some((skill, similarity)) = results.first() {
        if similarity > &0.8 {
            return Ok(Some(skill.clone()));
        }
    }

    Ok(None)
}
```

### 3. Causal Chain Analysis

```rust
async fn analyze_intervention_impact(
    db: &AgentDB,
    action: &str,
) -> Result<Vec<String>, AgentDBError> {
    let mut all_effects = Vec::new();
    let direct_effects = db.causal_query_effects(action).await?;

    // Collect direct and indirect effects
    for edge in direct_effects {
        all_effects.push(edge.effect.clone());

        // Find second-order effects
        let indirect = db.causal_query_effects(&edge.effect).await?;
        for indirect_edge in indirect {
            all_effects.push(indirect_edge.effect.clone());
        }
    }

    Ok(all_effects)
}
```

### 4. Multi-Session Learning

```rust
async fn cross_session_analysis(
    db: &AgentDB,
    session_ids: &[String],
) -> Result<ReflexionStats, AgentDBError> {
    let mut all_episodes = Vec::new();

    for session_id in session_ids {
        let episodes = db.reflexion_by_session(session_id).await?;
        all_episodes.extend(episodes);
    }

    // Compute aggregate statistics
    let total = all_episodes.len();
    let successful = all_episodes.iter().filter(|e| e.success).count();
    let avg_reward = all_episodes.iter().map(|e| e.reward).sum::<f64>() / total as f64;

    Ok(ReflexionStats {
        total_episodes: total,
        successful_episodes: successful,
        success_rate: successful as f64 / total as f64,
        avg_reward,
        avg_latency_ms: all_episodes.iter().map(|e| e.latency_ms).sum::<u64>() / total as u64,
        avg_tokens: all_episodes.iter().map(|e| e.tokens).sum::<u64>() / total as u64,
    })
}
```

## Best Practices

### Vector Dimension Selection

**DO:**
- Use 384-768 for sentence embeddings (all-MiniLM, BERT)
- Use 1536 for OpenAI embeddings (text-embedding-ada-002)
- Use 4096 for high-capacity models (custom architectures)

**DON'T:**
- Change dimension after database creation
- Use very high dimensions without SIMD (performance penalty)

### HNSW Tuning

**For Accuracy:**
```rust
AgentDBConfig {
    hnsw_m: 64,        // More connections
    hnsw_ef: 200,      // Higher search quality
    ..Default::default()
}
```

**For Speed:**
```rust
AgentDBConfig {
    hnsw_m: 16,        // Fewer connections
    hnsw_ef: 50,       // Lower search quality
    ..Default::default()
}
```

**For Balance:**
```rust
AgentDBConfig::default() // m=32, ef=100
```

### Reflexion Best Practices

**DO:**
- Store both successes and failures
- Include detailed critiques
- Use consistent reward scales (-1 to 1)
- Track token usage for cost analysis

**DON'T:**
- Only store successful episodes (biased learning)
- Use reward values outside [-1, 1]
- Forget to set session_id (needed for grouping)

### Causal Edge Management

**DO:**
- Update edges with new observations
- Set confidence based on sample size
- Use meaningful cause/effect names
- Track temporal information

**DON'T:**
- Create edges with sample_size=1 (insufficient evidence)
- Set confidence=1.0 without strong evidence
- Ignore edge direction

## Error Handling

```rust
use omega_agentdb::AgentDBError;

async fn safe_vector_search(
    db: &AgentDB,
    query: &[f32],
) -> Result<Vec<VectorResult>, AgentDBError> {
    match db.vector_search(query, 10).await {
        Ok(results) => Ok(results),
        Err(AgentDBError::QueryError(msg)) => {
            eprintln!("Query error: {}", msg);
            Ok(Vec::new()) // Return empty on error
        }
        Err(e) => Err(e), // Propagate other errors
    }
}
```

**Error Types:**
- `StorageError(String)` - Database storage error
- `QueryError(String)` - Invalid query parameters
- `NotFound(String)` - Entity not found

## Testing

```rust
#[tokio::test]
async fn test_vector_roundtrip() {
    let db = AgentDB::new(AgentDBConfig {
        dimension: 128,
        ..Default::default()
    }).await.unwrap();

    let embedding: Vec<f32> = (0..128).map(|i| i as f32 / 128.0).collect();
    let metadata = serde_json::json!({"test": true});

    // Store
    let id = db.vector_store(embedding.clone(), metadata.clone()).await.unwrap();

    // Retrieve
    let (retrieved_emb, retrieved_meta) = db.vector_get(&id).await.unwrap();

    assert_eq!(retrieved_emb.len(), 128);
    assert_eq!(retrieved_meta, metadata);

    // Search
    let results = db.vector_search(&embedding, 1).await.unwrap();
    assert_eq!(results.len(), 1);
    assert!(results[0].similarity > 0.99); // Should be very similar to itself
}
```

## Performance Optimization

### 1. Batch Operations

```rust
// Good: Batch insert
for embedding in embeddings {
    db.vector_store(embedding, metadata.clone()).await?;
}

// Better: Parallel insert (requires multiple db instances or interior mutability)
```

### 2. Search Optimization

```rust
// Warm up HNSW index
for _ in 0..100 {
    let dummy_query = vec![0.0; dimension];
    db.vector_search(&dummy_query, 1).await?;
}
```

### 3. Memory Management

```rust
// Clear when done
db.clear().await?;

// Get statistics
let stats = db.stats().await;
println!("Vectors: {}, Episodes: {}, Skills: {}",
    stats.vector_count,
    stats.episode_count,
    stats.skill_count
);
```

## Integration Examples

### With omega-memory

```rust
use omega_agentdb::*;
use omega_memory::*;

let db = AgentDB::new(AgentDBConfig::default()).await?;
let memory_system = CosmicMemory::new().await?;

// Store memory with vector
let memory = Memory::new(/* ... */);
let embedding = vec![0.1; 4096]; // From embedding model

db.vector_store(
    embedding,
    serde_json::to_value(&memory)?
).await?;
```

### With omega-runtime

```rust
use omega_runtime::*;
use omega_agentdb::*;

let runtime = OmegaRuntime::new(OmegaConfig::default()).await?;
let api = OmegaAPI::new(runtime);

// AgentDB is integrated into runtime
// Access via runtime's memory subsystem
```

## References

- **Source**: `omega/crates/omega-agentdb`
- **SIMD Analysis**: `docs/SIMD-IMPLEMENTATION-RESULTS.md`
- **Benchmarks**: `omega/crates/omega-agentdb/examples/benchmark_simd.rs`
- **Tests**: 228 tests passing, 100% core API coverage

## Version History

- **0.1.0** (2025-01-05): Initial release
  - SIMD optimization (13-41x speedup)
  - HNSW vector index
  - Reflexion, Causal, Skill subsystems
  - SimSIMD v5.9 integration
