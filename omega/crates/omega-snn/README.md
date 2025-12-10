# omega-snn

[![Crates.io](https://img.shields.io/crates/v/omega-snn)](https://crates.io/crates/omega-snn)
[![Documentation](https://docs.rs/omega-snn/badge.svg)](https://docs.rs/omega-snn)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Biologically-inspired spiking neural network with LIF neurons, STDP learning, and neuromodulation.

## Overview

`omega-snn` provides the neural substrate for the Omega cognitive architecture. Unlike rate-coded neural networks, spiking neural networks (SNNs) use discrete spike events for communication, matching biological neural computation.

Key components:
- **Leaky Integrate-and-Fire (LIF) Neurons**: Biologically plausible neuron dynamics
- **Spike-Timing Dependent Plasticity (STDP)**: Hebbian learning based on spike timing
- **Neuromodulation**: Dopamine, norepinephrine, serotonin, acetylcholine
- **Short-Term Plasticity**: Facilitation and depression
- **Population Coding**: Sparse distributed representations

## Features

- **LIF Neurons**: Membrane potential, refractory period, adaptive threshold
- **Adaptive LIF**: Spike-frequency adaptation for realistic dynamics
- **STDP Learning**: Asymmetric time-window for causal learning
- **4 Neuromodulators**: Global modulation of learning and behavior
- **Synaptic Dynamics**: Facilitation, depression, and saturation
- **Spike Trains**: Temporal coding and rate coding analysis
- **Neural Populations**: Excitatory, inhibitory, sensory, motor

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
omega-snn = "1.0.0"
```

## Quick Start

```rust
use omega_snn::{SNNEngine, NetworkConfig, NeuronType};

fn main() {
    // Create SNN with default configuration
    let config = NetworkConfig::default();
    let mut snn = SNNEngine::new(config);

    // Add neurons
    let sensory = snn.add_population(100, NeuronType::Sensory)?;
    let hidden = snn.add_population(500, NeuronType::Excitatory)?;
    let output = snn.add_population(50, NeuronType::Motor)?;

    // Connect populations
    snn.connect(&sensory, &hidden, 0.3, 0.5)?;  // 30% connectivity, weight 0.5
    snn.connect(&hidden, &output, 0.2, 0.3)?;

    // Input as spike train
    let input_spikes = generate_poisson_spikes(100.0, 100);  // 100 Hz, 100ms

    // Simulate
    for t in 0..1000 {
        // Inject input at sensory layer
        snn.inject_spikes(&sensory, &input_spikes[t]);

        // Step simulation (1ms)
        snn.step(1.0)?;

        // Get output spikes
        let output_spikes = snn.get_spikes(&output);
        if !output_spikes.is_empty() {
            println!("Output spikes at t={}: {:?}", t, output_spikes);
        }
    }
}
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    SPIKING NEURAL NETWORK                    │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────────────────────────────────────────────┐  │
│  │              NEUROMODULATOR SYSTEM                    │  │
│  │  Dopamine │ Norepinephrine │ Serotonin │ Acetylcholine│  │
│  └──────────────────────────────────────────────────────┘  │
│                          ↓                                   │
│  ┌──────────────────────────────────────────────────────┐  │
│  │                  NEURAL POPULATIONS                   │  │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐ │  │
│  │  │Excitatory│  │Inhibitory│  │ Sensory │  │  Motor  │ │  │
│  │  │ Neurons │  │ Neurons │  │ Neurons │  │ Neurons │ │  │
│  │  └─────────┘  └─────────┘  └─────────┘  └─────────┘ │  │
│  └──────────────────────────────────────────────────────┘  │
│                          ↓                                   │
│  ┌──────────────────────────────────────────────────────┐  │
│  │                 SYNAPTIC CONNECTIONS                  │  │
│  │        STDP │ Short-Term Plasticity │ Weights        │  │
│  └──────────────────────────────────────────────────────┘  │
│                          ↓                                   │
│  ┌──────────────────────────────────────────────────────┐  │
│  │                   SPIKE TRAINS                        │  │
│  │           Temporal coding │ Rate coding               │  │
│  └──────────────────────────────────────────────────────┘  │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## LIF Neurons

Leaky Integrate-and-Fire is the workhorse neuron model:

```rust
use omega_snn::{LIFNeuron, LIFParams, NeuronState};

// Create neuron with custom parameters
let params = LIFParams {
    tau_m: 20.0,        // Membrane time constant (ms)
    v_rest: -70.0,      // Resting potential (mV)
    v_thresh: -55.0,    // Spike threshold (mV)
    v_reset: -75.0,     // Reset after spike (mV)
    tau_ref: 2.0,       // Refractory period (ms)
    ..Default::default()
};

let mut neuron = LIFNeuron::new(params);

// Integrate input current
for t in 0..100 {
    let current = if t < 50 { 10.0 } else { 0.0 };

    if neuron.integrate(current, 1.0) {  // dt = 1ms
        println!("Spike at t={}", t);
    }

    println!("V = {:.2} mV", neuron.voltage());
}
```

### Adaptive LIF

Includes spike-frequency adaptation:

```rust
use omega_snn::AdaptiveLIFNeuron;

let mut neuron = AdaptiveLIFNeuron::new_default();

// With constant input, firing rate decreases due to adaptation
for t in 0..500 {
    if neuron.integrate(15.0, 1.0) {
        println!("Spike at t={}, adaptation={:.2}",
            t, neuron.adaptation());
    }
}
```

## STDP Learning

Spike-Timing Dependent Plasticity for causal learning:

```rust
use omega_snn::{STDPRule, STDPParams, Synapse};

let params = STDPParams {
    a_plus: 0.01,    // LTP amplitude
    a_minus: 0.012,  // LTD amplitude
    tau_plus: 20.0,  // LTP time constant (ms)
    tau_minus: 20.0, // LTD time constant (ms)
    w_max: 1.0,      // Maximum weight
    w_min: 0.0,      // Minimum weight
};

let stdp = STDPRule::new(params);
let mut synapse = Synapse::new(0.5);  // Initial weight 0.5

// Pre-before-post: potentiation
let pre_time = 100.0;
let post_time = 110.0;
let dw = stdp.compute_update(pre_time, post_time);
synapse.update_weight(dw);
println!("Weight after pre→post: {:.3}", synapse.weight());

// Post-before-pre: depression
let post_time = 200.0;
let pre_time = 210.0;
let dw = stdp.compute_update(pre_time, post_time);
synapse.update_weight(dw);
println!("Weight after post→pre: {:.3}", synapse.weight());
```

## Neuromodulation

Global modulation of network dynamics:

```rust
use omega_snn::{NeuromodulatorSystem, NeuromodulatorType};

let mut neuromod = NeuromodulatorSystem::new();

// Increase dopamine (reward/motivation)
neuromod.release(NeuromodulatorType::Dopamine, 0.8);

// Get current levels
let da = neuromod.level(NeuromodulatorType::Dopamine);
let ne = neuromod.level(NeuromodulatorType::Norepinephrine);
let ser = neuromod.level(NeuromodulatorType::Serotonin);
let ach = neuromod.level(NeuromodulatorType::Acetylcholine);

println!("Dopamine: {:.3} (reward, motivation)", da);
println!("Norepinephrine: {:.3} (arousal, attention)", ne);
println!("Serotonin: {:.3} (mood, inhibition)", ser);
println!("Acetylcholine: {:.3} (learning, memory)", ach);

// Neuromodulators decay over time
neuromod.decay(0.01);  // 1% decay per step
```

### Modulation Effects

```rust
// Dopamine modulates STDP learning rate
let learning_rate = base_rate * (1.0 + 0.5 * da);

// Norepinephrine modulates gain
let gain = base_gain * (1.0 + 0.3 * ne);

// Serotonin modulates inhibition
let inhibition = base_inhibition * (1.0 + 0.4 * ser);

// Acetylcholine modulates attention
let attention = base_attention * (1.0 + 0.6 * ach);
```

## Spike Trains

Temporal representations of neural activity:

```rust
use omega_snn::{SpikeTrain, SpikeAnalysis};

// Create spike train
let mut train = SpikeTrain::new();
train.add_spike(10.0);
train.add_spike(25.0);
train.add_spike(42.0);
train.add_spike(60.0);

// Analysis
let analysis = SpikeAnalysis::new(&train);

println!("Spike count: {}", analysis.count());
println!("Firing rate: {:.1} Hz", analysis.rate(100.0));  // 100ms window
println!("Mean ISI: {:.1} ms", analysis.mean_isi());
println!("CV of ISI: {:.3}", analysis.cv_isi());  // Regularity measure

// Inter-spike intervals
for isi in analysis.isis() {
    println!("ISI: {:.1} ms", isi);
}
```

## Neural Populations

Organize neurons into functional groups:

```rust
use omega_snn::{NeuralPopulation, PopulationActivity, NeuronType};

let mut excitatory = NeuralPopulation::new(800, NeuronType::Excitatory);
let mut inhibitory = NeuralPopulation::new(200, NeuronType::Inhibitory);

// Connect (E-I balance)
excitatory.connect_to(&inhibitory, 0.2, 0.5);  // E → I
inhibitory.connect_to(&excitatory, 0.4, -0.8); // I → E (inhibitory)

// Step simulation
excitatory.step(input, 1.0);
inhibitory.step(&[], 1.0);  // Driven by excitatory

// Population activity
let activity = excitatory.activity();
println!("Active neurons: {}/{}", activity.active_count, 800);
println!("Population rate: {:.1} Hz", activity.rate);
println!("Sparsity: {:.3}", activity.sparsity());
```

## Sparse Coding

Efficient distributed representations:

```rust
use omega_snn::SparseCode;

// Create sparse code (10% active)
let code = SparseCode::new(1000, 0.1);

// Encode input
let input = vec![0.5; 100];
let sparse = code.encode(&input);

println!("Active units: {}", sparse.iter().filter(|&&x| x > 0.0).count());
println!("Sparsity: {:.3}", code.sparsity(&sparse));
```

## Configuration

```rust
use omega_snn::NetworkConfig;

let config = NetworkConfig {
    dt: 1.0,                    // Time step (ms)
    neuron_type: Default::default(),
    tau_m: 20.0,                // Membrane time constant
    v_thresh: -55.0,            // Spike threshold
    stdp_enabled: true,         // Enable STDP learning
    stp_enabled: true,          // Short-term plasticity
    neuromod_enabled: true,     // Neuromodulation
    max_delay: 20.0,            // Maximum axonal delay (ms)
    ..Default::default()
};

let snn = SNNEngine::new(config);
```

## Use Cases

### 1. Sensory Processing

```rust
// Visual input as spike patterns
let visual_input = encode_image_as_spikes(&image)?;

// Process through visual hierarchy
let v1_spikes = v1_layer.process(&visual_input)?;
let v2_spikes = v2_layer.process(&v1_spikes)?;
let it_spikes = it_layer.process(&v2_spikes)?;  // Object recognition
```

### 2. Reinforcement Learning with Dopamine

```rust
// Reward prediction error modulates learning
if reward > expected_reward {
    neuromod.release(NeuromodulatorType::Dopamine, 0.8);
} else if reward < expected_reward {
    neuromod.release(NeuromodulatorType::Dopamine, -0.3);
}

// STDP learning is enhanced by dopamine
for synapse in active_synapses {
    let dw = stdp.compute_update(pre_time, post_time);
    let modulated_dw = dw * (1.0 + dopamine_level);
    synapse.update_weight(modulated_dw);
}
```

### 3. Working Memory

```rust
// Persistent activity through recurrent excitation
let mut wm_circuit = NeuralPopulation::new(100, NeuronType::Excitatory);
wm_circuit.connect_to(&wm_circuit, 0.3, 0.1);  // Recurrent connections

// Activate with input
wm_circuit.inject_current(input, 100.0);

// Activity persists after input removed
for t in 0..1000 {
    wm_circuit.step(&[], 1.0);
    if wm_circuit.activity().rate > 10.0 {
        println!("Working memory active at t={}", t);
    }
}
```

## Integration with Omega

```
omega-brain (Unified Integration)
    └── omega-snn (This crate)
            ├── LIF neurons
            ├── STDP learning
            ├── Neuromodulation
            ├── Populations
            └── Spike trains

Provides substrate for:
├── omega-attention - Neural attention
├── omega-consciousness - Neural correlates
└── omega-hippocampus - Memory circuits
```

## Related Crates

- **[omega-brain](../omega-brain)** - Unified cognitive architecture
- **[omega-attention](../omega-attention)** - Attention mechanisms
- **[omega-hippocampus](../omega-hippocampus)** - Memory circuits
- **[omega-sleep](../omega-sleep)** - Synaptic homeostasis

## References

- Gerstner, W., & Kistler, W. M. (2002). "Spiking Neuron Models"
- Bi, G., & Poo, M. (1998). "Synaptic Modifications in Cultured Hippocampal Neurons"
- Dayan, P., & Abbott, L. F. (2001). "Theoretical Neuroscience"
- Izhikevich, E. M. (2007). "Dynamical Systems in Neuroscience"
- Schultz, W. (2007). "Behavioral Dopamine Signals"

## License

Licensed under the MIT License. See [LICENSE](../../LICENSE) for details.
