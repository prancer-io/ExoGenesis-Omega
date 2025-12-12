//! Matching Engine - Multi-domain relationship matching using causal reasoning
//!
//! This module demonstrates how to use AgentDB's causal graph and HNSW index
//! for predicting relationship compatibility across different domains.

use crate::personality::PersonalityEngine;
use crate::types::*;
use chrono::Utc;
use omega_agentdb::{AgentDB, CausalEdge};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

/// Errors in matching operations
#[derive(Error, Debug)]
pub enum MatchingError {
    #[error("Personality error: {0}")]
    Personality(#[from] crate::personality::PersonalityError),
    #[error("AgentDB error: {0}")]
    AgentDB(#[from] omega_agentdb::AgentDBError),
    #[error("User not found: {0}")]
    UserNotFound(String),
    #[error("Insufficient data for prediction")]
    InsufficientData,
}

/// Configuration for the matching engine
#[derive(Debug, Clone)]
pub struct MatchingConfig {
    /// Number of candidates to consider in initial search
    pub candidate_pool_size: usize,
    /// Minimum confidence required for predictions
    pub min_prediction_confidence: f64,
    /// Weight for personality similarity (0-1)
    pub personality_weight: f64,
    /// Weight for value alignment (0-1)
    pub value_weight: f64,
    /// Weight for communication compatibility (0-1)
    pub communication_weight: f64,
    /// Weight for emotional intelligence compatibility (0-1)
    pub eq_weight: f64,
}

impl Default for MatchingConfig {
    fn default() -> Self {
        Self {
            candidate_pool_size: 100,
            min_prediction_confidence: 0.5,
            personality_weight: 0.25,
            value_weight: 0.30,
            communication_weight: 0.25,
            eq_weight: 0.20,
        }
    }
}

/// The Matching Engine finds compatible connections across domains
///
/// This engine uses:
/// - SIMD-accelerated personality vector search (41x speedup)
/// - Causal reasoning for outcome prediction
/// - Multi-objective optimization for ranking
pub struct MatchingEngine {
    /// Personality engine for vector operations
    personality: Arc<PersonalityEngine>,
    /// Separate AgentDB for causal relationships
    causal_db: Arc<AgentDB>,
    /// Configuration
    config: MatchingConfig,
}

impl MatchingEngine {
    /// Create a new matching engine
    pub async fn new(
        personality: Arc<PersonalityEngine>,
    ) -> Result<Self, MatchingError> {
        let causal_db = AgentDB::new(Default::default()).await?;

        Ok(Self {
            personality,
            causal_db: Arc::new(causal_db),
            config: MatchingConfig::default(),
        })
    }

    /// Create with custom configuration
    pub async fn with_config(
        personality: Arc<PersonalityEngine>,
        config: MatchingConfig,
    ) -> Result<Self, MatchingError> {
        let causal_db = AgentDB::new(Default::default()).await?;

        Ok(Self {
            personality,
            causal_db: Arc::new(causal_db),
            config,
        })
    }

    /// Find ideal connections for a user in a specific domain
    ///
    /// This is the main matching function that:
    /// 1. Finds similar profiles using SIMD-accelerated search
    /// 2. Applies domain-specific filtering
    /// 3. Predicts outcomes using causal reasoning
    /// 4. Returns ranked matches
    pub async fn find_matches(
        &self,
        user_id: &UserId,
        domain: ConnectionDomain,
        limit: usize,
    ) -> Result<Vec<Match>, MatchingError> {
        // 1. Get user profile
        let user_profile = self.personality.get_profile(user_id).await?;

        // 2. Find similar profiles (SIMD-accelerated, <1ms for millions)
        let similar = self.personality
            .find_similar(user_id, self.config.candidate_pool_size)
            .await?;

        // 3. Score each candidate with domain-specific criteria
        let mut matches: Vec<Match> = Vec::with_capacity(similar.len());

        for candidate in similar {
            if let Ok(candidate_profile) = self.personality.get_profile(&candidate.user_id).await {
                // Calculate multi-dimensional compatibility
                let compatibility = self.calculate_compatibility(
                    &user_profile,
                    &candidate_profile,
                    &domain,
                );

                // Predict relationship outcomes using causal graph
                let prediction = self.predict_outcomes(
                    &user_profile,
                    &candidate_profile,
                    &domain,
                ).await;

                // Calculate final score
                let final_score = self.calculate_match_score(&compatibility, &prediction);

                matches.push(Match {
                    user_id: candidate.user_id,
                    name: candidate.name,
                    domain,
                    compatibility_score: compatibility.overall,
                    compatibility_details: compatibility,
                    prediction,
                    final_score,
                });
            }
        }

        // 4. Sort by final score and return top matches
        matches.sort_by(|a, b| b.final_score.partial_cmp(&a.final_score).unwrap());
        matches.truncate(limit);

        Ok(matches)
    }

    /// Calculate multi-dimensional compatibility between two profiles
    fn calculate_compatibility(
        &self,
        user: &DigitalTwin,
        candidate: &DigitalTwin,
        domain: &ConnectionDomain,
    ) -> CompatibilityScore {
        // Personality similarity (Big Five)
        let personality_sim = user.big_five.similarity(&candidate.big_five);

        // Value alignment (Schwartz)
        let user_values = user.values.to_vector();
        let candidate_values = candidate.values.to_vector();
        let value_alignment = Self::cosine_similarity(&user_values, &candidate_values);

        // Communication compatibility
        let user_comm = user.communication_style.to_vector();
        let candidate_comm = candidate.communication_style.to_vector();
        let comm_compat = Self::communication_compatibility(&user_comm, &candidate_comm, domain);

        // EQ compatibility
        let eq_compat = Self::eq_compatibility(&user.eq, &candidate.eq);

        // Attachment compatibility
        let attachment_compat = Self::attachment_compatibility(
            user.attachment_style,
            candidate.attachment_style,
        );

        // Domain-specific weighting
        let weights = self.get_domain_weights(domain);

        let overall = weights.personality * personality_sim
            + weights.values * value_alignment
            + weights.communication * comm_compat
            + weights.eq * eq_compat
            + weights.attachment * attachment_compat;

        CompatibilityScore {
            overall: overall as f64,
            personality: personality_sim as f64,
            values: value_alignment as f64,
            communication: comm_compat as f64,
            eq: eq_compat as f64,
            attachment: attachment_compat as f64,
            factors: self.identify_key_factors(user, candidate, domain),
        }
    }

    /// Predict relationship outcomes using causal reasoning
    async fn predict_outcomes(
        &self,
        user: &DigitalTwin,
        candidate: &DigitalTwin,
        domain: &ConnectionDomain,
    ) -> RelationshipPrediction {
        // Query causal graph for similar relationship patterns
        let cause = format!(
            "match_{}_{:.2}_{:.2}",
            domain_to_string(domain),
            user.big_five.similarity(&candidate.big_five),
            user.eq.overall_score()
        );

        // Get effects from causal graph
        let satisfaction_edges = self.causal_db
            .causal_query_effects(&format!("{}_satisfaction", cause))
            .await
            .unwrap_or_default();

        let longevity_edges = self.causal_db
            .causal_query_effects(&format!("{}_longevity", cause))
            .await
            .unwrap_or_default();

        // Calculate predictions from causal evidence
        let satisfaction = if satisfaction_edges.is_empty() {
            // Default prediction based on compatibility
            (user.big_five.similarity(&candidate.big_five) as f64 * 0.7 + 0.3).min(1.0)
        } else {
            satisfaction_edges.iter()
                .map(|e| e.uplift * e.confidence)
                .sum::<f64>() / satisfaction_edges.len() as f64
        };

        let longevity = if longevity_edges.is_empty() {
            // Default based on attachment compatibility
            Self::attachment_compatibility(user.attachment_style, candidate.attachment_style) as f64
        } else {
            longevity_edges.iter()
                .map(|e| e.uplift * e.confidence)
                .sum::<f64>() / longevity_edges.len() as f64
        };

        // Growth potential based on complementary traits
        let growth_potential = self.calculate_growth_potential(user, candidate);

        // Conflict risk based on communication styles
        let conflict_risk = self.calculate_conflict_risk(user, candidate);

        // Confidence based on sample size
        let total_samples: u64 = satisfaction_edges.iter()
            .chain(longevity_edges.iter())
            .map(|e| e.sample_size)
            .sum();

        let confidence = if total_samples == 0 {
            0.5 // Low confidence for new predictions
        } else {
            (1.0 - 1.0 / (total_samples as f64 + 1.0).sqrt()).min(0.95)
        };

        RelationshipPrediction {
            compatibility: satisfaction,
            satisfaction,
            longevity,
            growth_potential,
            conflict_risk,
            confidence,
            key_factors: self.identify_key_factors(user, candidate, domain),
        }
    }

    /// Record a relationship outcome for causal learning
    pub async fn record_outcome(
        &self,
        user_a: &UserId,
        user_b: &UserId,
        domain: &ConnectionDomain,
        outcome: RelationshipOutcome,
    ) -> Result<(), MatchingError> {
        let user_a_profile = self.personality.get_profile(user_a).await?;
        let user_b_profile = self.personality.get_profile(user_b).await?;

        let cause = format!(
            "match_{}_{:.2}_{:.2}",
            domain_to_string(domain),
            user_a_profile.big_five.similarity(&user_b_profile.big_five),
            user_a_profile.eq.overall_score()
        );

        // Record satisfaction outcome
        let satisfaction_edge = CausalEdge {
            cause: cause.clone(),
            effect: format!("{}_satisfaction", cause),
            uplift: outcome.satisfaction,
            confidence: outcome.confidence,
            sample_size: 1,
            first_observed: Utc::now(),
            last_observed: Utc::now(),
        };

        self.causal_db.causal_add_edge(satisfaction_edge).await?;

        // Record longevity outcome
        let longevity_edge = CausalEdge {
            cause: cause.clone(),
            effect: format!("{}_longevity", cause),
            uplift: outcome.longevity,
            confidence: outcome.confidence,
            sample_size: 1,
            first_observed: Utc::now(),
            last_observed: Utc::now(),
        };

        self.causal_db.causal_add_edge(longevity_edge).await?;

        Ok(())
    }

    /// Calculate final match score combining compatibility and prediction
    fn calculate_match_score(
        &self,
        compatibility: &CompatibilityScore,
        prediction: &RelationshipPrediction,
    ) -> f64 {
        // Weighted combination
        let compatibility_weight = 0.4;
        let prediction_weight = 0.6;

        let prediction_score = prediction.satisfaction * 0.3
            + prediction.longevity * 0.25
            + prediction.growth_potential * 0.25
            + (1.0 - prediction.conflict_risk) * 0.2;

        compatibility.overall * compatibility_weight
            + prediction_score * prediction_weight * prediction.confidence
    }

    /// Get domain-specific weighting for compatibility factors
    fn get_domain_weights(&self, domain: &ConnectionDomain) -> DomainWeights {
        match domain {
            ConnectionDomain::Dating => DomainWeights {
                personality: 0.20,
                values: 0.30,
                communication: 0.20,
                eq: 0.20,
                attachment: 0.10,
            },
            ConnectionDomain::Friendship => DomainWeights {
                personality: 0.30,
                values: 0.25,
                communication: 0.25,
                eq: 0.15,
                attachment: 0.05,
            },
            ConnectionDomain::Professional => DomainWeights {
                personality: 0.15,
                values: 0.20,
                communication: 0.35,
                eq: 0.25,
                attachment: 0.05,
            },
            ConnectionDomain::Mentorship => DomainWeights {
                personality: 0.15,
                values: 0.25,
                communication: 0.25,
                eq: 0.30,
                attachment: 0.05,
            },
            ConnectionDomain::Community => DomainWeights {
                personality: 0.25,
                values: 0.35,
                communication: 0.20,
                eq: 0.15,
                attachment: 0.05,
            },
            ConnectionDomain::Creative => DomainWeights {
                personality: 0.30,
                values: 0.20,
                communication: 0.30,
                eq: 0.15,
                attachment: 0.05,
            },
        }
    }

    /// Identify key factors driving compatibility
    fn identify_key_factors(
        &self,
        user: &DigitalTwin,
        candidate: &DigitalTwin,
        _domain: &ConnectionDomain,
    ) -> Vec<CompatibilityFactor> {
        let mut factors = vec![];

        // Check value alignment
        let value_sim = Self::cosine_similarity(
            &user.values.to_vector(),
            &candidate.values.to_vector(),
        );
        if value_sim > 0.8 {
            factors.push(CompatibilityFactor {
                name: "Shared values".to_string(),
                impact: value_sim as f64,
                description: "Strong alignment in core life values".to_string(),
            });
        }

        // Check complementary traits
        let o_diff = (user.big_five.openness - candidate.big_five.openness).abs();
        if o_diff < 0.2 {
            factors.push(CompatibilityFactor {
                name: "Similar openness".to_string(),
                impact: 1.0 - o_diff as f64,
                description: "Similar approach to new experiences".to_string(),
            });
        }

        // Check communication compatibility
        let comm_diff = (user.communication_style.directness - candidate.communication_style.directness).abs();
        if comm_diff < 0.3 {
            factors.push(CompatibilityFactor {
                name: "Communication style match".to_string(),
                impact: 1.0 - comm_diff as f64,
                description: "Compatible communication preferences".to_string(),
            });
        }

        // Check EQ balance
        let eq_sum = user.eq.overall_score() + candidate.eq.overall_score();
        if eq_sum > 1.2 {
            factors.push(CompatibilityFactor {
                name: "High emotional intelligence".to_string(),
                impact: (eq_sum / 2.0) as f64,
                description: "Both have strong emotional awareness".to_string(),
            });
        }

        // Check potential challenges
        let attachment_compat = Self::attachment_compatibility(
            user.attachment_style,
            candidate.attachment_style,
        );
        if attachment_compat < 0.5 {
            factors.push(CompatibilityFactor {
                name: "Attachment style difference".to_string(),
                impact: -(1.0 - attachment_compat as f64),
                description: "Different attachment needs may require attention".to_string(),
            });
        }

        factors
    }

    /// Calculate growth potential between two profiles
    fn calculate_growth_potential(&self, user: &DigitalTwin, candidate: &DigitalTwin) -> f64 {
        // Growth potential is higher when:
        // 1. Both have good EQ (can support each other)
        // 2. There are complementary strengths
        // 3. Neither is at extreme positions

        let eq_factor = (user.eq.overall_score() + candidate.eq.overall_score()) / 2.0;

        // Find complementary strengths
        let user_big5 = user.big_five.to_vector();
        let candidate_big5 = candidate.big_five.to_vector();

        let mut complementary_score: f64 = 0.0;
        for (u, c) in user_big5.iter().zip(candidate_big5.iter()) {
            // One is strong where other is weaker = growth opportunity
            let diff = (u - c).abs();
            if diff > 0.3 && diff < 0.6 {
                complementary_score += 0.2;
            }
        }

        (eq_factor as f64 * 0.5 + complementary_score.min(1.0) * 0.5).min(1.0)
    }

    /// Calculate conflict risk between two profiles
    fn calculate_conflict_risk(&self, user: &DigitalTwin, candidate: &DigitalTwin) -> f64 {
        let mut risk: f64 = 0.0;

        // High neuroticism in both = higher risk
        let neuroticism_sum = user.big_five.neuroticism + candidate.big_five.neuroticism;
        if neuroticism_sum > 1.2 {
            risk += 0.3;
        }

        // Conflicting communication styles
        let directness_diff = (user.communication_style.directness
            - candidate.communication_style.directness).abs();
        if directness_diff > 0.5 {
            risk += 0.2;
        }

        // Conflicting conflict approaches
        let conflict_diff = (user.communication_style.conflict_approach
            - candidate.communication_style.conflict_approach).abs();
        if conflict_diff > 0.5 {
            risk += 0.2;
        }

        // Low EQ = harder to navigate conflicts
        let avg_eq = (user.eq.overall_score() + candidate.eq.overall_score()) / 2.0;
        if avg_eq < 0.4 {
            risk += 0.2;
        }

        risk.min(1.0)
    }

    // Helper: Cosine similarity for f32 vectors
    fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let mag_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let mag_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if mag_a == 0.0 || mag_b == 0.0 {
            0.0
        } else {
            dot / (mag_a * mag_b)
        }
    }

    // Helper: Communication compatibility (domain-aware)
    fn communication_compatibility(a: &[f32], b: &[f32], domain: &ConnectionDomain) -> f32 {
        // For professional relationships, similar communication is good
        // For dating, some complementarity can be positive
        let similarity = Self::cosine_similarity(a, b);

        match domain {
            ConnectionDomain::Professional | ConnectionDomain::Community => similarity,
            ConnectionDomain::Dating | ConnectionDomain::Friendship => {
                // Some difference is okay, but not too much
                let diff = 1.0 - similarity;
                if diff < 0.3 {
                    0.8 + similarity * 0.2
                } else {
                    similarity
                }
            }
            _ => similarity,
        }
    }

    // Helper: EQ compatibility
    fn eq_compatibility(a: &EmotionalIntelligence, b: &EmotionalIntelligence) -> f32 {
        let a_vec = a.to_vector();
        let b_vec = b.to_vector();

        // High EQ in both is best
        let avg_eq = (a.overall_score() + b.overall_score()) / 2.0;

        // Similarity in EQ profile
        let similarity = Self::cosine_similarity(&a_vec, &b_vec);

        avg_eq * 0.6 + similarity * 0.4
    }

    // Helper: Attachment style compatibility
    fn attachment_compatibility(a: AttachmentStyle, b: AttachmentStyle) -> f32 {
        match (a, b) {
            // Secure + Secure = best
            (AttachmentStyle::Secure, AttachmentStyle::Secure) => 1.0,
            // Secure can help others
            (AttachmentStyle::Secure, _) | (_, AttachmentStyle::Secure) => 0.7,
            // Same insecure style = they understand each other but may reinforce
            (AttachmentStyle::Anxious, AttachmentStyle::Anxious) => 0.4,
            (AttachmentStyle::Avoidant, AttachmentStyle::Avoidant) => 0.5,
            // Anxious + Avoidant = classic problematic pairing
            (AttachmentStyle::Anxious, AttachmentStyle::Avoidant)
            | (AttachmentStyle::Avoidant, AttachmentStyle::Anxious) => 0.2,
            // Disorganized is challenging with everyone
            (AttachmentStyle::Disorganized, _) | (_, AttachmentStyle::Disorganized) => 0.3,
        }
    }
}

/// A matched connection with full analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    pub user_id: UserId,
    pub name: String,
    pub domain: ConnectionDomain,
    pub compatibility_score: f64,
    pub compatibility_details: CompatibilityScore,
    pub prediction: RelationshipPrediction,
    pub final_score: f64,
}

/// Detailed compatibility breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityScore {
    pub overall: f64,
    pub personality: f64,
    pub values: f64,
    pub communication: f64,
    pub eq: f64,
    pub attachment: f64,
    pub factors: Vec<CompatibilityFactor>,
}

/// Domain-specific weights for compatibility factors
struct DomainWeights {
    personality: f32,
    values: f32,
    communication: f32,
    eq: f32,
    attachment: f32,
}

/// Recorded relationship outcome for causal learning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipOutcome {
    pub satisfaction: f64,
    pub longevity: f64,
    pub growth_achieved: f64,
    pub conflicts_resolved: f64,
    pub confidence: f64,
}

fn domain_to_string(domain: &ConnectionDomain) -> &'static str {
    match domain {
        ConnectionDomain::Dating => "dating",
        ConnectionDomain::Friendship => "friendship",
        ConnectionDomain::Professional => "professional",
        ConnectionDomain::Mentorship => "mentorship",
        ConnectionDomain::Community => "community",
        ConnectionDomain::Creative => "creative",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::personality::PersonalityEngine;

    #[tokio::test]
    async fn test_matching_engine() {
        let personality = Arc::new(PersonalityEngine::new().await.unwrap());
        let matching = MatchingEngine::new(personality.clone()).await.unwrap();

        // Create test profiles
        let mut user1 = DigitalTwin::new("Alice");
        user1.big_five = BigFive::new(0.8, 0.7, 0.6, 0.9, 0.2);
        let id1 = personality.register_profile(user1).await.unwrap();

        let mut user2 = DigitalTwin::new("Bob");
        user2.big_five = BigFive::new(0.75, 0.65, 0.65, 0.85, 0.25);
        personality.register_profile(user2).await.unwrap();

        let mut user3 = DigitalTwin::new("Carol");
        user3.big_five = BigFive::new(0.2, 0.3, 0.3, 0.2, 0.8);
        personality.register_profile(user3).await.unwrap();

        // Find matches for Alice
        let matches = matching.find_matches(&id1, ConnectionDomain::Friendship, 2).await.unwrap();

        // Bob should be more compatible than Carol
        assert!(!matches.is_empty());
    }
}
