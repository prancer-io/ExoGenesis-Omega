//! SYNESTHESIA - AI-Driven Music Visualization Platform
//!
//! A breakthrough entertainment platform that transforms music into immersive
//! visual experiences through AI-powered musical understanding.
//!
//! ## Architecture
//!
//! The platform consists of six main subsystems:
//!
//! - **Music**: Core musical understanding (signal, theory, structure, emotion)
//! - **Audio**: Real-time audio analysis, playback, and transcription
//! - **AI**: Semantic parsing, LLM integration, and scene generation
//! - **Revelation**: Progressive visual emergence - the "Painter's Algorithm"
//! - **Render**: GPU-accelerated 3D rendering with wgpu
//! - **UI**: Immediate-mode interface with egui
//!
//! ## Key Features
//!
//! - **Music-Driven Visualization**: Not just frequencies - key, chords, structure
//! - Emotional arc mapping from musical theory
//! - Song structure detection (verse, chorus, bridge, drop)
//! - Progressive revelation - visuals emerge as we understand the music
//! - Real-time lyric transcription with Whisper (optional)
//! - Beat-synchronized visual effects
//! - Open-source video generation (CogVideoX, Mochi, Open-Sora)
//! - VR-ready architecture (OpenXR planned)

pub mod app;
pub mod music;
pub mod audio;
pub mod ai;
pub mod revelation;
pub mod render;
pub mod ui;

pub use app::App;
pub use music::{MusicEngine, MusicUnderstanding, MusicAnalysis};
pub use audio::{AudioPlayer, AudioFeatures, TranscribedWord};
pub use ai::{SemanticParser, SemanticScene};
pub use revelation::{RevelationEngine, ClarityLevel, ClarityBreakdown};
pub use render::{Renderer, Scene, SceneMode};
pub use ui::UI;
