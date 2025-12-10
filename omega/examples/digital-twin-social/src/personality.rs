//! Personality Engine - Vector-based personality modeling using AgentDB
//!
//! This module demonstrates how to use omega-agentdb for storing and searching
//! personality vectors with SIMD-accelerated similarity computation.

use crate::types::*;
use chrono::Utc;
use omega_agentdb::{AgentDB, AgentDBConfig, Embedding, VectorResult};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Errors that can occur in the personality engine
#[derive(Error, Debug)]
pub enum PersonalityError {
    #[error("AgentDB error: {0}")]
    AgentDB(#[from] omega_agentdb::AgentDBError),
    #[error("Profile not found: {0}")]
    NotFound(String),
    #[error("Invalid embedding dimension: expected {expected}, got {got}")]
    InvalidDimension { expected: usize, got: usize },
}

/// Configuration for the personality engine
#[derive(Debug, Clone)]
pub struct PersonalityEngineConfig {
    /// Embedding dimension (default: 4096)
    pub embedding_dimension: usize,
    /// HNSW M parameter for graph connectivity
    pub hnsw_m: usize,
    /// HNSW ef parameter for search quality
    pub hnsw_ef: usize,
    /// Maximum profiles to cache in memory
    pub cache_size: usize,
}

impl Default for PersonalityEngineConfig {
    fn default() -> Self {
        Self {
            embedding_dimension: 4096,
            hnsw_m: 32,
            hnsw_ef: 100,
            cache_size: 100_000,
        }
    }
}

/// The Personality Engine manages digital twin profiles using AgentDB
///
/// This engine provides:
/// - SIMD-accelerated personality vector storage and search
/// - Automatic embedding generation from personality components
/// - Reflexion-based learning from user interactions
/// - Skill tracking for emotional intelligence development
pub struct PersonalityEngine {
    /// The underlying AgentDB instance
    db: Arc<AgentDB>,
    /// Local profile cache for fast lookups
    profiles: Arc<RwLock<HashMap<UserId, DigitalTwin>>>,
    /// Configuration
    config: PersonalityEngineConfig,
}

impl PersonalityEngine {
    /// Create a new personality engine with default configuration
    pub async fn new() -> Result<Self, PersonalityError> {
        Self::with_config(PersonalityEngineConfig::default()).await
    }

    /// Create a new personality engine with custom configuration
    pub async fn with_config(config: PersonalityEngineConfig) -> Result<Self, PersonalityError> {
        let db_config = AgentDBConfig {
            dimension: config.embedding_dimension,
            hnsw_m: config.hnsw_m,
            hnsw_ef: config.hnsw_ef,
            cache_size: config.cache_size,
        };

        let db = AgentDB::new(db_config).await?;

        Ok(Self {
            db: Arc::new(db),
            profiles: Arc::new(RwLock::new(HashMap::new())),
            config,
        })
    }

    /// Register a new digital twin profile
    ///
    /// This stores the profile in both the local cache and AgentDB's
    /// vector index for similarity search.
    pub async fn register_profile(&self, mut twin: DigitalTwin) -> Result<UserId, PersonalityError> {
        // Generate embedding from personality components
        twin.generate_embedding();
        twin.updated_at = Utc::now();

        let id = twin.id;

        // Store in AgentDB vector index
        let metadata = json!({
            "user_id": id.to_string(),
            "name": twin.name,
            "confidence": twin.confidence,
            "observation_count": twin.observation_count,
            "created_at": twin.created_at.to_rfc3339(),
            "updated_at": twin.updated_at.to_rfc3339(),
        });

        self.db.vector_store(twin.deep_embedding.clone(), metadata).await?;

        // Cache locally
        self.profiles.write().await.insert(id, twin);

        Ok(id)
    }

    /// Update an existing profile with new personality data
    pub async fn update_profile(&self, twin: &mut DigitalTwin) -> Result<(), PersonalityError> {
        // Regenerate embedding
        twin.generate_embedding();
        twin.updated_at = Utc::now();
        twin.observation_count += 1;

        // Increase confidence with more observations (asymptotic to 1.0)
        twin.confidence = 1.0 - (1.0 / (twin.observation_count as f64 + 1.0).sqrt());

        let id = twin.id;

        // Update in AgentDB
        let metadata = json!({
            "user_id": id.to_string(),
            "name": twin.name,
            "confidence": twin.confidence,
            "observation_count": twin.observation_count,
            "created_at": twin.created_at.to_rfc3339(),
            "updated_at": twin.updated_at.to_rfc3339(),
        });

        self.db.vector_store(twin.deep_embedding.clone(), metadata).await?;

        // Update cache
        self.profiles.write().await.insert(id, twin.clone());

        Ok(())
    }

    /// Get a profile by user ID
    pub async fn get_profile(&self, user_id: &UserId) -> Result<DigitalTwin, PersonalityError> {
        // Check cache first
        if let Some(profile) = self.profiles.read().await.get(user_id) {
            return Ok(profile.clone());
        }

        Err(PersonalityError::NotFound(user_id.to_string()))
    }

    /// Find similar profiles using SIMD-accelerated vector search
    ///
    /// This is the core matching function that leverages AgentDB's
    /// HNSW index with SimSIMD acceleration for sub-millisecond search.
    pub async fn find_similar(
        &self,
        user_id: &UserId,
        limit: usize,
    ) -> Result<Vec<SimilarProfile>, PersonalityError> {
        let profile = self.get_profile(user_id).await?;

        // Search using SIMD-accelerated cosine similarity
        let results = self.db.vector_search(&profile.deep_embedding, limit + 1).await?;

        // Filter out self and convert results
        let similar: Vec<SimilarProfile> = results
            .into_iter()
            .filter(|r| {
                r.metadata
                    .get("user_id")
                    .and_then(|v| v.as_str())
                    .map(|id| id != user_id.to_string())
                    .unwrap_or(true)
            })
            .take(limit)
            .map(|r| SimilarProfile {
                user_id: r.metadata
                    .get("user_id")
                    .and_then(|v| v.as_str())
                    .and_then(|s| Uuid::parse_str(s).ok())
                    .unwrap_or_else(Uuid::new_v4),
                name: r.metadata
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown")
                    .to_string(),
                similarity: r.similarity,
                confidence: r.metadata
                    .get("confidence")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0),
            })
            .collect();

        Ok(similar)
    }

    /// Find profiles similar to a given embedding vector
    ///
    /// Useful for finding matches based on specific personality aspects.
    pub async fn find_by_embedding(
        &self,
        embedding: &Embedding,
        limit: usize,
    ) -> Result<Vec<VectorResult>, PersonalityError> {
        if embedding.len() != self.config.embedding_dimension {
            return Err(PersonalityError::InvalidDimension {
                expected: self.config.embedding_dimension,
                got: embedding.len(),
            });
        }

        let results = self.db.vector_search(embedding, limit).await?;
        Ok(results)
    }

    /// Update personality traits from an emotional episode
    ///
    /// This demonstrates how to evolve the digital twin based on
    /// observed behavior and emotional patterns.
    pub async fn learn_from_episode(
        &self,
        user_id: &UserId,
        episode: &EmotionalEpisode,
    ) -> Result<(), PersonalityError> {
        let mut profile = self.get_profile(user_id).await?;

        // Update EQ based on episode
        if episode.emotional_regulation_score > 0.0 {
            // Weighted update to self-regulation
            let learning_rate = 0.1;
            profile.eq.self_regulation = profile.eq.self_regulation * (1.0 - learning_rate)
                + episode.emotional_regulation_score as f32 * learning_rate;
        }

        // Update observation count
        profile.observation_count += 1;

        // Store episode in reflexion system
        let reflexion_episode = omega_agentdb::ReflexionEpisode {
            id: None,
            session_id: user_id.to_string(),
            task: format!("emotional_episode_{:?}", episode.context),
            input: serde_json::to_value(&episode.trigger).unwrap_or_default(),
            output: serde_json::to_value(&episode.behavioral_response).unwrap_or_default(),
            reward: episode.emotional_regulation_score,
            success: episode.outcome == Outcome::Positive,
            critique: episode.self_reflection.clone().unwrap_or_default(),
            latency_ms: 0,
            tokens: 0,
            timestamp: episode.timestamp,
            embedding: episode.embedding.clone(),
        };

        self.db.reflexion_store(reflexion_episode).await?;

        // Update the profile
        self.update_profile(&mut profile).await?;

        Ok(())
    }

    /// Record a skill development milestone
    pub async fn record_skill(
        &self,
        user_id: &UserId,
        skill_name: &str,
        description: &str,
        success: bool,
    ) -> Result<(), PersonalityError> {
        let profile = self.get_profile(user_id).await?;

        // Create skill embedding from profile context
        let mut skill_embedding = profile.deep_embedding[..1024].to_vec();
        // Pad to 4096
        skill_embedding.resize(4096, 0.0);

        let skill = omega_agentdb::Skill {
            id: None,
            name: format!("{}_{}", user_id, skill_name),
            description: description.to_string(),
            embedding: skill_embedding,
            usage_count: 1,
            success_rate: if success { 1.0 } else { 0.0 },
            created_at: Utc::now(),
        };

        self.db.skill_create(skill).await?;

        Ok(())
    }

    /// Get statistics about profile learning
    pub async fn get_learning_stats(
        &self,
        user_id: &UserId,
    ) -> Result<LearningStats, PersonalityError> {
        let task_prefix = &user_id.to_string();
        let stats = self.db.reflexion_analyze(task_prefix).await?;

        Ok(LearningStats {
            total_episodes: stats.total_episodes,
            successful_episodes: stats.successful_episodes,
            success_rate: stats.success_rate,
            avg_emotional_regulation: stats.avg_reward,
        })
    }

    /// Get the number of profiles in the engine
    pub async fn profile_count(&self) -> usize {
        self.profiles.read().await.len()
    }
}

/// A profile that is similar to the query profile
#[derive(Debug, Clone)]
pub struct SimilarProfile {
    pub user_id: UserId,
    pub name: String,
    pub similarity: f64,
    pub confidence: f64,
}

/// Statistics about a user's learning/growth
#[derive(Debug, Clone)]
pub struct LearningStats {
    pub total_episodes: usize,
    pub successful_episodes: usize,
    pub success_rate: f64,
    pub avg_emotional_regulation: f64,
}

/// An emotional episode that can be used for learning
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmotionalEpisode {
    pub id: EpisodeId,
    pub context: String,
    pub trigger: Option<String>,
    pub initial_emotion: EmotionalState,
    pub regulated_emotion: EmotionalState,
    pub behavioral_response: String,
    pub outcome: Outcome,
    pub self_reflection: Option<String>,
    pub emotional_regulation_score: f64,
    pub embedding: Option<Vec<f32>>,
    pub timestamp: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Outcome {
    Positive,
    Neutral,
    Negative,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_personality_engine_creation() {
        let engine = PersonalityEngine::new().await.unwrap();
        assert_eq!(engine.profile_count().await, 0);
    }

    #[tokio::test]
    async fn test_register_and_find_profile() {
        let engine = PersonalityEngine::new().await.unwrap();

        // Create test profile
        let mut twin = DigitalTwin::new("Test User");
        twin.big_five = BigFive::new(0.8, 0.7, 0.6, 0.9, 0.3);

        let id = engine.register_profile(twin).await.unwrap();

        // Retrieve profile
        let retrieved = engine.get_profile(&id).await.unwrap();
        assert_eq!(retrieved.name, "Test User");
        assert!(retrieved.deep_embedding.len() == 4096);
    }

    #[tokio::test]
    async fn test_similarity_search() {
        let engine = PersonalityEngine::new().await.unwrap();

        // Create multiple profiles with varying personalities
        let mut twin1 = DigitalTwin::new("User 1");
        twin1.big_five = BigFive::new(0.9, 0.9, 0.9, 0.9, 0.1);

        let mut twin2 = DigitalTwin::new("User 2");
        twin2.big_five = BigFive::new(0.85, 0.85, 0.85, 0.85, 0.15);

        let mut twin3 = DigitalTwin::new("User 3");
        twin3.big_five = BigFive::new(0.1, 0.1, 0.1, 0.1, 0.9);

        let id1 = engine.register_profile(twin1).await.unwrap();
        let _id2 = engine.register_profile(twin2).await.unwrap();
        let _id3 = engine.register_profile(twin3).await.unwrap();

        // Find similar to User 1
        let similar = engine.find_similar(&id1, 2).await.unwrap();

        // User 2 should be more similar than User 3
        assert!(!similar.is_empty());
        if similar.len() >= 2 {
            assert!(similar[0].similarity > similar[1].similarity);
        }
    }
}
