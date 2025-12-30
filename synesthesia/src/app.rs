//! Main Application State
//!
//! Coordinates all subsystems: audio, AI, rendering, and UI.

use std::sync::Arc;
use std::time::Instant;

use anyhow::Result;
use crossbeam_channel::{Receiver, Sender};
use log::info;
use parking_lot::RwLock;
use winit::{
    dpi::PhysicalSize,
    event::{KeyEvent, WindowEvent},
    keyboard::{Key, NamedKey},
    window::Window,
};

use crate::audio::{AudioPlayer, AudioFeatures, TranscribedWord};
use crate::ai::{SemanticParser, SemanticScene};
use crate::render::{Renderer, Scene};
use crate::ui::UI;

/// Main application state
pub struct App {
    // Core systems
    renderer: Renderer,
    audio_player: AudioPlayer,
    semantic_parser: SemanticParser,
    ui: UI,

    // State
    current_scene: Arc<RwLock<Scene>>,
    audio_features: Arc<RwLock<AudioFeatures>>,

    // Communication channels
    word_rx: Receiver<TranscribedWord>,
    scene_rx: Receiver<SemanticScene>,

    // Timing
    last_frame: Instant,
    frame_count: u64,

    // Playback state
    is_playing: bool,
    song_path: Option<String>,
}

impl App {
    /// Create a new application instance
    pub async fn new(window: &Window, song_path: Option<String>) -> Result<Self> {
        info!("Initializing SYNESTHESIA application...");

        // Create communication channels
        let (word_tx, word_rx) = crossbeam_channel::unbounded();
        let (scene_tx, scene_rx) = crossbeam_channel::unbounded();

        // Initialize renderer
        info!("  → Initializing GPU renderer...");
        let renderer = Renderer::new(window).await?;

        // Initialize audio player
        info!("  → Initializing audio system...");
        let audio_player = AudioPlayer::new(word_tx)?;

        // Initialize AI systems
        info!("  → Initializing AI inference...");
        let semantic_parser = SemanticParser::new(scene_tx)?;

        // Initialize UI
        info!("  → Initializing UI...");
        let ui = UI::new(window, &renderer)?;

        // Create shared state
        let current_scene = Arc::new(RwLock::new(Scene::default()));
        let audio_features = Arc::new(RwLock::new(AudioFeatures::default()));

        info!("Application initialization complete!");

        Ok(Self {
            renderer,
            audio_player,
            semantic_parser,
            ui,
            current_scene,
            audio_features,
            word_rx,
            scene_rx,
            last_frame: Instant::now(),
            frame_count: 0,
            is_playing: false,
            song_path,
        })
    }

    /// Handle window input events
    pub fn handle_input(&mut self, event: &WindowEvent) -> bool {
        self.ui.handle_event(event)
    }

    /// Handle keyboard input
    pub fn handle_keyboard(&mut self, event: &KeyEvent) {
        if !event.state.is_pressed() {
            return;
        }

        match &event.logical_key {
            Key::Named(NamedKey::Space) => {
                self.toggle_playback();
            }
            Key::Named(NamedKey::Escape) => {
                self.stop_playback();
            }
            Key::Character(c) if c == "1" => {
                info!("Switching to Abstract Mode");
                self.current_scene.write().mode = crate::render::SceneMode::Abstract;
            }
            Key::Character(c) if c == "2" => {
                info!("Switching to Narrative Mode");
                self.current_scene.write().mode = crate::render::SceneMode::Narrative;
            }
            _ => {}
        }
    }

    /// Handle window resize
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.renderer.resize(new_size);
    }

    /// Update application state
    pub fn update(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_frame).as_secs_f64();
        self.last_frame = now;
        self.frame_count += 1;

        // Process transcribed words
        while let Ok(word) = self.word_rx.try_recv() {
            self.semantic_parser.process_word(word);
        }

        // Process semantic scene updates
        while let Ok(scene_update) = self.scene_rx.try_recv() {
            self.apply_semantic_scene(scene_update);
        }

        // Update audio features
        if let Some(features) = self.audio_player.get_features() {
            *self.audio_features.write() = features;
        }

        // Update scene with audio reactivity
        {
            let features = self.audio_features.read();
            let mut scene = self.current_scene.write();
            scene.update(delta, &features);
        }

        // Update UI
        self.ui.update(delta, &self.audio_features.read(), &self.current_scene.read());
    }

    /// Render the current frame
    pub fn render(&mut self) -> Result<()> {
        let scene = self.current_scene.read();
        let features = self.audio_features.read();

        self.renderer.render(&scene, &features, &mut self.ui)
    }

    /// Toggle playback state
    fn toggle_playback(&mut self) {
        if self.is_playing {
            self.audio_player.pause();
            self.is_playing = false;
            info!("Playback paused");
        } else {
            if let Some(path) = &self.song_path {
                if let Err(e) = self.audio_player.load(path) {
                    log::error!("Failed to load song: {:?}", e);
                    return;
                }
            }
            self.audio_player.play();
            self.is_playing = true;
            info!("Playback started");
        }
    }

    /// Stop playback
    fn stop_playback(&mut self) {
        self.audio_player.stop();
        self.is_playing = false;
        info!("Playback stopped");
    }

    /// Apply a semantic scene update from AI
    fn apply_semantic_scene(&mut self, semantic: SemanticScene) {
        let mut scene = self.current_scene.write();
        scene.apply_semantic(&semantic);
        info!("Applied semantic scene: {:?}", semantic.mood.primary);
    }

    /// Load a new song
    pub fn load_song(&mut self, path: &str) -> Result<()> {
        self.song_path = Some(path.to_string());
        self.audio_player.load(path)?;
        info!("Loaded song: {}", path);
        Ok(())
    }
}
