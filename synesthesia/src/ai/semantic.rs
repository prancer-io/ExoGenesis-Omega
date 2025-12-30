//! Semantic Parser
//!
//! Extracts semantic meaning from lyrics using LLMs.

use anyhow::Result;
use crossbeam_channel::Sender;
use serde::{Deserialize, Serialize};

use crate::audio::TranscribedWord;

/// Semantic scene description
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticScene {
    /// Scene setting (location, time, weather)
    pub setting: Setting,

    /// Characters in the scene
    pub characters: Vec<Character>,

    /// Actions happening
    pub actions: Vec<String>,

    /// Overall mood
    pub mood: Mood,

    /// Visual elements to include
    pub visual_elements: Vec<VisualElement>,

    /// Camera direction
    pub camera: CameraDirection,
}

impl Default for SemanticScene {
    fn default() -> Self {
        Self {
            setting: Setting::default(),
            characters: Vec::new(),
            actions: Vec::new(),
            mood: Mood::default(),
            visual_elements: Vec::new(),
            camera: CameraDirection::default(),
        }
    }
}

/// Scene setting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    /// Location description
    pub location: String,

    /// Time of day
    pub time_of_day: TimeOfDay,

    /// Weather conditions
    pub weather: Weather,

    /// Indoor or outdoor
    pub indoor: bool,
}

impl Default for Setting {
    fn default() -> Self {
        Self {
            location: "abstract void".to_string(),
            time_of_day: TimeOfDay::Night,
            weather: Weather::Clear,
            indoor: false,
        }
    }
}

/// Time of day
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TimeOfDay {
    Dawn,
    Day,
    Dusk,
    Night,
}

/// Weather conditions
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Weather {
    Clear,
    Rain,
    Snow,
    Fog,
    Storm,
}

/// Character in the scene
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    /// Unique ID for continuity
    pub id: String,

    /// Visual description
    pub description: String,

    /// Current state/action
    pub state: String,

    /// Emotional state
    pub emotion: String,
}

/// Scene mood
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mood {
    /// Primary emotion
    pub primary: String,

    /// Intensity 0-1
    pub intensity: f32,

    /// Color palette (hex colors)
    pub color_palette: Vec<String>,
}

impl Default for Mood {
    fn default() -> Self {
        Self {
            primary: "neutral".to_string(),
            intensity: 0.5,
            color_palette: vec!["#1a1a2e".to_string(), "#16213e".to_string(), "#0f3460".to_string()],
        }
    }
}

/// Visual element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualElement {
    /// Type of element
    pub element_type: VisualElementType,

    /// Description
    pub description: String,
}

/// Types of visual elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VisualElementType {
    Particle,
    Light,
    Object,
    Effect,
}

/// Camera direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraDirection {
    /// Shot type
    pub shot: ShotType,

    /// Camera movement
    pub movement: CameraMovement,

    /// What to focus on
    pub focus: String,
}

impl Default for CameraDirection {
    fn default() -> Self {
        Self {
            shot: ShotType::Wide,
            movement: CameraMovement::Orbit,
            focus: "center".to_string(),
        }
    }
}

/// Shot types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ShotType {
    Wide,
    Medium,
    Close,
    ExtremeClose,
}

/// Camera movements
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CameraMovement {
    Static,
    Pan,
    Dolly,
    Orbit,
    Follow,
}

/// Semantic parser using LLM
pub struct SemanticParser {
    scene_tx: Sender<SemanticScene>,
    word_buffer: Vec<TranscribedWord>,
    current_context: NarrativeContext,
}

/// Narrative context for continuity
#[derive(Debug, Clone, Default)]
pub struct NarrativeContext {
    /// Current setting
    pub current_setting: String,

    /// Active characters
    pub active_characters: Vec<String>,

    /// Emotional arc position
    pub emotional_arc: String,
}

impl SemanticParser {
    /// Create a new semantic parser
    pub fn new(scene_tx: Sender<SemanticScene>) -> Result<Self> {
        log::info!("Initializing semantic parser");

        Ok(Self {
            scene_tx,
            word_buffer: Vec::new(),
            current_context: NarrativeContext::default(),
        })
    }

    /// Process a new transcribed word
    pub fn process_word(&mut self, word: TranscribedWord) {
        self.word_buffer.push(word);

        // Process when we have enough words or after punctuation
        if self.word_buffer.len() >= 5 || self.ends_with_punctuation() {
            self.parse_buffer();
        }
    }

    fn ends_with_punctuation(&self) -> bool {
        self.word_buffer.last()
            .map(|w| w.text.ends_with('.') || w.text.ends_with('!') || w.text.ends_with('?'))
            .unwrap_or(false)
    }

    fn parse_buffer(&mut self) {
        if self.word_buffer.is_empty() {
            return;
        }

        let lyrics: String = self.word_buffer.iter()
            .map(|w| w.text.as_str())
            .collect::<Vec<_>>()
            .join(" ");

        log::debug!("Parsing lyrics: {}", lyrics);

        // TODO: Call LLM for actual semantic parsing
        // For now, use simple keyword matching
        let scene = self.simple_parse(&lyrics);

        if let Err(e) = self.scene_tx.send(scene) {
            log::error!("Failed to send scene: {:?}", e);
        }

        self.word_buffer.clear();
    }

    /// Simple keyword-based parsing (placeholder for LLM)
    fn simple_parse(&mut self, lyrics: &str) -> SemanticScene {
        let lyrics_lower = lyrics.to_lowercase();

        // Detect setting
        let setting = self.detect_setting(&lyrics_lower);

        // Detect mood
        let mood = self.detect_mood(&lyrics_lower);

        // Detect characters
        let characters = self.detect_characters(&lyrics_lower);

        // Detect visual elements
        let visual_elements = self.detect_visual_elements(&lyrics_lower);

        // Update context
        self.current_context.current_setting = setting.location.clone();

        SemanticScene {
            setting,
            characters,
            actions: vec![],
            mood,
            visual_elements,
            camera: CameraDirection::default(),
        }
    }

    fn detect_setting(&self, lyrics: &str) -> Setting {
        let location = if lyrics.contains("city") || lyrics.contains("street") {
            "city street"
        } else if lyrics.contains("beach") || lyrics.contains("ocean") || lyrics.contains("sea") {
            "beach"
        } else if lyrics.contains("forest") || lyrics.contains("tree") {
            "forest"
        } else if lyrics.contains("mountain") || lyrics.contains("sky") {
            "mountain peaks"
        } else if lyrics.contains("home") || lyrics.contains("room") {
            "interior room"
        } else {
            "abstract void"
        };

        let time_of_day = if lyrics.contains("night") || lyrics.contains("dark") || lyrics.contains("moon") {
            TimeOfDay::Night
        } else if lyrics.contains("dawn") || lyrics.contains("morning") || lyrics.contains("sunrise") {
            TimeOfDay::Dawn
        } else if lyrics.contains("sunset") || lyrics.contains("evening") || lyrics.contains("dusk") {
            TimeOfDay::Dusk
        } else {
            TimeOfDay::Day
        };

        let weather = if lyrics.contains("rain") {
            Weather::Rain
        } else if lyrics.contains("snow") {
            Weather::Snow
        } else if lyrics.contains("fog") || lyrics.contains("mist") {
            Weather::Fog
        } else if lyrics.contains("storm") || lyrics.contains("thunder") {
            Weather::Storm
        } else {
            Weather::Clear
        };

        Setting {
            location: location.to_string(),
            time_of_day,
            weather,
            indoor: lyrics.contains("inside") || lyrics.contains("room") || lyrics.contains("home"),
        }
    }

    fn detect_mood(&self, lyrics: &str) -> Mood {
        let (primary, intensity, colors) = if lyrics.contains("love") || lyrics.contains("heart") {
            ("love", 0.8, vec!["#ff1744", "#ff4081", "#f50057"])
        } else if lyrics.contains("sad") || lyrics.contains("cry") || lyrics.contains("tear") {
            ("sadness", 0.7, vec!["#1a237e", "#283593", "#3949ab"])
        } else if lyrics.contains("happy") || lyrics.contains("joy") || lyrics.contains("smile") {
            ("joy", 0.8, vec!["#ffc107", "#ffeb3b", "#fff176"])
        } else if lyrics.contains("angry") || lyrics.contains("hate") || lyrics.contains("rage") {
            ("anger", 0.9, vec!["#b71c1c", "#c62828", "#d32f2f"])
        } else if lyrics.contains("fear") || lyrics.contains("scared") || lyrics.contains("dark") {
            ("fear", 0.6, vec!["#1a1a1a", "#2d2d2d", "#3d3d3d"])
        } else if lyrics.contains("hope") || lyrics.contains("light") || lyrics.contains("dream") {
            ("hope", 0.7, vec!["#00bcd4", "#4dd0e1", "#80deea"])
        } else if lyrics.contains("peace") || lyrics.contains("calm") || lyrics.contains("quiet") {
            ("peace", 0.5, vec!["#81c784", "#a5d6a7", "#c8e6c9"])
        } else {
            ("neutral", 0.5, vec!["#1a1a2e", "#16213e", "#0f3460"])
        };

        Mood {
            primary: primary.to_string(),
            intensity,
            color_palette: colors.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn detect_characters(&self, lyrics: &str) -> Vec<Character> {
        let mut characters = Vec::new();

        if lyrics.contains("i ") || lyrics.contains("i'm") || lyrics.contains("my") {
            characters.push(Character {
                id: "narrator".to_string(),
                description: "protagonist figure".to_string(),
                state: if lyrics.contains("walk") { "walking" }
                       else if lyrics.contains("run") { "running" }
                       else if lyrics.contains("stand") { "standing" }
                       else { "present" }.to_string(),
                emotion: self.detect_mood(lyrics).primary,
            });
        }

        if lyrics.contains("you") || lyrics.contains("your") {
            characters.push(Character {
                id: "other".to_string(),
                description: "another figure".to_string(),
                state: "present".to_string(),
                emotion: "mysterious".to_string(),
            });
        }

        characters
    }

    fn detect_visual_elements(&self, lyrics: &str) -> Vec<VisualElement> {
        let mut elements = Vec::new();

        if lyrics.contains("fire") || lyrics.contains("burn") || lyrics.contains("flame") {
            elements.push(VisualElement {
                element_type: VisualElementType::Particle,
                description: "fire particles rising".to_string(),
            });
        }

        if lyrics.contains("star") || lyrics.contains("light") || lyrics.contains("glow") {
            elements.push(VisualElement {
                element_type: VisualElementType::Light,
                description: "glowing lights".to_string(),
            });
        }

        if lyrics.contains("rain") {
            elements.push(VisualElement {
                element_type: VisualElementType::Particle,
                description: "rain drops falling".to_string(),
            });
        }

        if lyrics.contains("wind") {
            elements.push(VisualElement {
                element_type: VisualElementType::Effect,
                description: "wind particles".to_string(),
            });
        }

        elements
    }
}
