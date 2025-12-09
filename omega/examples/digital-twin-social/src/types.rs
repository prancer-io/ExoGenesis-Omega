//! Core types for the Digital Twin Social Platform
//!
//! These types demonstrate how to model human personality, emotions,
//! and relationships using ExoGenesis Omega's type system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for a user in the system
pub type UserId = Uuid;

/// Unique identifier for a relationship between users
pub type RelationshipId = Uuid;

/// Unique identifier for an emotional episode
pub type EpisodeId = Uuid;

// =============================================================================
// PERSONALITY MODELING
// =============================================================================

/// Big Five (OCEAN) personality traits
/// Each dimension ranges from 0.0 to 1.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BigFive {
    /// Openness to experience - creativity, curiosity, intellectual interests
    pub openness: f32,
    /// Conscientiousness - organization, dependability, self-discipline
    pub conscientiousness: f32,
    /// Extraversion - sociability, assertiveness, positive emotionality
    pub extraversion: f32,
    /// Agreeableness - cooperation, trust, empathy
    pub agreeableness: f32,
    /// Neuroticism - emotional instability, anxiety, moodiness
    pub neuroticism: f32,
}

impl BigFive {
    /// Create a new BigFive profile with the given traits
    pub fn new(o: f32, c: f32, e: f32, a: f32, n: f32) -> Self {
        Self {
            openness: o.clamp(0.0, 1.0),
            conscientiousness: c.clamp(0.0, 1.0),
            extraversion: e.clamp(0.0, 1.0),
            agreeableness: a.clamp(0.0, 1.0),
            neuroticism: n.clamp(0.0, 1.0),
        }
    }

    /// Convert to a 5-dimensional vector
    pub fn to_vector(&self) -> [f32; 5] {
        [
            self.openness,
            self.conscientiousness,
            self.extraversion,
            self.agreeableness,
            self.neuroticism,
        ]
    }

    /// Calculate similarity with another BigFive profile (0.0 to 1.0)
    pub fn similarity(&self, other: &BigFive) -> f32 {
        let self_vec = self.to_vector();
        let other_vec = other.to_vector();

        let dot: f32 = self_vec.iter().zip(other_vec.iter()).map(|(a, b)| a * b).sum();
        let mag_self: f32 = self_vec.iter().map(|x| x * x).sum::<f32>().sqrt();
        let mag_other: f32 = other_vec.iter().map(|x| x * x).sum::<f32>().sqrt();

        if mag_self == 0.0 || mag_other == 0.0 {
            0.0
        } else {
            dot / (mag_self * mag_other)
        }
    }
}

impl Default for BigFive {
    fn default() -> Self {
        Self::new(0.5, 0.5, 0.5, 0.5, 0.5)
    }
}

/// Schwartz Values Theory - 10 basic human values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchwartzValues {
    /// Self-direction: independent thought and action
    pub self_direction: f32,
    /// Stimulation: excitement, novelty, challenge
    pub stimulation: f32,
    /// Hedonism: pleasure and sensuous gratification
    pub hedonism: f32,
    /// Achievement: personal success through competence
    pub achievement: f32,
    /// Power: social status, prestige, control
    pub power: f32,
    /// Security: safety, harmony, stability
    pub security: f32,
    /// Conformity: restraint of actions that harm others
    pub conformity: f32,
    /// Tradition: respect for cultural/religious customs
    pub tradition: f32,
    /// Benevolence: welfare of close others
    pub benevolence: f32,
    /// Universalism: welfare of all people and nature
    pub universalism: f32,
}

impl SchwartzValues {
    /// Convert to a 10-dimensional vector
    pub fn to_vector(&self) -> [f32; 10] {
        [
            self.self_direction, self.stimulation, self.hedonism,
            self.achievement, self.power, self.security,
            self.conformity, self.tradition, self.benevolence,
            self.universalism,
        ]
    }
}

impl Default for SchwartzValues {
    fn default() -> Self {
        Self {
            self_direction: 0.5,
            stimulation: 0.5,
            hedonism: 0.5,
            achievement: 0.5,
            power: 0.5,
            security: 0.5,
            conformity: 0.5,
            tradition: 0.5,
            benevolence: 0.5,
            universalism: 0.5,
        }
    }
}

/// Attachment style based on attachment theory
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AttachmentStyle {
    /// Comfortable with intimacy and autonomy
    Secure,
    /// Desires closeness but fears rejection
    Anxious,
    /// Values independence, uncomfortable with closeness
    Avoidant,
    /// Combination of anxious and avoidant patterns
    Disorganized,
}

impl AttachmentStyle {
    /// Convert to dimensional representation (anxiety, avoidance)
    pub fn to_dimensions(&self) -> (f32, f32) {
        match self {
            AttachmentStyle::Secure => (0.2, 0.2),
            AttachmentStyle::Anxious => (0.8, 0.2),
            AttachmentStyle::Avoidant => (0.2, 0.8),
            AttachmentStyle::Disorganized => (0.8, 0.8),
        }
    }
}

/// Emotional Intelligence (EQ) dimensions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalIntelligence {
    /// Ability to recognize own emotions
    pub self_awareness: f32,
    /// Ability to manage own emotions
    pub self_regulation: f32,
    /// Internal drive beyond external rewards
    pub motivation: f32,
    /// Ability to understand others' emotions
    pub empathy: f32,
    /// Ability to manage relationships
    pub social_skills: f32,
}

impl EmotionalIntelligence {
    /// Convert to a 5-dimensional vector
    pub fn to_vector(&self) -> [f32; 5] {
        [
            self.self_awareness,
            self.self_regulation,
            self.motivation,
            self.empathy,
            self.social_skills,
        ]
    }

    /// Calculate overall EQ score (0.0 to 1.0)
    pub fn overall_score(&self) -> f32 {
        (self.self_awareness + self.self_regulation + self.motivation
         + self.empathy + self.social_skills) / 5.0
    }
}

impl Default for EmotionalIntelligence {
    fn default() -> Self {
        Self {
            self_awareness: 0.5,
            self_regulation: 0.5,
            motivation: 0.5,
            empathy: 0.5,
            social_skills: 0.5,
        }
    }
}

/// Communication style preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationStyle {
    /// Directness (0 = indirect, 1 = very direct)
    pub directness: f32,
    /// Expressiveness (0 = reserved, 1 = very expressive)
    pub expressiveness: f32,
    /// Formality preference (0 = casual, 1 = formal)
    pub formality: f32,
    /// Conflict approach (0 = avoidant, 1 = confrontational)
    pub conflict_approach: f32,
    /// Listening vs speaking preference (0 = listener, 1 = speaker)
    pub listening_speaking: f32,
    /// Emotional vs logical communication (0 = emotional, 1 = logical)
    pub emotional_logical: f32,
}

impl CommunicationStyle {
    /// Convert to a 6-dimensional vector
    pub fn to_vector(&self) -> [f32; 6] {
        [
            self.directness,
            self.expressiveness,
            self.formality,
            self.conflict_approach,
            self.listening_speaking,
            self.emotional_logical,
        ]
    }
}

impl Default for CommunicationStyle {
    fn default() -> Self {
        Self {
            directness: 0.5,
            expressiveness: 0.5,
            formality: 0.5,
            conflict_approach: 0.5,
            listening_speaking: 0.5,
            emotional_logical: 0.5,
        }
    }
}

// =============================================================================
// DIGITAL TWIN PROFILE
// =============================================================================

/// Complete digital twin profile for a user
/// This is the core representation of a human in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalTwin {
    /// Unique identifier
    pub id: UserId,

    /// Display name
    pub name: String,

    // Personality components
    pub big_five: BigFive,
    pub values: SchwartzValues,
    pub attachment_style: AttachmentStyle,
    pub eq: EmotionalIntelligence,
    pub communication_style: CommunicationStyle,

    /// Deep embedding vector (4096 dimensions)
    /// This combines all personality aspects into a unified representation
    pub deep_embedding: Vec<f32>,

    /// Confidence in the profile (0.0 to 1.0)
    /// Increases with more data points
    pub confidence: f64,

    /// Number of observations used to build this profile
    pub observation_count: u64,

    /// When the profile was created
    pub created_at: DateTime<Utc>,

    /// Last time the profile was updated
    pub updated_at: DateTime<Utc>,
}

impl DigitalTwin {
    /// Create a new digital twin with default values
    pub fn new(name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            big_five: BigFive::default(),
            values: SchwartzValues::default(),
            attachment_style: AttachmentStyle::Secure,
            eq: EmotionalIntelligence::default(),
            communication_style: CommunicationStyle::default(),
            deep_embedding: vec![0.0; 4096],
            confidence: 0.0,
            observation_count: 0,
            created_at: now,
            updated_at: now,
        }
    }

    /// Generate deep embedding from personality components
    pub fn generate_embedding(&mut self) {
        let mut embedding = Vec::with_capacity(4096);

        // Big Five (5 dims, expanded to 256)
        for val in self.big_five.to_vector() {
            for i in 0..51 {
                embedding.push(val * (1.0 - (i as f32 / 51.0) * 0.1));
            }
        }
        embedding.push(self.big_five.to_vector().iter().sum::<f32>() / 5.0);

        // Values (10 dims, expanded to 512)
        for val in self.values.to_vector() {
            for i in 0..51 {
                embedding.push(val * (1.0 - (i as f32 / 51.0) * 0.1));
            }
        }
        for _ in 0..2 {
            embedding.push(self.values.to_vector().iter().sum::<f32>() / 10.0);
        }

        // Attachment (2 dims, expanded to 128)
        let (anxiety, avoidance) = self.attachment_style.to_dimensions();
        for i in 0..64 {
            embedding.push(anxiety * (1.0 - (i as f32 / 64.0) * 0.1));
        }
        for i in 0..64 {
            embedding.push(avoidance * (1.0 - (i as f32 / 64.0) * 0.1));
        }

        // EQ (5 dims, expanded to 256)
        for val in self.eq.to_vector() {
            for i in 0..51 {
                embedding.push(val * (1.0 - (i as f32 / 51.0) * 0.1));
            }
        }
        embedding.push(self.eq.overall_score());

        // Communication (6 dims, expanded to 256)
        for val in self.communication_style.to_vector() {
            for i in 0..42 {
                embedding.push(val * (1.0 - (i as f32 / 42.0) * 0.1));
            }
        }
        for _ in 0..4 {
            embedding.push(self.communication_style.to_vector().iter().sum::<f32>() / 6.0);
        }

        // Pad to 4096 with derived features
        while embedding.len() < 4096 {
            let idx = embedding.len() % 256;
            let base_val = embedding.get(idx).copied().unwrap_or(0.5);
            embedding.push(base_val * 0.95);
        }

        embedding.truncate(4096);
        self.deep_embedding = embedding;
    }
}

// =============================================================================
// EMOTIONAL STATES
// =============================================================================

/// Core emotions based on Plutchik's wheel
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum CoreEmotion {
    Joy,
    Trust,
    Fear,
    Surprise,
    Sadness,
    Disgust,
    Anger,
    Anticipation,
}

/// Current emotional state with intensity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalState {
    /// Primary emotion being experienced
    pub primary: CoreEmotion,
    /// Intensity of primary emotion (0.0 to 1.0)
    pub primary_intensity: f32,
    /// Secondary emotion if present
    pub secondary: Option<CoreEmotion>,
    /// Intensity of secondary emotion
    pub secondary_intensity: f32,
    /// Valence: negative (-1.0) to positive (1.0)
    pub valence: f32,
    /// Arousal: calm (0.0) to excited (1.0)
    pub arousal: f32,
    /// Dominance: submissive (0.0) to dominant (1.0)
    pub dominance: f32,
    /// Timestamp of this emotional state
    pub timestamp: DateTime<Utc>,
}

impl EmotionalState {
    /// Create a neutral emotional state
    pub fn neutral() -> Self {
        Self {
            primary: CoreEmotion::Anticipation,
            primary_intensity: 0.3,
            secondary: None,
            secondary_intensity: 0.0,
            valence: 0.0,
            arousal: 0.3,
            dominance: 0.5,
            timestamp: Utc::now(),
        }
    }

    /// Convert to VAD (Valence-Arousal-Dominance) vector
    pub fn to_vad(&self) -> [f32; 3] {
        [self.valence, self.arousal, self.dominance]
    }

    /// Calculate emotional distance from another state
    pub fn distance(&self, other: &EmotionalState) -> f32 {
        let self_vad = self.to_vad();
        let other_vad = other.to_vad();

        self_vad.iter()
            .zip(other_vad.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f32>()
            .sqrt()
    }
}

// =============================================================================
// RELATIONSHIPS
// =============================================================================

/// Type of connection between users
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ConnectionDomain {
    /// Romantic relationship seeking
    Dating,
    /// Platonic friendship
    Friendship,
    /// Professional/work relationship
    Professional,
    /// Mentorship (mentor or mentee)
    Mentorship,
    /// Community/group membership
    Community,
    /// Creative collaboration
    Creative,
}

/// Predicted outcomes for a potential relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipPrediction {
    /// Overall compatibility score (0.0 to 1.0)
    pub compatibility: f64,
    /// Predicted satisfaction level (0.0 to 1.0)
    pub satisfaction: f64,
    /// Predicted relationship longevity score (0.0 to 1.0)
    pub longevity: f64,
    /// Growth potential for both parties (0.0 to 1.0)
    pub growth_potential: f64,
    /// Risk of significant conflict (0.0 to 1.0)
    pub conflict_risk: f64,
    /// Confidence in these predictions (0.0 to 1.0)
    pub confidence: f64,
    /// Key factors driving compatibility
    pub key_factors: Vec<CompatibilityFactor>,
}

/// A factor contributing to compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityFactor {
    /// Name of the factor
    pub name: String,
    /// Impact on compatibility (-1.0 to 1.0)
    pub impact: f64,
    /// Explanation
    pub description: String,
}

// =============================================================================
// SENSOR DATA
// =============================================================================

/// Keystroke dynamics data from virtual keyboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeystrokeData {
    /// Time key was pressed down
    pub key_down: DateTime<Utc>,
    /// Time key was released
    pub key_up: DateTime<Utc>,
    /// Key that was pressed (anonymized)
    pub key_category: KeyCategory,
    /// Pressure if available (0.0 to 1.0)
    pub pressure: Option<f32>,
}

/// Category of key (for privacy, not the actual key)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum KeyCategory {
    Letter,
    Number,
    Punctuation,
    Space,
    Backspace,
    Enter,
    Other,
}

/// Wearable biometric data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricData {
    /// Heart rate in BPM
    pub heart_rate: Option<f32>,
    /// Heart rate variability (RMSSD in ms)
    pub hrv_rmssd: Option<f32>,
    /// Skin temperature in Celsius
    pub skin_temperature: Option<f32>,
    /// Electrodermal activity / skin conductance
    pub eda: Option<f32>,
    /// Activity level (steps per minute or similar)
    pub activity_level: Option<f32>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

// =============================================================================
// ARIA MESSAGES
// =============================================================================

/// A message in ARIA conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ARIAMessage {
    /// Unique message ID
    pub id: Uuid,
    /// Whether this is from user or ARIA
    pub is_user: bool,
    /// Message content
    pub content: String,
    /// Detected/expressed emotional state
    pub emotional_state: Option<EmotionalState>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// ARIA's response to user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ARIAResponse {
    /// The message content
    pub message: String,
    /// Suggested actions or reflections
    pub suggestions: Vec<String>,
    /// Whether this is a growth opportunity moment
    pub growth_opportunity: bool,
    /// Emotional tone of the response
    pub tone: EmotionalState,
    /// Which ARIA agent primarily handled this
    pub primary_agent: String,
}
