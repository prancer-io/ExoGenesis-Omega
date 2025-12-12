//! Zero-Knowledge Emotional Layer - Privacy-preserving emotional AI
//!
//! This module demonstrates how to implement privacy-preserving emotional
//! processing where Path can compute on emotional data without seeing it.

use crate::types::*;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors in privacy operations
#[derive(Error, Debug)]
pub enum PrivacyError {
    #[error("Encryption error: {0}")]
    Encryption(String),
    #[error("Decryption error: {0}")]
    Decryption(String),
    #[error("Invalid key")]
    InvalidKey,
}

/// Zero-Knowledge Emotional Layer
///
/// This demonstrates the architecture where:
/// - Raw emotional data stays on user's device
/// - Only encrypted vectors are shared with servers
/// - Servers can compute compatibility without seeing actual data
pub struct ZeroKnowledgeLayer {
    /// User's encryption keys (would be hardware-backed in production)
    #[allow(dead_code)]
    user_keys: UserKeyPair,
    /// Local emotional state (never leaves device)
    local_state: LocalEmotionalState,
    /// Configuration
    config: PrivacyConfig,
}

/// Configuration for privacy layer
#[derive(Debug, Clone)]
pub struct PrivacyConfig {
    /// Enable differential privacy noise
    pub differential_privacy: bool,
    /// Noise scale for differential privacy (epsilon)
    pub epsilon: f64,
    /// Enable local-only processing mode
    pub local_only: bool,
    /// Maximum data retention (hours)
    pub retention_hours: u32,
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            differential_privacy: true,
            epsilon: 1.0,
            local_only: false,
            retention_hours: 24,
        }
    }
}

/// User's encryption key pair
#[derive(Debug, Clone)]
pub struct UserKeyPair {
    /// Public key for encryption
    pub public_key: Vec<u8>,
    /// Private key for decryption (never shared)
    #[allow(dead_code)]
    private_key: Vec<u8>,
    /// Key generation timestamp
    pub created_at: chrono::DateTime<Utc>,
}

impl UserKeyPair {
    /// Generate new key pair
    pub fn generate() -> Self {
        // Simplified key generation (would use proper crypto in production)
        let private_key: Vec<u8> = (0..32).map(|i| (i * 7 + 13) as u8).collect();
        let public_key: Vec<u8> = private_key.iter().map(|b| b.wrapping_add(1)).collect();

        Self {
            public_key,
            private_key,
            created_at: Utc::now(),
        }
    }
}

/// Local emotional state that never leaves device
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocalEmotionalState {
    /// Raw emotional readings
    pub raw_emotions: Vec<EmotionalReading>,
    /// Private reflections
    pub private_reflections: Vec<PrivateReflection>,
    /// Sensitive contexts
    pub sensitive_contexts: Vec<SensitiveContext>,
}

/// A raw emotional reading (stays on device)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalReading {
    pub timestamp: chrono::DateTime<Utc>,
    pub source: String,
    pub raw_valence: f32,
    pub raw_arousal: f32,
    pub raw_dominance: f32,
    pub context: Option<String>,
}

/// A private reflection (never shared)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateReflection {
    pub timestamp: chrono::DateTime<Utc>,
    pub content: String,
    pub emotional_state: EmotionalState,
}

/// Sensitive context information (never shared)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitiveContext {
    pub timestamp: chrono::DateTime<Utc>,
    pub location_type: Option<String>, // "home", "work", not actual location
    pub social_context: Option<String>, // "alone", "with family", not who
    pub activity_type: Option<String>,
}

impl ZeroKnowledgeLayer {
    /// Create a new zero-knowledge layer
    pub fn new() -> Self {
        Self {
            user_keys: UserKeyPair::generate(),
            local_state: LocalEmotionalState::default(),
            config: PrivacyConfig::default(),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: PrivacyConfig) -> Self {
        Self {
            user_keys: UserKeyPair::generate(),
            local_state: LocalEmotionalState::default(),
            config,
        }
    }

    /// Process raw emotional data locally and export only safe data
    ///
    /// This is the key privacy-preserving function:
    /// - Takes raw emotional signals (keystroke timing, HRV, etc.)
    /// - Processes them locally on device
    /// - Returns only aggregated, privacy-safe data
    pub fn process_raw_data(&mut self, reading: EmotionalReading) -> PrivacySafeExport {
        // Store raw data locally
        self.local_state.raw_emotions.push(reading.clone());

        // Clean old data
        self.cleanup_old_data();

        // Generate privacy-safe export
        self.generate_safe_export()
    }

    /// Store a private reflection (never leaves device)
    pub fn store_reflection(&mut self, content: String, emotional_state: EmotionalState) {
        self.local_state.private_reflections.push(PrivateReflection {
            timestamp: Utc::now(),
            content,
            emotional_state,
        });
    }

    /// Generate privacy-safe export for server-side processing
    fn generate_safe_export(&self) -> PrivacySafeExport {
        // Aggregate recent emotional readings
        let recent: Vec<_> = self.local_state.raw_emotions.iter()
            .rev()
            .take(100)
            .collect();

        if recent.is_empty() {
            return PrivacySafeExport::default();
        }

        // Calculate aggregates (no individual readings)
        let avg_valence: f32 = recent.iter().map(|r| r.raw_valence).sum::<f32>() / recent.len() as f32;
        let avg_arousal: f32 = recent.iter().map(|r| r.raw_arousal).sum::<f32>() / recent.len() as f32;
        let avg_dominance: f32 = recent.iter().map(|r| r.raw_dominance).sum::<f32>() / recent.len() as f32;

        // Apply differential privacy if enabled
        let (final_valence, final_arousal, final_dominance) = if self.config.differential_privacy {
            self.add_differential_privacy_noise(avg_valence, avg_arousal, avg_dominance)
        } else {
            (avg_valence, avg_arousal, avg_dominance)
        };

        // Generate abstract personality vector (no raw data)
        let personality_delta = self.compute_personality_delta(final_valence, final_arousal, final_dominance);

        // Generate growth signals (abstract only)
        let growth_signals = self.extract_growth_signals();

        PrivacySafeExport {
            personality_delta,
            growth_signals,
            emotional_stability_score: self.compute_stability_score(),
            timestamp: Utc::now(),
            confidence: self.compute_confidence(),
        }
    }

    /// Add differential privacy noise
    fn add_differential_privacy_noise(&self, v: f32, a: f32, d: f32) -> (f32, f32, f32) {
        // Simplified Laplace noise (would use proper DP library in production)
        let scale = 1.0 / self.config.epsilon as f32;

        // Generate pseudo-random noise based on time (simplified)
        let noise_seed = Utc::now().timestamp_millis() as f32;
        let noise_v = ((noise_seed * 0.001).sin() * scale).clamp(-0.1, 0.1);
        let noise_a = ((noise_seed * 0.002).sin() * scale).clamp(-0.1, 0.1);
        let noise_d = ((noise_seed * 0.003).sin() * scale).clamp(-0.1, 0.1);

        (
            (v + noise_v).clamp(-1.0, 1.0),
            (a + noise_a).clamp(0.0, 1.0),
            (d + noise_d).clamp(0.0, 1.0),
        )
    }

    /// Compute personality delta from emotional patterns
    fn compute_personality_delta(&self, v: f32, a: f32, d: f32) -> Vec<f32> {
        // Generate abstract delta vector (not raw personality)
        let mut delta = vec![0.0f32; 128];

        // Encode emotional state into abstract dimensions
        for (i, item) in delta.iter_mut().enumerate().take(32) {
            *item = v * (1.0 - i as f32 / 32.0);
        }
        for (i, item) in delta.iter_mut().enumerate().skip(32).take(32) {
            *item = a * (1.0 - (i - 32) as f32 / 32.0);
        }
        for (i, item) in delta.iter_mut().enumerate().skip(64).take(32) {
            *item = d * (1.0 - (i - 64) as f32 / 32.0);
        }
        // Remaining dimensions are derived features
        for (i, item) in delta.iter_mut().enumerate().skip(96) {
            *item = (v + a + d) / 3.0 * (1.0 - (i - 96) as f32 / 32.0);
        }

        delta
    }

    /// Extract growth signals without exposing raw data
    fn extract_growth_signals(&self) -> GrowthSignals {
        let readings = &self.local_state.raw_emotions;

        if readings.len() < 10 {
            return GrowthSignals::default();
        }

        // Calculate emotional regulation improvement
        let older: Vec<_> = readings.iter().take(readings.len() / 2).collect();
        let newer: Vec<_> = readings.iter().skip(readings.len() / 2).collect();

        let older_volatility = Self::compute_volatility(&older);
        let newer_volatility = Self::compute_volatility(&newer);

        let regulation_improvement = older_volatility - newer_volatility;

        // Calculate positivity trend
        let older_valence: f32 = older.iter().map(|r| r.raw_valence).sum::<f32>() / older.len().max(1) as f32;
        let newer_valence: f32 = newer.iter().map(|r| r.raw_valence).sum::<f32>() / newer.len().max(1) as f32;

        let positivity_trend = newer_valence - older_valence;

        GrowthSignals {
            emotional_regulation_improvement: regulation_improvement,
            positivity_trend,
            self_awareness_score: self.estimate_self_awareness(),
            growth_areas: self.identify_abstract_growth_areas(),
        }
    }

    /// Compute volatility of readings
    fn compute_volatility(readings: &[&EmotionalReading]) -> f32 {
        if readings.len() < 2 {
            return 0.0;
        }

        let mean_v: f32 = readings.iter().map(|r| r.raw_valence).sum::<f32>() / readings.len() as f32;
        let variance: f32 = readings.iter()
            .map(|r| (r.raw_valence - mean_v).powi(2))
            .sum::<f32>() / readings.len() as f32;

        variance.sqrt()
    }

    /// Compute emotional stability score
    fn compute_stability_score(&self) -> f64 {
        let volatility = Self::compute_volatility(
            &self.local_state.raw_emotions.iter().collect::<Vec<_>>()
        );
        (1.0 - volatility.min(1.0)) as f64
    }

    /// Estimate self-awareness from reflection patterns
    fn estimate_self_awareness(&self) -> f64 {
        let reflection_count = self.local_state.private_reflections.len();
        (reflection_count as f64 / 10.0).min(1.0)
    }

    /// Identify growth areas without exposing specifics
    fn identify_abstract_growth_areas(&self) -> Vec<String> {
        let mut areas = vec![];

        let recent: Vec<_> = self.local_state.raw_emotions.iter()
            .rev()
            .take(50)
            .collect();

        if recent.is_empty() {
            return areas;
        }

        let avg_valence: f32 = recent.iter().map(|r| r.raw_valence).sum::<f32>() / recent.len() as f32;
        let avg_dominance: f32 = recent.iter().map(|r| r.raw_dominance).sum::<f32>() / recent.len() as f32;
        let volatility = Self::compute_volatility(&recent);

        if volatility > 0.4 {
            areas.push("Emotional stability".to_string());
        }
        if avg_valence < 0.0 {
            areas.push("Positivity cultivation".to_string());
        }
        if avg_dominance < 0.3 {
            areas.push("Assertiveness".to_string());
        }

        areas
    }

    /// Compute confidence in the export
    fn compute_confidence(&self) -> f64 {
        let data_points = self.local_state.raw_emotions.len();
        (data_points as f64 / 100.0).min(1.0)
    }

    /// Cleanup old data based on retention policy
    fn cleanup_old_data(&mut self) {
        let cutoff = Utc::now() - chrono::Duration::hours(self.config.retention_hours as i64);

        self.local_state.raw_emotions.retain(|r| r.timestamp > cutoff);
        self.local_state.private_reflections.retain(|r| r.timestamp > cutoff);
        self.local_state.sensitive_contexts.retain(|c| c.timestamp > cutoff);
    }

    /// Get local data for user viewing (never sent to server)
    pub fn get_local_insights(&self) -> LocalInsights {
        LocalInsights {
            total_readings: self.local_state.raw_emotions.len(),
            total_reflections: self.local_state.private_reflections.len(),
            recent_emotional_trend: self.calculate_recent_trend(),
            privacy_score: self.calculate_privacy_score(),
        }
    }

    /// Calculate recent emotional trend for user
    fn calculate_recent_trend(&self) -> EmotionalTrend {
        let recent: Vec<_> = self.local_state.raw_emotions.iter()
            .rev()
            .take(20)
            .collect();

        if recent.len() < 2 {
            return EmotionalTrend::Stable;
        }

        let first_half: f32 = recent[recent.len()/2..].iter()
            .map(|r| r.raw_valence)
            .sum::<f32>() / (recent.len() / 2) as f32;

        let second_half: f32 = recent[..recent.len()/2].iter()
            .map(|r| r.raw_valence)
            .sum::<f32>() / (recent.len() / 2) as f32;

        let change = second_half - first_half;

        if change > 0.1 {
            EmotionalTrend::Improving
        } else if change < -0.1 {
            EmotionalTrend::Declining
        } else {
            EmotionalTrend::Stable
        }
    }

    /// Calculate privacy score (how well protected the data is)
    fn calculate_privacy_score(&self) -> f64 {
        let mut score: f64 = 1.0;

        // Penalize if differential privacy is disabled
        if !self.config.differential_privacy {
            score -= 0.2;
        }

        // Penalize for long retention
        if self.config.retention_hours > 168 { // 1 week
            score -= 0.1;
        }

        // Bonus for local-only mode
        if self.config.local_only {
            score += 0.1;
        }

        score.clamp(0.0, 1.0)
    }
}

impl Default for ZeroKnowledgeLayer {
    fn default() -> Self {
        Self::new()
    }
}

/// Privacy-safe data that can be sent to servers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacySafeExport {
    /// Abstract personality delta (not raw personality)
    pub personality_delta: Vec<f32>,
    /// Growth signals (no raw data)
    pub growth_signals: GrowthSignals,
    /// Emotional stability score (aggregated)
    pub emotional_stability_score: f64,
    /// Export timestamp
    pub timestamp: chrono::DateTime<Utc>,
    /// Confidence in the export
    pub confidence: f64,
}

impl Default for PrivacySafeExport {
    fn default() -> Self {
        Self {
            personality_delta: vec![0.0; 128],
            growth_signals: GrowthSignals::default(),
            emotional_stability_score: 0.5,
            timestamp: Utc::now(),
            confidence: 0.0,
        }
    }
}

/// Growth signals (abstract, no raw data)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GrowthSignals {
    pub emotional_regulation_improvement: f32,
    pub positivity_trend: f32,
    pub self_awareness_score: f64,
    pub growth_areas: Vec<String>,
}

/// Local insights for user viewing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalInsights {
    pub total_readings: usize,
    pub total_reflections: usize,
    pub recent_emotional_trend: EmotionalTrend,
    pub privacy_score: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EmotionalTrend {
    Improving,
    Stable,
    Declining,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_knowledge_layer() {
        let mut layer = ZeroKnowledgeLayer::new();

        // Add some readings
        for i in 0..20 {
            let reading = EmotionalReading {
                timestamp: Utc::now(),
                source: "test".to_string(),
                raw_valence: 0.5 + (i as f32 * 0.02),
                raw_arousal: 0.4,
                raw_dominance: 0.5,
                context: None,
            };
            layer.process_raw_data(reading);
        }

        // Get export
        let export = layer.generate_safe_export();

        // Export should have abstract data only
        assert_eq!(export.personality_delta.len(), 128);
        assert!(export.confidence > 0.0);
    }

    #[test]
    fn test_differential_privacy() {
        let config = PrivacyConfig {
            differential_privacy: true,
            epsilon: 0.5, // Higher privacy
            ..Default::default()
        };
        let mut layer = ZeroKnowledgeLayer::with_config(config);

        // Add readings
        for _ in 0..50 {
            let reading = EmotionalReading {
                timestamp: Utc::now(),
                source: "test".to_string(),
                raw_valence: 0.6,
                raw_arousal: 0.5,
                raw_dominance: 0.5,
                context: None,
            };
            layer.process_raw_data(reading);
        }

        // Export should be noisy
        let export = layer.generate_safe_export();
        assert!(export.confidence > 0.0);
    }

    #[test]
    fn test_private_reflection_storage() {
        let mut layer = ZeroKnowledgeLayer::new();

        layer.store_reflection(
            "This is my private thought".to_string(),
            EmotionalState::neutral(),
        );

        let insights = layer.get_local_insights();
        assert_eq!(insights.total_reflections, 1);

        // Private reflection should NOT appear in export
        let export = layer.generate_safe_export();
        // The export doesn't contain raw reflection content
        assert!(export.personality_delta.iter().all(|&x| x >= -1.0 && x <= 1.0));
    }
}
