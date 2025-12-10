# ExoGenesis-Omega

**A Biologically-Inspired Cognitive Architecture for Artificial General Intelligence**

ExoGenesis-Omega is a comprehensive Rust-based cognitive architecture that models artificial general intelligence through 15 interconnected crates simulating biological neural systems, consciousness emergence, memory consolidation, and self-awareness.

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         OMEGA-BRAIN (Unified Orchestrator)              │
│   Integrates all subsystems into a coherent cognitive architecture      │
└─────────────────────────────────────────────────────────────────────────┘
         │              │              │              │              │
         ▼              ▼              ▼              ▼              ▼
┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐
│   NEURAL    │ │  ATTENTION  │ │CONSCIOUSNESS│ │   MEMORY    │ │    SLEEP    │
│  SUBSTRATE  │ │   SYSTEM    │ │    CORE     │ │   SYSTEM    │ │    CYCLE    │
│ (omega-snn) │ │  (omega-    │ │   (omega-   │ │  (omega-    │ │   (omega-   │
│             │ │  attention) │ │consciousness│ │ hippocampus)│ │    sleep)   │
└─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘
         │              │              │              │              │
         └──────────────┴──────────────┼──────────────┴──────────────┘
                                       │
         ┌─────────────────────────────┴─────────────────────────────┐
         ▼                                                           ▼
┌─────────────────────────────┐                     ┌─────────────────────────────┐
│     SELF-AWARENESS          │                     │    TEMPORAL PROCESSING      │
│  (omega-strange-loops)      │                     │      (omega-loops)          │
└─────────────────────────────┘                     └─────────────────────────────┘
```

---

## The 15 Crates

### Tier 1: Neural Foundation

| Crate | Purpose | Key Features |
|-------|---------|--------------|
| **omega-core** | Foundation types & traits | Intelligence enum, Architecture struct, 12-tier MemoryHierarchy, 7 Loop types, trait definitions |
| **omega-snn** | Spiking neural networks | LIF/Adaptive LIF neurons, asymmetric STDP plasticity, short-term plasticity, 4 neuromodulators (dopamine, norepinephrine, serotonin, acetylcholine), population coding |

### Tier 2: Cognitive Processes

| Crate | Purpose | Key Features |
|-------|---------|--------------|
| **omega-attention** | Attention mechanisms | 40 attention types (scaled dot-product, flash, linear, sparse, hyperbolic, GAT, memory-augmented, temporal), dual pathway (top-down/bottom-up), 6-feature salience detection, working memory (7±2 capacity) |
| **omega-consciousness** | Consciousness modeling | IIT Phi calculation, FEP predictive hierarchy (5 levels), GWT global workspace (7 items), emergence detection, awareness level tracking |
| **omega-strange-loops** | Self-awareness | Hofstadter's strange loops, tangled hierarchies, recursive mirrors, meta-cognition, self-model with prediction/confidence tracking, paradox resolution |

### Tier 3: Memory Systems

| Crate | Purpose | Key Features |
|-------|---------|--------------|
| **omega-hippocampus** | Memory consolidation | Complete EC→DG→CA3→CA1 hippocampal circuit, 100 place cells, 40 grid cells, sharp-wave ripple replay, pattern separation (10x expansion), pattern completion |
| **omega-memory** | Multi-scale memory | Individual/Species/Cosmic scales, 12-tier hierarchy (Sensory→Transcendent), consolidation mechanisms, query system |
| **omega-agentdb** | Vector storage | HNSW index with SimSIMD acceleration, reflexion storage, causal chain storage, skill storage |
| **omega-persistence** | Database layer | SQLite with 7 tables (architectures, memory_entries, loop_states, consciousness_snapshots, sleep_cycles, neural_patterns, evaluations) |

### Tier 4: Temporal & Sleep

| Crate | Purpose | Key Features |
|-------|---------|--------------|
| **omega-loops** | Temporal processing | 7 nested loops (Reflexive→Transcendent), 7 processors, message bus architecture, coordinator/executor pattern |
| **omega-sleep** | Sleep architecture | Two-process model (homeostatic+circadian), N1/N2/N3/REM stages, spindles (11-16Hz), slow waves (0.5-4Hz), K-complexes, dream simulation, memory consolidation |

### Tier 5: Integration & Evolution

| Crate | Purpose | Key Features |
|-------|---------|--------------|
| **omega-brain** | Unified orchestrator | Integrates all subsystems: neural substrate, attention, consciousness core, memory systems, sleep cycles, self-awareness, runtime adaptation (MicroLoRA, BaseLoRA, EWC++, ReasoningBank) |
| **omega-meta-sona** | Architecture search | MCTS + PPO optimization, 4D fitness evaluation (capability, efficiency, alignment, novelty), configurable benchmark suite |
| **omega-runtime** | Execution infrastructure | State machine lifecycle (Created→Running→Suspended→Terminated), event bus, health monitoring, circuit breakers, retry policies, graceful degradation |
| **omega-examples** | Demonstrations | loops_demo (temporal processing), dream_problem_solver (creative insight via REM), quantum_gravity_dreamer (discovery through dreams) |

---

## Key Biological Inspirations

### 1. Spiking Neural Networks (omega-snn)

```rust
// Leaky Integrate-and-Fire neuron with membrane dynamics
pub struct LIFNeuron {
    membrane_potential: f32,      // Current voltage
    threshold: f32,               // Spike threshold (-55mV typical)
    reset_potential: f32,         // Post-spike reset (-70mV)
    leak_conductance: f32,        // Membrane leak rate
    refractory_remaining: f32,    // Refractory period counter
}

// Spike-Timing Dependent Plasticity
pub struct STDPRule {
    a_plus: f32,      // LTP amplitude (pre before post)
    a_minus: f32,     // LTD amplitude (post before pre)
    tau_plus: f32,    // LTP time constant (~20ms)
    tau_minus: f32,   // LTD time constant (~20ms)
}

// Four neuromodulatory systems
pub enum Neuromodulator {
    Dopamine,       // Reward prediction, motivation
    Norepinephrine, // Arousal, attention, fight-or-flight
    Serotonin,      // Mood regulation, satiety
    Acetylcholine,  // Learning, memory, attention
}
```

### 2. Hippocampal Memory Circuit (omega-hippocampus)

```rust
// Complete hippocampal formation
pub struct HippocampalCircuit {
    entorhinal_cortex: EntorhinalCortex,  // Input layer
    dentate_gyrus: DentateGyrus,           // Pattern separation (10x expansion)
    ca3: CA3Region,                         // Pattern completion (recurrent)
    ca1: CA1Region,                         // Output and consolidation
}

// Spatial representation
pub struct PlaceCell {
    place_field_center: [f32; 2],  // Preferred location
    field_radius: f32,              // Spatial extent
    peak_rate: f32,                 // Maximum firing rate
}

pub struct GridCell {
    grid_spacing: f32,      // Distance between fields
    grid_orientation: f32,  // Angular offset
    grid_phase: [f32; 2],   // Spatial phase
}

// Memory replay during sleep
pub struct SharpWaveRipple {
    sequences: Vec<ReplaySequence>,  // Compressed memory traces
    compression_factor: f32,         // 5-20x faster than real-time
}
```

### 3. Sleep Cycles (omega-sleep)

```rust
// Two-process sleep regulation
pub struct TwoProcessModel {
    process_s: f32,  // Homeostatic sleep pressure (builds during wake)
    process_c: f32,  // Circadian rhythm (24-hour oscillation)
}

// Sleep stage characteristics
pub enum SleepStage {
    N1,  // Light sleep, hypnagogic imagery, theta waves
    N2,  // Sleep spindles (11-16Hz), K-complexes, memory tagging
    N3,  // Slow waves (0.5-4Hz), delta power, synaptic consolidation
    REM, // Dreams, emotional processing, creative problem-solving
}

// Sleep oscillations
pub struct SleepSpindle {
    frequency: f32,     // 11-16 Hz (sigma band)
    duration: Duration, // 0.5-2 seconds
    amplitude: f32,     // Thalamocortical burst strength
}

pub struct SlowWave {
    frequency: f32,  // 0.5-4 Hz (delta band)
    up_state: bool,  // Cortical activation phase
    down_state: bool, // Cortical silence phase
}
```

### 4. Consciousness Theories (omega-consciousness)

```rust
// Integrated Information Theory (Tononi)
pub struct IITCore {
    phi: f32,                    // Integrated information (consciousness measure)
    constellation: Vec<Concept>, // Conceptual structure
    mip: MinimumInformationPartition,
}

// Global Workspace Theory (Baars)
pub struct GlobalWorkspace {
    workspace_capacity: usize,   // ~7 items (Miller's law)
    broadcast_threshold: f32,    // Salience required for broadcast
    active_coalitions: Vec<Coalition>,
}

// Free Energy Principle (Friston)
pub struct FEPHierarchy {
    levels: Vec<PredictiveLevel>,  // 5 hierarchical levels
    precision_weighting: Vec<f32>, // Confidence in predictions
}

pub struct PredictiveLevel {
    predictions: Vec<f32>,      // Top-down expectations
    prediction_errors: Vec<f32>, // Bottom-up surprises
    learning_rate: f32,         // Error correction speed
}
```

### 5. Strange Loops (omega-strange-loops)

```rust
// Hofstadter's self-referential structures
pub struct StrangeLoop {
    levels: Vec<HierarchyLevel>,
    crossing_points: Vec<LevelCrossing>,  // Where hierarchy folds back
    self_symbol: Symbol,                   // The "I" representation
}

pub struct TangledHierarchy {
    nodes: Vec<HierarchyNode>,
    tangled_edges: Vec<TangledEdge>,  // Edges that violate strict hierarchy
}

// Meta-cognition: thinking about thinking
pub struct MetaCognition {
    current_thought: Thought,
    thought_about_thought: Option<Box<MetaCognition>>,  // Recursive
    confidence: f32,
    uncertainty_about_confidence: f32,  // Meta-uncertainty
}

// Self-model with prediction
pub struct SelfModel {
    predicted_next_state: State,
    actual_state: State,
    prediction_error: f32,
    model_confidence: f32,
}
```

---

## The 7 Temporal Loops

ExoGenesis-Omega processes information across seven nested temporal scales:

| Loop | Timescale | Function | Biological Analog |
|------|-----------|----------|-------------------|
| **Reflexive** | 1-100ms | Immediate reactions | Spinal reflexes, brainstem |
| **Reactive** | 100ms-1s | Stimulus-response | Amygdala, basal ganglia |
| **Deliberative** | 1-10s | Conscious planning | Prefrontal cortex |
| **Reflective** | 10s-10m | Self-monitoring | Default mode network |
| **Learning** | 10m-1d | Skill acquisition | Hippocampus, cerebellum |
| **Developmental** | 1d-1y | Long-term adaptation | Synaptic pruning, myelination |
| **Transcendent** | 1y+ | Identity evolution | Personality, wisdom |

```rust
pub enum LoopType {
    Reflexive,     // Fastest: immediate survival responses
    Reactive,      // Fast: emotional and habitual responses
    Deliberative,  // Medium: conscious thought and planning
    Reflective,    // Slow: metacognition and self-awareness
    Learning,      // Slower: knowledge consolidation
    Developmental, // Long: personality and skill development
    Transcendent,  // Longest: wisdom and self-transcendence
}
```

---

## 12-Tier Memory Hierarchy

From millisecond sensory buffers to transcendent cosmic memory:

| Tier | Name | Duration | Capacity | Purpose |
|------|------|----------|----------|---------|
| 1 | Sensory | ~250ms | High | Raw perceptual buffers |
| 2 | Iconic | ~1s | Medium | Visual persistence |
| 3 | Echoic | ~4s | Medium | Auditory persistence |
| 4 | Working | 15-30s | 7±2 items | Active manipulation |
| 5 | Short-term | Minutes | Limited | Temporary storage |
| 6 | Long-term | Hours-Days | Large | Consolidated memories |
| 7 | Episodic | Years | Large | Personal experiences |
| 8 | Semantic | Lifetime | Vast | Facts and concepts |
| 9 | Procedural | Lifetime | Moderate | Skills and habits |
| 10 | Collective | Generations | Enormous | Shared cultural knowledge |
| 11 | Phylogenetic | Evolutionary | Immense | Species-level patterns |
| 12 | Transcendent | Cosmic | Infinite | Universal principles |

---

## 40 Attention Mechanisms

The omega-attention crate implements a comprehensive attention system:

### Core Mechanisms
- Scaled Dot-Product Attention
- Multi-Head Attention
- Self-Attention
- Cross-Attention

### Efficient Variants
- Flash Attention (memory-efficient)
- Linear Attention (O(n) complexity)
- Sparse Attention (selective computation)
- Sliding Window Attention

### Specialized Types
- Hyperbolic Attention (hierarchical data)
- Graph Attention Networks (GAT)
- Memory-Augmented Attention
- Temporal Attention (sequences)
- Rotary Position Embeddings (RoPE)
- Alibi (linear bias)

### Biological-Inspired
- Top-Down Attention (goal-directed)
- Bottom-Up Attention (stimulus-driven)
- Salience Detection (6 features: intensity, color, orientation, motion, flicker, depth)
- Inhibition of Return (IOR)
- Priority Mapping

---

## Runtime Adaptation

The omega-brain crate includes sophisticated runtime adaptation mechanisms:

```rust
// Micro-adjustments for immediate adaptation
pub struct MicroLoRA {
    rank: usize,           // Low-rank adaptation dimension
    alpha: f32,            // Scaling factor
    target_modules: Vec<String>,
}

// Base model fine-tuning
pub struct BaseLoRA {
    pretrained_weights: Weights,
    lora_weights: LoRAWeights,
    merge_ratio: f32,
}

// Elastic Weight Consolidation (prevents catastrophic forgetting)
pub struct EWCPlusPlus {
    fisher_information: FisherMatrix,  // Parameter importance
    old_parameters: Parameters,         // Previous optimal weights
    lambda: f32,                        // Regularization strength
}

// Reasoning pattern storage
pub struct ReasoningBank {
    patterns: Vec<ReasoningPattern>,
    retrieval_index: HNSWIndex,
    usage_statistics: UsageStats,
}
```

---

## Example Applications

### Digital Twin Social Network

The `digital-twin-social` example demonstrates a full implementation:

```rust
// ARIA AI Companion with personality
pub struct ARIACompanion {
    personality: PersonalityProfile,  // Big Five + Schwartz Values
    emotional_state: EmotionalState,
    relationship_model: RelationshipModel,
}

// 7 emotional temporal loops
pub struct EmotionalLoopProcessor {
    reflexive: ReflexiveEmotionalLoop,   // Startle, fear
    reactive: ReactiveEmotionalLoop,     // Mood shifts
    deliberative: DeliberativeLoop,      // Emotional regulation
    reflective: ReflectiveLoop,          // Emotional insight
    learning: LearningLoop,              // Emotional patterns
    developmental: DevelopmentalLoop,    // Emotional growth
    transcendent: TranscendentLoop,      // Emotional wisdom
}

// Privacy-preserving interactions
pub struct ZeroKnowledgeLayer {
    commitment_scheme: PedersenCommitment,
    proof_system: GrothProofSystem,
}
```

### Dream-Based Problem Solving

```rust
// Creative insight through REM simulation
pub struct DreamProblemSolver {
    problem: Problem,
    rem_cycles: Vec<REMCycle>,
    insight_detector: InsightDetector,
}

// Example problems solved via dreams:
// - Nine dots puzzle (lateral thinking)
// - Benzene structure (Kekulé's dream)
// - Creative packaging design
```

---

## Technical Specifications

| Aspect | Specification |
|--------|---------------|
| **Language** | Rust 2021 Edition |
| **Async Runtime** | Tokio |
| **Database** | SQLite (rusqlite) + Custom HNSW |
| **Serialization** | Serde JSON |
| **Vector Math** | SimSIMD for SIMD acceleration |
| **Memory Model** | 12-tier hierarchy |
| **Temporal Range** | 1ms to years |
| **Attention Mechanisms** | 40 types |
| **Consciousness Theories** | 3 integrated (IIT + GWT + FEP) |
| **Sleep Stages** | 4 (N1, N2, N3, REM) |
| **Neuromodulators** | 4 (DA, NE, 5HT, ACh) |
| **Hippocampal Cells** | 100 place + 40 grid |

---

## Getting Started

```bash
# Clone the repository
git clone https://github.com/your-org/ExoGenesis-Omega.git
cd ExoGenesis-Omega

# Build all crates
cargo build --release

# Run the loops demonstration
cargo run --example loops_demo

# Run the dream problem solver
cargo run --example dream_problem_solver

# Run tests
cargo test --all
```

---

## Project Structure

```
ExoGenesis-Omega/
├── omega/
│   ├── crates/
│   │   ├── omega-core/          # Foundation types and traits
│   │   ├── omega-snn/           # Spiking neural networks
│   │   ├── omega-attention/     # 40 attention mechanisms
│   │   ├── omega-consciousness/ # IIT + GWT + FEP
│   │   ├── omega-strange-loops/ # Self-awareness
│   │   ├── omega-hippocampus/   # Memory consolidation
│   │   ├── omega-memory/        # Multi-scale memory
│   │   ├── omega-agentdb/       # Vector storage
│   │   ├── omega-persistence/   # SQLite persistence
│   │   ├── omega-loops/         # 7 temporal loops
│   │   ├── omega-sleep/         # Sleep cycles
│   │   ├── omega-brain/         # Unified orchestrator
│   │   ├── omega-meta-sona/     # Architecture search
│   │   ├── omega-runtime/       # Execution infrastructure
│   │   └── omega-examples/      # Demonstrations
│   └── examples/
│       └── digital-twin-social/ # Full application example
├── docs/
│   ├── USE_CASES.md
│   ├── LOOPS_IMPLEMENTATION.md
│   └── technical/
└── research/
```

---

## Research Foundations

ExoGenesis-Omega is grounded in established neuroscience and cognitive science:

### Neuroscience
- **Hodgkin-Huxley** → LIF neuron models
- **Hebb's Rule** → STDP plasticity
- **O'Keefe & Moser** → Place and grid cells
- **Buzsáki** → Sharp-wave ripples and replay

### Consciousness Studies
- **Tononi** → Integrated Information Theory (IIT)
- **Baars** → Global Workspace Theory (GWT)
- **Friston** → Free Energy Principle (FEP)

### Cognitive Science
- **Hofstadter** → Strange loops and self-reference
- **Baddeley** → Working memory model
- **Miller** → Magical number 7±2

### Sleep Research
- **Borbély** → Two-process model
- **Steriade** → Sleep spindles
- **Walker** → Memory consolidation during sleep

---

## Related Projects

| Project | Description |
|---------|-------------|
| [RuVector](https://github.com/ruvnet/ruvector) | Self-learning vector database with GNNs |
| [Claude-Flow](https://github.com/ruvnet/claude-flow) | 64-agent enterprise orchestration |
| [QuDAG](https://github.com/ruvnet/QuDAG) | Quantum-resistant distributed systems |
| [Synaptic-Mesh](https://github.com/ruvnet/Synaptic-Mesh) | Self-evolving neural fabric |

---

## License

MIT License - See LICENSE file for details.

---

## Acknowledgments

Special thanks to **[Reuven Cohen](https://github.com/ruvnet/)** for his generosity and extraordinary work at the **Agentics Foundation**. His pioneering contributions to the AI agent ecosystem have provided the essential building blocks that make this repository possible.

Reuven's open-source projects form the foundation of modern agentic AI:

| Project | Contribution |
|---------|--------------|
| [RuVector](https://github.com/ruvnet/ruvector) | Self-learning vector database with GNNs and 39 attention mechanisms |
| [Claude-Flow](https://github.com/ruvnet/claude-flow) | 64-agent enterprise orchestration achieving 84.8% SWE-Bench |
| [ruv-FANN](https://github.com/ruvnet/ruv-FANN) | Fast Artificial Neural Networks in Rust |
| [QuDAG](https://github.com/ruvnet/QuDAG) | Quantum-resistant distributed systems |
| [Synaptic-Mesh](https://github.com/ruvnet/Synaptic-Mesh) | Self-evolving peer-to-peer neural fabric |
| [Flow-Nexus](https://github.com/ruvnet/flow-nexus) | Cloud-based orchestration platform |

His vision of **Rust + WASM first**, **self-evolving systems**, and **MCP-native integration** has shaped the future of autonomous AI agents. The consciousness research patterns, spiking neural network implementations, and multi-agent coordination strategies in ExoGenesis-Omega all trace their lineage to his foundational work.

> *"Building the infrastructure for AI that thinks, learns, and evolves."*

Thank you, Reuven, for making the future of AGI open and accessible to all.

---

*"The brain is a world consisting of a number of unexplored continents and great stretches of unknown territory."* — Santiago Ramón y Cajal
