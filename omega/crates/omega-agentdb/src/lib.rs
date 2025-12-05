//! AgentDB wrapper for ExoGenesis Omega
//! Provides ReasoningBank, Reflexion, Causal, and Skill storage
//!
//! This is an in-memory implementation that mimics AgentDB's functionality
//! for the ExoGenesis Omega cognitive architecture.

mod hnsw;

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use hnsw::{HnswIndex, HnswConfig, VectorPoint};

pub type VectorId = String;
pub type ReflexionId = String;
pub type SkillId = String;
pub type Embedding = Vec<f32>;

/// Represents a single reflexion episode capturing agent learning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflexionEpisode {
    pub id: Option<ReflexionId>,
    pub session_id: String,
    pub task: String,
    pub input: serde_json::Value,
    pub output: serde_json::Value,
    pub reward: f64,
    pub success: bool,
    pub critique: String,
    pub latency_ms: u64,
    pub tokens: u64,
    pub timestamp: DateTime<Utc>,
    pub embedding: Option<Embedding>,
}

/// Represents a causal relationship between actions and outcomes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalEdge {
    pub cause: String,
    pub effect: String,
    pub uplift: f64,
    pub confidence: f64,
    pub sample_size: u64,
    pub first_observed: DateTime<Utc>,
    pub last_observed: DateTime<Utc>,
}

/// Represents a learned skill with semantic embedding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: Option<SkillId>,
    pub name: String,
    pub description: String,
    pub embedding: Embedding,
    pub usage_count: u64,
    pub success_rate: f64,
    pub created_at: DateTime<Utc>,
}

/// Result of a vector similarity search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorResult {
    pub id: VectorId,
    pub similarity: f64,
    pub metadata: serde_json::Value,
}

/// Configuration for AgentDB instance
#[derive(Debug, Clone)]
pub struct AgentDBConfig {
    pub dimension: usize,
    pub hnsw_m: usize,
    pub hnsw_ef: usize,
    pub cache_size: usize,
}

impl Default for AgentDBConfig {
    fn default() -> Self {
        Self {
            dimension: 4096,
            hnsw_m: 32,
            hnsw_ef: 100,
            cache_size: 100_000,
        }
    }
}

/// Main AgentDB interface providing vector storage, reflexion, causal, and skill management
pub struct AgentDB {
    config: AgentDBConfig,
    vector_index: Arc<RwLock<HnswIndex>>,
    episodes: Arc<RwLock<Vec<ReflexionEpisode>>>,
    causal_edges: Arc<RwLock<Vec<CausalEdge>>>,
    skills: Arc<RwLock<Vec<Skill>>>,
}

impl AgentDB {
    /// Creates a new AgentDB instance with the given configuration
    pub async fn new(config: AgentDBConfig) -> Result<Self, AgentDBError> {
        let hnsw_config = HnswConfig {
            ef_construction: config.hnsw_ef,
            ef_search: config.hnsw_ef,
            m: config.hnsw_m,
        };

        Ok(Self {
            config,
            vector_index: Arc::new(RwLock::new(HnswIndex::new(hnsw_config))),
            episodes: Arc::new(RwLock::new(Vec::new())),
            causal_edges: Arc::new(RwLock::new(Vec::new())),
            skills: Arc::new(RwLock::new(Vec::new())),
        })
    }

    // ==================== Vector Operations ====================

    /// Stores a vector embedding with associated metadata
    pub async fn vector_store(
        &self,
        embedding: Embedding,
        metadata: serde_json::Value,
    ) -> Result<VectorId, AgentDBError> {
        if embedding.len() != self.config.dimension {
            return Err(AgentDBError::StorageError(format!(
                "Embedding dimension {} does not match configured dimension {}",
                embedding.len(),
                self.config.dimension
            )));
        }

        let id = uuid::Uuid::new_v4().to_string();

        let point = VectorPoint {
            id: id.clone(),
            embedding,
            metadata,
        };

        self.vector_index.write().await.insert(point);
        Ok(id)
    }

    /// Searches for the k most similar vectors using HNSW index
    pub async fn vector_search(
        &self,
        query: &Embedding,
        k: usize,
    ) -> Result<Vec<VectorResult>, AgentDBError> {
        if query.len() != self.config.dimension {
            return Err(AgentDBError::QueryError(format!(
                "Query dimension {} does not match configured dimension {}",
                query.len(),
                self.config.dimension
            )));
        }

        let results = self.vector_index.write().await.search(query, k);

        Ok(results.into_iter().map(|r| VectorResult {
            id: r.id,
            similarity: r.similarity as f64,
            metadata: r.metadata,
        }).collect())
    }

    /// Retrieves a specific vector by ID
    pub async fn vector_get(&self, id: &str) -> Result<(Embedding, serde_json::Value), AgentDBError> {
        let index = self.vector_index.read().await;
        let point = index
            .get(id)
            .ok_or_else(|| AgentDBError::NotFound(format!("Vector {} not found", id)))?;

        Ok((point.embedding.clone(), point.metadata.clone()))
    }

    /// Deletes a vector by ID
    pub async fn vector_delete(&self, id: &str) -> Result<(), AgentDBError> {
        let mut index = self.vector_index.write().await;
        if index.remove(id) {
            Ok(())
        } else {
            Err(AgentDBError::NotFound(format!("Vector {} not found", id)))
        }
    }

    // ==================== Reflexion Operations ====================

    /// Stores a reflexion episode for learning from experience
    pub async fn reflexion_store(
        &self,
        mut episode: ReflexionEpisode,
    ) -> Result<ReflexionId, AgentDBError> {
        let id = uuid::Uuid::new_v4().to_string();
        episode.id = Some(id.clone());

        let mut episodes = self.episodes.write().await;
        episodes.push(episode);

        Ok(id)
    }

    /// Retrieves similar reflexion episodes for a given task
    pub async fn reflexion_retrieve(
        &self,
        task: &str,
        limit: usize,
    ) -> Result<Vec<ReflexionEpisode>, AgentDBError> {
        let episodes = self.episodes.read().await;

        // Simple substring matching for task similarity
        let mut matching: Vec<ReflexionEpisode> = episodes
            .iter()
            .filter(|ep| {
                ep.task.to_lowercase().contains(&task.to_lowercase())
                    || task.to_lowercase().contains(&ep.task.to_lowercase())
            })
            .cloned()
            .collect();

        // Sort by timestamp descending (most recent first)
        matching.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        matching.truncate(limit);

        Ok(matching)
    }

    /// Retrieves reflexion episodes by session ID
    pub async fn reflexion_by_session(
        &self,
        session_id: &str,
    ) -> Result<Vec<ReflexionEpisode>, AgentDBError> {
        let episodes = self.episodes.read().await;
        let matching: Vec<ReflexionEpisode> = episodes
            .iter()
            .filter(|ep| ep.session_id == session_id)
            .cloned()
            .collect();

        Ok(matching)
    }

    /// Analyzes reflexion episodes to calculate success metrics
    pub async fn reflexion_analyze(&self, task_prefix: &str) -> Result<ReflexionStats, AgentDBError> {
        let episodes = self.episodes.read().await;
        let matching: Vec<&ReflexionEpisode> = episodes
            .iter()
            .filter(|ep| ep.task.starts_with(task_prefix))
            .collect();

        if matching.is_empty() {
            return Ok(ReflexionStats::default());
        }

        let total = matching.len();
        let successful = matching.iter().filter(|ep| ep.success).count();
        let avg_reward = matching.iter().map(|ep| ep.reward).sum::<f64>() / total as f64;
        let avg_latency = matching.iter().map(|ep| ep.latency_ms).sum::<u64>() / total as u64;
        let avg_tokens = matching.iter().map(|ep| ep.tokens).sum::<u64>() / total as u64;

        Ok(ReflexionStats {
            total_episodes: total,
            successful_episodes: successful,
            success_rate: successful as f64 / total as f64,
            avg_reward,
            avg_latency_ms: avg_latency,
            avg_tokens,
        })
    }

    // ==================== Causal Operations ====================

    /// Adds or updates a causal edge
    pub async fn causal_add_edge(&self, edge: CausalEdge) -> Result<(), AgentDBError> {
        let mut edges = self.causal_edges.write().await;

        // Check if edge already exists
        if let Some(existing) = edges.iter_mut().find(|e| e.cause == edge.cause && e.effect == edge.effect) {
            // Update existing edge with new observations
            existing.uplift = (existing.uplift * existing.sample_size as f64
                + edge.uplift * edge.sample_size as f64)
                / (existing.sample_size + edge.sample_size) as f64;
            existing.confidence = edge.confidence.max(existing.confidence);
            existing.sample_size += edge.sample_size;
            existing.last_observed = edge.last_observed;
        } else {
            // Add new edge
            edges.push(edge);
        }

        Ok(())
    }

    /// Queries effects caused by a specific cause
    pub async fn causal_query_effects(&self, cause: &str) -> Result<Vec<CausalEdge>, AgentDBError> {
        let edges = self.causal_edges.read().await;
        let mut matching: Vec<CausalEdge> = edges
            .iter()
            .filter(|e| e.cause == cause)
            .cloned()
            .collect();

        // Sort by uplift descending
        matching.sort_by(|a, b| b.uplift.partial_cmp(&a.uplift).unwrap());

        Ok(matching)
    }

    /// Queries causes that lead to a specific effect
    pub async fn causal_query_causes(&self, effect: &str) -> Result<Vec<CausalEdge>, AgentDBError> {
        let edges = self.causal_edges.read().await;
        let mut matching: Vec<CausalEdge> = edges
            .iter()
            .filter(|e| e.effect == effect)
            .cloned()
            .collect();

        // Sort by uplift descending
        matching.sort_by(|a, b| b.uplift.partial_cmp(&a.uplift).unwrap());

        Ok(matching)
    }

    /// Finds causal paths between a cause and effect
    pub async fn causal_find_path(
        &self,
        start: &str,
        end: &str,
        max_depth: usize,
    ) -> Result<Vec<Vec<String>>, AgentDBError> {
        let edges = self.causal_edges.read().await;
        let mut paths: Vec<Vec<String>> = Vec::new();
        let mut current_path: Vec<String> = vec![start.to_string()];

        self.dfs_causal_path(&edges, start, end, &mut current_path, &mut paths, max_depth);

        Ok(paths)
    }

    // Helper for DFS path finding
    fn dfs_causal_path(
        &self,
        edges: &[CausalEdge],
        current: &str,
        target: &str,
        path: &mut Vec<String>,
        paths: &mut Vec<Vec<String>>,
        max_depth: usize,
    ) {
        if path.len() > max_depth {
            return;
        }

        if current == target {
            paths.push(path.clone());
            return;
        }

        for edge in edges.iter().filter(|e| e.cause == current) {
            if !path.contains(&edge.effect) {
                path.push(edge.effect.clone());
                self.dfs_causal_path(edges, &edge.effect, target, path, paths, max_depth);
                path.pop();
            }
        }
    }

    // ==================== Skill Operations ====================

    /// Creates a new skill with embedding
    pub async fn skill_create(&self, mut skill: Skill) -> Result<SkillId, AgentDBError> {
        if skill.embedding.len() != self.config.dimension {
            return Err(AgentDBError::StorageError(format!(
                "Skill embedding dimension {} does not match configured dimension {}",
                skill.embedding.len(),
                self.config.dimension
            )));
        }

        let id = uuid::Uuid::new_v4().to_string();
        skill.id = Some(id.clone());

        let mut skills = self.skills.write().await;
        skills.push(skill);

        Ok(id)
    }

    /// Searches for skills using semantic similarity
    pub async fn skill_search(&self, query: &str, limit: usize) -> Result<Vec<Skill>, AgentDBError> {
        let skills = self.skills.read().await;

        // Simple text matching (in production, would use query embedding)
        let query_lower = query.to_lowercase();
        let mut matching: Vec<Skill> = skills
            .iter()
            .filter(|s| {
                s.name.to_lowercase().contains(&query_lower)
                    || s.description.to_lowercase().contains(&query_lower)
            })
            .cloned()
            .collect();

        // Sort by usage count and success rate
        matching.sort_by(|a, b| {
            let score_a = a.usage_count as f64 * a.success_rate;
            let score_b = b.usage_count as f64 * b.success_rate;
            score_b.partial_cmp(&score_a).unwrap()
        });

        matching.truncate(limit);
        Ok(matching)
    }

    /// Searches for skills using embedding similarity
    pub async fn skill_search_by_embedding(
        &self,
        query_embedding: &Embedding,
        limit: usize,
    ) -> Result<Vec<(Skill, f64)>, AgentDBError> {
        if query_embedding.len() != self.config.dimension {
            return Err(AgentDBError::QueryError(format!(
                "Query embedding dimension {} does not match configured dimension {}",
                query_embedding.len(),
                self.config.dimension
            )));
        }

        let skills = self.skills.read().await;
        let mut results: Vec<(Skill, f64)> = skills
            .iter()
            .map(|skill| {
                let similarity = cosine_similarity(query_embedding, &skill.embedding);
                (skill.clone(), similarity)
            })
            .collect();

        // Sort by similarity descending
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        results.truncate(limit);

        Ok(results)
    }

    /// Updates skill usage statistics
    pub async fn skill_update_stats(
        &self,
        skill_id: &str,
        success: bool,
    ) -> Result<(), AgentDBError> {
        let mut skills = self.skills.write().await;
        let skill = skills
            .iter_mut()
            .find(|s| s.id.as_ref() == Some(&skill_id.to_string()))
            .ok_or_else(|| AgentDBError::NotFound(format!("Skill {} not found", skill_id)))?;

        skill.usage_count += 1;
        let new_successes = (skill.success_rate * (skill.usage_count - 1) as f64)
            + if success { 1.0 } else { 0.0 };
        skill.success_rate = new_successes / skill.usage_count as f64;

        Ok(())
    }

    /// Gets a skill by ID
    pub async fn skill_get(&self, skill_id: &str) -> Result<Skill, AgentDBError> {
        let skills = self.skills.read().await;
        skills
            .iter()
            .find(|s| s.id.as_ref() == Some(&skill_id.to_string()))
            .cloned()
            .ok_or_else(|| AgentDBError::NotFound(format!("Skill {} not found", skill_id)))
    }

    // ==================== Utility Methods ====================

    /// Returns statistics about the database
    pub async fn stats(&self) -> AgentDBStats {
        let vector_index = self.vector_index.read().await;
        let episodes = self.episodes.read().await;
        let edges = self.causal_edges.read().await;
        let skills = self.skills.read().await;

        AgentDBStats {
            vector_count: vector_index.len(),
            episode_count: episodes.len(),
            causal_edge_count: edges.len(),
            skill_count: skills.len(),
        }
    }

    /// Clears all data from the database
    pub async fn clear(&self) -> Result<(), AgentDBError> {
        let hnsw_config = HnswConfig {
            ef_construction: self.config.hnsw_ef,
            ef_search: self.config.hnsw_ef,
            m: self.config.hnsw_m,
        };

        let mut vector_index = self.vector_index.write().await;
        let mut episodes = self.episodes.write().await;
        let mut edges = self.causal_edges.write().await;
        let mut skills = self.skills.write().await;

        *vector_index = HnswIndex::new(hnsw_config);
        episodes.clear();
        edges.clear();
        skills.clear();

        Ok(())
    }
}

/// Statistics from reflexion analysis
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReflexionStats {
    pub total_episodes: usize,
    pub successful_episodes: usize,
    pub success_rate: f64,
    pub avg_reward: f64,
    pub avg_latency_ms: u64,
    pub avg_tokens: u64,
}

/// Overall database statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDBStats {
    pub vector_count: usize,
    pub episode_count: usize,
    pub causal_edge_count: usize,
    pub skill_count: usize,
}

/// Computes cosine similarity between two vectors
fn cosine_similarity(a: &[f32], b: &[f32]) -> f64 {
    if a.len() != b.len() {
        return 0.0;
    }

    let dot_product: f64 = a.iter().zip(b.iter()).map(|(x, y)| (*x as f64) * (*y as f64)).sum();
    let magnitude_a: f64 = a.iter().map(|x| (*x as f64) * (*x as f64)).sum::<f64>().sqrt();
    let magnitude_b: f64 = b.iter().map(|x| (*x as f64) * (*x as f64)).sum::<f64>().sqrt();

    if magnitude_a == 0.0 || magnitude_b == 0.0 {
        return 0.0;
    }

    dot_product / (magnitude_a * magnitude_b)
}

/// Error types for AgentDB operations
#[derive(Debug, thiserror::Error)]
pub enum AgentDBError {
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("Query error: {0}")]
    QueryError(String),
    #[error("Not found: {0}")]
    NotFound(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_vector_operations() {
        let db = AgentDB::new(AgentDBConfig {
            dimension: 128,
            ..Default::default()
        })
        .await
        .unwrap();

        // Create test embedding
        let embedding: Embedding = (0..128).map(|i| (i as f32) / 128.0).collect();
        let metadata = serde_json::json!({"test": "data"});

        // Store vector
        let id = db.vector_store(embedding.clone(), metadata.clone()).await.unwrap();

        // Retrieve vector
        let (retrieved_emb, retrieved_meta) = db.vector_get(&id).await.unwrap();
        assert_eq!(retrieved_emb.len(), 128);
        assert_eq!(retrieved_meta, metadata);

        // Search for similar vectors
        let results = db.vector_search(&embedding, 1).await.unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].similarity > 0.99);
    }

    #[tokio::test]
    async fn test_reflexion_operations() {
        let db = AgentDB::new(AgentDBConfig::default()).await.unwrap();

        let episode = ReflexionEpisode {
            id: None,
            session_id: "session-1".to_string(),
            task: "solve math problem".to_string(),
            input: serde_json::json!({"problem": "2+2"}),
            output: serde_json::json!({"answer": 4}),
            reward: 1.0,
            success: true,
            critique: "Correct answer".to_string(),
            latency_ms: 100,
            tokens: 50,
            timestamp: Utc::now(),
            embedding: None,
        };

        let id = db.reflexion_store(episode).await.unwrap();
        assert!(!id.is_empty());

        let retrieved = db.reflexion_retrieve("math", 10).await.unwrap();
        assert_eq!(retrieved.len(), 1);
        assert_eq!(retrieved[0].task, "solve math problem");
    }

    #[tokio::test]
    async fn test_causal_operations() {
        let db = AgentDB::new(AgentDBConfig::default()).await.unwrap();

        let edge = CausalEdge {
            cause: "use_cache".to_string(),
            effect: "faster_response".to_string(),
            uplift: 0.5,
            confidence: 0.95,
            sample_size: 100,
            first_observed: Utc::now(),
            last_observed: Utc::now(),
        };

        db.causal_add_edge(edge).await.unwrap();

        let effects = db.causal_query_effects("use_cache").await.unwrap();
        assert_eq!(effects.len(), 1);
        assert_eq!(effects[0].effect, "faster_response");
    }

    #[tokio::test]
    async fn test_skill_operations() {
        let db = AgentDB::new(AgentDBConfig {
            dimension: 64,
            ..Default::default()
        })
        .await
        .unwrap();

        let embedding: Embedding = (0..64).map(|i| (i as f32) / 64.0).collect();
        let skill = Skill {
            id: None,
            name: "code_generation".to_string(),
            description: "Generate Python code from natural language".to_string(),
            embedding,
            usage_count: 0,
            success_rate: 0.0,
            created_at: Utc::now(),
        };

        let id = db.skill_create(skill).await.unwrap();
        assert!(!id.is_empty());

        let results = db.skill_search("code", 10).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "code_generation");
    }

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 0.001);

        let c = vec![1.0, 0.0, 0.0];
        let d = vec![0.0, 1.0, 0.0];
        assert!(cosine_similarity(&c, &d).abs() < 0.001);

        let e = vec![1.0, 1.0, 0.0];
        let f = vec![1.0, 1.0, 0.0];
        assert!((cosine_similarity(&e, &f) - 1.0).abs() < 0.001);
    }

    #[tokio::test]
    async fn test_hnsw_vector_operations() {
        let db = AgentDB::new(AgentDBConfig {
            dimension: 128,
            ..Default::default()
        })
        .await
        .unwrap();

        // Store test vectors
        let emb1: Embedding = (0..128).map(|i| i as f32 / 128.0).collect();
        let emb2: Embedding = (0..128).map(|i| (128 - i) as f32 / 128.0).collect();

        let id1 = db.vector_store(emb1.clone(), serde_json::json!({"name": "v1"})).await.unwrap();
        let id2 = db.vector_store(emb2.clone(), serde_json::json!({"name": "v2"})).await.unwrap();

        // Retrieve specific vector
        let (retrieved, meta) = db.vector_get(&id2).await.unwrap();
        assert_eq!(retrieved.len(), 128);
        assert_eq!(meta["name"], "v2");

        // Delete a vector
        db.vector_delete(&id2).await.unwrap();
        assert!(db.vector_get(&id2).await.is_err());

        // Stats should reflect deletion
        let stats = db.stats().await;
        assert_eq!(stats.vector_count, 1);
    }

    #[tokio::test]
    async fn test_hnsw_large_dataset() {
        let db = AgentDB::new(AgentDBConfig {
            dimension: 64,
            hnsw_m: 16,
            hnsw_ef: 100,
            ..Default::default()
        })
        .await
        .unwrap();

        // Insert 100 vectors
        for i in 0..100 {
            let embedding: Embedding = (0..64).map(|j| ((i * j) as f32) / 1000.0).collect();
            db.vector_store(embedding, serde_json::json!({"index": i})).await.unwrap();
        }

        // Search for similar to vector 50
        let query: Embedding = (0..64).map(|j| ((50 * j) as f32) / 1000.0).collect();
        let results = db.vector_search(&query, 10).await.unwrap();

        // HNSW is approximate - just verify we get meaningful results
        assert!(!results.is_empty());
        assert!(results.len() <= 10);

        // Results should have reasonable similarity
        assert!(results[0].similarity > 0.5, "Top result should have >50% similarity");

        // Verify stats
        let stats = db.stats().await;
        assert_eq!(stats.vector_count, 100);
    }

    #[tokio::test]
    async fn test_hnsw_empty_search() {
        let db = AgentDB::new(AgentDBConfig {
            dimension: 32,
            ..Default::default()
        })
        .await
        .unwrap();

        let query: Embedding = vec![0.1; 32];
        let results = db.vector_search(&query, 10).await.unwrap();
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn test_hnsw_stats() {
        let db = AgentDB::new(AgentDBConfig {
            dimension: 16,
            ..Default::default()
        })
        .await
        .unwrap();

        let stats = db.stats().await;
        assert_eq!(stats.vector_count, 0);

        for i in 0..5 {
            let emb: Embedding = vec![i as f32; 16];
            db.vector_store(emb, serde_json::json!({})).await.unwrap();
        }

        let stats = db.stats().await;
        assert_eq!(stats.vector_count, 5);

        db.clear().await.unwrap();
        let stats = db.stats().await;
        assert_eq!(stats.vector_count, 0);
    }

    #[tokio::test]
    async fn test_hnsw_dimension_validation() {
        let db = AgentDB::new(AgentDBConfig {
            dimension: 64,
            ..Default::default()
        })
        .await
        .unwrap();

        // Try to store wrong dimension
        let wrong_emb: Embedding = vec![1.0; 32];
        let result = db.vector_store(wrong_emb, serde_json::json!({})).await;
        assert!(result.is_err());

        // Try to search with wrong dimension
        let wrong_query: Embedding = vec![1.0; 32];
        let result = db.vector_search(&wrong_query, 5).await;
        assert!(result.is_err());
    }
}
