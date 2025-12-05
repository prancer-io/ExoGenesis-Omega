# omega-meta-sona - Self-Organizing Neural Architecture

## Overview

`omega-meta-sona` is the intelligence design engine for ExoGenesis Omega. While SONA optimizes weights within a fixed architecture, META-SONA optimizes the architecture itself using Monte Carlo Tree Search (MCTS) and Proximal Policy Optimization (PPO).

**Key Features:**
- **Architecture Search**: MCTS-based exploration of architecture space
- **Hyperparameter Optimization**: PPO for fine-tuning
- **Multi-Objective Fitness**: Capability, efficiency, alignment, novelty
- **Intelligence Factory**: Specification-based intelligence creation
- **Evolution System**: Multi-generation architecture improvement
- **Lineage Tracking**: Full genealogy of evolved architectures

**Performance:**
- Architecture search: 100+ candidates evaluated per generation
- Fitness evaluation: 4 dimensions (capability, efficiency, alignment, novelty)
- Evolution: 10-50 generations typical for convergence

**Version:** 0.1.0
**Crate:** `omega-meta-sona`
**Location:** `omega/crates/omega-meta-sona`

## Installation

```toml
[dependencies]
omega-meta-sona = "0.1.0"
omega-core = "0.1.0"
```

## Core Concepts

### Architecture Components

```rust
pub struct ArchitectureNode {
    pub id: String,
    pub node_type: NodeType,
    pub parameters: Parameters,
    pub connections: Vec<Connection>,
}

pub enum NodeType {
    Input,
    Output,
    Dense,
    Convolution,
    Attention,
    Recurrent,
    Normalization,
    Activation,
    Custom(String),
}
```

### Computational Graph

```rust
pub struct ComputationalGraph {
    pub nodes: Vec<ArchitectureNode>,
    pub edges: Vec<(String, String)>, // (from_id, to_id)
    pub parameters_count: usize,
    pub flops_estimate: u64,
}
```

### Fitness Evaluation

**Multi-Objective Fitness:**

```rust
pub struct FitnessScore {
    pub overall: f64,      // Weighted combination
    pub capability: f64,   // Task performance (0-1)
    pub efficiency: f64,   // Resource usage (0-1)
    pub alignment: f64,    // Goal alignment (0-1)
    pub novelty: f64,      // Innovation (0-1)
    pub confidence: f64,   // Evaluation certainty (0-1)
}
```

**Computation:**
```
overall = 0.4 × capability + 0.3 × efficiency + 0.2 × alignment + 0.1 × novelty
```

### Intelligence Specification

```rust
pub struct IntelligenceSpec {
    pub name: String,
    pub paradigm_preference: Option<Paradigm>,
    pub substrate_preference: Option<SubstrateType>,
    pub min_capability: f64,
    pub max_parameters: Option<usize>,
    pub target_efficiency: f64,
    pub alignment_requirements: Vec<String>,
}
```

## API Reference

### Creating Intelligences

#### Basic Creation

```rust
use omega_meta_sona::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut meta_sona = MetaSONA::new();

    // Create with defaults
    let spec = IntelligenceSpec::default();
    let intelligence = meta_sona.create_intelligence(spec).await?;

    println!("Created: {} (fitness: {:.2})",
        intelligence.name,
        intelligence.architecture.fitness.unwrap().overall
    );

    Ok(())
}
```

#### Specification-Based Creation

```rust
use omega_meta_sona::*;
use omega_core::*;

let spec = IntelligenceSpec {
    name: "HighPerformanceAI".to_string(),
    paradigm_preference: Some(Paradigm::Neural),
    substrate_preference: Some(SubstrateType::Digital),
    min_capability: 0.85,        // Require 85%+ capability
    max_parameters: Some(10_000_000), // Max 10M parameters
    target_efficiency: 0.90,     // Target 90% efficiency
    alignment_requirements: vec![
        "safe".to_string(),
        "interpretable".to_string(),
    ],
};

let intelligence = meta_sona.create_intelligence(spec).await?;
```

### Architecture Evolution

#### Basic Evolution

```rust
use omega_meta_sona::*;
use omega_core::*;

// Start with base architecture
let base_arch = Architecture {
    id: "arch-v1".to_string(),
    name: "Transformer V1".to_string(),
    paradigm: Paradigm::Neural,
    substrate: SubstrateType::Digital,
    fitness: Some(FitnessScore {
        overall: 0.75,
        capability: 0.80,
        efficiency: 0.70,
        alignment: 0.75,
        novelty: 0.60,
        confidence: 0.85,
    }),
    lineage: vec![],
    created_at: chrono::Utc::now(),
};

// Evolve for 5 generations
let evolved = meta_sona.evolve_architecture(base_arch, 5).await?;

println!("Evolved: {} (fitness: {:.2} → {:.2})",
    evolved.name,
    base_arch.fitness.unwrap().overall,
    evolved.fitness.unwrap().overall
);
```

### Architecture Search (MCTS)

```rust
use omega_meta_sona::*;

let mcts_config = MCTSConfig {
    exploration_constant: 1.414, // UCB1 exploration parameter
    max_iterations: 1000,
    max_depth: 10,
    simulation_budget: 100,
};

let mut mcts = MCTS::new(mcts_config);

// Search for optimal architecture
let best_architecture = mcts.search(
    initial_state,
    fitness_evaluator,
).await?;
```

### Hyperparameter Optimization (PPO)

```rust
use omega_meta_sona::*;

let ppo_config = PPOConfig {
    learning_rate: 3e-4,
    gamma: 0.99,              // Discount factor
    epsilon: 0.2,             // Clipping parameter
    value_coef: 0.5,          // Value loss coefficient
    entropy_coef: 0.01,       // Entropy bonus
    max_grad_norm: 0.5,       // Gradient clipping
    batch_size: 64,
    epochs: 10,
};

let mut ppo = PPOOptimizer::new(ppo_config);

// Optimize hyperparameters
let optimized = ppo.optimize(
    architecture,
    training_data,
).await?;
```

### Fitness Evaluation

```rust
use omega_meta_sona::*;

let evaluator = FitnessEvaluator::new(MetricWeight {
    capability: 0.4,
    efficiency: 0.3,
    alignment: 0.2,
    novelty: 0.1,
});

let fitness = evaluator.evaluate(&architecture, &benchmarks).await?;

println!("Fitness Breakdown:");
println!("  Capability: {:.2}", fitness.capability);
println!("  Efficiency: {:.2}", fitness.efficiency);
println!("  Alignment: {:.2}", fitness.alignment);
println!("  Novelty: {:.2}", fitness.novelty);
println!("  Overall: {:.2}", fitness.overall);
```

## Common Patterns

### 1. Iterative Architecture Refinement

```rust
async fn refine_architecture(
    meta_sona: &mut MetaSONA,
    mut architecture: Architecture,
    target_fitness: f64,
    max_iterations: usize,
) -> Result<Architecture, Box<dyn std::error::Error>> {
    for iteration in 0..max_iterations {
        // Evolve for one generation
        architecture = meta_sona.evolve_architecture(architecture.clone(), 1).await?;

        let fitness = architecture.fitness.unwrap().overall;
        println!("Iteration {}: fitness = {:.4}", iteration, fitness);

        if fitness >= target_fitness {
            println!("Target fitness reached!");
            break;
        }
    }

    Ok(architecture)
}
```

### 2. Multi-Objective Optimization

```rust
async fn pareto_optimization(
    meta_sona: &mut MetaSONA,
    base_arch: Architecture,
) -> Result<Vec<Architecture>, Box<dyn std::error::Error>> {
    let mut pareto_front = Vec::new();

    // Generate multiple evolved variants
    for _ in 0..20 {
        let evolved = meta_sona.evolve_architecture(base_arch.clone(), 3).await?;

        // Check if non-dominated
        let is_dominated = pareto_front.iter().any(|arch: &Architecture| {
            let evolved_fitness = evolved.fitness.as_ref().unwrap();
            let arch_fitness = arch.fitness.as_ref().unwrap();

            arch_fitness.capability >= evolved_fitness.capability &&
            arch_fitness.efficiency >= evolved_fitness.efficiency &&
            arch_fitness.alignment >= evolved_fitness.alignment &&
            arch_fitness.novelty >= evolved_fitness.novelty
        });

        if !is_dominated {
            pareto_front.push(evolved);
        }
    }

    Ok(pareto_front)
}
```

### 3. Lineage Tracking

```rust
fn print_architecture_lineage(architecture: &Architecture) {
    println!("Architecture Lineage:");
    println!("  Current: {} (ID: {})", architecture.name, architecture.id);

    if !architecture.lineage.is_empty() {
        println!("  Parents:");
        for (i, parent_id) in architecture.lineage.iter().enumerate() {
            println!("    Generation {}: {}", i, parent_id);
        }
    } else {
        println!("  (Base architecture, no parents)");
    }
}
```

### 4. Specification-Guided Evolution

```rust
async fn evolve_with_constraints(
    meta_sona: &mut MetaSONA,
    base_arch: Architecture,
    spec: IntelligenceSpec,
) -> Result<Architecture, Box<dyn std::error::Error>> {
    let mut current = base_arch;

    for generation in 0..50 {
        current = meta_sona.evolve_architecture(current.clone(), 1).await?;

        let fitness = current.fitness.as_ref().unwrap();

        // Check if meets specification
        if fitness.capability >= spec.min_capability &&
           fitness.efficiency >= spec.target_efficiency {
            println!("Specification met at generation {}", generation);
            break;
        }
    }

    Ok(current)
}
```

### 5. Architecture Space Exploration

```rust
async fn explore_architecture_space(
    meta_sona: &mut MetaSONA,
) -> Result<Vec<Intelligence>, Box<dyn std::error::Error>> {
    let mut intelligences = Vec::new();

    // Explore different paradigms
    for paradigm in [Paradigm::Neural, Paradigm::Symbolic, Paradigm::Hybrid] {
        let spec = IntelligenceSpec {
            name: format!("{:?} Intelligence", paradigm),
            paradigm_preference: Some(paradigm),
            min_capability: 0.7,
            ..Default::default()
        };

        let intelligence = meta_sona.create_intelligence(spec).await?;
        intelligences.push(intelligence);
    }

    Ok(intelligences)
}
```

## Best Practices

### Fitness Score Design

**DO:**
- Weight dimensions according to task priorities
- Set confidence based on evaluation quality
- Use capability as primary metric
- Consider efficiency for production systems

**DON'T:**
- Ignore any dimension (leads to pathological solutions)
- Set all weights equally (no priorities)
- Use fitness without validation

### Evolution Parameters

**DO:**
- Start with 3-5 generations for exploration
- Use 10-20 generations for refinement
- Track lineage for reproducibility
- Save checkpoints during evolution

**DON'T:**
- Evolve for 100+ generations (diminishing returns)
- Ignore convergence detection
- Evolve without fitness baselines

### Architecture Search

**MCTS Configuration:**
```rust
// Exploration vs exploitation
// Higher exploration_constant = more exploration
let balanced_config = MCTSConfig {
    exploration_constant: 1.414,  // Balanced (√2)
    max_iterations: 1000,
    ..Default::default()
};

let exploration_config = MCTSConfig {
    exploration_constant: 2.0,    // More exploration
    max_iterations: 2000,
    ..Default::default()
};

let exploitation_config = MCTSConfig {
    exploration_constant: 0.7,    // More exploitation
    max_iterations: 500,
    ..Default::default()
};
```

### Intelligence Specification

**DO:**
```rust
// Clear, achievable constraints
let good_spec = IntelligenceSpec {
    min_capability: 0.80,         // Specific target
    max_parameters: Some(1_000_000), // Resource limit
    target_efficiency: 0.85,      // Performance goal
    alignment_requirements: vec!["safe".to_string()],
    ..Default::default()
};
```

**DON'T:**
```rust
// Impossible constraints
let bad_spec = IntelligenceSpec {
    min_capability: 0.99,         // Too high
    max_parameters: Some(1000),   // Too restrictive
    target_efficiency: 0.99,      // Unrealistic
    ..Default::default()
};
```

## Error Handling

```rust
use omega_meta_sona::{MetaSONA, FactoryError};

async fn safe_create_intelligence(
    meta_sona: &mut MetaSONA,
    spec: IntelligenceSpec,
) -> Result<Intelligence, Box<dyn std::error::Error>> {
    match meta_sona.create_intelligence(spec).await {
        Ok(intelligence) => Ok(intelligence),
        Err(FactoryError::SpecificationError(msg)) => {
            eprintln!("Invalid specification: {}", msg);
            Err(Box::new(FactoryError::SpecificationError(msg)))
        }
        Err(FactoryError::EvolutionError(msg)) => {
            eprintln!("Evolution failed: {}", msg);
            Err(Box::new(FactoryError::EvolutionError(msg)))
        }
        Err(e) => Err(Box::new(e)),
    }
}
```

**Error Types:**
- `SpecificationError(String)` - Invalid intelligence spec
- `EvolutionError(String)` - Evolution process failure
- `EvaluationError(String)` - Fitness evaluation error
- `MCTSError(String)` - Architecture search error

## Performance Optimization

### 1. Parallel Evolution

```rust
use tokio::task::JoinSet;

async fn parallel_evolution(
    meta_sona: &mut MetaSONA,
    base_arch: Architecture,
    num_variants: usize,
) -> Result<Vec<Architecture>, Box<dyn std::error::Error>> {
    let mut set = JoinSet::new();

    for i in 0..num_variants {
        let mut meta_sona_clone = MetaSONA::new();
        let arch_clone = base_arch.clone();

        set.spawn(async move {
            meta_sona_clone.evolve_architecture(arch_clone, 5).await
        });
    }

    let mut results = Vec::new();
    while let Some(res) = set.join_next().await {
        results.push(res??);
    }

    Ok(results)
}
```

### 2. Early Stopping

```rust
async fn evolve_with_early_stopping(
    meta_sona: &mut MetaSONA,
    base_arch: Architecture,
    max_generations: usize,
    patience: usize,
) -> Result<Architecture, Box<dyn std::error::Error>> {
    let mut best_fitness = 0.0;
    let mut no_improvement_count = 0;
    let mut current = base_arch;

    for generation in 0..max_generations {
        current = meta_sona.evolve_architecture(current.clone(), 1).await?;

        let fitness = current.fitness.unwrap().overall;

        if fitness > best_fitness {
            best_fitness = fitness;
            no_improvement_count = 0;
        } else {
            no_improvement_count += 1;
        }

        if no_improvement_count >= patience {
            println!("Early stopping at generation {}", generation);
            break;
        }
    }

    Ok(current)
}
```

## Integration Examples

### With omega-runtime

```rust
use omega_runtime::*;
use omega_meta_sona::*;

let config = OmegaConfig::default();
let runtime = OmegaRuntime::new(config).await?;
let api = OmegaAPI::new(runtime);

// Create intelligence via META-SONA
let mut meta_sona = MetaSONA::new();
let intelligence = meta_sona.create_intelligence(IntelligenceSpec::default()).await?;

// Register with runtime
api.register_intelligence(intelligence).await?;
```

### With omega-loops

```rust
use omega_loops::*;
use omega_meta_sona::*;

async fn evolutionary_loop(
    engine: &mut LoopEngine,
    meta_sona: &mut MetaSONA,
) -> Result<(), Box<dyn std::error::Error>> {
    let input = CycleInput {
        context: "architecture_evolution".to_string(),
        objectives: vec!["improve_architecture".to_string()],
        data: HashMap::new(),
    };

    // Run evolutionary loop cycle
    let output = engine.execute_cycle(LoopType::Evolutionary, input).await?;

    // Use insights to guide evolution
    if output.insights.contains(&"low_efficiency".to_string()) {
        let spec = IntelligenceSpec {
            target_efficiency: 0.90,
            ..Default::default()
        };
        meta_sona.create_intelligence(spec).await?;
    }

    Ok(())
}
```

## Testing

```rust
#[tokio::test]
async fn test_intelligence_creation() {
    let mut meta_sona = MetaSONA::new();
    let spec = IntelligenceSpec::default();

    let result = meta_sona.create_intelligence(spec).await;
    assert!(result.is_ok());

    let intelligence = result.unwrap();
    assert!(!intelligence.name.is_empty());
    assert!(intelligence.architecture.fitness.is_some());
}

#[tokio::test]
async fn test_architecture_evolution() {
    let mut meta_sona = MetaSONA::new();

    let base_arch = Architecture {
        id: "test-arch".to_string(),
        name: "Test".to_string(),
        paradigm: Paradigm::Neural,
        substrate: SubstrateType::Digital,
        fitness: Some(FitnessScore::default()),
        lineage: vec![],
        created_at: chrono::Utc::now(),
    };

    let evolved = meta_sona.evolve_architecture(base_arch.clone(), 2).await.unwrap();

    assert_eq!(evolved.lineage.len(), 1);
    assert_eq!(evolved.lineage[0], base_arch.id);
}
```

## Advanced Topics

### Custom Fitness Functions

```rust
use omega_meta_sona::*;

struct CustomFitnessEvaluator {
    task_specific_weights: HashMap<String, f64>,
}

impl CustomFitnessEvaluator {
    async fn evaluate(&self, architecture: &Architecture) -> FitnessScore {
        // Custom evaluation logic
        let capability = self.evaluate_capability(architecture).await;
        let efficiency = self.evaluate_efficiency(architecture).await;
        let alignment = self.evaluate_alignment(architecture).await;
        let novelty = self.evaluate_novelty(architecture).await;

        FitnessScore {
            overall: capability * 0.5 + efficiency * 0.3 + alignment * 0.15 + novelty * 0.05,
            capability,
            efficiency,
            alignment,
            novelty,
            confidence: 0.90,
        }
    }

    async fn evaluate_capability(&self, _arch: &Architecture) -> f64 {
        // Run benchmarks, measure performance
        0.85
    }

    async fn evaluate_efficiency(&self, _arch: &Architecture) -> f64 {
        // Measure FLOPs, memory usage, latency
        0.80
    }

    async fn evaluate_alignment(&self, _arch: &Architecture) -> f64 {
        // Safety tests, interpretability checks
        0.90
    }

    async fn evaluate_novelty(&self, _arch: &Architecture) -> f64 {
        // Compare to known architectures
        0.70
    }
}
```

### Architecture Ensembles

```rust
async fn create_ensemble(
    meta_sona: &mut MetaSONA,
    base_spec: IntelligenceSpec,
    ensemble_size: usize,
) -> Result<Vec<Intelligence>, Box<dyn std::error::Error>> {
    let mut ensemble = Vec::new();

    for i in 0..ensemble_size {
        let mut spec = base_spec.clone();
        spec.name = format!("{} (Ensemble {})", base_spec.name, i);

        let intelligence = meta_sona.create_intelligence(spec).await?;
        ensemble.push(intelligence);
    }

    Ok(ensemble)
}
```

## References

- **Source**: `omega/crates/omega-meta-sona`
- **Simulation Results**: `docs/COMPREHENSIVE-SIMULATION-RESULTS.md` (META-SONA: 86.42% fitness)
- **Tests**: 228 tests passing
- **Dependencies**: `omega-core`, `chrono`, `serde`, `tokio`

## Version History

- **0.1.0** (2025-01-05): Initial release
  - MCTS architecture search
  - PPO hyperparameter optimization
  - Multi-objective fitness evaluation
  - Intelligence factory system
  - Architecture evolution with lineage tracking
