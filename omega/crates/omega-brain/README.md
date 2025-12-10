# omega-brain

[![Crates.io](https://img.shields.io/crates/v/omega-brain)](https://crates.io/crates/omega-brain)
[![Documentation](https://docs.rs/omega-brain/badge.svg)](https://docs.rs/omega-brain)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Unified brain-like cognitive architecture integrating all Omega components into a coherent cognitive system.

## Overview

`omega-brain` is the central integration crate for the Omega cognitive architecture. It combines neural processing, attention, consciousness, memory, sleep, and self-awareness into a single unified system that processes information in a brain-like manner.

The cognitive cycle follows: **Perception → Attention → Processing → Consciousness → Memory → Action**

## Features

- **Neural Substrate**: Self-contained spiking network with LIF neurons and STDP learning
- **Attention System**: 40 attention mechanisms with top-down/bottom-up processing
- **Consciousness Core**: IIT (Phi), Global Workspace Theory, and Free Energy Principle
- **Memory System**: Hippocampal pattern separation/completion with replay
- **Sleep System**: SWS/REM consolidation cycles with circadian rhythm
- **Self-Awareness**: Strange loops, meta-cognition, and self-model
- **Runtime Adaptation**: MicroLoRA/BaseLoRA, EWC++, and ReasoningBank

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
omega-brain = "1.0.0"
```

## Quick Start

```rust
use omega_brain::{OmegaBrain, BrainConfig};

fn main() {
    // Create brain with default configuration
    let brain = OmegaBrain::new();

    // Process input through the cognitive cycle
    let input = vec![0.5; 32];
    let result = brain.process(&input).unwrap();

    println!("Consciousness level: {:.3}", result.consciousness_level);
    println!("Attention strength: {:.3}", result.attention_strength);
    println!("Memory encoded: {}", result.memory_encoded);

    // Get brain state
    let state = brain.state();
    println!("Cognitive mode: {}", state.cognitive_state.mode);
    println!("Self-reference: {:.3}", state.self_reference);
}
```

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                        OMEGA BRAIN                                   │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                    COGNITIVE CYCLE                           │   │
│  │  Input → Neural → Attention → Consciousness → Memory → Out  │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                              ↕                                       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │
│  │    NEURAL    │  │  ATTENTION   │  │CONSCIOUSNESS │              │
│  │  SUBSTRATE   │  │   SYSTEM     │  │    CORE      │              │
│  │              │  │              │  │              │              │
│  │ • LIF Neurons│  │ • 40 Mechs   │  │ • IIT (Φ)    │              │
│  │ • STDP       │  │ • Top-Down   │  │ • GWT        │              │
│  │ • Neuromod   │  │ • Bottom-Up  │  │ • Free Energy│              │
│  └──────────────┘  └──────────────┘  └──────────────┘              │
│                              ↕                                       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │
│  │    MEMORY    │  │    SLEEP     │  │    SELF      │              │
│  │   SYSTEM     │  │   SYSTEM     │  │  AWARENESS   │              │
│  │              │  │              │  │              │              │
│  │ • Hippocampus│  │ • SWS/REM    │  │ • Self-Model │              │
│  │ • Replay     │  │ • Spindles   │  │ • Meta-Cog   │              │
│  │ • Consolid.  │  │ • Circadian  │  │ • Loops      │              │
│  └──────────────┘  └──────────────┘  └──────────────┘              │
│                              ↕                                       │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                  RUNTIME ADAPTATION                          │   │
│  │     MicroLoRA (instant) │ BaseLoRA (long-term) │ EWC++      │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

## Cognitive Processing

### Full Cognitive Cycle

```rust
use omega_brain::OmegaBrain;

let brain = OmegaBrain::new();

// Full processing cycle
let input = vec![0.5; 32];
let result = brain.process(&input)?;

// Result contains:
// - output: Processed output vector
// - consciousness_level: 0-1 awareness level
// - attention_strength: How strongly input was attended
// - memory_encoded: Whether input was stored in memory
// - strange_loop_detected: Self-referential processing occurred
// - processing_time_ms: Cycle duration
```

### Directed Cognition

```rust
// Think about a specific topic
let topic = encode("What is consciousness?");
let thought = brain.think_about(&topic)?;

// Recall a memory
let cue = encode("yesterday's meeting");
if let Some(memory) = brain.recall(&cue)? {
    println!("Recalled: {:?}", memory);
}

// Store a new memory
let content = encode("Important fact");
brain.remember(&content, 0.9)?;  // High importance
```

### Sleep and Consolidation

```rust
// Enter sleep mode
brain.sleep()?;

// During sleep, memories are consolidated
// SWS: Declarative memory consolidation
// REM: Procedural and emotional memory

// Check if dreaming (REM sleep)
if brain.is_dreaming() {
    println!("Currently in REM sleep");
}

// Wake up
brain.wake()?;

// Force consolidation without full sleep
let consolidated = brain.consolidate_memories()?;
println!("Consolidated {} memories", consolidated);
```

## Brain State and Metrics

### State Monitoring

```rust
let state = brain.state();

println!("Cognitive State:");
println!("  Mode: {}", state.cognitive_state.mode);
println!("  Activity: {:.3}", state.cognitive_state.activity_level);
println!("  Integration: {:.3}", state.cognitive_state.integration);

println!("Awareness:");
println!("  Consciousness: {:.3}", state.consciousness_level);
println!("  Self-reference: {:.3}", state.self_reference);
println!("  Cycle count: {}", state.cycle_count);

if let Some(stage) = state.sleep_stage {
    println!("  Sleep stage: {}", stage);
}
```

### Metrics

```rust
let metrics = brain.metrics();

println!("Brain Metrics:");
println!("  Total cycles: {}", metrics.cycles);
println!("  Avg processing: {:.2}ms", metrics.avg_processing_time);
println!("  IIT Phi (Φ): {:.3}", metrics.phi);
println!("  Free energy: {:.3}", metrics.free_energy);
println!("  Consolidation: {:.1}%", metrics.consolidation_ratio * 100.0);
println!("  Strange loops: {}", metrics.strange_loop_count);
println!("  Spike rate: {:.1} Hz", metrics.spike_rate);
```

## Configuration

```rust
use omega_brain::{BrainConfig, BrainMode};

let config = BrainConfig {
    // Neural substrate
    input_dim: 32,
    hidden_dim: 64,
    output_dim: 32,

    // Consciousness
    phi_threshold: 0.1,      // Minimum Phi for consciousness
    workspace_capacity: 7,    // Global workspace size

    // Memory
    memory_capacity: 10000,   // Max memory traces
    consolidation_rate: 0.1,  // Memory consolidation speed

    // Sleep
    sleep_pressure_rate: 0.01,
    circadian_period: 24.0,   // Hours

    // Self-awareness
    meta_levels: 5,           // Depth of meta-cognition
    self_model_dim: 32,       // Self-model vector size

    // Mode
    mode: BrainMode::Awake,

    ..Default::default()
};

let brain = OmegaBrain::with_config(config);
```

## Runtime Adaptation

The brain uses continuous learning mechanisms inspired by [ruvector-sona](https://crates.io/crates/ruvector-sona):

### MicroLoRA (Instant Adaptation)

```rust
use omega_brain::runtime_adaptation::{LoRAAdapter, LoRAConfig, LoRARank};

// MicroLoRA: Rank 1-2 for immediate context adaptation
let config = LoRAConfig {
    rank: LoRARank::Micro(2),
    alpha: 4.0,
    learning_rate: 0.01,
    dim: 32,
    ..Default::default()
};

let mut adapter = LoRAAdapter::new(config);

// Instant adaptation
adapter.update(&input, &target);
let adapted = adapter.apply(&input);
```

### BaseLoRA (Long-Term Learning)

```rust
// BaseLoRA: Rank 4-16 for skill acquisition
let config = LoRAConfig {
    rank: LoRARank::Base(8),
    alpha: 16.0,
    learning_rate: 0.001,
    dim: 32,
    ..Default::default()
};
```

### EWC++ (Preventing Catastrophic Forgetting)

```rust
use omega_brain::runtime_adaptation::EWCPlusPlus;

let mut ewc = EWCPlusPlus::new(32, 1000.0);  // dim=32, lambda=1000

// Update Fisher information with gradients
ewc.update_fisher(&gradients);

// Store optimal weights after learning a task
ewc.store_optimal(&current_weights);

// Compute regularization penalty
let penalty = ewc.penalty(&new_weights);
```

### ReasoningBank (Pattern Storage)

```rust
use omega_brain::runtime_adaptation::{ReasoningBank, ReasoningPattern};

let mut bank = ReasoningBank::new(10, 1000);  // 10 clusters, 1000 max

// Store successful reasoning patterns
let pattern = ReasoningPattern {
    id: "pattern_1".to_string(),
    input: input_embedding,
    output: output_embedding,
    score: 0.95,
    usage_count: 1,
    cluster_id: 0,
};
bank.store(pattern);

// Retrieve similar patterns
let similar = bank.retrieve(&query_embedding, 5);
```

## Self-Awareness

The brain maintains a self-model through strange loops:

```rust
// Get current self-state
let self_state = brain.self_state();

// Check consciousness level
let consciousness = brain.consciousness_level();
println!("Consciousness: {:.3}", consciousness);

// Get IIT Phi value
let phi = brain.phi();
println!("Integrated information (Φ): {:.3}", phi);
```

## Thread Safety

OmegaBrain uses `Arc<RwLock<T>>` for all components, enabling safe concurrent access:

```rust
use std::sync::Arc;
use std::thread;

let brain = Arc::new(OmegaBrain::new());

// Spawn multiple processing threads
let handles: Vec<_> = (0..4).map(|i| {
    let brain = Arc::clone(&brain);
    thread::spawn(move || {
        let input = vec![i as f64 / 4.0; 32];
        brain.process(&input)
    })
}).collect();

for handle in handles {
    let result = handle.join().unwrap();
    println!("Result: {:?}", result);
}
```

## Integration with Omega

omega-brain is the apex integration layer:

```
omega-brain (This crate - Unified Integration)
├── Implements own:
│   ├── neural_substrate (LIF, STDP, neuromodulation)
│   ├── attention_system (40 mechanisms)
│   ├── consciousness_core (IIT, GWT, FEP)
│   ├── memory_system (hippocampus, replay)
│   ├── sleep_system (SWS, REM, circadian)
│   ├── self_awareness (strange loops, meta-cognition)
│   └── runtime_adaptation (LoRA, EWC++, ReasoningBank)
│
├── Uses:
│   ├── omega-core (Core types)
│   └── parking_lot (Efficient locks)
│
└── Used by:
    └── omega-runtime (Production runtime)
```

## Related Crates

- **[omega-snn](../omega-snn)** - Standalone spiking neural networks
- **[omega-attention](../omega-attention)** - Standalone attention mechanisms
- **[omega-consciousness](../omega-consciousness)** - Standalone consciousness models
- **[omega-hippocampus](../omega-hippocampus)** - Standalone hippocampal memory
- **[omega-sleep](../omega-sleep)** - Standalone sleep/wake cycles
- **[omega-strange-loops](../omega-strange-loops)** - Standalone self-reference
- **[omega-runtime](../omega-runtime)** - Production runtime

## References

- Hofstadter (2007) "I Am a Strange Loop"
- Tononi (2004) "Integrated Information Theory of Consciousness"
- Baars (1988) "A Cognitive Theory of Consciousness"
- Friston (2010) "The Free-Energy Principle"
- Diekelmann & Born (2010) "Sleep and Memory Consolidation"
- Hu et al. (2021) "LoRA: Low-Rank Adaptation of LLMs"

## License

Licensed under the MIT License. See [LICENSE](../../LICENSE) for details.
