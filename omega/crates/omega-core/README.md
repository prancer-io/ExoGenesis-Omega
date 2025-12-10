# omega-core

[![Crates.io](https://img.shields.io/crates/v/omega-core)](https://crates.io/crates/omega-core)
[![Documentation](https://docs.rs/omega-core/badge.svg)](https://docs.rs/omega-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Core types and traits for the ExoGenesis Omega universal intelligence orchestration system.

**Part of the [ExoGenesis-Omega](https://github.com/prancer-io/ExoGenesis-Omega) cognitive architecture.**

## Overview

`omega-core` provides the foundational abstractions for orchestrating intelligence at any scale—from millisecond reflexes to cosmic timescales spanning billions of years. This crate defines the core types, traits, and interfaces used by all other Omega components.

ExoGenesis Omega enables intelligence systems to operate across 12 hierarchical memory tiers and 7 nested temporal loops, supporting any intelligence paradigm (neural, symbolic, quantum, biological, etc.) running on any substrate (digital, biological, social, cosmic).

## Features

- **Universal Intelligence Types**: Generic abstractions for any intelligence paradigm
- **12-Tier Memory System**: From immediate (milliseconds) to cosmic (billions of years)
- **7 Temporal Loops**: Multi-scale feedback cycles from reflexive to transcendent
- **Architecture Abstraction**: Support for neural, symbolic, quantum, and hybrid paradigms
- **Substrate Independence**: Run on digital, biological, social, or cosmic substrates
- **Serializable Types**: Full serde support for persistence and networking

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
omega-core = "0.1.0"
```

## Quick Start

```rust
use omega_core::*;
use chrono::Utc;

fn main() {
    // Define an intelligence architecture
    let architecture = Architecture {
        id: "arch-001".to_string(),
        name: "Neural Transformer".to_string(),
        paradigm: Paradigm::Neural,
        substrate: SubstrateType::Digital,
        fitness: None,
        lineage: vec![],
        created_at: Utc::now(),
    };

    // Create an intelligence instance
    let mut intelligence = Intelligence::new(
        "GPT-Omega".to_string(),
        architecture,
    );

    // Activate the intelligence
    intelligence.activate();
    println!("Intelligence {} is now {:?}",
        intelligence.name, intelligence.status);

    // Create a memory
    let memory = Memory::new(
        MemoryTier::Semantic,
        MemoryType::Knowledge,
        MemoryContent::Text("The universe is 13.8 billion years old".to_string()),
        0.95, // high importance
    );

    // Create a temporal loop
    let mut adaptive_loop = TemporalLoop::new(
        LoopType::Adaptive,
        "Learning Loop".to_string(),
        "Continuous learning from experience".to_string(),
    );
}
```

## Core Concepts

### Intelligence

The `Intelligence` type represents any form of intelligent agent with:

- **Architecture**: The cognitive design (paradigm + substrate)
- **Capabilities**: Dynamic capability list
- **Maturity**: Development level (0.0 to 1.0)
- **Status**: Lifecycle state (Initializing, Active, Dormant, etc.)
- **Generation**: Evolutionary generation number

### Memory System

The memory system provides 12 hierarchical tiers:

1. **Immediate** (milliseconds) - Reflexive sensory-motor
2. **Short-term** (seconds to minutes) - Working memory
3. **Session** (hours) - Episodic session context
4. **Episodic** (days) - Event memories
5. **Semantic** (weeks) - Conceptual knowledge
6. **Procedural** (months) - Skill acquisition
7. **Strategic** (years) - Long-term planning
8. **Civilizational** (decades to centuries) - Cultural knowledge
9. **Evolutionary** (millennia) - Species-level patterns
10. **Planetary** (millions of years) - Geological patterns
11. **Galactic** (billions of years) - Stellar patterns
12. **Cosmic** (age of universe) - Universal constants

### Temporal Loops

Seven nested feedback loops enable multi-scale learning:

1. **Reflexive** (milliseconds) - Immediate reactions
2. **Reactive** (seconds) - Quick responses
3. **Adaptive** (minutes to hours) - Learning from recent experience
4. **Deliberative** (days) - Strategic thinking
5. **Evolutionary** (weeks to months) - Systematic improvement
6. **Transformative** (years) - Fundamental changes
7. **Transcendent** (decades+) - Paradigm shifts

### Architecture

The `Architecture` type defines the cognitive design:

- **Paradigm**: Neural, Symbolic, Quantum, Biological, Hybrid, etc.
- **Substrate**: Digital, Biological, Social, Cosmic, etc.
- **Fitness**: Multi-objective evaluation metrics
- **Lineage**: Evolutionary ancestry tracking

## Use Cases

### 1. Multi-Agent AI System

```rust
use omega_core::*;

// Create multiple specialized intelligences
let researcher = Intelligence::new(
    "Research Agent".to_string(),
    Architecture {
        paradigm: Paradigm::Neural,
        substrate: SubstrateType::Digital,
        // ... other fields
    },
);

let executor = Intelligence::new(
    "Execution Agent".to_string(),
    Architecture {
        paradigm: Paradigm::Symbolic,
        substrate: SubstrateType::Digital,
        // ... other fields
    },
);
```

### 2. Memory Management

```rust
use omega_core::*;

// Store different types of memories
let sensory = Memory::new(
    MemoryTier::Immediate,
    MemoryType::Sensory,
    MemoryContent::Sensory(vec![1, 2, 3, 4]),
    0.3,
);

let knowledge = Memory::new(
    MemoryTier::Semantic,
    MemoryType::Knowledge,
    MemoryContent::Text("E = mc²".to_string()),
    0.99,
);

// Check if memories have expired
println!("Sensory expired: {}", sensory.is_expired());
println!("Knowledge expired: {}", knowledge.is_expired());
```

### 3. Temporal Loop Orchestration

```rust
use omega_core::*;
use std::collections::HashMap;

let mut loop_instance = TemporalLoop::new(
    LoopType::Deliberative,
    "Planning Loop".to_string(),
    "Daily strategic planning".to_string(),
);

// Start a cycle
let input = CycleInput {
    data: HashMap::new(),
    context: "morning_planning".to_string(),
    objectives: vec!["optimize_workflow".to_string()],
};

let cycle_id = loop_instance.start_cycle(input);
println!("Started cycle: {}", cycle_id);
```

## Examples

### Complete Intelligence Lifecycle

```rust
use omega_core::*;
use chrono::Utc;

fn main() {
    // 1. Define architecture
    let mut arch = Architecture {
        id: uuid::Uuid::new_v4().to_string(),
        name: "Hybrid Cognitive System".to_string(),
        paradigm: Paradigm::Hybrid,
        substrate: SubstrateType::Digital,
        fitness: Some(FitnessScore {
            overall: 0.85,
            capability: 0.90,
            efficiency: 0.75,
            alignment: 0.88,
            novelty: 0.87,
        }),
        lineage: vec![],
        created_at: Utc::now(),
    };

    // 2. Create intelligence
    let mut intel = Intelligence::new("Omega-1".to_string(), arch);

    // 3. Add capabilities
    intel.capabilities.push("reasoning".to_string());
    intel.capabilities.push("learning".to_string());
    intel.capabilities.push("planning".to_string());

    // 4. Activate
    intel.activate();

    // 5. Update maturity
    intel.maturity = 0.75;

    println!("Intelligence {} operational with {} capabilities",
        intel.name, intel.capabilities.len());
}
```

### Multi-Tier Memory Storage

```rust
use omega_core::*;

fn create_memory_hierarchy() -> Vec<Memory> {
    let mut memories = Vec::new();

    // Immediate sensory memory
    memories.push(Memory::new(
        MemoryTier::Immediate,
        MemoryType::Sensory,
        MemoryContent::Sensory(vec![0; 1024]),
        0.2,
    ));

    // Episodic event memory
    memories.push(Memory::new(
        MemoryTier::Episodic,
        MemoryType::Event,
        MemoryContent::Text("User requested feature X".to_string()),
        0.7,
    ));

    // Semantic knowledge
    memories.push(Memory::new(
        MemoryTier::Semantic,
        MemoryType::Knowledge,
        MemoryContent::Structured(serde_json::json!({
            "concept": "machine learning",
            "definition": "algorithms that improve through experience"
        })),
        0.9,
    ));

    memories
}
```

## Architecture

`omega-core` sits at the foundation of the ExoGenesis Omega ecosystem:

```
┌─────────────────────────────────────────┐
│         omega-runtime                    │  ← Production orchestration
├─────────────────────────────────────────┤
│  omega-memory  │  omega-loops  │ meta-  │  ← High-level subsystems
│                │               │ sona   │
├────────────────┼───────────────┼────────┤
│ omega-agentdb  │ omega-persistence       │  ← Storage layer
├─────────────────────────────────────────┤
│            omega-core                    │  ← Core types & traits
└─────────────────────────────────────────┘
```

All other Omega crates depend on `omega-core` for shared types and interfaces.

## Performance

- Zero-cost abstractions using Rust's type system
- Efficient serialization with serde
- Minimal runtime overhead for type conversions
- Lock-free where possible

## Related Crates

### Storage & Infrastructure
- **[omega-persistence](../omega-persistence)** - SQLite-based persistence layer
- **[omega-agentdb](../omega-agentdb)** - SIMD vector database with HNSW

### Memory & Processing
- **[omega-memory](../omega-memory)** - 12-tier memory implementation
- **[omega-loops](../omega-loops)** - 7 temporal loop orchestration
- **[omega-meta-sona](../omega-meta-sona)** - Self-optimizing architecture search

### Brain-Like Cognition
- **[omega-brain](../omega-brain)** - Unified cognitive architecture
- **[omega-snn](../omega-snn)** - Spiking neural networks
- **[omega-attention](../omega-attention)** - 40 attention mechanisms
- **[omega-consciousness](../omega-consciousness)** - IIT, GWT, Free Energy
- **[omega-hippocampus](../omega-hippocampus)** - Hippocampal memory
- **[omega-sleep](../omega-sleep)** - Sleep/wake consolidation
- **[omega-strange-loops](../omega-strange-loops)** - Self-referential cognition

### Runtime
- **[omega-runtime](../omega-runtime)** - Production runtime orchestrator

## License

Licensed under the MIT License. See [LICENSE](../../LICENSE) for details.
