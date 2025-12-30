//! Runtime Player
//!
//! Loads .synth files and plays synchronized audio/visuals.
//! This is the "Pre-Rendered + Real-Time Blend" mode.

mod synth_loader;
mod audio_player;
mod sync;
mod renderer;
mod transitions;

pub use synth_loader::{SynthFile, SynthLoader, MusicAnalysis, Section, Chord, EmotionPoint, ClimaxPoint};
pub use audio_player::AudioPlayer;
pub use sync::SyncEngine;
pub use renderer::PlayerRenderer;
pub use transitions::{TransitionEngine, TransitionType, ActiveTransition, TransitionUniforms, TRANSITION_SHADER};

use crate::music::MusicUnderstanding;
use crate::revelation::ClarityBreakdown;

/// Main player that coordinates everything
pub struct Player {
    /// Loaded .synth file
    synth: Option<SynthFile>,

    /// Audio playback
    audio: AudioPlayer,

    /// Synchronization engine
    sync: SyncEngine,

    /// Transition engine
    transitions: TransitionEngine,

    /// GPU renderer (optional, created when GPU available)
    renderer: Option<PlayerRenderer>,

    /// Current playback time
    current_time: f64,

    /// Is playing
    is_playing: bool,

    /// Current clarity level (0.0 - 1.0)
    clarity: f32,

    /// Clarity breakdown for shader effects
    clarity_breakdown: ClarityBreakdown,
}

impl Player {
    /// Create new player
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            synth: None,
            audio: AudioPlayer::new()?,
            sync: SyncEngine::new(),
            transitions: TransitionEngine::new(),
            renderer: None,
            current_time: 0.0,
            is_playing: false,
            clarity: 0.0,
            clarity_breakdown: ClarityBreakdown::default(),
        })
    }

    /// Initialize GPU renderer
    pub fn init_renderer(
        &mut self,
        device: std::sync::Arc<wgpu::Device>,
        queue: std::sync::Arc<wgpu::Queue>,
        format: wgpu::TextureFormat,
    ) -> anyhow::Result<()> {
        self.renderer = Some(PlayerRenderer::new(device, queue, format)?);
        Ok(())
    }

    /// Load a .synth file
    pub fn load(&mut self, path: &str) -> anyhow::Result<()> {
        log::info!("Loading .synth file: {}", path);
        let synth = SynthFile::load(path)?;

        // Initialize sync engine with analysis
        self.sync.load_analysis(&synth.analysis);

        // Load transitions
        self.transitions.load_transitions(synth.transitions.clone());

        self.synth = Some(synth);
        self.current_time = 0.0;
        self.clarity = 0.0;
        self.clarity_breakdown = ClarityBreakdown::default();

        Ok(())
    }

    /// Load audio file (separate from .synth)
    pub fn load_audio(&mut self, path: &str) -> anyhow::Result<()> {
        self.audio.load(path)?;
        Ok(())
    }

    /// Start playback
    pub fn play(&mut self) {
        if self.synth.is_some() {
            self.audio.play();
            self.is_playing = true;
        }
    }

    /// Pause playback
    pub fn pause(&mut self) {
        self.audio.pause();
        self.is_playing = false;
    }

    /// Stop and reset
    pub fn stop(&mut self) {
        self.audio.stop();
        self.is_playing = false;
        self.current_time = 0.0;
    }

    /// Update player state (call every frame)
    pub fn update(&mut self, delta: f64) -> MusicUnderstanding {
        if self.is_playing {
            self.current_time = self.audio.position();
        }

        // Get real-time audio features
        let audio_features = self.audio.get_features();

        // Build music understanding from sync engine + real-time features
        let understanding = self.sync.get_understanding(self.current_time, &audio_features);

        // Update clarity based on accumulated understanding
        self.update_clarity(&understanding, delta as f32);

        // Update transition engine
        self.transitions.update(self.current_time, delta as f32, &understanding);

        // Update renderer if available
        if let Some(ref mut renderer) = self.renderer {
            renderer.update(
                self.current_time as f32,
                delta as f32,
                &understanding,
                self.clarity,
                &self.clarity_breakdown,
            );
        }

        understanding
    }

    /// Update clarity based on music understanding
    fn update_clarity(&mut self, music: &MusicUnderstanding, delta: f32) {
        // Clarity increases as we understand more about the music
        // Start at 0%, build up over time as patterns emerge

        let base_rate = 0.02; // Base clarity increase per second
        let beat_bonus = if music.signal.beat_strength > 0.5 { 0.05 } else { 0.0 };
        let section_bonus = if music.section_progress < 0.1 { 0.03 } else { 0.0 }; // New section

        let increase = (base_rate + beat_bonus + section_bonus) * delta;
        self.clarity = (self.clarity + increase).min(1.0);

        // Update breakdown
        self.clarity_breakdown.signal = (music.signal.rms * 0.8 + self.clarity_breakdown.signal * 0.2).min(1.0);
        self.clarity_breakdown.theory = (music.theory.key_confidence * 0.5 + self.clarity_breakdown.theory * 0.5).min(1.0);
        self.clarity_breakdown.structure = (music.section_progress + self.clarity_breakdown.structure * 0.9).min(1.0) / 2.0;
        self.clarity_breakdown.emotion = (music.emotion.intensity * 0.7 + self.clarity_breakdown.emotion * 0.3).min(1.0);

        // Reset on song restart
        if self.current_time < 0.1 {
            self.clarity = 0.0;
            self.clarity_breakdown = ClarityBreakdown::default();
        }
    }

    /// Render current frame
    pub fn render(&self, view: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder) {
        if let Some(ref renderer) = self.renderer {
            renderer.render(view, encoder);
        }
    }

    /// Get current clarity level
    pub fn clarity(&self) -> f32 {
        self.clarity
    }

    /// Get clarity breakdown
    pub fn clarity_breakdown(&self) -> &ClarityBreakdown {
        &self.clarity_breakdown
    }

    /// Get transition engine
    pub fn transitions(&self) -> &TransitionEngine {
        &self.transitions
    }

    /// Get mutable transition engine
    pub fn transitions_mut(&mut self) -> &mut TransitionEngine {
        &mut self.transitions
    }

    /// Is currently transitioning?
    pub fn is_transitioning(&self) -> bool {
        self.transitions.is_transitioning()
    }

    /// Get current segment
    pub fn current_segment(&self) -> u32 {
        self.transitions.current_segment()
    }

    /// Get current playback time
    pub fn time(&self) -> f64 {
        self.current_time
    }

    /// Get duration
    pub fn duration(&self) -> f64 {
        self.synth.as_ref()
            .map(|s| s.analysis.duration)
            .unwrap_or(0.0)
    }

    /// Is playing
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    /// Seek to position
    pub fn seek(&mut self, time: f64) {
        self.audio.seek(time);
        self.current_time = time;
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new().expect("Failed to create player")
    }
}

/// Re-export wgpu for convenience
pub use wgpu;
