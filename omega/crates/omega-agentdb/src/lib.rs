//! AgentDB wrapper for ExoGenesis Omega
//! Provides ReasoningBank, Reflexion, Causal, and Skill storage
//!
//! This is an in-memory implementation that mimics AgentDB's functionality
//! for the ExoGenesis Omega cognitive architecture.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

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
    vectors: Arc<RwLock<HashMap<VectorId, (Embedding, serde_json::Value)>>>,
    episodes: Arc<RwLock<Vec<ReflexionEpisode>>>,
    causal_edges: Arc<RwLock<Vec<CausalEdge>>>,
    skills: Arc<RwLock<Vec<Skill>>>,
}

impl AgentDB {
    /// Creates a new AgentDB instance with the given configuration
    pub async fn new(config: AgentDBConfig) -> Result<Self, AgentDBError> {
        Ok(Self {
            config,
            vectors: Arc::new(RwLock::new(HashMap::new())),
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
        let mut vectors = self.vectors.write().await;
        vectors.insert(id.clone(), (embedding, metadata));
        Ok(id)
    }

    /// Searches for the k most similar vectors using cosine similarity
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

        let vectors = self.vectors.read().await;
        let mut results: Vec<VectorResult> = Vec::new();

        for (id, (embedding, metadata)) in vectors.iter() {
            let similarity = cosine_similarity(query, embedding);
            results.push(VectorResult {
                id: id.clone(),
                similarity,
                metadata: metadata.clone(),
            });
        }

        // Sort by similarity descending and take top k
        results.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());
        results.truncate(k);

        Ok(results)
    }

    /// Retrieves a specific vector by ID
    pub async fn vector_get(&self, id: &str) -> Result<(Embedding, serde_json::Value), AgentDBError> {
        let vectors = self.vectors.read().await;
        vectors
            .get(id)
            .cloned()
            .ok_or_else(|| AgentDBError::NotFound(format!("Vector {} not found", id)))
    }

    /// Deletes a vector by ID
    pub async fn vector_delete(&self, id: &str) -> Result<(), AgentDBError> {
        let mut vectors = self.vectors.write().await;
        vectors
            .remove(id)
            .ok_or_else(|| AgentDBError::NotFound(format!("Vector {} not found", id)))?;
        Ok(())
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
        let vectors = self.vectors.read().await;
        let episodes = self.episodes.read().await;
        let edges = self.causal_edges.read().await;
        let skills = self.skills.read().await;

        AgentDBStats {
            vector_count: vectors.len(),
            episode_count: episodes.len(),
            causal_edge_count: edges.len(),
            skill_count: skills.len(),
        }
    }

    /// Clears all data from the database
    pub async fn clear(&self) -> Result<(), AgentDBError> {
        let mut vectors = self.vectors.write().await;
        let mut episodes = self.episodes.write().await;
        let mut edges = self.causal_edges.write().await;
        let mut skills = self.skills.write().await;

        vectors.clear();
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
}
