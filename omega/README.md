# ExoGenesis Omega

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**A revolutionary 7-layer temporal processing architecture for artificial general intelligence**

ExoGenesis Omega implements a multi-scale temporal loop system that operates across timescales from milliseconds (Reflexive) to decades (Transcendent), enabling continuous learning, adaptation, and evolution.

## Overview

The Omega system consists of **7 nested temporal loops**, each operating at a different timescale and abstraction level:

| Loop | Timescale | Description |
|------|-----------|-------------|
| **1. Reflexive** | 100ms | Immediate sensory-motor feedback and reflexive responses |
| **2. Reactive** | 5s | Quick decision-making based on current context |
| **3. Adaptive** | 30min | Learning from recent experiences and adapting behavior |
| **4. Deliberative** | 24h | Strategic planning and reflective analysis |
| **5. Evolutionary** | 7d | Systematic improvement through variation and selection |
| **6. Transformative** | 1y | Fundamental capability changes and restructuring |
| **7. Transcendent** | 10y | Paradigm shifts and emergent properties |

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                  Transcendent Loop (10y)                │
│  ┌───────────────────────────────────────────────────┐  │
│  │          Transformative Loop (1y)                 │  │
│  │  ┌─────────────────────────────────────────────┐  │  │
│  │  │      Evolutionary Loop (7d)                 │  │  │
│  │  │  ┌───────────────────────────────────────┐  │  │  │
│  │  │  │   Deliberative Loop (24h)            │  │  │  │
│  │  │  │  ┌─────────────────────────────────┐ │  │  │  │
│  │  │  │  │   Adaptive Loop (30min)         │ │  │  │  │
│  │  │  │  │  ┌───────────────────────────┐  │ │  │  │  │
│  │  │  │  │  │  Reactive Loop (5s)       │  │ │  │  │  │
│  │  │  │  │  │  ┌─────────────────────┐  │  │ │  │  │  │
│  │  │  │  │  │  │ Reflexive Loop (1ms)│  │  │ │  │  │  │
│  │  │  │  │  │  └─────────────────────┘  │  │ │  │  │  │
│  │  │  │  │  └───────────────────────────┘  │ │  │  │  │
│  │  │  │  └─────────────────────────────────┘ │  │  │  │
│  │  │  └───────────────────────────────────────┘  │  │  │
│  │  └─────────────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

## Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
omega-loops = { path = "crates/omega-loops" }
omega-core = { path = "crates/omega-core" }
tokio = { version = "1.35", features = ["full"] }
```

### Basic Usage

```rust
use omega_loops::LoopEngine;
use omega_core::{LoopType, CycleInput};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create and initialize the loop engine
    let mut engine = LoopEngine::new();
    engine.initialize().await?;

    // Execute a cycle in the Reflexive loop
    let input = CycleInput {
        data: HashMap::from([
            ("sensory_input".to_string(), serde_json::json!("data")),
        ]),
        context: "Process sensory data".to_string(),
        objectives: vec!["Immediate response".to_string()],
    };

    let output = engine.execute_cycle(LoopType::Reflexive, input).await?;
    println!("Processed: {:?}", output);

    // Shutdown
    engine.shutdown().await?;
    Ok(())
}
```

### Running the Demo

```bash
cd omega
cargo run --example loops_demo
```

## Building & Testing

```bash
# Build all crates
cargo build

# Run tests
cargo test

# Build specific crate
cargo build --package omega-loops

# Run tests with output
cargo test -- --nocapture
```

## Project Structure

```
omega/
├── crates/
│   ├── omega-core/         # Core types, traits, and abstractions
│   ├── omega-loops/        # Temporal loop system (THIS IMPLEMENTATION)
│   ├── omega-agentdb/      # Agent database integration
│   ├── omega-memory/       # Multi-tier memory system
│   ├── omega-meta-sona/    # Meta-cognitive SONA integration
│   └── omega-runtime/      # Runtime execution environment
├── examples/
│   └── loops_demo.rs       # Demonstration of temporal loops
└── Cargo.toml             # Workspace configuration
```

## Omega Loops Implementation

The `omega-loops` crate provides a complete implementation of the temporal loop system:

### Components

1. **LoopEngine** (`src/lib.rs`)
   - Main entry point coordinating all 7 temporal loops
   - Manages loop lifecycle (initialization, execution, shutdown)
   - Provides statistics and monitoring

2. **LoopCoordinator** (`src/coordinator.rs`)
   - Implements the `LoopManager` trait from omega-core
   - Manages loop creation, cycle execution, and history
   - Maintains loop type indexing for fast lookups

3. **LoopExecutor** (`src/executor.rs`)
   - Executes cycles for specific loop types
   - Routes cycle processing to appropriate processors
   - Handles cycle lifecycle management

4. **Cycle Processors** (`src/processors/`)
   - Individual processors for each loop type
   - Implements `CycleProcessor` trait
   - Specialized logic for each temporal scale:
     - `reflexive.rs` - Immediate sensory processing
     - `reactive.rs` - Quick decision-making
     - `adaptive.rs` - Experience-based learning
     - `deliberative.rs` - Strategic planning
     - `evolutionary.rs` - Systematic improvement
     - `transformative.rs` - Fundamental changes
     - `transcendent.rs` - Paradigm shifts

### Key Features

- ✅ **Fully Async**: Built on Tokio for efficient async processing
- ✅ **Type-Safe**: Leverages Rust's type system for safety
- ✅ **Trait-Based**: Clean abstraction through omega-core traits
- ✅ **Tested**: Comprehensive unit tests for all components
- ✅ **Documented**: Complete API documentation
- ✅ **Modular**: Each loop type is independently testable

### API Example

```rust
use omega_loops::{LoopEngine, LoopCoordinator};
use omega_core::{LoopType, CycleInput, LoopManager};

// Direct coordinator usage
let mut coordinator = LoopCoordinator::new();
let temporal_loop = coordinator.create_loop(
    LoopType::Reflexive,
    "My Loop".to_string(),
    "Description".to_string()
).await?;

// Start and complete a cycle
let cycle_id = coordinator.start_cycle(&temporal_loop.id, input).await?;
coordinator.complete_cycle(&temporal_loop.id, output).await?;

// Or use the high-level LoopEngine
let mut engine = LoopEngine::new();
engine.initialize().await?;
let output = engine.execute_cycle(LoopType::Reactive, input).await?;
```

## Testing

All tests pass successfully:

```bash
$ cargo test --package omega-loops
running 7 tests
test coordinator::tests::test_coordinator_create_loop ... ok
test coordinator::tests::test_coordinator_cycle_management ... ok
test coordinator::tests::test_list_loops ... ok
test executor::tests::test_executor_creation ... ok
test executor::tests::test_executor_lifecycle ... ok
test tests::test_all_loops_created ... ok
test tests::test_loop_engine_initialization ... ok
```

## Integration with Omega Core

The loops implementation integrates seamlessly with `omega-core`:

- Uses `LoopType` enum for loop classification
- Implements `LoopManager` trait for loop management
- Works with `TemporalLoop`, `CycleInput`, `CycleOutput` types
- Supports `LoopCycle` and `CycleMetrics` for tracking

## Future Enhancements

- [ ] Persistent cycle storage
- [ ] Inter-loop communication optimization
- [ ] Advanced metrics and telemetry
- [ ] Loop synchronization primitives
- [ ] Distributed loop execution
- [ ] Neural network integration for adaptive processing

## Contributing

Contributions are welcome! Please ensure:

1. All tests pass: `cargo test`
2. Code is formatted: `cargo fmt`
3. No clippy warnings: `cargo clippy`
4. Documentation is updated

## License

Licensed under either of:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## Acknowledgments

Part of the ExoGenesis Omega project - exploring the frontiers of artificial general intelligence through multi-scale temporal processing.

---

**Built with ❤️ by the ExoGenesis Omega Team**
