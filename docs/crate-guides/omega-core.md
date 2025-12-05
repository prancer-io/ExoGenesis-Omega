# omega-core - Core Types and Traits

## Overview

`omega-core` is the foundational crate for the ExoGenesis Omega universal intelligence orchestration system. It provides the core types, traits, and abstractions used by all other Omega subsystems.

**Key Features:**
- **Intelligence Types**: Comprehensive type system for representing any form of intelligence
- **12-Tier Memory System**: Memory types spanning milliseconds to cosmic timescales
- **7 Temporal Loops**: Multi-scale feedback cycles from reflexive to transcendent
- **Architecture Definitions**: Paradigms, substrates, and fitness evaluation
- **Zero Dependencies**: Pure Rust with minimal external dependencies

**Version:** 0.1.0
**Crate:** `omega-core`
**Location:** `omega/crates/omega-core`

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
omega-core = "0.1.0"
```

Or use a local path during development:

```toml
[dependencies]
omega-core = { path = "../omega/crates/omega-core" }
```

## Core Concepts

### 1. Intelligence System

#### Paradigms

Eight fundamental approaches to intelligence:

```rust
pub enum Paradigm {
    Neural,      // Neural networks, deep learning
    Symbolic,    // Logic, knowledge graphs
    Quantum,     // Quantum computing
    Biological,  // Bio-inspired systems
    Social,      // Multi-agent systems
    Physical,    // Physical computation
    Hybrid,      // Combinations of above
    Unknown,     // Undiscovered paradigms
}
```

#### Substrates

Nine levels of computational substrate:

```rust
pub enum SubstrateType {
    Digital,        // Silicon, digital computers
    Biological,     // Neurons, DNA
    Social,         // Human organizations
    Ecological,     // Ecosystem-level
    Geological,     // Planetary processes
    Stellar,        // Star-level processes
    Galactic,       // Galaxy-scale
    Cosmic,         // Universe-scale
    Transcendent,   // Beyond known physics
}
```

#### Intelligence

The core `Intelligence` struct represents any intelligent entity:

```rust
pub struct Intelligence {
    pub id: IntelligenceId,
    pub name: String,
    pub architecture: Architecture,
    pub capabilities: Vec<Capability>,
    pub status: IntelligenceStatus,
    pub generation: u32,
    pub created_at: DateTime<Utc>,
}
```

**Status States:**
- `Initializing` - Being created
- `Running` - Active execution
- `Paused` - Temporarily stopped
- `Learning` - In training mode
- `Evolving` - Architecture evolution
- `Stopped` - Shut down
- `Error` - Fault state

### 2. Memory System (12 Tiers)

Memory organized across 12 temporal/spatial scales:

| Tier | Name | Timescale | Retention | Use Case |
|------|------|-----------|-----------|----------|
| 1 | Immediate | Milliseconds | 1 second | Working memory |
| 2 | ShortTerm | Seconds-Minutes | 10 minutes | Recent context |
| 3 | Session | Hours | 24 hours | Current session |
| 4 | Episodic | Days | 30 days | Specific events |
| 5 | Semantic | Weeks | 1 year | Facts, knowledge |
| 6 | Procedural | Months | 5 years | Skills, procedures |
| 7 | Strategic | Years | 50 years | Long-term patterns |
| 8 | Civilizational | Decades-Centuries | 500 years | Cultural knowledge |
| 9 | Evolutionary | Millennia | 10,000 years | Species adaptations |
| 10 | Planetary | Millions of years | Permanent | Geological patterns |
| 11 | Galactic | Billions of years | Permanent | Cosmic evolution |
| 12 | Cosmic | Age of universe | Permanent | Universal constants |

#### Memory Content Types

```rust
pub enum MemoryContent {
    Text(String),                          // Plain text
    Structured(HashMap<String, Value>),    // JSON-like data
    Vector(Vec<f64>),                      // Embeddings
    Graph(GraphMemory),                    // Knowledge graphs
    Hybrid { /* mixed types */ },          // Combination
}
```

### 3. Temporal Loops (7 Levels)

Nested feedback cycles at different timescales:

| Loop | Name | Cycle Duration | Purpose |
|------|------|----------------|---------|
| 1 | Reflexive | 100ms | Sensory-motor feedback |
| 2 | Reactive | 5 seconds | Quick decisions |
| 3 | Adaptive | 30 minutes | Recent learning |
| 4 | Deliberative | 24 hours | Strategic planning |
| 5 | Evolutionary | 7 days | Systematic improvement |
| 6 | Transformative | 1 year | Capability changes |
| 7 | Transcendent | 10 years | Paradigm shifts |

#### Loop Coordination

Loops interact through four connection types:

```rust
pub enum ConnectionType {
    FeedForward,  // Faster → Slower
    FeedBack,     // Slower → Faster
    Resonance,    // Mutual reinforcement
    Inhibition,   // Suppression
}
```

## API Reference

### Intelligence Creation

```rust
use omega_core::*;
use chrono::Utc;

// Create architecture
let architecture = Architecture {
    id: "arch-001".to_string(),
    name: "Neural Transformer".to_string(),
    paradigm: Paradigm::Neural,
    substrate: SubstrateType::Digital,
    fitness: Some(FitnessScore {
        overall: 0.85,
        capability: 0.90,
        efficiency: 0.80,
        alignment: 0.85,
        novelty: 0.75,
        confidence: 0.88,
    }),
    lineage: vec![],
    created_at: Utc::now(),
};

// Create intelligence
let intelligence = Intelligence::new(
    "My AI".to_string(),
    architecture,
);

println!("Created: {} (Generation {})",
    intelligence.name,
    intelligence.generation
);
```

### Memory Operations

```rust
use omega_core::*;

// Create memory with automatic expiration
let memory = Memory::new(
    MemoryTier::Semantic,
    MemoryType::Knowledge,
    MemoryContent::Text("Rust is a systems programming language".to_string()),
    0.9, // importance
);

// Access memory (updates counters)
memory.access();

// Check expiration
if !memory.is_expired() {
    println!("Memory still valid");
}
```

### Temporal Loop Management

```rust
use omega_core::*;
use std::collections::HashMap;

// Create temporal loop
let mut loop_instance = TemporalLoop::new(
    LoopType::Adaptive,
    "Learning Loop".to_string(),
    "Continuous learning from experience".to_string(),
);

// Start cycle
let input = CycleInput {
    data: HashMap::new(),
    context: "task_execution".to_string(),
    objectives: vec!["learn_pattern".to_string()],
};

let cycle_id = loop_instance.start_cycle(input);

// ... perform work ...

// Complete cycle
let output = CycleOutput {
    results: HashMap::new(),
    insights: vec!["Pattern A detected".to_string()],
    actions: vec![],
    next_objectives: vec!["refine_pattern".to_string()],
};

let metrics = CycleMetrics {
    duration: chrono::Duration::seconds(30),
    success: true,
    quality: 0.85,
    efficiency: 0.90,
    novelty: 0.20,
    alignment: 0.95,
};

loop_instance.complete_cycle(output, metrics);
```

### Fitness Evaluation

```rust
use omega_core::FitnessScore;

let fitness = FitnessScore {
    overall: 0.82,     // Overall fitness (auto-computed or explicit)
    capability: 0.85,  // Task performance
    efficiency: 0.80,  // Resource usage
    alignment: 0.90,   // Goal alignment
    novelty: 0.70,     // Innovation
    confidence: 0.85,  // Certainty of evaluation
};

// Default creates balanced 0.5 scores
let default_fitness = FitnessScore::default();
```

## Common Patterns

### 1. Multi-Tier Memory Strategy

```rust
use omega_core::*;

fn store_experience(content: String, importance: f64) -> Vec<Memory> {
    let mut memories = Vec::new();

    // Store in immediate tier for current processing
    memories.push(Memory::new(
        MemoryTier::Immediate,
        MemoryType::Experience,
        MemoryContent::Text(content.clone()),
        importance,
    ));

    // High-importance items also go to semantic tier
    if importance > 0.8 {
        memories.push(Memory::new(
            MemoryTier::Semantic,
            MemoryType::Knowledge,
            MemoryContent::Text(content),
            importance,
        ));
    }

    memories
}
```

### 2. Loop Hierarchy

```rust
use omega_core::*;

fn setup_loop_hierarchy() -> Vec<TemporalLoop> {
    LoopType::all_loops()
        .into_iter()
        .map(|loop_type| {
            TemporalLoop::new(
                loop_type,
                format!("{:?} Loop", loop_type),
                loop_type.description().to_string(),
            )
        })
        .collect()
}
```

### 3. Capability System

```rust
use omega_core::*;

fn add_reasoning_capability(intelligence: &mut Intelligence) {
    let capability = Capability {
        id: uuid::Uuid::new_v4().to_string(),
        name: "Logical Reasoning".to_string(),
        description: "Ability to perform deductive and inductive reasoning".to_string(),
        category: CapabilityCategory::Reasoning,
    };

    intelligence.capabilities.push(capability);
}
```

### 4. Architecture Evolution

```rust
use omega_core::*;

fn evolve_architecture(
    parent: &Architecture,
    fitness_improvement: f64,
) -> Architecture {
    let mut child = parent.clone();
    child.id = uuid::Uuid::new_v4().to_string();
    child.name = format!("{} (Gen 2)", parent.name);
    child.lineage.push(parent.id.clone());

    // Update fitness
    if let Some(parent_fitness) = &parent.fitness {
        child.fitness = Some(FitnessScore {
            overall: parent_fitness.overall + fitness_improvement,
            capability: parent_fitness.capability + fitness_improvement * 0.8,
            efficiency: parent_fitness.efficiency + fitness_improvement * 1.2,
            ..*parent_fitness
        });
    }

    child.created_at = chrono::Utc::now();
    child
}
```

## Best Practices

### 1. Memory Tier Selection

**DO:**
- Use Immediate (T1) for current context only
- Use Semantic (T5) for important facts
- Use Procedural (T6) for learned skills
- Use higher tiers (T8+) sparingly for truly long-term data

**DON'T:**
- Store everything in Semantic tier
- Use Cosmic tier for temporary data
- Ignore expiration handling

### 2. Loop Type Usage

**DO:**
- Match loop type to task timescale
- Use Reflexive (L1) for real-time responses
- Use Adaptive (L3) for learning loops
- Use Evolutionary (L5) for systematic improvement

**DON'T:**
- Use slow loops for time-critical tasks
- Mix loop types without coordination
- Ignore cycle completion

### 3. Intelligence Status Management

```rust
// Good: Proper state transitions
intelligence.status = IntelligenceStatus::Initializing;
// ... setup ...
intelligence.status = IntelligenceStatus::Running;

// Bad: Skipping states
// intelligence.status = IntelligenceStatus::Running; // Before init!
```

### 4. Fitness Score Computation

**DO:**
- Set confidence based on evaluation quality
- Compute overall as weighted average
- Update all dimensions consistently

**DON'T:**
- Set overall without considering components
- Use perfect 1.0 scores without evidence
- Ignore novelty dimension

## Error Handling

```rust
use omega_core::OmegaError;

fn process_memory(memory: &Memory) -> Result<(), OmegaError> {
    if memory.is_expired() {
        return Err(OmegaError::OperationFailed(
            "Memory has expired".to_string()
        ));
    }

    // Process memory...
    Ok(())
}
```

**Error Types:**
- `IntelligenceNotFound(String)` - Intelligence ID not found
- `MemoryNotFound(String)` - Memory ID not found
- `LoopNotFound(String)` - Loop ID not found
- `InvalidTier(String)` - Invalid memory tier
- `InvalidLoopType(String)` - Invalid loop type
- `OperationFailed(String)` - Generic operation failure

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_memory_expiration() {
        let memory = Memory::new(
            MemoryTier::Immediate,
            MemoryType::Experience,
            MemoryContent::Text("test".to_string()),
            0.5,
        );

        // Immediate tier expires in 1 second
        assert!(memory.expires_at.is_some());
        assert!(!memory.is_expired());
    }

    #[test]
    fn test_loop_cycle() {
        let mut loop_instance = TemporalLoop::new(
            LoopType::Reflexive,
            "Test".to_string(),
            "Test loop".to_string(),
        );

        let input = CycleInput {
            data: HashMap::new(),
            context: "test".to_string(),
            objectives: vec![],
        };

        let cycle_id = loop_instance.start_cycle(input);
        assert!(!cycle_id.is_empty());
        assert_eq!(loop_instance.status, LoopStatus::Running);
    }
}
```

## Integration with Other Crates

### With omega-agentdb

```rust
use omega_core::*;
use omega_agentdb::*;

// Store intelligence metadata in AgentDB
let db = AgentDB::new(AgentDBConfig::default()).await?;
let metadata = serde_json::to_value(&intelligence)?;
let embedding = vec![0.1; 4096]; // From embedding model
db.vector_store(embedding, metadata).await?;
```

### With omega-memory

```rust
use omega_core::*;
use omega_memory::*;

// Create cosmic memory system
let memory_system = CosmicMemory::new().await?;

// Store using omega-core types
let memory = omega_core::Memory::new(/* ... */);
memory_system.store(memory).await?;
```

### With omega-loops

```rust
use omega_core::*;
use omega_loops::*;

// Create loop engine with core types
let mut engine = LoopEngine::new();
engine.initialize().await?;

// Execute cycle
let input = CycleInput { /* ... */ };
engine.execute_cycle(LoopType::Adaptive, input).await?;
```

## Performance Considerations

1. **Memory Allocation**: Use `Vec::with_capacity()` for known sizes
2. **Clone Cost**: `Intelligence` and `Architecture` are `Clone` but contain nested data
3. **Serialization**: All types are `Serialize`/`Deserialize` for persistence
4. **UUID Generation**: Uses v7 (time-based) for sortable IDs

## Advanced Topics

### Custom Memory Content

```rust
use serde_json::json;

let graph_memory = MemoryContent::Graph(GraphMemory {
    nodes: vec![
        MemoryNode {
            id: "node1".to_string(),
            label: "Concept A".to_string(),
            properties: HashMap::new(),
        }
    ],
    edges: vec![
        MemoryEdge {
            from: "node1".to_string(),
            to: "node2".to_string(),
            relationship: "causes".to_string(),
            weight: 0.9,
        }
    ],
});
```

### Loop Emergent Patterns

```rust
let pattern = EmergentPattern {
    id: uuid::Uuid::new_v4().to_string(),
    name: "Spiral Learning".to_string(),
    description: "Repeated refinement across multiple scales".to_string(),
    participating_loops: vec![
        LoopType::Adaptive,
        LoopType::Deliberative,
        LoopType::Evolutionary,
    ],
    discovered_at: Utc::now(),
    strength: 0.75,
};
```

## References

- **Source**: `omega/crates/omega-core`
- **Documentation**: `docs/ARCHITECTURE.md`
- **Tests**: 228 tests passing (100% coverage of core types)
- **Dependencies**: `serde`, `chrono`, `uuid`, `thiserror`

## Version History

- **0.1.0** (2025-01-05): Initial release
  - 12-tier memory system
  - 7 temporal loops
  - Intelligence type system
  - Architecture definitions
