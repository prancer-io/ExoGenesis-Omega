//! Individual scale memory (Tier 1-4)
//! Implements working memory storage with AgentDB integration

use crate::{Memory, MemoryContent, MemoryError, MemoryTier, Query};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Individual memory system managing Tier 1-4
pub struct IndividualMemory {
    /// Tier 1: Instant memory (in-memory only)
    instant: Arc<RwLock<HashMap<String, Memory>>>,

    /// Tier 2: Session memory (in-memory + AgentDB)
    session: Arc<RwLock<HashMap<String, Memory>>>,

    /// Tier 3: Episodic memory (AgentDB)
    episodic: Arc<RwLock<AgentDBWrapper>>,

    /// Tier 4: Semantic memory (AgentDB with indexing)
    semantic: Arc<RwLock<AgentDBWrapper>>,
}

impl IndividualMemory {
    pub async fn new() -> Result<Self, MemoryError> {
        let episodic_path = PathBuf::from("/tmp/omega/memory/episodic.agentdb");
        let semantic_path = PathBuf::from("/tmp/omega/memory/semantic.agentdb");

        // Ensure directories exist
        if let Some(parent) = episodic_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| MemoryError::Storage(format!("Failed to create directory: {}", e)))?;
        }

        Ok(Self {
            instant: Arc::new(RwLock::new(HashMap::new())),
            session: Arc::new(RwLock::new(HashMap::new())),
            episodic: Arc::new(RwLock::new(AgentDBWrapper::new(episodic_path).await?)),
            semantic: Arc::new(RwLock::new(AgentDBWrapper::new(semantic_path).await?)),
        })
    }

    pub async fn store(&self, memory: Memory) -> Result<String, MemoryError> {
        let id = memory.id.clone();

        match memory.tier {
            MemoryTier::Instant => {
                self.instant.write().await.insert(id.clone(), memory);
                self.prune_instant().await?;
            }
            MemoryTier::Session => {
                self.session.write().await.insert(id.clone(), memory);
                self.prune_session().await?;
            }
            MemoryTier::Episodic => {
                self.episodic.write().await.store(memory).await?;
            }
            MemoryTier::Semantic => {
                self.semantic.write().await.store(memory).await?;
            }
            _ => {
                return Err(MemoryError::Storage(format!(
                    "Invalid tier {:?} for individual memory",
                    memory.tier
                )));
            }
        }

        Ok(id)
    }

    pub async fn recall(
        &self,
        query: &Query,
        tiers: &[MemoryTier],
    ) -> Result<Vec<Memory>, MemoryError> {
        let mut results = Vec::new();

        for tier in tiers {
            match tier {
                MemoryTier::Instant => {
                    let instant_mem = self.instant.read().await;
                    let mut memories: Vec<Memory> = instant_mem.values().cloned().collect();
                    memories = self.filter_memories(memories, query);
                    results.extend(memories);
                }
                MemoryTier::Session => {
                    let session_mem = self.session.read().await;
                    let mut memories: Vec<Memory> = session_mem.values().cloned().collect();
                    memories = self.filter_memories(memories, query);
                    results.extend(memories);
                }
                MemoryTier::Episodic => {
                    let episodic_results = self.episodic.read().await.search(query).await?;
                    results.extend(episodic_results);
                }
                MemoryTier::Semantic => {
                    let semantic_results = self.semantic.read().await.search(query).await?;
                    results.extend(semantic_results);
                }
                _ => {}
            }
        }

        Ok(results)
    }

    pub async fn stats(&self) -> IndividualMemoryStats {
        let instant_count = self.instant.read().await.len();
        let session_count = self.session.read().await.len();
        let episodic_count = self.episodic.read().await.count().await;
        let semantic_count = self.semantic.read().await.count().await;

        IndividualMemoryStats {
            instant: instant_count,
            session: session_count,
            episodic: episodic_count,
            semantic: semantic_count,
            total: instant_count + session_count + episodic_count + semantic_count,
        }
    }

    async fn prune_instant(&self) -> Result<(), MemoryError> {
        let mut instant = self.instant.write().await;
        let max_size = MemoryTier::Instant.typical_size();

        if instant.len() > max_size {
            let mut entries: Vec<_> = instant.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            entries.sort_by(|a, b| {
                a.1.accessed_at
                    .cmp(&b.1.accessed_at)
            });

            let to_remove = entries.len() - max_size;
            for (key, _) in entries.iter().take(to_remove) {
                instant.remove(key);
            }
        }

        Ok(())
    }

    async fn prune_session(&self) -> Result<(), MemoryError> {
        let mut session = self.session.write().await;
        let max_size = MemoryTier::Session.typical_size();

        if session.len() > max_size {
            let mut entries: Vec<_> = session.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            entries.sort_by(|a, b| {
                b.1.relevance_score()
                    .partial_cmp(&a.1.relevance_score())
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            let to_remove = entries.len() - max_size;
            for (key, memory) in entries.iter().rev().take(to_remove) {
                // Promote important memories to episodic before removing
                if memory.importance > 0.3 {
                    let mut promoted = memory.clone();
                    promoted.tier = MemoryTier::Episodic;
                    self.episodic.write().await.store(promoted).await?;
                }
                session.remove(key);
            }
        }

        Ok(())
    }

    fn filter_memories(&self, memories: Vec<Memory>, query: &Query) -> Vec<Memory> {
        memories
            .into_iter()
            .filter(|m| {
                // Filter by importance threshold
                if let Some(min_importance) = query.min_importance {
                    if m.importance < min_importance {
                        return false;
                    }
                }

                // Text matching if text query provided
                if let Some(ref text) = query.text {
                    if let MemoryContent::Text(ref content) = m.content {
                        if !content.to_lowercase().contains(&text.to_lowercase()) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                true
            })
            .collect()
    }
}

/// AgentDB wrapper for persistent storage
pub struct AgentDBWrapper {
    path: PathBuf,
    memories: HashMap<String, Memory>,
}

impl AgentDBWrapper {
    async fn new(path: PathBuf) -> Result<Self, MemoryError> {
        let mut wrapper = Self {
            path,
            memories: HashMap::new(),
        };

        // Load existing memories if file exists
        if wrapper.path.exists() {
            wrapper.load().await?;
        }

        Ok(wrapper)
    }

    async fn store(&mut self, memory: Memory) -> Result<(), MemoryError> {
        self.memories.insert(memory.id.clone(), memory);
        self.save().await?;
        Ok(())
    }

    async fn search(&self, query: &Query) -> Result<Vec<Memory>, MemoryError> {
        let mut results: Vec<Memory> = self.memories.values().cloned().collect();

        // Filter by importance
        if let Some(min_importance) = query.min_importance {
            results.retain(|m| m.importance >= min_importance);
        }

        // Vector similarity search if embedding provided
        if let Some(ref query_embedding) = query.embedding {
            results.sort_by(|a, b| {
                let sim_a = cosine_similarity(&a.embedding, query_embedding);
                let sim_b = cosine_similarity(&b.embedding, query_embedding);
                sim_b.partial_cmp(&sim_a).unwrap_or(std::cmp::Ordering::Equal)
            });

            // Take top k results
            if let Some(limit) = query.limit {
                results.truncate(limit);
            }
        }

        Ok(results)
    }

    async fn count(&self) -> usize {
        self.memories.len()
    }

    async fn load(&mut self) -> Result<(), MemoryError> {
        let data = tokio::fs::read(&self.path).await?;
        self.memories = serde_json::from_slice(&data)?;
        Ok(())
    }

    async fn save(&self) -> Result<(), MemoryError> {
        let data = serde_json::to_vec_pretty(&self.memories)?;
        tokio::fs::write(&self.path, data).await?;
        Ok(())
    }
}

/// Cosine similarity between two vectors
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }

    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let mag_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let mag_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if mag_a == 0.0 || mag_b == 0.0 {
        return 0.0;
    }

    dot_product / (mag_a * mag_b)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndividualMemoryStats {
    pub instant: usize,
    pub session: usize,
    pub episodic: usize,
    pub semantic: usize,
    pub total: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MemoryContent;

    #[tokio::test]
    async fn test_instant_memory() {
        let mem = IndividualMemory::new().await.unwrap();
        let memory = Memory::new(
            MemoryTier::Instant,
            MemoryContent::Text("test".to_string()),
            vec![0.1, 0.2, 0.3],
            0.5,
        );

        let id = mem.store(memory).await.unwrap();
        assert!(!id.is_empty());
    }

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert_eq!(cosine_similarity(&a, &b), 1.0);

        let c = vec![1.0, 0.0, 0.0];
        let d = vec![0.0, 1.0, 0.0];
        assert_eq!(cosine_similarity(&c, &d), 0.0);
    }
}
