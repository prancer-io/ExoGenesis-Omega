//! Temporal Cascade - Predictions Across All Timescales
//!
//! Consciousness requires predictions at multiple temporal scales simultaneously.
//! From the Planck scale (10^-44 seconds) to cosmic scales (10^17 years),
//! each level feeds into and constrains the others.
//!
//! ```text
//! TEMPORAL PREDICTION CASCADE
//! ═══════════════════════════
//!
//! Scale          Duration        Example Predictions
//! ─────────────────────────────────────────────────────
//! Planck         ~10⁻⁴⁴ s       Quantum fluctuations
//! Femtosecond    ~10⁻¹⁵ s       Electron transitions
//! Picosecond     ~10⁻¹² s       Molecular vibrations
//! Nanosecond     ~10⁻⁹ s        Synaptic transmission
//! Microsecond    ~10⁻⁶ s        Neural spike timing
//! Millisecond    ~10⁻³ s        Sensory processing
//! Second         ~1 s           Immediate actions
//! Minute         ~60 s          Short-term planning
//! Hour           ~3600 s        Task completion
//! Day            ~86400 s       Daily routines
//! Year           ~3×10⁷ s       Life goals
//! Cosmic         ~10¹⁷+ s       Civilizational scale
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

use super::Result;

/// Planck scale identifier
pub const PLANCK_SCALE: usize = 0;
/// Cosmic scale identifier
pub const COSMIC_SCALE: usize = 11;

/// Temporal scales for prediction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TemporalScale {
    /// Planck time (~5.39 × 10^-44 s) - quantum realm
    Planck,
    /// Femtosecond (10^-15 s) - electron dynamics
    Femtosecond,
    /// Picosecond (10^-12 s) - molecular motion
    Picosecond,
    /// Nanosecond (10^-9 s) - synaptic events
    Nanosecond,
    /// Microsecond (10^-6 s) - neural spikes
    Microsecond,
    /// Millisecond (10^-3 s) - sensory binding
    Millisecond,
    /// Second - conscious moment
    Second,
    /// Minute - short-term memory
    Minute,
    /// Hour - working memory span
    Hour,
    /// Day - episodic memory
    Day,
    /// Year - autobiographical memory
    Year,
    /// Cosmic (10^17+ years) - civilizational/universal
    Cosmic,
}

impl TemporalScale {
    /// Get the characteristic duration for this scale (as Duration)
    /// Note: Sub-nanosecond scales (Planck, Femtosecond, Picosecond) round to minimum
    pub fn duration(&self) -> Duration {
        match self {
            Self::Planck => Duration::from_nanos(1), // Minimum representable
            Self::Femtosecond => Duration::from_nanos(1), // Minimum representable
            Self::Picosecond => Duration::from_nanos(1), // Minimum representable
            Self::Nanosecond => Duration::from_nanos(1),
            Self::Microsecond => Duration::from_micros(1),
            Self::Millisecond => Duration::from_millis(1),
            Self::Second => Duration::from_secs(1),
            Self::Minute => Duration::from_secs(60),
            Self::Hour => Duration::from_secs(3600),
            Self::Day => Duration::from_secs(86400),
            Self::Year => Duration::from_secs(31_536_000),
            Self::Cosmic => Duration::from_secs(u64::MAX), // Effectively infinite
        }
    }

    /// Get the characteristic duration in seconds as f64 (for sub-nanosecond precision)
    pub fn seconds_f64(&self) -> f64 {
        match self {
            Self::Planck => 5.39e-44,
            Self::Femtosecond => 1e-15,
            Self::Picosecond => 1e-12,
            Self::Nanosecond => 1e-9,
            Self::Microsecond => 1e-6,
            Self::Millisecond => 1e-3,
            Self::Second => 1.0,
            Self::Minute => 60.0,
            Self::Hour => 3600.0,
            Self::Day => 86400.0,
            Self::Year => 31_536_000.0,
            Self::Cosmic => f64::MAX,
        }
    }

    /// Get scale index (0 = Planck, 11 = Cosmic)
    pub fn index(&self) -> usize {
        match self {
            Self::Planck => 0,
            Self::Femtosecond => 1,
            Self::Picosecond => 2,
            Self::Nanosecond => 3,
            Self::Microsecond => 4,
            Self::Millisecond => 5,
            Self::Second => 6,
            Self::Minute => 7,
            Self::Hour => 8,
            Self::Day => 9,
            Self::Year => 10,
            Self::Cosmic => 11,
        }
    }

    /// Get scale from index
    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Planck),
            1 => Some(Self::Femtosecond),
            2 => Some(Self::Picosecond),
            3 => Some(Self::Nanosecond),
            4 => Some(Self::Microsecond),
            5 => Some(Self::Millisecond),
            6 => Some(Self::Second),
            7 => Some(Self::Minute),
            8 => Some(Self::Hour),
            9 => Some(Self::Day),
            10 => Some(Self::Year),
            11 => Some(Self::Cosmic),
            _ => None,
        }
    }

    /// Get all scales in order
    pub fn all() -> Vec<Self> {
        vec![
            Self::Planck,
            Self::Femtosecond,
            Self::Picosecond,
            Self::Nanosecond,
            Self::Microsecond,
            Self::Millisecond,
            Self::Second,
            Self::Minute,
            Self::Hour,
            Self::Day,
            Self::Year,
            Self::Cosmic,
        ]
    }

    /// Get the next finer scale
    pub fn finer(&self) -> Option<Self> {
        let idx = self.index();
        if idx > 0 {
            Self::from_index(idx - 1)
        } else {
            None
        }
    }

    /// Get the next coarser scale
    pub fn coarser(&self) -> Option<Self> {
        Self::from_index(self.index() + 1)
    }

    /// Human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Planck => "Planck",
            Self::Femtosecond => "Femtosecond",
            Self::Picosecond => "Picosecond",
            Self::Nanosecond => "Nanosecond",
            Self::Microsecond => "Microsecond",
            Self::Millisecond => "Millisecond",
            Self::Second => "Second",
            Self::Minute => "Minute",
            Self::Hour => "Hour",
            Self::Day => "Day",
            Self::Year => "Year",
            Self::Cosmic => "Cosmic",
        }
    }
}

/// A prediction at a specific temporal scale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalePrediction {
    /// Unique ID
    pub id: Uuid,
    /// Temporal scale
    pub scale: TemporalScale,
    /// Predicted state vector
    pub predicted_state: Vec<f64>,
    /// Prediction confidence (0-1)
    pub confidence: f64,
    /// Prediction horizon at this scale
    pub horizon: Duration,
    /// Precision (inverse variance)
    pub precision: f64,
    /// Prediction error from last verification
    pub last_error: f64,
    /// Number of successful predictions at this scale
    pub successes: u64,
    /// Total predictions made
    pub total: u64,
    /// Constraints from coarser scales
    pub top_down_constraints: Vec<f64>,
    /// Signals from finer scales
    pub bottom_up_signals: Vec<f64>,
}

impl ScalePrediction {
    pub fn new(scale: TemporalScale, dim: usize) -> Self {
        Self {
            id: Uuid::new_v4(),
            scale,
            predicted_state: vec![0.0; dim],
            confidence: 0.5,
            horizon: scale.duration() * 10, // Predict 10 units ahead
            precision: 1.0,
            last_error: 0.0,
            successes: 0,
            total: 0,
            top_down_constraints: vec![],
            bottom_up_signals: vec![],
        }
    }

    /// Accuracy rate
    pub fn accuracy(&self) -> f64 {
        if self.total == 0 {
            0.5
        } else {
            self.successes as f64 / self.total as f64
        }
    }

    /// Update prediction with new observation
    pub fn update(&mut self, observation: &[f64], learning_rate: f64) {
        self.total += 1;

        // Calculate prediction error
        let error: f64 = self.predicted_state.iter()
            .zip(observation.iter())
            .map(|(p, o)| (p - o).powi(2))
            .sum::<f64>()
            .sqrt();

        self.last_error = error;

        // Update success count
        if error < 0.5 {
            self.successes += 1;
        }

        // Update precision based on error
        self.precision = 1.0 / (error + 0.1);
        self.precision = self.precision.clamp(0.1, 10.0);

        // Update confidence
        self.confidence = 0.9 * self.confidence + 0.1 * (1.0 - error.min(1.0));

        // Learn from error - adjust predicted state
        for (i, (pred, obs)) in self.predicted_state.iter_mut()
            .zip(observation.iter()).enumerate()
        {
            // Incorporate top-down constraints
            let constraint = self.top_down_constraints.get(i).copied().unwrap_or(0.0);
            // Incorporate bottom-up signals
            let signal = self.bottom_up_signals.get(i).copied().unwrap_or(0.0);

            // Weighted update
            let target = 0.6 * obs + 0.2 * constraint + 0.2 * signal;
            *pred += learning_rate * self.precision * (target - *pred);
        }
    }

    /// Generate prediction for next timestep
    pub fn predict(&mut self, current_state: &[f64]) -> Vec<f64> {
        // Simple prediction: weighted combination of current state and learned pattern
        let mut prediction = Vec::with_capacity(current_state.len());

        for (i, &current) in current_state.iter().enumerate() {
            let learned = self.predicted_state.get(i).copied().unwrap_or(0.0);
            let constraint = self.top_down_constraints.get(i).copied().unwrap_or(0.0);

            // Prediction = current trend + learned pattern + top-down constraint
            let pred = 0.5 * current + 0.3 * learned + 0.2 * constraint;
            prediction.push(pred);
        }

        self.predicted_state = prediction.clone();
        prediction
    }
}

/// Configuration for the temporal cascade
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CascadeConfig {
    /// Dimension of state vectors
    pub state_dim: usize,
    /// Learning rate for prediction updates
    pub learning_rate: f64,
    /// Weight for top-down influence
    pub top_down_weight: f64,
    /// Weight for bottom-up influence
    pub bottom_up_weight: f64,
    /// Minimum scale to use (0 = Planck)
    pub min_scale: usize,
    /// Maximum scale to use (11 = Cosmic)
    pub max_scale: usize,
    /// Enable cross-scale coupling
    pub cross_scale_coupling: bool,
}

impl Default for CascadeConfig {
    fn default() -> Self {
        Self {
            state_dim: 64,
            learning_rate: 0.1,
            top_down_weight: 0.3,
            bottom_up_weight: 0.3,
            min_scale: 5,  // Start at millisecond for practical systems
            max_scale: 10, // Up to year scale
            cross_scale_coupling: true,
        }
    }
}

/// The Temporal Cascade - predictions at all timescales
pub struct TemporalCascade {
    config: CascadeConfig,
    /// Predictions at each scale
    scales: HashMap<TemporalScale, ScalePrediction>,
    /// Cross-scale coupling matrix
    coupling: Vec<Vec<f64>>,
    /// Overall cascade coherence
    coherence: f64,
    /// Total prediction error across scales
    total_error: f64,
    /// Dominant scale (where most activity is)
    dominant_scale: TemporalScale,
}

impl TemporalCascade {
    /// Create a new temporal cascade
    pub fn new(config: CascadeConfig) -> Self {
        let mut scales = HashMap::new();

        for scale in TemporalScale::all() {
            let idx = scale.index();
            if idx >= config.min_scale && idx <= config.max_scale {
                scales.insert(scale, ScalePrediction::new(scale, config.state_dim));
            }
        }

        // Initialize coupling matrix
        let num_scales = config.max_scale - config.min_scale + 1;
        let coupling = vec![vec![0.0; num_scales]; num_scales];

        Self {
            config,
            scales,
            coupling,
            coherence: 0.0,
            total_error: 0.0,
            dominant_scale: TemporalScale::Second,
        }
    }

    /// Process input through the cascade
    pub fn process(&mut self, input: &[f64]) -> Result<CascadeResult> {
        let mut predictions = HashMap::new();
        let mut errors = HashMap::new();
        let mut total_error = 0.0;

        // Bottom-up pass: finer scales inform coarser scales
        let mut bottom_up_signal = input.to_vec();

        for idx in self.config.min_scale..=self.config.max_scale {
            if let Some(scale) = TemporalScale::from_index(idx) {
                if let Some(predictor) = self.scales.get_mut(&scale) {
                    // Set bottom-up signals
                    predictor.bottom_up_signals = bottom_up_signal.clone();

                    // Generate prediction
                    let prediction = predictor.predict(&bottom_up_signal);
                    predictions.insert(scale, prediction.clone());

                    // This scale's prediction becomes next scale's bottom-up signal
                    bottom_up_signal = prediction;
                }
            }
        }

        // Top-down pass: coarser scales constrain finer scales
        let mut top_down_constraint = vec![0.0; self.config.state_dim];

        for idx in (self.config.min_scale..=self.config.max_scale).rev() {
            if let Some(scale) = TemporalScale::from_index(idx) {
                if let Some(predictor) = self.scales.get_mut(&scale) {
                    // Set top-down constraints
                    predictor.top_down_constraints = top_down_constraint.clone();

                    // This scale's state becomes next scale's top-down constraint
                    top_down_constraint = predictor.predicted_state.clone();
                }
            }
        }

        // Update with observation and calculate errors
        for idx in self.config.min_scale..=self.config.max_scale {
            if let Some(scale) = TemporalScale::from_index(idx) {
                if let Some(predictor) = self.scales.get_mut(&scale) {
                    predictor.update(input, self.config.learning_rate);
                    errors.insert(scale, predictor.last_error);
                    total_error += predictor.last_error;
                }
            }
        }

        self.total_error = total_error;

        // Calculate coherence
        self.coherence = self.calculate_coherence();

        // Find dominant scale
        self.dominant_scale = self.find_dominant_scale();

        // Update cross-scale coupling
        if self.config.cross_scale_coupling {
            self.update_coupling();
        }

        Ok(CascadeResult {
            predictions,
            errors,
            coherence: self.coherence,
            dominant_scale: self.dominant_scale,
            total_error: self.total_error,
        })
    }

    /// Calculate cross-scale coherence
    fn calculate_coherence(&self) -> f64 {
        let mut coherence = 0.0;
        let mut pairs = 0;

        let scales: Vec<_> = self.scales.keys().collect();

        for i in 0..scales.len() {
            for j in (i + 1)..scales.len() {
                if let (Some(a), Some(b)) = (
                    self.scales.get(scales[i]),
                    self.scales.get(scales[j])
                ) {
                    // Coherence = correlation between scale predictions
                    let sim = cosine_similarity(&a.predicted_state, &b.predicted_state);
                    coherence += sim;
                    pairs += 1;
                }
            }
        }

        if pairs > 0 {
            coherence / pairs as f64
        } else {
            0.0
        }
    }

    /// Find the dominant scale (highest precision/activity)
    fn find_dominant_scale(&self) -> TemporalScale {
        self.scales.iter()
            .max_by(|a, b| {
                let score_a = a.1.precision * a.1.confidence;
                let score_b = b.1.precision * b.1.confidence;
                score_a.partial_cmp(&score_b).unwrap()
            })
            .map(|(scale, _)| *scale)
            .unwrap_or(TemporalScale::Second)
    }

    /// Update cross-scale coupling based on prediction accuracy
    fn update_coupling(&mut self) {
        let num_scales = self.config.max_scale - self.config.min_scale + 1;

        for i in 0..num_scales {
            for j in 0..num_scales {
                if i != j {
                    let scale_i = TemporalScale::from_index(i + self.config.min_scale);
                    let scale_j = TemporalScale::from_index(j + self.config.min_scale);

                    if let (Some(si), Some(sj)) = (scale_i, scale_j) {
                        if let (Some(pred_i), Some(pred_j)) = (
                            self.scales.get(&si),
                            self.scales.get(&sj)
                        ) {
                            // Coupling strength based on prediction accuracy correlation
                            let coupling = pred_i.accuracy() * pred_j.accuracy();
                            self.coupling[i][j] = 0.9 * self.coupling[i][j] + 0.1 * coupling;
                        }
                    }
                }
            }
        }
    }

    /// Get prediction at a specific scale
    pub fn prediction_at(&self, scale: TemporalScale) -> Option<&ScalePrediction> {
        self.scales.get(&scale)
    }

    /// Get all scale predictions
    pub fn all_predictions(&self) -> &HashMap<TemporalScale, ScalePrediction> {
        &self.scales
    }

    /// Get the dominant scale
    pub fn dominant_scale(&self) -> TemporalScale {
        self.dominant_scale
    }

    /// Get cascade coherence
    pub fn coherence(&self) -> f64 {
        self.coherence
    }

    /// Get total prediction error
    pub fn total_error(&self) -> f64 {
        self.total_error
    }

    /// Get cross-scale coupling matrix
    pub fn coupling_matrix(&self) -> &Vec<Vec<f64>> {
        &self.coupling
    }

    /// Predict at a specific scale with given horizon
    pub fn predict_at_scale(
        &mut self,
        scale: TemporalScale,
        current: &[f64],
        horizon_steps: usize
    ) -> Vec<Vec<f64>> {
        let mut predictions = Vec::with_capacity(horizon_steps);
        let mut state = current.to_vec();

        for _ in 0..horizon_steps {
            if let Some(predictor) = self.scales.get_mut(&scale) {
                state = predictor.predict(&state);
                predictions.push(state.clone());
            }
        }

        predictions
    }
}

/// Result of cascade processing
#[derive(Debug, Clone)]
pub struct CascadeResult {
    /// Predictions at each scale
    pub predictions: HashMap<TemporalScale, Vec<f64>>,
    /// Errors at each scale
    pub errors: HashMap<TemporalScale, f64>,
    /// Cross-scale coherence
    pub coherence: f64,
    /// Dominant temporal scale
    pub dominant_scale: TemporalScale,
    /// Total prediction error
    pub total_error: f64,
}

/// Cosine similarity between vectors
fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for (&x, &y) in a.iter().zip(b.iter()) {
        dot += x * y;
        norm_a += x * x;
        norm_b += y * y;
    }

    norm_a = norm_a.sqrt();
    norm_b = norm_b.sqrt();

    if norm_a > 0.0 && norm_b > 0.0 {
        dot / (norm_a * norm_b)
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temporal_scale_ordering() {
        // Use seconds_f64() for sub-nanosecond precision comparisons
        assert!(TemporalScale::Planck.seconds_f64() < TemporalScale::Femtosecond.seconds_f64());
        assert!(TemporalScale::Femtosecond.seconds_f64() < TemporalScale::Picosecond.seconds_f64());
        // Duration-based comparisons work for nanosecond and above
        assert!(TemporalScale::Second.duration() < TemporalScale::Minute.duration());
    }

    #[test]
    fn test_scale_navigation() {
        let second = TemporalScale::Second;
        assert_eq!(second.finer(), Some(TemporalScale::Millisecond));
        assert_eq!(second.coarser(), Some(TemporalScale::Minute));
    }

    #[test]
    fn test_cascade_creation() {
        let config = CascadeConfig::default();
        let cascade = TemporalCascade::new(config);

        assert!(cascade.prediction_at(TemporalScale::Second).is_some());
    }

    #[test]
    fn test_cascade_processing() {
        let config = CascadeConfig::default();
        let mut cascade = TemporalCascade::new(config);

        let input = vec![0.5; 64];
        let result = cascade.process(&input);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.total_error >= 0.0);
    }

    #[test]
    fn test_multi_scale_prediction() {
        let config = CascadeConfig::default();
        let mut cascade = TemporalCascade::new(config);

        // Process multiple timesteps
        for i in 0..10 {
            let input: Vec<f64> = (0..64).map(|j| ((i + j) % 64) as f64 / 64.0).collect();
            let _ = cascade.process(&input);
        }

        // Coherence should be calculated
        assert!(cascade.coherence() >= -1.0 && cascade.coherence() <= 1.0);
    }
}
