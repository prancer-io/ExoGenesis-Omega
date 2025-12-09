//! PostgreSQL Backend with RuVector Integration
//!
//! This module provides a PostgreSQL-backed storage layer that leverages
//! RuVector's SIMD-accelerated vector operations and HNSW indexing for
//! sub-millisecond personality matching.
//!
//! ## Features
//!
//! - **SIMD-Accelerated Vector Search**: Uses RuVector's optimized distance computations
//! - **HNSW Indexing**: Hierarchical Navigable Small World graphs for approximate nearest neighbors
//! - **Persistent Storage**: All digital twins, emotional signals, and conversations are persisted
//! - **Zero-Knowledge Compatible**: Privacy-preserving aggregation layer included
//!
//! ## Usage
//!
//! ```rust,ignore
//! use digital_twin_social::postgres::PostgresBackend;
//!
//! let backend = PostgresBackend::connect("postgres://user:pass@localhost/path_social").await?;
//! let twin_id = backend.store_twin(&digital_twin).await?;
//! let matches = backend.find_matches(&twin_id, "dating", 10).await?;
//! ```

use crate::types::*;
use chrono::{DateTime, Utc};
use pgvector::Vector;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;
use thiserror::Error;
use tracing::{debug, info, instrument};
use uuid::Uuid;

/// Errors from PostgreSQL operations
#[derive(Error, Debug)]
pub enum PostgresError {
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Query error: {0}")]
    Query(#[from] sqlx::Error),

    #[error("Vector dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Serialization error: {0}")]
    Serialization(String),
}

/// Configuration for PostgreSQL connection
#[derive(Debug, Clone)]
pub struct PostgresConfig {
    /// Database connection URL
    pub database_url: String,
    /// Maximum connections in pool
    pub max_connections: u32,
    /// Connection timeout in seconds
    pub connect_timeout_secs: u64,
    /// Vector embedding dimension
    pub embedding_dimension: usize,
}

impl Default for PostgresConfig {
    fn default() -> Self {
        Self {
            database_url: "postgres://path_user:path_secure_password@localhost:5432/path_social"
                .to_string(),
            max_connections: 10,
            connect_timeout_secs: 30,
            embedding_dimension: 4096,
        }
    }
}

impl PostgresConfig {
    /// Create config from environment variables
    pub fn from_env() -> Self {
        let _ = dotenvy::dotenv();
        Self {
            database_url: std::env::var("DATABASE_URL").unwrap_or_else(|_| {
                "postgres://path_user:path_secure_password@localhost:5432/path_social".to_string()
            }),
            max_connections: std::env::var("DATABASE_MAX_CONNECTIONS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
            connect_timeout_secs: std::env::var("DATABASE_TIMEOUT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
            embedding_dimension: std::env::var("EMBEDDING_DIMENSION")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(4096),
        }
    }
}

/// PostgreSQL backend for PATH Social digital twin storage
pub struct PostgresBackend {
    pool: PgPool,
    config: PostgresConfig,
}

impl PostgresBackend {
    /// Connect to PostgreSQL with the given configuration
    #[instrument(skip(config))]
    pub async fn connect(config: PostgresConfig) -> Result<Self, PostgresError> {
        info!("Connecting to PostgreSQL database...");

        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .acquire_timeout(std::time::Duration::from_secs(config.connect_timeout_secs))
            .connect(&config.database_url)
            .await
            .map_err(|e| PostgresError::Connection(e.to_string()))?;

        info!("Successfully connected to PostgreSQL");

        Ok(Self { pool, config })
    }

    /// Connect using default configuration
    pub async fn connect_default() -> Result<Self, PostgresError> {
        Self::connect(PostgresConfig::default()).await
    }

    /// Connect using environment variables
    pub async fn connect_from_env() -> Result<Self, PostgresError> {
        Self::connect(PostgresConfig::from_env()).await
    }

    // ========================================================================
    // DIGITAL TWIN OPERATIONS
    // ========================================================================

    /// Store a new digital twin profile with optional archetype
    #[instrument(skip(self, twin))]
    pub async fn store_twin(&self, twin: &DigitalTwin) -> Result<Uuid, PostgresError> {
        self.store_twin_with_archetype(twin, None).await
    }

    /// Store a new digital twin profile with archetype
    #[instrument(skip(self, twin))]
    pub async fn store_twin_with_archetype(
        &self,
        twin: &DigitalTwin,
        archetype: Option<&str>,
    ) -> Result<Uuid, PostgresError> {
        let db_id = Uuid::new_v4();
        let user_id = twin.id.to_string();

        // Convert embedding to pgvector format
        let embedding = if twin.deep_embedding.len() == self.config.embedding_dimension {
            Some(Vector::from(twin.deep_embedding.clone()))
        } else if twin.deep_embedding.is_empty() {
            None
        } else {
            return Err(PostgresError::DimensionMismatch {
                expected: self.config.embedding_dimension,
                actual: twin.deep_embedding.len(),
            });
        };

        sqlx::query(
            r#"
            INSERT INTO digital_twins (
                id, user_id, name,
                openness, conscientiousness, extraversion, agreeableness, neuroticism,
                attachment_style,
                eq_self_awareness, eq_self_regulation, eq_motivation, eq_empathy, eq_social_skills,
                value_self_direction, value_stimulation, value_hedonism, value_achievement, value_power,
                value_security, value_conformity, value_tradition, value_benevolence, value_universalism,
                comm_directness, comm_expressiveness, comm_formality, comm_conflict_approach,
                comm_listening_speaking, comm_emotional_logical,
                deep_embedding, archetype
            ) VALUES (
                $1, $2, $3,
                $4, $5, $6, $7, $8,
                $9,
                $10, $11, $12, $13, $14,
                $15, $16, $17, $18, $19, $20, $21, $22, $23, $24,
                $25, $26, $27, $28, $29, $30,
                $31, $32
            )
            "#,
        )
        .bind(db_id)
        .bind(&user_id)
        .bind(&twin.name)
        .bind(twin.big_five.openness)
        .bind(twin.big_five.conscientiousness)
        .bind(twin.big_five.extraversion)
        .bind(twin.big_five.agreeableness)
        .bind(twin.big_five.neuroticism)
        .bind(format!("{:?}", twin.attachment_style).to_lowercase())
        .bind(twin.eq.self_awareness)
        .bind(twin.eq.self_regulation)
        .bind(twin.eq.motivation)
        .bind(twin.eq.empathy)
        .bind(twin.eq.social_skills)
        .bind(twin.values.self_direction)
        .bind(twin.values.stimulation)
        .bind(twin.values.hedonism)
        .bind(twin.values.achievement)
        .bind(twin.values.power)
        .bind(twin.values.security)
        .bind(twin.values.conformity)
        .bind(twin.values.tradition)
        .bind(twin.values.benevolence)
        .bind(twin.values.universalism)
        .bind(twin.communication_style.directness)
        .bind(twin.communication_style.expressiveness)
        .bind(twin.communication_style.formality)
        .bind(twin.communication_style.conflict_approach)
        .bind(twin.communication_style.listening_speaking)
        .bind(twin.communication_style.emotional_logical)
        .bind(embedding)
        .bind(archetype)
        .execute(&self.pool)
        .await?;

        debug!("Stored digital twin with ID: {}", db_id);
        Ok(db_id)
    }

    /// Retrieve a digital twin by ID
    #[instrument(skip(self))]
    pub async fn get_twin(&self, id: &Uuid) -> Result<DigitalTwin, PostgresError> {
        let row = sqlx::query(
            r#"
            SELECT
                id, user_id, name,
                openness, conscientiousness, extraversion, agreeableness, neuroticism,
                attachment_style,
                eq_self_awareness, eq_self_regulation, eq_motivation, eq_empathy, eq_social_skills,
                value_self_direction, value_stimulation, value_hedonism, value_achievement, value_power,
                value_security, value_conformity, value_tradition, value_benevolence, value_universalism,
                comm_directness, comm_expressiveness, comm_formality, comm_conflict_approach,
                comm_listening_speaking, comm_emotional_logical,
                deep_embedding, archetype
            FROM digital_twins
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| PostgresError::NotFound(format!("Twin not found: {}", id)))?;

        self.row_to_twin(&row)
    }

    /// Get twin by user_id string
    pub async fn get_twin_by_user_id(&self, user_id: &str) -> Result<DigitalTwin, PostgresError> {
        let row = sqlx::query(
            r#"
            SELECT
                id, user_id, name,
                openness, conscientiousness, extraversion, agreeableness, neuroticism,
                attachment_style,
                eq_self_awareness, eq_self_regulation, eq_motivation, eq_empathy, eq_social_skills,
                value_self_direction, value_stimulation, value_hedonism, value_achievement, value_power,
                value_security, value_conformity, value_tradition, value_benevolence, value_universalism,
                comm_directness, comm_expressiveness, comm_formality, comm_conflict_approach,
                comm_listening_speaking, comm_emotional_logical,
                deep_embedding, archetype
            FROM digital_twins
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| PostgresError::NotFound(format!("Twin not found: {}", user_id)))?;

        self.row_to_twin(&row)
    }

    // ========================================================================
    // SIMD-ACCELERATED VECTOR SEARCH (RuVector)
    // ========================================================================

    /// Find k most similar personality matches using HNSW index
    ///
    /// This leverages RuVector's SIMD-accelerated cosine distance computation
    /// for sub-millisecond search across millions of profiles.
    #[instrument(skip(self))]
    pub async fn find_similar_twins(
        &self,
        target_id: &Uuid,
        limit: usize,
    ) -> Result<Vec<SimilarTwin>, PostgresError> {
        let rows = sqlx::query(
            r#"
            SELECT
                t.id, t.user_id, t.name, t.archetype,
                1 - (t.deep_embedding <=> target.deep_embedding) as similarity
            FROM digital_twins t
            CROSS JOIN (
                SELECT deep_embedding FROM digital_twins WHERE id = $1
            ) target
            WHERE t.id != $1
              AND t.deep_embedding IS NOT NULL
            ORDER BY t.deep_embedding <=> target.deep_embedding
            LIMIT $2
            "#,
        )
        .bind(target_id)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        let results = rows
            .iter()
            .map(|row| SimilarTwin {
                id: row.get("id"),
                user_id: row.get("user_id"),
                name: row.get("name"),
                archetype: row.get("archetype"),
                similarity: row.get::<f64, _>("similarity") as f32,
            })
            .collect();

        Ok(results)
    }

    /// Find matches by raw embedding vector
    #[instrument(skip(self, embedding))]
    pub async fn find_by_embedding(
        &self,
        embedding: &[f32],
        limit: usize,
    ) -> Result<Vec<SimilarTwin>, PostgresError> {
        if embedding.len() != self.config.embedding_dimension {
            return Err(PostgresError::DimensionMismatch {
                expected: self.config.embedding_dimension,
                actual: embedding.len(),
            });
        }

        let vec = Vector::from(embedding.to_vec());

        let rows = sqlx::query(
            r#"
            SELECT
                id, user_id, name, archetype,
                1 - (deep_embedding <=> $1) as similarity
            FROM digital_twins
            WHERE deep_embedding IS NOT NULL
            ORDER BY deep_embedding <=> $1
            LIMIT $2
            "#,
        )
        .bind(&vec)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        let results = rows
            .iter()
            .map(|row| SimilarTwin {
                id: row.get("id"),
                user_id: row.get("user_id"),
                name: row.get("name"),
                archetype: row.get("archetype"),
                similarity: row.get::<f64, _>("similarity") as f32,
            })
            .collect();

        Ok(results)
    }

    // ========================================================================
    // COMPATIBILITY SCORING
    // ========================================================================

    /// Store a computed compatibility score
    pub async fn store_compatibility(
        &self,
        twin_a: &Uuid,
        twin_b: &Uuid,
        domain: &str,
        score: &CompatibilityScore,
    ) -> Result<Uuid, PostgresError> {
        let id = Uuid::new_v4();

        let factors_json =
            serde_json::to_value(&score.factors).map_err(|e| PostgresError::Serialization(e.to_string()))?;

        sqlx::query(
            r#"
            INSERT INTO compatibility_scores (
                id, twin_a_id, twin_b_id, domain,
                compatibility_score, satisfaction_prediction, longevity_prediction,
                growth_potential, conflict_risk, factors, causal_confidence
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (twin_a_id, twin_b_id, domain) DO UPDATE SET
                compatibility_score = EXCLUDED.compatibility_score,
                satisfaction_prediction = EXCLUDED.satisfaction_prediction,
                longevity_prediction = EXCLUDED.longevity_prediction,
                growth_potential = EXCLUDED.growth_potential,
                conflict_risk = EXCLUDED.conflict_risk,
                factors = EXCLUDED.factors,
                causal_confidence = EXCLUDED.causal_confidence,
                computed_at = NOW()
            "#,
        )
        .bind(id)
        .bind(twin_a)
        .bind(twin_b)
        .bind(domain)
        .bind(score.compatibility)
        .bind(score.satisfaction)
        .bind(score.longevity)
        .bind(score.growth_potential)
        .bind(score.conflict_risk)
        .bind(factors_json)
        .bind(score.confidence)
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    /// Get top matches for a user in a specific domain
    pub async fn get_domain_matches(
        &self,
        twin_id: &Uuid,
        domain: &str,
        limit: usize,
    ) -> Result<Vec<CompatibilityMatch>, PostgresError> {
        let rows = sqlx::query(
            r#"
            SELECT
                cs.twin_b_id as matched_id,
                dt.name, dt.archetype,
                cs.compatibility_score, cs.satisfaction_prediction, cs.longevity_prediction,
                cs.growth_potential, cs.conflict_risk, cs.causal_confidence
            FROM compatibility_scores cs
            JOIN digital_twins dt ON dt.id = cs.twin_b_id
            WHERE cs.twin_a_id = $1 AND cs.domain = $2
            ORDER BY cs.compatibility_score DESC
            LIMIT $3
            "#,
        )
        .bind(twin_id)
        .bind(domain)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        let results = rows
            .iter()
            .map(|row| CompatibilityMatch {
                matched_id: row.get("matched_id"),
                name: row.get("name"),
                archetype: row.get("archetype"),
                compatibility_score: row.get("compatibility_score"),
                satisfaction_prediction: row.get("satisfaction_prediction"),
                longevity_prediction: row.get("longevity_prediction"),
                growth_potential: row.get("growth_potential"),
                conflict_risk: row.get("conflict_risk"),
                confidence: row.get("causal_confidence"),
            })
            .collect();

        Ok(results)
    }

    // ========================================================================
    // EMOTIONAL SIGNALS
    // ========================================================================

    /// Store an emotional signal for a user
    pub async fn store_emotional_signal(
        &self,
        twin_id: &Uuid,
        signal: &EmotionalSignalRecord,
    ) -> Result<Uuid, PostgresError> {
        let id = Uuid::new_v4();

        let embedding = if !signal.embedding.is_empty() {
            Some(Vector::from(signal.embedding.clone()))
        } else {
            None
        };

        let context_json = signal
            .context
            .as_ref()
            .map(|c| serde_json::to_value(c).ok())
            .flatten();

        sqlx::query(
            r#"
            INSERT INTO emotional_signals (
                id, twin_id, source, valence, arousal, dominance, confidence,
                signal_embedding, context
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
        )
        .bind(id)
        .bind(twin_id)
        .bind(&signal.source)
        .bind(signal.valence)
        .bind(signal.arousal)
        .bind(signal.dominance)
        .bind(signal.confidence)
        .bind(embedding)
        .bind(context_json)
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    /// Get emotional trajectory for a user over time
    pub async fn get_emotional_trajectory(
        &self,
        twin_id: &Uuid,
        hours_back: i32,
    ) -> Result<Vec<EmotionalTrajectoryPoint>, PostgresError> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM get_emotional_trajectory($1, $2)
            "#,
        )
        .bind(twin_id)
        .bind(hours_back)
        .fetch_all(&self.pool)
        .await?;

        let results = rows
            .iter()
            .map(|row| EmotionalTrajectoryPoint {
                hour_bucket: row.get("hour_bucket"),
                avg_valence: row.get("avg_valence"),
                avg_arousal: row.get("avg_arousal"),
                signal_count: row.get::<i64, _>("signal_count") as u64,
            })
            .collect();

        Ok(results)
    }

    // ========================================================================
    // ARIA CONVERSATIONS
    // ========================================================================

    /// Store an ARIA conversation exchange
    pub async fn store_conversation(
        &self,
        twin_id: &Uuid,
        session_id: &Uuid,
        conversation: &ConversationRecord,
    ) -> Result<Uuid, PostgresError> {
        let id = Uuid::new_v4();

        let contributions_json = serde_json::to_value(&conversation.agent_contributions)
            .map_err(|e| PostgresError::Serialization(e.to_string()))?;

        let suggestions_json = serde_json::to_value(&conversation.suggestions)
            .map_err(|e| PostgresError::Serialization(e.to_string()))?;

        let emotional_tone_json = conversation
            .emotional_tone
            .as_ref()
            .map(|e| serde_json::to_value(e).ok())
            .flatten();

        let embedding = if !conversation.message_embedding.is_empty() {
            Some(Vector::from(conversation.message_embedding.clone()))
        } else {
            None
        };

        sqlx::query(
            r#"
            INSERT INTO aria_conversations (
                id, twin_id, session_id, user_message, aria_response,
                primary_agent, agent_contributions, suggestions, growth_opportunity,
                emotional_tone, message_embedding
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(id)
        .bind(twin_id)
        .bind(session_id)
        .bind(&conversation.user_message)
        .bind(&conversation.aria_response)
        .bind(&conversation.primary_agent)
        .bind(contributions_json)
        .bind(suggestions_json)
        .bind(conversation.growth_opportunity)
        .bind(emotional_tone_json)
        .bind(embedding)
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    /// Find similar past conversations for context
    pub async fn find_similar_conversations(
        &self,
        twin_id: &Uuid,
        message_embedding: &[f32],
        limit: usize,
    ) -> Result<Vec<ConversationMatch>, PostgresError> {
        let vec = Vector::from(message_embedding.to_vec());

        let rows = sqlx::query(
            r#"
            SELECT
                id, user_message, aria_response, primary_agent,
                1 - (message_embedding <=> $1) as similarity
            FROM aria_conversations
            WHERE twin_id = $2
              AND message_embedding IS NOT NULL
            ORDER BY message_embedding <=> $1
            LIMIT $3
            "#,
        )
        .bind(&vec)
        .bind(twin_id)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        let results = rows
            .iter()
            .map(|row| ConversationMatch {
                id: row.get("id"),
                user_message: row.get("user_message"),
                aria_response: row.get("aria_response"),
                primary_agent: row.get("primary_agent"),
                similarity: row.get::<f64, _>("similarity") as f32,
            })
            .collect();

        Ok(results)
    }

    // ========================================================================
    // CAUSAL GRAPH
    // ========================================================================

    /// Add or update a causal edge
    pub async fn upsert_causal_edge(&self, edge: &CausalEdgeRecord) -> Result<Uuid, PostgresError> {
        let id = Uuid::new_v4();

        sqlx::query(
            r#"
            INSERT INTO causal_edges (id, cause, effect, uplift, confidence, sample_size)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (cause, effect) DO UPDATE SET
                uplift = (causal_edges.uplift * causal_edges.sample_size + EXCLUDED.uplift * EXCLUDED.sample_size)
                       / (causal_edges.sample_size + EXCLUDED.sample_size),
                confidence = GREATEST(causal_edges.confidence, EXCLUDED.confidence),
                sample_size = causal_edges.sample_size + EXCLUDED.sample_size,
                last_observed = NOW()
            "#,
        )
        .bind(id)
        .bind(&edge.cause)
        .bind(&edge.effect)
        .bind(edge.uplift)
        .bind(edge.confidence)
        .bind(edge.sample_size as i64)
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    /// Query effects of a cause
    pub async fn get_causal_effects(&self, cause: &str) -> Result<Vec<CausalEdgeRecord>, PostgresError> {
        let rows = sqlx::query(
            r#"
            SELECT cause, effect, uplift, confidence, sample_size, first_observed, last_observed
            FROM causal_edges
            WHERE cause = $1
            ORDER BY uplift DESC
            "#,
        )
        .bind(cause)
        .fetch_all(&self.pool)
        .await?;

        let results = rows
            .iter()
            .map(|row| CausalEdgeRecord {
                cause: row.get("cause"),
                effect: row.get("effect"),
                uplift: row.get("uplift"),
                confidence: row.get("confidence"),
                sample_size: row.get::<i64, _>("sample_size") as u64,
                first_observed: row.get("first_observed"),
                last_observed: row.get("last_observed"),
            })
            .collect();

        Ok(results)
    }

    // ========================================================================
    // PRIVACY-SAFE INSIGHTS
    // ========================================================================

    /// Store a privacy-safe insight (differential privacy protected)
    pub async fn store_privacy_insight(&self, insight: &PrivacyInsight) -> Result<Uuid, PostgresError> {
        let id = Uuid::new_v4();

        sqlx::query(
            r#"
            INSERT INTO privacy_safe_insights (
                id, anon_id, emotional_trend, resilience_bucket, stability_bucket,
                noise_scale, k_anonymity, time_bucket
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(id)
        .bind(&insight.anon_id)
        .bind(&insight.emotional_trend)
        .bind(insight.resilience_bucket)
        .bind(insight.stability_bucket)
        .bind(insight.noise_scale)
        .bind(insight.k_anonymity)
        .bind(insight.time_bucket)
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    // ========================================================================
    // DATABASE STATISTICS
    // ========================================================================

    /// Get database statistics
    pub async fn get_stats(&self) -> Result<DatabaseStats, PostgresError> {
        let twin_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM digital_twins")
            .fetch_one(&self.pool)
            .await?;

        let signal_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM emotional_signals")
            .fetch_one(&self.pool)
            .await?;

        let conversation_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM aria_conversations")
            .fetch_one(&self.pool)
            .await?;

        let compatibility_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM compatibility_scores")
            .fetch_one(&self.pool)
            .await?;

        let causal_edge_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM causal_edges")
            .fetch_one(&self.pool)
            .await?;

        Ok(DatabaseStats {
            digital_twins: twin_count as u64,
            emotional_signals: signal_count as u64,
            conversations: conversation_count as u64,
            compatibility_scores: compatibility_count as u64,
            causal_edges: causal_edge_count as u64,
        })
    }

    // ========================================================================
    // HELPERS
    // ========================================================================

    fn row_to_twin(&self, row: &PgRow) -> Result<DigitalTwin, PostgresError> {
        let id: Uuid = row.get("id");
        let attachment_str: String = row.get("attachment_style");

        let attachment_style = match attachment_str.as_str() {
            "secure" => AttachmentStyle::Secure,
            "anxious" => AttachmentStyle::Anxious,
            "avoidant" => AttachmentStyle::Avoidant,
            "disorganized" => AttachmentStyle::Disorganized,
            _ => AttachmentStyle::Secure,
        };

        let deep_embedding: Option<Vector> = row.get("deep_embedding");

        Ok(DigitalTwin {
            id,
            name: row.get("name"),
            big_five: BigFive {
                openness: row.get("openness"),
                conscientiousness: row.get("conscientiousness"),
                extraversion: row.get("extraversion"),
                agreeableness: row.get("agreeableness"),
                neuroticism: row.get("neuroticism"),
            },
            values: SchwartzValues {
                self_direction: row.get("value_self_direction"),
                stimulation: row.get("value_stimulation"),
                hedonism: row.get("value_hedonism"),
                achievement: row.get("value_achievement"),
                power: row.get("value_power"),
                security: row.get("value_security"),
                conformity: row.get("value_conformity"),
                tradition: row.get("value_tradition"),
                benevolence: row.get("value_benevolence"),
                universalism: row.get("value_universalism"),
            },
            attachment_style,
            eq: EmotionalIntelligence {
                self_awareness: row.get("eq_self_awareness"),
                self_regulation: row.get("eq_self_regulation"),
                motivation: row.get("eq_motivation"),
                empathy: row.get("eq_empathy"),
                social_skills: row.get("eq_social_skills"),
            },
            communication_style: CommunicationStyle {
                directness: row.get("comm_directness"),
                expressiveness: row.get("comm_expressiveness"),
                formality: row.get("comm_formality"),
                conflict_approach: row.get("comm_conflict_approach"),
                listening_speaking: row.get("comm_listening_speaking"),
                emotional_logical: row.get("comm_emotional_logical"),
            },
            deep_embedding: deep_embedding
                .map(|v| v.to_vec())
                .unwrap_or_default(),
            confidence: 1.0,
            observation_count: 1,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
}

// ============================================================================
// DATA TYPES FOR POSTGRES OPERATIONS
// ============================================================================

/// Result of finding similar twins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarTwin {
    pub id: Uuid,
    pub user_id: String,
    pub name: String,
    pub archetype: Option<String>,
    pub similarity: f32,
}

/// Compatibility score for storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityScore {
    pub compatibility: f32,
    pub satisfaction: f32,
    pub longevity: f32,
    pub growth_potential: f32,
    pub conflict_risk: f32,
    pub factors: Vec<String>,
    pub confidence: f32,
}

/// Result of compatibility query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityMatch {
    pub matched_id: Uuid,
    pub name: String,
    pub archetype: Option<String>,
    pub compatibility_score: f32,
    pub satisfaction_prediction: f32,
    pub longevity_prediction: f32,
    pub growth_potential: f32,
    pub conflict_risk: f32,
    pub confidence: f32,
}

/// Emotional signal record for storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalSignalRecord {
    pub source: String,
    pub valence: f32,
    pub arousal: f32,
    pub dominance: f32,
    pub confidence: f32,
    pub embedding: Vec<f32>,
    pub context: Option<serde_json::Value>,
}

/// Point in emotional trajectory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalTrajectoryPoint {
    pub hour_bucket: DateTime<Utc>,
    pub avg_valence: f32,
    pub avg_arousal: f32,
    pub signal_count: u64,
}

/// Conversation record for storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationRecord {
    pub user_message: String,
    pub aria_response: String,
    pub primary_agent: String,
    pub agent_contributions: Vec<String>,
    pub suggestions: Vec<String>,
    pub growth_opportunity: bool,
    pub emotional_tone: Option<serde_json::Value>,
    pub message_embedding: Vec<f32>,
}

/// Result of conversation similarity search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMatch {
    pub id: Uuid,
    pub user_message: String,
    pub aria_response: String,
    pub primary_agent: String,
    pub similarity: f32,
}

/// Causal edge record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalEdgeRecord {
    pub cause: String,
    pub effect: String,
    pub uplift: f32,
    pub confidence: f32,
    pub sample_size: u64,
    pub first_observed: DateTime<Utc>,
    pub last_observed: DateTime<Utc>,
}

/// Privacy-safe insight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyInsight {
    pub anon_id: String,
    pub emotional_trend: String,
    pub resilience_bucket: i32,
    pub stability_bucket: i32,
    pub noise_scale: f32,
    pub k_anonymity: i32,
    pub time_bucket: chrono::NaiveDate,
}

/// Database statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseStats {
    pub digital_twins: u64,
    pub emotional_signals: u64,
    pub conversations: u64,
    pub compatibility_scores: u64,
    pub causal_edges: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = PostgresConfig::default();
        assert_eq!(config.embedding_dimension, 4096);
        assert_eq!(config.max_connections, 10);
    }

    #[test]
    fn test_compatibility_score_serialization() {
        let score = CompatibilityScore {
            compatibility: 0.85,
            satisfaction: 0.80,
            longevity: 0.75,
            growth_potential: 0.90,
            conflict_risk: 0.20,
            factors: vec!["value_alignment".to_string()],
            confidence: 0.95,
        };

        let json = serde_json::to_string(&score).unwrap();
        let decoded: CompatibilityScore = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.compatibility, score.compatibility);
    }
}
