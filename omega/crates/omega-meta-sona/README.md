# omega-meta-sona

[![Crates.io](https://img.shields.io/crates/v/omega-meta-sona)](https://crates.io/crates/omega-meta-sona)
[![Documentation](https://docs.rs/omega-meta-sona/badge.svg)](https://docs.rs/omega-meta-sona)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Self-Optimizing Neural Architecture (META-SONA) with evolutionary search, MCTS-based architecture discovery, and multi-objective fitness evaluation.

## Overview

`omega-meta-sona` is the intelligence design engine for ExoGenesis Omega. While SONA optimizes weights within a fixed architecture, META-SONA optimizes the architecture itself. It discovers, evaluates, and evolves cognitive architectures using Monte Carlo Tree Search (MCTS), Proximal Policy Optimization (PPO), and multi-objective fitness functions.

META-SONA enables AI systems to design better AI systems—a key capability for recursive self-improvement and open-ended intelligence evolution.

## Features

- **Architecture Search**: MCTS for exploring the space of possible architectures
- **Hyperparameter Optimization**: PPO for fine-tuning architecture parameters
- **Multi-Objective Fitness**: Evaluate capability, efficiency, alignment, and novelty
- **Intelligence Factory**: High-level API for creating and evolving intelligences
- **Lineage Tracking**: Full ancestry and evolution history
- **Parallel Evaluation**: Concurrent fitness assessment for speed
- **Type-Safe Design**: Strongly typed architecture representation

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
omega-meta-sona = "0.1.0"
```

## Quick Start

```rust
use omega_meta_sona::{MetaSONA, IntelligenceSpec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create META-SONA instance
    let mut meta_sona = MetaSONA::new();

    // Define specification for desired intelligence
    let spec = IntelligenceSpec {
        name: "Advanced Reasoner".to_string(),
        min_capability: 0.85,
        max_inference_time: Some(100.0), // milliseconds
        required_capabilities: vec![
            "reasoning".to_string(),
            "planning".to_string(),
        ],
        ..Default::default()
    };

    // Create intelligence that meets specification
    let intelligence = meta_sona.create_intelligence(spec).await?;

    println!("Created: {}", intelligence.name);
    println!("Architecture: {}", intelligence.architecture.name);

    if let Some(fitness) = intelligence.architecture.fitness {
        println!("Fitness: {:.2}", fitness.overall);
        println!("Capability: {:.2}", fitness.capability);
        println!("Efficiency: {:.2}", fitness.efficiency);
    }

    Ok(())
}
```

## Core Concepts

### Architecture Space

META-SONA explores a vast space of possible architectures defined by:

- **Paradigms**: Neural, Symbolic, Quantum, Biological, Hybrid
- **Substrates**: Digital, Biological, Social, Cosmic
- **Components**: Layers, attention mechanisms, memory modules
- **Connections**: Skip connections, recurrent loops, hierarchies
- **Parameters**: Learning rates, layer sizes, activation functions

### MCTS Architecture Search

Monte Carlo Tree Search explores the architecture space:

1. **Selection**: Pick promising architecture branch using UCB1
2. **Expansion**: Generate new architecture variations
3. **Simulation**: Evaluate architecture fitness
4. **Backpropagation**: Update node statistics

The search balances exploration (novel architectures) vs. exploitation (refining promising designs).

### Multi-Objective Fitness

Architectures are evaluated across four dimensions:

```rust
pub struct FitnessScore {
    pub overall: f64,     // Weighted combination
    pub capability: f64,  // Task performance
    pub efficiency: f64,  // Resource usage
    pub alignment: f64,   // Safety & ethics
    pub novelty: f64,     // Innovation metric
}
```

### PPO Optimization

After MCTS finds promising architectures, PPO fine-tunes hyperparameters:

- Continuous optimization of architecture parameters
- Generalized Advantage Estimation (GAE) for gradient estimation
- Clipped surrogate objective prevents destructive updates
- Adaptive learning rates for stable convergence

## Use Cases

### 1. Creating Custom Intelligence

```rust
use omega_meta_sona::{MetaSONA, IntelligenceSpec};

let mut meta_sona = MetaSONA::new();

// Specification for a fast, efficient agent
let spec = IntelligenceSpec {
    name: "Speed Optimizer".to_string(),
    min_capability: 0.75,
    max_inference_time: Some(10.0), // Very fast
    min_efficiency: Some(0.90),      // Very efficient
    paradigm_preference: Some(vec!["neural".to_string()]),
    substrate_preference: Some(vec!["digital".to_string()]),
    ..Default::default()
};

let intelligence = meta_sona.create_intelligence(spec).await?;

println!("Created fast agent: {}", intelligence.name);
println!("Inference time: {}ms",
    intelligence.architecture.fitness.unwrap().efficiency * 100.0);
```

### 2. Evolving an Architecture

```rust
use omega_meta_sona::{MetaSONA, Architecture, Paradigm, SubstrateType};
use chrono::Utc;

let mut meta_sona = MetaSONA::new();

// Start with base architecture
let base = Architecture {
    id: "arch-base".to_string(),
    name: "Transformer".to_string(),
    paradigm: Paradigm::Neural,
    substrate: SubstrateType::Digital,
    fitness: Some(FitnessScore {
        overall: 0.70,
        capability: 0.75,
        efficiency: 0.65,
        alignment: 0.70,
        novelty: 0.60,
    }),
    lineage: vec![],
    created_at: Utc::now(),
};

// Evolve for 5 generations
let evolved = meta_sona.evolve_architecture(base, 5).await?;

println!("Evolution complete!");
println!("Base fitness: 0.70");
println!("Evolved fitness: {:.2}", evolved.fitness.unwrap().overall);
println!("Generation: {}", evolved.lineage.len());
```

### 3. Multi-Objective Optimization

```rust
use omega_meta_sona::{IntelligenceSpec, MetaSONA};

let mut meta_sona = MetaSONA::new();

// Balanced across all objectives
let balanced_spec = IntelligenceSpec {
    name: "Balanced AI".to_string(),
    min_capability: 0.80,
    min_efficiency: Some(0.75),
    min_alignment: Some(0.85),
    min_novelty: Some(0.70),
    ..Default::default()
};

let balanced = meta_sona.create_intelligence(balanced_spec).await?;

// Capability-focused
let capability_spec = IntelligenceSpec {
    name: "Power AI".to_string(),
    min_capability: 0.95,
    min_efficiency: Some(0.50), // Lower bar
    ..Default::default()
};

let powerful = meta_sona.create_intelligence(capability_spec).await?;

// Efficiency-focused
let efficiency_spec = IntelligenceSpec {
    name: "Efficient AI".to_string(),
    min_capability: 0.70,
    min_efficiency: Some(0.95),
    max_inference_time: Some(5.0),
    ..Default::default()
};

let efficient = meta_sona.create_intelligence(efficiency_spec).await?;
```

### 4. Hybrid Architecture Discovery

```rust
use omega_meta_sona::{MetaSONA, IntelligenceSpec};

let mut meta_sona = MetaSONA::new();

// Search for hybrid neural-symbolic architecture
let spec = IntelligenceSpec {
    name: "Hybrid Reasoner".to_string(),
    paradigm_preference: Some(vec![
        "neural".to_string(),
        "symbolic".to_string(),
        "hybrid".to_string(),
    ]),
    required_capabilities: vec![
        "reasoning".to_string(),
        "learning".to_string(),
        "explanation".to_string(),
    ],
    min_capability: 0.85,
    ..Default::default()
};

let hybrid = meta_sona.create_intelligence(spec).await?;

println!("Discovered architecture: {}", hybrid.architecture.name);
println!("Paradigm: {:?}", hybrid.architecture.paradigm);
```

### 5. Lineage Tracking and Analysis

```rust
use omega_meta_sona::{MetaSONA, Architecture};

let mut meta_sona = MetaSONA::new();

// Create base architecture
let spec = IntelligenceSpec::default();
let gen0 = meta_sona.create_intelligence(spec).await?;

// Evolve through multiple generations
let gen1_arch = meta_sona.evolve_architecture(
    gen0.architecture.clone(),
    1
).await?;

let gen2_arch = meta_sona.evolve_architecture(gen1_arch.clone(), 1).await?;
let gen3_arch = meta_sona.evolve_architecture(gen2_arch.clone(), 1).await?;

// Analyze lineage
println!("Evolution history:");
for (i, ancestor_id) in gen3_arch.lineage.iter().enumerate() {
    println!("  Generation {}: {}", i, ancestor_id);
}

println!("\nFitness progression:");
println!("  Gen 0: {:.2}", gen0.architecture.fitness.unwrap().overall);
println!("  Gen 1: {:.2}", gen1_arch.fitness.unwrap().overall);
println!("  Gen 2: {:.2}", gen2_arch.fitness.unwrap().overall);
println!("  Gen 3: {:.2}", gen3_arch.fitness.unwrap().overall);
```

## Examples

### Intelligence Factory Workflow

```rust
use omega_meta_sona::{IntelligenceFactory, IntelligenceSpec};

let mut factory = IntelligenceFactory::new();

// Create multiple specialized intelligences
let specs = vec![
    IntelligenceSpec {
        name: "Researcher".to_string(),
        required_capabilities: vec!["research".to_string(), "analysis".to_string()],
        min_capability: 0.85,
        ..Default::default()
    },
    IntelligenceSpec {
        name: "Executor".to_string(),
        required_capabilities: vec!["planning".to_string(), "execution".to_string()],
        max_inference_time: Some(50.0),
        ..Default::default()
    },
    IntelligenceSpec {
        name: "Reviewer".to_string(),
        required_capabilities: vec!["review".to_string(), "critique".to_string()],
        min_alignment: Some(0.95),
        ..Default::default()
    },
];

let mut intelligences = Vec::new();
for spec in specs {
    let intelligence = factory.create_intelligence(spec).await?;
    intelligences.push(intelligence);
}

println!("Created {} specialized intelligences", intelligences.len());
```

### Custom Fitness Evaluation

```rust
use omega_meta_sona::{FitnessEvaluator, MetricWeight};

let mut evaluator = FitnessEvaluator::new();

// Customize metric weights
evaluator.set_weights(MetricWeight {
    capability: 0.40,
    efficiency: 0.30,
    alignment: 0.20,
    novelty: 0.10,
});

// Evaluate architecture
let fitness = evaluator.evaluate(&architecture).await?;

println!("Custom fitness: {:.2}", fitness.overall);
println!("  Capability (40%): {:.2}", fitness.capability);
println!("  Efficiency (30%): {:.2}", fitness.efficiency);
println!("  Alignment (20%): {:.2}", fitness.alignment);
println!("  Novelty (10%): {:.2}", fitness.novelty);
```

## Architecture

META-SONA's internal structure:

```
┌──────────────────────────────────────────┐
│            MetaSONA                       │
│  - High-level orchestration               │
│  - Intelligence creation API              │
└────────────┬─────────────────────────────┘
             │
             ▼
┌──────────────────────────────────────────┐
│       IntelligenceFactory                 │
│  - Specification processing               │
│  - Architecture assembly                  │
│  - Evolution coordination                 │
└──┬─────────┬──────────────┬──────────────┘
   │         │              │
   ▼         ▼              ▼
┌──────┐ ┌───────┐ ┌──────────────┐
│ MCTS │ │  PPO  │ │   Fitness    │
│Search│ │ Opt.  │ │  Evaluator   │
└──────┘ └───────┘ └──────────────┘
   │         │              │
   ▼         ▼              ▼
┌──────────────────────────────────────────┐
│      ArchitectureSpace                    │
│  - Encoding/decoding                      │
│  - Mutation operators                     │
│  - Crossover operators                    │
└──────────────────────────────────────────┘
```

## Performance

META-SONA performance characteristics:

- **MCTS Search**: ~100-1000 iterations for good results
- **Architecture Evaluation**: ~10-100ms per candidate
- **PPO Optimization**: ~50-200 steps for convergence
- **Total Creation Time**: 1-10 seconds per intelligence

Parallelization:
- Multiple architecture evaluations in parallel
- Batch PPO updates for efficiency
- Async MCTS simulations

## Related Crates

- **[omega-core](../omega-core)** - Core architecture types
- **[omega-loops](../omega-loops)** - Transformative loop for long-term evolution
- **[omega-memory](../omega-memory)** - Memory of successful architectures
- **[omega-agentdb](../omega-agentdb)** - Storage for architecture variants
- **[omega-persistence](../omega-persistence)** - Persisting evolved architectures
- **[omega-runtime](../omega-runtime)** - Runtime deployment of created intelligences
- **[omega-brain](../omega-brain)** - Unified cognitive architecture
- **[omega-snn](../omega-snn)** - Neural substrate for evolved architectures

## License

Licensed under the MIT License. See [LICENSE](../../LICENSE) for details.
