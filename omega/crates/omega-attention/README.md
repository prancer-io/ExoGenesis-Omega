# omega-attention

[![Crates.io](https://img.shields.io/crates/v/omega-attention)](https://crates.io/crates/omega-attention)
[![Documentation](https://docs.rs/omega-attention/badge.svg)](https://docs.rs/omega-attention)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Brain-like selective attention system with 40 attention mechanisms, working memory gating, and top-down/bottom-up processing.

**Part of the [ExoGenesis-Omega](https://github.com/prancer-io/ExoGenesis-Omega) cognitive architecture.**

## Overview

`omega-attention` implements a biologically-inspired attention system modeled after neuroscience research on selective attention and transformer architectures. It provides 40 attention mechanisms ranging from standard scaled dot-product attention to advanced hyperbolic, graph, and memory-augmented variants.

The system combines:
- **Top-Down Attention**: Goal-driven, task-relevant selection
- **Bottom-Up Attention**: Stimulus-driven, salience-based capture
- **Working Memory**: Capacity-limited storage with gated access (7±2 items)
- **Attention Spotlight**: Winner-take-all competition for resource allocation

## Features

- **40 Attention Mechanisms**: Comprehensive library including Flash, Linear, Sparse, Hyperbolic, Graph, Memory-augmented, Multi-head, Cross-attention, and more
- **Salience Computation**: Bottom-up attention based on novelty, contrast, motion, and change detection
- **Priority Maps**: Combined top-down/bottom-up priority for attention allocation
- **Working Memory Gating**: Input/output/forget gates mimicking biological WM
- **Configurable Architecture**: Customize attention dimensions, heads, dropout, and more

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
omega-attention = "1.0.0"
```

## Quick Start

```rust
use omega_attention::{AttentionSystem, AttentionConfig, AttentionType};

fn main() {
    // Create attention system with default configuration
    let config = AttentionConfig::default();
    let mut system = AttentionSystem::new(config);

    // Input to attend to
    let input = vec![0.5; 64];
    let goals = vec![0.8; 64];  // Current goals/task
    let context = vec![0.3; 64];

    // Process through attention system
    let output = system.attend(&input, &goals, &context).unwrap();

    println!("Attention strength: {:.3}", output.max_attention);
    println!("Attended values: {:?}", &output.attended_values[..5]);

    // Check working memory state
    let state = system.state();
    println!("Working memory: {}/{} items", state.wm_items, state.wm_capacity);
}
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                   ATTENTION SYSTEM                          │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌────────────────────┐    ┌────────────────────┐           │
│  │   TOP-DOWN         │    │   BOTTOM-UP        │           │
│  │   (Goal-driven)    │    │   (Salience)       │           │
│  │                    │    │                    │           │
│  │  • Task relevance  │    │  • Novelty         │           │
│  │  • Expected value  │    │  • Contrast        │           │
│  │  • Memory match    │    │  • Motion          │           │
│  └────────┬───────────┘    └────────┬───────────┘           │
│           │                         │                        │
│           └───────────┬─────────────┘                        │
│                       ▼                                      │
│           ┌───────────────────────┐                         │
│           │   ATTENTION CONTROL   │                         │
│           │   (Priority Map)      │                         │
│           └───────────┬───────────┘                         │
│                       ▼                                      │
│           ┌───────────────────────┐                         │
│           │   ATTENTION MECHANISMS│                         │
│           │   (40 types)          │                         │
│           └───────────┬───────────┘                         │
│                       ▼                                      │
│           ┌───────────────────────┐                         │
│           │   WORKING MEMORY      │                         │
│           │   (Gated Access)      │                         │
│           └───────────────────────┘                         │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Attention Mechanisms

### Core Mechanisms

| Type | Description | Use Case |
|------|-------------|----------|
| `ScaledDotProduct` | Standard transformer attention | General purpose |
| `FlashAttention` | Memory-efficient O(N) attention | Long sequences |
| `LinearAttention` | Kernel-based linear complexity | Very long sequences |
| `MultiHeadAttention` | Parallel attention heads | Rich representations |

### Advanced Mechanisms

| Type | Description | Use Case |
|------|-------------|----------|
| `SparseAttention` | Top-k sparsity patterns | Efficiency |
| `HyperbolicAttention` | Hyperbolic space embeddings | Hierarchical data |
| `GraphAttention` | Graph neural network attention | Relational data |
| `MemoryAugmented` | External memory access | Long-term context |
| `CrossAttention` | Query/key from different sources | Multi-modal fusion |

### Biological Mechanisms

| Type | Description | Use Case |
|------|-------------|----------|
| `SalienceAttention` | Bottom-up salience maps | Novelty detection |
| `InhibitionOfReturn` | Temporal attention suppression | Visual search |
| `FeatureIntegration` | Binding features to locations | Object recognition |

## Working Memory

The working memory system implements Miller's "magical number 7±2" with biological gating:

```rust
use omega_attention::{WorkingMemory, WorkingMemoryItem, WMGate};

// Create working memory with capacity 7
let mut wm = WorkingMemory::new(7);

// Configure input gate (controls what enters WM)
wm.input_gate.threshold = 0.5;  // Minimum importance to enter
wm.input_gate.openness = 0.8;   // Gate openness

// Store high-importance item
let item = WorkingMemoryItem::new(vec![1.0, 2.0, 3.0], 0.9);
assert!(wm.try_store(item));  // Passes gate

// Items decay over time
wm.decay(0.1);  // Reduce all activations

// Rehearse to maintain items
wm.rehearse("item_id");  // Boost activation

// Find similar items
let query = vec![1.1, 2.1, 3.1];
let similar = wm.find_similar(&query, 3);
```

## Salience Computation

Bottom-up attention is driven by stimulus salience:

```rust
use omega_attention::{SalienceComputer, SalienceFeature};

let mut computer = SalienceComputer::new();

// Process input to extract salience features
let input = vec![0.5; 64];
let salience_map = computer.compute(&input);

// Individual feature contributions
let features = computer.extract_features(&input);
for feature in features {
    match feature {
        SalienceFeature::Novelty(n) => println!("Novelty: {:.3}", n),
        SalienceFeature::Contrast(c) => println!("Contrast: {:.3}", c),
        SalienceFeature::Motion(m) => println!("Motion: {:.3}", m),
        SalienceFeature::Change(ch) => println!("Change: {:.3}", ch),
    }
}
```

## Configuration

```rust
use omega_attention::AttentionConfig;

let config = AttentionConfig {
    dim: 64,                    // Attention dimension
    num_heads: 8,               // Number of attention heads
    head_dim: 8,                // Dimension per head
    dropout: 0.1,               // Dropout rate
    top_down_weight: 0.6,       // Weight for goal-driven attention
    bottom_up_weight: 0.4,      // Weight for salience-driven attention
    wm_capacity: 7,             // Working memory capacity
    mechanism: AttentionType::MultiHeadAttention,
    ..Default::default()
};
```

## Use Cases

### 1. Selective Processing

```rust
// Focus attention on task-relevant features
let goals = encode_task("summarize the document");
let document = encode_document(text);

let attended = system.attend(&document, &goals, &context)?;
// attended.attended_values contains task-relevant information
```

### 2. Novelty Detection

```rust
// Automatically detect novel/unexpected inputs
let salience = salience_computer.compute(&new_input);
if salience.max() > 0.8 {
    println!("Novel stimulus detected!");
    system.focus(&new_input);  // Shift attention
}
```

### 3. Memory Consolidation

```rust
// Important items enter working memory
let output = system.attend(&input, &goals, &context)?;
if output.max_attention > 0.7 {
    // High attention = important = store in WM
    let wm = system.working_memory();
    println!("Working memory now has {} items", wm.len());
}
```

## Integration with Omega

omega-attention is a core component of the Omega cognitive architecture:

```
omega-brain (Unified Cognitive System)
    └── omega-attention (Selective Processing)
    └── omega-consciousness (Awareness)
    └── omega-hippocampus (Memory)
    └── omega-snn (Neural Substrate)
```

## Performance

- **40 attention mechanisms** for diverse use cases
- **O(N²)** standard attention, **O(N)** linear/flash attention
- **Configurable sparsity** for efficiency
- **Parallel processing** with multi-head attention

## Related Crates

- **[omega-brain](../omega-brain)** - Unified cognitive architecture
- **[omega-consciousness](../omega-consciousness)** - Global workspace and IIT
- **[omega-snn](../omega-snn)** - Spiking neural network substrate
- **[omega-hippocampus](../omega-hippocampus)** - Memory encoding and retrieval

## References

- Vaswani et al. (2017) "Attention Is All You Need"
- Desimone & Duncan (1995) "Neural Mechanisms of Selective Visual Attention"
- Corbetta & Shulman (2002) "Control of Goal-Directed and Stimulus-Driven Attention"
- Cowan (2001) "The Magical Number 4 in Short-Term Memory"

## License

Licensed under the MIT License. See [LICENSE](../../LICENSE) for details.
