//! .synth File Loader
//!
//! Loads binary .synth files created by the Python analyzer.

use std::fs::File;
use std::io::{Read, BufReader};
use std::path::Path;
use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};

/// Magic bytes for .synth files
const MAGIC: &[u8] = b"SYNTH\x00\x01\x00";

/// Complete .synth file structure
#[derive(Debug, Clone)]
pub struct SynthFile {
    pub version: (u8, u8),
    pub created_at: String,
    pub analysis: MusicAnalysis,
    pub video_segments: Vec<VideoSegment>,
    pub transitions: Vec<Transition>,
    pub shader_curves: Option<ShaderCurves>,
    pub style_name: String,
}

/// Complete music analysis
#[derive(Debug, Clone, Deserialize)]
pub struct MusicAnalysis {
    pub duration: f64,
    pub sample_rate: u32,
    pub audio_hash: String,

    pub key: String,
    pub mode: String,
    pub key_confidence: f32,

    pub tempo: f32,
    pub tempo_confidence: f32,
    pub beats: Vec<f64>,
    pub downbeats: Vec<f64>,
    pub time_signature: (u8, u8),

    pub chords: Vec<Chord>,
    pub sections: Vec<Section>,
    pub climaxes: Vec<ClimaxPoint>,

    pub energy_curve: Vec<f32>,
    pub tension_curve: Vec<f32>,
    pub loudness_curve: Vec<f32>,

    pub emotion_arc: Vec<EmotionPoint>,

    #[serde(default)]
    pub spectral_centroid: Vec<f32>,
    #[serde(default)]
    pub spectral_flux: Vec<f32>,
}

impl Default for MusicAnalysis {
    fn default() -> Self {
        Self {
            duration: 0.0,
            sample_rate: 44100,
            audio_hash: String::new(),
            key: "C".to_string(),
            mode: "major".to_string(),
            key_confidence: 0.0,
            tempo: 120.0,
            tempo_confidence: 0.0,
            beats: Vec::new(),
            downbeats: Vec::new(),
            time_signature: (4, 4),
            chords: Vec::new(),
            sections: Vec::new(),
            climaxes: Vec::new(),
            energy_curve: Vec::new(),
            tension_curve: Vec::new(),
            loudness_curve: Vec::new(),
            emotion_arc: Vec::new(),
            spectral_centroid: Vec::new(),
            spectral_flux: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Chord {
    pub time: f64,
    pub duration: f64,
    pub chord: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Section {
    #[serde(rename = "type")]
    pub section_type: String,
    pub start: f64,
    pub end: f64,
    pub energy: f32,
    pub repetition: u8,
    pub confidence: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClimaxPoint {
    pub time: f64,
    pub intensity: f32,
    #[serde(rename = "type")]
    pub climax_type: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmotionPoint {
    pub time: f64,
    pub emotion: String,
    pub intensity: f32,
    pub valence: f32,
    pub arousal: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VideoSegment {
    pub segment_id: u32,
    pub start_time: f64,
    pub end_time: f64,
    pub video_path: String,
    pub mood: String,
    pub clarity_level: f32,
    pub base_hue: f32,
    pub saturation: f32,
    pub brightness: f32,
    pub motion_speed: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Transition {
    pub time: f64,
    pub from_segment: u32,
    pub to_segment: u32,
    pub transition_type: String,
    pub duration: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ShaderCurves {
    pub times: Vec<f64>,
    pub bloom_intensity: Vec<f32>,
    pub chromatic_amount: Vec<f32>,
    pub vignette_strength: Vec<f32>,
    pub grain_amount: Vec<f32>,
    pub color_shift: Vec<f32>,
}

impl SynthFile {
    /// Load from file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        // Read magic
        let mut magic = [0u8; 8];
        reader.read_exact(&mut magic)?;
        if &magic != MAGIC {
            bail!("Invalid .synth file (bad magic)");
        }

        // Read flags
        let mut flags_bytes = [0u8; 4];
        reader.read_exact(&mut flags_bytes)?;
        let flags = u32::from_le_bytes(flags_bytes);
        let has_video = flags & 0x01 != 0;
        let has_shader = flags & 0x02 != 0;

        // Read analysis section
        let analysis = Self::read_msgpack_section::<MusicAnalysis>(&mut reader)?;

        // Read video segments (if present)
        let (video_segments, transitions) = if has_video {
            let segments = Self::read_msgpack_section::<Vec<VideoSegment>>(&mut reader)?;
            let trans = Self::read_msgpack_section::<Vec<Transition>>(&mut reader)?;
            (segments, trans)
        } else {
            (Vec::new(), Vec::new())
        };

        // Read shader curves (if present)
        let shader_curves = if has_shader {
            Some(Self::read_msgpack_section::<ShaderCurves>(&mut reader)?)
        } else {
            None
        };

        // Read style
        #[derive(Deserialize)]
        struct StyleData {
            name: String,
            #[allow(dead_code)]
            params: serde_json::Value,
        }
        let style: StyleData = Self::read_msgpack_section(&mut reader)?;

        Ok(SynthFile {
            version: (1, 0),
            created_at: String::new(),
            analysis,
            video_segments,
            transitions,
            shader_curves,
            style_name: style.name,
        })
    }

    fn read_msgpack_section<T: for<'de> Deserialize<'de>>(reader: &mut BufReader<File>) -> Result<T> {
        // Read size
        let mut size_bytes = [0u8; 4];
        reader.read_exact(&mut size_bytes)?;
        let size = u32::from_le_bytes(size_bytes) as usize;

        // Read data
        let mut data = vec![0u8; size];
        reader.read_exact(&mut data)?;

        // Deserialize
        let value: T = rmp_serde::from_slice(&data)?;
        Ok(value)
    }
}

/// Utility for loading .synth files
pub struct SynthLoader;

impl SynthLoader {
    /// Load a .synth file from path
    pub fn load<P: AsRef<Path>>(path: P) -> Result<SynthFile> {
        SynthFile::load(path)
    }

    /// Check if a file is a valid .synth file
    pub fn is_valid<P: AsRef<Path>>(path: P) -> bool {
        let path = path.as_ref();
        if let Ok(file) = File::open(path) {
            let mut reader = BufReader::new(file);
            let mut magic = [0u8; 8];
            if reader.read_exact(&mut magic).is_ok() {
                return &magic == MAGIC;
            }
        }
        false
    }
}

impl MusicAnalysis {
    /// Get section at time
    pub fn section_at(&self, time: f64) -> Option<&Section> {
        self.sections.iter().find(|s| time >= s.start && time < s.end)
    }

    /// Get chord at time
    pub fn chord_at(&self, time: f64) -> Option<&Chord> {
        self.chords.iter().find(|c| time >= c.time && time < c.time + c.duration)
    }

    /// Get emotion at time
    pub fn emotion_at(&self, time: f64) -> Option<&EmotionPoint> {
        // Find the emotion point just before or at this time
        self.emotion_arc.iter().rev().find(|e| e.time <= time)
    }

    /// Get energy at time (interpolated)
    pub fn energy_at(&self, time: f64) -> f32 {
        if self.energy_curve.is_empty() {
            return 0.5;
        }

        // 10Hz sample rate
        let idx = (time * 10.0) as usize;
        if idx >= self.energy_curve.len() {
            *self.energy_curve.last().unwrap_or(&0.5)
        } else {
            self.energy_curve[idx]
        }
    }

    /// Get tension at time
    pub fn tension_at(&self, time: f64) -> f32 {
        if self.tension_curve.is_empty() {
            return 0.0;
        }

        let idx = (time * 10.0) as usize;
        if idx >= self.tension_curve.len() {
            *self.tension_curve.last().unwrap_or(&0.0)
        } else {
            self.tension_curve[idx]
        }
    }

    /// Get nearest climax
    pub fn nearest_climax(&self, time: f64) -> Option<(&ClimaxPoint, f64)> {
        self.climaxes.iter()
            .map(|c| (c, (c.time - time).abs()))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }

    /// Is at or near a beat
    pub fn is_beat(&self, time: f64, tolerance: f64) -> bool {
        self.beats.iter().any(|b| (b - time).abs() < tolerance)
    }

    /// Is at or near a downbeat
    pub fn is_downbeat(&self, time: f64, tolerance: f64) -> bool {
        self.downbeats.iter().any(|b| (b - time).abs() < tolerance)
    }
}
