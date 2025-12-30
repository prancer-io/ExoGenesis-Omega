//! SYNESTHESIA - AI-Driven Music Visualization Platform
//!
//! A breakthrough entertainment platform that transforms music into immersive
//! visual experiences through AI-powered musical understanding.
//!
//! ## Architecture
//!
//! The platform consists of two main modes:
//!
//! ### Full Mode (Original)
//! - **Music**: Core musical understanding (signal, theory, structure, emotion)
//! - **Audio**: Real-time audio analysis, playback, and transcription
//! - **AI**: Semantic parsing, LLM integration, and scene generation
//! - **Revelation**: Progressive visual emergence - the "Painter's Algorithm"
//! - **Render**: GPU-accelerated 3D rendering with wgpu
//! - **UI**: Immediate-mode interface with egui
//!
//! ### Player Mode (Pre-Rendered + Real-Time Blend)
//! - **Player**: Loads .synth files with pre-computed analysis
//! - **Shaders**: Reactive GPU shaders driven by music understanding
//! - **Sync**: Merges pre-analysis with real-time audio features
//!
//! ## Key Features
//!
//! - **Music-Driven Visualization**: Not just frequencies - key, chords, structure
//! - Emotional arc mapping from musical theory
//! - Song structure detection (verse, chorus, bridge, drop)
//! - Progressive revelation - visuals emerge as we understand the music
//! - Beat-synchronized visual effects
//! - Pre-rendered video segments + real-time shader effects

// Core modules
pub mod app;
pub mod music;
pub mod audio;
pub mod ai;
pub mod revelation;
pub mod render;
pub mod ui;

// Player mode (Pre-Rendered + Real-Time Blend)
pub mod player;
pub mod shaders;
pub mod styles;

// Core exports
pub use app::App;
pub use music::{MusicEngine, MusicUnderstanding, MusicAnalysis};
pub use audio::{AudioPlayer, AudioFeatures, TranscribedWord};
pub use ai::{SemanticParser, SemanticScene};
pub use revelation::{RevelationEngine, ClarityLevel, ClarityBreakdown};
pub use render::{Renderer, Scene, SceneMode};
pub use ui::UI;

// Player mode exports
pub use player::{Player, SynthFile, SynthLoader, TransitionEngine, TransitionType};
pub use shaders::{ShaderUniforms, VERTEX_SHADER, FRAGMENT_SHADER, REVELATION_SHADER};
pub use styles::{Style, StyleManager, ColorPalette, EffectParams};
