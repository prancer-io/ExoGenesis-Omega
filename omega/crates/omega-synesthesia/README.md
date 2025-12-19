# omega-synesthesia: Transform Music into Walkable 3D Worlds

[![Crates.io](https://img.shields.io/crates/v/omega-synesthesia.svg)](https://crates.io/crates/omega-synesthesia)
[![Documentation](https://docs.rs/omega-synesthesia/badge.svg)](https://docs.rs/omega-synesthesia)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> **Listen to music and walk through it as a living, breathing world**

`omega-synesthesia` converts audio and music into immersive, explorable 3D environments. Different musical genres create distinct world aesthetics, with procedural generation driven by musical features like rhythm, harmony, timbre, and emotion.

## Overview

Synesthesia is the neurological phenomenon where stimulation of one sense triggers experiences in another (e.g., "seeing" colors when hearing sounds). `omega-synesthesia` computationally realizes this by transforming audio into fully-navigable 3D landscapes:

- **🎵 Classical** → Elegant marble halls with flowing fountains
- **🎸 Rock** → Volcanic terrain with crystal formations
- **🎹 Jazz** → Warm amber spaces with smooth undulations
- **🎧 Electronic** → Neon-lit geometric structures
- **🌊 Ambient** → Ethereal fog-shrouded expanses

## Features

- 🎨 **Genre-Specific Worlds** - Classical, jazz, rock, electronic, ambient styles
- 🎶 **Musical Feature Extraction** - Pitch, timbre, rhythm, emotion analysis
- 🌍 **Procedural Generation** - Chunk-based infinite worlds
- ⚡ **Beat-Reactive Lighting** - Lights pulse with music
- 🎭 **Emotion-Driven Materials** - Textures reflect musical mood
- 📐 **Advanced Geometry** - Procedural meshes, LOD, instancing
- 🎯 **Spatial Navigation** - Walk through time via omega-mindscape integration
- 📦 **GLTF Export** - Import into Unreal Engine, Unity, Blender
- 🎬 **Animation System** - Time-synchronized visual effects
- 🔊 **Multi-Format Audio** - WAV, MP3 (via feature flags)

## Theoretical Foundation

### Synesthesia Research
Based on chromesthesia (sound-to-color synesthesia) research:
- Consistent associations between pitch and color
- Timbre → texture mappings
- Rhythm → spatial patterns

### Music Information Retrieval (MIR)
Advanced audio analysis techniques:
- **FFT** - Spectral decomposition
- **Onset Detection** - Note beginning detection
- **Beat Tracking** - Tempo and rhythm extraction
- **Chroma** - Pitch class profiles
- **MFCC** - Timbre features

### Emotion Modeling
Russell's Circumplex Model:
- **Valence** (positive/negative) → color warmth
- **Arousal** (calm/excited) → saturation and brightness

### Procedural Content Generation
Techniques from:
- Minecraft (chunk-based worlds)
- No Man's Sky (procedural generation)
- Dreams (emotion-driven aesthetics)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
omega-synesthesia = "1.0"

# Optional: MP3 support (requires additional codec)
omega-synesthesia = { version = "1.0", features = ["mp3"] }
```

## Quick Start

### Basic Audio-to-World Pipeline

```rust
use omega_synesthesia::{SynesthesiaEngine, Genre, AudioSource};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create engine for classical music
    let mut engine = SynesthesiaEngine::new(Genre::Classical);

    // Load audio file
    engine.load_audio(AudioSource::File("beethoven_symphony.wav")).await?;

    // Generate the 3D world
    let world = engine.generate_world().await?;

    println!("World generated!");
    println!("  Chunks: {}", world.chunks.len());
    println!("  Geometries: {}", world.geometries.len());
    println!("  Lights: {}", world.lights.len());

    // Export to GLTF for visualization
    world.export_gltf("beethoven_world.gltf").await?;

    println!("Exported to beethoven_world.gltf");
    println!("Open in Blender, Unreal Engine, or Unity!");

    Ok(())
}
```

### Genre Comparison

```rust
use omega_synesthesia::{SynesthesiaEngine, Genre, AudioSource};

async fn compare_genres(audio_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let genres = vec![
        Genre::Classical,
        Genre::Jazz,
        Genre::Rock,
        Genre::Electronic,
        Genre::Ambient,
    ];

    for genre in genres {
        let mut engine = SynesthesiaEngine::new(genre);
        engine.load_audio(AudioSource::File(audio_path)).await?;
        let world = engine.generate_world().await?;

        let filename = format!("world_{:?}.gltf", genre);
        world.export_gltf(&filename).await?;

        println!("Generated {} world: {}", format!("{:?}", genre), filename);
    }

    Ok(())
}
```

### Navigate Through Musical Time

```rust
use omega_synesthesia::{SynesthesiaEngine, Genre, AudioSource};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = SynesthesiaEngine::new(Genre::Electronic);
    engine.load_audio(AudioSource::File("edm_track.wav")).await?;
    let world = engine.generate_world().await?;

    // Get integrated mindscape navigator
    let mindscape = engine.mindscape();

    // Navigate to 2:30 in the song (150 seconds)
    let coordinate = mindscape.navigate_to_time(150.0).await?;

    println!("Navigated to 2:30");
    println!("  Position: ({:.2}, {:.2}, {:.2})",
        coordinate.position.x,
        coordinate.position.y,
        coordinate.position.z
    );

    // Look around at nearby musical moments
    let nearby_moments = mindscape.look_around(10.0);

    for (label, distance) in nearby_moments {
        println!("  Nearby: '{}' at distance {:.2}", label, distance);
    }

    Ok(())
}
```

### Real-Time Visualization

```rust
use omega_synesthesia::{SynesthesiaEngine, Genre, AudioSource};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = SynesthesiaEngine::new(Genre::Ambient);
    engine.load_audio(AudioSource::File("ambient.wav")).await?;

    // Analyze audio
    let features = engine.analyze_features().await?;

    println!("Audio Analysis:");
    println!("  Duration: {:.2}s", features.duration_seconds);
    println!("  Tempo: {:.1} BPM", features.tempo);
    println!("  Key: {:?}", features.key);
    println!("  Avg Pitch: {:.1} Hz", features.average_pitch);

    // Generate world chunks on-demand
    for chunk_idx in 0..10 {
        let chunk = engine.generate_chunk(chunk_idx).await?;
        println!("Chunk {}: {} geometries", chunk_idx, chunk.geometries.len());

        // Stream to renderer...
    }

    Ok(())
}
```

## Musical Features → Visual Mappings

### Pitch
- **Low (bass)**: Deep valleys, dark colors
- **Mid**: Flat terrain, neutral colors
- **High (treble)**: Mountains, bright colors

### Timbre
- **Soft** (flute, strings): Smooth surfaces, gentle textures
- **Harsh** (distortion): Rough geometry, sharp edges
- **Metallic** (bells): Crystalline structures, reflective materials

### Rhythm
- **Slow** (largo): Large, spacious structures
- **Medium** (andante): Balanced geometry
- **Fast** (presto): Dense, intricate patterns

### Harmony
- **Consonant** (major chords): Symmetric, orderly structures
- **Dissonant** (diminished): Asymmetric, chaotic forms

### Emotion (Valence + Arousal)
- **Happy** (high valence, high arousal): Warm colors, energetic lighting
- **Sad** (low valence, low arousal): Cool colors, dim lighting
- **Angry** (low valence, high arousal): Red/orange, intense contrast
- **Calm** (high valence, low arousal): Soft pastels, gentle gradients

## Genre Aesthetics

### Classical
```
Environment: Marble halls, columns, fountains
Colors: Whites, golds, soft blues
Geometry: Symmetrical, geometric, elegant
Lighting: Soft ambient, gentle spotlights
Texture: Smooth, polished, pristine
```

### Jazz
```
Environment: Warm clubs, intimate spaces
Colors: Amber, bronze, deep browns
Geometry: Organic curves, smooth surfaces
Lighting: Moody, atmospheric, smoky
Texture: Velvet, wood, brass
```

### Rock
```
Environment: Volcanic terrain, crystal formations
Colors: Reds, oranges, blacks
Geometry: Sharp edges, angular forms
Lighting: Dynamic, high contrast
Texture: Rough, metallic, crystalline
```

### Electronic
```
Environment: Neon-lit geometric structures
Colors: Cyan, magenta, electric blue
Geometry: Precise, modular, grid-based
Lighting: Pulsing, synchronized to beats
Texture: Glossy, reflective, digital
```

### Ambient
```
Environment: Ethereal fog, floating islands
Colors: Soft pastels, faded tones
Geometry: Flowing, organic, minimal
Lighting: Diffuse, atmospheric, hazy
Texture: Soft, misty, translucent
```

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│              SYNESTHESIA ENGINE (Main Orchestrator)         │
└──────┬──────────┬──────────┬──────────┬──────────┬─────────┘
       │          │          │          │          │
       ▼          ▼          ▼          ▼          ▼
  ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ ┌─────────┐
  │ Audio  │ │Feature │ │Spatial │ │ World  │ │  GLTF   │
  │Analysis│ │Extract │ │Mapping │ │  Gen   │ │ Export  │
  └────────┘ └────────┘ └────────┘ └────────┘ └─────────┘
       │          │          │          │          │
       │          │          │          │          │
    ┌─┴──────────┴──────────┴──────────┴──────────┴─┐
    │                                                  │
    ▼                                                  ▼
┌────────────┐                                  ┌──────────┐
│   Audio    │                                  │  World   │
│  Features  │                                  │  Output  │
├────────────┤                                  ├──────────┤
│ - Onset    │                                  │ - Chunks │
│ - Beat     │                                  │ - Geom   │
│ - Chroma   │                                  │ - Lights │
│ - MFCC     │                                  │ - Mats   │
│ - Pitch    │                                  │ - Anims  │
│ - Timbre   │                                  │ - LODs   │
│ - Emotion  │                                  │ - GLTF   │
└────────────┘                                  └──────────┘
```

### Processing Pipeline

1. **Audio Loading** → WAV/MP3 decode
2. **Feature Extraction** → FFT, onset, beat, chroma, MFCC
3. **Musical Analysis** → Pitch, timbre, rhythm, harmony, emotion
4. **Spatial Mapping** → Features → 3D coordinates (via omega-mindscape)
5. **World Generation** → Procedural chunks, geometry, materials
6. **Enhancement** → Lights, textures, animations, LOD
7. **Export** → GLTF with complete scene graph

## Audio Format Support

### Built-In
- **WAV** - Uncompressed, full quality ✅

### Feature Flags
- **MP3** - Compressed, smaller files (requires `mp3` feature)
- **FLAC** - Lossless compression (requires `flac` feature)
- **OGG Vorbis** - Open compression (requires `ogg` feature)

```toml
[dependencies]
omega-synesthesia = { version = "1.0", features = ["mp3", "flac"] }
```

## GLTF Export Details

Generated GLTF files include:

- **Scene Graph** - Complete hierarchy of nodes
- **Meshes** - Procedural geometry with normals and UVs
- **Materials** - PBR materials with:
  - Base color (emotion-driven)
  - Metallic/roughness (timbre-based)
  - Emissive (beat-reactive)
- **Lights** - Point, directional, spot lights
- **Animations** - Time-synchronized visual effects
- **Extensions** - Custom metadata for musical features

### Import Workflow

**Blender:**
```
1. File → Import → glTF 2.0
2. Select generated .gltf file
3. Enjoy walkable music world!
```

**Unreal Engine:**
```
1. Content Browser → Import
2. Select .gltf file
3. Configure import settings
4. Add to scene
```

**Unity:**
```
1. Install glTFast package
2. Drag .gltf into Assets
3. Add prefab to scene
```

## Advanced Features

### LOD (Level of Detail)

Automatic LOD generation for performance:

```rust
// LOD0: High detail (near camera)
// LOD1: Medium detail
// LOD2: Low detail (far from camera)

world.configure_lod(
    LodConfig {
        distances: vec![10.0, 50.0, 200.0],
        reduction_factors: vec![1.0, 0.5, 0.2],
    }
).await?;
```

### Instancing

Efficient rendering via geometry instancing:

```rust
// Repeated geometric patterns use instancing
// e.g., Electronic genre: 1000s of cubes with different transforms
```

### Biome System

Musical terrain types:

- **Melodic Regions**: Prominent melody → distinct landmarks
- **Harmonic Zones**: Chord progressions → structured areas
- **Rhythmic Terrains**: Strong beats → repeating patterns
- **Ambient Expanses**: Background → subtle variations

### Navigation Integration (omega-mindscape)

```rust
// Mindscape provides spatial memory navigation
let mindscape = engine.mindscape();

// Navigate through musical time
mindscape.navigate_to_time(120.0).await?;

// Look around at similar musical moments
let nearby = mindscape.look_around(5.0);

// Enter dream state to discover musical connections
mindscape.enter_dream_state().await?;
let discoveries = mindscape.dream_explore(60.0).await?;
```

## Use Cases

### 1. Music Visualization

Create stunning visuals for:
- Concerts and live performances
- Music videos
- VR music experiences
- Educational music theory tools

### 2. Generative Art

Procedural art driven by music:
- Gallery installations
- NFT collections
- Algorithmic compositions

### 3. Game Level Design

Use music to generate game levels:
- Rhythm-based platformers
- Audio-reactive puzzle games
- Procedural exploration games

### 4. Therapeutic Applications

Spatial music for therapy:
- Music therapy visualization
- Guided meditation environments
- Sensory integration tools

### 5. Educational Tools

Teach music concepts spatially:
- Visualize harmony and melody
- Understand musical structure
- Explore compositional techniques

## Performance

- **Audio Analysis**: ~100ms per 60s of audio
- **World Generation**: ~500ms per chunk (depends on complexity)
- **GLTF Export**: ~200ms + file write time
- **Memory**: ~100MB for 5-minute song world
- **LOD Impact**: 2-10x performance improvement

## Examples

The crate includes 4 comprehensive examples:

### 1. synesthesia_demo.rs
Basic audio-to-world pipeline:
```bash
cargo run --example synesthesia_demo
```

### 2. synesthesia_simulation.rs
Full simulation with multiple genres:
```bash
cargo run --example synesthesia_simulation
```

### 3. synesthesia_visualizer.rs
Real-time visualization:
```bash
cargo run --example synesthesia_visualizer
```

### 4. dream_3d_walkthrough.rs
Integrated with omega-mindscape for dream exploration:
```bash
cargo run --example dream_3d_walkthrough
```

## API Reference

### Main Types

```rust
pub struct SynesthesiaEngine { ... }
pub struct SynesthesiaWorld { ... }
pub enum Genre { Classical, Jazz, Rock, Electronic, Ambient }
pub enum AudioSource { File(String), Stream(Vec<u8>) }
pub struct MusicalFeatures { ... }
pub struct WorldChunk { ... }
pub struct GeometryData { ... }
pub struct MaterialData { ... }
pub struct LightData { ... }
pub struct AnimationData { ... }
```

### Main Methods

```rust
impl SynesthesiaEngine {
    pub fn new(genre: Genre) -> Self;
    pub async fn load_audio(&mut self, source: AudioSource) -> Result<()>;
    pub async fn analyze_features(&self) -> Result<MusicalFeatures>;
    pub async fn generate_world(&mut self) -> Result<SynesthesiaWorld>;
    pub async fn generate_chunk(&mut self, index: usize) -> Result<WorldChunk>;
    pub fn mindscape(&self) -> &MindscapeExplorer;
}

impl SynesthesiaWorld {
    pub async fn export_gltf(&self, path: &str) -> Result<()>;
    pub fn get_chunk(&self, index: usize) -> Option<&WorldChunk>;
    pub fn total_geometry_count(&self) -> usize;
    pub fn total_light_count(&self) -> usize;
}
```

## Integration with Other Omega Crates

### omega-mindscape

Navigate through musical memory:
```rust
let mindscape = engine.mindscape();
mindscape.navigate_to_time(timestamp).await?;
```

### omega-brain

Process music cognitively:
```rust
// Feed musical features to cognitive loops
brain.process(&musical_embedding).await?;
```

### omega-consciousness

Measure consciousness of musical regions:
```rust
// Use IIT to find "conscious" musical moments
let phi = mindscape.measure_consciousness().await?;
```

## Limitations

- **Audio Length**: Worlds grow linearly with audio duration (~1 chunk/10s)
- **Format Support**: WAV built-in, others require features
- **Real-Time**: Not suitable for live audio (yet)
- **Determinism**: Same audio + genre → same world
- **Single Channel**: Mono and stereo supported, no surround

## Future Enhancements

- [ ] Real-time audio input (microphone, streaming)
- [ ] VR/AR support (WebXR export)
- [ ] Multi-channel audio (5.1, 7.1 surround)
- [ ] Machine learning genre classification
- [ ] Collaborative multi-user worlds
- [ ] MIDI input (not just audio)
- [ ] Video synchronization
- [ ] Physics simulation (walkable terrain)

## Contributing

Contributions welcome! This is a novel application of synesthesia research.

Areas for contribution:
- New genre styles
- Additional audio formats
- Performance optimizations
- VR/AR export formats
- Real-time streaming
- ML-based feature extraction

## License

MIT License - see LICENSE file

## Citations & References

- Cytowic, R. E. (2002). *Synesthesia: A Union of the Senses*. MIT Press.
- Müller, M. (2015). *Fundamentals of Music Processing*. Springer.
- Russell, J. A. (1980). *A circumplex model of affect*. Journal of Personality and Social Psychology.
- Goto, M. (2001). *An audio-based real-time beat tracking system*. ICASSP.
- Logan, B. (2000). *Mel Frequency Cepstral Coefficients for Music Modeling*. ISMIR.

## Related Crates

- [omega-mindscape](../omega-mindscape) - Navigate memories spatially
- [omega-brain](../omega-brain) - Unified cognitive architecture
- [omega-consciousness](../omega-consciousness) - Consciousness measurement
- [omega-sleep](../omega-sleep) - REM dream exploration

## See Also

- [omega-examples](../omega-examples) - Complete demonstrations
- [GLTF Specification](https://www.khronos.org/gltf/) - glTF 2.0 format
- [Music Information Retrieval](https://www.ismir.net/) - Academic conference

---

**Built with ❤️ for musical worlds**

*Listen with your eyes. See with your ears.*
