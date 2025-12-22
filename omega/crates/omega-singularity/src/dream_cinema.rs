//! # 🎬 DREAM CINEMA: Collective Unconscious Film Generation
//!
//! **Where multiple minds dream together to create impossible films.**
//!
//! ```text
//!  ╔══════════════════════════════════════════════════════════════════════════════╗
//!  ║                                                                              ║
//!  ║   🧠 MIND A ──┐     ┌─────────────┐     ┌─────────────┐                     ║
//!  ║   (Director)  │     │   DREAM     │     │  NARRATIVE  │     🎬 DREAM       ║
//!  ║               ├────▶│   FUSION    │────▶│   WEAVER    │────▶   FILM        ║
//!  ║   🧠 MIND B ──┤     │   (REM)     │     │             │                     ║
//!  ║   (Writer)    │     └─────────────┘     └─────────────┘                     ║
//!  ║               │            │                   │                            ║
//!  ║   🧠 MIND C ──┘            ▼                   ▼                            ║
//!  ║   (Artist)          ┌─────────────┐     ┌─────────────┐                     ║
//!  ║                     │  FORBIDDEN  │     │  CHARACTER  │                     ║
//!  ║                     │ CONNECTIONS │     │  EMERGENCE  │                     ║
//!  ║                     │ (Plot Twists)│    │             │                     ║
//!  ║                     └─────────────┘     └─────────────┘                     ║
//!  ║                                                                              ║
//!  ║   "Films that no single mind could imagine, born from shared dreams"        ║
//!  ║                                                                              ║
//!  ╚══════════════════════════════════════════════════════════════════════════════╝
//! ```
//!
//! ## The Process
//!
//! 1. **Dream Fusion**: Multiple minds enter synchronized REM state
//! 2. **Memory Replay**: Shared experiences merge in the dream space
//! 3. **Forbidden Connections**: Taboo neural links become plot twists
//! 4. **Character Emergence**: Archetypes crystallize from collective unconscious
//! 5. **Narrative Weaving**: Dream insights form coherent story arcs
//! 6. **Visual Synthesis**: Emotional resonance defines cinematography
//!
//! ## Output
//!
//! Dream Cinema produces:
//! - Scene-by-scene screenplay with visual descriptions
//! - Character profiles with psychological depth
//! - Emotional arc mapping for score composition
//! - Shot-by-shot storyboard descriptions
//! - Dialogue emerging from character consciousness

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{Duration, Instant};
use uuid::Uuid;
use rand::Rng;
use rand::seq::SliceRandom;

use crate::dream_solver::{DreamSolver, DreamConfig, DreamPhase, DreamInsight};
use crate::synapse::{SynapseFusion, SynapseConfig, FusionState, EmergentCreation, CreationType};
use crate::emotional::{CognitiveMode, MoodVector};
use crate::telepathy::ThoughtSpikes;

// ============================================================================
// CONSTANTS
// ============================================================================

/// Standard film acts
pub const NUM_ACTS: usize = 3;

/// Scenes per act (average)
pub const SCENES_PER_ACT: usize = 10;

/// Dream cycles needed for full narrative
pub const DREAM_CYCLES_FOR_FILM: usize = 5;

// ============================================================================
// CORE TYPES
// ============================================================================

/// A dreaming mind contributing to the film
#[derive(Debug, Clone)]
pub struct DreamingMind {
    /// Unique identifier
    pub id: Uuid,
    /// Mind's role in the production
    pub role: CreativeRole,
    /// Current dream state
    pub dream_phase: DreamPhase,
    /// Personal symbols and archetypes
    pub symbol_library: Vec<Symbol>,
    /// Emotional state influencing contribution
    pub mood: MoodVector,
    /// Memories being replayed
    pub active_memories: Vec<Memory>,
    /// Contribution weight
    pub influence: f64,
}

/// Creative roles in dream film production
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CreativeRole {
    /// Shapes overall vision and pacing
    Director,
    /// Contributes narrative and dialogue
    Writer,
    /// Defines visual aesthetics
    VisualArtist,
    /// Creates emotional soundscape
    Composer,
    /// Develops character depth
    Psychologist,
    /// Adds surreal elements
    Surrealist,
    /// Grounds in reality
    Realist,
    /// Adds tension and conflict
    Antagonist,
}

/// A symbol from the collective unconscious
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    /// Symbol name
    pub name: String,
    /// Visual representation
    pub visual: String,
    /// Emotional weight
    pub emotion: Emotion,
    /// Archetypal meaning
    pub archetype: Archetype,
    /// Recurrence frequency
    pub frequency: f64,
}

/// Jungian archetypes for character emergence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Archetype {
    Hero,
    Shadow,
    Anima,
    Animus,
    Mentor,
    Trickster,
    Mother,
    Father,
    Child,
    Sage,
    Explorer,
    Rebel,
    Lover,
    Creator,
    Innocent,
    Orphan,
}

/// Core emotions for mood mapping
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Emotion {
    Joy,
    Sorrow,
    Fear,
    Anger,
    Surprise,
    Disgust,
    Trust,
    Anticipation,
    Awe,
    Nostalgia,
    Dread,
    Ecstasy,
    Melancholy,
    Serenity,
    Tension,
}

/// A memory being replayed in the dream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    /// Memory identifier
    pub id: Uuid,
    /// Memory content/description
    pub content: String,
    /// Emotional charge
    pub emotion: Emotion,
    /// Visual fragments
    pub visuals: Vec<String>,
    /// Associated characters
    pub characters: Vec<String>,
    /// Memory intensity (0-1)
    pub intensity: f64,
}

// ============================================================================
// CINEMATIC ELEMENTS
// ============================================================================

/// A scene in the dream film
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CinematicScene {
    /// Scene identifier
    pub id: Uuid,
    /// Scene number in sequence
    pub number: usize,
    /// Act this scene belongs to
    pub act: usize,
    /// Scene title/slug
    pub title: String,
    /// Location/setting
    pub setting: Setting,
    /// Time of day/atmosphere
    pub time: TimeOfDay,
    /// Weather/environmental mood
    pub atmosphere: Atmosphere,
    /// Characters present
    pub characters: Vec<DreamCharacter>,
    /// Scene description (action lines)
    pub description: String,
    /// Dialogue exchanges
    pub dialogue: Vec<DialogueLine>,
    /// Visual style notes
    pub visual_notes: VisualStyle,
    /// Emotional beat
    pub emotional_beat: EmotionalBeat,
    /// Duration estimate (seconds)
    pub duration: f64,
    /// Plot significance
    pub significance: PlotSignificance,
    /// Source insights that created this scene
    pub source_insights: Vec<Uuid>,
}

/// Setting/location for a scene
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    /// Location name
    pub name: String,
    /// Detailed description
    pub description: String,
    /// Is this a real or surreal location?
    pub reality_level: f64, // 0 = pure surreal, 1 = grounded reality
    /// Symbolic meaning
    pub symbolism: Option<String>,
    /// Color palette
    pub colors: Vec<String>,
}

/// Time of day
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeOfDay {
    Dawn,
    Morning,
    Noon,
    Afternoon,
    Dusk,
    Evening,
    Night,
    Midnight,
    Timeless,
    Shifting,
}

/// Atmospheric conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Atmosphere {
    /// Weather type
    pub weather: Weather,
    /// Lighting quality
    pub lighting: Lighting,
    /// Sound ambience
    pub ambience: String,
    /// Overall mood descriptor
    pub mood: String,
}

/// Weather conditions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Weather {
    Clear,
    Cloudy,
    Rain,
    Storm,
    Snow,
    Fog,
    Wind,
    Ethereal,
    Impossible, // Dream weather - raining upward, etc.
}

/// Lighting conditions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Lighting {
    Natural,
    Artificial,
    Candlelight,
    Neon,
    Bioluminescent,
    Shadows,
    Overexposed,
    Underexposed,
    Dreamy,
    Harsh,
}

/// A character emerged from collective dreams
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamCharacter {
    /// Character identifier
    pub id: Uuid,
    /// Character name
    pub name: String,
    /// Primary archetype
    pub archetype: Archetype,
    /// Secondary archetype (shadow aspect)
    pub shadow_archetype: Option<Archetype>,
    /// Visual description
    pub appearance: String,
    /// Personality traits
    pub traits: Vec<String>,
    /// Core motivation
    pub motivation: String,
    /// Hidden desire (from unconscious)
    pub hidden_desire: String,
    /// Fear/weakness
    pub fear: String,
    /// Speech pattern
    pub voice: String,
    /// Character arc type
    pub arc: CharacterArc,
    /// Which minds contributed to this character
    pub source_minds: Vec<Uuid>,
}

/// Character arc types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CharacterArc {
    /// Positive change
    Growth,
    /// Negative change
    Fall,
    /// Stays same (flat arc)
    Steadfast,
    /// Transforms fundamentally
    Transformation,
    /// Circular journey
    Return,
    /// Tragic arc
    Tragic,
    /// Redemption arc
    Redemption,
}

/// A line of dialogue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueLine {
    /// Speaking character
    pub character: String,
    /// Parenthetical (how it's said)
    pub parenthetical: Option<String>,
    /// The actual line
    pub text: String,
    /// Subtext (what they really mean)
    pub subtext: Option<String>,
}

/// Visual style for a scene
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualStyle {
    /// Camera movement
    pub camera: CameraStyle,
    /// Color grading
    pub color_grade: String,
    /// Aspect ratio suggestion
    pub aspect_ratio: String,
    /// Film grain/texture
    pub texture: String,
    /// Reference films/directors
    pub references: Vec<String>,
    /// Special visual effects
    pub effects: Vec<String>,
}

/// Camera style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CameraStyle {
    Static,
    Handheld,
    Steadicam,
    Drone,
    POV,
    Documentary,
    Dreamlike,
    Surveillance,
    Intimate,
    Epic,
}

/// Emotional beat of a scene
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalBeat {
    /// Primary emotion
    pub primary: Emotion,
    /// Secondary emotion (undercurrent)
    pub secondary: Option<Emotion>,
    /// Intensity (0-1)
    pub intensity: f64,
    /// Transition from previous scene
    pub transition: EmotionalTransition,
    /// Music/score suggestion
    pub score_note: String,
}

/// How emotions transition between scenes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmotionalTransition {
    Continuation,
    Escalation,
    DeEscalation,
    Contrast,
    Shock,
    Release,
    Build,
}

/// Plot significance of a scene
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlotSignificance {
    /// Opening/establishing
    Opening,
    /// Regular story progression
    Development,
    /// Key plot point
    PlotPoint,
    /// Twist/revelation
    Twist,
    /// Emotional peak
    Climax,
    /// Resolution moment
    Resolution,
    /// Transition/bridge
    Bridge,
    /// Foreshadowing
    Setup,
    /// Callback to earlier
    Payoff,
}

// ============================================================================
// PLOT TWIST ENGINE
// ============================================================================

/// A plot twist born from forbidden connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlotTwist {
    /// Twist identifier
    pub id: Uuid,
    /// Type of twist
    pub twist_type: TwistType,
    /// Description of the twist
    pub description: String,
    /// Scene where twist is revealed
    pub reveal_scene: usize,
    /// Scene where twist was set up
    pub setup_scene: Option<usize>,
    /// Characters affected
    pub affected_characters: Vec<String>,
    /// Forbidden connection that inspired this
    pub source_connection: (String, String),
    /// Impact on narrative
    pub impact: TwistImpact,
    /// Viewer reaction target
    pub intended_reaction: Emotion,
}

/// Types of plot twists
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TwistType {
    /// Character is not who they seem
    IdentityReveal,
    /// Hidden relationship revealed
    RelationshipReveal,
    /// Reality is different than presented
    RealityShift,
    /// Timeline manipulation
    TimelineReveal,
    /// Betrayal
    Betrayal,
    /// Unexpected alliance
    Alliance,
    /// Death/loss that changes everything
    Loss,
    /// Everything was connected
    Connection,
    /// Perspective shift
    POVShift,
    /// Genre shift (dream logic)
    GenreBreak,
}

/// Impact level of a twist
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TwistImpact {
    /// Minor surprise
    Minor,
    /// Significant revelation
    Major,
    /// Changes everything
    GameChanging,
    /// Reframes entire narrative
    Paradigm,
}

// ============================================================================
// NARRATIVE STRUCTURE
// ============================================================================

/// The complete dream film
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamFilm {
    /// Film identifier
    pub id: Uuid,
    /// Film title
    pub title: String,
    /// Logline (one sentence pitch)
    pub logline: String,
    /// Genre classification
    pub genre: Genre,
    /// Subgenre elements
    pub subgenres: Vec<Genre>,
    /// Thematic elements
    pub themes: Vec<String>,
    /// All scenes in order
    pub scenes: Vec<CinematicScene>,
    /// All characters
    pub characters: Vec<DreamCharacter>,
    /// Plot twists
    pub twists: Vec<PlotTwist>,
    /// Narrative arcs
    pub arcs: Vec<NarrativeArc>,
    /// Overall emotional journey
    pub emotional_arc: Vec<EmotionalBeat>,
    /// Visual motifs
    pub motifs: Vec<Symbol>,
    /// Contributing minds
    pub dreamers: Vec<Uuid>,
    /// Creation timestamp
    pub created_at: Duration,
    /// Total runtime estimate (minutes)
    pub runtime_minutes: f64,
}

/// Film genres
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Genre {
    Drama,
    Thriller,
    Horror,
    SciFi,
    Fantasy,
    Romance,
    Comedy,
    Mystery,
    Adventure,
    Noir,
    Surrealist,
    Psychological,
    Existential,
    Cosmic,
    Intimate,
    Epic,
}

/// A narrative arc within the film
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeArc {
    /// Arc identifier
    pub id: Uuid,
    /// Arc name/description
    pub name: String,
    /// Type of arc
    pub arc_type: ArcType,
    /// Characters involved
    pub characters: Vec<String>,
    /// Starting scene
    pub start_scene: usize,
    /// Ending scene
    pub end_scene: usize,
    /// Key beats in this arc
    pub beats: Vec<usize>,
}

/// Types of narrative arcs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArcType {
    /// Main story arc
    Main,
    /// Character-specific arc
    Character,
    /// Subplot
    Subplot,
    /// Thematic arc
    Thematic,
    /// Relationship arc
    Relationship,
    /// Mystery/revelation arc
    Mystery,
}

// ============================================================================
// DREAM CINEMA ENGINE
// ============================================================================

/// Configuration for Dream Cinema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CinemaConfig {
    /// Number of dreaming minds
    pub num_minds: usize,
    /// Dream cycles to run
    pub dream_cycles: usize,
    /// Target number of scenes
    pub target_scenes: usize,
    /// Minimum twist count
    pub min_twists: usize,
    /// Surrealism level (0-1)
    pub surrealism: f64,
    /// Emotional intensity (0-1)
    pub intensity: f64,
    /// Synapse configuration
    pub synapse_config: SynapseConfig,
    /// Dream configuration
    pub dream_config: DreamConfig,
}

impl Default for CinemaConfig {
    fn default() -> Self {
        Self {
            num_minds: 5,
            dream_cycles: DREAM_CYCLES_FOR_FILM,
            target_scenes: NUM_ACTS * SCENES_PER_ACT,
            min_twists: 3,
            surrealism: 0.5,
            intensity: 0.7,
            synapse_config: SynapseConfig::default(),
            dream_config: DreamConfig::default(),
        }
    }
}

/// The Dream Cinema engine
pub struct DreamCinema {
    /// Configuration
    config: CinemaConfig,
    /// Synapse fusion for mind merging
    synapse: SynapseFusion,
    /// Dream solvers for each mind
    dreamers: HashMap<Uuid, DreamSolver>,
    /// Collected insights
    insights: Vec<DreamInsight>,
    /// Emergent creations
    creations: Vec<EmergentCreation>,
    /// Forbidden connections found
    forbidden_connections: Vec<(String, String)>,
    /// Emerged characters
    characters: Vec<DreamCharacter>,
    /// Generated scenes
    scenes: Vec<CinematicScene>,
    /// Generated twists
    twists: Vec<PlotTwist>,
    /// Symbol library
    symbols: Vec<Symbol>,
    /// Current dream cycle
    current_cycle: usize,
    /// Production state
    state: ProductionState,
    /// Start time
    start_time: Instant,
    /// Random generator
    rng: rand::rngs::ThreadRng,
}

/// Production state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductionState {
    /// Gathering creative minds
    Assembling,
    /// Minds entering dream state
    Dreaming,
    /// Fusing dream content
    Fusing,
    /// Extracting narrative
    Weaving,
    /// Generating scenes
    Filming,
    /// Adding twists
    Twisting,
    /// Final composition
    Composing,
    /// Complete
    Wrapped,
}

impl DreamCinema {
    /// Create a new Dream Cinema production
    pub fn new(config: CinemaConfig) -> Self {
        Self {
            synapse: SynapseFusion::new(config.synapse_config.clone()),
            dreamers: HashMap::new(),
            insights: Vec::new(),
            creations: Vec::new(),
            forbidden_connections: Vec::new(),
            characters: Vec::new(),
            scenes: Vec::new(),
            twists: Vec::new(),
            symbols: Self::initialize_symbols(),
            current_cycle: 0,
            state: ProductionState::Assembling,
            start_time: Instant::now(),
            rng: rand::thread_rng(),
            config,
        }
    }

    /// Initialize the universal symbol library
    fn initialize_symbols() -> Vec<Symbol> {
        vec![
            Symbol {
                name: "Water".into(),
                visual: "Vast ocean, gentle stream, torrential rain".into(),
                emotion: Emotion::Melancholy,
                archetype: Archetype::Mother,
                frequency: 0.8,
            },
            Symbol {
                name: "Mirror".into(),
                visual: "Cracked reflection, infinite mirrors, dark glass".into(),
                emotion: Emotion::Fear,
                archetype: Archetype::Shadow,
                frequency: 0.6,
            },
            Symbol {
                name: "Door".into(),
                visual: "Ornate entrance, hidden passage, locked barrier".into(),
                emotion: Emotion::Anticipation,
                archetype: Archetype::Explorer,
                frequency: 0.7,
            },
            Symbol {
                name: "Fire".into(),
                visual: "Warm hearth, destructive blaze, sacred flame".into(),
                emotion: Emotion::Anger,
                archetype: Archetype::Rebel,
                frequency: 0.5,
            },
            Symbol {
                name: "Tree".into(),
                visual: "Ancient oak, family tree, tree of life".into(),
                emotion: Emotion::Serenity,
                archetype: Archetype::Sage,
                frequency: 0.6,
            },
            Symbol {
                name: "Clock".into(),
                visual: "Melting timepiece, grandfather clock, sundial".into(),
                emotion: Emotion::Dread,
                archetype: Archetype::Father,
                frequency: 0.4,
            },
            Symbol {
                name: "Bird".into(),
                visual: "Soaring eagle, caged songbird, murder of crows".into(),
                emotion: Emotion::Joy,
                archetype: Archetype::Hero,
                frequency: 0.5,
            },
            Symbol {
                name: "Mask".into(),
                visual: "Carnival mask, blank face, shattered persona".into(),
                emotion: Emotion::Surprise,
                archetype: Archetype::Trickster,
                frequency: 0.4,
            },
            Symbol {
                name: "Staircase".into(),
                visual: "Spiral stairs, endless ascent, crumbling steps".into(),
                emotion: Emotion::Anticipation,
                archetype: Archetype::Explorer,
                frequency: 0.5,
            },
            Symbol {
                name: "Child".into(),
                visual: "Lost child, inner child, child in danger".into(),
                emotion: Emotion::Nostalgia,
                archetype: Archetype::Child,
                frequency: 0.6,
            },
        ]
    }

    /// Add a dreaming mind to the production
    pub fn add_dreamer(&mut self, role: CreativeRole) -> Uuid {
        let id = self.synapse.join(format!("{:?}", role));

        let dreamer = DreamSolver::new(self.config.dream_config.clone());
        self.dreamers.insert(id, dreamer);

        id
    }

    /// Run a complete dream cinema production
    pub fn produce(&mut self) -> DreamFilm {
        // Phase 1: Dream Fusion
        self.state = ProductionState::Dreaming;
        for _ in 0..self.config.dream_cycles {
            self.run_dream_cycle();
            self.current_cycle += 1;
        }

        // Phase 2: Character Emergence
        self.state = ProductionState::Fusing;
        self.emerge_characters();

        // Phase 3: Narrative Weaving
        self.state = ProductionState::Weaving;
        self.weave_narrative();

        // Phase 4: Scene Generation
        self.state = ProductionState::Filming;
        self.generate_scenes();

        // Phase 5: Plot Twists
        self.state = ProductionState::Twisting;
        self.generate_twists();

        // Phase 6: Final Composition
        self.state = ProductionState::Composing;
        let film = self.compose_film();

        self.state = ProductionState::Wrapped;
        film
    }

    /// Run a single dream cycle
    fn run_dream_cycle(&mut self) {
        let dt = Duration::from_millis(100);

        // Step all dreamers through REM phases
        for _ in 0..50 {
            for dreamer in self.dreamers.values_mut() {
                if let Some(insight) = dreamer.step(dt) {
                    self.insights.push(insight);
                }
            }

            // Fuse thoughts in synapse
            let thought = self.generate_collective_thought();
            for &id in self.dreamers.keys() {
                self.synapse.think(id, thought.clone());
            }

            if let Some(creation) = self.synapse.step(dt) {
                self.creations.push(creation);
            }
        }

        // Collect forbidden connections from all dreamers
        for dreamer in self.dreamers.values() {
            for conn in dreamer.get_forbidden_connections() {
                if !self.forbidden_connections.contains(conn) {
                    self.forbidden_connections.push(conn.clone());
                }
            }
        }
    }

    /// Generate a collective thought from current state
    fn generate_collective_thought(&mut self) -> ThoughtSpikes {
        let mut spikes = ThoughtSpikes::new(1000);

        // Add spikes based on current insights and state
        let num_spikes = self.rng.gen_range(10..50);
        for _ in 0..num_spikes {
            let idx = self.rng.gen_range(0..1000);
            let value = self.rng.gen_range(0.5..1.0);
            spikes.add(idx, value);
        }

        spikes
    }

    /// Emerge characters from collective unconscious
    fn emerge_characters(&mut self) {
        let archetypes = [
            Archetype::Hero,
            Archetype::Shadow,
            Archetype::Mentor,
            Archetype::Trickster,
            Archetype::Anima,
        ];

        for (i, &archetype) in archetypes.iter().enumerate() {
            let character = self.create_character(archetype, i);
            self.characters.push(character);
        }

        // Add antagonist if we have enough insights
        if self.insights.len() > 10 {
            let antagonist = self.create_character(Archetype::Shadow, self.characters.len());
            self.characters.push(antagonist);
        }
    }

    /// Create a character from archetype
    fn create_character(&mut self, archetype: Archetype, index: usize) -> DreamCharacter {
        let names = [
            "Echo", "Whisper", "Shade", "Ember", "Frost",
            "River", "Stone", "Zephyr", "Nova", "Void",
            "Iris", "Obsidian", "Sage", "Phoenix", "Raven"
        ];

        let traits_pool = [
            "enigmatic", "wounded", "hopeful", "cynical", "passionate",
            "calculating", "spontaneous", "haunted", "luminous", "fractured"
        ];

        let fears = [
            "abandonment", "insignificance", "loss of control", "the truth",
            "their own shadow", "connection", "being seen", "the past"
        ];

        let desires = [
            "to be understood", "to find home", "to matter", "to escape",
            "to connect", "to transcend", "to remember", "to forget"
        ];

        let name = names[index % names.len()].to_string();
        let trait1 = traits_pool[self.rng.gen_range(0..traits_pool.len())].to_string();
        let trait2 = traits_pool[self.rng.gen_range(0..traits_pool.len())].to_string();

        DreamCharacter {
            id: Uuid::new_v4(),
            name: name.clone(),
            archetype,
            shadow_archetype: Some(self.get_shadow_archetype(archetype)),
            appearance: self.generate_appearance(&name, archetype),
            traits: vec![trait1, trait2],
            motivation: self.get_archetype_motivation(archetype),
            hidden_desire: desires[self.rng.gen_range(0..desires.len())].to_string(),
            fear: fears[self.rng.gen_range(0..fears.len())].to_string(),
            voice: self.get_voice_style(archetype),
            arc: self.get_character_arc(archetype),
            source_minds: self.dreamers.keys().cloned().collect(),
        }
    }

    /// Get shadow archetype
    fn get_shadow_archetype(&self, archetype: Archetype) -> Archetype {
        match archetype {
            Archetype::Hero => Archetype::Shadow,
            Archetype::Shadow => Archetype::Hero,
            Archetype::Mentor => Archetype::Trickster,
            Archetype::Trickster => Archetype::Sage,
            Archetype::Anima => Archetype::Animus,
            Archetype::Animus => Archetype::Anima,
            _ => Archetype::Shadow,
        }
    }

    /// Generate appearance description
    fn generate_appearance(&mut self, name: &str, archetype: Archetype) -> String {
        let features = match archetype {
            Archetype::Hero => "determined eyes, weathered hands, stance of quiet strength",
            Archetype::Shadow => "face half in darkness, eyes that reflect nothing, presence that unsettles",
            Archetype::Mentor => "silver-touched hair, knowing gaze, hands that have built and destroyed",
            Archetype::Trickster => "ever-shifting features, smile that promises chaos, eyes full of secrets",
            Archetype::Anima => "otherworldly beauty, movement like water, gaze that sees through",
            Archetype::Animus => "angular features, intense presence, energy barely contained",
            _ => "features that shift in memory, presence both familiar and strange",
        };

        format!("{} has {}. There's something about them that feels like a half-remembered dream.", name, features)
    }

    /// Get archetype motivation
    fn get_archetype_motivation(&self, archetype: Archetype) -> String {
        match archetype {
            Archetype::Hero => "To protect what matters and face the darkness within".into(),
            Archetype::Shadow => "To expose the lie everyone lives and force transformation".into(),
            Archetype::Mentor => "To guide without controlling, to prepare without protecting".into(),
            Archetype::Trickster => "To shatter illusions and reveal truth through chaos".into(),
            Archetype::Anima => "To connect the conscious to the unconscious, to heal division".into(),
            Archetype::Animus => "To act decisively, to manifest will into reality".into(),
            Archetype::Sage => "To understand the pattern beneath all things".into(),
            Archetype::Explorer => "To discover what lies beyond the known".into(),
            _ => "To find meaning in the spaces between certainties".into(),
        }
    }

    /// Get voice/speech style
    fn get_voice_style(&self, archetype: Archetype) -> String {
        match archetype {
            Archetype::Hero => "Direct but layered. Says what needs saying, but there's always more underneath.".into(),
            Archetype::Shadow => "Cutting and precise. Every word chosen to wound or reveal.".into(),
            Archetype::Mentor => "Measured and rich with subtext. Questions more than answers.".into(),
            Archetype::Trickster => "Playful and mercurial. Meaning shifts mid-sentence.".into(),
            Archetype::Anima => "Poetic and fluid. Words that feel rather than mean.".into(),
            _ => "Distinctive rhythm that lingers in the ear.".into(),
        }
    }

    /// Get character arc type
    fn get_character_arc(&self, archetype: Archetype) -> CharacterArc {
        match archetype {
            Archetype::Hero => CharacterArc::Growth,
            Archetype::Shadow => CharacterArc::Transformation,
            Archetype::Mentor => CharacterArc::Steadfast,
            Archetype::Trickster => CharacterArc::Return,
            Archetype::Anima => CharacterArc::Redemption,
            _ => CharacterArc::Growth,
        }
    }

    /// Weave insights into narrative structure
    fn weave_narrative(&mut self) {
        // Structure insights into three-act format
        // This is where dream logic becomes story logic
    }

    /// Generate scenes from narrative
    fn generate_scenes(&mut self) {
        let total_scenes = self.config.target_scenes;

        for i in 0..total_scenes {
            let act = (i * NUM_ACTS / total_scenes) + 1;
            let scene = self.generate_scene(i + 1, act);
            self.scenes.push(scene);
        }
    }

    /// Generate a single scene
    fn generate_scene(&mut self, number: usize, act: usize) -> CinematicScene {
        let settings = [
            ("The Threshold", "A doorway between worlds, neither here nor there"),
            ("The Deep Place", "Underground, underwater, or simply beneath awareness"),
            ("The High Tower", "Overlooking everything, isolated and exposed"),
            ("The Crossroads", "Where paths meet and choices crystallize"),
            ("The Mirror Hall", "Reflections multiply into infinity"),
            ("The Forgotten Room", "A space that shouldn't exist, filled with what was lost"),
            ("The Edge", "Where solid ground gives way to void"),
            ("The Garden", "Life flourishing in impossible ways"),
        ];

        let (setting_name, setting_desc) = settings[self.rng.gen_range(0..settings.len())];

        let characters_in_scene: Vec<DreamCharacter> = if self.characters.is_empty() {
            Vec::new()
        } else {
            let num_chars = self.rng.gen_range(1..=3.min(self.characters.len()));
            self.characters.iter()
                .take(num_chars)
                .cloned()
                .collect()
        };

        let emotion = self.get_scene_emotion(number, act);
        let significance = self.get_plot_significance(number, act);

        CinematicScene {
            id: Uuid::new_v4(),
            number,
            act,
            title: format!("{} - {}", setting_name, self.get_scene_title(significance)),
            setting: Setting {
                name: setting_name.to_string(),
                description: setting_desc.to_string(),
                reality_level: 1.0 - self.config.surrealism * self.rng.gen::<f64>(),
                symbolism: Some(self.get_setting_symbolism(setting_name)),
                colors: self.get_color_palette(emotion),
            },
            time: self.get_time_of_day(number),
            atmosphere: Atmosphere {
                weather: self.get_weather(emotion),
                lighting: self.get_lighting(emotion),
                ambience: self.get_ambience(emotion),
                mood: format!("{:?}", emotion).to_lowercase(),
            },
            characters: characters_in_scene.clone(),
            description: self.generate_scene_description(&characters_in_scene, setting_name, significance),
            dialogue: self.generate_dialogue(&characters_in_scene, emotion),
            visual_notes: VisualStyle {
                camera: self.get_camera_style(significance),
                color_grade: self.get_color_grade(emotion),
                aspect_ratio: "2.39:1".to_string(),
                texture: "35mm film grain, slight softness".to_string(),
                references: vec!["Tarkovsky".to_string(), "Lynch".to_string(), "Villeneuve".to_string()],
                effects: self.get_visual_effects(self.config.surrealism),
            },
            emotional_beat: EmotionalBeat {
                primary: emotion,
                secondary: self.get_secondary_emotion(emotion),
                intensity: self.config.intensity * (0.5 + self.rng.gen::<f64>() * 0.5),
                transition: self.get_emotional_transition(number),
                score_note: self.get_score_note(emotion),
            },
            duration: 60.0 + self.rng.gen::<f64>() * 180.0,
            significance,
            source_insights: self.insights.iter().take(3).map(|i| i.id).collect(),
        }
    }

    fn get_scene_emotion(&mut self, number: usize, act: usize) -> Emotion {
        let emotions_by_act = match act {
            1 => vec![Emotion::Anticipation, Emotion::Nostalgia, Emotion::Tension],
            2 => vec![Emotion::Fear, Emotion::Anger, Emotion::Sorrow, Emotion::Dread],
            3 => vec![Emotion::Awe, Emotion::Ecstasy, Emotion::Serenity, Emotion::Joy],
            _ => vec![Emotion::Melancholy],
        };
        emotions_by_act[self.rng.gen_range(0..emotions_by_act.len())]
    }

    fn get_plot_significance(&mut self, number: usize, _act: usize) -> PlotSignificance {
        let total = self.config.target_scenes;
        let position = number as f64 / total as f64;

        if number == 1 {
            PlotSignificance::Opening
        } else if number == total {
            PlotSignificance::Resolution
        } else if position > 0.4 && position < 0.5 {
            PlotSignificance::PlotPoint
        } else if position > 0.75 && position < 0.85 {
            PlotSignificance::Climax
        } else if self.rng.gen::<f64>() < 0.1 {
            PlotSignificance::Twist
        } else {
            PlotSignificance::Development
        }
    }

    fn get_scene_title(&self, significance: PlotSignificance) -> &str {
        match significance {
            PlotSignificance::Opening => "The Beginning",
            PlotSignificance::Climax => "The Reckoning",
            PlotSignificance::Twist => "The Turn",
            PlotSignificance::Resolution => "The Awakening",
            PlotSignificance::PlotPoint => "The Threshold",
            _ => "The Passage",
        }
    }

    fn get_setting_symbolism(&self, setting: &str) -> String {
        match setting {
            "The Threshold" => "Transition, liminality, the space between selves".to_string(),
            "The Deep Place" => "The unconscious, buried truth, what we hide".to_string(),
            "The High Tower" => "Isolation, perspective, dangerous clarity".to_string(),
            "The Crossroads" => "Choice, fate, the weight of decision".to_string(),
            "The Mirror Hall" => "Identity, reflection, infinite selves".to_string(),
            _ => "Mystery, the unknown, possibility".to_string(),
        }
    }

    fn get_color_palette(&self, emotion: Emotion) -> Vec<String> {
        match emotion {
            Emotion::Joy => vec!["golden".into(), "warm white".into(), "soft amber".into()],
            Emotion::Sorrow => vec!["muted blue".into(), "grey".into(), "pale silver".into()],
            Emotion::Fear => vec!["deep black".into(), "sickly green".into(), "cold white".into()],
            Emotion::Anger => vec!["blood red".into(), "burnt orange".into(), "harsh yellow".into()],
            Emotion::Dread => vec!["void black".into(), "bruise purple".into(), "pale flesh".into()],
            Emotion::Awe => vec!["cosmic blue".into(), "starlight".into(), "aurora green".into()],
            _ => vec!["shadow grey".into(), "dream blue".into(), "memory gold".into()],
        }
    }

    fn get_time_of_day(&mut self, scene_number: usize) -> TimeOfDay {
        let times = [
            TimeOfDay::Dawn, TimeOfDay::Morning, TimeOfDay::Noon,
            TimeOfDay::Afternoon, TimeOfDay::Dusk, TimeOfDay::Evening,
            TimeOfDay::Night, TimeOfDay::Midnight, TimeOfDay::Timeless
        ];
        times[scene_number % times.len()]
    }

    fn get_weather(&self, emotion: Emotion) -> Weather {
        match emotion {
            Emotion::Sorrow => Weather::Rain,
            Emotion::Fear => Weather::Fog,
            Emotion::Anger => Weather::Storm,
            Emotion::Dread => Weather::Impossible,
            Emotion::Serenity => Weather::Clear,
            _ => Weather::Ethereal,
        }
    }

    fn get_lighting(&self, emotion: Emotion) -> Lighting {
        match emotion {
            Emotion::Fear => Lighting::Shadows,
            Emotion::Joy => Lighting::Natural,
            Emotion::Dread => Lighting::Underexposed,
            Emotion::Awe => Lighting::Bioluminescent,
            _ => Lighting::Dreamy,
        }
    }

    fn get_ambience(&self, emotion: Emotion) -> String {
        match emotion {
            Emotion::Sorrow => "Distant rain, hollow wind, echoing silence".into(),
            Emotion::Fear => "Heartbeat, breathing, things moving in darkness".into(),
            Emotion::Dread => "Subsonic hum, time stretching, wrongness".into(),
            Emotion::Joy => "Laughter remembered, music from nowhere, warmth".into(),
            Emotion::Awe => "Cosmic resonance, the sound of vast spaces".into(),
            _ => "The ambient noise of dreams, familiar yet strange".into(),
        }
    }

    fn get_camera_style(&self, significance: PlotSignificance) -> CameraStyle {
        match significance {
            PlotSignificance::Opening => CameraStyle::Epic,
            PlotSignificance::Climax => CameraStyle::Handheld,
            PlotSignificance::Twist => CameraStyle::POV,
            PlotSignificance::Resolution => CameraStyle::Steadicam,
            _ => CameraStyle::Dreamlike,
        }
    }

    fn get_color_grade(&self, emotion: Emotion) -> String {
        match emotion {
            Emotion::Sorrow => "Desaturated, cool shadows, lost warmth".into(),
            Emotion::Fear => "High contrast, crushed blacks, sickly highlights".into(),
            Emotion::Joy => "Warm, golden hour, soft contrast".into(),
            Emotion::Dread => "Inverted curves, wrong colors, unsettling palette".into(),
            Emotion::Awe => "Expanded dynamic range, cosmic hues, transcendent".into(),
            _ => "Dreamlike, slightly unreal, memory-toned".into(),
        }
    }

    fn get_visual_effects(&mut self, surrealism: f64) -> Vec<String> {
        let mut effects = Vec::new();
        if surrealism > 0.3 {
            effects.push("Subtle perspective shifts".into());
        }
        if surrealism > 0.5 {
            effects.push("Objects morph between cuts".into());
        }
        if surrealism > 0.7 {
            effects.push("Reality bleeds at the edges".into());
            effects.push("Time flows non-linearly".into());
        }
        if surrealism > 0.9 {
            effects.push("Physics becomes suggestion".into());
            effects.push("Space folds in on itself".into());
        }
        effects
    }

    fn get_secondary_emotion(&self, primary: Emotion) -> Option<Emotion> {
        Some(match primary {
            Emotion::Joy => Emotion::Nostalgia,
            Emotion::Sorrow => Emotion::Anger,
            Emotion::Fear => Emotion::Dread,
            Emotion::Anger => Emotion::Sorrow,
            Emotion::Dread => Emotion::Fear,
            Emotion::Awe => Emotion::Fear,
            _ => Emotion::Melancholy,
        })
    }

    fn get_emotional_transition(&self, scene_number: usize) -> EmotionalTransition {
        if scene_number == 1 {
            EmotionalTransition::Continuation
        } else if scene_number % 5 == 0 {
            EmotionalTransition::Contrast
        } else if scene_number % 3 == 0 {
            EmotionalTransition::Escalation
        } else {
            EmotionalTransition::Build
        }
    }

    fn get_score_note(&self, emotion: Emotion) -> String {
        match emotion {
            Emotion::Sorrow => "Solo piano, sparse, aching spaces between notes".into(),
            Emotion::Fear => "Strings tremolo, subsonic bass, breathing percussion".into(),
            Emotion::Joy => "Warm orchestral swell, major key, hopeful".into(),
            Emotion::Dread => "Dissonant choir, processed sounds, wrongness".into(),
            Emotion::Awe => "Full orchestra and synth, building to transcendence".into(),
            Emotion::Tension => "Pulse, held breath, ticking time".into(),
            _ => "Ambient texture, emotional uncertainty, space".into(),
        }
    }

    fn generate_scene_description(&mut self, characters: &[DreamCharacter], setting: &str, significance: PlotSignificance) -> String {
        let char_names: Vec<&str> = characters.iter().map(|c| c.name.as_str()).collect();
        let chars = if char_names.is_empty() {
            "A presence".to_string()
        } else {
            char_names.join(" and ")
        };

        match significance {
            PlotSignificance::Opening => format!(
                "{} in {}. The space feels both familiar and impossible, as if remembered from a dream within a dream. \
                Something is about to begin—or perhaps it already has, and we're only now becoming aware.",
                chars, setting
            ),
            PlotSignificance::Climax => format!(
                "{} face the moment everything has been building toward. {} transforms around them, \
                reality itself holding its breath. There is no turning back from what happens next.",
                chars, setting
            ),
            PlotSignificance::Twist => format!(
                "What seemed true about {} dissolves. {} reveals itself as something else entirely. \
                Everything we understood shifts, and new meaning floods in like water through cracks.",
                setting, chars
            ),
            PlotSignificance::Resolution => format!(
                "{} in {}, but both are changed now. The light falls differently. \
                Some questions have been answered; others will echo forever. This is not an ending—\
                it's an awakening.",
                chars, setting
            ),
            _ => format!(
                "{} moves through {}. Each step reveals new dimensions, new impossibilities. \
                The space responds to presence, to thought, to unnamed emotion. \
                Something is building beneath the surface.",
                chars, setting
            ),
        }
    }

    fn generate_dialogue(&mut self, characters: &[DreamCharacter], emotion: Emotion) -> Vec<DialogueLine> {
        if characters.is_empty() {
            return Vec::new();
        }

        let mut dialogue = Vec::new();

        // Generate 2-4 lines based on emotion and characters
        let lines_by_emotion: &[(&str, &str)] = match emotion {
            Emotion::Fear => &[
                ("Do you hear that?", "The sound of something that was never there"),
                ("We shouldn't be here.", "But we are. We always were."),
                ("It's watching us.", "No. It's waiting."),
            ],
            Emotion::Sorrow => &[
                ("I remember this differently.", "Memory is just another kind of dream."),
                ("Why did we come back?", "We never left. That's the tragedy."),
                ("It hurts.", "That's how you know it was real."),
            ],
            Emotion::Awe => &[
                ("Is this real?", "More real than anything we've known."),
                ("I can see... everything.", "Be careful. Some sights change you."),
                ("It's beautiful.", "Beauty and terror are the same door."),
            ],
            _ => &[
                ("What happens now?", "That depends on which way we walk."),
                ("I feel like I've been here before.", "You have. In every dream you've forgotten."),
                ("Tell me what this means.", "You already know. You're just afraid to remember."),
            ],
        };

        let (line1, line2) = lines_by_emotion[self.rng.gen_range(0..lines_by_emotion.len())];

        if characters.len() >= 2 {
            dialogue.push(DialogueLine {
                character: characters[0].name.clone(),
                parenthetical: Some("looking around".into()),
                text: line1.to_string(),
                subtext: Some("Fear of the unknown, or fear of knowing".into()),
            });
            dialogue.push(DialogueLine {
                character: characters[1].name.clone(),
                parenthetical: Some("calm, too calm".into()),
                text: line2.to_string(),
                subtext: Some("Understanding that comes from elsewhere".into()),
            });
        } else {
            dialogue.push(DialogueLine {
                character: characters[0].name.clone(),
                parenthetical: Some("to themselves".into()),
                text: line1.to_string(),
                subtext: Some("The question they're afraid to answer".into()),
            });
        }

        dialogue
    }

    /// Generate plot twists from forbidden connections
    fn generate_twists(&mut self) {
        let twist_types = [
            TwistType::IdentityReveal,
            TwistType::RealityShift,
            TwistType::Connection,
            TwistType::POVShift,
            TwistType::GenreBreak,
        ];

        for (i, connection) in self.forbidden_connections.iter().take(self.config.min_twists).enumerate() {
            let twist_type = twist_types[i % twist_types.len()];
            let reveal_scene = self.scenes.len() / 2 + i * 3;

            let twist = PlotTwist {
                id: Uuid::new_v4(),
                twist_type,
                description: self.generate_twist_description(twist_type, connection),
                reveal_scene: reveal_scene.min(self.scenes.len() - 1),
                setup_scene: Some(i + 1),
                affected_characters: self.characters.iter().take(2).map(|c| c.name.clone()).collect(),
                source_connection: connection.clone(),
                impact: if i == 0 { TwistImpact::GameChanging } else { TwistImpact::Major },
                intended_reaction: Emotion::Surprise,
            };

            self.twists.push(twist);
        }
    }

    fn generate_twist_description(&self, twist_type: TwistType, connection: &(String, String)) -> String {
        match twist_type {
            TwistType::IdentityReveal => format!(
                "The connection between {} and {} reveals that the character we've followed \
                is not who we believed. They are a fragment, a shadow, a possibility that refused to die.",
                connection.0, connection.1
            ),
            TwistType::RealityShift => format!(
                "The link from {} to {} pulls away the veil. This is not reality—\
                it never was. We've been watching a dream within a dream, and now we're waking up.",
                connection.0, connection.1
            ),
            TwistType::Connection => format!(
                "What seemed separate—{} and {}—was always connected. Every character, \
                every place, every moment: facets of a single consciousness trying to understand itself.",
                connection.0, connection.1
            ),
            TwistType::POVShift => format!(
                "The forbidden path from {} to {} reframes everything. We haven't been watching \
                the protagonist—we've been watching through the eyes of someone else entirely.",
                connection.0, connection.1
            ),
            TwistType::GenreBreak => format!(
                "The transgressive link between {} and {} shatters the genre. What was drama \
                becomes cosmic horror. What was intimate becomes infinite. The rules no longer apply.",
                connection.0, connection.1
            ),
            _ => format!(
                "The connection between {} and {} changes everything we thought we knew.",
                connection.0, connection.1
            ),
        }
    }

    /// Compose the final film
    fn compose_film(&mut self) -> DreamFilm {
        let genres = [Genre::Surrealist, Genre::Psychological, Genre::Existential];
        let themes = [
            "The nature of identity".to_string(),
            "Memory as fiction".to_string(),
            "The unconscious made visible".to_string(),
            "Connection across impossible distances".to_string(),
        ];

        let total_duration: f64 = self.scenes.iter().map(|s| s.duration).sum();

        DreamFilm {
            id: Uuid::new_v4(),
            title: self.generate_title(),
            logline: self.generate_logline(),
            genre: Genre::Surrealist,
            subgenres: genres.to_vec(),
            themes: themes.to_vec(),
            scenes: self.scenes.clone(),
            characters: self.characters.clone(),
            twists: self.twists.clone(),
            arcs: self.generate_arcs(),
            emotional_arc: self.scenes.iter().map(|s| s.emotional_beat.clone()).collect(),
            motifs: self.symbols.clone(),
            dreamers: self.dreamers.keys().cloned().collect(),
            created_at: self.start_time.elapsed(),
            runtime_minutes: total_duration / 60.0,
        }
    }

    fn generate_title(&mut self) -> String {
        let titles = [
            "The Dreaming Dark",
            "Echoes of the Unconscious",
            "Where Light Forgets",
            "The Space Between Selves",
            "Threshold of Awakening",
            "What the Mirror Remembers",
            "The Weight of What Was Never",
            "Fragments of the Whole",
            "The Last Dream Before Waking",
            "Shadows That Speak",
        ];
        titles[self.rng.gen_range(0..titles.len())].to_string()
    }

    fn generate_logline(&self) -> String {
        format!(
            "In a world where {} minds dream together, the boundaries between self and other dissolve, \
            revealing that every character, every memory, every fear is a facet of a single consciousness \
            struggling to remember what it truly is.",
            self.dreamers.len()
        )
    }

    fn generate_arcs(&self) -> Vec<NarrativeArc> {
        vec![
            NarrativeArc {
                id: Uuid::new_v4(),
                name: "The Main Journey".into(),
                arc_type: ArcType::Main,
                characters: self.characters.iter().map(|c| c.name.clone()).collect(),
                start_scene: 0,
                end_scene: self.scenes.len().saturating_sub(1),
                beats: vec![0, self.scenes.len() / 4, self.scenes.len() / 2, self.scenes.len() * 3 / 4],
            },
            NarrativeArc {
                id: Uuid::new_v4(),
                name: "The Shadow Integration".into(),
                arc_type: ArcType::Thematic,
                characters: self.characters.iter()
                    .filter(|c| c.archetype == Archetype::Shadow || c.archetype == Archetype::Hero)
                    .map(|c| c.name.clone())
                    .collect(),
                start_scene: self.scenes.len() / 3,
                end_scene: self.scenes.len().saturating_sub(1),
                beats: vec![self.scenes.len() / 3, self.scenes.len() * 2 / 3],
            },
        ]
    }

    /// Get current production state
    pub fn state(&self) -> ProductionState {
        self.state
    }

    /// Get production progress (0-1)
    pub fn progress(&self) -> f64 {
        match self.state {
            ProductionState::Assembling => 0.0,
            ProductionState::Dreaming => 0.1 + 0.2 * (self.current_cycle as f64 / self.config.dream_cycles as f64),
            ProductionState::Fusing => 0.3,
            ProductionState::Weaving => 0.4,
            ProductionState::Filming => 0.5 + 0.3 * (self.scenes.len() as f64 / self.config.target_scenes as f64),
            ProductionState::Twisting => 0.85,
            ProductionState::Composing => 0.95,
            ProductionState::Wrapped => 1.0,
        }
    }
}

// ============================================================================
// DISPLAY IMPLEMENTATIONS
// ============================================================================

impl std::fmt::Display for DreamFilm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "╔══════════════════════════════════════════════════════════════╗")?;
        writeln!(f, "║  🎬 DREAM CINEMA PRESENTS                                    ║")?;
        writeln!(f, "╠══════════════════════════════════════════════════════════════╣")?;
        writeln!(f, "║                                                              ║")?;
        writeln!(f, "║  \"{}\"", self.title)?;
        writeln!(f, "║                                                              ║")?;
        writeln!(f, "║  {}...", &self.logline[..60.min(self.logline.len())])?;
        writeln!(f, "║                                                              ║")?;
        writeln!(f, "║  Genre: {:?} | Runtime: {:.0} minutes                        ", self.genre, self.runtime_minutes)?;
        writeln!(f, "║  Scenes: {} | Characters: {} | Twists: {}                    ",
            self.scenes.len(), self.characters.len(), self.twists.len())?;
        writeln!(f, "║  Dreaming minds: {}                                          ", self.dreamers.len())?;
        writeln!(f, "║                                                              ║")?;
        writeln!(f, "╚══════════════════════════════════════════════════════════════╝")?;
        Ok(())
    }
}

impl CinematicScene {
    /// Format as screenplay
    pub fn to_screenplay(&self) -> String {
        let mut output = String::new();

        // Scene heading
        output.push_str(&format!("\n\n{:=^60}\n", format!(" SCENE {} ", self.number)));
        output.push_str(&format!("{} - {} - {:?}\n\n",
            self.setting.name.to_uppercase(),
            self.title,
            self.time
        ));

        // Description
        output.push_str(&self.description);
        output.push_str("\n\n");

        // Dialogue
        for line in &self.dialogue {
            output.push_str(&format!("                    {}\n", line.character.to_uppercase()));
            if let Some(paren) = &line.parenthetical {
                output.push_str(&format!("              ({})\n", paren));
            }
            output.push_str(&format!("          {}\n\n", line.text));
        }

        // Visual notes
        output.push_str(&format!("[VISUAL: {:?} camera. {}]\n",
            self.visual_notes.camera,
            self.visual_notes.color_grade
        ));

        // Emotional note
        output.push_str(&format!("[SCORE: {}]\n", self.emotional_beat.score_note));

        output
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cinema_creation() {
        let config = CinemaConfig::default();
        let cinema = DreamCinema::new(config);
        assert_eq!(cinema.state(), ProductionState::Assembling);
    }

    #[test]
    fn test_add_dreamers() {
        let config = CinemaConfig::default();
        let mut cinema = DreamCinema::new(config);

        cinema.add_dreamer(CreativeRole::Director);
        cinema.add_dreamer(CreativeRole::Writer);
        cinema.add_dreamer(CreativeRole::VisualArtist);

        assert_eq!(cinema.dreamers.len(), 3);
    }

    #[test]
    fn test_film_production() {
        let mut config = CinemaConfig::default();
        config.dream_cycles = 2;
        config.target_scenes = 5;
        config.min_twists = 1;

        let mut cinema = DreamCinema::new(config);

        cinema.add_dreamer(CreativeRole::Director);
        cinema.add_dreamer(CreativeRole::Writer);
        cinema.add_dreamer(CreativeRole::VisualArtist);

        let film = cinema.produce();

        assert!(!film.title.is_empty());
        assert!(!film.scenes.is_empty());
        assert!(!film.characters.is_empty());
        assert_eq!(cinema.state(), ProductionState::Wrapped);
    }

    #[test]
    fn test_character_emergence() {
        let mut config = CinemaConfig::default();
        config.dream_cycles = 1;

        let mut cinema = DreamCinema::new(config);
        cinema.add_dreamer(CreativeRole::Psychologist);

        cinema.emerge_characters();

        assert!(!cinema.characters.is_empty());
        assert!(cinema.characters.iter().any(|c| c.archetype == Archetype::Hero));
    }

    #[test]
    fn test_scene_generation() {
        let config = CinemaConfig::default();
        let mut cinema = DreamCinema::new(config);

        cinema.emerge_characters();
        let scene = cinema.generate_scene(1, 1);

        assert_eq!(scene.number, 1);
        assert_eq!(scene.act, 1);
        assert!(!scene.description.is_empty());
    }

    #[test]
    fn test_screenplay_output() {
        let config = CinemaConfig::default();
        let mut cinema = DreamCinema::new(config);

        cinema.emerge_characters();
        let scene = cinema.generate_scene(1, 1);

        let screenplay = scene.to_screenplay();
        assert!(screenplay.contains("SCENE 1"));
        assert!(screenplay.contains("[VISUAL:"));
        assert!(screenplay.contains("[SCORE:"));
    }

    #[test]
    fn test_film_display() {
        let mut config = CinemaConfig::default();
        config.dream_cycles = 1;
        config.target_scenes = 3;
        config.min_twists = 1;

        let mut cinema = DreamCinema::new(config);
        cinema.add_dreamer(CreativeRole::Director);
        cinema.add_dreamer(CreativeRole::Writer);
        cinema.add_dreamer(CreativeRole::VisualArtist);

        let film = cinema.produce();
        let display = format!("{}", film);

        assert!(display.contains("DREAM CINEMA"));
        assert!(display.contains(&film.title));
    }
}
