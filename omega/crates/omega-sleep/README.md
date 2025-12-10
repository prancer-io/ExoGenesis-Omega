# omega-sleep

[![Crates.io](https://img.shields.io/crates/v/omega-sleep)](https://crates.io/crates/omega-sleep)
[![Documentation](https://docs.rs/omega-sleep/badge.svg)](https://docs.rs/omega-sleep)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Biologically-inspired sleep/wake cycle simulation with memory consolidation, circadian rhythms, and sleep architecture.

**Part of the [ExoGenesis-Omega](https://github.com/prancer-io/ExoGenesis-Omega) cognitive architecture.**

## Overview

`omega-sleep` implements the neuroscience of sleep for artificial cognitive systems. Sleep is essential for memory consolidation, synaptic homeostasis, and cognitive maintenance. This crate provides:

- **Sleep Stages**: N1, N2, N3 (SWS), and REM sleep
- **Slow Wave Sleep (SWS)**: Deep sleep with delta waves and memory consolidation
- **REM Sleep**: Rapid eye movement with dreaming and memory reorganization
- **Sleep Spindles**: Thalamocortical oscillations during N2
- **Circadian Rhythm**: 24-hour biological clock with melatonin/temperature
- **Sleep Pressure**: Homeostatic sleep drive (Process S)

## Features

- **Two-Process Model**: Circadian (Process C) + Homeostatic (Process S)
- **Memory Consolidation**: Declarative in SWS, procedural in REM
- **Sleep Architecture**: Realistic progression through sleep stages
- **Slow Oscillations**: Delta waves (0.5-4 Hz) during N3
- **Sleep Spindles**: Sigma activity (12-16 Hz) during N2
- **K-Complexes**: Large deflections during N2
- **Circadian Markers**: Melatonin, core body temperature

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
omega-sleep = "1.0.0"
```

## Quick Start

```rust
use omega_sleep::{SleepController, SleepConfig, SleepStage};

fn main() {
    // Create sleep controller
    let config = SleepConfig::default();
    let mut controller = SleepController::new(config);

    // Build sleep pressure during wake
    for _ in 0..16 * 60 {  // 16 hours awake
        controller.advance_time(1.0);  // 1 minute steps
    }

    println!("Sleep pressure: {:.3}", controller.sleep_pressure());
    println!("Should sleep: {}", controller.should_sleep());

    // Fall asleep
    controller.fall_asleep()?;

    // Progress through sleep cycles
    for _ in 0..8 * 60 {  // 8 hours of sleep
        controller.advance_time(1.0);

        let stage = controller.current_stage();
        println!("Current stage: {:?}", stage);

        // Consolidation strength varies by stage
        let strength = stage.consolidation_strength();
        if strength > 0.5 {
            println!("  Good time for consolidation!");
        }
    }

    // Wake up
    controller.wake_up()?;
}
```

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                        SLEEP SYSTEM                                  │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                    TWO-PROCESS MODEL                         │   │
│  │                                                              │   │
│  │   Process S (Homeostatic)    Process C (Circadian)          │   │
│  │   ┌──────────────────┐       ┌──────────────────────┐       │   │
│  │   │ Sleep pressure   │       │ 24-hour rhythm       │       │   │
│  │   │ builds during    │       │ • Melatonin          │       │   │
│  │   │ wake, decays     │       │ • Temperature        │       │   │
│  │   │ during sleep     │       │ • Alertness          │       │   │
│  │   └──────────────────┘       └──────────────────────┘       │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                              ↓                                       │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                    SLEEP STAGES                              │   │
│  │                                                              │   │
│  │   Wake → N1 → N2 → N3 (SWS) → N2 → REM → N2 → N3 → ...     │   │
│  │                                                              │   │
│  │   N1: Transition (5 min)     N2: Light sleep (20 min)       │   │
│  │   N3: Deep/SWS (30 min)      REM: Dreaming (20 min)         │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                              ↓                                       │
│  ┌──────────────────┐  ┌──────────────────┐  ┌─────────────────┐   │
│  │   SLOW WAVES     │  │  SLEEP SPINDLES  │  │   REM SLEEP     │   │
│  │   (N3/SWS)       │  │     (N2)         │  │                 │   │
│  │                  │  │                  │  │  • Theta waves  │   │
│  │  • Delta 0.5-4Hz │  │  • Sigma 12-16Hz │  │  • Eye movement │   │
│  │  • Up/down states│  │  • K-complexes   │  │  • Muscle atonia│   │
│  │  • Consolidation │  │  • Memory xfer   │  │  • Dreams       │   │
│  └──────────────────┘  └──────────────────┘  └─────────────────┘   │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

## Sleep Stages

```rust
use omega_sleep::SleepStage;

let stage = SleepStage::N3;  // Deep sleep

// Stage properties
println!("Duration: {} min", stage.typical_duration());
println!("Consolidation: {:.1}%", stage.consolidation_strength() * 100.0);
println!("Is sleeping: {}", stage.is_sleeping());

// All stages
for stage in [SleepStage::Wake, SleepStage::N1, SleepStage::N2,
              SleepStage::N3, SleepStage::REM] {
    println!("{:?}: {:.0} min, {:.0}% consolidation",
        stage,
        stage.typical_duration(),
        stage.consolidation_strength() * 100.0);
}
// Output:
// Wake: 0 min, 0% consolidation
// N1: 5 min, 10% consolidation
// N2: 20 min, 30% consolidation
// N3: 30 min, 100% consolidation
// REM: 20 min, 70% consolidation
```

## Slow Wave Sleep (SWS)

Deep sleep with memory consolidation:

```rust
use omega_sleep::{SlowWaveSleep, SlowWave};

let mut sws = SlowWaveSleep::new();

// Progress through slow waves
for step in 0..1000 {
    sws.update(0.01, 0.8);  // dt=10ms, target_power=0.8

    // Check if in up-state (good for replay)
    if sws.is_replay_window() {
        trigger_memory_replay();
    }

    // Monitor delta power
    if step % 100 == 0 {
        println!("Delta power: {:.3}", sws.delta_power());
        println!("In up-state: {}", sws.in_up_state());
    }
}
```

## Sleep Spindles

Thalamocortical oscillations during N2:

```rust
use omega_sleep::{SpindleGenerator, SleepSpindle, KComplex};

let mut gen = SpindleGenerator::new();

// Generate spindles during N2 sleep
for minute in 0..20 {
    if let Some(spindle) = gen.step(1.0) {  // 1 minute step
        println!("Spindle at minute {}", minute);
        println!("  Frequency: {:.1} Hz", spindle.frequency);
        println!("  Duration: {:.0} ms", spindle.duration_ms);
        println!("  Amplitude: {:.3}", spindle.amplitude);
    }
}

// Sigma power indicates spindle activity
println!("Sigma power: {:.3}", gen.sigma_power());

// K-complexes (large deflections)
let spontaneous_kc = KComplex::generate(false);
let evoked_kc = KComplex::generate(true);

println!("K-complex amplitude: {:.3}", spontaneous_kc.amplitude);
println!("Evoked: {}", evoked_kc.evoked);
```

## Circadian Rhythm

24-hour biological clock:

```rust
use omega_sleep::{CircadianRhythm, TimeOfDay};

let mut rhythm = CircadianRhythm::new();

// Set time of day
rhythm.set_time(TimeOfDay::new(8, 0));  // 8:00 AM

println!("At 8 AM:");
println!("  Alertness: {:.3}", rhythm.current_alertness());
println!("  Sleep drive: {:.3}", rhythm.current_sleep_drive());
println!("  Melatonin: {:.3}", rhythm.melatonin());
println!("  Temperature deviation: {:.3}°", rhythm.temperature_deviation());

// At 3 AM
rhythm.set_time(TimeOfDay::new(3, 0));

println!("\nAt 3 AM:");
println!("  Alertness: {:.3}", rhythm.current_alertness());
println!("  Sleep drive: {:.3}", rhythm.current_sleep_drive());
println!("  Melatonin: {:.3}", rhythm.melatonin());

// Light exposure suppresses melatonin
rhythm.set_light(2000.0);  // Bright light (lux)
println!("\nWith bright light:");
println!("  Melatonin: {:.3}", rhythm.melatonin());  // Suppressed
```

## Memory Consolidation

```rust
use omega_sleep::{MemoryConsolidator, ConsolidationEvent};

let mut consolidator = MemoryConsolidator::new();

// Add memories from wake period
consolidator.add_memory(memory1, 0.8);  // High importance
consolidator.add_memory(memory2, 0.3);  // Low importance

// Consolidation during N3 (SWS)
let sws_events = consolidator.consolidate_sws(&sws_state)?;
for event in sws_events {
    println!("Consolidated: {} (replay count: {})",
        event.memory_id, event.replay_count);
}

// Consolidation during REM
let rem_events = consolidator.consolidate_rem(&rem_state)?;
for event in rem_events {
    println!("Reorganized: {} (integration: {:.3})",
        event.memory_id, event.integration_score);
}
```

## REM Sleep

Rapid eye movement sleep with dreams:

```rust
use omega_sleep::{REMSleep, DreamContent};

let mut rem = REMSleep::new();

// Enter REM
rem.enter()?;

// Progress through REM episode
for step in 0..1000 {
    rem.update(0.01, 0.7);  // dt=10ms, rem_density=0.7

    // Dream content emerges
    if rem.is_dreaming() {
        let dream = rem.current_dream();
        println!("Dream emotional valence: {:.3}", dream.valence);
        println!("Dream bizarreness: {:.3}", dream.bizarreness);
    }

    // Eye movements
    println!("Eye movement density: {:.3}", rem.eye_movement_density());
}

// REM is characterized by muscle atonia
println!("Muscle tone: {:.3}", rem.muscle_tone());  // Very low
```

## Configuration

```rust
use omega_sleep::SleepConfig;

let config = SleepConfig {
    cycle_duration_hours: 1.5,    // 90-minute cycles
    cycles_per_night: 5,          // 5 cycles = 7.5 hours
    wake_threshold: 0.8,          // Pressure to wake
    pressure_decay_rate: 0.02,    // Pressure drops during sleep
    pressure_build_rate: 0.01,    // Pressure builds during wake
    rem_proportion_increase: 0.1, // REM increases later in night
    sws_proportion_decrease: 0.1, // SWS decreases later in night
    ..Default::default()
};

let controller = SleepController::new(config);
```

## Use Cases

### 1. Cognitive System Maintenance

```rust
// After extended operation, system needs sleep
if controller.sleep_pressure() > 0.8 {
    controller.fall_asleep()?;

    while controller.is_sleeping() {
        let stage = controller.current_stage();

        // Consolidate during SWS
        if stage == SleepStage::N3 {
            hippocampus.replay_memories()?;
        }

        // Prune weak connections during REM
        if stage == SleepStage::REM {
            neural_network.synaptic_homeostasis()?;
        }

        controller.advance_time(1.0);
    }
}
```

### 2. Memory Prioritization

```rust
// Important memories get replayed more during SWS
let memories_by_importance: Vec<_> = memories
    .iter()
    .sorted_by(|a, b| b.importance.partial_cmp(&a.importance).unwrap())
    .collect();

during_sws(|| {
    for memory in memories_by_importance.iter().take(10) {
        replay(memory);  // Top 10 memories replayed
    }
});
```

### 3. Circadian-Aware Scheduling

```rust
// Schedule demanding tasks when alertness is high
let current_time = get_time();
rhythm.set_time(current_time);

if rhythm.current_alertness() > 0.7 {
    perform_complex_reasoning();
} else {
    perform_routine_maintenance();
}
```

## Integration with Omega

```
omega-brain (Unified Integration)
    └── omega-sleep (This crate)
            ├── Sleep stages
            ├── Slow wave sleep
            ├── REM sleep
            ├── Spindles
            └── Circadian rhythm

Interacts with:
├── omega-hippocampus - Memory replay
├── omega-snn - Synaptic homeostasis
└── omega-consciousness - Reduced awareness
```

## Related Crates

- **[omega-brain](../omega-brain)** - Unified cognitive architecture
- **[omega-hippocampus](../omega-hippocampus)** - Memory for consolidation
- **[omega-snn](../omega-snn)** - Neural substrate for homeostasis
- **[omega-consciousness](../omega-consciousness)** - Awareness reduction

## References

- Borbély, A. A. (1982). "A two process model of sleep regulation"
- Diekelmann, S., & Born, J. (2010). "The memory function of sleep"
- Tononi, G., & Cirelli, C. (2006). "Sleep function and synaptic homeostasis"
- Steriade, M. (2006). "Grouping of brain rhythms in corticothalamic systems"
- Stickgold, R. (2005). "Sleep-dependent memory consolidation"

## License

Licensed under the MIT License. See [LICENSE](../../LICENSE) for details.
