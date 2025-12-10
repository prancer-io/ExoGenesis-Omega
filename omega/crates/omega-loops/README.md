# omega-loops

[![Crates.io](https://img.shields.io/crates/v/omega-loops)](https://crates.io/crates/omega-loops)
[![Documentation](https://docs.rs/omega-loops/badge.svg)](https://docs.rs/omega-loops)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

7 temporal cognitive loops from Reflexive (1ms) to Transcendent (10y) for multi-scale intelligence processing.

**Part of the [ExoGenesis-Omega](https://github.com/prancer-io/ExoGenesis-Omega) cognitive architecture.**

## Overview

`omega-loops` implements a hierarchical system of 7 nested temporal loops that enable intelligence systems to operate coherently across vastly different timescales. From millisecond reflexes to decade-long transformations, each loop processes information at its natural temporal scale while coordinating with faster and slower loops.

Inspired by biological cognitive architectures (from reflexes to strategic planning) and extended to cosmic scales, this system enables AI to be simultaneously reactive, adaptive, deliberative, and transformative.

## Features

- **7 Temporal Loops**: Reflexive (100ms), Reactive (5s), Adaptive (30min), Deliberative (24h), Evolutionary (7d), Transformative (1y), Transcendent (10y)
- **Nested Architecture**: Faster loops provide input to slower loops
- **Cycle Management**: Track cycle execution, metrics, and outputs
- **Message Bus**: Inter-loop communication and coordination
- **Loop Executor**: Async execution with processors for each loop type
- **Metrics Tracking**: Cycle count, success rate, average duration
- **Graceful Shutdown**: Proper cleanup of all running loops

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
omega-loops = "0.1.0"
```

## Quick Start

```rust
use omega_loops::{LoopEngine, LoopType, CycleInput};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create and initialize the loop engine
    let mut engine = LoopEngine::new();
    engine.initialize().await?;

    // Execute a cycle in the adaptive loop
    let input = CycleInput {
        data: HashMap::new(),
        context: "learning_session".to_string(),
        objectives: vec!["improve_accuracy".to_string()],
    };

    let output = engine.execute_cycle(LoopType::Adaptive, input).await?;
    println!("Cycle completed: {}", output.cycle_id);

    // Get statistics for all loops
    let stats = engine.get_stats().await;
    for (loop_type, stat) in stats {
        println!("{:?}: {} cycles, {:.1}% success",
            loop_type,
            stat.cycles_completed,
            stat.success_rate * 100.0
        );
    }

    // Shutdown gracefully
    engine.shutdown().await?;
    Ok(())
}
```

## Core Concepts

### The 7 Temporal Loops

Each loop operates at a specific timescale:

1. **Reflexive** (100ms)
   - Immediate sensory-motor coupling
   - Reflex actions, emergency responses
   - Examples: Collision avoidance, error handling

2. **Reactive** (5 seconds)
   - Fast decision-making
   - Pattern recognition, quick responses
   - Examples: User interaction, real-time adaptation

3. **Adaptive** (30 minutes)
   - Learning from recent experience
   - Short-term optimization
   - Examples: Session learning, tactic adjustment

4. **Deliberative** (24 hours)
   - Strategic planning and reasoning
   - Goal-directed behavior
   - Examples: Daily planning, project management

5. **Evolutionary** (7 days)
   - Systematic improvement and growth
   - Habit formation, skill development
   - Examples: Weekly retrospectives, skill practice

6. **Transformative** (1 year)
   - Fundamental architectural changes
   - Paradigm shifts in capabilities
   - Examples: Annual reviews, major upgrades

7. **Transcendent** (10 years)
   - Long-term vision and legacy
   - Civilizational-scale patterns
   - Examples: Decade planning, generational knowledge

### Loop Coordination

Loops interact through:

- **Message Bus**: Asynchronous communication between loops
- **Nested Execution**: Faster loops execute within slower loop cycles
- **State Sharing**: Shared context and memory across loops
- **Metrics Propagation**: Performance metrics flow between loops

### Cycle Execution

Each cycle follows a standard flow:

1. **Input**: Receive context, data, and objectives
2. **Process**: Execute loop-specific logic
3. **Output**: Return results and state changes
4. **Metrics**: Update success rate and timing

## Use Cases

### 1. Real-Time Agent with Multi-Scale Planning

```rust
use omega_loops::{LoopEngine, LoopType, CycleInput};
use std::collections::HashMap;

let mut engine = LoopEngine::new();
engine.initialize().await?;

// Reflexive loop handles immediate responses
tokio::spawn(async move {
    loop {
        let input = CycleInput {
            data: get_sensor_data(),
            context: "real_time_control".to_string(),
            objectives: vec!["maintain_stability".to_string()],
        };
        engine.execute_cycle(LoopType::Reflexive, input).await.ok();
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
});

// Adaptive loop runs every 30 minutes for learning
tokio::spawn(async move {
    loop {
        let input = CycleInput {
            data: get_recent_performance(),
            context: "learning_update".to_string(),
            objectives: vec!["improve_model".to_string()],
        };
        engine.execute_cycle(LoopType::Adaptive, input).await.ok();
        tokio::time::sleep(Duration::from_secs(1800)).await;
    }
});

// Deliberative loop runs daily for strategic planning
tokio::spawn(async move {
    loop {
        let input = CycleInput {
            data: get_daily_summary(),
            context: "daily_planning".to_string(),
            objectives: vec!["optimize_goals".to_string()],
        };
        engine.execute_cycle(LoopType::Deliberative, input).await.ok();
        tokio::time::sleep(Duration::from_secs(86400)).await;
    }
});
```

### 2. Learning System with Hierarchical Consolidation

```rust
use omega_loops::{LoopEngine, LoopType};

let mut engine = LoopEngine::new();
engine.initialize().await?;

// Reactive loop: Immediate pattern recognition
let reactive_input = CycleInput {
    data: get_current_patterns(),
    context: "pattern_detection".to_string(),
    objectives: vec!["recognize_patterns".to_string()],
};
engine.execute_cycle(LoopType::Reactive, reactive_input).await?;

// Adaptive loop: Consolidate patterns into rules
let adaptive_input = CycleInput {
    data: get_detected_patterns(),
    context: "rule_learning".to_string(),
    objectives: vec!["form_rules".to_string()],
};
engine.execute_cycle(LoopType::Adaptive, adaptive_input).await?;

// Evolutionary loop: Refine and generalize rules
let evolutionary_input = CycleInput {
    data: get_learned_rules(),
    context: "rule_refinement".to_string(),
    objectives: vec!["generalize_rules".to_string()],
};
engine.execute_cycle(LoopType::Evolutionary, evolutionary_input).await?;
```

### 3. Self-Improving Architecture

```rust
use omega_loops::{LoopEngine, LoopType};

let mut engine = LoopEngine::new();
engine.initialize().await?;

// Evolutionary loop: Weekly performance review
let weekly_input = CycleInput {
    data: get_weekly_metrics(),
    context: "weekly_review".to_string(),
    objectives: vec!["identify_weaknesses".to_string()],
};
engine.execute_cycle(LoopType::Evolutionary, weekly_input).await?;

// Transformative loop: Annual architecture evolution
let yearly_input = CycleInput {
    data: get_annual_performance(),
    context: "architecture_evolution".to_string(),
    objectives: vec!["evolve_architecture".to_string()],
};
engine.execute_cycle(LoopType::Transformative, yearly_input).await?;
```

### 4. Multi-Agent Coordination

```rust
use omega_loops::{LoopEngine, LoopType};

// Each agent has its own loop engine
let mut agent1_engine = LoopEngine::new();
let mut agent2_engine = LoopEngine::new();

agent1_engine.initialize().await?;
agent2_engine.initialize().await?;

// Reactive coordination between agents
let coord_input = CycleInput {
    data: get_agent_states(),
    context: "agent_coordination".to_string(),
    objectives: vec!["synchronize_actions".to_string()],
};

agent1_engine.execute_cycle(LoopType::Reactive, coord_input.clone()).await?;
agent2_engine.execute_cycle(LoopType::Reactive, coord_input).await?;

// Deliberative planning for collaborative tasks
let planning_input = CycleInput {
    data: get_task_requirements(),
    context: "collaborative_planning".to_string(),
    objectives: vec!["divide_work".to_string()],
};

agent1_engine.execute_cycle(LoopType::Deliberative, planning_input.clone()).await?;
agent2_engine.execute_cycle(LoopType::Deliberative, planning_input).await?;
```

### 5. Monitoring and Diagnostics

```rust
use omega_loops::{LoopEngine, LoopType};

let mut engine = LoopEngine::new();
engine.initialize().await?;

// Run some cycles...

// Check which loops are active
println!("Engine running: {}", engine.is_running().await);

// Get detailed statistics
let stats = engine.get_stats().await;

for (loop_type, stat) in stats {
    println!("\n{:?} Loop:", loop_type);
    println!("  Cycles: {}", stat.cycles_completed);
    println!("  Success: {:.1}%", stat.success_rate * 100.0);
    println!("  Avg time: {:?}", stat.average_cycle_time);
}

// Access coordinator for direct loop management
let coordinator = engine.coordinator();
let coord = coordinator.read().await;

for loop_type in LoopType::all_loops() {
    if let Ok(Some(loop_info)) = coord.get_loop_by_type(loop_type).await {
        println!("{:?} status: {:?}", loop_type, loop_info.status);
    }
}
```

## Examples

### Custom Loop Processor

```rust
use omega_loops::{LoopExecutor, LoopType, CycleInput, CycleOutput};

// Create custom processor for specific loop type
struct CustomAdaptiveProcessor;

impl CustomAdaptiveProcessor {
    async fn process(&self, input: CycleInput) -> Result<CycleOutput, Box<dyn std::error::Error>> {
        // Custom processing logic
        let mut output_data = HashMap::new();
        output_data.insert(
            "learned".to_string(),
            serde_json::json!({"patterns": ["pattern1", "pattern2"]})
        );

        Ok(CycleOutput {
            cycle_id: uuid::Uuid::new_v4().to_string(),
            success: true,
            data: output_data,
            timestamp: chrono::Utc::now(),
        })
    }
}
```

### Loop Engine with Custom Configuration

```rust
use omega_loops::{LoopEngine, LoopCoordinator};

let mut engine = LoopEngine::new();

// Access coordinator to customize loop behavior
let coordinator = engine.coordinator();
let mut coord = coordinator.write().await;

// Create loops with custom configurations
for loop_type in LoopType::all_loops() {
    coord.create_loop(
        loop_type,
        format!("Custom {:?}", loop_type),
        format!("Customized {} loop", loop_type.description()),
    ).await?;
}
```

## Architecture

The loop system is structured hierarchically:

```
┌──────────────────────────────────────────┐
│           LoopEngine                      │
│  - Initialization & shutdown              │
│  - Statistics aggregation                 │
└────────────┬─────────────────────────────┘
             │
             ▼
┌──────────────────────────────────────────┐
│        LoopCoordinator                    │
│  - Loop creation & management             │
│  - State tracking                         │
│  - Message bus coordination               │
└────────────┬─────────────────────────────┘
             │
             ▼
┌──────────────────────────────────────────┐
│         LoopExecutors (7)                 │
│  - Reflexive executor                     │
│  - Reactive executor                      │
│  - Adaptive executor                      │
│  - Deliberative executor                  │
│  - Evolutionary executor                  │
│  - Transformative executor                │
│  - Transcendent executor                  │
└──────────────────────────────────────────┘
```

## Performance

Loop system performance characteristics:

- **Reflexive**: <1ms execution overhead
- **Reactive**: <10ms execution overhead
- **Adaptive**: <100ms execution overhead
- **Higher Loops**: Dominated by processing logic

Memory usage: ~1KB per loop + cycle state

## Related Crates

- **[omega-core](../omega-core)** - Core loop types and traits
- **[omega-memory](../omega-memory)** - Memory consolidation across loops
- **[omega-agentdb](../omega-agentdb)** - Skill and reflexion storage
- **[omega-meta-sona](../omega-meta-sona)** - Architecture evolution in transformative loop
- **[omega-runtime](../omega-runtime)** - Production orchestration
- **[omega-persistence](../omega-persistence)** - Loop state persistence
- **[omega-brain](../omega-brain)** - Unified cognitive processing
- **[omega-sleep](../omega-sleep)** - Sleep/wake cycle integration

## License

Licensed under the MIT License. See [LICENSE](../../LICENSE) for details.
