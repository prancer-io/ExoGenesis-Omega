//! # omega-synesthesia-renderer
//!
//! GPU-accelerated real-time renderer for omega-synesthesia worlds using wgpu.
//!
//! This crate provides high-performance 3D rendering with PBR materials,
//! targeting 60+ FPS for real-time music visualization.
//!
//! ## Features
//!
//! - **GPU-accelerated rendering**: Uses wgpu (WebGPU standard)
//! - **PBR materials**: Physically-based rendering with metallic/roughness workflow
//! - **Incremental updates**: Add geometry on-the-fly without blocking
//! - **Cross-platform**: Desktop, Web (WASM), Mobile
//! - **60+ FPS target**: Optimized for real-time performance
//!
//! ## Example
//!
//! ```no_run
//! use omega_synesthesia_renderer::{SynesthesiaRenderer, RenderConfig};
//! use winit::event_loop::EventLoop;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let event_loop = EventLoop::new()?;
//! let config = RenderConfig::default();
//!
//! let mut renderer = SynesthesiaRenderer::new(&event_loop, config).await?;
//!
//! // Add geometry incrementally
//! renderer.add_mesh(mesh)?;
//! renderer.update_camera(position, target);
//!
//! // Render loop
//! event_loop.run(move |event, target| {
//!     renderer.handle_event(&event);
//!     renderer.render()?;
//! })?;
//! # Ok(())
//! # }
//! ```

mod renderer;
mod shader;
mod camera;
mod camera_follow;
mod mesh;
mod material;
pub mod particle_system;
pub mod video_export;

pub use renderer::{SynesthesiaRenderer, RenderConfig};
pub use camera::{Camera, CameraController};
pub use camera_follow::{CameraFollowController, FollowMode};
pub use particle_system::{ParticleSystem, ParticleConfig};
pub use video_export::VideoExporter;
pub use mesh::{Mesh, Vertex};
pub use material::{Material, PbrMaterial};

/// Terrain generation configuration
#[derive(Debug, Clone, Copy)]
pub struct TerrainConfig {
    pub resolution: u32,
    pub heightmap_seed: u32,
    pub max_height: f32,
    pub roughness: f32,
    pub wave_amplitude: f32,
    pub wave_frequency: f32,
    pub wave_time: f32,
}

impl Default for TerrainConfig {
    fn default() -> Self {
        Self {
            resolution: 30,
            heightmap_seed: 0,
            max_height: 5.0,
            roughness: 0.5,
            wave_amplitude: 0.0,
            wave_frequency: 0.0,
            wave_time: 0.0,
        }
    }
}

use thiserror::Error;

/// Errors that can occur during rendering
#[derive(Error, Debug)]
pub enum RenderError {
    #[error("Failed to create wgpu surface: {0}")]
    SurfaceCreation(String),

    #[error("Failed to request wgpu adapter: {0}")]
    AdapterRequest(String),

    #[error("Failed to request wgpu device: {0}")]
    DeviceRequest(String),

    #[error("Shader compilation error: {0}")]
    ShaderCompilation(String),

    #[error("Mesh creation error: {0}")]
    MeshCreation(String),

    #[error("Texture loading error: {0}")]
    TextureLoading(String),

    #[error("Render pass error: {0}")]
    RenderPass(String),
}

pub type Result<T> = std::result::Result<T, RenderError>;
