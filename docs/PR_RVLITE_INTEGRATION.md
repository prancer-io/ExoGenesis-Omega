# PR: RvLite Integration for Long-Term Memory Persistence

## Summary

This PR integrates [RvLite](https://github.com/ruvnet/ruvector/tree/main/npm/packages/rvlite) as the persistent storage backend for ExoGenesis-Omega's 12-tier cosmic memory system. RvLite is a lightweight (~850KB WASM) vector database with graph capabilities, providing efficient similarity search and relationship traversal for long-term memory consolidation.

## Key Changes

### New Files

| File | Lines | Description |
|------|-------|-------------|
| `omega-agentdb/src/rvlite_backend.rs` | ~1300 | Core RvLite implementation with VectorBackend and GraphBackend traits |
| `omega-memory/src/rvlite_bridge.rs` | ~1170 | Bridge connecting 12-tier cosmic memory to RvLite storage |
| `omega-memory/examples/rvlite_simulation.rs` | ~250 | Comprehensive simulation demonstrating all RvLite features |

### Updated Dependencies

| Crate | Changes |
|-------|---------|
| `omega-hippocampus` | Added `omega-memory` + `omega-agentdb` for memory consolidation |
| `omega-sleep` | Added `omega-memory` + `omega-hippocampus` for dream-state replay |
| `omega-consciousness` | Added optional `memory-integration` feature |
| `omega-brain` | Added optional `persistent-memory` feature |
| `omega-strange-loops` | Added optional `memory-integration` feature |
| `omega-mindscape` | Added `omega-agentdb` for spatial memory navigation |

## Architecture

### Memory Tier System

The integration preserves ExoGenesis-Omega's 12-tier cosmic memory hierarchy:

```
Instant → Session → Episodic → Semantic → Collective → Evolutionary
   ↓         ↓          ↓           ↓           ↓            ↓
Working   Current    Personal    Learned     Shared      Species
Context   Session    Events      Facts       Knowledge   Patterns
                                    ↓
                              Transcendent → Cosmic → Universal → Omega
                                    ↓            ↓          ↓         ↓
                              Beyond-Self   Galactic   All-Reality  Ultimate
```

### Core Traits

```rust
/// Vector storage operations
pub trait VectorBackend {
    async fn upsert(&self, id: &str, vector: &[f32], metadata: Value) -> Result<()>;
    async fn query(&self, vector: &[f32], top_k: usize, filter: Option<Value>) -> Result<Vec<SearchResult>>;
    async fn delete(&self, id: &str) -> Result<()>;
}

/// Graph relationship operations
pub trait GraphBackend {
    async fn add_edge(&self, from: &str, to: &str, rel_type: &str, weight: f32) -> Result<()>;
    async fn traverse(&self, start: &str, max_depth: usize, rel_filter: Option<&str>) -> Result<Vec<GraphNode>>;
    async fn find_path(&self, from: &str, to: &str) -> Result<Option<Vec<String>>>;
}
```

### Bio-Inspired Memory Consolidation

The integration implements hippocampal-style memory processes:

1. **Memory Replay** - Strengthens important memories through repeated activation
2. **Tier Promotion** - Promotes frequently accessed memories to higher tiers
3. **Decay Mechanism** - Weakens rarely accessed memories over time
4. **Consolidation** - Periodic cleanup and optimization of memory store

```rust
pub struct ConsolidationThresholds {
    pub importance_threshold: f32,      // Min importance for promotion
    pub min_access_count: u32,          // Min accesses for promotion
    pub min_age_hours: u64,             // Min age before consolidation
    pub decay_rate_individual: f32,     // Per-memory decay rate
    pub decay_rate_species: f32,        // Evolutionary tier decay rate
}
```

## Features

### Semantic Search

SIMD-optimized similarity search with multiple distance metrics:
- Cosine similarity (default)
- Euclidean distance
- Dot product

### Graph Relationships

Six relationship types for causal and associative linking:
- `Causes` - Causal relationships
- `Precedes` - Temporal ordering
- `SimilarTo` - Semantic similarity
- `PartOf` - Compositional relationships
- `AssociatedWith` - General associations
- `ContradictsBy` - Contradictory information

### Persistence

- JSON export/import for portable storage
- Configurable auto-sync intervals
- Tier-based persistence (choose which tiers to persist)

## Simulation Results

The example simulation demonstrates:

```
═══════════════════════════════════════════════════════════════
       RvLite Integration Simulation - ExoGenesis Omega
═══════════════════════════════════════════════════════════════

[1/7] Creating RvLite Bridge...
      ✓ Bridge created with 128 dimensions

[2/7] Storing memories across tiers...
      ✓ Stored mem-0 at tier Instant (importance: 0.30)
      ✓ Stored mem-1 at tier Session (importance: 0.50)
      ...

[3/7] Testing semantic similarity search...
      Query returned 3 results

[4/7] Adding graph relationships...
      ✓ Created 5 relationships
      Related to mem-0 (within 3 hops): 4 nodes

[5/7] Testing memory replay (hippocampal strengthening)...
      ✓ Completed 5 replay cycles

[6/7] Testing memory consolidation...
      Consolidation report:
        - Promoted: 2
        - Decayed: 3
        - Removed: 0

[7/7] Testing persistence (save/load)...
      ✓ Saved to /tmp/omega_rvlite_test.json
      ✓ Loaded 6 memories from file
      ✓ Data integrity verified

═══════════════════════════════════════════════════════════════
                      SIMULATION COMPLETE
═══════════════════════════════════════════════════════════════
```

## Why RvLite?

| Feature | RvLite | Traditional DBs |
|---------|--------|-----------------|
| Size | ~850KB WASM | 10-100MB+ |
| Vector Search | Native SIMD | Requires extensions |
| Graph Queries | Built-in Cypher | Separate system |
| Embedding | Direct integration | External service |
| Portability | Runs anywhere | Server required |

## Commits

1. `eb35b5b` - feat(memory): integrate RvLite for long-term memory persistence
2. `b4d8589` - feat(deps): expose RvLite access across cognitive architecture crates
3. `134bd75` - feat(examples): add RvLite integration simulation example

## Testing

```bash
# Build RvLite-related crates
cd omega/crates
cargo build -p omega-memory -p omega-agentdb

# Run simulation
cargo run --example rvlite_simulation -p omega-memory
```

## Future Work

- [ ] Integrate with `omega-sleep` for dream-state memory replay
- [ ] Connect to `omega-consciousness` for phi-weighted consolidation
- [ ] Add distributed persistence across `omega-collective`
- [ ] Implement HNSW indexing for billion-scale memory search

## Related

- [RuVector Project](https://github.com/ruvnet/ruvector)
- [RvLite Documentation](https://github.com/ruvnet/ruvector/tree/main/npm/packages/rvlite)
- [ExoGenesis-Omega Memory System](omega/crates/omega-memory)
