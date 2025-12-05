# omega-loops - 7 Temporal Cognitive Loops

## Overview

`omega-loops` implements the temporal loop system for ExoGenesis Omega, providing 7 nested feedback cycles operating at different timescales from milliseconds (Reflexive) to decades (Transcendent). These loops enable multi-scale learning, adaptation, and evolution.

**Key Features:**
- **7 Temporal Scales**: 100ms to 10 years cycle durations
- **Loop Coordination**: Feed-forward, feedback, resonance, and inhibition
- **Cycle Management**: Input/output tracking, metrics, and history
- **Emergent Patterns**: Detection of cross-loop synergies
- **Async Execution**: Tokio-based concurrent loop processing

**Architecture:**
- **LoopEngine**: Coordinates all 7 loops
- **LoopCoordinator**: Manages loop lifecycle and state
- **LoopExecutor**: Executes individual loop cycles
- **Processors**: Loop-specific logic for each temporal scale

**Version:** 0.1.0
**Crate:** `omega-loops`
**Location:** `omega/crates/omega-loops`

## Installation

```toml
[dependencies]
omega-loops = "0.1.0"
omega-core = "0.1.0"  # For LoopType, CycleInput/Output
tokio = { version = "1", features = ["full"] }
```

## Core Concepts

### The 7 Temporal Loops

| Loop | Name | Cycle Duration | Purpose | Example Use |
|------|------|----------------|---------|-------------|
| L1 | Reflexive | 100ms | Sensory-motor feedback | Keyboard input → response |
| L2 | Reactive | 5 seconds | Quick decisions | Error detection → fix |
| L3 | Adaptive | 30 minutes | Learning from experience | Pattern recognition |
| L4 | Deliberative | 24 hours | Strategic planning | Daily reflection |
| L5 | Evolutionary | 7 days | Systematic improvement | Weekly optimization |
| L6 | Transformative | 1 year | Capability changes | Architecture evolution |
| L7 | Transcendent | 10 years | Paradigm shifts | Fundamental breakthroughs |

### Loop Hierarchy

Loops are nested and interact:

```
┌─────────────────────────────────────────────────┐
│ L7: Transcendent (10 years)                    │
│ ┌─────────────────────────────────────────────┐ │
│ │ L6: Transformative (1 year)                 │ │
│ │ ┌─────────────────────────────────────────┐ │ │
│ │ │ L5: Evolutionary (7 days)               │ │ │
│ │ │ ┌─────────────────────────────────────┐ │ │ │
│ │ │ │ L4: Deliberative (24 hours)         │ │ │ │
│ │ │ │ ┌─────────────────────────────────┐ │ │ │ │
│ │ │ │ │ L3: Adaptive (30 min)           │ │ │ │ │
│ │ │ │ │ ┌─────────────────────────────┐ │ │ │ │ │
│ │ │ │ │ │ L2: Reactive (5 sec)        │ │ │ │ │ │
│ │ │ │ │ │ ┌─────────────────────────┐ │ │ │ │ │ │
│ │ │ │ │ │ │ L1: Reflexive (100ms)   │ │ │ │ │ │ │
│ │ │ │ │ │ └─────────────────────────┘ │ │ │ │ │ │
│ │ │ │ │ └─────────────────────────────┘ │ │ │ │ │
│ │ │ │ └─────────────────────────────────┘ │ │ │ │
│ │ │ └─────────────────────────────────────┘ │ │ │
│ │ └─────────────────────────────────────────┘ │ │
│ └─────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────┘
```

### Loop Connections

Loops interact through four connection types:

```rust
pub enum ConnectionType {
    FeedForward,  // Fast loop → Slow loop (insight propagation)
    FeedBack,     // Slow loop → Fast loop (guidance)
    Resonance,    // Mutual reinforcement
    Inhibition,   // One suppresses another
}
```

**Examples:**
- **FeedForward**: Reflexive detects pattern → Adaptive learns it
- **FeedBack**: Deliberative sets strategy → Reactive follows it
- **Resonance**: Adaptive + Evolutionary align on solution
- **Inhibition**: Deliberative overrides hasty Reactive response

### Cycle Structure

Each loop cycle has:

```rust
pub struct LoopCycle {
    pub id: CycleId,
    pub cycle_number: u64,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub input: CycleInput,
    pub output: Option<CycleOutput>,
    pub metrics: CycleMetrics,
}
```

**Cycle Metrics:**
- Duration: How long cycle took
- Success: Whether objectives were met
- Quality: Output quality (0.0-1.0)
- Efficiency: Resource usage
- Novelty: Innovation level
- Alignment: Goal alignment

## API Reference

### Engine Initialization

```rust
use omega_loops::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create loop engine
    let mut engine = LoopEngine::new();

    // Initialize all 7 loops
    engine.initialize().await?;

    println!("Engine running: {}", engine.is_running().await);

    // Shutdown gracefully
    engine.shutdown().await?;

    Ok(())
}
```

### Executing Cycles

```rust
use omega_loops::*;
use omega_core::*;
use std::collections::HashMap;

async fn execute_learning_cycle(
    engine: &mut LoopEngine,
) -> Result<CycleOutput, Box<dyn std::error::Error>> {
    // Prepare input
    let input = CycleInput {
        data: {
            let mut map = HashMap::new();
            map.insert("task".to_string(), serde_json::json!("learn_pattern"));
            map.insert("observations".to_string(), serde_json::json!([1, 2, 3, 5, 8]));
            map
        },
        context: "fibonacci_sequence".to_string(),
        objectives: vec![
            "identify_pattern".to_string(),
            "predict_next".to_string(),
        ],
    };

    // Execute in Adaptive loop (30 min cycle)
    let output = engine.execute_cycle(LoopType::Adaptive, input).await?;

    println!("Insights: {:?}", output.insights);
    println!("Next objectives: {:?}", output.next_objectives);

    Ok(output)
}
```

### Loop Coordination

```rust
use omega_loops::*;

async fn coordinate_loops(engine: &LoopEngine) -> Result<(), Box<dyn std::error::Error>> {
    let coordinator = engine.coordinator();
    let mut coord = coordinator.write().await;

    // Create connection: Reflexive feeds into Adaptive
    coord.add_connection(LoopConnection {
        from_loop: LoopType::Reflexive,
        to_loop: LoopType::Adaptive,
        connection_type: ConnectionType::FeedForward,
        strength: 0.8,
    }).await?;

    // Create synchronization point
    coord.add_sync_point(SyncPoint {
        loops: vec![LoopType::Adaptive, LoopType::Deliberative],
        trigger_condition: "pattern_discovered".to_string(),
        action: "consolidate_learning".to_string(),
    }).await?;

    Ok(())
}
```

### Accessing Loop Statistics

```rust
let stats = engine.get_stats().await;

for (loop_type, loop_stats) in stats {
    println!("{:?} Loop:", loop_type);
    println!("  Cycles: {}", loop_stats.cycles_completed);
    println!("  Success Rate: {:.1}%", loop_stats.success_rate * 100.0);
    println!("  Avg Cycle Time: {:?}", loop_stats.average_cycle_time);
}
```

### Custom Loop Processing

```rust
use omega_loops::*;
use omega_core::*;

async fn custom_deliberative_cycle(
    coordinator: Arc<RwLock<LoopCoordinator>>,
    input: CycleInput,
) -> Result<CycleOutput, Box<dyn std::error::Error>> {
    // Access coordinator
    let coord = coordinator.read().await;

    // Get loop state
    let loop_data = coord.get_loop_by_type(LoopType::Deliberative).await?;

    // Custom processing logic
    let insights = vec![
        "Analyzed 24-hour performance".to_string(),
        "Identified optimization opportunities".to_string(),
    ];

    let output = CycleOutput {
        results: HashMap::new(),
        insights,
        actions: vec![],
        next_objectives: vec!["implement_optimizations".to_string()],
    };

    Ok(output)
}
```

## Common Patterns

### 1. Multi-Loop Learning Pipeline

```rust
async fn learning_pipeline(engine: &mut LoopEngine) -> Result<(), Box<dyn std::error::Error>> {
    // L1: Reflexive - Gather immediate observations
    let reflexive_input = CycleInput {
        data: HashMap::from([
            ("sensor_data".to_string(), serde_json::json!([1.2, 3.4, 5.6]))
        ]),
        context: "real_time_monitoring".to_string(),
        objectives: vec!["detect_anomaly".to_string()],
    };
    let reflexive_output = engine.execute_cycle(LoopType::Reflexive, reflexive_input).await?;

    // L3: Adaptive - Learn from observations
    let adaptive_input = CycleInput {
        data: HashMap::from([
            ("reflexive_insights".to_string(), serde_json::to_value(&reflexive_output.insights)?)
        ]),
        context: "pattern_learning".to_string(),
        objectives: vec!["learn_pattern".to_string()],
    };
    let adaptive_output = engine.execute_cycle(LoopType::Adaptive, adaptive_input).await?;

    // L5: Evolutionary - Systematize learning
    let evolutionary_input = CycleInput {
        data: HashMap::from([
            ("patterns".to_string(), serde_json::to_value(&adaptive_output.results)?)
        ]),
        context: "systematic_improvement".to_string(),
        objectives: vec!["optimize_detection".to_string()],
    };
    engine.execute_cycle(LoopType::Evolutionary, evolutionary_input).await?;

    Ok(())
}
```

### 2. Adaptive Feedback Control

```rust
async fn feedback_control(engine: &mut LoopEngine) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        // Fast loop: Monitor current state
        let reactive_input = CycleInput {
            data: HashMap::new(),
            context: "system_monitoring".to_string(),
            objectives: vec!["check_health".to_string()],
        };
        let reactive_output = engine.execute_cycle(LoopType::Reactive, reactive_input).await?;

        // Check if intervention needed
        if reactive_output.results.contains_key("error_detected") {
            // Slow loop: Plan corrective action
            let deliberative_input = CycleInput {
                data: HashMap::from([
                    ("error".to_string(), reactive_output.results["error_detected"].clone())
                ]),
                context: "error_correction".to_string(),
                objectives: vec!["plan_fix".to_string()],
            };
            let plan = engine.execute_cycle(LoopType::Deliberative, deliberative_input).await?;

            // Fast loop: Execute plan
            let execution_input = CycleInput {
                data: HashMap::from([
                    ("plan".to_string(), serde_json::to_value(&plan)?)
                ]),
                context: "execute_fix".to_string(),
                objectives: vec!["apply_fix".to_string()],
            };
            engine.execute_cycle(LoopType::Reactive, execution_input).await?;
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
```

### 3. Emergent Pattern Detection

```rust
async fn detect_emergent_patterns(
    coordinator: Arc<RwLock<LoopCoordinator>>,
) -> Result<Vec<EmergentPattern>, Box<dyn std::error::Error>> {
    let coord = coordinator.read().await;

    let mut patterns = Vec::new();

    // Example: Detect resonance between Adaptive and Evolutionary loops
    let adaptive_loop = coord.get_loop_by_type(LoopType::Adaptive).await?;
    let evolutionary_loop = coord.get_loop_by_type(LoopType::Evolutionary).await?;

    if let (Some(adaptive), Some(evolutionary)) = (adaptive_loop, evolutionary_loop) {
        // Check for aligned success patterns
        if adaptive.metrics.success_rate > 0.8 && evolutionary.metrics.success_rate > 0.8 {
            patterns.push(EmergentPattern {
                id: uuid::Uuid::new_v4().to_string(),
                name: "Rapid Learning Synergy".to_string(),
                description: "Adaptive and Evolutionary loops showing high success rates".to_string(),
                participating_loops: vec![LoopType::Adaptive, LoopType::Evolutionary],
                discovered_at: chrono::Utc::now(),
                strength: 0.85,
            });
        }
    }

    Ok(patterns)
}
```

### 4. Loop Scheduler

```rust
use tokio::time::{interval, Duration};

async fn loop_scheduler(engine: &mut LoopEngine) -> Result<(), Box<dyn std::error::Error>> {
    // Schedule different loops at their natural frequencies
    let mut reflexive_interval = interval(Duration::from_millis(100));
    let mut reactive_interval = interval(Duration::from_secs(5));
    let mut adaptive_interval = interval(Duration::from_secs(30 * 60));

    loop {
        tokio::select! {
            _ = reflexive_interval.tick() => {
                let input = CycleInput { /* ... */ };
                engine.execute_cycle(LoopType::Reflexive, input).await?;
            }
            _ = reactive_interval.tick() => {
                let input = CycleInput { /* ... */ };
                engine.execute_cycle(LoopType::Reactive, input).await?;
            }
            _ = adaptive_interval.tick() => {
                let input = CycleInput { /* ... */ };
                engine.execute_cycle(LoopType::Adaptive, input).await?;
            }
        }
    }
}
```

## Best Practices

### Loop Selection

**DO:**
- Match loop type to task timescale
- Use Reflexive (L1) for < 1 second tasks
- Use Adaptive (L3) for learning tasks
- Use Deliberative (L4) for planning tasks
- Use Evolutionary (L5) for optimization

**DON'T:**
- Use slow loops for time-critical tasks
- Use fast loops for long-term planning
- Run all loops at maximum frequency

### Cycle Input/Output Design

**DO:**
- Provide clear context
- Set specific objectives
- Include relevant data
- Use structured JSON for complex data

**DON'T:**
- Leave objectives empty
- Put large data in HashMap (use references)
- Ignore output insights

### Loop Coordination

**DO:**
- Use FeedForward for insight propagation
- Use FeedBack for guidance/control
- Set appropriate connection strengths
- Monitor for resonance patterns

**DON'T:**
- Create circular FeedForward loops (deadlock risk)
- Over-connect loops (noise)
- Ignore inhibition when needed

### Error Handling

```rust
async fn safe_cycle_execution(
    engine: &mut LoopEngine,
    loop_type: LoopType,
    input: CycleInput,
) -> Result<CycleOutput, Box<dyn std::error::Error>> {
    match engine.execute_cycle(loop_type, input).await {
        Ok(output) => Ok(output),
        Err(e) => {
            eprintln!("Cycle failed for {:?}: {}", loop_type, e);
            // Return default output
            Ok(CycleOutput {
                results: HashMap::new(),
                insights: vec!["Error occurred".to_string()],
                actions: vec![],
                next_objectives: vec![],
            })
        }
    }
}
```

## Performance Considerations

### 1. Async Execution

All loops run asynchronously using Tokio:

```rust
// Good: Concurrent execution
let reflexive_future = engine.execute_cycle(LoopType::Reflexive, input1);
let reactive_future = engine.execute_cycle(LoopType::Reactive, input2);

let (reflexive_output, reactive_output) = tokio::join!(reflexive_future, reactive_future);
```

### 2. Cycle Frequency

```rust
// Good: Respect natural frequencies
let reflexive_interval = Duration::from_millis(100);  // 10 Hz
let reactive_interval = Duration::from_secs(5);       // 0.2 Hz
let adaptive_interval = Duration::from_secs(1800);    // ~0.0005 Hz
```

### 3. Memory Management

```rust
// Good: Limit history size
const MAX_HISTORY_SIZE: usize = 1000;

if loop_instance.history.len() > MAX_HISTORY_SIZE {
    loop_instance.history.drain(0..loop_instance.history.len() - MAX_HISTORY_SIZE);
}
```

## Integration Examples

### With omega-memory

```rust
use omega_loops::*;
use omega_memory::*;

async fn loop_with_memory(
    engine: &mut LoopEngine,
    memory_system: &CosmicMemory,
) -> Result<(), Box<dyn std::error::Error>> {
    let input = CycleInput { /* ... */ };
    let output = engine.execute_cycle(LoopType::Adaptive, input).await?;

    // Store loop insights in memory
    for insight in output.insights {
        let memory = Memory::new(
            MemoryTier::Episodic,
            MemoryContent::Text(insight),
            vec![0.1; 1536],
            0.7,
        );
        memory_system.store(memory).await?;
    }

    Ok(())
}
```

### With omega-meta-sona

```rust
use omega_loops::*;
use omega_meta_sona::*;

async fn evolutionary_loop_with_meta_sona(
    engine: &mut LoopEngine,
    meta_sona: &mut MetaSONA,
) -> Result<(), Box<dyn std::error::Error>> {
    let input = CycleInput {
        context: "architecture_optimization".to_string(),
        objectives: vec!["improve_fitness".to_string()],
        data: HashMap::new(),
    };

    let output = engine.execute_cycle(LoopType::Evolutionary, input).await?;

    // Use output to guide architecture evolution
    if output.results.contains_key("fitness_improvement") {
        let base_arch = omega_core::Architecture { /* ... */ };
        let evolved = meta_sona.evolve_architecture(base_arch, 3).await?;
        println!("Evolved architecture: {}", evolved.name);
    }

    Ok(())
}
```

## Testing

```rust
#[tokio::test]
async fn test_loop_engine() {
    let mut engine = LoopEngine::new();
    assert!(!engine.is_running().await);

    engine.initialize().await.unwrap();
    assert!(engine.is_running().await);

    let input = CycleInput {
        data: HashMap::new(),
        context: "test".to_string(),
        objectives: vec!["test_objective".to_string()],
    };

    let output = engine.execute_cycle(LoopType::Reflexive, input).await.unwrap();
    assert!(!output.insights.is_empty());

    engine.shutdown().await.unwrap();
    assert!(!engine.is_running().await);
}

#[tokio::test]
async fn test_all_loops_created() {
    let mut engine = LoopEngine::new();
    engine.initialize().await.unwrap();

    let stats = engine.get_stats().await;
    assert_eq!(stats.len(), 7); // All 7 loops

    engine.shutdown().await.unwrap();
}
```

## Advanced Topics

### Custom Loop Processor

```rust
use omega_loops::processors::*;

struct CustomAdaptiveProcessor;

impl CustomAdaptiveProcessor {
    async fn process(&self, input: CycleInput) -> Result<CycleOutput, Box<dyn std::error::Error>> {
        // Custom logic for Adaptive loop
        let insights = vec![
            "Custom adaptive processing".to_string(),
            format!("Processed context: {}", input.context),
        ];

        Ok(CycleOutput {
            results: HashMap::new(),
            insights,
            actions: vec![],
            next_objectives: input.objectives,
        })
    }
}
```

### Loop Metrics Analysis

```rust
async fn analyze_loop_performance(engine: &LoopEngine) -> LoopPerformanceReport {
    let stats = engine.get_stats().await;

    let mut report = LoopPerformanceReport {
        total_cycles: 0,
        average_success_rate: 0.0,
        bottleneck_loop: None,
    };

    let mut min_success_rate = 1.0;

    for (loop_type, loop_stats) in stats {
        report.total_cycles += loop_stats.cycles_completed;
        report.average_success_rate += loop_stats.success_rate;

        if loop_stats.success_rate < min_success_rate {
            min_success_rate = loop_stats.success_rate;
            report.bottleneck_loop = Some(loop_type);
        }
    }

    report.average_success_rate /= 7.0; // 7 loops
    report
}

struct LoopPerformanceReport {
    total_cycles: u64,
    average_success_rate: f64,
    bottleneck_loop: Option<LoopType>,
}
```

## References

- **Source**: `omega/crates/omega-loops`
- **Core Types**: `omega-core` LoopType, CycleInput/Output
- **Examples**: `omega/crates/omega-loops/examples/`
- **Tests**: 228 tests passing, full loop coverage

## Version History

- **0.1.0** (2025-01-05): Initial release
  - 7 temporal loops (100ms to 10 years)
  - Loop coordination and connections
  - Cycle management and metrics
  - Async execution with Tokio
