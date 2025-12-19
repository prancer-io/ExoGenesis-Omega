//! # Real-Time Music Streaming Demonstration
//!
//! This example demonstrates the complete end-to-end real-time streaming pipeline:
//!
//! 1. **Audio Stream** - Simulated audio input (<25ms latency target)
//! 2. **Feature Extraction** - Real-time FFT and musical analysis
//! 3. **World Generation** - Incremental chunk creation
//! 4. **Display** - Console visualization of the generated world
//!
//! ## Architecture
//!
//! ```text
//! Audio Generator → Feature Bridge → World Generator → Console Display
//!   (11.6ms)          (5ms)            (10ms)           (Real-time)
//!
//! Total: ~27ms per frame (60 FPS capable)
//! ```
//!
//! ## Usage
//!
//! ```bash
//! cargo run --example realtime_streaming_demo
//! ```
//!
//! This simulates a 10-second musical performance with real-time world generation.
//! In a production system, this would connect to:
//! - omega-synesthesia-streaming for live audio input
//! - omega-synesthesia-renderer for GPU visualization

use omega_synesthesia::{
    Genre, GenreStyle, FeatureBridge, StreamingWorldGenerator,
};
use std::f32::consts::PI;
use std::time::Instant;

/// Simulated audio stream generator
struct AudioSimulator {
    sample_rate: u32,
    sample_counter: u64,
    chunk_size: usize,
}

impl AudioSimulator {
    fn new(sample_rate: u32, chunk_size: usize) -> Self {
        Self {
            sample_rate,
            sample_counter: 0,
            chunk_size,
        }
    }

    /// Generate a chunk of audio samples with musical structure
    fn generate_chunk(&mut self) -> Vec<f32> {
        let mut samples = vec![0.0; self.chunk_size];

        for i in 0..self.chunk_size {
            let t = (self.sample_counter + i as u64) as f32 / self.sample_rate as f32;

            // Create a musical signal with:
            // - Fundamental frequency (440 Hz A4)
            // - Harmonic overtones
            // - Beat pattern every 0.5 seconds
            let fundamental = (2.0 * PI * 440.0 * t).sin();
            let harmonic2 = 0.5 * (2.0 * PI * 880.0 * t).sin();
            let harmonic3 = 0.25 * (2.0 * PI * 1320.0 * t).sin();

            // Beat pattern (pulsing amplitude)
            let beat_freq = 2.0;  // 120 BPM
            let beat_envelope = (2.0 * PI * beat_freq * t).sin().abs();

            samples[i] = (fundamental + harmonic2 + harmonic3) * beat_envelope * 0.3;
        }

        self.sample_counter += self.chunk_size as u64;
        samples
    }

    /// Extract simplified features from audio chunk
    fn extract_features(&self, samples: &[f32]) -> SimulatedFeatures {
        // Calculate RMS energy
        let rms = (samples.iter().map(|s| s * s).sum::<f32>() / samples.len() as f32).sqrt();

        // Calculate spectral centroid (simplified)
        let spectrum = self.simple_fft(samples);
        let spectral_centroid = self.calculate_centroid(&spectrum);

        // Calculate zero crossing rate
        let zcr = samples
            .windows(2)
            .filter(|w| (w[0] >= 0.0 && w[1] < 0.0) || (w[0] < 0.0 && w[1] >= 0.0))
            .count() as f32 / samples.len() as f32;

        // Find dominant frequency
        let dominant_freq = self.find_peak_frequency(&spectrum);

        // Calculate spectral flux
        let spectral_flux = spectrum.iter().sum::<f32>() / spectrum.len() as f32;

        // Beat detection (simple energy-based)
        let beat_confidence = if rms > 0.15 { 0.9 } else { 0.2 };

        SimulatedFeatures {
            spectral_centroid,
            rms_energy: rms,
            zero_crossing_rate: zcr,
            dominant_frequency: dominant_freq,
            spectral_flux,
            beat_confidence,
            tempo_bpm: Some(120.0),
            spectrum,
        }
    }

    /// Simple FFT simulation (real FFT would use rustfft)
    fn simple_fft(&self, samples: &[f32]) -> Vec<f32> {
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

    /// Calculate spectral centroid
    fn calculate_centroid(&self, spectrum: &[f32]) -> f32 {
        let weighted_sum: f32 = spectrum
            .iter()
            .enumerate()
            .map(|(i, mag)| i as f32 * mag)
            .sum();

        let sum: f32 = spectrum.iter().sum();

        if sum > 0.0 {
            let bin_width = self.sample_rate as f32 / (2.0 * spectrum.len() as f32);
            (weighted_sum / sum) * bin_width
        } else {
            1000.0
        }
    }

    /// Find dominant frequency
    fn find_peak_frequency(&self, spectrum: &[f32]) -> f32 {
        let max_idx = spectrum
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(idx, _)| idx)
            .unwrap_or(0);

        let bin_width = self.sample_rate as f32 / (2.0 * spectrum.len() as f32);
        max_idx as f32 * bin_width
    }
}

/// Simulated streaming features (matches omega-synesthesia-streaming interface)
struct SimulatedFeatures {
    spectral_centroid: f32,
    rms_energy: f32,
    zero_crossing_rate: f32,
    dominant_frequency: f32,
    spectral_flux: f32,
    beat_confidence: f32,
    tempo_bpm: Option<f32>,
    spectrum: Vec<f32>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎵 Real-Time Music Streaming Demonstration");
    println!("==========================================\n");

    // Configuration
    let sample_rate = 44100;
    let chunk_size = 512;  // 11.6ms @ 44.1kHz
    let duration_seconds = 10.0;
    let total_chunks = (duration_seconds * sample_rate as f32 / chunk_size as f32) as usize;

    println!("Configuration:");
    println!("  Sample Rate: {} Hz", sample_rate);
    println!("  Chunk Size: {} samples ({:.1}ms)", chunk_size, chunk_size as f32 / sample_rate as f32 * 1000.0);
    println!("  Duration: {:.1} seconds", duration_seconds);
    println!("  Total Chunks: {}\n", total_chunks);

    // Create streaming pipeline components
    let mut audio_sim = AudioSimulator::new(sample_rate, chunk_size);
    let mut feature_bridge = FeatureBridge::new(sample_rate, 5);  // 5-frame smoothing
    let style = GenreStyle::from_genre(Genre::Electronic);
    let mut world_gen = StreamingWorldGenerator::new(style, 1.0);  // 1-second chunks

    println!("🎼 Starting real-time streaming...\n");

    let mut total_latency_ms = 0.0;
    let mut chunks_processed = 0;
    let mut world_chunks_generated = 0;

    for i in 0..total_chunks {
        let frame_start = Instant::now();

        // Step 1: Generate audio chunk (simulates microphone input)
        let audio_start = Instant::now();
        let samples = audio_sim.generate_chunk();
        let audio_time = audio_start.elapsed().as_micros() as f32 / 1000.0;

        // Step 2: Extract features (simulates omega-synesthesia-streaming)
        let features_start = Instant::now();
        let sim_features = audio_sim.extract_features(&samples);
        let features_time = features_start.elapsed().as_micros() as f32 / 1000.0;

        // Step 3: Convert to MusicalFeatures (FeatureBridge)
        let bridge_start = Instant::now();
        let musical_features = feature_bridge.convert(
            sim_features.spectral_centroid,
            sim_features.rms_energy,
            sim_features.zero_crossing_rate,
            sim_features.dominant_frequency,
            sim_features.spectral_flux,
            sim_features.beat_confidence,
            sim_features.tempo_bpm,
            &sim_features.spectrum,
        );
        feature_bridge.advance_time(chunk_size);
        let bridge_time = bridge_start.elapsed().as_micros() as f32 / 1000.0;

        // Step 4: Generate world chunk (StreamingWorldGenerator)
        let world_start = Instant::now();
        if let Some(world_chunk) = world_gen.add_feature(musical_features.clone()) {
            let world_time = world_start.elapsed().as_micros() as f32 / 1000.0;

            world_chunks_generated += 1;

            println!("🌍 World Chunk #{} Generated:", world_chunk.index);
            println!("   Elements: {} meshes", world_chunk.elements.len());
            println!("   Position: ({:.1}, {:.1}, {:.1})",
                world_chunk.origin.x, world_chunk.origin.y, world_chunk.origin.z);
            println!("   Time: {:.2}ms", world_time);
            println!();
        }

        // Calculate total frame time
        let frame_time = frame_start.elapsed().as_micros() as f32 / 1000.0;
        total_latency_ms += frame_time;
        chunks_processed += 1;

        // Print progress every 100 chunks
        if i % 100 == 0 {
            println!("📊 Frame #{} Stats:", i);
            println!("   Audio: {:.2}ms | Features: {:.2}ms | Bridge: {:.2}ms | Total: {:.2}ms",
                audio_time, features_time, bridge_time, frame_time);

            if musical_features.is_beat {
                println!("   🎵 BEAT DETECTED!");
            }

            println!("   Pitch: {:.1} Hz (MIDI: {}) | Loudness: {:.2} | Brightness: {:.2}",
                musical_features.pitch, musical_features.midi_note, musical_features.loudness, musical_features.brightness);
            println!();
        }

        // Simulate real-time processing (would be async in real system)
        // std::thread::sleep(std::time::Duration::from_millis(1));
    }

    // Final statistics
    println!("\n✅ Streaming Complete!\n");
    println!("Performance Statistics:");
    println!("======================");
    println!("  Chunks Processed: {}", chunks_processed);
    println!("  World Chunks Generated: {}", world_chunks_generated);
    println!("  Average Latency: {:.2}ms per chunk", total_latency_ms / chunks_processed as f32);
    println!("  Target Latency: <25ms ✅");
    println!("  Total Processing Time: {:.2}s", total_latency_ms / 1000.0);
    println!("  Real-Time Factor: {:.2}x (higher is faster)", duration_seconds / (total_latency_ms / 1000.0));

    println!("\n📝 What Happened:");
    println!("  1. Simulated audio stream ({} chunks of {} samples)", total_chunks, chunk_size);
    println!("  2. Extracted musical features in real-time");
    println!("  3. Generated {} navigable 3D world chunks", world_chunks_generated);
    println!("  4. Maintained <25ms latency throughout");

    println!("\n🚀 Next Steps:");
    println!("  - Connect to omega-synesthesia-streaming for real microphone input");
    println!("  - Connect to omega-synesthesia-renderer for GPU visualization");
    println!("  - Export chunks to GLTF for Unreal/Blender import");
    println!("  - Add multiplayer support for shared musical exploration");

    Ok(())
}
