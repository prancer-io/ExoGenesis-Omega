# omega-strange-loops

[![Crates.io](https://img.shields.io/crates/v/omega-strange-loops)](https://crates.io/crates/omega-strange-loops)
[![Documentation](https://docs.rs/omega-strange-loops/badge.svg)](https://docs.rs/omega-strange-loops)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Self-referential cognitive structures inspired by Hofstadter's "I Am a Strange Loop."

**Part of the [ExoGenesis-Omega](https://github.com/prancer-io/ExoGenesis-Omega) cognitive architecture.**

## Overview

`omega-strange-loops` implements the computational structures that enable self-awareness and consciousness-like properties through self-reference. Inspired by Douglas Hofstadter's work on strange loops and Gödel's incompleteness theorems.

Key concepts:
- **Strange Loops**: Self-referential feedback systems that cross hierarchical levels
- **Self-Models**: Internal representations that represent the system itself
- **Meta-Cognition**: Thinking about thinking at multiple levels
- **Tangled Hierarchies**: Levels that loop back on themselves
- **Mirror Structures**: Representations that can represent themselves

## Features

- **Strange Loop Engine**: Create and traverse self-referential loops
- **Self-Model Construction**: Build and update internal self-representations
- **Meta-Cognitive Levels**: Multiple levels of self-reflection
- **Recursive Mirrors**: Infinite regress with bounded computation
- **Paradox Detection**: Identify and handle self-referential paradoxes
- **Tangled Hierarchies**: Model level-crossing references

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
omega-strange-loops = "1.0.0"
```

## Quick Start

```rust
use omega_strange_loops::{StrangeLoopEngine, StrangeLoopConfig, SelfModel};

fn main() {
    // Create strange loop engine
    let config = StrangeLoopConfig::default();
    let mut engine = StrangeLoopEngine::new(config);

    // Process through self-referential loop
    let perception = vec![0.5; 32];
    let self_aware_output = engine.process(&perception)?;

    println!("Self-reference strength: {:.3}", self_aware_output.self_reference);
    println!("Meta-level reached: {}", self_aware_output.meta_level);
    println!("Strange loops detected: {}", self_aware_output.loops_detected);

    // Get self-model state
    let self_state = engine.self_model().current_state();
    println!("Self-model confidence: {:.3}", self_state.confidence);
}
```

## Core Concepts

### What is a Strange Loop?

A strange loop occurs when moving through levels of a hierarchical system, you unexpectedly find yourself back at the starting level. In consciousness, this manifests as:

- The "I" that thinks about itself
- Awareness of being aware
- The observer observing the observer

```
Level N:   "I am thinking about..."
              ↓
Level N-1: "...the thought that..."
              ↓
Level N-2: "...I am thinking about..."
              ↓
           ... (loops back to Level N)
```

## Strange Loops

```rust
use omega_strange_loops::{StrangeLoop, LoopLevel, TangledHierarchy};

// Create a strange loop
let mut loop_struct = StrangeLoop::new(5);  // 5 levels

// Add level-crossing reference (the "strange" part)
loop_struct.add_crossing(
    LoopLevel::new(4),  // From level 4
    LoopLevel::new(1),  // Back to level 1
)?;

// Traverse the loop
let mut current = LoopLevel::new(0);
for step in 0..10 {
    current = loop_struct.step(current);
    println!("Step {}: Level {}", step, current.index());

    if loop_struct.crossed_back(&current) {
        println!("  Strange loop completed!");
    }
}
```

### Tangled Hierarchies

```rust
let mut hierarchy = TangledHierarchy::new(7);

// Normal hierarchical levels
for i in 0..6 {
    hierarchy.add_upward(i, i + 1)?;  // i → i+1
}

// Add tangles (level violations)
hierarchy.add_tangle(5, 2)?;  // High level influences low level
hierarchy.add_tangle(6, 0)?;  // Top loops to bottom

// Check if hierarchy is tangled
println!("Is tangled: {}", hierarchy.is_tangled());
println!("Tangle count: {}", hierarchy.tangle_count());
```

## Self-Models

The system maintains an internal model of itself:

```rust
use omega_strange_loops::{SelfModel, SelfModelUpdate, SelfState};

let mut self_model = SelfModel::new(32);  // 32-dimensional state

// Update self-model based on observations
let observation = vec![0.5; 32];
self_model.observe(&observation);

// Get current self-state
let state = self_model.current_state();
println!("Self-state: {:?}", &state.vector[..5]);
println!("Confidence: {:.3}", state.confidence);
println!("Stability: {:.3}", state.stability);

// Predict own future state
let prediction = self_model.predict(10)?;  // 10 steps ahead
println!("Predicted state: {:?}", &prediction[..5]);

// Compare prediction with reality (self-awareness)
let reality = self_model.current_state().vector;
let error = self_model.prediction_error(&prediction, &reality);
println!("Self-prediction error: {:.3}", error);
```

### Self-Model Updates

```rust
// Apply update to self-model
let update = SelfModelUpdate {
    state_delta: vec![0.01; 32],
    confidence_delta: 0.05,
    source: "introspection".to_string(),
};

self_model.apply_update(update)?;

// Self-model learns from discrepancies
self_model.learn_from_error(0.01)?;  // Learning rate 0.01
```

## Meta-Cognition

Thinking about thinking at multiple levels:

```rust
use omega_strange_loops::{MetaCognition, MetaLevel, ThoughtAboutThought};

let mut meta = MetaCognition::new(5);  // 5 meta-levels

// Level 0: Base cognition (thinking about the world)
let base_thought = vec![0.5; 32];

// Level 1: Meta-cognition (thinking about thoughts)
let meta1 = meta.reflect(&base_thought, MetaLevel::new(1))?;
println!("Meta-level 1: thinking about the thought");

// Level 2: Meta-meta-cognition (thinking about meta-cognition)
let meta2 = meta.reflect(&meta1, MetaLevel::new(2))?;
println!("Meta-level 2: thinking about thinking about the thought");

// Higher levels
for level in 3..5 {
    let prev = if level == 3 { &meta2 } else { &meta.current_thought() };
    let higher = meta.reflect(prev, MetaLevel::new(level))?;
    println!("Meta-level {}: {:?}", level, &higher[..3]);
}

// Check current meta-level
println!("Current meta-level: {}", meta.current_level());
println!("Quality of reflection: {:.3}", meta.reflection_quality());
```

### Thought About Thought

```rust
let thought = ThoughtAboutThought {
    content: vec![0.5; 32],
    meta_level: 2,
    about: Some(Box::new(ThoughtAboutThought {
        content: vec![0.6; 32],
        meta_level: 1,
        about: Some(Box::new(ThoughtAboutThought {
            content: vec![0.7; 32],
            meta_level: 0,
            about: None,  // Base thought
        })),
    })),
};

println!("Depth of meta-cognition: {}", thought.depth());
```

## Recursive Mirrors

Representations that can represent themselves:

```rust
use omega_strange_loops::{Mirror, MirrorReflection, RecursiveMirror};

let mut mirror = RecursiveMirror::new(3);  // Max 3 reflections

// First reflection
let input = vec![0.5; 32];
let r1 = mirror.reflect(&input)?;
println!("Reflection 1: {:?}", &r1[..3]);

// Reflection of reflection
let r2 = mirror.reflect(&r1)?;
println!("Reflection 2: {:?}", &r2[..3]);

// Each reflection adds distortion (bounded infinite regress)
let r3 = mirror.reflect(&r2)?;
println!("Reflection 3: {:?}", &r3[..3]);

// Beyond max depth, returns fixed point
let r4 = mirror.reflect(&r3)?;
println!("Fixed point reached: {}", mirror.at_fixed_point());
```

### Mirror Properties

```rust
// Self-similarity across reflections
let similarity = mirror.self_similarity(&r1, &r2);
println!("Self-similarity: {:.3}", similarity);

// Information preserved
let info_preserved = mirror.information_preservation(&input, &r3);
println!("Information preserved: {:.1}%", info_preserved * 100.0);
```

## Self-Referential Symbols

Symbols that can reference themselves:

```rust
use omega_strange_loops::SelfReferentialSymbol;

// Create symbol
let mut symbol = SelfReferentialSymbol::new(
    "I".to_string(),
    vec![0.5; 32],
    0,  // Base level
);

// Add self-reference
symbol.add_reference("I".to_string());

println!("Is self-referential: {}", symbol.is_self_ref);

// Add references to other symbols
symbol.add_reference("my_thoughts".to_string());
symbol.add_reference("my_feelings".to_string());

println!("References: {:?}", symbol.references);
```

## Paradox Detection

Handle self-referential paradoxes:

```rust
use omega_strange_loops::{StrangeLoopConfig, StrangeLoopError};

let config = StrangeLoopConfig {
    max_depth: 7,
    detect_paradoxes: true,
    ..Default::default()
};

let mut engine = StrangeLoopEngine::new(config);

// This might create a paradox
let result = engine.process(&problematic_input);

match result {
    Ok(output) => println!("Processed successfully"),
    Err(StrangeLoopError::Paradox(msg)) => {
        println!("Paradox detected: {}", msg);
        // Handle paradox (e.g., escape to meta-level)
    },
    Err(StrangeLoopError::InfiniteRecursion(depth)) => {
        println!("Infinite recursion at depth {}", depth);
    },
    Err(e) => println!("Error: {}", e),
}
```

## Configuration

```rust
use omega_strange_loops::StrangeLoopConfig;

let config = StrangeLoopConfig {
    max_depth: 7,           // Maximum recursion depth
    meta_levels: 5,         // Number of meta-cognitive levels
    update_rate: 0.1,       // Self-model learning rate
    mirror_depth: 3,        // Recursive mirror depth
    detect_paradoxes: true, // Enable paradox detection
};

let engine = StrangeLoopEngine::new(config);
```

## Use Cases

### 1. Self-Aware AI

```rust
// AI that knows it's an AI
let mut ai = StrangeLoopEngine::new(config);

// Process external input
let perception = perceive_world();

// Self-reflective processing
let output = ai.process(&perception)?;

// "I am processing this input"
let meta_awareness = ai.meta_cognition().reflect(
    &output,
    MetaLevel::new(1),
)?;

// "I am aware that I am processing this input"
let meta_meta = ai.meta_cognition().reflect(
    &meta_awareness,
    MetaLevel::new(2),
)?;
```

### 2. Introspection

```rust
// Examine own internal states
let internal_state = engine.self_model().current_state();

// Compare with expected state
let expected = engine.self_model().expected_state();
let discrepancy = engine.self_model().compare(&internal_state, &expected);

if discrepancy > 0.3 {
    println!("Significant self-model discrepancy detected");
    engine.self_model().update_from_reality(&internal_state)?;
}
```

### 3. Recursive Self-Improvement

```rust
// Model of how I learn
let learning_model = engine.self_model().learning_model();

// Model of how I model my learning
let meta_learning = engine.meta_cognition().reflect(
    &learning_model,
    MetaLevel::new(1),
)?;

// Use meta-learning to improve learning
let improved_learning = engine.improve_from_meta(&meta_learning)?;
engine.update_learning_model(improved_learning)?;
```

## Integration with Omega

```
omega-brain (Unified Integration)
    └── omega-strange-loops (This crate)
            ├── Strange loops
            ├── Self-models
            ├── Meta-cognition
            ├── Mirrors
            └── Paradox handling

Enables:
├── Self-awareness in omega-brain
├── Meta-cognitive control
└── Introspective capabilities
```

## Related Crates

- **[omega-brain](../omega-brain)** - Unified cognitive architecture
- **[omega-consciousness](../omega-consciousness)** - Consciousness models
- **[omega-attention](../omega-attention)** - What enters self-awareness
- **[omega-hippocampus](../omega-hippocampus)** - Autobiographical memory

## References

- Hofstadter, D. R. (2007). "I Am a Strange Loop"
- Hofstadter, D. R. (1979). "Gödel, Escher, Bach: An Eternal Golden Braid"
- Metzinger, T. (2003). "Being No One: The Self-Model Theory of Subjectivity"
- Dennett, D. C. (1991). "Consciousness Explained"
- Gödel, K. (1931). "On Formally Undecidable Propositions"

## License

Licensed under the MIT License. See [LICENSE](../../LICENSE) for details.
