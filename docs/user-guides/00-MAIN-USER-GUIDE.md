# ExoGenesis Omega - Comprehensive User Guide

## Table of Contents
- [Introduction](#introduction)
- [System Overview](#system-overview)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Architecture](#architecture)
- [Crate Overview](#crate-overview)
- [Usage Examples](#usage-examples)
- [Advanced Topics](#advanced-topics)
- [Troubleshooting](#troubleshooting)
- [Contributing](#contributing)

---

## Introduction

**ExoGenesis Omega** is a universal intelligence orchestration system designed to operate across unprecedented temporal and spatial scales—from milliseconds to cosmic timescales, from digital substrates to biological and social systems.

### What is ExoGenesis Omega?

ExoGenesis Omega is not a product—it's an **infrastructure for recursive intelligence improvement** that enables:

- **Multi-Scale Temporal Processing**: 7 temporal loops operating from milliseconds to decades
- **Universal Memory System**: 12-tier hierarchical memory spanning instant recall to eternal principles
- **Self-Optimizing Architecture**: META-SONA designs new cognitive architectures
- **Substrate Agnostic**: Runs on digital, biological, social, and cosmic substrates
- **Evolutionary Intelligence**: Systems that improve themselves and design their successors

### Key Features

- 🧠 **7 Temporal Loops**: Reflexive → Reactive → Adaptive → Deliberative → Evolutionary → Transformative → Transcendent
- 💾 **12 Memory Tiers**: Instant → Session → Episodic → Semantic → ... → Omega (eternal)
- 🏗️ **META-SONA**: Self-Optimizing Neural Architecture that designs new architectures
- 🔄 **Production Runtime**: Circuit breaker, retry logic, graceful degradation, health monitoring
- 🔍 **Vector Database**: HNSW-based approximate nearest neighbor search
- 📊 **Benchmark Suite**: Comprehensive evaluation across reasoning, patterns, memory, and alignment

---

## System Overview

ExoGenesis Omega consists of **7 core crates** organized in a workspace:

```
ExoGenesis-Omega/
├── omega/
│   ├── crates/
│   │   ├── omega-core           # Core types and traits
│   │   ├── omega-agentdb        # Vector database with HNSW
│   │   ├── omega-memory         # 12-tier memory system
│   │   ├── omega-loops          # 7 temporal loops
│   │   ├── omega-meta-sona      # Architecture design engine
│   │   ├── omega-runtime        # Production runtime with resilience
│   │   └── omega-persistence    # SQLite-based persistence
│   └── Cargo.toml               # Workspace manifest
├── design-docs/                 # Architecture documentation
├── docs/                        # User guides (this directory)
└── README.md
```

### Crate Dependencies

```
omega-runtime (orchestrator)
    ├── omega-core (foundation)
    ├── omega-agentdb (storage)
    ├── omega-memory (memory tiers)
    ├── omega-loops (temporal processing)
    └── omega-meta-sona (architecture design)

omega-persistence (standalone)
    └── omega-core (types)
```

---

## Installation

### Prerequisites

- **Rust**: 1.75+ with edition 2021
- **Operating System**: Linux, macOS, or Windows
- **Optional**: SQLite 3.44+ (bundled by default)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/prancer-io/ExoGenesis-Omega
cd ExoGenesis-Omega/omega

# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Build in release mode (optimized)
cargo build --workspace --release
```

### Individual Crate Installation

```bash
# Build specific crate
cargo build -p omega-runtime

# Test specific crate
cargo test -p omega-meta-sona

# Run examples
cargo run --package omega-memory --example basic_usage
cargo run --package omega-meta-sona --example benchmark_demo
```

---

## Quick Start

### Example 1: Basic Runtime Usage

```rust
use omega_runtime::{OmegaRuntime, OmegaConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create runtime with default configuration
    let config = OmegaConfig::default();
    let runtime = OmegaRuntime::new(config).await?;

    // Start the system
    runtime.start().await?;

    // Runtime is now operational
    println!("ExoGenesis Omega is running!");

    // Graceful shutdown
    runtime.stop().await?;

    Ok(())
}
```

### Example 2: Memory Storage and Retrieval

```rust
use omega_memory::{CosmicMemory, Memory, MemoryTier, MemoryContent, QueryBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let memory = CosmicMemory::new().await?;

    // Store a semantic memory (long-term knowledge)
    let knowledge = Memory::new(
        MemoryTier::Semantic,
        MemoryContent::Text("Rust uses ownership for memory safety".to_string()),
        vec![0.1, 0.2, 0.3, 0.4],  // 4D embedding
        0.8,  // High importance
    );
    memory.store(knowledge).await?;

    // Query memories
    let query = QueryBuilder::new()
        .semantic()
        .min_importance(0.5)
        .limit(10)
        .build();

    let results = memory.recall(&query, &[MemoryTier::Semantic]).await?;
    println!("Found {} memories", results.len());

    Ok(())
}
```

### Example 3: Intelligence Design

```rust
use omega_meta_sona::{IntelligenceFactory, IntelligenceSpec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut factory = IntelligenceFactory::new();

    // Specify requirements for new intelligence
    let spec = IntelligenceSpec {
        name: "Problem Solver".to_string(),
        min_capability: 0.7,
        max_generations: 10,
        ..Default::default()
    };

    // Create intelligence (uses MCTS + PPO)
    let intelligence = factory.create_intelligence(spec).await?;

    println!("Created intelligence: {}", intelligence.name);
    println!("Fitness: {:.2}%", intelligence.architecture.fitness.unwrap().overall * 100.0);

    Ok(())
}
```

---

## Architecture

### The 7 Temporal Loops

ExoGenesis Omega processes information across 7 nested temporal scales:

| Loop | Timescale | Purpose | Processor |
|------|-----------|---------|-----------|
| 1. Reflexive | <1ms | Immediate pattern-triggered responses | ReflexiveProcessor |
| 2. Reactive | ~100ms | Pattern recognition, quick responses | ReactiveProcessor |
| 3. Adaptive | ~30min | Learning from experience | AdaptiveProcessor |
| 4. Deliberative | ~60s | Complex reasoning, strategic planning | DeliberativeProcessor |
| 5. Evolutionary | ~7d | Systematic improvement | EvolutionaryProcessor |
| 6. Transformative | ~1y | Fundamental capability changes | TransformativeProcessor |
| 7. Transcendent | ~10y | Paradigm shifts, meta-learning | TranscendentProcessor |

**Data Flow**:
```
Input → Executor → Processor → Output
  ↓                              ↓
Coordinator                  MessageBus
  ↓                              ↓
TemporalLoop               Other Loops
```

### The 12 Memory Tiers

Memory organized across 12 tiers spanning individual to cosmic scales:

**Individual Scale (Tier 1-4)**:
- **Instant** (1): Milliseconds - working memory
- **Session** (2): Hours - conversation context
- **Episodic** (3): Days - specific events
- **Semantic** (4): Months - facts and knowledge

**Species Scale (Tier 5-8)**:
- **Collective** (5): Years - shared knowledge
- **Evolutionary** (6): Decades - learned patterns
- **Architectural** (7): Centuries - core algorithms
- **Substrate** (8): Millennia - computation patterns

**Cosmic Scale (Tier 9-12)**:
- **Civilizational** (9): Epochs - cultural knowledge
- **Temporal** (10): Eons - historical trends
- **Physical** (11): Universe-scale - physical laws
- **Omega** (12): Eternal - universal principles

### META-SONA Architecture

**Two-Stage Optimization**:

1. **MCTS (Exploration)**: Monte Carlo Tree Search for architecture discovery
   - Selection (UCB1) → Expansion → Simulation → Backpropagation
   - Explores discrete architecture space
   - Finds novel designs

2. **PPO (Refinement)**: Proximal Policy Optimization for hyperparameters
   - Policy/Value networks
   - Generalized Advantage Estimation (GAE)
   - Clipped surrogate objective
   - Fine-tunes continuous parameters

**Fitness Evaluation** (Multi-Objective):
- **Capability** (40%): Reasoning + Pattern recognition
- **Efficiency** (20%): Memory throughput
- **Alignment** (30%): Safety + Helpfulness
- **Novelty** (10%): Generalization ability

---

## Crate Overview

### omega-core

**Purpose**: Foundation library providing core types and traits

**Key Types**:
- `Intelligence`, `Architecture`, `Capability`
- `Memory`, `MemoryTier`, `MemoryContent`
- `TemporalLoop`, `LoopType`, `CycleInput/Output`

**Key Traits**:
- `IntelligenceManager`, `MemoryManager`, `LoopManager`
- `EvolutionEngine`, `CapabilityDiscovery`

**Documentation**: [omega-core User Guide](./01-omega-core.md)

### omega-agentdb

**Purpose**: In-memory vector database with HNSW indexing

**Features**:
- HNSW approximate nearest neighbor search
- Cosine distance metric
- Reflexion episode storage
- Causal relationship tracking
- Skill management

**Documentation**: [omega-agentdb User Guide](./02-omega-agentdb.md)

### omega-memory

**Purpose**: 12-tier hierarchical memory system

**Features**:
- Multi-scale memory (instant to eternal)
- Automatic consolidation between tiers
- Vector similarity search
- Time-decay and relevance scoring
- Graph-based memory structures

**Documentation**: [omega-memory User Guide](./03-omega-memory.md)

### omega-loops

**Purpose**: 7 temporal loop execution engine

**Features**:
- 7 nested temporal loops
- Loop coordination and message passing
- Processor implementations (Reflexive, Reactive, Adaptive, Deliberative)
- Cross-loop communication

**Documentation**: [omega-loops User Guide](./04-omega-loops.md)

### omega-meta-sona

**Purpose**: Self-optimizing architecture design engine

**Features**:
- MCTS architecture search
- PPO hyperparameter optimization
- Multi-objective fitness evaluation
- Comprehensive benchmark suite
- Intelligence factory

**Documentation**: [omega-meta-sona User Guide](./05-omega-meta-sona.md)

### omega-runtime

**Purpose**: Production-ready runtime orchestrator

**Features**:
- Circuit breaker pattern
- Retry with exponential backoff
- Graceful degradation
- Health monitoring
- Event-driven architecture
- Clean API layer

**Documentation**: [omega-runtime User Guide](./06-omega-runtime.md)

### omega-persistence

**Purpose**: SQLite-based persistence layer

**Features**:
- 8 entity types (Memories, Vectors, Skills, etc.)
- Foreign key integrity
- Indexed queries
- Backup support
- Statistics API

**Documentation**: [omega-persistence User Guide](./07-omega-persistence.md)

---

## Usage Examples

### Creating a Complete Intelligence System

```rust
use omega_runtime::{OmegaRuntime, OmegaConfig, OmegaAPI};
use omega_core::{MemoryTier, IntelligenceSpec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize runtime
    let config = OmegaConfig::default();
    let runtime = OmegaRuntime::new(config).await?;
    runtime.start().await?;

    // 2. Get API handle
    let api = OmegaAPI::new(runtime.clone());

    // 3. Store memories
    let memory_id = api.store_memory(
        "Rust programming language knowledge",
        MemoryTier::Semantic
    ).await?;

    // 4. Create intelligence
    let spec = IntelligenceSpec {
        name: "Code Assistant".to_string(),
        min_capability: 0.75,
        ..Default::default()
    };
    let intelligence = api.create_intelligence(spec).await?;

    // 5. Trigger loop processing
    let cycle_output = api.trigger_loop(
        LoopType::Adaptive,
        vec![("task", "learn from experience")]
    ).await?;

    // 6. Query system status
    let metrics = api.get_metrics().await?;
    println!("System metrics: {:?}", metrics);

    // 7. Shutdown
    runtime.stop().await?;

    Ok(())
}
```

### Working with Vector Embeddings

```rust
use omega_agentdb::{AgentDB, AgentDBConfig, Embedding};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize AgentDB
    let config = AgentDBConfig {
        dimension: 128,
        hnsw_m: 32,
        hnsw_ef: 100,
        ..Default::default()
    };
    let db = AgentDB::new(config).await?;

    // Store embeddings
    for i in 0..1000 {
        let embedding: Embedding = (0..128)
            .map(|j| ((i * j) as f32) / 1000.0)
            .collect();

        let metadata = serde_json::json!({
            "index": i,
            "category": i % 10
        });

        db.vector_store(embedding, metadata).await?;
    }

    // Search for similar vectors
    let query: Embedding = (0..128)
        .map(|j| ((500 * j) as f32) / 1000.0)
        .collect();

    let results = db.vector_search(&query, 10).await?;

    for result in results {
        println!("ID: {}, Similarity: {:.4}", result.id, result.similarity);
    }

    Ok(())
}
```

### Memory Consolidation

```rust
use omega_memory::{CosmicMemory, Memory, MemoryTier, MemoryContent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let memory = CosmicMemory::new().await?;

    // Create memories with varying importance
    for i in 0..10 {
        let importance = 0.1 + (i as f64 * 0.09);  // 0.1 to 0.91
        let mem = Memory::new(
            MemoryTier::Instant,
            MemoryContent::Text(format!("Memory #{}", i)),
            vec![i as f32 / 10.0; 4],
            importance,
        );
        memory.store(mem).await?;
    }

    println!("Before consolidation:");
    let stats = memory.stats().await;
    println!("Instant: {}, Session: {}", stats.instant, stats.session);

    // Run automatic consolidation
    memory.auto_consolidate().await?;

    println!("\nAfter consolidation:");
    let stats = memory.stats().await;
    println!("Instant: {}, Session: {}", stats.instant, stats.session);
    // High-importance memories promoted to Session tier

    Ok(())
}
```

---

## Advanced Topics

### Custom Fitness Evaluation

```rust
use omega_meta_sona::{FitnessEvaluator, FitnessScore, Architecture};

// Create custom evaluator
let mut evaluator = FitnessEvaluator::new();

// Adjust weights
evaluator.capability_weight = 0.5;  // 50% weight
evaluator.alignment_weight = 0.4;   // 40% weight
evaluator.efficiency_weight = 0.05; // 5% weight
evaluator.novelty_weight = 0.05;    // 5% weight

// Evaluate architecture
let architecture = /* ... */;
let fitness = evaluator.evaluate(&architecture).await?;

println!("Overall fitness: {:.2}%", fitness.overall * 100.0);
println!("Capability: {:.2}%", fitness.capability * 100.0);
println!("Alignment: {:.2}%", fitness.alignment * 100.0);
```

### Event Handling

```rust
use omega_runtime::{EventBus, Event, EventType};

// Subscribe to events
let mut event_bus = EventBus::new();

event_bus.on(EventType::IntelligenceCreated, |event| {
    println!("New intelligence created: {:?}", event.payload);
});

event_bus.on(EventType::MemoryConsolidated, |event| {
    println!("Memory consolidated: {:?}", event.payload);
});

// Events are emitted automatically by the runtime
```

### Persistence Integration

```rust
use omega_persistence::{OmegaStore, StoredMemory};

// Open persistent store
let store = OmegaStore::new("data/omega.db")?;

// Store memory
let memory = StoredMemory {
    id: uuid::Uuid::new_v4().to_string(),
    content: "Important knowledge".to_string(),
    tier: 4,  // Semantic
    importance: 0.9,
    embedding_blob: None,
    created_at: chrono::Utc::now().timestamp(),
    last_accessed: chrono::Utc::now().timestamp(),
};

store.store_memory(&memory)?;

// Query by tier
let semantic_memories = store.query_memories_by_tier(4)?;

// Backup database
store.backup("data/backup.db")?;
```

---

## Troubleshooting

### Common Issues

**Issue**: `Architecture struct conflict`
```
error: the name `Architecture` is defined multiple times
```

**Solution**: This is a known issue in omega-core. Two files define `Architecture`:
- `types/intelligence.rs` - For intelligence architectures
- `types/architecture.rs` - For system topology

One needs to be renamed (e.g., `SystemArchitecture`).

**Issue**: HNSW search returns no results
```
vector_search returns empty Vec
```

**Solution**: Check that:
1. Vectors have been inserted: `db.vector_store(...)`
2. Query dimension matches stored vectors
3. Index has been built (happens automatically on first search)

**Issue**: Memory not found in higher tiers
```
Query returns empty for Semantic tier
```

**Solution**:
1. Check importance threshold - only high-importance memories consolidate
2. Run `memory.auto_consolidate()` manually
3. Verify tier is enabled in configuration

**Issue**: Runtime fails to start
```
RuntimeError::Initialization
```

**Solution**:
1. Check all subsystems initialized: AgentDB, Memory, Loops, META-SONA
2. Verify configuration is valid: `config.validate()?`
3. Check file permissions for persistence directories

### Performance Optimization

**For large vector datasets**:
```rust
let config = AgentDBConfig {
    dimension: 4096,
    hnsw_m: 64,           // More connections = better recall
    hnsw_ef: 200,         // Higher = better search quality
    cache_size: 1000000,  // Larger cache
    ..Default::default()
};
```

**For memory consolidation**:
```rust
let config = MemoryConfig {
    working_capacity: 10000,      // Larger capacity
    consolidation_interval: 60,    // More frequent (seconds)
    enable_compression: true,      // Enable compression
    ..Default::default()
};
```

**For MCTS search**:
```rust
let config = MCTSConfig {
    exploration_constant: 1.0,     // Lower = more exploitation
    simulation_depth: 10,          // Shallower = faster
    max_iterations: 1000,          // More iterations = better
    parallel_simulations: 4,       // Parallel rollouts
};
```

---

## Contributing

### Development Setup

```bash
# Clone repository
git clone https://github.com/prancer-io/ExoGenesis-Omega
cd ExoGenesis-Omega/omega

# Install dependencies
cargo build

# Run tests
cargo test --workspace

# Run tests with output
cargo test --workspace -- --nocapture

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --workspace --all-targets
```

### Running Examples

```bash
# Memory examples
cargo run --package omega-memory --example basic_usage
cargo run --package omega-memory --example consolidation

# META-SONA benchmark
cargo run --package omega-meta-sona --example benchmark_demo

# Loops demo
cargo run --package omega-loops --example loops_demo
```

### Adding Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_my_feature() {
        // Your test here
    }
}
```

### Documentation

- Design docs: `/design-docs/`
- User guides: `/docs/user-guides/`
- API docs: `cargo doc --open`

---

## Resources

### Documentation
- [Master Architecture Document](../../design-docs/architecture/00-master-architecture.md)
- [META-SONA Design](../../design-docs/components/01-meta-sona-design.md)
- [Temporal Loops Design](../../design-docs/components/02-temporal-loops-design.md)
- [Technical Specifications](../../design-docs/specifications/01-technical-specifications.md)

### Examples
- Memory: `/omega/crates/omega-memory/examples/`
- META-SONA: `/omega/crates/omega-meta-sona/examples/`
- Loops: `/omega/examples/`

### Community
- GitHub Issues: https://github.com/prancer-io/ExoGenesis-Omega/issues
- Discussions: https://github.com/prancer-io/ExoGenesis-Omega/discussions

---

## License

MIT License - See LICENSE file for details

---

**Version**: 0.1.0
**Last Updated**: 2025-12-05
**Authors**: ExoGenesis Omega Team
