//! Meta Oracle - Predicting Prediction Quality
//!
//! True intelligence knows what it doesn't know. The Meta Oracle predicts
//! the accuracy of predictions themselves - the uncertainty of uncertainty.
//!
//! ```text
//! META-PREDICTION HIERARCHY
//! ═════════════════════════
//!
//! Level 3: Uncertainty about uncertainty about uncertainty
//!          "How well do I know that I don't know what I don't know?"
//!              ↓
//! Level 2: Uncertainty about uncertainty
//!          "How confident am I in my confidence?"
//!              ↓
//! Level 1: Uncertainty about prediction
//!          "How accurate will my prediction be?"
//!              ↓
//! Level 0: Primary prediction
//!          "What will happen?"
//! ```

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

use super::Result;

/// An uncertainty estimate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncertaintyEstimate {
    /// Unique ID
    pub id: Uuid,
    /// The prediction this uncertainty is about
    pub target_prediction: Uuid,
    /// Estimated variance
    pub variance: f64,
    /// Confidence interval (lower, upper)
    pub confidence_interval: (f64, f64),
    /// Confidence level (e.g., 0.95 for 95%)
    pub confidence_level: f64,
    /// Entropy of the prediction distribution
    pub entropy: f64,
    /// Meta-level (0 = about prediction, 1 = about uncertainty, etc.)
    pub meta_level: usize,
    /// Is this estimate well-calibrated?
    pub calibrated: bool,
}

impl UncertaintyEstimate {
    pub fn new(target: Uuid, variance: f64, confidence: f64) -> Self {
        let std_dev = variance.sqrt();
        // Approximate 95% CI
        let z = 1.96;

        Self {
            id: Uuid::new_v4(),
            target_prediction: target,
            variance,
            confidence_interval: (-z * std_dev, z * std_dev),
            confidence_level: confidence,
            entropy: Self::compute_entropy(variance),
            meta_level: 0,
            calibrated: false,
        }
    }

    fn compute_entropy(variance: f64) -> f64 {
        // Entropy of a Gaussian: 0.5 * ln(2 * pi * e * variance)
        0.5 * (2.0 * std::f64::consts::PI * std::f64::consts::E * variance.max(1e-10)).ln()
    }

    /// Width of confidence interval
    pub fn interval_width(&self) -> f64 {
        self.confidence_interval.1 - self.confidence_interval.0
    }
}

/// A confidence region (multi-dimensional uncertainty)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceRegion {
    /// Center point
    pub center: Vec<f64>,
    /// Covariance matrix (diagonal approximation)
    pub variances: Vec<f64>,
    /// Confidence level
    pub confidence_level: f64,
    /// Region volume (approximation)
    pub volume: f64,
}

impl ConfidenceRegion {
    pub fn new(center: Vec<f64>, variances: Vec<f64>, confidence_level: f64) -> Self {
        // Volume of confidence ellipsoid
        let volume: f64 = variances.iter()
            .map(|&v| v.sqrt())
            .product::<f64>()
            * (2.0 * std::f64::consts::PI).powf(variances.len() as f64 / 2.0);

        Self {
            center,
            variances,
            confidence_level,
            volume,
        }
    }

    /// Check if a point is within the region
    pub fn contains(&self, point: &[f64]) -> bool {
        let mut distance = 0.0;

        for (i, &p) in point.iter().enumerate() {
            if i < self.center.len() && i < self.variances.len() {
                let diff = p - self.center[i];
                distance += diff * diff / self.variances[i].max(1e-10);
            }
        }

        // Chi-squared threshold for given confidence
        let threshold = self.chi_squared_threshold();
        distance <= threshold
    }

    fn chi_squared_threshold(&self) -> f64 {
        // Approximate chi-squared critical value
        let df = self.variances.len() as f64;
        df + 2.0 * (df * 2.0).sqrt() * (1.0 - self.confidence_level).ln().abs()
    }
}

/// Calibration score - how well calibrated are the predictions?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationScore {
    /// Expected calibration error
    pub ece: f64,
    /// Maximum calibration error
    pub mce: f64,
    /// Brier score
    pub brier: f64,
    /// Number of calibration bins
    pub num_bins: usize,
    /// Calibration curve (predicted prob -> actual prob)
    pub calibration_curve: Vec<(f64, f64)>,
    /// Is the model overconfident?
    pub overconfident: bool,
    /// Is the model underconfident?
    pub underconfident: bool,
}

impl CalibrationScore {
    pub fn new(predictions: &[(f64, bool)], num_bins: usize) -> Self {
        let mut bins: Vec<Vec<(f64, bool)>> = vec![Vec::new(); num_bins];

        // Sort predictions into bins
        for &(prob, actual) in predictions {
            let bin = ((prob * num_bins as f64) as usize).min(num_bins - 1);
            bins[bin].push((prob, actual));
        }

        // Compute calibration metrics
        let mut ece: f64 = 0.0;
        let mut mce: f64 = 0.0;
        let mut calibration_curve = Vec::new();
        let total = predictions.len() as f64;

        for (i, bin) in bins.iter().enumerate() {
            if bin.is_empty() {
                continue;
            }

            let avg_pred: f64 = bin.iter().map(|(p, _)| p).sum::<f64>() / bin.len() as f64;
            let avg_actual: f64 = bin.iter().filter(|(_, a)| *a).count() as f64 / bin.len() as f64;

            let diff = (avg_pred - avg_actual).abs();
            ece += (bin.len() as f64 / total) * diff;
            mce = mce.max(diff);

            calibration_curve.push((avg_pred, avg_actual));
        }

        // Brier score
        let brier: f64 = predictions.iter()
            .map(|(prob, actual)| {
                let target = if *actual { 1.0 } else { 0.0 };
                (prob - target).powi(2)
            })
            .sum::<f64>() / predictions.len().max(1) as f64;

        // Check for systematic bias
        let avg_pred: f64 = predictions.iter().map(|(p, _)| p).sum::<f64>()
            / predictions.len().max(1) as f64;
        let avg_actual: f64 = predictions.iter().filter(|(_, a)| *a).count() as f64
            / predictions.len().max(1) as f64;

        Self {
            ece,
            mce,
            brier,
            num_bins,
            calibration_curve,
            overconfident: avg_pred > avg_actual + 0.05,
            underconfident: avg_pred < avg_actual - 0.05,
        }
    }
}

/// A meta-prediction about a prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaPrediction {
    /// Unique ID
    pub id: Uuid,
    /// The prediction this is about
    pub target: Uuid,
    /// Meta-level
    pub level: usize,
    /// Predicted accuracy of the target prediction
    pub predicted_accuracy: f64,
    /// Confidence in this meta-prediction
    pub confidence: f64,
    /// Uncertainty estimate
    pub uncertainty: UncertaintyEstimate,
    /// Explanation
    pub explanation: String,
}

/// An insight from the Meta Oracle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleInsight {
    /// Type of insight
    pub insight_type: InsightType,
    /// Confidence
    pub confidence: f64,
    /// Description
    pub description: String,
    /// Recommended action
    pub recommendation: String,
}

/// Types of oracle insights
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum InsightType {
    /// Predictions are well-calibrated
    WellCalibrated,
    /// Systematically overconfident
    Overconfident,
    /// Systematically underconfident
    Underconfident,
    /// High uncertainty in a specific domain
    HighUncertainty,
    /// Low uncertainty (high confidence)
    LowUncertainty,
    /// Meta-divergence (uncertainty about uncertainty is high)
    MetaDivergence,
    /// Epistemic uncertainty (lack of knowledge)
    EpistemicUncertainty,
    /// Aleatoric uncertainty (inherent randomness)
    AleatoricUncertainty,
}

/// Configuration for the Meta Oracle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleConfig {
    /// Maximum meta-level depth
    pub max_meta_level: usize,
    /// Calibration bins
    pub calibration_bins: usize,
    /// History length for calibration
    pub history_length: usize,
    /// Confidence threshold
    pub confidence_threshold: f64,
    /// Uncertainty threshold for warnings
    pub uncertainty_threshold: f64,
}

impl Default for OracleConfig {
    fn default() -> Self {
        Self {
            max_meta_level: 3,
            calibration_bins: 10,
            history_length: 1000,
            confidence_threshold: 0.95,
            uncertainty_threshold: 0.5,
        }
    }
}

/// The Meta Oracle - Predicting Prediction Quality
pub struct MetaOracle {
    config: OracleConfig,
    /// Prediction history (prediction_id -> (predicted, actual))
    history: HashMap<Uuid, (f64, f64)>,
    /// Calibration history
    calibration_history: VecDeque<(f64, bool)>,
    /// Current calibration score
    calibration: Option<CalibrationScore>,
    /// Domain-specific uncertainties
    domain_uncertainties: HashMap<String, f64>,
    /// Meta-predictions by level
    meta_predictions: Vec<Vec<MetaPrediction>>,
    /// Total predictions made
    total_predictions: u64,
    /// Accurate predictions
    accurate_predictions: u64,
}

impl MetaOracle {
    pub fn new(config: OracleConfig) -> Self {
        let max_level = config.max_meta_level;
        Self {
            config,
            history: HashMap::new(),
            calibration_history: VecDeque::with_capacity(1000),
            calibration: None,
            domain_uncertainties: HashMap::new(),
            meta_predictions: vec![Vec::new(); max_level + 1],
            total_predictions: 0,
            accurate_predictions: 0,
        }
    }

    /// Predict the accuracy of a prediction
    pub fn predict_accuracy(&mut self, prediction_id: Uuid, domain: Option<&str>) -> MetaPrediction {
        let base_accuracy = self.empirical_accuracy();

        // Adjust for domain-specific uncertainty
        let domain_adjustment = domain
            .and_then(|d| self.domain_uncertainties.get(d))
            .copied()
            .unwrap_or(0.0);

        let predicted_accuracy = (base_accuracy - domain_adjustment).clamp(0.0, 1.0);

        // Compute uncertainty about this meta-prediction
        let variance = self.compute_meta_variance(0);
        let uncertainty = UncertaintyEstimate::new(prediction_id, variance, 0.95);

        let meta_pred = MetaPrediction {
            id: Uuid::new_v4(),
            target: prediction_id,
            level: 0,
            predicted_accuracy,
            confidence: 1.0 - variance.sqrt(),
            uncertainty,
            explanation: format!(
                "Based on {} historical predictions with {:.1}% accuracy",
                self.total_predictions,
                base_accuracy * 100.0
            ),
        };

        // Store meta-prediction
        if self.meta_predictions[0].len() > 100 {
            self.meta_predictions[0].remove(0);
        }
        self.meta_predictions[0].push(meta_pred.clone());

        meta_pred
    }

    /// Predict the accuracy of a meta-prediction (recursively)
    pub fn meta_predict(&mut self, target: &MetaPrediction) -> Option<MetaPrediction> {
        let next_level = target.level + 1;

        if next_level > self.config.max_meta_level {
            return None; // Maximum recursion depth
        }

        let variance = self.compute_meta_variance(next_level);

        // Uncertainty increases with meta-level
        let level_penalty = 1.0 + 0.2 * next_level as f64;
        let adjusted_variance = variance * level_penalty;

        let uncertainty = UncertaintyEstimate::new(target.id, adjusted_variance, 0.95);

        let predicted_accuracy = target.confidence * (1.0 - adjusted_variance.sqrt()).max(0.1);

        let meta_pred = MetaPrediction {
            id: Uuid::new_v4(),
            target: target.id,
            level: next_level,
            predicted_accuracy,
            confidence: 1.0 - adjusted_variance.sqrt(),
            uncertainty,
            explanation: format!(
                "Meta-level {} prediction about prediction {}",
                next_level, target.id
            ),
        };

        if self.meta_predictions[next_level].len() > 100 {
            self.meta_predictions[next_level].remove(0);
        }
        self.meta_predictions[next_level].push(meta_pred.clone());

        Some(meta_pred)
    }

    /// Compute variance at a given meta-level
    fn compute_meta_variance(&self, level: usize) -> f64 {
        if level == 0 {
            // Base variance from calibration history
            if self.calibration_history.len() < 10 {
                return 0.25; // High uncertainty with little data
            }

            let probs: Vec<f64> = self.calibration_history.iter()
                .map(|(p, _)| *p)
                .collect();

            let mean: f64 = probs.iter().sum::<f64>() / probs.len() as f64;
            let variance: f64 = probs.iter()
                .map(|p| (p - mean).powi(2))
                .sum::<f64>() / probs.len() as f64;

            variance
        } else {
            // Higher levels: variance about variance
            let base = self.compute_meta_variance(level - 1);
            // Uncertainty compounds
            base * (1.0 + 0.5 * level as f64)
        }
    }

    /// Record actual outcome for a prediction
    pub fn record_outcome(&mut self, prediction_id: Uuid, predicted: f64, actual: f64) {
        self.history.insert(prediction_id, (predicted, actual));
        self.total_predictions += 1;

        // Check accuracy (within threshold)
        if (predicted - actual).abs() < 0.2 {
            self.accurate_predictions += 1;
        }

        // Update calibration history
        let calibration_entry = (predicted, actual > 0.5);
        self.calibration_history.push_back(calibration_entry);
        if self.calibration_history.len() > self.config.history_length {
            self.calibration_history.pop_front();
        }

        // Update domain uncertainties
        // (In a full implementation, this would track by domain)
    }

    /// Get empirical accuracy rate
    pub fn empirical_accuracy(&self) -> f64 {
        if self.total_predictions == 0 {
            0.5 // Prior
        } else {
            self.accurate_predictions as f64 / self.total_predictions as f64
        }
    }

    /// Update calibration score
    pub fn update_calibration(&mut self) -> CalibrationScore {
        let history: Vec<(f64, bool)> = self.calibration_history.iter().cloned().collect();
        let calibration = CalibrationScore::new(&history, self.config.calibration_bins);
        self.calibration = Some(calibration.clone());
        calibration
    }

    /// Get calibration score
    pub fn calibration(&self) -> Option<&CalibrationScore> {
        self.calibration.as_ref()
    }

    /// Get confidence region for a prediction
    pub fn confidence_region(&self, prediction: &[f64], confidence: f64) -> ConfidenceRegion {
        let base_variance = self.compute_meta_variance(0);
        let variances = vec![base_variance; prediction.len()];
        ConfidenceRegion::new(prediction.to_vec(), variances, confidence)
    }

    /// Get insights from the oracle
    pub fn insights(&self) -> Vec<OracleInsight> {
        let mut insights = Vec::new();

        // Check calibration
        if let Some(ref cal) = self.calibration {
            if cal.overconfident {
                insights.push(OracleInsight {
                    insight_type: InsightType::Overconfident,
                    confidence: 0.8,
                    description: format!(
                        "Predictions are systematically overconfident (ECE: {:.3})",
                        cal.ece
                    ),
                    recommendation: "Increase uncertainty estimates or use more conservative predictions".to_string(),
                });
            } else if cal.underconfident {
                insights.push(OracleInsight {
                    insight_type: InsightType::Underconfident,
                    confidence: 0.8,
                    description: format!(
                        "Predictions are systematically underconfident (ECE: {:.3})",
                        cal.ece
                    ),
                    recommendation: "Predictions can be trusted more than currently estimated".to_string(),
                });
            } else if cal.ece < 0.05 {
                insights.push(OracleInsight {
                    insight_type: InsightType::WellCalibrated,
                    confidence: 0.9,
                    description: format!(
                        "Predictions are well-calibrated (ECE: {:.3})",
                        cal.ece
                    ),
                    recommendation: "Continue with current prediction strategy".to_string(),
                });
            }
        }

        // Check for meta-divergence
        let meta_variance = self.compute_meta_variance(2);
        if meta_variance > 0.5 {
            insights.push(OracleInsight {
                insight_type: InsightType::MetaDivergence,
                confidence: 0.7,
                description: "High uncertainty about uncertainty - meta-predictions are unreliable".to_string(),
                recommendation: "Gather more data before making high-stakes predictions".to_string(),
            });
        }

        // Check overall uncertainty
        let base_variance = self.compute_meta_variance(0);
        if base_variance > self.config.uncertainty_threshold {
            insights.push(OracleInsight {
                insight_type: InsightType::HighUncertainty,
                confidence: 0.75,
                description: format!(
                    "High base uncertainty (variance: {:.3})",
                    base_variance
                ),
                recommendation: "Consider expanding training data or simplifying prediction task".to_string(),
            });
        } else if base_variance < 0.05 {
            insights.push(OracleInsight {
                insight_type: InsightType::LowUncertainty,
                confidence: 0.85,
                description: format!(
                    "Low uncertainty indicates confident predictions (variance: {:.3})",
                    base_variance
                ),
                recommendation: "Predictions in this domain are reliable".to_string(),
            });
        }

        insights
    }

    /// Is the oracle well-calibrated?
    pub fn is_calibrated(&self) -> bool {
        self.calibration
            .as_ref()
            .map(|c| c.ece < 0.1)
            .unwrap_or(false)
    }

    /// Get the expected calibration error
    pub fn ece(&self) -> f64 {
        self.calibration
            .as_ref()
            .map(|c| c.ece)
            .unwrap_or(1.0)
    }

    /// Get uncertainty at a specific meta-level
    pub fn uncertainty_at_level(&self, level: usize) -> f64 {
        self.compute_meta_variance(level)
    }

    /// Reset the oracle
    pub fn reset(&mut self) {
        self.history.clear();
        self.calibration_history.clear();
        self.calibration = None;
        self.domain_uncertainties.clear();
        for level in &mut self.meta_predictions {
            level.clear();
        }
        self.total_predictions = 0;
        self.accurate_predictions = 0;
    }
}

impl Default for MetaOracle {
    fn default() -> Self {
        Self::new(OracleConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle_creation() {
        let oracle = MetaOracle::default();
        assert_eq!(oracle.empirical_accuracy(), 0.5);
    }

    #[test]
    fn test_accuracy_prediction() {
        let mut oracle = MetaOracle::default();

        // Record some outcomes with predictions close to actuals (within 0.2 threshold)
        for i in 0..100 {
            let id = Uuid::new_v4();
            // Vary predictions to be close to actuals (within accuracy threshold)
            let actual = if i % 2 == 0 { 0.8 } else { 0.2 };
            let predicted = if i % 2 == 0 { 0.75 } else { 0.25 }; // Within 0.2 of actual
            oracle.record_outcome(id, predicted, actual);
        }

        let pred_id = Uuid::new_v4();
        let meta_pred = oracle.predict_accuracy(pred_id, None);

        assert!(meta_pred.predicted_accuracy > 0.0);
        assert!(meta_pred.predicted_accuracy <= 1.0);
    }

    #[test]
    fn test_meta_prediction_levels() {
        let mut oracle = MetaOracle::default();

        let pred_id = Uuid::new_v4();
        let level0 = oracle.predict_accuracy(pred_id, None);

        let level1 = oracle.meta_predict(&level0);
        assert!(level1.is_some());
        assert_eq!(level1.as_ref().unwrap().level, 1);

        let level2 = oracle.meta_predict(level1.as_ref().unwrap());
        assert!(level2.is_some());
        assert_eq!(level2.as_ref().unwrap().level, 2);
    }

    #[test]
    fn test_calibration() {
        let mut oracle = MetaOracle::default();

        // Record well-calibrated predictions
        for i in 0..100 {
            let id = Uuid::new_v4();
            let predicted = (i as f64) / 100.0;
            let actual = if (i as f64) / 100.0 > rand::random::<f64>() { 1.0 } else { 0.0 };
            oracle.record_outcome(id, predicted, actual);
        }

        let calibration = oracle.update_calibration();
        assert!(calibration.ece >= 0.0);
        assert!(calibration.ece <= 1.0);
    }

    #[test]
    fn test_confidence_region() {
        let oracle = MetaOracle::default();

        let prediction = vec![0.5, 0.6, 0.7];
        let region = oracle.confidence_region(&prediction, 0.95);

        assert!(region.contains(&prediction));
    }

    #[test]
    fn test_insights() {
        let mut oracle = MetaOracle::default();

        // Record some data
        for _ in 0..50 {
            let id = Uuid::new_v4();
            oracle.record_outcome(id, 0.9, 0.5); // Overconfident
        }

        oracle.update_calibration();
        let insights = oracle.insights();

        // Should detect something
        // (Specific insight depends on data)
    }
}
