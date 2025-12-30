# Emergent Revelation: The Painter's Algorithm

## The Vision

> "First we may not know what is painting, but when time passes we understand and start fully visualizing"

Like watching a master painter work - initial brushstrokes seem random, abstract, meaningless. But as they continue, patterns emerge, forms take shape, and suddenly meaning crystallizes. This is how SYNESTHESIA should work.

## Core Concept: Progressive Semantic Clarity

```
Time: 0s                    30s                    60s                    120s
      │                     │                      │                      │
      ▼                     ▼                      ▼                      ▼
   ╔══════════╗         ╔══════════╗          ╔══════════╗          ╔══════════╗
   ║ ░░░░░░░░ ║         ║ ▓▓▒▒░░░░ ║          ║ █▓▓▒▒░░░ ║          ║ ████▓▓▒▒ ║
   ║ ░░░░░░░░ ║    →    ║ ▒▒▒▓▓░░░ ║     →    ║ ▓▓██▓▓▒▒ ║     →    ║ ████████ ║
   ║ ░░░░░░░░ ║         ║ ░░░▒▒▒▒░ ║          ║ ▒▒▓▓██▓▓ ║          ║ ██████▓▓ ║
   ╚══════════╝         ╚══════════╝          ╚══════════╝          ╚══════════╝

   Abstract noise      Forms emerge         Shapes clarify      Full revelation
   No context yet      Some lyrics heard    Emotional arc       Complete meaning
```

## The Semantic Accumulation Model

### Context Window (Rolling Memory)

```rust
/// Semantic context that accumulates over time
pub struct SemanticContext {
    /// Accumulated lyric fragments
    pub lyrics: Vec<TimedLyric>,

    /// Confidence score 0.0 (abstract) → 1.0 (clear)
    pub clarity: f32,

    /// Detected themes with confidence
    pub themes: HashMap<Theme, f32>,

    /// Emotional trajectory
    pub emotional_arc: Vec<EmotionalPoint>,

    /// Visual elements to manifest
    pub pending_elements: Vec<VisualElement>,

    /// Currently revealed elements
    pub revealed_elements: Vec<VisualElement>,
}

impl SemanticContext {
    /// Add new lyric, update understanding
    pub fn accumulate(&mut self, lyric: TimedLyric) {
        self.lyrics.push(lyric);

        // Clarity increases with more context
        self.clarity = (self.lyrics.len() as f32 / 20.0).min(1.0);

        // Re-analyze themes with fuller context
        self.themes = self.analyze_themes();

        // Gradually reveal visual elements
        self.reveal_pending();
    }
}
```

### Clarity Levels

| Level | Clarity | Visual State | What We Know |
|-------|---------|--------------|--------------|
| 0 | 0.0-0.1 | Pure noise/color | Audio frequencies only |
| 1 | 0.1-0.3 | Flowing abstract | Basic mood (energy level) |
| 2 | 0.3-0.5 | Forms emerging | First themes detected |
| 3 | 0.5-0.7 | Shapes visible | Setting/location hints |
| 4 | 0.7-0.9 | Scene forming | Characters/narrative |
| 5 | 0.9-1.0 | Full revelation | Complete understanding |

## Open-Source Video Generation Models

### Primary Models for Integration

#### 1. CogVideoX (THUDM) ⭐ Recommended
- **Source**: https://github.com/THUDM/CogVideo
- **License**: Apache 2.0
- **Capabilities**:
  - Text-to-video generation
  - 6-second clips at 480p
  - Strong semantic understanding
- **Why**: Best open-source semantic-to-video model

#### 2. Stable Video Diffusion (Stability AI)
- **Source**: https://github.com/Stability-AI/generative-models
- **License**: Stability AI Community License
- **Capabilities**:
  - Image-to-video (img2vid)
  - Motion from still frame
  - 4-second clips at 576x1024
- **Why**: Excellent for evolving a "base painting"

#### 3. Open-Sora (HPC-AI Tech)
- **Source**: https://github.com/hpcaitech/Open-Sora
- **License**: Apache 2.0
- **Capabilities**:
  - Longer videos (up to 16s)
  - Variable resolution
  - Strong motion coherence
- **Why**: Best for longer, coherent sequences

#### 4. AnimateDiff
- **Source**: https://github.com/guoyww/AnimateDiff
- **License**: Apache 2.0
- **Capabilities**:
  - Works with any Stable Diffusion checkpoint
  - Adds motion to still images
  - Very customizable
- **Why**: Modular, can use custom trained models

#### 5. Mochi 1 (Genmo)
- **Source**: https://github.com/genmoai/mochi
- **License**: Apache 2.0
- **Capabilities**:
  - Newest generation (late 2024)
  - High quality motion
  - Strong prompt adherence
- **Why**: State-of-the-art quality

### Recommended Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      REVELATION ENGINE                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────┐    ┌──────────────┐    ┌─────────────────┐   │
│  │   WHISPER   │───▶│  SEMANTIC    │───▶│   CLARITY       │   │
│  │ Transcriber │    │  ACCUMULATOR │    │   CALCULATOR    │   │
│  └─────────────┘    └──────────────┘    └────────┬────────┘   │
│                                                   │             │
│                                           clarity: 0.0-1.0      │
│                                                   │             │
│  ┌────────────────────────────────────────────────▼──────────┐ │
│  │                   DIFFUSION CONTROLLER                     │ │
│  │  ┌──────────────────────────────────────────────────────┐ │ │
│  │  │  Noise Level = 1.0 - clarity                         │ │ │
│  │  │  Prompt Strength = clarity * 0.8                     │ │ │
│  │  │  Guidance Scale = 3 + (clarity * 12)                 │ │ │
│  │  └──────────────────────────────────────────────────────┘ │ │
│  └───────────────────────────────────────────────────────────┘ │
│                              │                                  │
│                              ▼                                  │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │              VIDEO GENERATION (CogVideoX)                  │ │
│  │  • Low clarity: High noise, abstract forms                │ │
│  │  • Mid clarity: Emerging shapes, partial scenes           │ │
│  │  • High clarity: Full scene, detailed visuals             │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## The Revelation Pipeline

### Phase 1: Abstract Genesis (0-20 seconds)
```rust
fn generate_abstract_frame(audio: &AudioFeatures) -> VideoFrame {
    // Pure audio-reactive abstraction
    let prompt = format!(
        "abstract flowing colors, {} energy, {} rhythm",
        if audio.rms > 0.6 { "high" } else { "calm" },
        if audio.is_beat { "pulsing" } else { "smooth" }
    );

    generate_video(
        prompt: &prompt,
        noise_level: 0.9,      // High noise = abstract
        guidance_scale: 3.0,   // Low guidance = freedom
        steps: 10,             // Few steps = rough
    )
}
```

### Phase 2: Forms Emerge (20-60 seconds)
```rust
fn generate_emerging_frame(
    audio: &AudioFeatures,
    context: &SemanticContext,
) -> VideoFrame {
    // Some lyrics known, themes starting to appear
    let detected_themes = context.themes
        .iter()
        .filter(|(_, conf)| *conf > 0.3)
        .collect::<Vec<_>>();

    let prompt = format!(
        "abstract forms becoming {}, {} atmosphere, dreamlike, emerging from mist",
        detected_themes.first().map(|(t, _)| t.description()).unwrap_or("shapes"),
        context.primary_mood(),
    );

    generate_video(
        prompt: &prompt,
        noise_level: 0.6,      // Medium noise
        guidance_scale: 7.0,   // Medium guidance
        steps: 20,             // More refinement
    )
}
```

### Phase 3: Clarity Dawns (60-120 seconds)
```rust
fn generate_clarifying_frame(
    audio: &AudioFeatures,
    context: &SemanticContext,
) -> VideoFrame {
    // Rich context now available
    let scene = context.synthesize_scene();

    let prompt = format!(
        "{} in {} during {}, {} mood, {}. {}",
        scene.subjects.join(", "),
        scene.location,
        scene.time_of_day,
        scene.emotional_tone,
        scene.weather,
        scene.action_description,
    );

    generate_video(
        prompt: &prompt,
        noise_level: 0.2,      // Low noise = clear
        guidance_scale: 12.0,  // High guidance = faithful
        steps: 30,             // Full refinement
    )
}
```

### Phase 4: Full Revelation (120+ seconds)
```rust
fn generate_revealed_frame(
    audio: &AudioFeatures,
    context: &SemanticContext,
) -> VideoFrame {
    // Complete understanding, full visual fidelity
    let narrative = context.generate_narrative();

    let prompt = format!(
        "cinematic scene: {} {} {}. Style: {}, Lighting: {}, Camera: {}",
        narrative.setting,
        narrative.character_action,
        narrative.emotional_context,
        narrative.visual_style,
        narrative.lighting,
        narrative.camera_movement,
    );

    generate_video(
        prompt: &prompt,
        noise_level: 0.05,     // Minimal noise
        guidance_scale: 15.0,  // Maximum guidance
        steps: 50,             // Maximum quality
    )
}
```

## Implementation Strategy

### 1. Local Inference Stack

```toml
# Cargo.toml additions for video generation
[dependencies]
# Rust bindings for video diffusion
candle-core = "0.4"           # ML framework
candle-transformers = "0.4"   # Model implementations
hf-hub = "0.3"                # Model downloading

# Video encoding
ffmpeg-next = "6.0"           # Video encoding/decoding
```

### 2. Model Loading

```rust
pub struct RevelationEngine {
    /// Video generation model (CogVideoX)
    video_model: CogVideoModel,

    /// Text encoder for prompts
    text_encoder: T5Encoder,

    /// Semantic context accumulator
    context: SemanticContext,

    /// Current clarity level
    clarity: f32,

    /// Frame buffer for smooth transitions
    frame_buffer: RingBuffer<VideoFrame>,
}

impl RevelationEngine {
    pub async fn new() -> Result<Self> {
        // Load models from HuggingFace
        let api = hf_hub::api::sync::Api::new()?;

        let video_model_path = api
            .model("THUDM/CogVideoX-2b".to_string())
            .get("model.safetensors")?;

        // Initialize with low-precision for speed
        let video_model = CogVideoModel::load(
            video_model_path,
            DType::F16,
            &Device::cuda_if_available(0)?,
        )?;

        Ok(Self { ... })
    }
}
```

### 3. Progressive Generation

```rust
impl RevelationEngine {
    /// Generate next frame based on accumulated context
    pub fn generate_frame(&mut self, audio: &AudioFeatures) -> VideoFrame {
        // Build prompt based on clarity level
        let prompt = match self.clarity {
            c if c < 0.2 => self.abstract_prompt(audio),
            c if c < 0.5 => self.emerging_prompt(audio),
            c if c < 0.8 => self.clarifying_prompt(audio),
            _ => self.revealed_prompt(audio),
        };

        // Dynamic generation parameters
        let params = GenerationParams {
            noise_level: 1.0 - self.clarity,
            guidance_scale: 3.0 + (self.clarity * 12.0),
            num_inference_steps: (10 + (self.clarity * 40.0) as usize),
            fps: 24,
            num_frames: 6,  // ~250ms of video
        };

        self.video_model.generate(&prompt, params)
    }

    /// Accumulate new lyric and update clarity
    pub fn process_lyric(&mut self, lyric: TranscribedWord) {
        self.context.accumulate(lyric);
        self.clarity = self.context.calculate_clarity();
    }
}
```

## The "Painting" Aesthetic

### Visual Progression

1. **Canvas Prime** (0s): Dark, textured base - like a prepared canvas
2. **First Strokes** (10s): Bold color fields responding to audio
3. **Underpainting** (30s): Loose forms suggesting shapes
4. **Block-in** (60s): Major elements established but soft
5. **Refinement** (90s): Details emerging, edges sharpening
6. **Final Glazes** (120s): Full color, depth, and meaning

### Shader Implementation

```wgsl
// revelation_shader.wgsl

struct Uniforms {
    clarity: f32,
    time: f32,
    beat_intensity: f32,
}

@fragment
fn fragment_main(
    @location(0) uv: vec2<f32>,
    @builtin(position) frag_coord: vec4<f32>,
) -> @location(0) vec4<f32> {
    // Sample the generated video frame
    let video_color = textureSample(video_texture, video_sampler, uv);

    // Apply painterly effect based on clarity
    let brush_noise = fbm_noise(uv * (10.0 - uniforms.clarity * 8.0), 4u);
    let stroke_direction = vec2<f32>(
        cos(brush_noise * 6.28),
        sin(brush_noise * 6.28)
    );

    // Smear effect - stronger when less clear
    let smear_amount = (1.0 - uniforms.clarity) * 0.1;
    let smeared_uv = uv + stroke_direction * smear_amount;
    let smeared_color = textureSample(video_texture, video_sampler, smeared_uv);

    // Blend based on clarity
    let paint_color = mix(smeared_color, video_color, uniforms.clarity);

    // Add canvas texture when abstract
    let canvas = canvas_texture(uv) * (1.0 - uniforms.clarity) * 0.3;

    // Beat reactive highlights
    let highlight = uniforms.beat_intensity * (1.0 - uniforms.clarity) * 0.2;

    return paint_color + vec4<f32>(canvas, canvas, canvas, 0.0)
                       + vec4<f32>(highlight, highlight * 0.8, highlight * 0.6, 0.0);
}
```

## Hardware Requirements

### Minimum (720p, 12 FPS generation)
- NVIDIA RTX 3070 (8GB VRAM)
- 32GB RAM
- NVMe SSD for model loading

### Recommended (1080p, 24 FPS generation)
- NVIDIA RTX 4080 (16GB VRAM)
- 64GB RAM
- NVMe SSD RAID

### Optimal (4K, 30 FPS generation)
- NVIDIA RTX 4090 (24GB VRAM) or A100
- 128GB RAM
- High-speed storage

## Model Fine-Tuning for Music Visuals

### Training Data
- Music video stills with lyric annotations
- Artist-provided "vision boards" per song
- Genre-specific visual aesthetics

### LoRA Fine-Tuning
```python
# Fine-tune CogVideoX for music visualization
from cogvideo import CogVideoXModel
from peft import LoraConfig, get_peft_model

# Load base model
model = CogVideoXModel.from_pretrained("THUDM/CogVideoX-2b")

# Configure LoRA for efficient fine-tuning
lora_config = LoraConfig(
    r=16,
    lora_alpha=32,
    target_modules=["q_proj", "v_proj", "k_proj", "out_proj"],
    lora_dropout=0.05,
)

# Create trainable model
peft_model = get_peft_model(model, lora_config)

# Train on music-visual pairs
train_music_visual_lora(
    model=peft_model,
    dataset="music_visual_pairs",
    epochs=10,
    lr=1e-4,
)
```

## Next Steps

1. **Integrate CogVideoX** into synesthesia crate
2. **Build clarity accumulator** in semantic parser
3. **Create shader pipeline** for painterly effects
4. **Test with demo songs** to tune revelation timing
5. **Fine-tune model** on music visualization data

---

*"The canvas knows nothing of what will emerge. But stroke by stroke, the painting reveals its truth."*
