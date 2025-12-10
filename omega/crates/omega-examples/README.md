# omega-examples

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Example applications demonstrating ExoGenesis Omega brain capabilities.

**Part of the [ExoGenesis-Omega](https://github.com/prancer-io/ExoGenesis-Omega) cognitive architecture.**

## Overview

This crate contains runnable examples that showcase the cognitive capabilities of the Omega Brain ecosystem. Each example demonstrates a different aspect of the system, from temporal loops to creative dream-based problem solving.

## Available Examples

### 1. loops_demo

Demonstrates the 7 temporal cognitive loops system:
- Reflexive (100ms) - Immediate reactions
- Reactive (5s) - Situational responses
- Deliberative (1min) - Thoughtful decisions
- Adaptive (30min) - Behavioral adjustments
- Reflective (1 day) - Meta-cognitive analysis
- Evolutionary (1 week) - Long-term optimization
- Transformative (1 year+) - Fundamental restructuring

```bash
cargo run --bin loops_demo
```

### 2. dream_problem_solver

A creative problem solving simulation using REM sleep dynamics:
- Neural network simulates dream states
- Reduced prefrontal inhibition enables novel associations
- Extracts insights from bizarre dream combinations
- Synthesizes creative solutions to hard problems

Featured problems:
1. **Nine Dots** - Classic lateral thinking puzzle
2. **Benzene Structure** - Kekulé's historical dream discovery
3. **Sustainable Packaging** - Innovation challenge

```bash
cargo run --bin dream_problem_solver
```

## Running Examples

```bash
# Navigate to workspace root
cd omega

# Run specific example
cargo run --bin dream_problem_solver
cargo run --bin loops_demo
```

## Use Cases Documentation

See `omega/examples/USE_CASES.md` for detailed documentation of 10 comprehensive use cases:

1. Persistent Memory AI Agents
2. Digital Twin with Life History
3. Autonomous Robot with Sleep Cycles
4. AI System Introspection
5. Creative Problem Solving with Dreams
6. Collective Consciousness Simulation
7. Adaptive Tutoring System
8. Emotional AI Companion
9. Game AI with Realistic Cognition
10. Autonomous Scientific Hypothesis Generation

## Architecture

The examples demonstrate integration of multiple omega crates:

```
┌─────────────────────────────────────────────┐
│              omega-examples                  │
│  - loops_demo (temporal cognition)          │
│  - dream_problem_solver (creative AI)       │
└────────┬────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────────────┐
│              Omega Crates                    │
│  - omega-brain (unified architecture)        │
│  - omega-loops (temporal processing)         │
│  - omega-sleep (REM/NREM simulation)        │
│  - omega-consciousness (awareness models)    │
│  - omega-hippocampus (memory formation)      │
│  - omega-snn (spiking neural networks)       │
└─────────────────────────────────────────────┘
```

## Related Crates

- **[omega-brain](../omega-brain)** - Unified cognitive architecture
- **[omega-loops](../omega-loops)** - 7 temporal loops
- **[omega-sleep](../omega-sleep)** - Sleep/wake cycles
- **[omega-consciousness](../omega-consciousness)** - Awareness models
- **[omega-hippocampus](../omega-hippocampus)** - Memory circuits
- **[omega-snn](../omega-snn)** - Spiking neural networks

## License

Licensed under the MIT License. See [LICENSE](../../LICENSE) for details.
