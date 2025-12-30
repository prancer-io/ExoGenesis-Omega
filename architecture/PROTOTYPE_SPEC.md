# SYNESTHESIA Prototype Specification

## Goal
Build a **minimal but magical** prototype in 4-6 weeks that demonstrates the core breakthrough: **AI understands lyrics and creates meaningful visuals**.

---

## Prototype Scope

### What We Build
1. Desktop app (Rust + wgpu)
2. Load any MP3/audio file
3. Real-time lyric transcription (Whisper)
4. Semantic parsing (local LLM)
5. Dynamic 3D scene generation (procedural + diffusion backgrounds)

### What We Skip (For Now)
- VR support
- Multiplayer
- Artist tools
- Production infrastructure

---

## Technical Stack

```
┌─────────────────────────────────────────────────────┐
│                 PROTOTYPE STACK                      │
├─────────────────────────────────────────────────────┤
│                                                     │
│  FRONTEND                                           │
│  ├── Window: winit (cross-platform)                 │
│  ├── Rendering: wgpu (Vulkan/Metal/DX12)            │
│  ├── UI: egui (immediate mode)                      │
│  └── 3D Scene: Custom renderer + bevy_ecs           │
│                                                     │
│  AI INFERENCE                                       │
│  ├── Whisper: whisper.cpp (CPU) / whisper-rs       │
│  ├── LLM: llama.cpp via llama-cpp-rs               │
│  ├── Diffusion: stable-diffusion.cpp (optional)    │
│  └── Audio: symphonia (decode) + rubato (resample) │
│                                                     │
│  OMEGA CORE                                         │
│  ├── omega-brain: Cognitive coordination            │
│  ├── omega-perception: Audio feature extraction     │
│  ├── omega-language: Lyric semantics               │
│  ├── omega-emotion: Mood detection                 │
│  ├── omega-creativity: Scene direction             │
│  └── omega-world: 3D scene state                   │
│                                                     │
└─────────────────────────────────────────────────────┘
```

---

## Core Pipeline

### 1. Audio Input & Transcription

```rust
// Real-time audio processing with Whisper
pub struct AudioProcessor {
    whisper: WhisperContext,
    audio_buffer: RingBuffer<f32>,
    transcription_tx: Sender<TranscribedWord>,
}

pub struct TranscribedWord {
    pub text: String,
    pub start_time: f64,
    pub end_time: f64,
    pub confidence: f32,
}

impl AudioProcessor {
    pub fn process_chunk(&mut self, samples: &[f32], sample_rate: u32) {
        self.audio_buffer.push(samples);

        // Process every 500ms of audio
        if self.audio_buffer.len() >= sample_rate as usize / 2 {
            let chunk = self.audio_buffer.drain();
            let words = self.whisper.transcribe(&chunk, sample_rate);

            for word in words {
                self.transcription_tx.send(word);
            }
        }
    }
}
```

### 2. Semantic Parser

```rust
// LLM-based semantic extraction
pub struct SemanticParser {
    llm: LlamaContext,
    prompt_template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticScene {
    pub setting: Setting,
    pub characters: Vec<Character>,
    pub actions: Vec<Action>,
    pub mood: Mood,
    pub visual_elements: Vec<VisualElement>,
    pub camera: CameraDirection,
}

#[derive(Debug, Clone)]
pub struct Setting {
    pub location: String,        // "city street", "beach", "forest"
    pub time_of_day: TimeOfDay,  // dawn, day, dusk, night
    pub weather: Weather,        // clear, rain, snow, fog
    pub indoor: bool,
}

#[derive(Debug, Clone)]
pub struct Character {
    pub id: String,              // For continuity
    pub description: String,     // "hooded figure", "woman in red dress"
    pub state: String,           // "walking", "standing", "running"
    pub emotion: String,         // "sad", "hopeful", "angry"
}

impl SemanticParser {
    pub fn parse(&self, lyrics: &[TranscribedWord], context: &NarrativeContext) -> SemanticScene {
        let prompt = self.build_prompt(lyrics, context);
        let response = self.llm.complete(&prompt);
        self.parse_response(&response)
    }

    fn build_prompt(&self, lyrics: &[TranscribedWord], context: &NarrativeContext) -> String {
        format!(r#"
You are a visual director translating song lyrics into scene descriptions.

Previous scene context:
- Setting: {}
- Characters: {:?}
- Emotional arc: {}

Current lyrics: "{}"

Generate a JSON scene description:
{{
  "setting": {{
    "location": "specific place",
    "time_of_day": "dawn|day|dusk|night",
    "weather": "clear|rain|snow|fog",
    "indoor": boolean
  }},
  "characters": [
    {{
      "id": "unique_id for continuity",
      "description": "visual description",
      "state": "what they're doing",
      "emotion": "how they feel"
    }}
  ],
  "actions": ["what's happening"],
  "mood": {{
    "primary": "emotion word",
    "intensity": 0.0-1.0,
    "color_palette": ["hex colors"]
  }},
  "visual_elements": [
    {{"type": "particle|light|object", "description": "..."}}
  ],
  "camera": {{
    "shot": "wide|medium|close|extreme_close",
    "movement": "static|pan|dolly|orbit",
    "focus": "what to focus on"
  }}
}}

IMPORTANT: Maintain visual continuity with previous scene unless lyrics indicate a change.
"#,
            context.current_setting,
            context.active_characters,
            context.emotional_arc,
            lyrics.iter().map(|w| &w.text).collect::<Vec<_>>().join(" ")
        )
    }
}
```

### 3. Scene Renderer

```rust
// 3D scene generation and rendering
pub struct SceneRenderer {
    device: wgpu::Device,
    queue: wgpu::Queue,

    // Scene elements
    environment: Environment,
    characters: Vec<CharacterInstance>,
    particles: ParticleSystem,
    lights: Vec<Light>,

    // Shaders
    pbr_pipeline: wgpu::RenderPipeline,
    particle_pipeline: wgpu::RenderPipeline,
    post_process: PostProcessPipeline,
}

pub struct Environment {
    skybox: Skybox,
    ground: Ground,
    fog: FogSettings,
    ambient: AmbientLight,

    // Generated background (from diffusion)
    background_texture: Option<wgpu::Texture>,
}

impl SceneRenderer {
    pub fn apply_scene(&mut self, scene: &SemanticScene) {
        // Update environment
        self.environment.apply_setting(&scene.setting);
        self.environment.apply_mood(&scene.mood);

        // Update characters
        self.update_characters(&scene.characters);

        // Update effects
        self.particles.configure_for_mood(&scene.mood);
        self.update_lights(&scene.visual_elements);

        // Apply camera
        self.camera.apply_direction(&scene.camera);
    }

    pub fn render(&mut self, audio_features: &AudioFeatures) -> wgpu::SurfaceTexture {
        // Audio-reactive modifications
        let beat_intensity = audio_features.beat_intensity;

        // 1. Render scene to HDR buffer
        self.render_scene(beat_intensity);

        // 2. Render particles
        self.render_particles(audio_features);

        // 3. Post-processing (bloom, chromatic aberration on beat)
        self.post_process.apply(beat_intensity);

        self.present()
    }
}
```

### 4. Omega Integration

```rust
// Cognitive coordination using Omega crates
use omega_brain::CognitiveBrain;
use omega_perception::AudioPerception;
use omega_language::LanguageProcessor;
use omega_emotion::EmotionalState;
use omega_creativity::CreativeDirector;
use omega_world::WorldState;

pub struct SynesthesiaCore {
    brain: CognitiveBrain,
    perception: AudioPerception,
    language: LanguageProcessor,
    emotion: EmotionalState,
    creativity: CreativeDirector,
    world: WorldState,
}

impl SynesthesiaCore {
    pub fn process_frame(
        &mut self,
        audio_chunk: &[f32],
        transcribed_words: &[TranscribedWord],
        delta_time: f64,
    ) -> FrameOutput {
        // 1. Perception: Extract audio features
        let audio_features = self.perception.analyze(audio_chunk);

        // 2. Language: Process new lyrics
        if !transcribed_words.is_empty() {
            self.language.process_lyrics(transcribed_words);
        }

        // 3. Emotion: Update emotional state
        self.emotion.update(&audio_features, self.language.current_sentiment());

        // 4. Brain: Coordinate decision-making
        let attention = self.brain.compute_attention(&[
            self.language.current_focus(),
            self.emotion.current_state(),
            audio_features.energy_level(),
        ]);

        // 5. Creativity: Direct the visual scene
        let scene_update = self.creativity.direct_scene(
            &self.language.semantic_context(),
            &self.emotion.current_state(),
            &attention,
        );

        // 6. World: Update 3D world state
        self.world.apply_update(&scene_update, delta_time);

        FrameOutput {
            scene: self.world.current_scene(),
            audio_features,
            emotion: self.emotion.current_state(),
        }
    }
}
```

---

## Visual Style

### Aesthetic Direction
- **Cinematic**: Film-quality lighting, depth of field, motion blur
- **Stylized**: Slightly stylized rather than photorealistic (more forgiving for generation)
- **Mood-driven**: Color palettes shift with emotion
- **Responsive**: Every beat creates visual reaction

### Scene Types

#### 1. Abstract Mode (Default)
```
- Particle systems driven by audio
- Procedural geometry that morphs with music
- Color palette from semantic mood
- Text overlays of key lyrics
```

#### 2. Narrative Mode (When lyrics detected)
```
- Generated backgrounds from diffusion
- Procedural character silhouettes
- Environmental effects matching lyrics
- Camera moves following narrative
```

### Example Scenes

**Lyric: "I'm walking alone through the rain"**
```json
{
  "background": "City street at night, wet pavement, neon reflections",
  "character": "Silhouette figure walking, head down",
  "effects": ["Rain particles", "Puddle reflections", "Dim street lights"],
  "mood": "Blue/gray color palette, slow camera follow",
  "audio_reactive": "Rain intensity matches drum pattern"
}
```

**Lyric: "You set my heart on fire"**
```json
{
  "background": "Abstract warm void, orange/red gradients",
  "character": "Glowing figure, heart area emphasized",
  "effects": ["Fire particles from chest", "Heat distortion", "Embers rising"],
  "mood": "Orange/red/gold palette, camera pushes in",
  "audio_reactive": "Fire intensity matches bass"
}
```

---

## File Structure

```
synesthesia/
├── Cargo.toml
├── src/
│   ├── main.rs                 # App entry, window setup
│   ├── app.rs                  # Main application state
│   │
│   ├── audio/
│   │   ├── mod.rs
│   │   ├── player.rs           # Audio playback (rodio)
│   │   ├── analyzer.rs         # Feature extraction
│   │   └── whisper.rs          # Transcription
│   │
│   ├── ai/
│   │   ├── mod.rs
│   │   ├── semantic.rs         # LLM semantic parsing
│   │   ├── diffusion.rs        # Background generation
│   │   └── prompts.rs          # Prompt templates
│   │
│   ├── omega/
│   │   ├── mod.rs
│   │   ├── brain.rs            # Cognitive coordination
│   │   ├── perception.rs       # Audio perception
│   │   ├── language.rs         # Lyric processing
│   │   ├── emotion.rs          # Emotional state
│   │   ├── creativity.rs       # Scene direction
│   │   └── world.rs            # World state
│   │
│   ├── render/
│   │   ├── mod.rs
│   │   ├── scene.rs            # Scene graph
│   │   ├── environment.rs      # Skybox, ground, fog
│   │   ├── characters.rs       # Character rendering
│   │   ├── particles.rs        # Particle systems
│   │   ├── post_process.rs     # Bloom, chromatic
│   │   └── shaders/
│   │       ├── pbr.wgsl
│   │       ├── particle.wgsl
│   │       └── post.wgsl
│   │
│   └── ui/
│       ├── mod.rs
│       └── overlay.rs          # egui overlays
│
├── assets/
│   ├── models/                 # Basic 3D models
│   ├── textures/               # Procedural textures
│   └── fonts/                  # UI fonts
│
└── models/                     # AI model files
    ├── whisper-small.bin       # ~500MB
    └── mistral-7b-q4.gguf      # ~4GB
```

---

## Development Phases

### Week 1-2: Foundation
- [ ] Project setup with wgpu rendering
- [ ] Audio playback and feature extraction
- [ ] Basic 3D scene with procedural elements
- [ ] Audio-reactive visuals (non-AI)

### Week 3-4: AI Integration
- [ ] Whisper integration for transcription
- [ ] LLM integration for semantic parsing
- [ ] Scene generation from semantics
- [ ] Narrative continuity system

### Week 5-6: Polish
- [ ] Visual quality improvements
- [ ] Performance optimization
- [ ] Demo songs with tuned experiences
- [ ] Video capture for demos

---

## Success Criteria

### The "Magic Moment"
When someone plays a song they know well and sees the visuals:
1. **Recognize** their song's story in the visuals
2. **Feel** the emotional arc visualized
3. **Notice** specific lyrics creating specific scenes
4. **Say "Wow"** at least once

### Technical Metrics
- 60fps minimum on RTX 3060
- <300ms latency from lyric to visual
- Semantic parsing accuracy >80% on test set
- Scene continuity maintained across song

---

## Demo Songs (For Testing)

1. **Emotional ballad**: "Someone Like You" - Adele
   - Clear narrative lyrics
   - Strong emotional arc
   - Tests: sadness, longing, resolution

2. **Upbeat pop**: "Blinding Lights" - The Weeknd
   - Energetic with abstract imagery
   - Strong beat for audio-reactive
   - Tests: energy, night city, driving

3. **Hip-hop storytelling**: "Lose Yourself" - Eminem
   - Dense, fast lyrics
   - Narrative progression
   - Tests: parsing speed, story tracking

4. **Ambient/minimal**: "Weightless" - Marconi Union
   - Minimal lyrics
   - Tests: pure audio visualization
   - Demonstrates graceful fallback

---

## Getting Started

```bash
# Clone and setup
cd synesthesia
cargo build --release

# Download models
./scripts/download_models.sh

# Run with test song
cargo run --release -- --song "path/to/song.mp3"
```

---

*This prototype will prove the core value proposition. If people find it magical, we have a product. If not, we learn what's missing before building the full platform.*
