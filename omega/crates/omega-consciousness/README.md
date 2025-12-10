# omega-consciousness

[![Crates.io](https://img.shields.io/crates/v/omega-consciousness)](https://crates.io/crates/omega-consciousness)
[![Documentation](https://docs.rs/omega-consciousness/badge.svg)](https://docs.rs/omega-consciousness)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Computational substrate for conscious experience implementing IIT, Free Energy Principle, and Global Workspace Theory.

## Overview

`omega-consciousness` implements the leading scientific theories of consciousness as computational models:

- **Integrated Information Theory (IIT)**: Computes Phi (Φ), the measure of integrated information
- **Free Energy Principle (FEP)**: Predictive processing and active inference
- **Global Workspace Theory (GWT)**: Competition for conscious access via broadcast
- **Emergence Detection**: Identifies novel causal powers and self-organization

These theories are implemented as composable modules that work together to create a unified consciousness substrate.

## Features

- **IIT Phi (Φ) Computation**: Measure integrated information in system states
- **Cause-Effect Structures**: Information geometry and intrinsic causation
- **Predictive Hierarchy**: Multi-level prediction error minimization
- **Active Inference**: Action selection to minimize free energy
- **Global Workspace**: Coalition formation and broadcast mechanism
- **Emergence Metrics**: Detect downward causation and self-organization

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
omega-consciousness = "1.0.0"
```

## Quick Start

```rust
use omega_consciousness::{ConsciousnessEngine, ConsciousnessConfig};

fn main() {
    // Create consciousness engine
    let config = ConsciousnessConfig::default();
    let mut engine = ConsciousnessEngine::new(config);

    // Process input through consciousness system
    let input = vec![0.5; 64];
    let context = vec![0.3; 64];

    let state = engine.process(&input, &context).unwrap();

    println!("IIT Phi (Φ): {:.3}", state.phi);
    println!("Free Energy: {:.3}", state.free_energy);
    println!("Is Conscious: {}", state.is_conscious);
    println!("Emergence: {:.3}", state.emergence);
}
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    CONSCIOUSNESS SYSTEM                      │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌────────────────────────────────────────────────────────┐ │
│  │                 GLOBAL WORKSPACE                        │ │
│  │     Broadcast │ Competition │ Integration              │ │
│  └────────────────────────────────────────────────────────┘ │
│                          ↕                                   │
│  ┌──────────────────┐  ┌──────────────────────────────────┐ │
│  │ IIT (Φ)          │  │ FREE ENERGY PRINCIPLE            │ │
│  │                  │  │                                  │ │
│  │ • Integration    │  │ • Prediction hierarchy           │ │
│  │ • Exclusion      │  │ • Error minimization             │ │
│  │ • Composition    │  │ • Active inference               │ │
│  └──────────────────┘  └──────────────────────────────────┘ │
│                          ↕                                   │
│  ┌────────────────────────────────────────────────────────┐ │
│  │              EMERGENCE DETECTION                        │ │
│  │   Downward causation │ Novel powers │ Self-organization│ │
│  └────────────────────────────────────────────────────────┘ │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Integrated Information Theory (IIT)

IIT posits that consciousness corresponds to integrated information (Φ).

### Computing Phi

```rust
use omega_consciousness::{PhiComputer, IntegratedInformation};

let mut phi_computer = PhiComputer::new(64);

// Compute Phi for a system state
let state = vec![0.5; 64];
let phi = phi_computer.compute_phi(&state)?;

println!("Integrated information: {:.4}", phi);

// Check against consciousness threshold
let threshold = 0.1;
if phi > threshold {
    println!("System is conscious (Φ > {})", threshold);
}
```

### Cause-Effect Structures

```rust
use omega_consciousness::{CauseEffectStructure, Partition};

let phi_computer = PhiComputer::new(16);

// Analyze cause-effect structure
let state = vec![1.0; 16];
let ces = phi_computer.analyze_structure(&state)?;

println!("Cause information: {:.3}", ces.cause_info);
println!("Effect information: {:.3}", ces.effect_info);
println!("Integration: {:.3}", ces.integrated_info);
```

## Free Energy Principle

The FEP models organisms as minimizing surprise/free energy through prediction.

### Predictive Processing

```rust
use omega_consciousness::{FreeEnergyMinimizer, PredictiveHierarchy};

// Create 5-level predictive hierarchy
let mut fep = FreeEnergyMinimizer::new(5, 64);

// Process sensory input
let sensory_input = vec![0.5; 64];
let context = vec![0.3; 64];

let (free_energy, prediction_error) = fep.process(&sensory_input, &context)?;

println!("Free energy: {:.3}", free_energy);
println!("Prediction error: {:.3}", prediction_error);
```

### Active Inference

```rust
use omega_consciousness::ActiveInference;

let mut inference = ActiveInference::new(64, 4);  // 64 state dim, 4 actions

// Select action that minimizes expected free energy
let current_state = vec![0.5; 64];
let action = inference.select_action(&current_state)?;

println!("Selected action: {}", action);
```

## Global Workspace Theory

GWT models consciousness as broadcast of winning content to specialized processors.

### Workspace Competition

```rust
use omega_consciousness::{GlobalWorkspace, WorkspaceContent, Coalition};

let mut workspace = GlobalWorkspace::new(7);  // Capacity 7

// Multiple contents compete for access
let content1 = WorkspaceContent::new(
    vec![0.8; 64],
    0.9,  // High activation
    "visual_perception".to_string(),
);

let content2 = WorkspaceContent::new(
    vec![0.5; 64],
    0.6,  // Lower activation
    "auditory_perception".to_string(),
);

workspace.compete(content1);
workspace.compete(content2);

// Broadcast winning content
workspace.broadcast();

// Check what's in workspace
for id in workspace.content_ids() {
    println!("Conscious content: {}", id);
}
```

### Coalition Formation

```rust
// Coalitions of related content
let coalition = Coalition::new(vec![
    WorkspaceContent::new(vec![0.5; 64], 0.8, "face".to_string()),
    WorkspaceContent::new(vec![0.6; 64], 0.7, "voice".to_string()),
    WorkspaceContent::new(vec![0.4; 64], 0.6, "name".to_string()),
]);

// Coalition strength is sum of members
println!("Coalition strength: {:.3}", coalition.total_activation());
```

## Emergence Detection

Detect when new causal powers emerge from system interactions.

### Self-Organization

```rust
use omega_consciousness::{EmergenceDetector, SelfOrganization};

let mut detector = EmergenceDetector::new();

let state = vec![0.5; 64];
let phi = 0.5;
let free_energy = 0.3;

// Detect emergence level
let emergence = detector.detect(&state, phi, free_energy);
println!("Emergence level: {:.3}", emergence);

// Check self-organization
let mut so = SelfOrganization::new();
so.order_parameter = 0.6;
so.entropy = 0.4;
so.complexity = 0.7;
so.criticality = 0.8;

if so.is_self_organizing() {
    println!("System is self-organizing");
}

if so.is_critical() {
    println!("System is near critical point");
}
```

### Causal Powers

```rust
use omega_consciousness::CausalPower;

let detector = EmergenceDetector::new();

// Measure causal power at different scales
let micro_state = vec![0.5; 64];
let macro_state = vec![0.5; 16];

let micro_power = detector.measure_causal_power(&micro_state);
let macro_power = detector.measure_causal_power(&macro_state);

// Novel macro causation if macro > sum of micro
if macro_power > micro_power {
    println!("Downward causation detected!");
}
```

## Configuration

```rust
use omega_consciousness::ConsciousnessConfig;

let config = ConsciousnessConfig {
    state_dim: 64,           // Dimension of state vectors
    hierarchy_levels: 5,      // FEP hierarchy depth
    workspace_capacity: 7,    // GWT workspace size
    phi_threshold: 0.1,       // Minimum Φ for consciousness
    precision_weight: 1.0,    // FEP precision weighting
};

let engine = ConsciousnessEngine::new(config);
```

## Consciousness State

```rust
use omega_consciousness::ConsciousnessState;

let state = engine.process(&input, &context)?;

// Consciousness state includes:
println!("Phi (Φ): {:.3}", state.phi);
println!("Free Energy: {:.3}", state.free_energy);
println!("Prediction Error: {:.3}", state.prediction_error);
println!("Emergence: {:.3}", state.emergence);
println!("Is Conscious: {}", state.is_conscious);
println!("Workspace Contents: {:?}", state.workspace_contents);
println!("Active Coalitions: {}", state.active_coalitions);
```

## Theoretical Foundation

### IIT Axioms

1. **Existence**: Consciousness exists (intrinsic, from system's perspective)
2. **Composition**: Composed of multiple phenomenal distinctions
3. **Information**: Each experience is specific (differentiated)
4. **Integration**: Cannot be reduced to independent parts
5. **Exclusion**: Definite borders, excludes supersets/subsets

### FEP Principles

1. Systems minimize variational free energy
2. Perception = inference about hidden states
3. Action = making predictions come true
4. Learning = optimizing generative model

### GWT Principles

1. Specialized unconscious processors
2. Global workspace for information sharing
3. Competition for conscious access
4. Broadcast creates widespread availability

## Integration with Omega

```
omega-brain (Unified Integration)
    └── omega-consciousness (This crate)
            ├── IIT module
            ├── FEP module
            ├── GWT module
            └── Emergence module

omega-consciousness is used for:
- Determining what content becomes conscious
- Computing Phi for system integration
- Predictive processing of sensory input
- Active inference for action selection
```

## Related Crates

- **[omega-brain](../omega-brain)** - Unified cognitive architecture
- **[omega-attention](../omega-attention)** - What enters consciousness
- **[omega-strange-loops](../omega-strange-loops)** - Self-referential consciousness
- **[omega-hippocampus](../omega-hippocampus)** - Memory for conscious replay

## References

- Tononi, G. (2004). "An information integration theory of consciousness"
- Oizumi, M., Albantakis, L., & Tononi, G. (2014). "From the Phenomenology to the Mechanisms of Consciousness: IIT 3.0"
- Friston, K. (2010). "The free-energy principle: a unified brain theory?"
- Baars, B. J. (1988). "A Cognitive Theory of Consciousness"
- Dehaene, S., & Changeux, J. P. (2011). "Experimental and Theoretical Approaches to Conscious Processing"

## License

Licensed under the MIT License. See [LICENSE](../../LICENSE) for details.
