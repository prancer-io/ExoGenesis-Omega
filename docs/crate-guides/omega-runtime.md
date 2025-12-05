# omega-runtime - Production Runtime Orchestrator

## Overview

`omega-runtime` provides a unified orchestration layer for all ExoGenesis Omega subsystems. It integrates AgentDB, Memory, Loops, and Meta-SONA into a cohesive runtime environment with health monitoring, circuit breakers, retry logic, and graceful degradation.

**Key Features:**
- **Unified API**: Single entry point for all subsystem operations
- **Health Monitoring**: Real-time subsystem health checks
- **Circuit Breakers**: Automatic failure isolation
- **Retry Policies**: Configurable retry with exponential backoff
- **Event System**: Comprehensive event bus for system-wide notifications
- **State Management**: Robust lifecycle management
- **Graceful Degradation**: Fallback modes when subsystems fail

**Production-Ready:**
- ✅ Health checks for all 4 subsystems
- ✅ Automatic recovery from transient failures
- ✅ Resource monitoring and limits
- ✅ Structured logging with tracing
- ✅ Metrics export

**Version:** 0.1.0
**Crate:** `omega-runtime`
**Location:** `omega/crates/omega-runtime`

## Installation

```toml
[dependencies]
omega-runtime = "0.1.0"
omega-core = "0.1.0"
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
```

## Core Concepts

### Runtime Architecture

```
┌──────────────────────────────────────────────────┐
│              OmegaRuntime                        │
├──────────────────────────────────────────────────┤
│  ┌────────────┐  ┌────────────┐  ┌────────────┐ │
│  │  AgentDB   │  │   Memory   │  │   Loops    │ │
│  │ Subsystem  │  │ Subsystem  │  │ Subsystem  │ │
│  └────────────┘  └────────────┘  └────────────┘ │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐ │
│  │ Meta-SONA  │  │  Health    │  │  Circuit   │ │
│  │ Subsystem  │  │ Monitor    │  │  Breakers  │ │
│  └────────────┘  └────────────┘  └────────────┘ │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐ │
│  │  Retry     │  │ Degradation│  │   Events   │ │
│  │  Policy    │  │ Manager    │  │    Bus     │ │
│  └────────────┘  └────────────┘  └────────────┘ │
└──────────────────────────────────────────────────┘
                      ▲
                      │
                  OmegaAPI
```

### Runtime State Machine

```rust
pub enum RuntimeState {
    Created,      // Initial state
    Starting,     // Starting subsystems
    Running,      // All subsystems operational
    Degraded,     // Some subsystems failed
    Stopping,     // Graceful shutdown
    Stopped,      // All subsystems stopped
    Error,        // Unrecoverable error
}
```

### Health Status

```rust
pub struct SubsystemHealth {
    pub name: String,
    pub status: HealthStatus,
    pub last_check: DateTime<Utc>,
    pub error_count: u64,
    pub uptime_seconds: u64,
}

pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}
```

### Configuration

```rust
pub struct OmegaConfig {
    pub agentdb: AgentDBConfig,
    pub memory: MemoryConfig,
    pub loops: LoopsConfig,
    pub meta_sona: MetaSONAConfig,
    pub health_monitor: HealthMonitorConfig,
    pub retry_policy: RetryConfig,
}
```

## API Reference

### Runtime Initialization

```rust
use omega_runtime::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create runtime with default config
    let config = OmegaConfig::default();
    let runtime = Arc::new(OmegaRuntime::new(config).await?);

    // Start all subsystems
    runtime.start().await?;

    println!("Runtime state: {:?}", runtime.state().await);

    // Create API
    let api = OmegaAPI::new(runtime.clone());

    // Use API for operations...

    // Shutdown gracefully
    runtime.stop().await?;

    Ok(())
}
```

### Custom Configuration

```rust
use omega_runtime::*;

let config = OmegaConfig {
    agentdb: AgentDBConfig {
        dimension: 1536,         // OpenAI embeddings
        hnsw_m: 32,
        hnsw_ef: 100,
        cache_size: 1_000_000,   // 1M vectors
    },
    memory: MemoryConfig {
        max_memories_per_tier: 10_000,
        auto_consolidate: true,
        consolidation_interval_secs: 3600, // 1 hour
    },
    loops: LoopsConfig {
        enable_reflexive: true,
        enable_reactive: true,
        enable_adaptive: true,
        enable_deliberative: true,
        enable_evolutionary: false,  // Disable long-running loops
        enable_transformative: false,
        enable_transcendent: false,
    },
    meta_sona: MetaSONAConfig {
        mcts_iterations: 1000,
        ppo_learning_rate: 3e-4,
        evolution_enabled: true,
    },
    health_monitor: HealthMonitorConfig {
        check_interval_secs: 30,
        failure_threshold: 3,
        recovery_timeout_secs: 300,
    },
    retry_policy: RetryConfigBuilder::default()
        .max_retries(3)
        .initial_delay_ms(100)
        .max_delay_ms(5000)
        .build(),
};

let runtime = OmegaRuntime::new(config).await?;
```

### Using the API

#### Store and Query Memories

```rust
use omega_runtime::*;
use omega_core::*;

let api = OmegaAPI::new(runtime.clone());

// Store memory
let memory = Memory::new(
    MemoryTier::Semantic,
    MemoryContent::Text("Important fact".to_string()),
    vec![0.1; 1536],
    0.9,
);

let memory_id = api.store_memory(memory).await?;

// Query memories
let query = QueryBuilder::new()
    .tiers(vec![MemoryTier::Semantic])
    .limit(10)
    .build();

let results = api.query_memories(query).await?;
```

#### Execute Loop Cycles

```rust
let input = CycleInput {
    data: HashMap::new(),
    context: "task_processing".to_string(),
    objectives: vec!["complete_task".to_string()],
};

let output = api.execute_loop_cycle(LoopType::Adaptive, input).await?;
```

#### Create Intelligences

```rust
let spec = IntelligenceSpec {
    name: "TaskAgent".to_string(),
    min_capability: 0.8,
    ..Default::default()
};

let intelligence = api.create_intelligence(spec).await?;
```

#### Vector Operations

```rust
let embedding = vec![0.1; 1536];
let metadata = serde_json::json!({"type": "document"});

// Store vector
let vector_id = api.store_vector(embedding.clone(), metadata).await?;

// Search vectors
let results = api.search_vectors(&embedding, 10).await?;
```

### Health Monitoring

```rust
use omega_runtime::*;

// Get overall runtime health
let health = api.get_runtime_health().await?;

println!("Runtime state: {:?}", health.state);
println!("Uptime: {} seconds", health.uptime_seconds);

// Get subsystem health
for subsystem in health.subsystems {
    println!("{}: {:?}", subsystem.name, subsystem.status);
    if subsystem.error_count > 0 {
        println!("  Errors: {}", subsystem.error_count);
    }
}
```

### Metrics and Monitoring

```rust
let metrics = api.get_metrics().await?;

println!("Operations:");
println!("  Memory stores: {}", metrics.memory_stores);
println!("  Memory queries: {}", metrics.memory_queries);
println!("  Loop cycles: {}", metrics.loop_cycles);
println!("  Vector searches: {}", metrics.vector_searches);

println!("\nPerformance:");
println!("  Avg query latency: {:?}", metrics.avg_query_latency);
println!("  Avg cycle time: {:?}", metrics.avg_cycle_time);
```

### Event System

```rust
use omega_runtime::*;

// Subscribe to events
let mut event_rx = runtime.subscribe_events().await;

tokio::spawn(async move {
    while let Some(event) = event_rx.recv().await {
        match event {
            OmegaEvent::MemoryStored { tier, id } => {
                println!("Memory stored in {:?}: {}", tier, id);
            }
            OmegaEvent::LoopCycleCompleted { loop_type, success } => {
                println!("{:?} loop cycle: {}", loop_type, if success { "✓" } else { "✗" });
            }
            OmegaEvent::HealthCheckFailed { subsystem, error } => {
                eprintln!("Health check failed for {}: {}", subsystem, error);
            }
            _ => {}
        }
    }
});
```

## Common Patterns

### 1. Production Deployment

```rust
use omega_runtime::*;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup logging
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .with_line_number(true)
        .init();

    info!("Starting ExoGenesis Omega Runtime");

    // Load config from environment or file
    let config = load_production_config()?;

    // Create runtime
    let runtime = Arc::new(OmegaRuntime::new(config).await?);

    // Start with health monitoring
    runtime.start().await?;

    // Create API
    let api = OmegaAPI::new(runtime.clone());

    // Setup graceful shutdown
    let runtime_clone = runtime.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        info!("Shutdown signal received");
        runtime_clone.stop().await.unwrap();
    });

    // Run application
    run_application(api).await?;

    Ok(())
}
```

### 2. Error Handling with Circuit Breakers

```rust
async fn resilient_operation(api: &OmegaAPI) -> Result<(), APIError> {
    // Circuit breaker automatically opens after repeated failures
    // and closes after recovery timeout

    for _ in 0..10 {
        match api.store_memory(memory.clone()).await {
            Ok(id) => println!("Stored: {}", id),
            Err(APIError::CircuitBreakerOpen) => {
                println!("Circuit breaker open, waiting...");
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
            Err(e) => return Err(e),
        }
    }

    Ok(())
}
```

### 3. Graceful Degradation

```rust
async fn query_with_fallback(
    api: &OmegaAPI,
    query: Query,
) -> Result<Vec<Memory>, APIError> {
    match api.query_memories(query.clone()).await {
        Ok(results) => Ok(results),
        Err(APIError::SubsystemDegraded(subsystem)) => {
            eprintln!("{} degraded, using fallback", subsystem);

            // Fallback to simpler query
            let fallback_query = QueryBuilder::new()
                .tiers(vec![MemoryTier::Immediate, MemoryTier::Session])
                .limit(5)
                .build();

            api.query_memories(fallback_query).await
        }
        Err(e) => Err(e),
    }
}
```

### 4. Multi-Subsystem Workflow

```rust
async fn intelligent_workflow(api: &OmegaAPI) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create intelligence
    let spec = IntelligenceSpec {
        name: "WorkflowAgent".to_string(),
        min_capability: 0.85,
        ..Default::default()
    };
    let intelligence = api.create_intelligence(spec).await?;

    // 2. Execute adaptive loop
    let input = CycleInput {
        context: "workflow_execution".to_string(),
        objectives: vec!["process_data".to_string()],
        data: HashMap::new(),
    };
    let output = api.execute_loop_cycle(LoopType::Adaptive, input).await?;

    // 3. Store results in memory
    for insight in output.insights {
        let memory = Memory::new(
            MemoryTier::Episodic,
            MemoryContent::Text(insight),
            vec![0.1; 1536],
            0.8,
        );
        api.store_memory(memory).await?;
    }

    // 4. Query related memories
    let query = QueryBuilder::new()
        .tiers(vec![MemoryTier::Episodic, MemoryTier::Semantic])
        .limit(10)
        .build();
    let memories = api.query_memories(query).await?;

    println!("Workflow completed, retrieved {} memories", memories.len());

    Ok(())
}
```

### 5. Health Monitoring Loop

```rust
async fn monitor_health(api: OmegaAPI) {
    let mut interval = tokio::time::interval(Duration::from_secs(30));

    loop {
        interval.tick().await;

        match api.get_runtime_health().await {
            Ok(health) => {
                if health.state == RuntimeState::Degraded {
                    eprintln!("Runtime degraded!");
                    for subsystem in health.subsystems {
                        if subsystem.status != HealthStatus::Healthy {
                            eprintln!("  {}: {:?}", subsystem.name, subsystem.status);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to get health: {}", e);
            }
        }
    }
}
```

## Best Practices

### Configuration

**DO:**
- Use environment variables for secrets
- Set appropriate resource limits
- Enable health monitoring in production
- Configure retry policies for transient failures

**DON'T:**
- Hardcode credentials
- Disable health checks
- Use default config in production

### Error Handling

**DO:**
```rust
match api.store_memory(memory).await {
    Ok(id) => println!("Stored: {}", id),
    Err(APIError::SubsystemDegraded(name)) => {
        // Use fallback
    }
    Err(APIError::CircuitBreakerOpen) => {
        // Wait and retry
    }
    Err(e) => {
        // Log and propagate
        tracing::error!("Failed to store memory: {}", e);
        return Err(e.into());
    }
}
```

### Resource Management

**DO:**
- Monitor memory usage
- Set limits on vector cache size
- Limit concurrent operations
- Clean up old memories with consolidation

**DON'T:**
- Store unbounded data
- Ignore memory pressure
- Run all 7 loops in resource-constrained environments

### Shutdown

**DO:**
```rust
// Graceful shutdown
tokio::select! {
    _ = tokio::signal::ctrl_c() => {
        runtime.stop().await?;
    }
    result = application_logic() => {
        runtime.stop().await?;
        result?;
    }
}
```

## Performance Optimization

### 1. Concurrent Operations

```rust
use tokio::task::JoinSet;

async fn batch_operations(api: &OmegaAPI, memories: Vec<Memory>) -> Result<(), APIError> {
    let mut set = JoinSet::new();

    for memory in memories {
        let api_clone = api.clone();
        set.spawn(async move {
            api_clone.store_memory(memory).await
        });
    }

    while let Some(res) = set.join_next().await {
        res??; // Handle both join and API errors
    }

    Ok(())
}
```

### 2. Connection Pooling

Runtime automatically manages connection pools for all subsystems. Configure pool sizes:

```rust
let config = OmegaConfig {
    agentdb: AgentDBConfig {
        cache_size: 1_000_000,  // Adjust based on available RAM
        ..Default::default()
    },
    ..Default::default()
};
```

### 3. Metrics Collection

```rust
use std::time::Instant;

async fn monitored_operation(api: &OmegaAPI) -> Result<(), APIError> {
    let start = Instant::now();

    let result = api.query_memories(query).await;

    let duration = start.elapsed();
    tracing::info!("Query completed in {:?}", duration);

    result.map(|_| ())
}
```

## Integration Examples

### With Web Framework (Axum)

```rust
use axum::{Router, Json, extract::State};
use omega_runtime::*;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let runtime = Arc::new(OmegaRuntime::new(OmegaConfig::default()).await.unwrap());
    runtime.start().await.unwrap();

    let api = OmegaAPI::new(runtime.clone());

    let app = Router::new()
        .route("/health", axum::routing::get(health_check))
        .route("/memories", axum::routing::post(store_memory))
        .with_state(api);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_check(State(api): State<OmegaAPI>) -> Json<RuntimeHealth> {
    Json(api.get_runtime_health().await.unwrap())
}
```

## Testing

```rust
#[tokio::test]
async fn test_runtime_lifecycle() {
    let config = OmegaConfig::default();
    let runtime = OmegaRuntime::new(config).await.unwrap();

    // Start
    runtime.start().await.unwrap();
    assert_eq!(runtime.state().await, RuntimeState::Running);

    // Use
    let api = OmegaAPI::new(Arc::new(runtime));
    let metrics = api.get_metrics().await.unwrap();
    assert_eq!(metrics.memory_stores, 0);

    // Stop
    api.shutdown().await.unwrap();
}
```

## References

- **Source**: `omega/crates/omega-runtime`
- **Examples**: Production deployment patterns
- **Tests**: 228 tests passing, full subsystem integration coverage

## Version History

- **0.1.0** (2025-01-05): Initial release
  - Unified API for all subsystems
  - Health monitoring and circuit breakers
  - Event system and metrics
  - Production-ready error handling
