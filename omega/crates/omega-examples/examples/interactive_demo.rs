//! # Interactive Music Visualization Demo
//!
//! This example provides an interactive demonstration of omega-synesthesia with:
//! - Real-time music visualization
//! - On-screen controls and HUD
//! - Multiple camera modes
//! - Genre switching
//! - Performance metrics display
//!
//! ## Controls
//!
//! - **Space**: Play/Pause
//! - **1-4**: Camera modes (Orbit/Tracking/Cinematic/FirstPerson)
//! - **G**: Cycle through genres
//! - **+/-**: Adjust speed
//! - **R**: Reset
//! - **ESC**: Exit
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example interactive_demo --release
//! ```

use omega_synesthesia::{
    GenreStyle, FeatureBridge, StreamingWorldGenerator,
    MeshConverter,
};
use omega_synesthesia_renderer::{
    Renderer, RendererConfig, Camera, CameraMode, CameraController,
};
use glam::{Vec3, Mat4};
use winit::{
    event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode},
    event_loop::{EventLoop, ControlFlow},
    window::WindowBuilder,
};
use std::f32::consts::PI;
use std::time::{Instant, Duration};

/// Audio simulation for demo
struct DemoAudioGenerator {
    sample_rate: u32,
    sample_counter: u64,
    chunk_size: usize,
    frequency_offset: f32,
    is_playing: bool,
}

impl DemoAudioGenerator {
    fn new(sample_rate: u32, chunk_size: usize) -> Self {
        Self {
            sample_rate,
            sample_counter: 0,
            chunk_size,
            frequency_offset: 0.0,
            is_playing: true,
        }
    }

    fn generate_musical_chunk(&mut self) -> Vec<f32> {
        if !self.is_playing {
            return vec![0.0; self.chunk_size];
        }

        let mut samples = vec![0.0; self.chunk_size];
        for i in 0..self.chunk_size {
            let t = (self.sample_counter + i as u64) as f32 / self.sample_rate as f32;

            // Create a more musical pattern
            let base_freq = 220.0 + self.frequency_offset;
            let fundamental = (2.0 * PI * base_freq * t).sin();
            let harmonic2 = 0.5 * (2.0 * PI * base_freq * 2.0 * t).sin();
            let harmonic3 = 0.25 * (2.0 * PI * base_freq * 3.0 * t).sin();

            // Add rhythm
            let beat_freq = 2.0; // 120 BPM
            let beat_envelope = (2.0 * PI * beat_freq * t).sin().abs().powf(2.0);

            // Add melody variation
            let melody = (2.0 * PI * 0.5 * t).sin() * 100.0;
            let melody_tone = (2.0 * PI * (base_freq + melody) * t).sin() * 0.3;

            samples[i] = ((fundamental + harmonic2 + harmonic3) * beat_envelope + melody_tone) * 0.3;
        }

        self.sample_counter += self.chunk_size as u64;
        self.frequency_offset = (self.frequency_offset + 0.5) % 200.0;

        samples
    }

    fn extract_features(&self, samples: &[f32]) -> (f32, f32, f32, f32, f32, f32, Option<f32>, Vec<f32>) {
        let rms = (samples.iter().map(|s| s * s).sum::<f32>() / samples.len() as f32).sqrt();
        let spectrum = simple_fft(samples, self.sample_rate);
        let spectral_centroid = calculate_centroid(&spectrum, self.sample_rate);
        let zcr = samples.windows(2)
            .filter(|w| (w[0] >= 0.0 && w[1] < 0.0) || (w[0] < 0.0 && w[1] >= 0.0))
            .count() as f32 / samples.len() as f32;
        let dominant_freq = find_peak_frequency(&spectrum, self.sample_rate);
        let spectral_flux = spectrum.iter().sum::<f32>() / spectrum.len() as f32;
        let beat_confidence = if rms > 0.15 { 0.9 } else { 0.2 };

        (spectral_centroid, rms, zcr, dominant_freq, spectral_flux, beat_confidence, Some(120.0), spectrum)
    }

    fn toggle_playback(&mut self) {
        self.is_playing = !self.is_playing;
    }

    fn reset(&mut self) {
        self.sample_counter = 0;
        self.frequency_offset = 0.0;
    }
}

fn simple_fft(samples: &[f32], _sample_rate: u32) -> Vec<f32> {
    let fft_size = samples.len().min(512);
    let mut spectrum = vec![0.0; fft_size / 2];
    for k in 0..spectrum.len() {
        let mut real = 0.0;
        let mut imag = 0.0;
        for (n, sample) in samples[..fft_size].iter().enumerate() {
            let phase = -2.0 * PI * k as f32 * n as f32 / fft_size as f32;
            real += sample * phase.cos();
            imag += sample * phase.sin();
        }
        spectrum[k] = (real * real + imag * imag).sqrt() / fft_size as f32;
    }
    spectrum
}

fn calculate_centroid(spectrum: &[f32], sample_rate: u32) -> f32 {
    let weighted_sum: f32 = spectrum.iter().enumerate().map(|(i, mag)| i as f32 * mag).sum();
    let sum: f32 = spectrum.iter().sum();
    if sum > 0.0 {
        let bin_width = sample_rate as f32 / (2.0 * spectrum.len() as f32);
        (weighted_sum / sum) * bin_width
    } else {
        1000.0
    }
}

fn find_peak_frequency(spectrum: &[f32], sample_rate: u32) -> f32 {
    let max_idx = spectrum.iter().enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(idx, _)| idx)
        .unwrap_or(0);
    let bin_width = sample_rate as f32 / (2.0 * spectrum.len() as f32);
    max_idx as f32 * bin_width
}

/// Application state
struct DemoApp {
    audio_gen: DemoAudioGenerator,
    feature_bridge: FeatureBridge,
    world_gen: StreamingWorldGenerator,
    mesh_converter: MeshConverter,
    current_genre_idx: usize,
    genres: Vec<(&'static str, GenreStyle)>,
    camera_mode: CameraMode,
    playback_speed: f32,
    frame_count: u64,
    fps: f32,
    last_fps_update: Instant,
    total_chunks: usize,
    avg_latency: f32,
}

impl DemoApp {
    fn new() -> Self {
        let sample_rate = 44100;
        let chunk_size = 512;

        let genres = vec![
            ("Electronic", GenreStyle::electronic()),
            ("Classical", GenreStyle::classical()),
            ("Jazz", GenreStyle::jazz()),
            ("Metal", GenreStyle::metal()),
            ("Ambient", GenreStyle::ambient()),
        ];

        Self {
            audio_gen: DemoAudioGenerator::new(sample_rate, chunk_size),
            feature_bridge: FeatureBridge::new(sample_rate, 5),
            world_gen: StreamingWorldGenerator::new(genres[0].1.clone(), 1.0),
            mesh_converter: MeshConverter::new(1),
            current_genre_idx: 0,
            genres,
            camera_mode: CameraMode::Tracking,
            playback_speed: 1.0,
            frame_count: 0,
            fps: 60.0,
            last_fps_update: Instant::now(),
            total_chunks: 0,
            avg_latency: 0.0,
        }
    }

    fn cycle_genre(&mut self) {
        self.current_genre_idx = (self.current_genre_idx + 1) % self.genres.len();
        let new_style = self.genres[self.current_genre_idx].1.clone();
        self.world_gen = StreamingWorldGenerator::new(new_style, 1.0);
        println!("🎵 Switched to: {}", self.genres[self.current_genre_idx].0);
    }

    fn cycle_camera(&mut self) {
        self.camera_mode = match self.camera_mode {
            CameraMode::Orbit => CameraMode::Tracking,
            CameraMode::Tracking => CameraMode::Cinematic,
            CameraMode::Cinematic => CameraMode::FirstPerson,
            CameraMode::FirstPerson => CameraMode::Orbit,
        };
        println!("📷 Camera mode: {:?}", self.camera_mode);
    }

    fn update(&mut self) -> Option<Vec<omega_synesthesia_renderer::RendererMesh>> {
        let start_time = Instant::now();

        // Generate audio
        let samples = self.audio_gen.generate_musical_chunk();
        let (centroid, rms, zcr, freq, flux, beat, tempo, spectrum) =
            self.audio_gen.extract_features(&samples);

        // Convert to musical features
        let features = self.feature_bridge.convert(
            centroid, rms, zcr, freq, flux, beat, tempo, &spectrum
        );
        self.feature_bridge.advance_time(512);

        // Generate world chunk
        if let Some(chunk) = self.world_gen.add_feature(features) {
            let meshes = self.mesh_converter.convert_chunk(&chunk);

            let latency = start_time.elapsed().as_micros() as f32 / 1000.0;
            self.avg_latency = (self.avg_latency * self.total_chunks as f32 + latency)
                / (self.total_chunks + 1) as f32;
            self.total_chunks += 1;

            return Some(meshes);
        }

        None
    }

    fn update_fps(&mut self) {
        self.frame_count += 1;
        let elapsed = self.last_fps_update.elapsed();
        if elapsed >= Duration::from_millis(500) {
            self.fps = self.frame_count as f32 / elapsed.as_secs_f32();
            self.frame_count = 0;
            self.last_fps_update = Instant::now();
        }
    }

    fn get_hud_text(&self) -> String {
        format!(
            "omega-synesthesia Interactive Demo\n\
             \n\
             Genre: {}\n\
             Camera: {:?}\n\
             Speed: {:.1}x\n\
             \n\
             Performance:\n\
             FPS: {:.1}\n\
             Latency: {:.2}ms\n\
             Chunks: {}\n\
             \n\
             Controls:\n\
             SPACE - Play/Pause\n\
             1-4   - Camera Mode\n\
             G     - Change Genre\n\
             +/-   - Speed\n\
             R     - Reset\n\
             ESC   - Exit",
            self.genres[self.current_genre_idx].0,
            self.camera_mode,
            self.playback_speed,
            self.fps,
            self.avg_latency,
            self.total_chunks,
        )
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║     OMEGA-SYNESTHESIA INTERACTIVE DEMO - V1.0.0                   ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝\n");

    // Create event loop and window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("omega-synesthesia Interactive Demo - V1.0.0")
        .with_inner_size(winit::dpi::PhysicalSize::new(1280, 720))
        .build(&event_loop)?;

    // Initialize renderer
    let config = RendererConfig {
        msaa_samples: 4,
        vsync: true,
        max_meshes: 10000,
    };

    let mut renderer = pollster::block_on(Renderer::new(&window, config))?;
    let mut app = DemoApp::new();
    let mut camera_controller = CameraController::new(app.camera_mode);

    println!("✅ Renderer initialized");
    println!("🎵 Starting music visualization...\n");
    println!("Press 'H' for help\n");

    // Main event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                    ..
                } => {
                    match keycode {
                        VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
                        VirtualKeyCode::Space => {
                            app.audio_gen.toggle_playback();
                            println!("⏯️  {}", if app.audio_gen.is_playing { "Playing" } else { "Paused" });
                        }
                        VirtualKeyCode::Key1 => {
                            app.camera_mode = CameraMode::Orbit;
                            camera_controller.set_mode(app.camera_mode);
                            println!("📷 Camera: Orbit");
                        }
                        VirtualKeyCode::Key2 => {
                            app.camera_mode = CameraMode::Tracking;
                            camera_controller.set_mode(app.camera_mode);
                            println!("📷 Camera: Tracking");
                        }
                        VirtualKeyCode::Key3 => {
                            app.camera_mode = CameraMode::Cinematic;
                            camera_controller.set_mode(app.camera_mode);
                            println!("📷 Camera: Cinematic");
                        }
                        VirtualKeyCode::Key4 => {
                            app.camera_mode = CameraMode::FirstPerson;
                            camera_controller.set_mode(app.camera_mode);
                            println!("📷 Camera: FirstPerson");
                        }
                        VirtualKeyCode::G => app.cycle_genre(),
                        VirtualKeyCode::R => {
                            app.audio_gen.reset();
                            println!("🔄 Reset");
                        }
                        VirtualKeyCode::H => {
                            println!("\n{}\n", app.get_hud_text());
                        }
                        VirtualKeyCode::Equals | VirtualKeyCode::Plus => {
                            app.playback_speed = (app.playback_speed + 0.1).min(2.0);
                            println!("⚡ Speed: {:.1}x", app.playback_speed);
                        }
                        VirtualKeyCode::Minus => {
                            app.playback_speed = (app.playback_speed - 0.1).max(0.1);
                            println!("🐌 Speed: {:.1}x", app.playback_speed);
                        }
                        _ => {}
                    }
                }
                WindowEvent::Resized(new_size) => {
                    renderer.resize(new_size.width, new_size.height);
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                // Update app state
                if let Some(meshes) = app.update() {
                    // Upload meshes to GPU (in real implementation)
                    // For now, just track that we generated them
                }

                app.update_fps();

                // Update camera
                let timeline_position = app.total_chunks as f32 * 0.1;
                camera_controller.update(timeline_position, 0.016);

                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Render frame
                match renderer.render() {
                    Ok(_) => {}
                    Err(e) => eprintln!("Render error: {:?}", e),
                }
            }
            _ => {}
        }
    });
}
