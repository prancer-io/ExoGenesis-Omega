//! # Omega Memory - 12-Tier Cosmic Memory System
//!
//! Implements hierarchical memory spanning from instant (milliseconds)
//! to omega (universe-scale) timescales.

pub mod tiers;
pub mod individual;
pub mod species;
pub mod cosmic;
pub mod query;
pub mod consolidation;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

pub use tiers::MemoryTier;
pub use query::{Query, QueryBuilder};
pub use consolidation::MemoryConsolidator;

/// Memory content types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryContent {
    /// Raw sensory data
    Sensory(Vec<u8>),
    /// Text-based memory
    Text(String),
    /// Structured data
    Structured(serde_json::Value),
    /// Embedding vector
    Embedding(Vec<f32>),
    /// Multi-modal memory
    MultiModal {
        text: Option<String>,
        embedding: Vec<f32>,
        metadata: serde_json::Value,
    },
}

/// Core memory structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub id: String,
    pub tier: MemoryTier,
    pub content: MemoryContent,
    pub embedding: Vec<f32>,
    pub importance: f64,
    pub created_at: DateTime<Utc>,
    pub accessed_at: DateTime<Utc>,
    pub access_count: u64,
    pub metadata: serde_json::Value,
}

impl Memory {
    pub fn new(
        tier: MemoryTier,
        content: MemoryContent,
        embedding: Vec<f32>,
        importance: f64,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            tier,
            content,
            embedding,
            importance,
            created_at: now,
            accessed_at: now,
            access_count: 0,
            metadata: serde_json::Value::Object(Default::default()),
        }
    }

    pub fn touch(&mut self) {
        self.accessed_at = Utc::now();
        self.access_count += 1;
    }

    pub fn relevance_score(&self) -> f64 {
        let time_decay = self.time_decay_factor();
        let access_boost = (self.access_count as f64).ln_1p() * 0.1;
        self.importance * time_decay + access_boost
    }

    fn time_decay_factor(&self) -> f64 {
        let age = Utc::now().signed_duration_since(self.accessed_at);
        let hours = age.num_hours() as f64;

        match self.tier {
            MemoryTier::Instant => (-hours / 0.01).exp(),
            MemoryTier::Session => (-hours / 24.0).exp(),
            MemoryTier::Episodic => (-hours / 168.0).exp(), // 1 week
            MemoryTier::Semantic => 1.0, // No decay for semantic
            _ => 1.0, // Higher tiers don't decay
        }
    }
}

/// Memory errors
#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Query error: {0}")]
    Query(String),

    #[error("Consolidation error: {0}")]
    Consolidation(String),

    #[error("AgentDB error: {0}")]
    AgentDB(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Main Cosmic Memory system integrating all tiers
pub struct CosmicMemory {
    individual: Arc<RwLock<individual::IndividualMemory>>,
    species: Arc<RwLock<species::SpeciesMemory>>,
    cosmic_scale: Arc<RwLock<cosmic::CosmicScaleMemory>>,
    consolidator: Arc<MemoryConsolidator>,
}

impl CosmicMemory {
    pub async fn new() -> Result<Self, MemoryError> {
        let individual = Arc::new(RwLock::new(individual::IndividualMemory::new().await?));
        let species = Arc::new(RwLock::new(species::SpeciesMemory::new().await?));
        let cosmic_scale = Arc::new(RwLock::new(cosmic::CosmicScaleMemory::new().await?));

        let consolidator = Arc::new(MemoryConsolidator::new(
            individual.clone(),
            species.clone(),
            cosmic_scale.clone(),
        ));

        Ok(Self {
            individual,
            species,
            cosmic_scale,
            consolidator,
        })
    }

    /// Store a memory in the appropriate tier
    pub async fn store(&self, memory: Memory) -> Result<String, MemoryError> {
        let id = memory.id.clone();

        match memory.tier {
            MemoryTier::Instant | MemoryTier::Session | MemoryTier::Episodic | MemoryTier::Semantic => {
                self.individual.write().await.store(memory).await?;
            }
            MemoryTier::Collective | MemoryTier::Evolutionary | MemoryTier::Architectural | MemoryTier::Substrate => {
                self.species.write().await.store(memory).await?;
            }
            MemoryTier::Civilizational | MemoryTier::Temporal | MemoryTier::Physical | MemoryTier::Omega => {
                self.cosmic_scale.write().await.store(memory).await?;
            }
        }

        Ok(id)
    }

    /// Recall memories matching the query across specified tiers
    pub async fn recall(
        &self,
        query: &Query,
        tiers: &[MemoryTier],
    ) -> Result<Vec<Memory>, MemoryError> {
        let mut results = Vec::new();

        // Query individual tiers (1-4)
        if tiers.iter().any(|t| matches!(t, MemoryTier::Instant | MemoryTier::Session | MemoryTier::Episodic | MemoryTier::Semantic)) {
            let individual_results = self.individual.read().await.recall(query, tiers).await?;
            results.extend(individual_results);
        }

        // Query species tiers (5-8)
        if tiers.iter().any(|t| matches!(t, MemoryTier::Collective | MemoryTier::Evolutionary | MemoryTier::Architectural | MemoryTier::Substrate)) {
            let species_results = self.species.read().await.recall(query, tiers).await?;
            results.extend(species_results);
        }

        // Query cosmic tiers (9-12)
        if tiers.iter().any(|t| matches!(t, MemoryTier::Civilizational | MemoryTier::Temporal | MemoryTier::Physical | MemoryTier::Omega)) {
            let cosmic_results = self.cosmic_scale.read().await.recall(query, tiers).await?;
            results.extend(cosmic_results);
        }

        // Sort by relevance
        results.sort_by(|a, b| {
            b.relevance_score()
                .partial_cmp(&a.relevance_score())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(results)
    }

    /// Consolidate memories from one tier to another
    pub async fn consolidate(
        &self,
        from_tier: MemoryTier,
        to_tier: MemoryTier,
    ) -> Result<(), MemoryError> {
        self.consolidator.consolidate(from_tier, to_tier).await
    }

    /// Run automatic consolidation based on importance and age
    pub async fn auto_consolidate(&self) -> Result<(), MemoryError> {
        self.consolidator.auto_consolidate().await
    }

    /// Get statistics about memory usage across all tiers
    pub async fn stats(&self) -> MemoryStats {
        let individual_stats = self.individual.read().await.stats().await;
        let species_stats = self.species.read().await.stats().await;
        let cosmic_stats = self.cosmic_scale.read().await.stats().await;

        MemoryStats {
            individual: individual_stats,
            species: species_stats,
            cosmic: cosmic_stats,
            total_memories: individual_stats.total + species_stats.total + cosmic_stats.total,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub individual: individual::IndividualMemoryStats,
    pub species: species::SpeciesMemoryStats,
    pub cosmic: cosmic::CosmicMemoryStats,
    pub total_memories: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_creation() {
        let memory = Memory::new(
            MemoryTier::Session,
            MemoryContent::Text("test".to_string()),
            vec![0.1, 0.2, 0.3],
            0.5,
        );

        assert_eq!(memory.tier, MemoryTier::Session);
        assert_eq!(memory.importance, 0.5);
        assert_eq!(memory.access_count, 0);
    }

    #[tokio::test]
    async fn test_cosmic_memory_init() {
        let result = CosmicMemory::new().await;
        assert!(result.is_ok());
    }
}
