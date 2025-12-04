use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;

pub type MemoryId = String;
pub type TierId = u8;

/// 12-Tier Cosmic Memory System
/// Each tier represents a different temporal and spatial scale of memory
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MemoryTier {
    /// Tier 1: Immediate (milliseconds) - Working memory, current context
    Immediate = 1,
    /// Tier 2: Short-term (seconds to minutes) - Recent interactions
    ShortTerm = 2,
    /// Tier 3: Session (hours) - Current session context
    Session = 3,
    /// Tier 4: Episodic (days) - Specific events and experiences
    Episodic = 4,
    /// Tier 5: Semantic (weeks) - Facts, concepts, knowledge
    Semantic = 5,
    /// Tier 6: Procedural (months) - Skills and procedures
    Procedural = 6,
    /// Tier 7: Strategic (years) - Long-term strategies and patterns
    Strategic = 7,
    /// Tier 8: Civilizational (decades to centuries) - Cultural and societal knowledge
    Civilizational = 8,
    /// Tier 9: Evolutionary (millennia) - Species-level adaptations
    Evolutionary = 9,
    /// Tier 10: Planetary (millions of years) - Planetary-scale patterns
    Planetary = 10,
    /// Tier 11: Galactic (billions of years) - Galactic-scale knowledge
    Galactic = 11,
    /// Tier 12: Cosmic (age of universe) - Universal constants and principles
    Cosmic = 12,
}

impl MemoryTier {
    pub fn retention_duration(&self) -> Option<Duration> {
        match self {
            MemoryTier::Immediate => Some(Duration::milliseconds(1000)),
            MemoryTier::ShortTerm => Some(Duration::minutes(10)),
            MemoryTier::Session => Some(Duration::hours(24)),
            MemoryTier::Episodic => Some(Duration::days(30)),
            MemoryTier::Semantic => Some(Duration::days(365)),
            MemoryTier::Procedural => Some(Duration::days(365 * 5)),
            MemoryTier::Strategic => Some(Duration::days(365 * 50)),
            MemoryTier::Civilizational => Some(Duration::days(365 * 500)),
            MemoryTier::Evolutionary => Some(Duration::days(365 * 10_000)),
            MemoryTier::Planetary => None, // Geological time - effectively permanent
            MemoryTier::Galactic => None,  // Effectively permanent
            MemoryTier::Cosmic => None,    // Universal constants - permanent
        }
    }

    pub fn all_tiers() -> Vec<MemoryTier> {
        vec![
            MemoryTier::Immediate,
            MemoryTier::ShortTerm,
            MemoryTier::Session,
            MemoryTier::Episodic,
            MemoryTier::Semantic,
            MemoryTier::Procedural,
            MemoryTier::Strategic,
            MemoryTier::Civilizational,
            MemoryTier::Evolutionary,
            MemoryTier::Planetary,
            MemoryTier::Galactic,
            MemoryTier::Cosmic,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryType {
    Experience,
    Knowledge,
    Skill,
    Pattern,
    Insight,
    Relationship,
    Concept,
    Goal,
    Value,
    Principle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetadata {
    pub importance: f64,      // 0.0 to 1.0
    pub confidence: f64,      // 0.0 to 1.0
    pub access_count: u64,
    pub last_accessed: DateTime<Utc>,
    pub source: String,
    pub tags: Vec<String>,
    pub associations: Vec<MemoryId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub id: MemoryId,
    pub tier: MemoryTier,
    pub memory_type: MemoryType,
    pub content: MemoryContent,
    pub metadata: MemoryMetadata,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryContent {
    Text(String),
    Structured(HashMap<String, serde_json::Value>),
    Vector(Vec<f64>),
    Graph(GraphMemory),
    Hybrid {
        text: Option<String>,
        structured: Option<HashMap<String, serde_json::Value>>,
        vector: Option<Vec<f64>>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphMemory {
    pub nodes: Vec<MemoryNode>,
    pub edges: Vec<MemoryEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryNode {
    pub id: String,
    pub label: String,
    pub properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEdge {
    pub from: String,
    pub to: String,
    pub relationship: String,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryQuery {
    pub tiers: Vec<MemoryTier>,
    pub memory_types: Option<Vec<MemoryType>>,
    pub tags: Option<Vec<String>>,
    pub min_importance: Option<f64>,
    pub min_confidence: Option<f64>,
    pub time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    pub limit: Option<usize>,
}

impl Memory {
    pub fn new(
        tier: MemoryTier,
        memory_type: MemoryType,
        content: MemoryContent,
        importance: f64,
    ) -> Self {
        let created_at = Utc::now();
        let expires_at = tier.retention_duration().map(|d| created_at + d);

        Self {
            id: Uuid::now_v7().to_string(),
            tier,
            memory_type,
            content,
            metadata: MemoryMetadata {
                importance,
                confidence: 1.0,
                access_count: 0,
                last_accessed: created_at,
                source: String::from("unknown"),
                tags: Vec::new(),
                associations: Vec::new(),
            },
            created_at,
            expires_at,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at
            .map(|exp| exp < Utc::now())
            .unwrap_or(false)
    }

    pub fn access(&mut self) {
        self.metadata.access_count += 1;
        self.metadata.last_accessed = Utc::now();
    }
}
