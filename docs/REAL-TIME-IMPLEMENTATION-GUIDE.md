# Real-Time 3D Generation: Tactical Implementation Guide

**Goal:** Transform omega-synesthesia from offline batch processing to real-time streaming

**Timeline:** 2-3 weeks for functional prototype
**Target Latency:** <25ms (perceptually instant)
**Target Performance:** 60 FPS @ 1080p, <15% CPU

---

## 🎯 THE TRANSFORMATION

### Current Flow (Offline)
```
┌──────────────────────────────────────────────────────────┐
│  Load Entire File (2-300MB) → 500-5000ms                 │
│           ↓                                               │
│  Analyze All Audio → 100-500ms                           │
│           ↓                                               │
│  Generate Entire World → 500-2000ms                      │
│           ↓                                               │
│  Export GLTF → 200-500ms                                 │
│           ↓                                               │
│  Render Static Scene                                     │
│                                                           │
│  TOTAL TIME: 1-6 seconds (BLOCKING)                      │
└──────────────────────────────────────────────────────────┘
```

### New Flow (Real-Time)
```
┌──────────────────────────────────────────────────────────┐
│  Audio Stream (512 samples @ 44.1kHz = 11.6ms chunks)    │
│           ↓                                               │
│  Ring Buffer (1024 samples = 23ms latency)               │
│           ↓                                               │
│  FFT (2048 samples) → 5ms (SIMD optimized)               │
│           ↓                                               │
│  Feature Extraction → 1ms (parallel)                     │
│           ↓                                               │
│  Incremental World Generation → 3ms                      │
│           ↓                                               │
│  GPU Render (wgpu) → 16ms @ 60 FPS                       │
│                                                           │
│  TOTAL LATENCY: ~23ms (REAL-TIME, NON-BLOCKING)         │
└──────────────────────────────────────────────────────────┘
```

---

## 📦 NEW CRATES TO CREATE

### 1. omega-synesthesia-streaming

**Purpose:** Real-time audio input and buffering

**Structure:**
```
omega-synesthesia-streaming/
├── Cargo.toml
└── src/
    ├── lib.rs               # Public API
    ├── audio_input.rs       # cpal integration
    ├── ring_buffer.rs       # Lock-free SPSC
    ├── stream_config.rs     # Configuration
    └── error.rs             # Error types
```

**Cargo.toml:**
```toml
[package]
name = "omega-synesthesia-streaming"
version = "0.1.0"
edition = "2021"

[dependencies]
cpal = "0.15"              # Cross-platform audio
ringbuf = "0.4"            # Lock-free ring buffer
thiserror = "1.0"
tokio = { version = "1", features = ["full"] }
crossbeam = "0.8"          # MPSC channels
```

**Key Implementation (lib.rs):**
```rust
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::{HeapRb, HeapProducer, HeapConsumer};

pub struct AudioInputStream {
    stream: cpal::Stream,
    consumer: HeapConsumer<f32>,
    config: StreamConfig,
}

impl AudioInputStream {
    pub fn new(source: AudioSource) -> Result<Self> {
        let host = cpal::default_host();

        let device = match source {
            AudioSource::Microphone => host.default_input_device()?,
            AudioSource::SystemAudio => host.default_output_device()?, // Loopback
            AudioSource::Device(name) => host.input_devices()?
                .find(|d| d.name().ok() == Some(name))?,
        };

        let config = device.default_input_config()?;
        let sample_rate = config.sample_rate().0;

        // Ring buffer: 1 second capacity
        let ring = HeapRb::<f32>::new(sample_rate as usize);
        let (mut producer, consumer) = ring.split();

        let stream = device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                // Lock-free push to ring buffer
                producer.push_slice(data);
            },
            |err| eprintln!("Audio error: {}", err),
            None,
        )?;

        stream.play()?;

        Ok(Self { stream, consumer, config })
    }

    /// Get next chunk (non-blocking)
    pub fn read_chunk(&mut self, size: usize) -> Option<Vec<f32>> {
        if self.consumer.len() >= size {
            let mut chunk = vec![0.0; size];
            self.consumer.pop_slice(&mut chunk);
            Some(chunk)
        } else {
            None // Not enough data yet
        }
    }

    /// Blocking read with timeout
    pub async fn read_chunk_async(&mut self, size: usize) -> Result<Vec<f32>> {
        loop {
            if let Some(chunk) = self.read_chunk(size) {
                return Ok(chunk);
            }
            tokio::time::sleep(Duration::from_millis(1)).await;
        }
    }
}

pub enum AudioSource {
    Microphone,
    SystemAudio,
    Device(String),
}

pub struct StreamConfig {
    pub sample_rate: u32,
    pub channels: u16,
    pub chunk_size: usize,     // 512 samples = 11.6ms @ 44.1kHz
    pub buffer_size: usize,    // 4096 samples = ~93ms buffer
}
```

---

### 2. omega-synesthesia-renderer

**Purpose:** GPU-accelerated real-time rendering with wgpu

**Structure:**
```
omega-synesthesia-renderer/
├── Cargo.toml
└── src/
    ├── lib.rs               # Renderer interface
    ├── wgpu_backend.rs      # wgpu implementation
    ├── camera.rs            # Camera controller
    ├── mesh.rs              # GPU mesh buffer
    ├── material.rs          # PBR shader
    ├── light.rs             # Dynamic lighting
    ├── shaders/
    │   ├── vertex.wgsl
    │   ├── fragment.wgsl
    │   └── compute.wgsl
    └── pipelines.rs         # Render pipelines
```

**Cargo.toml:**
```toml
[package]
name = "omega-synesthesia-renderer"
version = "0.1.0"

[dependencies]
wgpu = "0.18"
winit = "0.29"             # Window management
glam = "0.25"              # Math
bytemuck = "1.14"          # Zero-copy casting
pollster = "0.3"           # Async runtime for wgpu
```

**Key Implementation (lib.rs):**
```rust
use wgpu::util::DeviceExt;
use glam::{Mat4, Vec3};

pub struct SynesthesiaRenderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,

    // Dynamic geometry
    mesh_buffers: Vec<MeshBuffer>,
    material_bind_group: wgpu::BindGroup,

    // Camera
    camera: Camera,
    camera_buffer: wgpu::Buffer,
}

impl SynesthesiaRenderer {
    pub fn new(window: &winit::window::Window) -> Self {
        // Initialize wgpu
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            ..Default::default()
        })).unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::MULTI_DRAW_INDIRECT,
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        )).unwrap();

        // Create render pipeline
        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/pbr.wgsl"));
        let pipeline = create_pbr_pipeline(&device, &shader);

        // ... setup camera, buffers, etc.

        Self { device, queue, surface, /* ... */ }
    }

    /// Update world incrementally (called every frame)
    pub fn update(&mut self, new_chunk: &WorldChunk) {
        // Convert procedural geometry to GPU buffers
        for element in &new_chunk.elements {
            let mesh_buffer = self.create_mesh_buffer(element);
            self.mesh_buffers.push(mesh_buffer);
        }

        // Remove old chunks (keep only last 60 seconds)
        if self.mesh_buffers.len() > 1000 {
            self.mesh_buffers.drain(0..100); // Remove oldest
        }
    }

    /// Render current frame (16ms budget @ 60 FPS)
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(/* ... */),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.pipeline);

            // Render all visible meshes
            for mesh_buffer in &self.mesh_buffers {
                if self.camera.is_visible(&mesh_buffer.bounds) {
                    mesh_buffer.draw(&mut render_pass);
                }
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    fn create_mesh_buffer(&self, element: &WorldElement) -> MeshBuffer {
        // Convert procedural mesh to GPU buffer
        let vertices = element.geometry.vertices();
        let indices = element.geometry.indices();

        let vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        MeshBuffer {
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,
            bounds: element.geometry.bounding_box(),
        }
    }
}

struct MeshBuffer {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
    bounds: BoundingBox,
}

struct Camera {
    position: Vec3,
    target: Vec3,
    up: Vec3,
    fov: f32,
    aspect: f32,
}
```

**PBR Shader (shaders/pbr.wgsl):**
```wgsl
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
}

struct CameraUniform {
    view_proj: mat4x4<f32>,
    position: vec3<f32>,
}

@group(0) @binding(0) var<uniform> camera: CameraUniform;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.world_position = in.position;
    out.world_normal = in.normal;
    out.uv = in.uv;
    out.clip_position = camera.view_proj * vec4<f32>(in.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // PBR lighting calculation
    let N = normalize(in.world_normal);
    let V = normalize(camera.position - in.world_position);
    let L = normalize(vec3<f32>(1.0, 1.0, 1.0)); // Light direction

    let NdotL = max(dot(N, L), 0.0);
    let diffuse = vec3<f32>(0.8, 0.5, 0.3) * NdotL;

    let ambient = vec3<f32>(0.1, 0.1, 0.1);
    let color = ambient + diffuse;

    return vec4<f32>(color, 1.0);
}
```

---

## 🔧 INTEGRATION WITH EXISTING CODE

### Modify omega-synesthesia/src/lib.rs

**Add Real-Time Mode:**
```rust
use omega_synesthesia_streaming::AudioInputStream;
use omega_synesthesia_renderer::SynesthesiaRenderer;

pub struct SynesthesiaEngine {
    // Existing fields...
    genre: Genre,
    mindscape: Arc<RwLock<MindscapeExplorer>>,

    // NEW: Real-time components
    mode: EngineMode,
    audio_stream: Option<AudioInputStream>,
    renderer: Option<SynesthesiaRenderer>,
}

pub enum EngineMode {
    Offline,   // Existing behavior
    RealTime,  // New streaming mode
}

impl SynesthesiaEngine {
    /// NEW: Create real-time engine
    pub fn new_realtime(genre: Genre, audio_source: AudioSource) -> Result<Self> {
        let audio_stream = AudioInputStream::new(audio_source)?;
        let renderer = None; // Created when window is available

        Ok(Self {
            genre,
            mindscape: Arc::new(RwLock::new(MindscapeExplorer::new())),
            mode: EngineMode::RealTime,
            audio_stream: Some(audio_stream),
            renderer,
            // ... other fields
        })
    }

    /// NEW: Main real-time loop
    pub async fn run_realtime(&mut self, window: &winit::window::Window) -> Result<()> {
        let mut renderer = SynesthesiaRenderer::new(window);
        let mut audio_stream = self.audio_stream.take().unwrap();

        let mut analyzer = AudioAnalyzer::new(44100);
        let mut feature_extractor = FeatureExtractor::new();
        let mut world_generator = WorldGenerator::new(self.genre);

        loop {
            // 1. Read audio chunk (11.6ms)
            let audio_chunk = match audio_stream.read_chunk(512) {
                Some(chunk) => chunk,
                None => {
                    tokio::time::sleep(Duration::from_millis(5)).await;
                    continue;
                }
            };

            // 2. Analyze audio (5ms)
            let spectral = analyzer.analyze_frame(&audio_chunk)?;

            // 3. Extract features (1ms)
            let features = feature_extractor.extract(&spectral)?;

            // 4. Generate world chunk (3ms)
            let world_chunk = world_generator.generate_incremental(&features)?;

            // 5. Update renderer (immediate, GPU async)
            renderer.update(&world_chunk);

            // 6. Render frame (16ms @ 60 FPS)
            renderer.render()?;

            // Total: ~23ms latency, 60 FPS rendering
        }
    }
}
```

---

## 🚀 QUICK START EXAMPLE

**New Example: examples/realtime_demo.rs**
```rust
use omega_synesthesia::{SynesthesiaEngine, Genre, AudioSource};
use winit::{
    event::*,
    event_loop::{EventLoop, ControlFlow},
    window::WindowBuilder,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create event loop and window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("omega-synesthesia: Real-Time Music Visualization")
        .with_inner_size(winit::dpi::LogicalSize::new(1920, 1080))
        .build(&event_loop)?;

    // Create real-time engine
    println!("Initializing real-time synesthesia engine...");
    let mut engine = SynesthesiaEngine::new_realtime(
        Genre::Electronic,
        AudioSource::SystemAudio,  // Visualize what's playing on your computer
    )?;

    println!("Starting visualization...");
    println!("Playing music on your computer will generate the world!");

    // Run real-time loop in separate task
    let window_clone = &window;
    tokio::spawn(async move {
        engine.run_realtime(window_clone).await.unwrap();
    });

    // Window event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.virtual_keycode == Some(VirtualKeyCode::Escape) {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
```

**Usage:**
```bash
# 1. Play music on Spotify/YouTube/etc.
# 2. Run the example
cargo run --example realtime_demo --release

# 3. Watch music transform into 3D world in real-time!
```

---

## 📊 PERFORMANCE OPTIMIZATION CHECKLIST

### Audio Processing
- [x] Use SIMD for FFT (rustfft already uses SIMD)
- [ ] Parallel feature extraction (rayon)
- [ ] Reuse FFT buffers (avoid allocations)
- [ ] Profile with `perf` / `flamegraph`

### World Generation
- [ ] Object pooling for geometry
- [ ] Incremental mesh updates (delta encoding)
- [ ] Spatial hashing for visibility
- [ ] Frustum culling

### Rendering
- [ ] GPU instancing for repeated shapes
- [ ] Mipmaps for textures
- [ ] LOD switching based on distance
- [ ] Occlusion culling
- [ ] Multi-threaded command buffer generation

### Memory
- [ ] Ring buffers for audio (lock-free)
- [ ] Arena allocators for temp data
- [ ] Memory pool for meshes
- [ ] Lazy texture loading

---

## 🧪 TESTING STRATEGY

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_stream_chunk_size() {
        let stream = AudioInputStream::new(AudioSource::Microphone).unwrap();
        let chunk = stream.read_chunk(512);
        assert_eq!(chunk.map(|c| c.len()), Some(512));
    }

    #[test]
    fn test_incremental_generation() {
        let mut gen = WorldGenerator::new(Genre::Electronic);
        let chunk1 = gen.generate_incremental(&features1).unwrap();
        let chunk2 = gen.generate_incremental(&features2).unwrap();

        // Chunks should be continuous
        assert!(chunk2.timestamp > chunk1.timestamp);
    }
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_end_to_end_latency() {
    let mut engine = SynesthesiaEngine::new_realtime(Genre::Jazz, AudioSource::Microphone)?;

    let start = Instant::now();

    // Process 1 second of audio
    for _ in 0..86 {  // 86 chunks @ 11.6ms = 1 second
        engine.process_chunk().await?;
    }

    let elapsed = start.elapsed();
    assert!(elapsed < Duration::from_millis(1100)); // <10% overhead
}
```

### Benchmarks
```bash
cargo bench --bench realtime_performance

# Expected results:
# audio_input       time: [10.2 ms 10.5 ms 10.8 ms]
# fft_analysis      time: [4.8 ms 5.1 ms 5.4 ms]
# feature_extract   time: [0.9 ms 1.0 ms 1.1 ms]
# world_gen         time: [2.5 ms 2.8 ms 3.1 ms]
# render_frame      time: [14.5 ms 16.2 ms 18.1 ms]
```

---

## 🎯 MILESTONES & TIMELINE

### Week 1: Foundation
- [ ] Create `omega-synesthesia-streaming` crate
- [ ] Implement cpal audio input
- [ ] Ring buffer with lock-free SPSC
- [ ] Benchmark audio throughput (target: <5% CPU)

### Week 2: Rendering
- [ ] Create `omega-synesthesia-renderer` crate
- [ ] wgpu initialization and pipeline
- [ ] PBR shader implementation
- [ ] Dynamic mesh buffer management
- [ ] Benchmark rendering (target: 60 FPS @ 1080p)

### Week 3: Integration & Polish
- [ ] Integrate streaming with existing analysis
- [ ] Incremental world generation
- [ ] Camera controls (keyboard/mouse)
- [ ] Real-time demo example
- [ ] End-to-end latency measurement (target: <25ms)

### Week 4: Testing & Documentation
- [ ] Unit tests (80%+ coverage)
- [ ] Integration tests
- [ ] Performance benchmarks
- [ ] User documentation
- [ ] Video demo recording

---

## 📈 SUCCESS CRITERIA

| Metric | Target | Stretch Goal |
|--------|--------|--------------|
| **Latency** | <25ms | <15ms |
| **FPS** | 60 | 120 |
| **CPU Usage** | <20% | <10% |
| **Memory** | <100MB | <50MB |
| **Startup Time** | <2s | <1s |
| **Chunk Size** | 512 samples | 256 samples |

---

## 🚨 RISK MITIGATION

### Technical Risks

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| **Audio latency too high** | High | Medium | Use smaller chunks (256), optimize FFT |
| **GPU memory overflow** | High | Low | Implement aggressive culling, LOD |
| **CPU bottleneck** | Medium | Medium | Parallel processing, SIMD |
| **Platform compatibility** | Medium | Medium | Test on Windows/macOS/Linux early |

### Fallback Plan

If real-time proves too difficult initially:
1. **Pseudo-real-time**: 50-100ms latency (still feels responsive)
2. **Offline-first**: Generate world ahead, stream rendering
3. **Hybrid**: Real-time for simple genres, offline for complex

---

## 🎉 DEMO SCRIPT

**For stakeholders/investors:**

1. **Open terminal, run demo**
   ```bash
   cargo run --example realtime_demo --release
   ```

2. **Play music (Spotify, YouTube, etc.)**

3. **Show transformation in real-time:**
   - "Notice how the world builds itself as the song plays"
   - "The colors match the mood - bright for happy, dark for sad"
   - "These spikes are the drums, the flowing shapes are the melody"
   - "Watch what happens at the drop..."

4. **Interactive elements:**
   - Use mouse to look around
   - Press W/A/S/D to fly through the world
   - Press Space to pause time
   - Press R to change genre style on-the-fly

5. **Performance metrics:**
   - Show FPS counter (60+)
   - Show latency meter (<25ms)
   - Show CPU usage (<20%)

6. **Wow factor:**
   - "This is running on a laptop"
   - "Any song, any genre, instant 3D world"
   - "Imagine this in VR at a concert with 10,000 people"

---

## 📚 ADDITIONAL RESOURCES

### Learning Materials
- [wgpu Tutorial](https://sotrh.github.io/learn-wgpu/)
- [Audio Programming in Rust](https://docs.rs/cpal/latest/cpal/)
- [Real-Time DSP](https://www.dspguide.com/)

### Reference Implementations
- [nannou](https://github.com/nannou-org/nannou) - Creative coding framework
- [bevy](https://github.com/bevyengine/bevy) - Game engine with wgpu
- [cpal examples](https://github.com/RustAudio/cpal/tree/master/examples)

### Tools
- `cargo flamegraph` - Profiling
- `cargo bench` - Benchmarking
- `wgpu-profiler` - GPU profiling

---

## ✅ CONCLUSION

This guide provides a **concrete, actionable path** to transform omega-synesthesia from offline to real-time. The technology is proven, the architecture is sound, and the performance targets are achievable.

**Key Takeaways:**
- Real-time is possible with ~23ms latency
- Requires 2 new crates (streaming + renderer)
- 3-4 weeks for functional prototype
- Foundation for all future features (VR, multiplayer, AI)

**Next Action:** Start with Week 1 - audio streaming foundation.

---

*omega-synesthesia: Real-time music worlds*
*Implementation guide v1.0*
*2025-12-18*
