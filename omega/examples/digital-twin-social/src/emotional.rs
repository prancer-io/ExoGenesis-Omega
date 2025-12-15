//! Emotional Loop Processor - 7 temporal loops for emotional processing
//!
//! This module demonstrates how to use omega-loops for processing emotional
//! states at different timescales, from instant reactions to lifetime patterns.

use crate::types::*;
use chrono::{DateTime, Duration, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

// Note: This module implements emotional processing loops inspired by
// omega-loops architecture but tailored for emotional states.

/// Errors in emotional processing
#[derive(Error, Debug)]
pub enum EmotionalError {
    #[error("Loop error: {0}")]
    Loop(String),
    #[error("Invalid emotional state")]
    InvalidState,
}

/// The 7 emotional temporal loops
///
/// These map to different timescales of human emotional processing:
/// 1. Reflexive: Instant emotional reactions (milliseconds)
/// 2. Mood: Current emotional state (minutes)
/// 3. Daily: Daily emotional patterns (hours)
/// 4. Trait: Stable personality traits (weeks)
/// 5. Growth: Character development (months)
/// 6. Phase: Life phase changes (years)
/// 7. Identity: Core identity (lifetime)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EmotionalLoop {
    /// Loop 1: Instant reflexive emotions (~10ms)
    Reflexive,
    /// Loop 2: Current mood state (~5 minutes)
    Mood,
    /// Loop 3: Daily emotional patterns (~24 hours)
    Daily,
    /// Loop 4: Stable personality traits (~2-4 weeks)
    Trait,
    /// Loop 5: Character growth trajectory (~3-6 months)
    Growth,
    /// Loop 6: Life phase transitions (~1-5 years)
    Phase,
    /// Loop 7: Core identity and purpose (lifetime)
    Identity,
}

impl EmotionalLoop {
    /// Get the typical duration for this loop's processing window
    pub fn window_duration(&self) -> Duration {
        match self {
            EmotionalLoop::Reflexive => Duration::milliseconds(100),
            EmotionalLoop::Mood => Duration::minutes(5),
            EmotionalLoop::Daily => Duration::hours(24),
            EmotionalLoop::Trait => Duration::weeks(2),
            EmotionalLoop::Growth => Duration::days(90),
            EmotionalLoop::Phase => Duration::days(365),
            EmotionalLoop::Identity => Duration::days(365 * 10),
        }
    }

    /// Get the processing interval for this loop
    pub fn interval(&self) -> Duration {
        match self {
            EmotionalLoop::Reflexive => Duration::milliseconds(10),
            EmotionalLoop::Mood => Duration::seconds(30),
            EmotionalLoop::Daily => Duration::hours(1),
            EmotionalLoop::Trait => Duration::days(1),
            EmotionalLoop::Growth => Duration::weeks(1),
            EmotionalLoop::Phase => Duration::days(30),
            EmotionalLoop::Identity => Duration::days(90),
        }
    }
}

/// State for a single emotional loop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopState {
    pub loop_type: EmotionalLoop,
    pub current_state: EmotionalState,
    pub history: VecDeque<EmotionalState>,
    pub max_history: usize,
    pub last_processed: DateTime<Utc>,
    pub cycle_count: u64,
}

impl LoopState {
    pub fn new(loop_type: EmotionalLoop) -> Self {
        let max_history = match loop_type {
            EmotionalLoop::Reflexive => 100,
            EmotionalLoop::Mood => 50,
            EmotionalLoop::Daily => 30,
            EmotionalLoop::Trait => 14,
            EmotionalLoop::Growth => 12,
            EmotionalLoop::Phase => 10,
            EmotionalLoop::Identity => 5,
        };

        Self {
            loop_type,
            current_state: EmotionalState::neutral(),
            history: VecDeque::with_capacity(max_history),
            max_history,
            last_processed: Utc::now(),
            cycle_count: 0,
        }
    }

    /// Add a new emotional state to history
    pub fn record_state(&mut self, state: EmotionalState) {
        if self.history.len() >= self.max_history {
            self.history.pop_front();
        }
        self.history.push_back(state.clone());
        self.current_state = state;
        self.last_processed = Utc::now();
        self.cycle_count += 1;
    }

    /// Calculate average emotional state over history
    pub fn average_state(&self) -> EmotionalState {
        if self.history.is_empty() {
            return EmotionalState::neutral();
        }

        let mut sum_valence = 0.0f32;
        let mut sum_arousal = 0.0f32;
        let mut sum_dominance = 0.0f32;

        for state in &self.history {
            sum_valence += state.valence;
            sum_arousal += state.arousal;
            sum_dominance += state.dominance;
        }

        let count = self.history.len() as f32;

        EmotionalState {
            primary: self.current_state.primary,
            primary_intensity: self.current_state.primary_intensity,
            secondary: self.current_state.secondary,
            secondary_intensity: self.current_state.secondary_intensity,
            valence: sum_valence / count,
            arousal: sum_arousal / count,
            dominance: sum_dominance / count,
            timestamp: Utc::now(),
        }
    }

    /// Calculate emotional volatility (variance in VAD space)
    pub fn volatility(&self) -> f32 {
        if self.history.len() < 2 {
            return 0.0;
        }

        let avg = self.average_state();
        let avg_vad = avg.to_vad();

        let variance: f32 = self.history.iter()
            .map(|s| {
                let vad = s.to_vad();
                vad.iter()
                    .zip(avg_vad.iter())
                    .map(|(a, b)| (a - b).powi(2))
                    .sum::<f32>()
            })
            .sum::<f32>() / self.history.len() as f32;

        variance.sqrt()
    }
}

/// The Emotional Loop Processor manages all 7 temporal loops
///
/// This demonstrates the Omega Brain's nested temporal loop architecture
/// applied to emotional processing.
pub struct EmotionalLoopProcessor {
    /// State for each loop
    loops: Arc<RwLock<[LoopState; 7]>>,
    /// Input buffer for raw emotional signals
    input_buffer: Arc<RwLock<VecDeque<EmotionalSignal>>>,
    /// Threshold for escalating to higher loops
    escalation_threshold: f32,
}

/// Raw emotional signal from sensors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalSignal {
    pub source: SignalSource,
    pub valence: f32,
    pub arousal: f32,
    pub dominance: f32,
    pub confidence: f32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SignalSource {
    Keyboard,
    Wearable,
    Facial,
    Voice,
    Text,
    Context,
}

impl EmotionalLoopProcessor {
    /// Create a new emotional loop processor
    pub fn new() -> Self {
        Self {
            loops: Arc::new(RwLock::new([
                LoopState::new(EmotionalLoop::Reflexive),
                LoopState::new(EmotionalLoop::Mood),
                LoopState::new(EmotionalLoop::Daily),
                LoopState::new(EmotionalLoop::Trait),
                LoopState::new(EmotionalLoop::Growth),
                LoopState::new(EmotionalLoop::Phase),
                LoopState::new(EmotionalLoop::Identity),
            ])),
            input_buffer: Arc::new(RwLock::new(VecDeque::with_capacity(1000))),
            escalation_threshold: 0.3,
        }
    }

    /// Add a raw emotional signal to be processed
    pub async fn add_signal(&self, signal: EmotionalSignal) {
        let mut buffer = self.input_buffer.write().await;
        if buffer.len() >= 1000 {
            buffer.pop_front();
        }
        buffer.push_back(signal);
    }

    /// Process Loop 1: Reflexive emotions (instant reactions)
    ///
    /// This loop processes raw signals and generates immediate emotional state.
    pub async fn process_reflexive(&self) -> EmotionalState {
        let mut buffer = self.input_buffer.write().await;
        let mut loops = self.loops.write().await;

        if buffer.is_empty() {
            return loops[0].current_state.clone();
        }

        // Aggregate recent signals (weighted by recency and confidence)
        let mut weighted_valence = 0.0f32;
        let mut weighted_arousal = 0.0f32;
        let mut weighted_dominance = 0.0f32;
        let mut total_weight = 0.0f32;

        let now = Utc::now();
        let window = EmotionalLoop::Reflexive.window_duration();

        let signals: Vec<_> = buffer.drain(..).collect();
        for signal in signals {
            let age = now.signed_duration_since(signal.timestamp);
            if age <= window {
                let recency_weight = 1.0 - (age.num_milliseconds() as f32 / window.num_milliseconds() as f32);
                let weight = signal.confidence * recency_weight;

                weighted_valence += signal.valence * weight;
                weighted_arousal += signal.arousal * weight;
                weighted_dominance += signal.dominance * weight;
                total_weight += weight;
            }
        }

        let state = if total_weight > 0.0 {
            EmotionalState {
                primary: self.valence_to_emotion(weighted_valence / total_weight),
                primary_intensity: (weighted_arousal / total_weight).abs(),
                secondary: None,
                secondary_intensity: 0.0,
                valence: weighted_valence / total_weight,
                arousal: weighted_arousal / total_weight,
                dominance: weighted_dominance / total_weight,
                timestamp: now,
            }
        } else {
            EmotionalState::neutral()
        };

        loops[0].record_state(state.clone());
        state
    }

    /// Process Loop 2: Mood state (aggregated over minutes)
    ///
    /// This loop smooths reflexive states into a stable mood.
    pub async fn process_mood(&self) -> EmotionalState {
        let mut loops = self.loops.write().await;

        // Aggregate reflexive states
        let reflexive_avg = loops[0].average_state();
        let current_mood = &loops[1].current_state;

        // Smooth transition (exponential moving average)
        let alpha = 0.3; // Smoothing factor
        let new_state = EmotionalState {
            primary: reflexive_avg.primary,
            primary_intensity: current_mood.primary_intensity * (1.0 - alpha)
                + reflexive_avg.primary_intensity * alpha,
            secondary: reflexive_avg.secondary,
            secondary_intensity: reflexive_avg.secondary_intensity,
            valence: current_mood.valence * (1.0 - alpha) + reflexive_avg.valence * alpha,
            arousal: current_mood.arousal * (1.0 - alpha) + reflexive_avg.arousal * alpha,
            dominance: current_mood.dominance * (1.0 - alpha) + reflexive_avg.dominance * alpha,
            timestamp: Utc::now(),
        };

        loops[1].record_state(new_state.clone());
        new_state
    }

    /// Process Loop 3: Daily patterns
    ///
    /// Identifies circadian emotional rhythms.
    pub async fn process_daily(&self) -> DailyPattern {
        let loops = self.loops.read().await;
        let mood_history = &loops[1].history;

        // Group by hour and calculate averages
        let mut hourly_valence = [0.0f32; 24];
        let mut hourly_count = [0u32; 24];

        for state in mood_history {
            let hour = state.timestamp.hour() as usize;
            hourly_valence[hour] += state.valence;
            hourly_count[hour] += 1;
        }

        // Calculate averages
        let hourly_averages: Vec<f32> = hourly_valence.iter()
            .zip(hourly_count.iter())
            .map(|(sum, count)| if *count > 0 { sum / *count as f32 } else { 0.0 })
            .collect();

        // Find peak and trough hours
        let (peak_hour, _) = hourly_averages.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap_or((12, &0.0));

        let (trough_hour, _) = hourly_averages.iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap_or((3, &0.0));

        DailyPattern {
            hourly_valence: hourly_averages.try_into().unwrap_or([0.0; 24]),
            peak_hour: peak_hour as u8,
            trough_hour: trough_hour as u8,
            volatility: loops[2].volatility(),
        }
    }

    /// Process Loop 4: Trait stability
    ///
    /// Identifies stable personality-level emotional patterns.
    pub async fn process_traits(&self) -> TraitAnalysis {
        let loops = self.loops.read().await;

        // Calculate trait-level stability
        let baseline_valence = loops[3].average_state().valence;
        let baseline_arousal = loops[3].average_state().arousal;
        let baseline_dominance = loops[3].average_state().dominance;

        // Calculate how much daily patterns deviate from baseline
        let daily_deviation = loops[2].volatility();

        // Trait stability = inverse of deviation
        let stability = 1.0 / (1.0 + daily_deviation);

        TraitAnalysis {
            baseline_valence,
            baseline_arousal,
            baseline_dominance,
            stability,
            dominant_pattern: self.valence_to_emotion(baseline_valence),
        }
    }

    /// Process Loop 5: Growth trajectory
    ///
    /// Tracks personal development and emotional intelligence growth.
    pub async fn process_growth(&self) -> GrowthAnalysis {
        let loops = self.loops.read().await;
        let trait_history = &loops[3].history;

        if trait_history.len() < 2 {
            return GrowthAnalysis {
                eq_growth_rate: 0.0,
                stability_trend: 0.0,
                resilience_score: 0.5,
                growth_areas: vec![],
            };
        }

        // Calculate trend in emotional regulation
        let recent_volatility = loops[3].volatility();
        let older_states: Vec<_> = trait_history.iter().take(trait_history.len() / 2).collect();
        let recent_states: Vec<_> = trait_history.iter().skip(trait_history.len() / 2).collect();

        let older_avg_valence: f32 = older_states.iter().map(|s| s.valence).sum::<f32>()
            / older_states.len().max(1) as f32;
        let recent_avg_valence: f32 = recent_states.iter().map(|s| s.valence).sum::<f32>()
            / recent_states.len().max(1) as f32;

        let valence_trend = recent_avg_valence - older_avg_valence;

        // Growth areas based on current weaknesses
        let mut growth_areas = vec![];
        let avg_state = loops[4].average_state();

        if avg_state.dominance < 0.4 {
            growth_areas.push("Assertiveness".to_string());
        }
        if recent_volatility > 0.5 {
            growth_areas.push("Emotional stability".to_string());
        }
        if avg_state.valence < 0.0 {
            growth_areas.push("Positivity cultivation".to_string());
        }

        GrowthAnalysis {
            eq_growth_rate: valence_trend,
            stability_trend: -recent_volatility + 0.5,
            resilience_score: 1.0 - recent_volatility.min(1.0),
            growth_areas,
        }
    }

    /// Process Loop 6: Life phase analysis
    pub async fn process_life_phase(&self) -> LifePhaseAnalysis {
        let loops = self.loops.read().await;
        let growth_avg = loops[4].average_state();

        // Simplified life phase detection based on emotional patterns
        let phase = if growth_avg.arousal > 0.7 && growth_avg.dominance > 0.6 {
            LifePhase::Striving
        } else if growth_avg.valence > 0.5 && growth_avg.dominance > 0.5 {
            LifePhase::Flourishing
        } else if growth_avg.valence < 0.0 {
            LifePhase::Challenging
        } else {
            LifePhase::Stable
        };

        LifePhaseAnalysis {
            current_phase: phase,
            phase_duration_months: loops[5].cycle_count as u32,
            transition_indicators: vec![],
        }
    }

    /// Process Loop 7: Core identity
    pub async fn process_identity(&self) -> IdentityAnalysis {
        let loops = self.loops.read().await;

        // Core identity is the most stable emotional signature
        let identity_avg = loops[6].average_state();

        // Calculate core values based on long-term emotional patterns
        let core_values = self.derive_core_values(&identity_avg);

        IdentityAnalysis {
            core_emotional_signature: identity_avg,
            core_values,
            identity_coherence: 1.0 - loops[6].volatility(),
        }
    }

    /// Get the current state of all loops
    pub async fn get_all_states(&self) -> [EmotionalState; 7] {
        let loops = self.loops.read().await;
        [
            loops[0].current_state.clone(),
            loops[1].current_state.clone(),
            loops[2].current_state.clone(),
            loops[3].current_state.clone(),
            loops[4].current_state.clone(),
            loops[5].current_state.clone(),
            loops[6].current_state.clone(),
        ]
    }

    /// Check if emotional state should escalate to higher loops
    pub async fn check_escalation(&self) -> Vec<EmotionalLoop> {
        let loops = self.loops.read().await;
        let mut escalations = vec![];

        // Check if reflexive volatility exceeds threshold
        if loops[0].volatility() > self.escalation_threshold {
            escalations.push(EmotionalLoop::Mood);
        }

        // Check if mood deviation is significant
        let mood_deviation = (loops[1].current_state.valence - loops[3].average_state().valence).abs();
        if mood_deviation > self.escalation_threshold * 2.0 {
            escalations.push(EmotionalLoop::Daily);
        }

        escalations
    }

    // Helper: Convert valence to primary emotion
    fn valence_to_emotion(&self, valence: f32) -> CoreEmotion {
        if valence > 0.5 {
            CoreEmotion::Joy
        } else if valence > 0.2 {
            CoreEmotion::Trust
        } else if valence > -0.2 {
            CoreEmotion::Anticipation
        } else if valence > -0.5 {
            CoreEmotion::Sadness
        } else {
            CoreEmotion::Fear
        }
    }

    // Helper: Derive core values from emotional patterns
    fn derive_core_values(&self, state: &EmotionalState) -> Vec<String> {
        let mut values = vec![];

        if state.dominance > 0.6 {
            values.push("Leadership".to_string());
            values.push("Achievement".to_string());
        }
        if state.valence > 0.5 {
            values.push("Optimism".to_string());
            values.push("Connection".to_string());
        }
        if state.arousal < 0.4 {
            values.push("Peace".to_string());
            values.push("Stability".to_string());
        }

        if values.is_empty() {
            values.push("Balance".to_string());
        }

        values
    }
}

impl Default for EmotionalLoopProcessor {
    fn default() -> Self {
        Self::new()
    }
}

// Analysis result types

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyPattern {
    pub hourly_valence: [f32; 24],
    pub peak_hour: u8,
    pub trough_hour: u8,
    pub volatility: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitAnalysis {
    pub baseline_valence: f32,
    pub baseline_arousal: f32,
    pub baseline_dominance: f32,
    pub stability: f32,
    pub dominant_pattern: CoreEmotion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthAnalysis {
    pub eq_growth_rate: f32,
    pub stability_trend: f32,
    pub resilience_score: f32,
    pub growth_areas: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LifePhase {
    Striving,
    Flourishing,
    Challenging,
    Stable,
    Transitioning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifePhaseAnalysis {
    pub current_phase: LifePhase,
    pub phase_duration_months: u32,
    pub transition_indicators: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityAnalysis {
    pub core_emotional_signature: EmotionalState,
    pub core_values: Vec<String>,
    pub identity_coherence: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_emotional_loop_processor() {
        let processor = EmotionalLoopProcessor::new();

        // Add some test signals
        for i in 0..10 {
            let signal = EmotionalSignal {
                source: SignalSource::Keyboard,
                valence: 0.5 + (i as f32 * 0.05),
                arousal: 0.4,
                dominance: 0.5,
                confidence: 0.8,
                timestamp: Utc::now(),
            };
            processor.add_signal(signal).await;
        }

        // Process reflexive loop
        let state = processor.process_reflexive().await;
        assert!(state.valence > 0.0);
    }

    #[tokio::test]
    async fn test_mood_smoothing() {
        let processor = EmotionalLoopProcessor::new();

        // Add varying signals
        for i in 0..20 {
            let valence = if i % 2 == 0 { 0.8 } else { 0.2 };
            let signal = EmotionalSignal {
                source: SignalSource::Text,
                valence,
                arousal: 0.5,
                dominance: 0.5,
                confidence: 0.9,
                timestamp: Utc::now(),
            };
            processor.add_signal(signal).await;
        }

        processor.process_reflexive().await;

        // Process mood multiple times to allow EMA to converge
        let mut mood = processor.process_mood().await;
        for _ in 0..10 {
            mood = processor.process_mood().await;
        }

        // Mood should be smoothed between extremes after convergence
        assert!(
            mood.valence >= 0.2 && mood.valence <= 0.8,
            "Expected mood valence to be smoothed between 0.2 and 0.8 after convergence, got {}",
            mood.valence
        );
    }
}
