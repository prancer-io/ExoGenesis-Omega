# Digital Twin Social Media Platform

> An ExoGenesis Omega example demonstrating how to build the world's most advanced human graph powered by multi-agent AI.

## Overview

This example showcases how ExoGenesis Omega's components can power a next-generation social platform with:

- **Digital Twins**: 4096-dimensional personality modeling
- **Emotional AI**: 7 temporal loops processing emotions from milliseconds to lifetime
- **Intelligent Matching**: Causal reasoning for relationship prediction
- **ARIA Presence**: Multi-agent orchestration for coherent AI companionship
- **Zero-Knowledge Privacy**: See everything, store nothing

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         DIGITAL TWIN PLATFORM                               │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    USER TOUCHPOINTS                                  │   │
│  │  ┌───────────┐  ┌───────────┐  ┌───────────┐  ┌───────────┐        │   │
│  │  │ Mobile App│  │ Keyboard  │  │ Wearables │  │  Web App  │        │   │
│  │  └─────┬─────┘  └─────┬─────┘  └─────┬─────┘  └─────┬─────┘        │   │
│  └────────┼──────────────┼──────────────┼──────────────┼────────────────┘   │
│           └──────────────┴──────────────┴──────────────┘                     │
│                                    │                                         │
│  ┌─────────────────────────────────▼───────────────────────────────────┐   │
│  │              ZERO-KNOWLEDGE EMOTIONAL LAYER                          │   │
│  │  Client-Side Encryption → Differential Privacy → Safe Export         │   │
│  └─────────────────────────────────┬───────────────────────────────────┘   │
│                                    │                                         │
│  ┌─────────────────────────────────▼───────────────────────────────────┐   │
│  │                    OMEGA BRAIN (7 Temporal Loops)                    │   │
│  │  Loop 1: Reflexive (~10ms)    │  Loop 5: Growth (~months)           │   │
│  │  Loop 2: Mood (~5min)         │  Loop 6: Life Phase (~years)        │   │
│  │  Loop 3: Daily (~24hr)        │  Loop 7: Identity (~lifetime)       │   │
│  │  Loop 4: Traits (~weeks)      │                                      │   │
│  └─────────────────────────────────┬───────────────────────────────────┘   │
│                                    │                                         │
│  ┌─────────────────────────────────▼───────────────────────────────────┐   │
│  │                    RUVECTOR / AGENTDB                                │   │
│  │  • SIMD-accelerated (41x faster)  • Reflexion learning              │   │
│  │  • 4096-dim personality vectors   • Causal relationship graph       │   │
│  │  • HNSW index (<1ms search)       • Skill library                   │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Quick Start

```bash
# From the omega directory
cd examples/digital-twin-social

# Run the demo
cargo run --release
```

## Components Demonstrated

### 1. Personality Engine (`personality.rs`)

Uses omega-agentdb for SIMD-accelerated personality vector operations:

```rust
use digital_twin_social::personality::PersonalityEngine;

let engine = PersonalityEngine::new().await?;

// Register a digital twin
let mut twin = DigitalTwin::new("Alice");
twin.big_five = BigFive::new(0.9, 0.7, 0.6, 0.85, 0.2);
let id = engine.register_profile(twin).await?;

// Find similar profiles (<1ms for millions)
let similar = engine.find_similar(&id, 10).await?;
```

### 2. Emotional Loops (`emotional.rs`)

Implements Omega Brain's 7 temporal loops for emotional processing:

```rust
use digital_twin_social::emotional::EmotionalLoopProcessor;

let processor = EmotionalLoopProcessor::new();

// Add emotional signal
processor.add_signal(signal).await;

// Process through loops
let reflexive = processor.process_reflexive().await;  // ~10ms
let mood = processor.process_mood().await;            // ~5min aggregation
let traits = processor.process_traits().await;       // Stable patterns
let growth = processor.process_growth().await;       // Long-term trajectory
```

### 3. Matching Engine (`matching.rs`)

Multi-domain relationship matching with causal prediction:

```rust
use digital_twin_social::matching::MatchingEngine;

let matching = MatchingEngine::new(personality_engine).await?;

// Find matches across domains
let dating_matches = matching.find_matches(&user_id, ConnectionDomain::Dating, 10).await?;
let work_matches = matching.find_matches(&user_id, ConnectionDomain::Professional, 10).await?;

// Each match includes:
// - Compatibility score
// - Predicted satisfaction
// - Predicted longevity
// - Growth potential
// - Conflict risk
```

### 4. ARIA Swarm (`aria.rs`)

Multi-agent orchestration for coherent AI presence:

```rust
use digital_twin_social::aria::ARIASwarm;

let aria = ARIASwarm::new().await?;

// Process message through all agents
let response = aria.process_message(&user_id, "I'm feeling stressed", None).await?;

// Response synthesized from:
// - Empathy Agent (emotional validation)
// - Growth Coach (development opportunities)
// - Relationship Advisor (social insights)
// - Values Guardian (alignment checking)
// - Wellness Agent (biometric integration)
```

### 5. Zero-Knowledge Privacy (`privacy.rs`)

Privacy-preserving emotional AI:

```rust
use digital_twin_social::privacy::ZeroKnowledgeLayer;

let mut layer = ZeroKnowledgeLayer::new();

// Raw data stays on device
layer.process_raw_data(emotional_reading);

// Only safe exports leave device
let export = layer.generate_safe_export();
// Contains: personality_delta, growth_signals, stability_score
// Does NOT contain: raw emotions, private reflections, context
```

### 6. Sensor Integration (`sensors.rs`)

Keyboard and wearable emotional inference:

```rust
use digital_twin_social::sensors::{KeyboardSensor, WearableSensor};

// Keyboard emotional inference from typing patterns
let mut keyboard = KeyboardSensor::new();
keyboard.add_keystroke(key_down, key_up, category, pressure);
let emotion = keyboard.infer_emotion()?;

// Wearable inference from biometrics
let mut wearable = WearableSensor::new();
wearable.add_biometric_data(hrv, heart_rate, temp);
let emotion = wearable.infer_emotion()?;
```

## Type System

### Personality Modeling

```rust
// Big Five (OCEAN) personality traits
pub struct BigFive {
    pub openness: f32,
    pub conscientiousness: f32,
    pub extraversion: f32,
    pub agreeableness: f32,
    pub neuroticism: f32,
}

// Schwartz Values (10 basic human values)
pub struct SchwartzValues {
    pub self_direction: f32,
    pub stimulation: f32,
    // ... 8 more
}

// Emotional Intelligence
pub struct EmotionalIntelligence {
    pub self_awareness: f32,
    pub self_regulation: f32,
    pub motivation: f32,
    pub empathy: f32,
    pub social_skills: f32,
}

// Complete Digital Twin (4096-dim embedding)
pub struct DigitalTwin {
    pub big_five: BigFive,
    pub values: SchwartzValues,
    pub attachment_style: AttachmentStyle,
    pub eq: EmotionalIntelligence,
    pub communication_style: CommunicationStyle,
    pub deep_embedding: Vec<f32>,  // 4096 dimensions
}
```

### Emotional States

```rust
pub struct EmotionalState {
    pub primary: CoreEmotion,
    pub primary_intensity: f32,
    pub valence: f32,      // -1.0 to 1.0
    pub arousal: f32,      // 0.0 to 1.0
    pub dominance: f32,    // 0.0 to 1.0
}

pub enum CoreEmotion {
    Joy, Trust, Fear, Surprise,
    Sadness, Disgust, Anger, Anticipation,
}
```

## Performance

| Operation | Performance |
|-----------|-------------|
| Personality vector search | <1ms for millions (SIMD) |
| Emotional loop processing | <10ms (reflexive loop) |
| Match scoring | ~5ms per candidate |
| ARIA response synthesis | ~50ms (5 agents) |
| Privacy export generation | <1ms |

## Use Cases

This example demonstrates building:

1. **Dating Apps** - Deep compatibility beyond surface traits
2. **Friend Matching** - Shared values and complementary personalities
3. **Team Building** - Communication style and conflict pattern analysis
4. **Mentorship** - Growth potential and EQ compatibility
5. **Community Building** - Value-aligned group formation

## Integration with Path

This architecture directly supports Path's vision:

| Path Feature | Omega Component |
|--------------|-----------------|
| Human Graph | 4096-dim personality vectors in AgentDB |
| Always-on AI | 7 temporal loops + sensor fusion |
| Keyboard inference | KeyboardSensor with privacy layer |
| Wearable integration | WearableSensor with biometric analysis |
| ARIA presence | ARIASwarm multi-agent orchestration |
| Zero-knowledge | ZeroKnowledgeLayer with differential privacy |

## License

MIT License - Part of ExoGenesis Omega

## Learn More

- [omega-agentdb documentation](../../docs/crate-guides/omega-agentdb.md)
- [omega-loops documentation](../../docs/crate-guides/omega-loops.md)
- [omega-memory documentation](../../docs/crate-guides/omega-memory.md)
- [Master Architecture](../../design-docs/architecture/00-master-architecture.md)
