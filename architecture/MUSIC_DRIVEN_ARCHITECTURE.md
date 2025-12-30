# SYNESTHESIA: Music-Driven Architecture

## Core Principle

> **Music IS the story. Lyrics are optional commentary.**

An instrumental piece can make you cry, pump your fist, or feel transcendence. The visualization must respond to the MUSIC, not just words on top of it.

## The Three Layers of Musical Understanding

```
┌─────────────────────────────────────────────────────────────────────┐
│                                                                     │
│  LAYER 3: NARRATIVE                                                 │
│  ┌───────────────────────────────────────────────────────────────┐ │
│  │ Emotional Arc • Story Structure • Climax Map • Scene Changes  │ │
│  └───────────────────────────────────────────────────────────────┘ │
│                              ▲                                      │
│  LAYER 2: MUSICAL THEORY     │                                      │
│  ┌───────────────────────────┴───────────────────────────────────┐ │
│  │ Key • Chords • Tempo • Structure • Timbre • Tension/Release   │ │
│  └───────────────────────────────────────────────────────────────┘ │
│                              ▲                                      │
│  LAYER 1: SIGNAL             │                                      │
│  ┌───────────────────────────┴───────────────────────────────────┐ │
│  │ FFT • Beats • Onsets • Energy • Spectral Features             │ │
│  └───────────────────────────────────────────────────────────────┘ │
│                              ▲                                      │
│                         RAW AUDIO                                   │
└─────────────────────────────────────────────────────────────────────┘
```

## Layer 1: Signal Analysis (Real-Time, <10ms)

These are computed every audio frame:

| Feature | Description | Visual Application |
|---------|-------------|-------------------|
| `spectrum[512]` | FFT frequency bins | Direct shape modulation |
| `rms` | Overall loudness | Global brightness/scale |
| `bass/mid/high` | Frequency band energy | Color channel intensity |
| `is_beat` | Beat detected | Trigger effects |
| `beat_strength` | How strong the beat | Effect intensity |
| `onset` | New sound started | Spawn particles |
| `spectral_centroid` | Brightness of sound | Color temperature |
| `spectral_flux` | Rate of change | Motion speed |
| `zero_crossing_rate` | Noisiness | Texture roughness |

## Layer 2: Musical Theory (Real-Time, 50-500ms window)

Higher-level musical understanding:

| Feature | Description | Visual Application |
|---------|-------------|-------------------|
| `key` | Musical key (C major, A minor...) | Color palette selection |
| `mode` | Major/Minor/Modal | Warm vs cool, open vs tense |
| `chord` | Current chord | Harmonic color relationships |
| `chord_tension` | Dissonance level (0-1) | Visual distortion/clarity |
| `tempo` | BPM | Animation speed baseline |
| `tempo_derivative` | Speeding up/slowing down | Camera movement |
| `time_signature` | 4/4, 3/4, 6/8... | Pattern symmetry |
| `timbre_brightness` | Bright vs dark sound | Lighting warmth |
| `timbre_roughness` | Smooth vs harsh | Surface texture |

## Layer 3: Structural Narrative (Analyzed per-section)

The story of the song:

| Feature | Description | Visual Application |
|---------|-------------|-------------------|
| `section` | Intro/Verse/Chorus/Bridge/Drop | Scene type |
| `section_energy` | Energy level of section | Visual complexity |
| `section_position` | Progress through section | Revelation progress |
| `approaching_transition` | Change coming soon | Anticipation effects |
| `climax_distance` | How far from peak | Build intensity |
| `repetition_count` | Times we've heard this | Familiarity/variation |
| `energy_arc_position` | Overall song journey | Global progression |
| `narrative_beat` | Rising/Climax/Falling | Scene dramatics |

## The Emotional Map

Music theory → Emotion → Visual language:

```
┌─────────────────────────────────────────────────────────────────┐
│                    MUSIC → EMOTION → VISUAL                     │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  MAJOR KEY + Fast Tempo + High Energy                           │
│  ────────────────────────────────────────                       │
│  Emotion: Joy, Triumph, Celebration                             │
│  Visual:  Bright yellows/oranges, upward motion, expansion,     │
│           particles exploding outward, warm lighting            │
│                                                                 │
│  MINOR KEY + Slow Tempo + Low Energy                            │
│  ────────────────────────────────────────                       │
│  Emotion: Melancholy, Reflection, Loss                          │
│  Visual:  Deep blues/purples, downward drift, contraction,      │
│           rain-like particles, cold dim lighting                │
│                                                                 │
│  MINOR KEY + Fast Tempo + High Energy                           │
│  ────────────────────────────────────────                       │
│  Emotion: Intensity, Urgency, Power                             │
│  Visual:  Deep reds/blacks, aggressive motion, sharp edges,     │
│           fire particles, dramatic shadows                      │
│                                                                 │
│  MAJOR KEY + Slow Tempo + Rising Dynamics                       │
│  ────────────────────────────────────────                       │
│  Emotion: Hope, Anticipation, Dawn                              │
│  Visual:  Soft golds/whites, gentle upward float, gradual       │
│           brightening, lens flares emerging                     │
│                                                                 │
│  DISSONANT + Any Tempo + Unstable                               │
│  ────────────────────────────────────────                       │
│  Emotion: Tension, Unease, Chaos                                │
│  Visual:  Clashing colors, fragmentation, glitch effects,       │
│           unstable camera, chromatic aberration                 │
│                                                                 │
│  CONSONANT + Resolution + Arrival                               │
│  ────────────────────────────────────────                       │
│  Emotion: Relief, Completion, Peace                             │
│  Visual:  Harmonious palette, stability, clarity, symmetry,     │
│           everything settling into place                        │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Song Structure → Visual Narrative

```
INTRO (0:00-0:30)
├── Energy: Low → Building
├── Revelation: 0% → 15%
├── Visual: Dark void, single elements appearing
└── Purpose: Establish mood, tease what's coming

VERSE 1 (0:30-1:00)
├── Energy: Moderate, steady
├── Revelation: 15% → 35%
├── Visual: Scene taking shape, characters emerging
└── Purpose: Tell the story, build foundation

PRE-CHORUS (1:00-1:15)
├── Energy: Rising
├── Revelation: 35% → 50%
├── Visual: Tension building, colors intensifying
└── Purpose: Create anticipation

CHORUS (1:15-1:45)
├── Energy: HIGH
├── Revelation: 50% → 75%
├── Visual: Full expression, maximum impact
└── Purpose: Emotional payoff

VERSE 2 (1:45-2:15)
├── Energy: Reset to moderate
├── Revelation: 75% (maintain)
├── Visual: Variation on established scene
└── Purpose: Develop, add depth

BRIDGE (2:15-2:45)
├── Energy: Different, often lower
├── Revelation: 75% → 85%
├── Visual: New perspective, reflection
└── Purpose: Contrast, prepare for finale

FINAL CHORUS (2:45-3:15)
├── Energy: MAXIMUM
├── Revelation: 85% → 100%
├── Visual: Everything revealed, full glory
└── Purpose: Ultimate climax

OUTRO (3:15-3:45)
├── Energy: Falling
├── Revelation: 100% → fade
├── Visual: Graceful dissolution, memory
└── Purpose: Closure, lingering emotion
```

## Technology Stack

### Audio Analysis Libraries

**Primary: Aubio (C, with Rust bindings)**
```
- Beat/tempo detection
- Onset detection
- Pitch detection
- FFT analysis
- Real-time capable
```

**Secondary: Essentia (C++, for offline analysis)**
```
- Key detection
- Chord estimation
- Music structure segmentation
- Mood classification
- 100+ audio descriptors
```

**Rust-Native Options:**
```
- fundsp: Real-time audio DSP
- rustfft: FFT computation
- pitch_detection: Pitch tracking
- beat_detector: Beat detection
```

### Offline Analysis Pipeline

```python
# analyze_song.py - Run once per song
from essentia.standard import *

def analyze_song(audio_path):
    # Load audio
    audio = MonoLoader(filename=audio_path)()

    # Key detection
    key, scale, strength = KeyExtractor()(audio)

    # Chord progression
    chords = ChordsDetection()(audio)

    # Structure segmentation
    segments = MusicStructure()(audio)

    # Energy curve
    energy = EnergyBand()(audio)

    # Climax detection
    climaxes = find_climaxes(energy, segments)

    # Emotional arc
    emotion_arc = map_emotions(key, scale, energy, segments)

    return {
        'key': key,
        'scale': scale,
        'chords': chords,
        'segments': segments,
        'energy_curve': energy,
        'climaxes': climaxes,
        'emotion_arc': emotion_arc,
    }
```

## The .synth File Format (Revised)

```rust
/// Complete song analysis + pre-rendered visuals
pub struct SynthFile {
    /// File format version
    pub version: u32,

    /// Original audio fingerprint
    pub audio_hash: [u8; 32],

    /// Musical analysis
    pub music: MusicAnalysis,

    /// Pre-rendered video segments
    pub video_segments: Vec<VideoSegment>,

    /// Timeline of visual events
    pub visual_timeline: Vec<VisualEvent>,

    /// Shader parameter curves
    pub shader_curves: ShaderCurves,

    /// Optional: Lyric data
    pub lyrics: Option<LyricData>,
}

pub struct MusicAnalysis {
    /// Song duration
    pub duration: f64,

    /// Detected key and mode
    pub key: Key,
    pub mode: Mode,

    /// Tempo information
    pub tempo: f32,
    pub tempo_curve: Vec<(f64, f32)>,  // (time, bpm)

    /// Beat timestamps
    pub beats: Vec<f64>,
    pub downbeats: Vec<f64>,  // First beat of each bar

    /// Chord progression
    pub chords: Vec<ChordEvent>,

    /// Song structure
    pub sections: Vec<SectionMarker>,

    /// Energy curve (sampled at 10Hz)
    pub energy_curve: Vec<f32>,

    /// Tension curve (harmonic tension over time)
    pub tension_curve: Vec<f32>,

    /// Climax points (timestamps of peak moments)
    pub climaxes: Vec<ClimaxPoint>,

    /// Emotional arc
    pub emotion_arc: Vec<EmotionPoint>,
}

pub struct SectionMarker {
    pub start_time: f64,
    pub end_time: f64,
    pub section_type: SectionType,
    pub energy_level: f32,
    pub repetition: u8,  // 1st chorus, 2nd chorus, etc.
}

pub struct ChordEvent {
    pub time: f64,
    pub chord: Chord,
    pub tension: f32,  // 0.0 = consonant, 1.0 = dissonant
}

pub struct ClimaxPoint {
    pub time: f64,
    pub intensity: f32,
    pub climax_type: ClimaxType,  // Drop, Crescendo, Emotional, etc.
}

pub struct EmotionPoint {
    pub time: f64,
    pub primary_emotion: Emotion,
    pub intensity: f32,
    pub secondary_emotion: Option<Emotion>,
}
```

## Music-Driven Revelation

The "painter revealing" now tracks **musical understanding**:

```rust
pub struct MusicDrivenRevelation {
    // How well we understand the music
    key_confidence: f32,        // Do we know the key?
    structure_confidence: f32,  // Do we know where we are?
    pattern_confidence: f32,    // Have we heard this before?

    // Revelation state
    clarity: f32,
}

impl MusicDrivenRevelation {
    pub fn update(&mut self, music: &MusicUnderstanding, time: f64) {
        // Early song: we're learning the music
        if time < 30.0 {
            self.clarity = 0.1 + (time / 30.0) * 0.2;
        }

        // Key detected: major clarity boost
        if music.key_confidence > 0.8 && self.key_confidence < 0.8 {
            self.clarity += 0.15;
            self.key_confidence = music.key_confidence;
        }

        // First chorus: structure revealed
        if music.section == Section::Chorus && self.structure_confidence < 0.5 {
            self.clarity += 0.2;
            self.structure_confidence = 0.8;
        }

        // Pattern repetition: familiarity builds
        if music.repetition_count > 0 {
            self.pattern_confidence = (music.repetition_count as f32 * 0.2).min(1.0);
            self.clarity = self.clarity.max(0.5 + self.pattern_confidence * 0.3);
        }

        // Climax: full revelation
        if music.is_climax {
            self.clarity = 1.0;
        }
    }
}
```

## Real-Time vs Offline Split

### Offline (Pre-processing, hours)

```
INPUT: Raw audio file
        │
        ▼
┌───────────────────┐
│  ESSENTIA         │
│  - Full song key  │
│  - All chords     │
│  - Structure map  │
│  - Energy curve   │
│  - Emotion arc    │
└─────────┬─────────┘
          │
          ▼
┌───────────────────┐
│  VIDEO GENERATOR  │
│  - CogVideoX      │
│  - Per-section    │
│  - Mood-matched   │
└─────────┬─────────┘
          │
          ▼
┌───────────────────┐
│  .synth FILE      │
│  - Analysis data  │
│  - Video segments │
│  - Shader curves  │
└───────────────────┘
```

### Runtime (Real-time, 60fps)

```
INPUT: .synth file + audio playback
        │
        ▼
┌───────────────────────────────────────────────────────────────┐
│                     RUNTIME ENGINE                             │
│                                                                │
│  ┌─────────────┐   ┌─────────────┐   ┌───────────────────┐   │
│  │ Audio       │   │ Real-time   │   │ Video Compositor  │   │
│  │ Playback    │──▶│ Analysis    │──▶│                   │   │
│  │             │   │ (Layer 1)   │   │  Pre-rendered +   │   │
│  └─────────────┘   └─────────────┘   │  Reactive shaders │   │
│                                       │  + Particles      │   │
│  ┌─────────────┐                      │  + Transitions    │   │
│  │ Pre-baked   │─────────────────────▶│                   │   │
│  │ Analysis    │                      └───────────────────┘   │
│  │ (Layer 2-3) │                              │               │
│  └─────────────┘                              ▼               │
│                                        ┌─────────────┐        │
│                                        │   OUTPUT    │        │
│                                        │   60 FPS    │        │
│                                        └─────────────┘        │
└───────────────────────────────────────────────────────────────┘
```

## Implementation Phases

### Phase 1: Music Analysis Foundation (2 weeks)
- Integrate Aubio for real-time (beat, onset, FFT)
- Build Essentia pipeline for offline (key, chords, structure)
- Create MusicUnderstanding data structures
- Test on 10 diverse songs

### Phase 2: Emotion Mapping (1 week)
- Music theory → Emotion rules
- Emotion → Visual parameter mapping
- Build palette/lighting/motion systems
- Validate against human perception

### Phase 3: Pre-render Pipeline (2 weeks)
- Video generation per section
- Mood-matched prompts from music analysis
- .synth file format implementation
- Segment stitching with transitions

### Phase 4: Runtime Player (2 weeks)
- Hardware video decode
- Real-time shader composition
- Music-synced transitions
- Beat-reactive effects

### Phase 5: Revelation System (1 week)
- Music-driven clarity calculation
- Progressive visual emergence
- Climax detection → Full reveal
- Polish and tuning

## Hardware Requirements

**Generation (Offline):**
- Any CUDA GPU for Essentia/analysis
- RTX 4090 / A100 for video generation
- 32GB RAM
- ~30 min - 2 hours per song

**Playback (Runtime):**
- Any GPU with hardware video decode
- 4GB VRAM sufficient
- Real 60fps on integrated graphics

## Why This Works

1. **Music is Universal** - Works for any language, instrumental, or vocal
2. **Theory is Objective** - Key, tempo, structure are measurable
3. **Emotions are Mapped** - Centuries of music theory tell us what means what
4. **Pre-render Enables Quality** - Offline = unlimited compute time
5. **Real-time Adds Life** - Shaders make pre-rendered feel reactive
6. **The Revelation is Musical** - Understanding grows as the song teaches us

---

*"Music expresses that which cannot be put into words and that which cannot remain silent."* — Victor Hugo
