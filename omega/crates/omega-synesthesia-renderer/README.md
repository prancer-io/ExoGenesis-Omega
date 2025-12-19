# omega-synesthesia-renderer

GPU-accelerated real-time renderer for omega-synesthesia worlds using wgpu.

## Overview

omega-synesthesia-renderer provides high-performance 3D rendering for music visualization:

- **GPU-accelerated rendering** using wgpu (WebGPU standard)
- **Physically-based rendering (PBR)** with metallic-roughness workflow
- **Incremental mesh updates** - add geometry on-the-fly without blocking
- **Cross-platform** - Desktop, Web (WASM), Mobile
- **60+ FPS target** - optimized for real-time performance

## Features

- 🎨 **PBR materials**: Physically-based rendering with Cook-Torrance BRDF
- ⚡ **GPU acceleration**: wgpu for modern graphics APIs (Vulkan, Metal, DX12)
- 🏗️ **Incremental updates**: Add meshes during rendering without blocking
- 🎮 **Camera controls**: WASD movement, mouse orbit, scroll zoom
- 🌈 **Genre-specific materials**: Classical, Jazz, Rock, Electronic, Ambient
- 📱 **Cross-platform**: Windows, macOS, Linux, Web (WASM)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
omega-synesthesia-renderer = { path = "../omega-synesthesia-renderer" }
```

## Quick Start

### Basic Rendering

```rust
use omega_synesthesia_renderer::{
    SynesthesiaRenderer, RenderConfig, Mesh, PbrMaterial
};
use winit::event_loop::EventLoop;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    let config = RenderConfig::default();

    // Create renderer
    let mut renderer = SynesthesiaRenderer::new(&event_loop, config).await?;

    // Create a cube mesh
    let cube = Mesh::cube(2.0, [0.8, 0.3, 0.2, 1.0]);
    let material = PbrMaterial::metallic([0.8, 0.3, 0.2], 0.3);

    // Add to scene
    renderer.add_mesh(cube, material)?;

    // Render loop
    event_loop.run(move |event, target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => {
                renderer.handle_event(&event);
            }
            winit::event::Event::MainEventsCleared => {
                renderer.render()?;
            }
            _ => {}
        }
    })?;

    Ok(())
}
```

### Genre-Specific Materials

```rust
use omega_synesthesia_renderer::material::MaterialPresets;

// Classical music - elegant marble
let marble = MaterialPresets::marble();
renderer.add_mesh(mesh, marble)?;

// Jazz - warm wood
let wood = MaterialPresets::wood();
renderer.add_mesh(mesh, wood)?;

// Electronic - neon metal with emission
let neon = MaterialPresets::neon_metal();
renderer.add_mesh(mesh, neon)?;
```

### Camera Control

```rust
use omega_synesthesia_renderer::Camera;
use glam::Vec3;

// Update camera position
renderer.update_camera(
    Vec3::new(0.0, 10.0, 20.0),  // position
    Vec3::ZERO                    // target
);

// Camera automatically handles:
// - WASD movement
// - Mouse drag to orbit
// - Scroll to zoom
// - Smooth damping
```

## Architecture

```
Mesh + Material → GPU Upload → PBR Shader → Frame → Present
     (CPU)         (wgpu)       (WGSL)     (60 FPS)  (Screen)
```

### Rendering Pipeline

1. **Mesh Creation** (CPU)
   - Vertices: position, normal, UV, color
   - Indices: triangle connectivity
   - Material: PBR parameters

2. **GPU Upload** (wgpu)
   - Vertex buffer creation
   - Index buffer creation
   - Material uniform buffer
   - Bind group creation

3. **PBR Shader** (WGSL)
   - Cook-Torrance BRDF
   - GGX normal distribution
   - Fresnel-Schlick approximation
   - HDR tonemapping
   - Gamma correction

4. **Frame Rendering** (60 FPS)
   - Camera matrix update
   - For each mesh:
     - Set material bind group
     - Draw indexed triangles
   - Present to screen

## API Reference

### `RenderConfig`

Configuration for the renderer.

```rust
pub struct RenderConfig {
    pub width: u32,          // Window width (default: 1280)
    pub height: u32,         // Window height (default: 720)
    pub title: String,       // Window title
    pub target_fps: u32,     // Target FPS (0 = unlimited)
    pub vsync: bool,         // Enable VSync
    pub msaa_samples: u32,   // MSAA samples (1, 2, 4, 8)
}

impl RenderConfig {
    pub fn default() -> Self;  // 1280x720, 60 FPS, VSync, 4x MSAA
}
```

### `SynesthesiaRenderer`

Main renderer struct.

```rust
pub struct SynesthesiaRenderer { /* ... */ }

impl SynesthesiaRenderer {
    pub async fn new(
        event_loop: &EventLoop<()>,
        config: RenderConfig
    ) -> Result<Self>;

    pub fn add_mesh(&self, mesh: Mesh, material: PbrMaterial) -> Result<()>;
    pub fn update_camera(&mut self, position: Vec3, target: Vec3);
    pub fn render(&mut self) -> Result<()>;
    pub fn fps(&self) -> f32;
    pub fn window(&self) -> &Window;
}
```

### `Mesh`

3D mesh with vertices and indices.

```rust
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub name: String,
}

impl Mesh {
    pub fn new(name: String, vertices: Vec<Vertex>, indices: Vec<u32>) -> Self;
    pub fn cube(size: f32, color: [f32; 4]) -> Self;
    pub fn sphere(radius: f32, subdivisions: u32, color: [f32; 4]) -> Self;
    pub fn vertex_count(&self) -> usize;
    pub fn triangle_count(&self) -> usize;
}
```

### `Vertex`

Vertex format for meshes.

```rust
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],  // 3D position
    pub normal: [f32; 3],    // Surface normal
    pub uv: [f32; 2],        // Texture coordinates
    pub color: [f32; 4],     // RGBA color
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, uv: Vec2, color: [f32; 4]) -> Self;
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}
```

### `PbrMaterial`

Physically-based material.

```rust
pub struct PbrMaterial {
    pub base_color: [f32; 4],    // Albedo (RGBA)
    pub metallic: f32,           // 0.0 = dielectric, 1.0 = metal
    pub roughness: f32,          // 0.0 = smooth, 1.0 = rough
    pub emission: [f32; 3],      // RGB emission
    pub emission_strength: f32,  // Emission multiplier
    pub normal_strength: f32,    // Normal map intensity
    pub ao_strength: f32,        // Ambient occlusion
}

impl PbrMaterial {
    pub fn default() -> Self;
    pub fn matte(color: [f32; 3]) -> Self;
    pub fn metallic(color: [f32; 3], roughness: f32) -> Self;
    pub fn glossy(color: [f32; 3]) -> Self;
    pub fn emissive(color: [f32; 3], strength: f32) -> Self;
}
```

### `MaterialPresets`

Genre-specific material presets.

```rust
pub struct MaterialPresets;

impl MaterialPresets {
    pub fn marble() -> PbrMaterial;         // Classical
    pub fn wood() -> PbrMaterial;           // Jazz
    pub fn volcanic_rock() -> PbrMaterial;  // Rock
    pub fn neon_metal() -> PbrMaterial;     // Electronic
    pub fn ethereal() -> PbrMaterial;       // Ambient
}
```

### `Camera`

3D perspective camera.

```rust
pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(position: Vec3, target: Vec3, fov: f32, aspect: f32) -> Self;
    pub fn view_matrix(&self) -> Mat4;
    pub fn projection_matrix(&self) -> Mat4;
    pub fn move_forward(&mut self, distance: f32);
    pub fn move_right(&mut self, distance: f32);
    pub fn orbit(&mut self, yaw: f32, pitch: f32);
}
```

## PBR Shader Details

### Cook-Torrance BRDF

The renderer implements physically-based rendering using the Cook-Torrance BRDF:

```
f(l, v) = (D * G * F) / (4 * (n·l) * (n·v))
```

Where:
- **D**: GGX normal distribution function
- **G**: Schlick-GGX geometry function
- **F**: Fresnel-Schlick approximation

### WGSL Shader Pipeline

**Vertex Shader:**
```wgsl
@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    out.clip_position = camera.view_proj * vec4<f32>(vertex.position, 1.0);
    out.world_position = vertex.position;
    out.normal = normalize(vertex.normal);
    return out;
}
```

**Fragment Shader:**
```wgsl
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // PBR calculations
    let NDF = distribution_ggx(N, H, roughness);
    let G = geometry_smith(N, V, L, roughness);
    let F = fresnel_schlick(max(dot(H, V), 0.0), F0);

    let specular = (NDF * G * F) / (4 * NdotV * NdotL + 0.0001);
    let Lo = (kD * albedo / PI + specular) * radiance * NdotL;

    // Tonemapping + gamma correction
    color = color / (color + 1.0);  // Reinhard
    color = pow(color, 1.0 / 2.2);   // Gamma
}
```

## Performance

### Target Metrics

| Metric | Target | Typical |
|--------|--------|---------|
| Frame Rate | 60 FPS | 60-120 FPS |
| Frame Time | 16.7ms | 10-15ms |
| Mesh Upload | <1ms | 0.5ms |
| Draw Call | <0.1ms | 0.05ms |

### Optimization Techniques

- **Batching**: Group meshes by material to reduce bind group switches
- **Instancing**: Planned for repeated geometry
- **LOD**: Planned for distant objects
- **Frustum Culling**: Planned for large scenes

### Memory Usage

Per mesh:
```
Vertices: 1000 verts * 48 bytes = 48 KB
Indices: 3000 indices * 4 bytes = 12 KB
Material: 64 bytes (uniform buffer)
Total: ~60 KB per mesh
```

For 100 active meshes: **~6 MB GPU memory**

## Integration with omega-synesthesia-streaming

```rust
use omega_synesthesia_renderer::{SynesthesiaRenderer, Mesh, PbrMaterial};
use omega_synesthesia_streaming::{AudioInputStream, FeatureExtractor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up renderer
    let event_loop = EventLoop::new()?;
    let config = RenderConfig::default();
    let mut renderer = SynesthesiaRenderer::new(&event_loop, config).await?;

    // Set up audio streaming
    let stream_config = StreamConfig::low_latency();
    let mut stream = AudioInputStream::new(AudioSource::Microphone, stream_config.clone()).await?;
    let mut extractor = FeatureExtractor::new(stream_config.sample_rate, stream_config.chunk_size);

    stream.start()?;

    // Real-time audio-to-visual pipeline
    let mut frame_counter = 0;

    event_loop.run(move |event, target| {
        match event {
            Event::MainEventsCleared => {
                // Get audio chunk (non-blocking)
                if let Some(chunk) = stream.try_next_chunk() {
                    let features = extractor.extract(&chunk)?;

                    // Generate mesh from features
                    let size = 1.0 + features.rms_energy * 5.0;
                    let color_hue = (features.spectral_centroid / 4000.0).min(1.0);
                    let color = [color_hue, 1.0 - color_hue, 0.5, 1.0];

                    let mesh = if features.beat_confidence > 0.7 {
                        Mesh::sphere(size, 2, color)
                    } else {
                        Mesh::cube(size, color)
                    };

                    // Create material based on energy
                    let material = if features.rms_energy > 0.5 {
                        PbrMaterial::emissive(color, features.rms_energy * 2.0)
                    } else {
                        PbrMaterial::metallic(color, 0.3)
                    };

                    // Add to scene (incremental, non-blocking)
                    renderer.add_mesh(mesh, material)?;
                }

                // Render frame (60 FPS)
                renderer.render()?;
                frame_counter += 1;
            }
            _ => {}
        }
    })?;

    Ok(())
}
```

## Examples

See `omega-examples/` for:
- `basic_renderer.rs` - Simple cube rendering
- `material_showcase.rs` - All material presets
- `realtime_audio_visual.rs` - Live music visualization
- `camera_controls.rs` - Interactive camera demo

## Troubleshooting

### "Failed to request adapter"

No compatible graphics adapter found. Check:
- Update graphics drivers
- Try fallback adapter: `force_fallback_adapter: true`

### Low FPS

- Reduce MSAA samples (4x → 2x or 1x)
- Lower window resolution
- Disable VSync for uncapped FPS

### Shader compilation errors

- Check wgpu version compatibility
- Ensure WGSL shader syntax is correct
- Review browser console (WASM)

## Future Enhancements

- [ ] Depth buffer and z-fighting prevention
- [ ] Shadow mapping
- [ ] Instanced rendering
- [ ] LOD system
- [ ] Frustum culling
- [ ] Post-processing effects (bloom, SSAO)
- [ ] Texture support
- [ ] Normal mapping
- [ ] IBL (Image-Based Lighting)

## License

MIT License - See LICENSE file for details

## Contributing

Contributions welcome! Please see CONTRIBUTING.md for guidelines.

## References

- [wgpu](https://wgpu.rs/) - WebGPU implementation in Rust
- [Learn wgpu](https://sotrh.github.io/learn-wgpu/) - Excellent wgpu tutorial
- [PBR Theory](https://learnopengl.com/PBR/Theory) - PBR rendering explained
- [WGSL Spec](https://www.w3.org/TR/WGSL/) - WebGPU Shading Language specification
