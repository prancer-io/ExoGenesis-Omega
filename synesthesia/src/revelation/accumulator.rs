//! Semantic Accumulator
//!
//! Accumulates semantic context over time as more lyrics and analysis arrive.
//! Like a painter building up layers of understanding.

use std::collections::HashMap;
use crate::audio::TranscribedWord;
use crate::ai::SemanticScene;

/// Accumulated semantic context
#[derive(Debug, Clone)]
pub struct SemanticAccumulator {
    /// All words heard so far
    words: Vec<TimedWord>,

    /// Detected themes with confidence scores
    themes: HashMap<String, f32>,

    /// Mood history for emotional arc
    mood_history: Vec<MoodPoint>,

    /// Accumulated scene fragments
    scene_fragments: Vec<SemanticScene>,

    /// Detected entities (people, places, things)
    entities: HashMap<String, EntityInfo>,

    /// Current narrative state
    narrative: NarrativeState,
}

#[derive(Debug, Clone)]
pub struct TimedWord {
    pub text: String,
    pub timestamp: f64,
    pub confidence: f32,
}

#[derive(Debug, Clone)]
pub struct MoodPoint {
    pub mood: String,
    pub intensity: f32,
    pub timestamp: f64,
}

#[derive(Debug, Clone)]
pub struct EntityInfo {
    pub entity_type: EntityType,
    pub mentions: u32,
    pub first_seen: f64,
    pub confidence: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EntityType {
    Person,
    Place,
    Object,
    Concept,
    Action,
}

#[derive(Debug, Clone, Default)]
pub struct NarrativeState {
    pub detected_setting: Option<String>,
    pub detected_characters: Vec<String>,
    pub detected_actions: Vec<String>,
    pub emotional_arc: Vec<String>,
    pub time_of_day: Option<String>,
    pub weather: Option<String>,
}

/// Synthesized scene from accumulated context
#[derive(Debug, Clone)]
pub struct SynthesizedScene {
    pub subjects: String,
    pub location: String,
    pub time_of_day: String,
    pub mood: String,
    pub action: String,
}

/// Full narrative synthesis
#[derive(Debug, Clone)]
pub struct Narrative {
    pub setting: String,
    pub action: String,
    pub emotional_context: String,
    pub visual_style: String,
    pub lighting: String,
    pub camera: String,
}

impl SemanticAccumulator {
    /// Create new accumulator
    pub fn new() -> Self {
        Self {
            words: Vec::new(),
            themes: HashMap::new(),
            mood_history: Vec::new(),
            scene_fragments: Vec::new(),
            entities: HashMap::new(),
            narrative: NarrativeState::default(),
        }
    }

    /// Add a transcribed word
    pub fn add_word(&mut self, word: TranscribedWord) {
        self.words.push(TimedWord {
            text: word.text.clone(),
            timestamp: word.timestamp,
            confidence: word.confidence,
        });

        // Analyze the new word
        self.analyze_word(&word.text);
    }

    /// Add a semantic scene from AI analysis
    pub fn add_scene(&mut self, scene: SemanticScene) {
        // Update mood history
        self.mood_history.push(MoodPoint {
            mood: format!("{:?}", scene.mood.primary),
            intensity: scene.mood.intensity,
            timestamp: self.current_time(),
        });

        // Update narrative state
        self.narrative.detected_setting = Some(format!("{:?}", scene.setting.location));
        self.narrative.time_of_day = Some(format!("{:?}", scene.setting.time_of_day));
        self.narrative.weather = Some(format!("{:?}", scene.setting.weather));

        for action in &scene.actions {
            if !self.narrative.detected_actions.contains(action) {
                self.narrative.detected_actions.push(action.clone());
            }
        }

        // Add to scene fragments
        self.scene_fragments.push(scene);
    }

    /// Analyze a word for themes and entities
    fn analyze_word(&mut self, word: &str) {
        let word_lower = word.to_lowercase();

        // Theme detection (simple keyword matching - LLM does the real work)
        let theme_keywords: &[(&str, &[&str])] = &[
            ("love", &["love", "heart", "kiss", "embrace", "together", "forever"]),
            ("loss", &["gone", "lost", "miss", "away", "empty", "alone"]),
            ("journey", &["road", "path", "walk", "run", "travel", "home"]),
            ("night", &["night", "dark", "moon", "stars", "dream", "sleep"]),
            ("nature", &["sky", "rain", "sun", "wind", "sea", "mountain"]),
            ("struggle", &["fight", "pain", "hard", "try", "break", "fall"]),
            ("hope", &["hope", "light", "rise", "believe", "new", "tomorrow"]),
            ("memory", &["remember", "memory", "past", "yesterday", "time"]),
        ];

        for (theme, keywords) in theme_keywords {
            if keywords.iter().any(|k| word_lower.contains(k)) {
                let entry = self.themes.entry(theme.to_string()).or_insert(0.0);
                *entry = (*entry + 0.2).min(1.0);
            }
        }

        // Entity detection
        if word.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
            && word.len() > 2
        {
            let entry = self.entities.entry(word.to_string())
                .or_insert(EntityInfo {
                    entity_type: EntityType::Person,
                    mentions: 0,
                    first_seen: self.current_time(),
                    confidence: 0.5,
                });
            entry.mentions += 1;
            entry.confidence = (entry.confidence + 0.1).min(1.0);
        }
    }

    /// Get current time (from most recent word)
    fn current_time(&self) -> f64 {
        self.words.last().map(|w| w.timestamp).unwrap_or(0.0)
    }

    /// Get word count
    pub fn word_count(&self) -> usize {
        self.words.len()
    }

    /// Get theme count
    pub fn theme_count(&self) -> usize {
        self.themes.iter().filter(|(_, c)| **c > 0.3).count()
    }

    /// Get scene fragment count
    pub fn scene_count(&self) -> usize {
        self.scene_fragments.len()
    }

    /// Get primary detected mood
    pub fn primary_mood(&self) -> Option<String> {
        self.mood_history.last().map(|m| m.mood.clone())
    }

    /// Get theme hints (themes with some confidence)
    pub fn theme_hints(&self) -> Vec<String> {
        let mut themes: Vec<_> = self.themes.iter()
            .filter(|(_, conf)| **conf > 0.2)
            .collect();
        themes.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
        themes.into_iter().take(3).map(|(t, _)| t.clone()).collect()
    }

    /// Get detected setting
    pub fn detected_setting(&self) -> Option<String> {
        self.narrative.detected_setting.clone()
    }

    /// Synthesize a scene from accumulated context
    pub fn synthesize_scene(&self) -> SynthesizedScene {
        let subjects = if !self.narrative.detected_characters.is_empty() {
            self.narrative.detected_characters.join(" and ")
        } else {
            "shadowy figures".to_string()
        };

        let location = self.narrative.detected_setting
            .clone()
            .unwrap_or_else(|| "mysterious landscape".to_string());

        let time_of_day = self.narrative.time_of_day
            .clone()
            .unwrap_or_else(|| "twilight".to_string());

        let mood = self.primary_mood()
            .unwrap_or_else(|| "contemplative".to_string());

        let action = self.narrative.detected_actions
            .last()
            .cloned()
            .unwrap_or_else(|| "existing".to_string());

        SynthesizedScene {
            subjects,
            location,
            time_of_day,
            mood,
            action,
        }
    }

    /// Generate full narrative from accumulated context
    pub fn generate_narrative(&self) -> Narrative {
        let scene = self.synthesize_scene();

        // Derive visual style from themes
        let visual_style = if self.themes.get("night").unwrap_or(&0.0) > &0.5 {
            "neo-noir, high contrast"
        } else if self.themes.get("nature").unwrap_or(&0.0) > &0.5 {
            "naturalistic, organic"
        } else if self.themes.get("love").unwrap_or(&0.0) > &0.5 {
            "romantic, soft focus"
        } else if self.themes.get("struggle").unwrap_or(&0.0) > &0.5 {
            "gritty, documentary"
        } else {
            "cinematic, atmospheric"
        };

        // Derive lighting from time of day and mood
        let lighting = match scene.time_of_day.as_str() {
            s if s.contains("Night") => "moonlit with neon accents",
            s if s.contains("Dawn") => "golden hour, warm rays",
            s if s.contains("Dusk") => "purple twilight, long shadows",
            _ => "natural daylight with dramatic contrast",
        };

        // Camera based on energy
        let camera = if self.mood_history.last().map(|m| m.intensity > 0.7).unwrap_or(false) {
            "dynamic tracking, handheld energy"
        } else {
            "smooth dolly, contemplative pace"
        };

        Narrative {
            setting: scene.location,
            action: scene.action,
            emotional_context: scene.mood,
            visual_style: visual_style.to_string(),
            lighting: lighting.to_string(),
            camera: camera.to_string(),
        }
    }

    /// Get emotional arc summary
    pub fn emotional_arc(&self) -> Vec<(f64, String, f32)> {
        self.mood_history.iter()
            .map(|m| (m.timestamp, m.mood.clone(), m.intensity))
            .collect()
    }
}

impl Default for SemanticAccumulator {
    fn default() -> Self {
        Self::new()
    }
}
