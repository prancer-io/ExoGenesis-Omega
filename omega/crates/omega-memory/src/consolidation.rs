//! Memory consolidation between tiers

use crate::{
    cosmic::CosmicScaleMemory, individual::IndividualMemory, species::SpeciesMemory, Memory,
    MemoryError, MemoryTier, Query, QueryBuilder,
};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Handles consolidation of memories between tiers
pub struct MemoryConsolidator {
    individual: Arc<RwLock<IndividualMemory>>,
    species: Arc<RwLock<SpeciesMemory>>,
    cosmic_scale: Arc<RwLock<CosmicScaleMemory>>,
}

impl MemoryConsolidator {
    pub fn new(
        individual: Arc<RwLock<IndividualMemory>>,
        species: Arc<RwLock<SpeciesMemory>>,
        cosmic_scale: Arc<RwLock<CosmicScaleMemory>>,
    ) -> Self {
        Self {
            individual,
            species,
            cosmic_scale,
        }
    }

    /// Consolidate memories from one tier to the next
    pub async fn consolidate(
        &self,
        from_tier: MemoryTier,
        to_tier: MemoryTier,
    ) -> Result<(), MemoryError> {
        // Validate consolidation path
        if from_tier >= to_tier {
            return Err(MemoryError::Consolidation(
                "Can only consolidate to higher tiers".to_string(),
            ));
        }

        // Get memories from source tier
        let query = QueryBuilder::new()
            .tier(from_tier)
            .min_importance(to_tier.importance_threshold())
            .build();

        let memories = match from_tier.scale() {
            crate::tiers::MemoryScale::Individual => {
                self.individual
                    .read()
                    .await
                    .recall(&query, &[from_tier])
                    .await?
            }
            crate::tiers::MemoryScale::Species => {
                self.species
                    .read()
                    .await
                    .recall(&query, &[from_tier])
                    .await?
            }
            crate::tiers::MemoryScale::Cosmic => {
                self.cosmic_scale
                    .read()
                    .await
                    .recall(&query, &[from_tier])
                    .await?
            }
        };

        // Consolidate and store in target tier
        for memory in memories {
            let consolidated = self.consolidate_memory(memory, to_tier)?;

            match to_tier.scale() {
                crate::tiers::MemoryScale::Individual => {
                    self.individual.write().await.store(consolidated).await?;
                }
                crate::tiers::MemoryScale::Species => {
                    self.species.write().await.store(consolidated).await?;
                }
                crate::tiers::MemoryScale::Cosmic => {
                    self.cosmic_scale.write().await.store(consolidated).await?;
                }
            }
        }

        Ok(())
    }

    /// Automatically consolidate based on age and importance
    pub async fn auto_consolidate(&self) -> Result<(), MemoryError> {
        // Consolidate Instant -> Session
        self.consolidate_by_importance(MemoryTier::Instant, MemoryTier::Session)
            .await?;

        // Consolidate Session -> Episodic
        self.consolidate_by_importance(MemoryTier::Session, MemoryTier::Episodic)
            .await?;

        // Consolidate Episodic -> Semantic
        self.consolidate_by_importance(MemoryTier::Episodic, MemoryTier::Semantic)
            .await?;

        // Consolidate Semantic -> Collective
        self.consolidate_by_importance(MemoryTier::Semantic, MemoryTier::Collective)
            .await?;

        Ok(())
    }

    async fn consolidate_by_importance(
        &self,
        from_tier: MemoryTier,
        to_tier: MemoryTier,
    ) -> Result<(), MemoryError> {
        let query = QueryBuilder::new()
            .tier(from_tier)
            .min_importance(to_tier.importance_threshold())
            .build();

        let memories = match from_tier.scale() {
            crate::tiers::MemoryScale::Individual => {
                self.individual
                    .read()
                    .await
                    .recall(&query, &[from_tier])
                    .await?
            }
            crate::tiers::MemoryScale::Species => {
                self.species
                    .read()
                    .await
                    .recall(&query, &[from_tier])
                    .await?
            }
            crate::tiers::MemoryScale::Cosmic => {
                self.cosmic_scale
                    .read()
                    .await
                    .recall(&query, &[from_tier])
                    .await?
            }
        };

        // Consolidate high-importance memories
        for memory in memories {
            if memory.importance >= to_tier.importance_threshold() {
                let consolidated = self.consolidate_memory(memory, to_tier)?;

                match to_tier.scale() {
                    crate::tiers::MemoryScale::Individual => {
                        self.individual.write().await.store(consolidated).await?;
                    }
                    crate::tiers::MemoryScale::Species => {
                        self.species.write().await.store(consolidated).await?;
                    }
                    crate::tiers::MemoryScale::Cosmic => {
                        self.cosmic_scale.write().await.store(consolidated).await?;
                    }
                }
            }
        }

        Ok(())
    }

    fn consolidate_memory(&self, mut memory: Memory, to_tier: MemoryTier) -> Result<Memory, MemoryError> {
        // Update tier
        memory.tier = to_tier;

        // Boost importance slightly for consolidation
        memory.importance = (memory.importance * 1.1).min(1.0);

        // Generate new ID for consolidated memory
        memory.id = uuid::Uuid::new_v4().to_string();

        // Add consolidation metadata
        if let serde_json::Value::Object(ref mut map) = memory.metadata {
            map.insert(
                "consolidated_at".to_string(),
                serde_json::Value::String(chrono::Utc::now().to_rfc3339()),
            );
            map.insert(
                "consolidation_tier".to_string(),
                serde_json::Value::Number(serde_json::Number::from(to_tier as u8)),
            );
        }

        Ok(memory)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MemoryContent;

    #[tokio::test]
    async fn test_consolidation() {
        let individual = Arc::new(RwLock::new(IndividualMemory::new().await.unwrap()));
        let species = Arc::new(RwLock::new(SpeciesMemory::new().await.unwrap()));
        let cosmic = Arc::new(RwLock::new(CosmicScaleMemory::new().await.unwrap()));

        let consolidator = MemoryConsolidator::new(individual.clone(), species, cosmic);

        // Store a high-importance instant memory
        let memory = Memory::new(
            MemoryTier::Instant,
            MemoryContent::Text("important".to_string()),
            vec![0.5, 0.5, 0.5],
            0.9,
        );

        individual.write().await.store(memory).await.unwrap();

        // Consolidate to session
        consolidator
            .consolidate(MemoryTier::Instant, MemoryTier::Session)
            .await
            .unwrap();
    }
}
