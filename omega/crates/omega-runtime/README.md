# omega-runtime

[![Crates.io](https://img.shields.io/crates/v/omega-runtime)](https://crates.io/crates/omega-runtime)
[![Documentation](https://docs.rs/omega-runtime/badge.svg)](https://docs.rs/omega-runtime)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Production runtime orchestrator integrating all ExoGenesis Omega subsystems with health monitoring, circuit breakers, and graceful degradation.

**Part of the [ExoGenesis-Omega](https://github.com/prancer-io/ExoGenesis-Omega) cognitive architecture.**

## Overview

`omega-runtime` is the production-ready orchestration layer that brings together all ExoGenesis Omega components: AgentDB for vector storage, the 12-tier Memory system, 7 temporal Loops, and META-SONA for architecture evolution. It provides a unified, type-safe API with built-in reliability features for deploying AI systems at scale.

The runtime handles subsystem lifecycle management, inter-component communication, health monitoring, automatic recovery, and graceful degradation under failure conditions.

## Features

- **Unified API**: Single interface to all Omega subsystems
- **Health Monitoring**: Continuous health checks for all components
- **Circuit Breakers**: Automatic failure detection and isolation
- **Graceful Degradation**: Maintains reduced functionality during failures
- **Retry Logic**: Configurable exponential backoff with jitter
- **Event System**: Comprehensive event bus for system-wide notifications
- **State Management**: Robust state machine with lifecycle tracking
- **Configuration**: Validated configuration with sensible defaults
- **Metrics**: Built-in performance and usage metrics
- **Async-First**: Full Tokio integration for high concurrency

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
omega-runtime = "0.1.0"
```

## Quick Start

```rust
use omega_runtime::{OmegaRuntime, OmegaConfig, OmegaAPI};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create runtime with default configuration
    let config = OmegaConfig::default();
    let runtime = Arc::new(OmegaRuntime::new(config).await?);

    // Start all subsystems
    runtime.start().await?;

    // Create high-level API
    let api = OmegaAPI::new(runtime.clone());

    // Store a memory
    api.store_memory(
        omega_runtime::MemoryTier::Semantic,
        "Rust is a systems programming language".to_string(),
        vec![0.1; 768],
        0.9,
    ).await?;

    // Create an intelligence
    let spec = omega_runtime::IntelligenceSpec {
        name: "Assistant".to_string(),
        min_capability: 0.80,
        ..Default::default()
    };

    let intelligence = api.create_intelligence(spec).await?;
    println!("Created intelligence: {}", intelligence.name);

    // Get runtime health
    let health = api.health_status().await?;
    println!("Runtime health: {:?}", health.overall);

    // Shutdown gracefully
    runtime.stop().await?;
    Ok(())
}
```

## Core Concepts

### OmegaRuntime

The main orchestrator managing all subsystems:

- **AgentDB**: Vector database for semantic search
- **Memory**: 12-tier cosmic memory system
- **Loops**: 7 temporal cognitive loops
- **META-SONA**: Architecture search and evolution

Each subsystem is independently monitored and can fail gracefully.

### OmegaAPI

High-level interface abstracting subsystem details:

```rust
impl OmegaAPI {
    // Memory operations
    pub async fn store_memory(...) -> Result<String>;
    pub async fn recall_memories(...) -> Result<Vec<Memory>>;

    // Intelligence operations
    pub async fn create_intelligence(...) -> Result<Intelligence>;
    pub async fn list_intelligences(...) -> Result<Vec<Intelligence>>;

    // Loop operations
    pub async fn execute_loop_cycle(...) -> Result<CycleOutput>;
    pub async fn loop_status(...) -> Result<LoopStatus>;

    // System operations
    pub async fn health_status() -> Result<HealthStatus>;
    pub async fn metrics() -> Result<RuntimeMetrics>;
}
```

### Health Monitoring

Continuous monitoring with automatic recovery:

- **Subsystem Health**: Independent health checks
- **Performance Metrics**: Latency, throughput, error rates
- **Resource Usage**: Memory, CPU, storage
- **Automatic Recovery**: Restart failed components

### Circuit Breakers

Prevent cascade failures:

- **Closed**: Normal operation
- **Open**: Failures detected, bypass subsystem
- **Half-Open**: Testing recovery

### Event System

System-wide event bus:

```rust
pub enum OmegaEvent {
    MemoryStored { tier, id },
    MemoryConsolidated { from_tier, to_tier, count },
    LoopCycleStarted { loop_type, cycle_id },
    LoopCycleCompleted { loop_type, cycle_id, success },
    IntelligenceCreated { id, name },
    ArchitectureEvolved { id, generation, fitness },
    HealthChanged { component, old, new },
    ErrorOccurred { component, error },
}
```

## Use Cases

### 1. Production AI Agent Deployment

```rust
use omega_runtime::{OmegaRuntime, OmegaConfig, OmegaAPI};
use std::sync::Arc;

// Configure for production
let config = OmegaConfig {
    memory: MemoryConfig {
        enable_auto_consolidation: true,
        consolidation_interval_secs: 3600, // 1 hour
        ..Default::default()
    },
    loops: LoopsConfig {
        enable_all_loops: true,
        ..Default::default()
    },
    agentdb: AgentDBConfig {
        dimension: 1536, // OpenAI embeddings
        hnsw_m: 32,
        hnsw_ef: 100,
        ..Default::default()
    },
    ..Default::default()
};

let runtime = Arc::new(OmegaRuntime::new(config).await?);
runtime.start().await?;

let api = OmegaAPI::new(runtime.clone());

// Agent runs indefinitely
loop {
    // Process user input
    let user_message = receive_user_message().await;

    // Recall relevant memories
    let memories = api.recall_memories(
        user_message.embedding.clone(),
        vec![MemoryTier::Session, MemoryTier::Semantic],
        10,
    ).await?;

    // Generate response using memories
    let response = generate_response(&user_message, &memories);

    // Store interaction as episodic memory
    api.store_memory(
        MemoryTier::Episodic,
        format!("User: {} | Response: {}", user_message.text, response),
        user_message.embedding,
        0.7,
    ).await?;

    send_response(response).await;
}
```

### 2. Health Monitoring and Alerts

```rust
use omega_runtime::{OmegaAPI, HealthStatus};
use tokio::time::{interval, Duration};

let api = OmegaAPI::new(runtime.clone());

// Monitor health every 30 seconds
let mut health_check = interval(Duration::from_secs(30));

loop {
    health_check.tick().await;

    let health = api.health_status().await?;

    match health.overall {
        HealthStatus::Healthy => {
            println!("✓ All systems operational");
        }
        HealthStatus::Degraded => {
            println!("⚠ System degraded:");
            for (component, status) in health.subsystems {
                if status != HealthStatus::Healthy {
                    println!("  {} is {:?}", component, status);
                }
            }
        }
        HealthStatus::Unhealthy => {
            println!("✗ System unhealthy - triggering alerts");
            send_alert(&health).await;
        }
    }
}
```

### 3. Multi-Agent Orchestration

```rust
use omega_runtime::{OmegaAPI, IntelligenceSpec, LoopType, CycleInput};

let api = OmegaAPI::new(runtime.clone());

// Create specialized agents
let researcher = api.create_intelligence(IntelligenceSpec {
    name: "Researcher".to_string(),
    required_capabilities: vec!["research".to_string()],
    min_capability: 0.85,
    ..Default::default()
}).await?;

let executor = api.create_intelligence(IntelligenceSpec {
    name: "Executor".to_string(),
    required_capabilities: vec!["execution".to_string()],
    max_inference_time: Some(100.0),
    ..Default::default()
}).await?;

// Coordinate through deliberative loop
let planning_input = CycleInput {
    data: get_task_data(),
    context: "multi_agent_planning".to_string(),
    objectives: vec!["coordinate_agents".to_string()],
};

let plan = api.execute_loop_cycle(LoopType::Deliberative, planning_input).await?;

println!("Coordination plan: {:?}", plan);
```

### 4. Self-Improving System

```rust
use omega_runtime::{OmegaAPI, LoopType, MemoryTier};

let api = OmegaAPI::new(runtime.clone());

// Run evolutionary loop weekly
tokio::spawn(async move {
    let mut week_timer = tokio::time::interval(Duration::from_secs(7 * 24 * 3600));

    loop {
        week_timer.tick().await;

        // Collect performance data
        let metrics = api.metrics().await.unwrap();

        // Analyze and consolidate learnings
        let input = CycleInput {
            data: metrics_to_data(&metrics),
            context: "weekly_improvement".to_string(),
            objectives: vec!["identify_improvements".to_string()],
        };

        let output = api.execute_loop_cycle(LoopType::Evolutionary, input).await.unwrap();

        // Store improvements as strategic memories
        for improvement in extract_improvements(&output) {
            api.store_memory(
                MemoryTier::Strategic,
                improvement.description,
                improvement.embedding,
                0.95,
            ).await.unwrap();
        }

        println!("Weekly evolution completed");
    }
});
```

### 5. Graceful Degradation Example

```rust
use omega_runtime::{OmegaAPI, OmegaEvent};

let api = OmegaAPI::new(runtime.clone());

// Subscribe to health events
let mut event_rx = runtime.subscribe_events().await;

tokio::spawn(async move {
    while let Some(event) = event_rx.recv().await {
        match event {
            OmegaEvent::HealthChanged { component, new, .. } => {
                match (component.as_str(), new) {
                    ("agentdb", HealthStatus::Unhealthy) => {
                        println!("AgentDB failed - falling back to exact match");
                        // Use simpler memory retrieval
                    }
                    ("meta_sona", HealthStatus::Unhealthy) => {
                        println!("META-SONA failed - using fixed architectures");
                        // Use pre-created architectures
                    }
                    ("loops", HealthStatus::Degraded) => {
                        println!("Loops degraded - reducing to essential loops only");
                        // Run only reflexive and reactive loops
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
});
```

## Examples

### Custom Configuration

```rust
use omega_runtime::{OmegaConfig, MemoryConfig, LoopsConfig, AgentDBConfig};

let config = OmegaConfig {
    memory: MemoryConfig {
        enable_auto_consolidation: true,
        consolidation_interval_secs: 1800,
        max_memories_per_tier: 10_000,
    },
    loops: LoopsConfig {
        enable_all_loops: true,
        reflexive_interval_ms: 100,
        reactive_interval_ms: 5000,
        adaptive_interval_ms: 1_800_000, // 30 min
    },
    agentdb: AgentDBConfig {
        dimension: 4096,
        hnsw_m: 64,
        hnsw_ef: 200,
        cache_size: 500_000,
    },
    meta_sona: MetaSONAConfig {
        enable_evolution: true,
        mcts_iterations: 1000,
        ppo_steps: 100,
    },
};

let runtime = OmegaRuntime::new(config).await?;
```

### Metrics Collection

```rust
let api = OmegaAPI::new(runtime.clone());

let metrics = api.metrics().await?;

println!("Runtime Metrics:");
println!("  Uptime: {:?}", metrics.uptime);
println!("  Total memories: {}", metrics.total_memories);
println!("  Memory by tier: {:?}", metrics.memories_by_tier);
println!("  Total cycles: {}", metrics.total_cycles);
println!("  Cycles by loop: {:?}", metrics.cycles_by_loop);
println!("  Intelligences: {}", metrics.intelligence_count);
println!("  Avg response time: {:?}", metrics.avg_response_time);
println!("  Error rate: {:.2}%", metrics.error_rate * 100.0);
```

## Architecture

The runtime integrates all Omega subsystems:

```
┌──────────────────────────────────────────┐
│          OmegaAPI (Public)                │
│  - Memory operations                      │
│  - Intelligence operations                │
│  - Loop operations                        │
│  - System operations                      │
└────────────┬─────────────────────────────┘
             │
             ▼
┌──────────────────────────────────────────┐
│         OmegaRuntime (Core)               │
│  - Lifecycle management                   │
│  - State machine                          │
│  - Event bus                              │
│  - Health monitoring                      │
└──┬────────┬────────────┬─────────────┬───┘
   │        │            │             │
   ▼        ▼            ▼             ▼
┌──────┐┌────────┐┌───────────┐┌───────────┐
│Agent ││Memory  ││  Loops    ││META-SONA  │
│ DB   ││System  ││  Engine   ││  Factory  │
└──────┘└────────┘└───────────┘└───────────┘
```

## Performance

Runtime overhead is minimal:

- **API Call Overhead**: <1ms
- **Health Check**: <10ms per subsystem
- **Event Propagation**: <1ms
- **State Transitions**: <100μs

Memory usage: ~10MB base + subsystem usage

## Related Crates

### Core Infrastructure
- **[omega-core](../omega-core)** - Core types (used by runtime)
- **[omega-persistence](../omega-persistence)** - Storage backend

### Subsystems
- **[omega-agentdb](../omega-agentdb)** - Vector database subsystem
- **[omega-memory](../omega-memory)** - Memory subsystem
- **[omega-loops](../omega-loops)** - Loop engine subsystem
- **[omega-meta-sona](../omega-meta-sona)** - Architecture evolution subsystem

### Brain-Like Cognition
- **[omega-brain](../omega-brain)** - Unified cognitive architecture
- **[omega-snn](../omega-snn)** - Spiking neural networks
- **[omega-attention](../omega-attention)** - Attention mechanisms
- **[omega-consciousness](../omega-consciousness)** - Consciousness models
- **[omega-hippocampus](../omega-hippocampus)** - Memory circuits
- **[omega-sleep](../omega-sleep)** - Sleep/wake consolidation
- **[omega-strange-loops](../omega-strange-loops)** - Self-awareness

## License

Licensed under the MIT License. See [LICENSE](../../LICENSE) for details.
