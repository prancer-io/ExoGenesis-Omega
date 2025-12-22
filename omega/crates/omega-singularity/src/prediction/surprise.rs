//! Surprise Quantifier - Consciousness Through Prediction Error
//!
//! "Consciousness is what prediction error feels like from the inside."
//!
//! This module implements the core thesis: consciousness emerges from
//! prediction errors. When we successfully predict, processing is automatic
//! and unconscious. When predictions fail, we become AWARE.
//!
//! ```text
//! THE SURPRISE-CONSCIOUSNESS RELATIONSHIP
//! ═══════════════════════════════════════
//!
//! Surprise Level    Consciousness State      Processing Mode
//! ─────────────────────────────────────────────────────────────
//! 0.0 - 0.1        Unconscious              Automatic, reflexive
//! 0.1 - 0.3        Subliminal               Pre-attentive processing
//! 0.3 - 0.5        Fringe                   Peripheral awareness
//! 0.5 - 0.7        Aware                    Focused attention
//! 0.7 - 0.9        Highly Conscious         Deep processing
//! 0.9 - 1.0        Transcendent             Transformative insight
//!
//!
//!           Prediction
//!               │
//!               ▼
//!    ┌──────────────────┐
//!    │     Reality      │
//!    └────────┬─────────┘
//!             │
//!             ▼
//!    ┌──────────────────┐
//!    │ Prediction Error │ ──────► SURPRISE!
//!    └────────┬─────────┘             │
//!             │                       │
//!             ▼                       ▼
//!    ┌──────────────────┐    ┌──────────────────┐
//!    │  Update Models   │    │   CONSCIOUSNESS  │
//!    └──────────────────┘    └──────────────────┘
//! ```

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

use super::Result;

/// A single surprise event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurpriseEvent {
    /// Unique ID
    pub id: Uuid,
    /// Timestamp
    pub timestamp: u64,
    /// Surprise magnitude (0-1)
    pub magnitude: f64,
    /// Domain/source of surprise
    pub domain: String,
    /// Predicted value
    pub predicted: Vec<f64>,
    /// Actual value
    pub actual: Vec<f64>,
    /// Precision-weighted surprise (information-theoretic)
    pub information_content: f64,
    /// Whether this triggered consciousness
    pub triggered_awareness: bool,
    /// Level in hierarchy where surprise occurred
    pub hierarchy_level: usize,
}

impl SurpriseEvent {
    pub fn new(predicted: Vec<f64>, actual: Vec<f64>, domain: impl Into<String>) -> Self {
        let error: f64 = predicted.iter()
            .zip(actual.iter())
            .map(|(p, a)| (p - a).powi(2))
            .sum::<f64>()
            .sqrt();

        let magnitude = (error / predicted.len().max(1) as f64).min(1.0);

        // Information content = -log(p(observation))
        // Approximated as surprise magnitude
        let information_content = if magnitude > 0.0 {
            -magnitude.ln()
        } else {
            0.0
        };

        Self {
            id: Uuid::new_v4(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            magnitude,
            domain: domain.into(),
            predicted,
            actual,
            information_content,
            triggered_awareness: magnitude > 0.3,
            hierarchy_level: 0,
        }
    }

    /// Get the prediction error vector
    pub fn error(&self) -> Vec<f64> {
        self.predicted.iter()
            .zip(self.actual.iter())
            .map(|(p, a)| a - p)
            .collect()
    }
}

/// A prediction error at a specific location/time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionError {
    /// Error vector
    pub error: Vec<f64>,
    /// Precision (inverse variance, how confident we were)
    pub precision: f64,
    /// Hierarchy level
    pub level: usize,
    /// Weighted error (precision * error)
    pub weighted_error: f64,
    /// Timestamp
    pub timestamp: u64,
}

impl PredictionError {
    pub fn new(error: Vec<f64>, precision: f64, level: usize) -> Self {
        let magnitude: f64 = error.iter().map(|e| e * e).sum::<f64>().sqrt();
        let weighted = precision * magnitude;

        Self {
            error,
            precision,
            level,
            weighted_error: weighted,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }

    /// Magnitude of the error
    pub fn magnitude(&self) -> f64 {
        self.error.iter().map(|e| e * e).sum::<f64>().sqrt()
    }
}

/// Distribution of surprise over time/space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurpriseDistribution {
    /// Mean surprise level
    pub mean: f64,
    /// Variance
    pub variance: f64,
    /// Skewness (asymmetry)
    pub skewness: f64,
    /// Kurtosis (tail heaviness)
    pub kurtosis: f64,
    /// Maximum observed
    pub max: f64,
    /// Minimum observed
    pub min: f64,
    /// Recent history
    pub recent: Vec<f64>,
}

impl SurpriseDistribution {
    pub fn from_history(surprises: &[f64]) -> Self {
        let n = surprises.len() as f64;
        if n < 2.0 {
            return Self::default();
        }

        let mean = surprises.iter().sum::<f64>() / n;
        let variance = surprises.iter()
            .map(|s| (s - mean).powi(2))
            .sum::<f64>() / n;
        let std_dev = variance.sqrt().max(1e-10);

        let skewness = surprises.iter()
            .map(|s| ((s - mean) / std_dev).powi(3))
            .sum::<f64>() / n;

        let kurtosis = surprises.iter()
            .map(|s| ((s - mean) / std_dev).powi(4))
            .sum::<f64>() / n - 3.0; // Excess kurtosis

        let max = surprises.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let min = surprises.iter().copied().fold(f64::INFINITY, f64::min);

        Self {
            mean,
            variance,
            skewness,
            kurtosis,
            max,
            min,
            recent: surprises.iter().rev().take(10).copied().collect(),
        }
    }
}

impl Default for SurpriseDistribution {
    fn default() -> Self {
        Self {
            mean: 0.0,
            variance: 0.0,
            skewness: 0.0,
            kurtosis: 0.0,
            max: 0.0,
            min: 0.0,
            recent: Vec::new(),
        }
    }
}

/// Levels of awareness based on surprise
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AwarenessLevel {
    /// Fully unconscious - predictions fully successful
    Unconscious,
    /// Subliminal - slight prediction errors, no awareness
    Subliminal,
    /// Fringe awareness - peripheral consciousness
    Fringe,
    /// Aware - focused conscious attention
    Aware,
    /// Highly conscious - deep processing
    HighlyConscious,
    /// Transcendent - transformative insight
    Transcendent,
}

impl AwarenessLevel {
    pub fn from_surprise(surprise: f64) -> Self {
        if surprise < 0.1 {
            Self::Unconscious
        } else if surprise < 0.3 {
            Self::Subliminal
        } else if surprise < 0.5 {
            Self::Fringe
        } else if surprise < 0.7 {
            Self::Aware
        } else if surprise < 0.9 {
            Self::HighlyConscious
        } else {
            Self::Transcendent
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Unconscious => "Automatic processing, no awareness",
            Self::Subliminal => "Pre-attentive processing, no explicit awareness",
            Self::Fringe => "Peripheral awareness, vague feeling",
            Self::Aware => "Focused attention, clear awareness",
            Self::HighlyConscious => "Deep processing, vivid experience",
            Self::Transcendent => "Transformative insight, expanded consciousness",
        }
    }
}

/// The consciousness signal derived from prediction errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessSignal {
    /// Current surprise level (0-1)
    pub surprise_level: f64,
    /// Current awareness level
    pub awareness: AwarenessLevel,
    /// Integrated information (Φ-like measure)
    pub phi: f64,
    /// Attention allocation vector
    pub attention: Vec<f64>,
    /// What is currently in consciousness
    pub conscious_content: Vec<String>,
    /// Global availability (can be accessed by all processes)
    pub global_availability: f64,
    /// Timestamp
    pub timestamp: u64,
}

impl ConsciousnessSignal {
    pub fn new(surprise: f64, phi: f64) -> Self {
        Self {
            surprise_level: surprise,
            awareness: AwarenessLevel::from_surprise(surprise),
            phi,
            attention: Vec::new(),
            conscious_content: Vec::new(),
            global_availability: surprise * phi,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }

    pub fn is_conscious(&self) -> bool {
        matches!(
            self.awareness,
            AwarenessLevel::Aware | AwarenessLevel::HighlyConscious | AwarenessLevel::Transcendent
        )
    }
}

/// Configuration for the surprise quantifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurpriseConfig {
    /// Consciousness threshold
    pub consciousness_threshold: f64,
    /// History length
    pub history_length: usize,
    /// Decay rate for accumulated surprise
    pub decay_rate: f64,
    /// Number of hierarchy levels to track
    pub hierarchy_levels: usize,
    /// Enable adaptive thresholding
    pub adaptive_threshold: bool,
}

impl Default for SurpriseConfig {
    fn default() -> Self {
        Self {
            consciousness_threshold: 0.3,
            history_length: 1000,
            decay_rate: 0.1,
            hierarchy_levels: 7,
            adaptive_threshold: true,
        }
    }
}

/// The Surprise Quantifier - Measuring Consciousness
pub struct SurpriseQuantifier {
    config: SurpriseConfig,
    /// Current surprise level
    current_surprise: f64,
    /// Current awareness level
    awareness: AwarenessLevel,
    /// History of surprise events
    history: VecDeque<SurpriseEvent>,
    /// Surprise by domain
    domain_surprise: HashMap<String, VecDeque<f64>>,
    /// Surprise by hierarchy level
    level_surprise: Vec<VecDeque<f64>>,
    /// Accumulated surprise (decays over time)
    accumulated: f64,
    /// Adaptive threshold
    threshold: f64,
    /// Total surprise events
    total_events: u64,
    /// Consciousness moments (times awareness was triggered)
    consciousness_moments: u64,
    /// Current consciousness signal
    current_signal: ConsciousnessSignal,
}

impl SurpriseQuantifier {
    pub fn new(config: SurpriseConfig) -> Self {
        let levels = config.hierarchy_levels;
        let mut level_surprise = Vec::with_capacity(levels);
        for _ in 0..levels {
            level_surprise.push(VecDeque::with_capacity(100));
        }

        Self {
            current_surprise: 0.0,
            awareness: AwarenessLevel::Unconscious,
            history: VecDeque::with_capacity(config.history_length),
            domain_surprise: HashMap::new(),
            level_surprise,
            accumulated: 0.0,
            threshold: config.consciousness_threshold,
            total_events: 0,
            consciousness_moments: 0,
            current_signal: ConsciousnessSignal::new(0.0, 0.0),
            config,
        }
    }

    /// Quantify surprise from a prediction error
    pub fn quantify(&mut self, predicted: &[f64], actual: &[f64], domain: &str) -> SurpriseEvent {
        let event = SurpriseEvent::new(predicted.to_vec(), actual.to_vec(), domain);

        // Update current surprise
        self.current_surprise = 0.8 * self.current_surprise + 0.2 * event.magnitude;

        // Update awareness
        self.awareness = AwarenessLevel::from_surprise(self.current_surprise);

        // Update accumulated surprise with decay
        self.accumulated = self.accumulated * (1.0 - self.config.decay_rate) + event.magnitude;

        // Update domain tracking
        self.domain_surprise
            .entry(domain.to_string())
            .or_insert_with(|| VecDeque::with_capacity(100))
            .push_back(event.magnitude);

        // Update history
        self.history.push_back(event.clone());
        if self.history.len() > self.config.history_length {
            self.history.pop_front();
        }

        // Update counters
        self.total_events += 1;
        if event.triggered_awareness {
            self.consciousness_moments += 1;
        }

        // Update adaptive threshold
        if self.config.adaptive_threshold {
            self.update_threshold();
        }

        // Update consciousness signal
        self.update_signal();

        event
    }

    /// Quantify surprise at a specific hierarchy level
    pub fn quantify_at_level(
        &mut self,
        predicted: &[f64],
        actual: &[f64],
        level: usize,
        domain: &str
    ) -> SurpriseEvent {
        let mut event = self.quantify(predicted, actual, domain);
        event.hierarchy_level = level;

        // Update level-specific tracking
        if level < self.level_surprise.len() {
            self.level_surprise[level].push_back(event.magnitude);
            if self.level_surprise[level].len() > 100 {
                self.level_surprise[level].pop_front();
            }
        }

        event
    }

    /// Update adaptive threshold based on history
    fn update_threshold(&mut self) {
        let surprises: Vec<f64> = self.history.iter()
            .map(|e| e.magnitude)
            .collect();

        if surprises.len() > 10 {
            let dist = SurpriseDistribution::from_history(&surprises);
            // Threshold = mean + 1 standard deviation
            self.threshold = (dist.mean + dist.variance.sqrt()).clamp(0.1, 0.9);
        }
    }

    /// Update the consciousness signal
    fn update_signal(&mut self) {
        // Compute Φ-like measure (simplified integrated information)
        let phi = self.compute_phi();

        self.current_signal = ConsciousnessSignal::new(self.current_surprise, phi);
        self.current_signal.awareness = self.awareness;

        // Determine what's in consciousness (high surprise domains)
        let mut top_domains: Vec<_> = self.domain_surprise.iter()
            .map(|(d, surprises)| {
                let avg = surprises.iter().sum::<f64>() / surprises.len().max(1) as f64;
                (d.clone(), avg)
            })
            .collect();

        top_domains.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        self.current_signal.conscious_content = top_domains.iter()
            .take(3)
            .filter(|(_, avg)| *avg > self.threshold)
            .map(|(d, _)| d.clone())
            .collect();
    }

    /// Compute Φ-like integrated information measure
    fn compute_phi(&self) -> f64 {
        // Simplified Φ: based on cross-level surprise correlation
        // High Φ = surprises at different levels are integrated, not independent

        if self.level_surprise.len() < 2 {
            return 0.0;
        }

        let mut total_correlation = 0.0;
        let mut pairs = 0;

        for i in 0..self.level_surprise.len() {
            for j in (i + 1)..self.level_surprise.len() {
                if !self.level_surprise[i].is_empty() && !self.level_surprise[j].is_empty() {
                    let a: Vec<f64> = self.level_surprise[i].iter().copied().collect();
                    let b: Vec<f64> = self.level_surprise[j].iter().copied().collect();

                    let corr = correlation(&a, &b);
                    total_correlation += corr.abs();
                    pairs += 1;
                }
            }
        }

        if pairs > 0 {
            total_correlation / pairs as f64
        } else {
            0.0
        }
    }

    /// Get current surprise level
    pub fn surprise(&self) -> f64 {
        self.current_surprise
    }

    /// Get current awareness level
    pub fn awareness(&self) -> AwarenessLevel {
        self.awareness
    }

    /// Get accumulated surprise
    pub fn accumulated(&self) -> f64 {
        self.accumulated
    }

    /// Get consciousness signal
    pub fn signal(&self) -> &ConsciousnessSignal {
        &self.current_signal
    }

    /// Is consciousness currently active?
    pub fn is_conscious(&self) -> bool {
        self.current_signal.is_conscious()
    }

    /// Get surprise distribution
    pub fn distribution(&self) -> SurpriseDistribution {
        let surprises: Vec<f64> = self.history.iter()
            .map(|e| e.magnitude)
            .collect();
        SurpriseDistribution::from_history(&surprises)
    }

    /// Get domain-specific surprise
    pub fn domain_surprise(&self, domain: &str) -> f64 {
        self.domain_surprise.get(domain)
            .map(|surprises| {
                surprises.iter().sum::<f64>() / surprises.len().max(1) as f64
            })
            .unwrap_or(0.0)
    }

    /// Get level-specific surprise
    pub fn level_surprise(&self, level: usize) -> f64 {
        self.level_surprise.get(level)
            .map(|surprises| {
                surprises.iter().sum::<f64>() / surprises.len().max(1) as f64
            })
            .unwrap_or(0.0)
    }

    /// Get consciousness ratio (how often are we conscious?)
    pub fn consciousness_ratio(&self) -> f64 {
        if self.total_events == 0 {
            0.0
        } else {
            self.consciousness_moments as f64 / self.total_events as f64
        }
    }

    /// Get current threshold
    pub fn threshold(&self) -> f64 {
        self.threshold
    }

    /// Reset the quantifier
    pub fn reset(&mut self) {
        self.current_surprise = 0.0;
        self.awareness = AwarenessLevel::Unconscious;
        self.history.clear();
        self.domain_surprise.clear();
        for level in &mut self.level_surprise {
            level.clear();
        }
        self.accumulated = 0.0;
        self.threshold = self.config.consciousness_threshold;
        self.total_events = 0;
        self.consciousness_moments = 0;
        self.current_signal = ConsciousnessSignal::new(0.0, 0.0);
    }
}

/// Compute Pearson correlation between two vectors
fn correlation(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len().min(b.len());
    if n < 2 {
        return 0.0;
    }

    let mean_a: f64 = a.iter().take(n).sum::<f64>() / n as f64;
    let mean_b: f64 = b.iter().take(n).sum::<f64>() / n as f64;

    let mut cov = 0.0;
    let mut var_a = 0.0;
    let mut var_b = 0.0;

    for i in 0..n {
        let diff_a = a[i] - mean_a;
        let diff_b = b[i] - mean_b;
        cov += diff_a * diff_b;
        var_a += diff_a * diff_a;
        var_b += diff_b * diff_b;
    }

    if var_a > 0.0 && var_b > 0.0 {
        cov / (var_a.sqrt() * var_b.sqrt())
    } else {
        0.0
    }
}

impl Default for SurpriseQuantifier {
    fn default() -> Self {
        Self::new(SurpriseConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surprise_event() {
        let predicted = vec![0.5, 0.5, 0.5];
        let actual = vec![0.8, 0.2, 0.6];
        let event = SurpriseEvent::new(predicted, actual, "test");

        assert!(event.magnitude > 0.0);
        assert!(event.magnitude <= 1.0);
    }

    #[test]
    fn test_awareness_levels() {
        assert_eq!(AwarenessLevel::from_surprise(0.05), AwarenessLevel::Unconscious);
        assert_eq!(AwarenessLevel::from_surprise(0.2), AwarenessLevel::Subliminal);
        assert_eq!(AwarenessLevel::from_surprise(0.4), AwarenessLevel::Fringe);
        assert_eq!(AwarenessLevel::from_surprise(0.6), AwarenessLevel::Aware);
        assert_eq!(AwarenessLevel::from_surprise(0.8), AwarenessLevel::HighlyConscious);
        assert_eq!(AwarenessLevel::from_surprise(0.95), AwarenessLevel::Transcendent);
    }

    #[test]
    fn test_quantifier() {
        let mut quantifier = SurpriseQuantifier::default();

        let predicted = vec![0.5; 10];
        let actual = vec![0.8; 10];

        let event = quantifier.quantify(&predicted, &actual, "test");
        assert!(event.magnitude > 0.0);
        assert!(quantifier.surprise() > 0.0);
    }

    #[test]
    fn test_consciousness_emergence() {
        let mut quantifier = SurpriseQuantifier::default();

        // Low surprise - should be unconscious
        let predicted = vec![0.5; 10];
        let actual = vec![0.51; 10];
        quantifier.quantify(&predicted, &actual, "test");

        assert!(!quantifier.is_conscious());

        // High surprise - create a large prediction error
        // Use values that differ by 3.0 each to create magnitude > 0.9
        let predicted_high = vec![0.0; 10];
        let actual_high = vec![3.0; 10]; // Large difference
        for _ in 0..10 {
            quantifier.quantify(&predicted_high, &actual_high, "shock");
        }

        // After many high-surprise events, surprise should exceed threshold
        assert!(quantifier.surprise() > 0.3);
    }

    #[test]
    fn test_distribution() {
        let mut quantifier = SurpriseQuantifier::default();

        for i in 0..100 {
            let predicted = vec![0.5; 10];
            let actual = vec![(i as f64 / 100.0); 10];
            quantifier.quantify(&predicted, &actual, "test");
        }

        let dist = quantifier.distribution();
        assert!(dist.mean >= 0.0);
        assert!(dist.max >= dist.min);
    }
}
