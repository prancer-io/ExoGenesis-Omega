//! # Omega Synesthesia - Convert Music into Walkable 3D Worlds
//!
//! The Synesthesia Engine transforms audio into immersive, navigable 3D environments
//! where you can literally walk through a symphony and touch the notes.
//!
//! ## Core Concepts
//!
//! - **Audio Features**: Extract frequency, amplitude, timbre, rhythm from audio
//! - **Spatial Mapping**: Convert musical features to 3D coordinates and shapes
//! - **Genre Styles**: Different musical genres create distinct world aesthetics
//! - **Temporal Navigation**: Walk forward through time, experiencing music spatially
//! - **Emotional Terrain**: Valleys of sadness, peaks of joy, storms of intensity
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                        SYNESTHESIA ENGINE                                │
//! ├─────────────────────────────────────────────────────────────────────────┤
//! │                                                                          │
//! │  ┌──────────────┐     ┌──────────────┐     ┌──────────────────────────┐ │
//! │  │   AUDIO      │     │   FEATURE    │     │   WORLD                  │ │
//! │  │   ANALYZER   │────►│   MAPPER     │────►│   GENERATOR              │ │
//! │  │              │     │              │     │                          │ │
//! │  │  FFT         │     │  Pitch→Y     │     │  Meshes                  │ │
//! │  │  Beats       │     │  Time→X      │     │  Materials               │ │
//! │  │  Timbre      │     │  Volume→Scale│     │  Lights                  │ │
//! │  │  Emotion     │     │  Timbre→Color│     │  Particles               │ │
//! │  └──────────────┘     └──────────────┘     └──────────────────────────┘ │
//! │                                                     │                    │
//! │                              ┌──────────────────────┴────────────────┐  │
//! │                              ▼                                       ▼  │
//! │                    ┌──────────────────┐              ┌──────────────────┐│
//! │                    │  BEVY RENDERER   │              │  GLTF EXPORT     ││
//! │                    │  (Real-time)     │              │  (Unreal/Blender)││
//! │                    └──────────────────┘              └──────────────────┘│
//! │                                                                          │
//! └─────────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use omega_synesthesia::{SynesthesiaEngine, AudioSource, Genre};
//!
//! // Create engine with classical style
//! let mut engine = SynesthesiaEngine::new(Genre::Classical);
//!
//! // Load audio
//! engine.load_audio(AudioSource::File("beethoven_9th.mp3"))?;
//!
//! // Generate 3D world
//! let world = engine.generate_world()?;
//!
//! // Export to glTF for Unreal Engine
//! world.export_gltf("beethoven_world.gltf")?;
//!
//! // Or render in real-time with Bevy
//! engine.run_realtime()?;
//! ```

pub mod audio;
pub mod features;
pub mod mapping;
pub mod world;
pub mod genre;
pub mod geometry;
pub mod materials;
pub mod export;

pub use audio::{AudioAnalyzer, AudioSource, AudioFrame, SpectralData, TestSignalType};
pub use features::{MusicalFeatures, FeatureExtractor, EmotionalValence};
pub use mapping::{SpatialMapper, MappingConfig, Coordinate3D};
pub use world::{SynesthesiaWorld, WorldChunk, WorldElement};
pub use genre::{Genre, GenreStyle, StylePreset};
pub use geometry::{MeshGenerator, Primitive, ProceduralMesh};
pub use materials::{MaterialPalette, SynMaterial, TextureType};
pub use export::{GltfExporter, ExportConfig};

use omega_mindscape::MindscapeExplorer;
use parking_lot::RwLock;
use std::sync::Arc;
use thiserror::Error;

/// Errors in synesthesia processing
#[derive(Debug, Error)]
pub enum SynesthesiaError {
    #[error("Audio loading failed: {0}")]
    AudioLoadError(String),

    #[error("Feature extraction failed: {0}")]
    FeatureError(String),

    #[error("World generation failed: {0}")]
    WorldGenError(String),

    #[error("Export failed: {0}")]
    ExportError(String),

    #[error("Rendering failed: {0}")]
    RenderError(String),

    #[error("Invalid configuration: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, SynesthesiaError>;

/// Configuration for the Synesthesia Engine
#[derive(Debug, Clone)]
pub struct SynesthesiaConfig {
    /// Musical genre/style for world generation
    pub genre: Genre,
    /// Samples per second for audio analysis
    pub sample_rate: u32,
    /// FFT window size
    pub fft_size: usize,
    /// World scale (units per second of audio)
    pub time_scale: f32,
    /// Height scale for pitch mapping
    pub pitch_scale: f32,
    /// Enable volumetric effects
    pub volumetrics: bool,
    /// Enable particle systems
    pub particles: bool,
    /// Export quality (0.0-1.0)
    pub quality: f32,
}

impl Default for SynesthesiaConfig {
    fn default() -> Self {
        Self {
            genre: Genre::Classical,
            sample_rate: 44100,
            fft_size: 2048,
            time_scale: 10.0,      // 10 units per second
            pitch_scale: 50.0,     // 50 units from lowest to highest
            volumetrics: true,
            particles: true,
            quality: 1.0,
        }
    }
}

/// The main Synesthesia Engine
///
/// Orchestrates the transformation of audio into 3D worlds
pub struct SynesthesiaEngine {
    config: SynesthesiaConfig,

    /// Audio analyzer
    analyzer: Arc<RwLock<AudioAnalyzer>>,

    /// Feature extractor
    feature_extractor: Arc<RwLock<FeatureExtractor>>,

    /// Spatial mapper
    mapper: Arc<RwLock<SpatialMapper>>,

    /// World generator
    world: Arc<RwLock<Option<SynesthesiaWorld>>>,

    /// Mindscape integration for navigation
    mindscape: Arc<RwLock<MindscapeExplorer>>,

    /// Current playback position (seconds) - for future real-time playback
    #[allow(dead_code)]
    playback_position: f64,

    /// Total duration (seconds)
    duration: f64,
}

impl SynesthesiaEngine {
    /// Create a new Synesthesia Engine with specified genre
    pub fn new(genre: Genre) -> Self {
        let config = SynesthesiaConfig {
            genre,
            ..Default::default()
        };
        Self::with_config(config)
    }

    /// Create with custom configuration
    pub fn with_config(config: SynesthesiaConfig) -> Self {
        Self {
            analyzer: Arc::new(RwLock::new(AudioAnalyzer::new(config.sample_rate, config.fft_size))),
            feature_extractor: Arc::new(RwLock::new(FeatureExtractor::new())),
            mapper: Arc::new(RwLock::new(SpatialMapper::new(config.time_scale, config.pitch_scale))),
            world: Arc::new(RwLock::new(None)),
            mindscape: Arc::new(RwLock::new(MindscapeExplorer::new())),
            config,
            playback_position: 0.0,
            duration: 0.0,
        }
    }

    /// Load audio from a source
    pub fn load_audio(&mut self, source: AudioSource) -> Result<()> {
        let mut analyzer = self.analyzer.write();
        analyzer.load(source)?;
        self.duration = analyzer.duration();
        Ok(())
    }

    /// Generate the complete 3D world from loaded audio
    pub fn generate_world(&mut self) -> Result<SynesthesiaWorld> {
        println!("🎵 Generating Synesthesia World...");
        println!("   Genre: {:?}", self.config.genre);
        println!("   Duration: {:.1}s", self.duration);

        // Extract features from audio
        println!("\n📊 Extracting musical features...");
        let features = {
            let analyzer = self.analyzer.read();
            let mut extractor = self.feature_extractor.write();
            extractor.extract_all(&analyzer)?
        };

        println!("   Extracted {} feature frames", features.len());

        // Map features to 3D coordinates
        println!("\n🗺️  Mapping to 3D space...");
        let spatial_data = {
            let mapper = self.mapper.read();
            mapper.map_features(&features, &self.config.genre.get_style())?
        };

        // Generate world geometry
        println!("\n🏗️  Building world geometry...");
        let style = self.config.genre.get_style();
        let mut world = SynesthesiaWorld::new(style.clone());

        world.generate_from_spatial_data(&spatial_data, &style)?;

        // Store memories in mindscape for navigation
        println!("\n🧠 Integrating with mindscape...");
        self.integrate_with_mindscape(&world, &features)?;

        // Store world
        {
            let mut world_lock = self.world.write();
            *world_lock = Some(world.clone());
        }

        println!("\n✅ World generation complete!");
        println!("   Chunks: {}", world.chunks.len());
        println!("   Elements: {}", world.total_elements());
        println!("   Vertices: ~{}", world.estimated_vertices());

        Ok(world)
    }

    /// Integrate world landmarks with mindscape for navigation
    fn integrate_with_mindscape(&self, _world: &SynesthesiaWorld, features: &[MusicalFeatures]) -> Result<()> {
        let mindscape = self.mindscape.write();

        // Create embeddings from musical features and store as memories
        for (i, feature) in features.iter().enumerate() {
            if i % 100 == 0 {  // Sample every 100th frame
                let embedding = feature.to_embedding();
                let label = format!("moment_{:.1}s", feature.timestamp);
                let _ = mindscape.remember(&label, &embedding);
            }
        }

        Ok(())
    }

    /// Export the world to glTF format
    pub fn export_gltf(&self, path: &str) -> Result<()> {
        let world = self.world.read();
        let world = world.as_ref().ok_or_else(|| {
            SynesthesiaError::ExportError("No world generated yet".to_string())
        })?;

        let exporter = GltfExporter::new(ExportConfig::default());
        exporter.export(world, path)?;

        Ok(())
    }

    /// Get current world
    pub fn world(&self) -> Option<SynesthesiaWorld> {
        self.world.read().clone()
    }

    /// Navigate to a specific time in the world
    pub fn navigate_to_time(&mut self, time_seconds: f64) -> Result<Coordinate3D> {
        let mapper = self.mapper.read();
        Ok(mapper.time_to_position(time_seconds))
    }

    /// Get the mindscape explorer for navigation
    pub fn mindscape(&self) -> Arc<RwLock<MindscapeExplorer>> {
        self.mindscape.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = SynesthesiaEngine::new(Genre::Classical);
        assert!(engine.world().is_none());
    }

    #[test]
    fn test_config_default() {
        let config = SynesthesiaConfig::default();
        assert_eq!(config.sample_rate, 44100);
        assert_eq!(config.fft_size, 2048);
    }
}
