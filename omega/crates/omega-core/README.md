# omega-core

Core types and traits for ExoGenesis Omega - Universal Intelligence Orchestration

## Overview

`omega-core` provides the foundational building blocks for orchestrating intelligence at any scale, from milliseconds to cosmic timescales.

## Features

- **12-Tier Cosmic Memory System**: Memory spanning from immediate (milliseconds) to cosmic (billions of years)
- **7 Temporal Loops**: Multi-scale feedback cycles from reflexive to transcendent
- **Universal Intelligence Types**: Support for any paradigm (Neural, Symbolic, Quantum, etc.) and substrate (Digital, Biological, Cosmic, etc.)
- **Async/Await Support**: Full async support with Tokio
- **Type Safety**: Strongly typed with Rust's type system

## Installation

```toml
[dependencies]
omega-core = "0.1.0"
```

## Quick Start

```rust
use omega_core::*;
use chrono::Utc;

// Create an intelligence
let architecture = Architecture {
    id: "arch-1".to_string(),
    name: "Neural Network".to_string(),
    paradigm: Paradigm::Neural,
    substrate: SubstrateType::Digital,
    fitness: None,
    lineage: vec![],
    created_at: Utc::now(),
};

let intelligence = Intelligence::new("My AI".to_string(), architecture);

// Create a memory in the semantic tier
let memory = Memory::new(
    MemoryTier::Semantic,
    MemoryType::Knowledge,
    MemoryContent::Text("Important fact".to_string()),
    0.9, // importance
);

// Create a temporal loop
let mut loop_instance = TemporalLoop::new(
    LoopType::Adaptive,
    "Learning Loop".to_string(),
    "Continuous learning from experience".to_string(),
);
```

## Architecture

### Intelligence System

The intelligence system supports:
- Multiple paradigms: Neural, Symbolic, Quantum, Biological, Social, Physical, Hybrid
- Various substrates: Digital, Biological, Social, Ecological, Geological, Stellar, Galactic, Cosmic
- Dynamic capabilities and evolution
- Fitness tracking and lineage

### Memory Tiers

1. **Immediate** (milliseconds) - Working memory
2. **Short-term** (seconds-minutes) - Recent interactions
3. **Session** (hours) - Current session context
4. **Episodic** (days) - Specific events
5. **Semantic** (weeks) - Facts and knowledge
6. **Procedural** (months) - Skills and procedures
7. **Strategic** (years) - Long-term strategies
8. **Civilizational** (decades-centuries) - Cultural knowledge
9. **Evolutionary** (millennia) - Species-level adaptations
10. **Planetary** (millions of years) - Planetary patterns
11. **Galactic** (billions of years) - Galactic knowledge
12. **Cosmic** (age of universe) - Universal constants

### Temporal Loops

1. **Reflexive** (milliseconds) - Immediate feedback
2. **Reactive** (seconds) - Quick decisions
3. **Adaptive** (minutes-hours) - Learning from experience
4. **Deliberative** (days) - Strategic planning
5. **Evolutionary** (weeks-months) - Systematic improvement
6. **Transformative** (years) - Fundamental changes
7. **Transcendent** (decades+) - Paradigm shifts

## Traits

The crate provides several core traits:

- `IntelligenceManager` - Create and manage intelligences
- `MemoryManager` - Store and query memories across tiers
- `LoopManager` - Manage temporal loops and cycles
- `EvolutionEngine` - Evolve architectures through variation and selection
- `CapabilityDiscovery` - Discover and integrate capabilities

## Examples

See the `examples/` directory for complete examples:

- `basic_intelligence.rs` - Creating and managing intelligences
- `memory_system.rs` - Working with the memory tiers
- `temporal_loops.rs` - Running multi-scale feedback loops
- `evolution.rs` - Evolving architectures

## Testing

```bash
cargo test
```

## License

MIT OR Apache-2.0
