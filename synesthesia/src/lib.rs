//! SYNESTHESIA - AI-Driven Music Visualization Platform
//!
//! A breakthrough entertainment platform that transforms music into immersive
//! visual experiences through AI-powered semantic understanding.
//!
//! ## Architecture
//!
//! The platform consists of five main subsystems:
//!
//! - **Audio**: Real-time audio analysis, playback, and transcription
//! - **AI**: Semantic parsing, LLM integration, and scene generation
//! - **Revelation**: Progressive visual emergence - the "Painter's Algorithm"
//! - **Render**: GPU-accelerated 3D rendering with wgpu
//! - **UI**: Immediate-mode interface with egui
//!
//! ## Key Features
//!
//! - Real-time lyric transcription with Whisper
//! - Semantic scene generation from lyrics
//! - Progressive revelation - visuals emerge like a painting
//! - Beat-synchronized visual effects
//! - Multiple visualization modes (Abstract/Narrative)
//! - Open-source video generation (CogVideoX, Mochi, Open-Sora)
//! - VR-ready architecture (OpenXR planned)

pub mod app;
pub mod audio;
pub mod ai;
pub mod revelation;
pub mod render;
pub mod ui;

pub use app::App;
pub use audio::{AudioPlayer, AudioFeatures, TranscribedWord};
pub use ai::{SemanticParser, SemanticScene};
pub use revelation::{RevelationEngine, ClarityLevel, SemanticAccumulator};
pub use render::{Renderer, Scene, SceneMode};
pub use ui::UI;
