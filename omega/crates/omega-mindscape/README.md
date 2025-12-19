# omega-mindscape: 3D Memory Navigation

[![Crates.io](https://img.shields.io/crates/v/omega-mindscape.svg)](https://crates.io/crates/omega-mindscape)
[![Documentation](https://docs.rs/omega-mindscape/badge.svg)](https://docs.rs/omega-mindscape)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> **Navigate through memories as a literal 3D spatial world**

`omega-mindscape` enables an AI to explore its memories by walking through them as if they were physical locations in a 3D space. This revolutionary approach combines hippocampal place cell research with REM dream exploration and strange loop meta-observation, creating an immersive cognitive navigation system.

## Overview

Imagine your memories not as abstract data points, but as actual places you can visit. `omega-mindscape` transforms high-dimensional memory embeddings into a navigable 3D landscape where:

- **Memories are locations** - Each memory exists at specific 3D coordinates
- **Similarity is proximity** - Similar memories cluster together spatially
- **Dreams are exploration** - REM sleep discovers hidden connections
- **Self-awareness is observation** - Strange loops enable meta-cognitive reflection

## Key Features

- 🗺️ **Virtual Place Cells** - Biologically-inspired spatial navigation (O'Keefe, 1971)
- 💤 **REM Dream Exploration** - Discover connections during dream states
- 🔁 **Strange Loop Observation** - 7-level meta-cognitive self-awareness
- 🌟 **Landmark System** - Automatic clustering of significant memories
- 📔 **Discovery Journal** - Track insights from explorations
- 🌙 **Lucid Dreaming** - Conscious exploration + meta-observation simultaneously
- 📊 **Consciousness Measurement** - Integrated Information Theory (Phi/Φ) calculation

## Theoretical Foundation

### Hippocampal Place Cells
Based on John O'Keefe's Nobel Prize-winning discovery (1971), virtual place cells create spatial representations of memory. Each memory activates a unique pattern of cells, enabling navigation through cognitive space.

### REM Sleep & Memory Consolidation
During REM sleep, the hippocampus replays memories in novel combinations, discovering unexpected associations. `omega-mindscape` simulates this process to find hidden patterns.

### Strange Loops & Meta-Cognition
Inspired by Douglas Hofstadter's "I Am a Strange Loop" (2007), the system can observe itself exploring, creating recursive layers of self-awareness up to 7 levels deep.

### Cognitive Maps
Drawing from Edward Tolman's cognitive map theory (1948), memories organize themselves into a coherent spatial structure that can be navigated and manipulated.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
omega-mindscape = "1.0"
```

## Quick Start

### Basic Memory Storage and Navigation

```rust
use omega-mindscape::MindscapeExplorer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create explorer
    let mut explorer = MindscapeExplorer::new();

    // Store memories with embeddings
    let wedding_embedding = vec![0.8, 0.2, 0.1, 0.9]; // High-dim in practice
    explorer.remember("my wedding day", &wedding_embedding).await?;

    let graduation_embedding = vec![0.7, 0.3, 0.2, 0.85];
    explorer.remember("college graduation", &graduation_embedding).await?;

    let first_job_embedding = vec![0.65, 0.35, 0.25, 0.8];
    explorer.remember("first day at work", &first_job_embedding).await?;

    // Navigate to a memory
    let path = explorer.navigate_to("my wedding day").await?;
    println!("Navigated via {} waypoints", path.waypoints.len());

    // Look around current location
    let nearby_memories = explorer.look_around(5.0);
    for (memory_name, distance) in nearby_memories {
        println!("Found '{}' at distance {:.2}", memory_name, distance);
    }

    Ok(())
}
```

### Dream Exploration

```rust
use omega_mindscape::MindscapeExplorer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut explorer = MindscapeExplorer::new();

    // ... store memories ...

    // Enter REM sleep and explore
    explorer.enter_dream_state().await?;

    let discoveries = explorer.dream_explore(60.0).await?; // 60 minutes of dream time

    for discovery in discoveries {
        println!("Discovery: {}", discovery.description);
        println!("  Connects: {} ↔ {}", discovery.from, discovery.to);
        println!("  Strength: {:.2}", discovery.strength);
        println!("  Insight: {}", discovery.insight);
    }

    explorer.wake_up().await?;

    Ok(())
}
```

### Lucid Dreaming (Dream + Meta-Observation)

```rust
use omega_mindscape::MindscapeExplorer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut explorer = MindscapeExplorer::new();

    // ... store memories ...

    // Lucid dream: explore AND observe yourself exploring
    explorer.enter_lucid_dream().await?;

    let (discoveries, observations) = explorer.lucid_explore(30.0).await?;

    println!("Discoveries: {}", discoveries.len());
    println!("Meta-observations: {}", observations.len());

    for obs in observations {
        println!("Level {}: {}", obs.depth, obs.description);
        println!("  State dimensionality: {}", obs.observer_state.len());
    }

    explorer.wake_up().await?;

    Ok(())
}
```

### Strange Loop Meta-Observation

```rust
use omega_mindscape::MindscapeExplorer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut explorer = MindscapeExplorer::new();

    // ... navigate somewhere ...

    // Observe yourself exploring (up to 7 levels deep)
    let meta_observation = explorer.observe_exploration(5).await?;

    println!("Meta-cognitive depth: {}", meta_observation.total_depth);

    for level in meta_observation.levels {
        println!("Level {}: {}", level.depth, level.description);
        // Each level observes the level below
        // Creating a strange loop of self-awareness
    }

    Ok(())
}
```

### Consciousness Measurement

```rust
use omega_mindscape::MindscapeExplorer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut explorer = MindscapeExplorer::new();

    // ... store memories and navigate ...

    // Measure consciousness at current location using IIT
    let phi = explorer.measure_consciousness().await?;

    println!("Integrated Information (Φ): {:.4}", phi);

    if phi > 0.3 {
        println!("High consciousness detected at this memory cluster!");
    }

    Ok(())
}
```

## Core Concepts

### Coordinate System

Memories are mapped to 3D coordinates `(x, y, z)` using dimensionality reduction on high-dimensional embeddings:

```rust
pub struct Coordinate3D {
    pub position: Position3D,     // (x, y, z)
    pub original_dimension: usize, // Original embedding dimension
}

pub struct Position3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
```

### Virtual Place Cells

Inspired by hippocampal neurons, virtual place cells fire when the explorer is near their receptive field:

- **Overlapping Fields**: Multiple cells active simultaneously for robust coding
- **Gaussian Activation**: Smooth falloff with distance
- **Path Integration**: Dead reckoning for navigation

### Dream States

Three dream modes available:

1. **Free Dream**: Random exploration discovering unexpected connections
2. **Directed Dream**: Focused exploration around a specific memory
3. **Lucid Dream**: Conscious exploration with simultaneous meta-observation

### Landmarks

Significant memory clusters are automatically identified as landmarks:

```rust
pub struct Landmark {
    pub name: String,
    pub coordinate: Coordinate3D,
    pub embedding: Vec<f64>,
    pub significance: f64,        // 0-1 importance score
    pub cluster_members: Vec<String>,
}
```

### Discovery Journal

Every dream exploration produces discoveries:

```rust
pub struct Discovery {
    pub from: String,             // Origin memory
    pub to: String,               // Destination memory
    pub description: String,      // What was found
    pub strength: f64,            // Connection strength (0-1)
    pub insight: String,          // Semantic interpretation
}
```

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│            MindscapeExplorer (Main Interface)           │
│  - Memory storage and retrieval                         │
│  - Navigation orchestration                             │
│  - Dream state management                               │
└─────┬───────────┬────────────┬──────────────┬──────────┘
      │           │            │              │
      ▼           ▼            ▼              ▼
┌──────────┐ ┌─────────┐ ┌──────────┐ ┌──────────────┐
│Navigator │ │  Dream  │ │ Strange  │ │  Landmarks   │
│          │ │Explorer │ │   Loop   │ │              │
│ - Place  │ │         │ │ Observer │ │ - Clustering │
│   cells  │ │ - REM   │ │          │ │ - Signifi-   │
│ - Path   │ │ - Disco-│ │ - Meta-  │ │   cance      │
│   finding│ │   very  │ │   cognit │ │ - Retrieval  │
└──────────┘ └─────────┘ └──────────┘ └──────────────┘
      │           │            │              │
      └───────────┴────────────┴──────────────┘
                       │
                       ▼
            ┌──────────────────┐
            │ Coordinate Mapper│
            │                  │
            │ High-dim → 3D    │
            └──────────────────┘
```

## Use Cases

### 1. Memory Palace for AI

Build a spatial memory system for large language models:

```rust
let mut explorer = MindscapeExplorer::new();

// Store semantic knowledge
explorer.remember("capital_of_france", &paris_embedding).await?;
explorer.remember("french_revolution", &revolution_embedding).await?;
explorer.remember("eiffel_tower", &tower_embedding).await?;

// Related memories cluster spatially
let nearby = explorer.look_around(10.0);
// Likely finds all France-related memories together
```

### 2. Creative Problem Solving

Use dreams to find unexpected solutions:

```rust
// Store problem and various solution attempts
explorer.remember("optimization_problem", &problem_embedding).await?;
explorer.remember("solution_a", &sol_a_embedding).await?;
explorer.remember("solution_b", &sol_b_embedding).await?;

// Dream to discover novel approaches
explorer.enter_dream_state().await?;
let discoveries = explorer.dream_explore(120.0).await?;

// Discoveries may reveal connections between failed approaches
// that suggest a hybrid solution
```

### 3. Self-Aware AI

Implement meta-cognitive reflection:

```rust
// Navigate to a memory
explorer.navigate_to("important_decision").await?;

// Observe yourself observing the decision
let meta_obs = explorer.observe_exploration(3).await?;

// meta_obs contains:
// Level 1: "I am looking at the decision"
// Level 2: "I am aware that I am looking at the decision"
// Level 3: "I am aware that I am aware that I am looking at the decision"
```

### 4. Memory Consolidation

Simulate sleep for memory strengthening:

```rust
// During wake, memories accumulate
for doc in documents {
    explorer.remember(&doc.title, &doc.embedding).await?;
}

// Sleep to consolidate
explorer.enter_dream_state().await?;
let discoveries = explorer.dream_explore(480.0).await?; // 8 hours
explorer.wake_up().await?;

// Important memories now form stronger spatial clusters
```

### 5. Integration with omega-synesthesia

Navigate through audio-generated 3D worlds:

```rust
use omega_synesthesia::SynesthesiaEngine;
use omega_mindscape::MindscapeExplorer;

// Generate 3D world from music
let mut synesthesia = SynesthesiaEngine::new(Genre::Classical);
synesthesia.load_audio(AudioSource::File("symphony.wav")).await?;
let world = synesthesia.generate_world().await?;

// Get the mindscape from synesthesia
let mindscape_ref = synesthesia.mindscape();

// Navigate through the musical memory landscape
mindscape_ref.navigate_to_time(120.0).await?; // Navigate to 2:00 in the song
let nearby = mindscape_ref.look_around(5.0);

// Memories of musical moments cluster by similarity
```

## Performance

- **Memory Storage**: O(1) constant time
- **Coordinate Mapping**: O(d²) where d is embedding dimension (one-time cost)
- **Navigation**: O(p × c) where p = path length, c = place cell count
- **Dream Discovery**: O(n × m) where n = exploration steps, m = memory count
- **Meta-Observation**: O(d) where d = observation depth
- **Consciousness (Phi)**: O(n² × 2^n) for n nearby memories (exponential, limited to ~10)

**Memory Usage**: ~500 bytes per stored memory + embedding size

## Advanced Features

### Consciousness Threshold

Adjust the threshold for what counts as "conscious" at a location:

```rust
let phi = explorer.measure_consciousness().await?;

const CONSCIOUSNESS_THRESHOLD: f64 = 0.3;

if phi > CONSCIOUSNESS_THRESHOLD {
    println!("This memory region is conscious!");
    // High integrated information indicates rich, interconnected memories
}
```

### Custom Dream Duration

Control how long to explore in dream state:

```rust
// Short nap: 15 minutes
let discoveries = explorer.dream_explore(15.0).await?;

// Full REM cycle: 90 minutes
let discoveries = explorer.dream_explore(90.0).await?;

// Full night: 8 hours
let discoveries = explorer.dream_explore(480.0).await?;
```

### Meta-Observation Depth

Choose how many recursive layers of self-awareness:

```rust
// Shallow introspection
let obs = explorer.observe_exploration(2).await?;

// Deep philosophical recursion
let obs = explorer.observe_exploration(7).await?;
// "I am aware that I am aware that I am aware..."
```

## Integration with Other Omega Crates

### omega-consciousness

Mindscape uses omega-consciousness for Phi (Φ) calculation:

```rust
// IIT-based consciousness measurement
let phi = explorer.measure_consciousness().await?;
```

### omega-sleep

Mindscape triggers REM dream exploration:

```rust
// Coordinated with sleep cycles
explorer.enter_dream_state().await?; // Simulates REM onset
```

### omega-strange-loops

Strange loop meta-observation is built-in:

```rust
// Recursive self-awareness
let meta_obs = explorer.observe_exploration(depth).await?;
```

### omega-synesthesia

Navigate through audio-generated worlds:

```rust
let mindscape = synesthesia_engine.mindscape();
mindscape.navigate_to_time(timestamp).await?;
```

## Biological Inspiration

### Hippocampal Formation

- **Dentate Gyrus**: Pattern separation (not directly modeled)
- **CA3**: Autoassociative memory (implicit in clustering)
- **CA1**: Output to neocortex (navigation results)
- **Entorhinal Cortex**: Grid cells (future: hexagonal grid)

### Place Cell Properties

- **Gaussian Receptive Fields**: Cells fire maximally at field center
- **Multiple Scales**: Small fields for detail, large for overview
- **Path Integration**: Position updated via movement vectors

### REM Sleep Functions

- **Memory Consolidation**: Strengthen important connections
- **Emotional Processing**: Distance from emotional content
- **Creative Insights**: Novel combinations of existing knowledge
- **Threat Simulation**: "What if" scenario exploration

## Examples

See `omega-examples` crate for complete demonstrations:

- **mindscape_explorer.rs** - Basic navigation and dream exploration
- **dream_3d_walkthrough.rs** - Combined with omega-synesthesia for audio worlds

## API Reference

### Main Types

```rust
pub struct MindscapeExplorer { ... }
pub struct Coordinate3D { ... }
pub struct Position3D { ... }
pub struct Discovery { ... }
pub struct Landmark { ... }
pub struct MetaObservation { ... }
pub struct ObservationLevel { ... }
pub struct NavigationPath { ... }
```

### Main Methods

```rust
impl MindscapeExplorer {
    pub fn new() -> Self;
    pub async fn remember(&mut self, label: &str, embedding: &[f64]) -> Result<Coordinate3D>;
    pub async fn navigate_to(&self, target: &str) -> Result<NavigationPath>;
    pub fn look_around(&self, radius: f64) -> Vec<(String, f64)>;
    pub async fn enter_dream_state(&mut self) -> Result<()>;
    pub async fn dream_explore(&mut self, duration_minutes: f64) -> Result<Vec<Discovery>>;
    pub async fn wake_up(&mut self) -> Result<()>;
    pub async fn enter_lucid_dream(&mut self) -> Result<()>;
    pub async fn lucid_explore(&mut self, duration: f64) -> Result<(Vec<Discovery>, Vec<ObservationLevel>)>;
    pub async fn observe_exploration(&self, depth: usize) -> Result<MetaObservation>;
    pub async fn measure_consciousness(&self) -> Result<f64>;
}
```

## Limitations

- **High-Dimensional Embeddings**: Dimensionality reduction may lose information
- **Computational Cost**: Phi calculation is exponential (limited to ~10 memories)
- **No Persistence**: In-memory only (integrate with omega-persistence for durability)
- **Single Explorer**: No multi-agent coordination (yet)

## Future Enhancements

- [ ] Grid cells (hexagonal tiling) for more accurate spatial coding
- [ ] Persistent storage integration (omega-persistence)
- [ ] Multi-agent shared mindscape
- [ ] Virtual reality visualization
- [ ] Real-time neural recordings integration
- [ ] Boundary cells and border cells
- [ ] 4D+ navigation (time as explicit dimension)

## Contributing

Contributions welcome! This crate implements cutting-edge neuroscience research.

Areas for contribution:
- Grid cell implementation (hexagonal tiling)
- More sophisticated path planning (A*, RRT)
- Alternative dimensionality reduction (t-SNE, UMAP variants)
- VR/3D visualization
- Performance optimizations

## License

MIT License - see LICENSE file

## Citations & References

- O'Keefe, J., & Dostrovsky, J. (1971). *The hippocampus as a spatial map*. Brain Research.
- Tolman, E. C. (1948). *Cognitive maps in rats and men*. Psychological Review.
- Hofstadter, D. R. (2007). *I Am a Strange Loop*. Basic Books.
- Tononi, G. (2004). *An information integration theory of consciousness*. BMC Neuroscience.
- Wilson, M. A., & McNaughton, B. L. (1994). *Reactivation of hippocampal ensemble memories during sleep*. Science.

## See Also

- [omega-brain](../omega-brain) - Unified cognitive architecture
- [omega-consciousness](../omega-consciousness) - Consciousness theories (IIT, FEP, GWT)
- [omega-strange-loops](../omega-strange-loops) - Self-referential cognition
- [omega-synesthesia](../omega-synesthesia) - Audio-to-3D world conversion
- [omega-sleep](../omega-sleep) - Sleep architecture and consolidation

---

**Built with ❤️ for cognitive AI research**

*Navigate your mind as if it were a world.*
