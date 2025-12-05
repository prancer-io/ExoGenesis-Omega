//! Species scale memory (Tier 5-8)
//! Collective, evolutionary, architectural, and substrate memory

use crate::{Memory, MemoryError, MemoryTier, Query};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Species memory system managing Tier 5-8
pub struct SpeciesMemory {
    collective: HashMap<String, Memory>,
    evolutionary: HashMap<String, Memory>,
    architectural: HashMap<String, Memory>,
    substrate: HashMap<String, Memory>,
}

impl SpeciesMemory {
    pub async fn new() -> Result<Self, MemoryError> {
        Ok(Self {
            collective: HashMap::new(),
            evolutionary: HashMap::new(),
            architectural: HashMap::new(),
            substrate: HashMap::new(),
        })
    }

    pub async fn store(&mut self, memory: Memory) -> Result<(), MemoryError> {
        match memory.tier {
            MemoryTier::Collective => {
                self.collective.insert(memory.id.clone(), memory);
            }
            MemoryTier::Evolutionary => {
                self.evolutionary.insert(memory.id.clone(), memory);
            }
            MemoryTier::Architectural => {
                self.architectural.insert(memory.id.clone(), memory);
            }
            MemoryTier::Substrate => {
                self.substrate.insert(memory.id.clone(), memory);
            }
            _ => {
                return Err(MemoryError::Storage(format!(
                    "Invalid tier {:?} for species memory",
                    memory.tier
                )));
            }
        }

        Ok(())
    }

    pub async fn recall(
        &self,
        query: &Query,
        tiers: &[MemoryTier],
    ) -> Result<Vec<Memory>, MemoryError> {
        let mut results = Vec::new();

        for tier in tiers {
            let tier_memories = match tier {
                MemoryTier::Collective => &self.collective,
                MemoryTier::Evolutionary => &self.evolutionary,
                MemoryTier::Architectural => &self.architectural,
                MemoryTier::Substrate => &self.substrate,
                _ => continue,
            };

            let memories: Vec<Memory> = tier_memories
                .values()
                .filter(|m| {
                    if let Some(min_importance) = query.min_importance {
                        m.importance >= min_importance
                    } else {
                        true
                    }
                })
                .cloned()
                .collect();

            results.extend(memories);
        }

        Ok(results)
    }

    pub async fn stats(&self) -> SpeciesMemoryStats {
        SpeciesMemoryStats {
            collective: self.collective.len(),
            evolutionary: self.evolutionary.len(),
            architectural: self.architectural.len(),
            substrate: self.substrate.len(),
            total: self.collective.len()
                + self.evolutionary.len()
                + self.architectural.len()
                + self.substrate.len(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeciesMemoryStats {
    pub collective: usize,
    pub evolutionary: usize,
    pub architectural: usize,
    pub substrate: usize,
    pub total: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MemoryContent;

    #[tokio::test]
    async fn test_species_memory_storage() {
        let mut mem = SpeciesMemory::new().await.unwrap();
        let memory = Memory::new(
            MemoryTier::Collective,
            MemoryContent::Text("collective knowledge".to_string()),
            vec![0.5, 0.5, 0.5],
            0.7,
        );

        mem.store(memory).await.unwrap();
        let stats = mem.stats().await;
        assert_eq!(stats.collective, 1);
    }
}
