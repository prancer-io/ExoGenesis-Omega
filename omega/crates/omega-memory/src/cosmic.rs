//! Cosmic scale memory (Tier 9-12)
//! Civilizational, temporal, physical, and omega memory

use crate::{Memory, MemoryError, MemoryTier, Query};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Cosmic scale memory system managing Tier 9-12
pub struct CosmicScaleMemory {
    civilizational: HashMap<String, Memory>,
    temporal: HashMap<String, Memory>,
    physical: HashMap<String, Memory>,
    omega: HashMap<String, Memory>,
}

impl CosmicScaleMemory {
    pub async fn new() -> Result<Self, MemoryError> {
        Ok(Self {
            civilizational: HashMap::new(),
            temporal: HashMap::new(),
            physical: HashMap::new(),
            omega: HashMap::new(),
        })
    }

    pub async fn store(&mut self, memory: Memory) -> Result<(), MemoryError> {
        match memory.tier {
            MemoryTier::Civilizational => {
                self.civilizational.insert(memory.id.clone(), memory);
            }
            MemoryTier::Temporal => {
                self.temporal.insert(memory.id.clone(), memory);
            }
            MemoryTier::Physical => {
                self.physical.insert(memory.id.clone(), memory);
            }
            MemoryTier::Omega => {
                self.omega.insert(memory.id.clone(), memory);
            }
            _ => {
                return Err(MemoryError::Storage(format!(
                    "Invalid tier {:?} for cosmic memory",
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
                MemoryTier::Civilizational => &self.civilizational,
                MemoryTier::Temporal => &self.temporal,
                MemoryTier::Physical => &self.physical,
                MemoryTier::Omega => &self.omega,
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

    pub async fn stats(&self) -> CosmicMemoryStats {
        CosmicMemoryStats {
            civilizational: self.civilizational.len(),
            temporal: self.temporal.len(),
            physical: self.physical.len(),
            omega: self.omega.len(),
            total: self.civilizational.len()
                + self.temporal.len()
                + self.physical.len()
                + self.omega.len(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmicMemoryStats {
    pub civilizational: usize,
    pub temporal: usize,
    pub physical: usize,
    pub omega: usize,
    pub total: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MemoryContent;

    #[tokio::test]
    async fn test_cosmic_memory_storage() {
        let mut mem = CosmicScaleMemory::new().await.unwrap();
        let memory = Memory::new(
            MemoryTier::Omega,
            MemoryContent::Text("universal truth".to_string()),
            vec![1.0, 1.0, 1.0],
            0.99,
        );

        mem.store(memory).await.unwrap();
        let stats = mem.stats().await;
        assert_eq!(stats.omega, 1);
    }
}
