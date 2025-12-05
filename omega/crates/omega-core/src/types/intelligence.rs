use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub type IntelligenceId = String;
pub type ArchitectureId = String;
pub type CapabilityId = String;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Paradigm {
    Neural,
    Symbolic,
    Quantum,
    Biological,
    Social,
    Physical,
    Hybrid,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubstrateType {
    Digital,
    Biological,
    Social,
    Ecological,
    Geological,
    Stellar,
    Galactic,
    Cosmic,
    Transcendent,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IntelligenceStatus {
    Initializing,
    Running,
    Paused,
    Learning,
    Evolving,
    Stopped,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub id: CapabilityId,
    pub name: String,
    pub description: String,
    pub category: CapabilityCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CapabilityCategory {
    Reasoning,
    Memory,
    Learning,
    Creation,
    Communication,
    Planning,
    Perception,
    Action,
    MetaCognition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Architecture {
    pub id: ArchitectureId,
    pub name: String,
    pub paradigm: Paradigm,
    pub substrate: SubstrateType,
    pub fitness: Option<FitnessScore>,
    pub lineage: Vec<ArchitectureId>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitnessScore {
    pub overall: f64,
    pub capability: f64,
    pub efficiency: f64,
    pub alignment: f64,
    pub novelty: f64,
    pub confidence: f64,
}

impl Default for FitnessScore {
    fn default() -> Self {
        Self {
            overall: 0.5,
            capability: 0.5,
            efficiency: 0.5,
            alignment: 0.5,
            novelty: 0.5,
            confidence: 0.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intelligence {
    pub id: IntelligenceId,
    pub name: String,
    pub architecture: Architecture,
    pub capabilities: Vec<Capability>,
    pub status: IntelligenceStatus,
    pub generation: u32,
    pub created_at: DateTime<Utc>,
}

impl Intelligence {
    pub fn new(name: String, architecture: Architecture) -> Self {
        Self {
            id: Uuid::now_v7().to_string(),
            name,
            architecture,
            capabilities: Vec::new(),
            status: IntelligenceStatus::Initializing,
            generation: 0,
            created_at: Utc::now(),
        }
    }
}
