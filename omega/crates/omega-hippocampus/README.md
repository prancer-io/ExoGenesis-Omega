# omega-hippocampus

[![Crates.io](https://img.shields.io/crates/v/omega-hippocampus)](https://crates.io/crates/omega-hippocampus)
[![Documentation](https://docs.rs/omega-hippocampus/badge.svg)](https://docs.rs/omega-hippocampus)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Biologically-inspired hippocampal memory system with pattern separation, completion, and sharp-wave ripple replay.

**Part of the [ExoGenesis-Omega](https://github.com/prancer-io/ExoGenesis-Omega) cognitive architecture.**

## Overview

`omega-hippocampus` implements a computational model of the hippocampal formation, the brain region critical for episodic memory and spatial navigation. The model includes:

- **Dentate Gyrus (DG)**: Pattern separation via sparse coding
- **CA3**: Autoassociative network for pattern completion
- **CA1**: Output layer for memory consolidation
- **Entorhinal Cortex**: Grid cells and input/output interface
- **Place Cells**: Spatial memory representations
- **Sharp-Wave Ripples**: Memory replay during rest/sleep

## Features

- **Pattern Separation**: Orthogonalize similar inputs using sparse DG coding
- **Pattern Completion**: Retrieve full patterns from partial cues via CA3
- **Spatial Navigation**: Place cells, grid cells, and head direction cells
- **Memory Replay**: Sharp-wave ripple events for consolidation
- **Configurable Sparsity**: Control DG expansion ratio and activity level
- **Hebbian Learning**: STDP-like synaptic plasticity

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
omega-hippocampus = "1.0.0"
```

## Quick Start

```rust
use omega_hippocampus::{Hippocampus, HippocampusConfig};

fn main() {
    // Create hippocampus with default configuration
    let config = HippocampusConfig::default();
    let mut hippo = Hippocampus::new(config);

    // Encode a new memory
    let input = vec![0.5; 256];
    let memory_id = hippo.encode(&input, None)?;
    println!("Encoded memory: {}", memory_id);

    // Retrieve with partial cue (pattern completion)
    let partial_cue = create_partial_cue(&input, 0.3);  // 30% of original
    let retrieved = hippo.retrieve(&partial_cue)?;

    if let Some(memory) = retrieved {
        println!("Retrieved memory with similarity: {:.3}", memory.similarity);
    }

    // Trigger replay during "sleep"
    let replayed = hippo.replay(10)?;  // Replay 10 memories
    println!("Replayed {} memories", replayed.len());
}
```

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                      HIPPOCAMPAL FORMATION                           │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │                    ENTORHINAL CORTEX (EC)                       │ │
│  │            Grid Cells │ Input/Output Interface                  │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                    ↓ Perforant Path              ↑                   │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │                    DENTATE GYRUS (DG)                           │ │
│  │         Pattern Separation │ Sparse Coding │ 10x Expansion      │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                    ↓ Mossy Fibers                                    │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │                         CA3                                      │ │
│  │    Autoassociative │ Pattern Completion │ Recurrent Connections │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                    ↓ Schaffer Collaterals                           │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │                         CA1                                      │ │
│  │          Output Layer │ Consolidation │ To Neocortex            │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                                                                      │
│  ┌─────────────────────┐  ┌──────────────────────────────────────┐ │
│  │    PLACE CELLS      │  │         SHARP-WAVE RIPPLES           │ │
│  │  Spatial Memory     │  │   Memory Replay │ Consolidation      │ │
│  └─────────────────────┘  └──────────────────────────────────────┘ │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

## Pattern Separation (Dentate Gyrus)

The DG separates similar patterns into distinct representations:

```rust
use omega_hippocampus::{DentateGyrus, GranuleCell};

// Create DG with 10x expansion
let mut dg = DentateGyrus::new(256, 2560, 0.02);  // 2% sparsity

// Encode two similar patterns
let pattern1 = vec![0.5; 256];
let pattern2 = pattern1.iter().map(|x| x + 0.1).collect();  // Slight variation

let sparse1 = dg.encode(&pattern1);
let sparse2 = dg.encode(&pattern2);

// Sparse codes are more orthogonal than original patterns
let original_sim = cosine_similarity(&pattern1, &pattern2);
let sparse_sim = cosine_similarity(&sparse1, &sparse2);

println!("Original similarity: {:.3}", original_sim);    // ~0.99
println!("Sparse similarity: {:.3}", sparse_sim);        // ~0.30
```

## Pattern Completion (CA3)

CA3 implements autoassociative memory for completing partial patterns:

```rust
use omega_hippocampus::{CA3Network, PatternCompletion};

let mut ca3 = CA3Network::new(512, 0.04);  // 4% recurrent connectivity

// Store a pattern
let pattern = vec![0.5; 512];
ca3.store(&pattern)?;

// Retrieve with only 30% of the pattern
let partial = create_partial_pattern(&pattern, 0.3);
let completed = ca3.complete(&partial, 100)?;  // 100 iterations

// Measure completion accuracy
let accuracy = cosine_similarity(&pattern, &completed);
println!("Completion accuracy: {:.3}", accuracy);  // ~0.95
```

## Spatial Navigation

### Place Cells

```rust
use omega_hippocampus::{PlaceCell, PlaceField};

// Create place cell with field at (50, 50) with radius 10
let mut pc = PlaceCell::new(0, (50.0, 50.0), 10.0);

// Compute firing rate at different locations
let rate_at_center = pc.compute(50.0, 50.0);  // ~1.0
let rate_at_edge = pc.compute(60.0, 50.0);     // ~0.37
let rate_outside = pc.compute(100.0, 100.0);   // ~0.0

println!("At center: {:.3}", rate_at_center);
println!("At edge: {:.3}", rate_at_edge);
println!("Outside: {:.3}", rate_outside);
```

### Grid Cells

```rust
use omega_hippocampus::{GridCell, EntorhinalCortex};

// Create grid cell with 30cm spacing
let mut gc = GridCell::new(0, 30.0, 0.0);

// Activation is periodic with hexagonal pattern
let a1 = gc.compute(0.0, 0.0);    // Peak
let a2 = gc.compute(15.0, 0.0);   // Trough
let a3 = gc.compute(30.0, 0.0);   // Adjacent peak

println!("Origin: {:.3}", a1);
println!("Half spacing: {:.3}", a2);
println!("Full spacing: {:.3}", a3);
```

### Head Direction Cells

```rust
use omega_hippocampus::place_cells::HeadDirectionCell;
use std::f64::consts::PI;

// Cell preferring 0 degrees (facing right)
let mut hd = HeadDirectionCell::new(0.0);

let facing_preferred = hd.compute(0.0);   // ~1.0
let facing_opposite = hd.compute(PI);      // ~0.0
let facing_orthogonal = hd.compute(PI/2.0); // ~0.5

println!("Preferred: {:.3}", facing_preferred);
println!("Opposite: {:.3}", facing_opposite);
```

## Sharp-Wave Ripples

Memory replay during rest/sleep:

```rust
use omega_hippocampus::{ReplayBuffer, SharpWaveRipple, ReplayEvent};

let mut buffer = ReplayBuffer::new(1000, 0.7);

// Add memories to buffer
for i in 0..100 {
    let pattern = generate_memory(i);
    buffer.add(pattern, 0.5 + (i as f64 * 0.005));  // Increasing importance
}

// Trigger sharp-wave ripple replay
let ripple = buffer.trigger_ripple()?;

println!("Ripple:");
println!("  Frequency: {:.1} Hz", ripple.frequency);
println!("  Duration: {:.1} ms", ripple.duration_ms);
println!("  Memories replayed: {}", ripple.replayed_count);

// Get replay events
let events = buffer.get_replay_events(10);
for event in events {
    println!("  Replayed memory with importance {:.3}", event.importance);
}
```

## Configuration

```rust
use omega_hippocampus::HippocampusConfig;

let config = HippocampusConfig {
    input_dim: 256,
    dg_size: 2560,         // 10x expansion
    ca3_size: 512,
    ca1_size: 256,
    dg_sparsity: 0.02,     // 2% active
    ca3_recurrence: 0.04,  // 4% recurrent connections
    learning_rate: 0.01,
    replay_buffer_size: 1000,
    ripple_threshold: 0.7,
};

let hippo = Hippocampus::new(config);
```

## Memory Traces

```rust
use omega_hippocampus::MemoryTrace;

// Memory traces contain multi-layer representations
let trace = hippo.encode(&input, Some("important_event".to_string()))?;

println!("Memory trace:");
println!("  ID: {}", trace.id);
println!("  DG code sparsity: {:.3}", sparsity(&trace.dg_code));
println!("  CA3 representation: {} dims", trace.ca3_code.len());
println!("  CA1 output: {} dims", trace.ca1_output.len());
println!("  Strength: {:.3}", trace.strength);
println!("  Age: {:?}", trace.created_at.elapsed());
```

## Use Cases

### 1. Episodic Memory

```rust
// Store episodes with context
let episode = Episode {
    what: encode("met John"),
    where_: encode("coffee shop"),
    when: encode("yesterday"),
};

let memory_id = hippo.encode_episode(&episode)?;

// Later, cue with partial info
let cue = encode("coffee shop");
let recalled = hippo.recall_episode(&cue)?;
// Returns: what=met John, where=coffee shop, when=yesterday
```

### 2. Spatial Memory

```rust
// Build spatial map as agent explores
let mut spatial_map = SpatialMap::new(100.0, 100.0, 20);

for position in agent_path {
    spatial_map.update(position, &observation);
}

// Compute rate map for place cell
let rate_map = spatial_map.compute_rate_map(place_cell_id, num_bins);
```

### 3. Memory Consolidation

```rust
// During "sleep", consolidate important memories
hippo.initiate_consolidation()?;

// Replay prioritized by importance
let consolidated = hippo.consolidate(|memory| {
    memory.importance > 0.5 && memory.replay_count < 3
})?;

println!("Consolidated {} memories", consolidated);
```

## Integration with Omega

```
omega-brain (Unified Integration)
    └── omega-hippocampus (This crate)
            ├── Pattern separation (DG)
            ├── Pattern completion (CA3)
            ├── Memory output (CA1)
            ├── Spatial navigation (EC)
            └── Replay during sleep

Used with:
├── omega-sleep - Triggers replay during SWS/REM
├── omega-attention - What gets encoded
└── omega-consciousness - Conscious recall
```

## Related Crates

- **[omega-brain](../omega-brain)** - Unified cognitive architecture
- **[omega-sleep](../omega-sleep)** - Triggers memory consolidation
- **[omega-memory](../omega-memory)** - Higher-level memory interface
- **[omega-agentdb](../omega-agentdb)** - Vector storage for embeddings

## References

- Rolls, E. T. (2013). "The mechanisms for pattern completion and pattern separation in the hippocampus"
- Marr, D. (1971). "Simple memory: a theory for archicortex"
- O'Keefe, J., & Dostrovsky, J. (1971). "The hippocampus as a spatial map"
- Hafting, T., et al. (2005). "Microstructure of a spatial map in the entorhinal cortex"
- Buzsáki, G. (2015). "Hippocampal sharp wave‐ripple"

## License

Licensed under the MIT License. See [LICENSE](../../LICENSE) for details.
