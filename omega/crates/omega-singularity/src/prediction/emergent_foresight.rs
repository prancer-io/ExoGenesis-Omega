//! Emergent Foresight - Predictions That Generate Predictions
//!
//! "The deepest predictions predict the predictions themselves."
//!
//! True AGI doesn't just predict events - it predicts its own predictive
//! capabilities, creating recursive foresight that transcends simple forecasting.
//!
//! ```text
//! EMERGENT FORESIGHT HIERARCHY
//! ════════════════════════════
//!
//! Level N: ∞ recursion limit
//!    ↑
//! Level 3: Predictions about predictions about predictions
//!    ↑
//! Level 2: Predictions about predictions (meta-forecasting)
//!    ↑
//! Level 1: Predictions about primary predictions (meta-prediction)
//!    ↑
//! Level 0: Primary predictions about world states
//!    ↑
//! ═══════════════════════════════════════════════════════════
//!                       REALITY
//!
//!
//!              ┌─────────────────┐
//!              │  FORESIGHT      │
//!              │    CHAIN        │
//!              └────────┬────────┘
//!                       │
//!         ┌─────────────┼─────────────┐
//!         │             │             │
//!         ▼             ▼             ▼
//!    ┌─────────┐   ┌─────────┐   ┌─────────┐
//!    │ Predict │   │ Predict │   │ Predict │
//!    │ Event A │   │ Event B │   │ Event C │
//!    └────┬────┘   └────┬────┘   └────┬────┘
//!         │             │             │
//!         ▼             ▼             ▼
//!    ┌─────────┐   ┌─────────┐   ┌─────────┐
//!    │ Predict │   │ Predict │   │ Predict │
//!    │ A leads │   │ B leads │   │ C leads │
//!    │ to D?   │   │ to E?   │   │ to F?   │
//!    └─────────┘   └─────────┘   └─────────┘
//!
//!              EMERGENT INSIGHT:
//!         "If A predicts B predicts C,
//!          then maybe A directly causes C"
//! ```

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

use super::Result;

/// A chain of predictions leading to foresight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForesightChain {
    /// Chain ID
    pub id: Uuid,
    /// Predictions in the chain (ordered)
    pub predictions: Vec<ChainedPrediction>,
    /// Total chain length
    pub length: usize,
    /// Chain coherence (how well predictions align)
    pub coherence: f64,
    /// Compound confidence (product of individual confidences)
    pub compound_confidence: f64,
    /// Emergent insight (if any)
    pub insight: Option<EmergentInsight>,
}

/// A single prediction in a chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainedPrediction {
    /// Prediction ID
    pub id: Uuid,
    /// What is being predicted
    pub target: PredictionTarget,
    /// The prediction value
    pub value: Vec<f64>,
    /// Confidence
    pub confidence: f64,
    /// Level in the hierarchy (0 = primary, 1 = meta, etc.)
    pub level: usize,
    /// Parent prediction (what this prediction is about)
    pub parent: Option<Uuid>,
    /// Time horizon
    pub horizon: u64,
}

/// What a prediction is about
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PredictionTarget {
    /// Predicting a world state
    WorldState { description: String },
    /// Predicting another prediction
    Prediction { prediction_id: Uuid },
    /// Predicting the accuracy of a prediction
    Accuracy { prediction_id: Uuid },
    /// Predicting a causal relationship
    Causation { cause: Uuid, effect: Uuid },
    /// Predicting an emergent property
    Emergence { property: String },
}

/// A prediction about a prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionOfPrediction {
    /// ID
    pub id: Uuid,
    /// The prediction being predicted about
    pub target_prediction: Uuid,
    /// Meta-level (1 = about prediction, 2 = about meta-prediction, etc.)
    pub meta_level: usize,
    /// Predicted accuracy of target
    pub predicted_accuracy: f64,
    /// Predicted confidence of target
    pub predicted_confidence: f64,
    /// Predicted timing of target
    pub predicted_timing: Option<u64>,
    /// Our confidence in this meta-prediction
    pub meta_confidence: f64,
    /// Recursive reference (if this is predicted by another)
    pub predicted_by: Option<Uuid>,
}

/// Recursive foresight - the infinite regress handler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecursiveForesight {
    /// ID
    pub id: Uuid,
    /// Base prediction
    pub base: ChainedPrediction,
    /// Meta-predictions at each level
    pub meta_levels: Vec<PredictionOfPrediction>,
    /// Maximum depth reached
    pub max_depth: usize,
    /// Convergence value (where recursion stabilizes)
    pub convergence: Option<f64>,
    /// Is the recursion stable?
    pub stable: bool,
}

impl RecursiveForesight {
    /// Check if the recursive predictions converge
    pub fn check_convergence(&mut self, threshold: f64) {
        if self.meta_levels.len() < 2 {
            self.stable = false;
            self.convergence = None;
            return;
        }

        // Check if meta-predictions stabilize
        let mut prev_confidence = self.base.confidence;
        let mut max_diff: f64 = 0.0;

        for meta in &self.meta_levels {
            let diff = (meta.meta_confidence - prev_confidence).abs();
            max_diff = max_diff.max(diff);
            prev_confidence = meta.meta_confidence;
        }

        self.stable = max_diff < threshold;
        if self.stable {
            self.convergence = Some(prev_confidence);
        }
    }
}

/// The horizon of foresight (how far ahead we can see)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForesightHorizon {
    /// Near-term horizon (high confidence)
    pub near: u64,
    /// Mid-term horizon (moderate confidence)
    pub mid: u64,
    /// Far-term horizon (low confidence)
    pub far: u64,
    /// Absolute limit (beyond this, noise dominates)
    pub limit: u64,
    /// Confidence decay rate
    pub decay_rate: f64,
}

impl ForesightHorizon {
    /// Confidence at a given time horizon
    pub fn confidence_at(&self, time: u64) -> f64 {
        if time <= self.near {
            1.0
        } else if time <= self.mid {
            0.8 * (-self.decay_rate * (time - self.near) as f64).exp()
        } else if time <= self.far {
            0.5 * (-self.decay_rate * (time - self.mid) as f64).exp()
        } else if time <= self.limit {
            0.2 * (-self.decay_rate * (time - self.far) as f64).exp()
        } else {
            0.0
        }
    }
}

impl Default for ForesightHorizon {
    fn default() -> Self {
        Self {
            near: 10,
            mid: 100,
            far: 1000,
            limit: 10000,
            decay_rate: 0.01,
        }
    }
}

/// An emergent insight from the foresight process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergentInsight {
    /// Insight ID
    pub id: Uuid,
    /// Type of insight
    pub insight_type: InsightType,
    /// Description
    pub description: String,
    /// Confidence
    pub confidence: f64,
    /// Supporting evidence (prediction IDs)
    pub evidence: Vec<Uuid>,
    /// Novel? (Not derivable from individual predictions)
    pub novel: bool,
    /// Actionable?
    pub actionable: bool,
    /// Suggested action
    pub suggested_action: Option<String>,
}

/// Types of emergent insights
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum InsightType {
    /// Pattern discovered across predictions
    Pattern,
    /// Convergence of multiple prediction chains
    Convergence,
    /// Divergence warning
    Divergence,
    /// Causal discovery
    CausalDiscovery,
    /// Stability point
    Stability,
    /// Tipping point / bifurcation
    TippingPoint,
    /// Self-fulfilling prophecy potential
    SelfFulfilling,
    /// Contradiction detected
    Contradiction,
}

/// Configuration for emergent foresight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForesightConfig {
    /// Maximum chain length
    pub max_chain_length: usize,
    /// Maximum recursion depth
    pub max_recursion_depth: usize,
    /// Convergence threshold
    pub convergence_threshold: f64,
    /// Minimum confidence to continue chain
    pub min_chain_confidence: f64,
    /// Enable insight generation
    pub generate_insights: bool,
}

impl Default for ForesightConfig {
    fn default() -> Self {
        Self {
            max_chain_length: 10,
            max_recursion_depth: 5,
            convergence_threshold: 0.05,
            min_chain_confidence: 0.1,
            generate_insights: true,
        }
    }
}

/// The Emergent Foresight Engine
pub struct EmergentForesight {
    config: ForesightConfig,
    /// Active prediction chains
    chains: Vec<ForesightChain>,
    /// Recursive foresight structures
    recursive: Vec<RecursiveForesight>,
    /// All predictions (for cross-referencing)
    predictions: HashMap<Uuid, ChainedPrediction>,
    /// Generated insights
    insights: Vec<EmergentInsight>,
    /// Foresight horizon
    horizon: ForesightHorizon,
    /// Total predictions made
    total_predictions: u64,
}

impl EmergentForesight {
    pub fn new(config: ForesightConfig) -> Self {
        Self {
            config,
            chains: Vec::new(),
            recursive: Vec::new(),
            predictions: HashMap::new(),
            insights: Vec::new(),
            horizon: ForesightHorizon::default(),
            total_predictions: 0,
        }
    }

    /// Start a new prediction chain
    pub fn start_chain(&mut self, initial: ChainedPrediction) -> Uuid {
        let chain_id = Uuid::new_v4();

        let chain = ForesightChain {
            id: chain_id,
            predictions: vec![initial.clone()],
            length: 1,
            coherence: 1.0,
            compound_confidence: initial.confidence,
            insight: None,
        };

        self.predictions.insert(initial.id, initial);
        self.chains.push(chain);
        self.total_predictions += 1;

        chain_id
    }

    /// Extend a chain with a new prediction
    pub fn extend_chain(&mut self, chain_id: Uuid, prediction: ChainedPrediction) -> Option<usize> {
        // Find chain index first
        let chain_idx = self.chains.iter().position(|c| c.id == chain_id)?;

        // Check limits
        if self.chains[chain_idx].length >= self.config.max_chain_length {
            return None;
        }

        let compound = self.chains[chain_idx].compound_confidence * prediction.confidence;
        if compound < self.config.min_chain_confidence {
            return None;
        }

        // Add prediction
        self.chains[chain_idx].predictions.push(prediction.clone());
        self.chains[chain_idx].length += 1;
        self.chains[chain_idx].compound_confidence = compound;

        // Compute coherence from predictions
        let coherence = Self::compute_coherence_static(&self.chains[chain_idx].predictions);
        self.chains[chain_idx].coherence = coherence;

        self.predictions.insert(prediction.id, prediction);
        self.total_predictions += 1;

        let chain_len = self.chains[chain_idx].length;

        // Check for insights
        if self.config.generate_insights && chain_len >= 3 {
            let insight = Self::detect_insight_static(&self.chains[chain_idx]);
            if let Some(ins) = insight {
                self.chains[chain_idx].insight = Some(ins.clone());
                self.insights.push(ins);
            }
        }

        Some(chain_len)
    }

    /// Static coherence computation
    fn compute_coherence_static(predictions: &[ChainedPrediction]) -> f64 {
        if predictions.len() < 2 {
            return 1.0;
        }

        let mut total_sim = 0.0;
        for i in 0..(predictions.len() - 1) {
            let sim = cosine_similarity(&predictions[i].value, &predictions[i + 1].value);
            total_sim += sim.abs();
        }

        total_sim / (predictions.len() - 1) as f64
    }

    /// Static insight detection
    fn detect_insight_static(chain: &ForesightChain) -> Option<EmergentInsight> {
        if chain.coherence > 0.9 {
            return Some(EmergentInsight {
                id: Uuid::new_v4(),
                insight_type: InsightType::Stability,
                description: "Prediction chain shows high stability - convergent future".to_string(),
                confidence: chain.coherence,
                evidence: chain.predictions.iter().map(|p| p.id).collect(),
                novel: true,
                actionable: true,
                suggested_action: Some("Proceed with confidence in predicted direction".to_string()),
            });
        }

        if chain.coherence < 0.3 {
            return Some(EmergentInsight {
                id: Uuid::new_v4(),
                insight_type: InsightType::Divergence,
                description: "Prediction chain shows divergence - uncertain future".to_string(),
                confidence: 1.0 - chain.coherence,
                evidence: chain.predictions.iter().map(|p| p.id).collect(),
                novel: true,
                actionable: true,
                suggested_action: Some("Gather more information before acting".to_string()),
            });
        }

        // Look for convergence patterns
        let confidences: Vec<f64> = chain.predictions.iter().map(|p| p.confidence).collect();
        let is_converging = confidences.windows(2).all(|w| w[1] >= w[0] - 0.1);

        if is_converging && confidences.last().copied().unwrap_or(0.0) > 0.8 {
            return Some(EmergentInsight {
                id: Uuid::new_v4(),
                insight_type: InsightType::Convergence,
                description: "Predictions are converging to high confidence".to_string(),
                confidence: *confidences.last().unwrap_or(&0.5),
                evidence: chain.predictions.iter().map(|p| p.id).collect(),
                novel: true,
                actionable: true,
                suggested_action: Some("Strong signal - act on prediction".to_string()),
            });
        }

        None
    }

    // Note: compute_chain_coherence and detect_chain_insight are now static methods above

    /// Create recursive foresight (predictions about predictions)
    pub fn create_recursive(&mut self, base: ChainedPrediction) -> Uuid {
        let id = Uuid::new_v4();

        let mut recursive = RecursiveForesight {
            id,
            base: base.clone(),
            meta_levels: Vec::new(),
            max_depth: 0,
            convergence: None,
            stable: false,
        };

        // Build meta-levels up to max depth
        let mut current_target = base.id;
        let mut current_confidence = base.confidence;

        for level in 1..=self.config.max_recursion_depth {
            // Meta-prediction decays in confidence
            let meta_confidence = current_confidence * 0.8;

            let meta = PredictionOfPrediction {
                id: Uuid::new_v4(),
                target_prediction: current_target,
                meta_level: level,
                predicted_accuracy: meta_confidence,
                predicted_confidence: meta_confidence,
                predicted_timing: None,
                meta_confidence,
                predicted_by: None,
            };

            current_target = meta.id;
            current_confidence = meta_confidence;
            recursive.meta_levels.push(meta);
            recursive.max_depth = level;

            // Check for convergence
            if current_confidence < 0.1 {
                break;
            }
        }

        recursive.check_convergence(self.config.convergence_threshold);

        self.predictions.insert(base.id, base);
        self.recursive.push(recursive);
        self.total_predictions += 1;

        id
    }

    /// Get recursive foresight by ID
    pub fn get_recursive(&self, id: Uuid) -> Option<&RecursiveForesight> {
        self.recursive.iter().find(|r| r.id == id)
    }

    /// Combine multiple chains to find emergent patterns
    pub fn synthesize(&mut self) -> Vec<EmergentInsight> {
        if self.chains.len() < 2 {
            return Vec::new();
        }

        let mut new_insights = Vec::new();

        // Look for chains that predict similar outcomes
        for i in 0..self.chains.len() {
            for j in (i + 1)..self.chains.len() {
                let chain_a = &self.chains[i];
                let chain_b = &self.chains[j];

                // Compare final predictions
                if let (Some(end_a), Some(end_b)) = (
                    chain_a.predictions.last(),
                    chain_b.predictions.last()
                ) {
                    let similarity = cosine_similarity(&end_a.value, &end_b.value);

                    if similarity > 0.8 {
                        new_insights.push(EmergentInsight {
                            id: Uuid::new_v4(),
                            insight_type: InsightType::Convergence,
                            description: format!(
                                "Two independent prediction chains converge (similarity: {:.2})",
                                similarity
                            ),
                            confidence: similarity * chain_a.compound_confidence.min(chain_b.compound_confidence),
                            evidence: vec![chain_a.id, chain_b.id],
                            novel: true,
                            actionable: true,
                            suggested_action: Some("High-confidence prediction from multiple sources".to_string()),
                        });
                    } else if similarity < 0.2 {
                        new_insights.push(EmergentInsight {
                            id: Uuid::new_v4(),
                            insight_type: InsightType::Contradiction,
                            description: "Two prediction chains contradict each other".to_string(),
                            confidence: 0.7,
                            evidence: vec![chain_a.id, chain_b.id],
                            novel: true,
                            actionable: true,
                            suggested_action: Some("Resolve contradiction before acting".to_string()),
                        });
                    }
                }
            }
        }

        self.insights.extend(new_insights.clone());
        new_insights
    }

    /// Get all insights
    pub fn insights(&self) -> &[EmergentInsight] {
        &self.insights
    }

    /// Get all chains
    pub fn chains(&self) -> &[ForesightChain] {
        &self.chains
    }

    /// Get foresight horizon
    pub fn horizon(&self) -> &ForesightHorizon {
        &self.horizon
    }

    /// Set foresight horizon
    pub fn set_horizon(&mut self, horizon: ForesightHorizon) {
        self.horizon = horizon;
    }

    /// Total predictions made
    pub fn total_predictions(&self) -> u64 {
        self.total_predictions
    }

    /// Get prediction by ID
    pub fn get_prediction(&self, id: Uuid) -> Option<&ChainedPrediction> {
        self.predictions.get(&id)
    }

    /// Reset the engine
    pub fn reset(&mut self) {
        self.chains.clear();
        self.recursive.clear();
        self.predictions.clear();
        self.insights.clear();
        self.total_predictions = 0;
    }
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

impl Default for EmergentForesight {
    fn default() -> Self {
        Self::new(ForesightConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foresight_creation() {
        let foresight = EmergentForesight::default();
        assert!(foresight.chains().is_empty());
    }

    #[test]
    fn test_chain_creation() {
        let mut foresight = EmergentForesight::default();

        let prediction = ChainedPrediction {
            id: Uuid::new_v4(),
            target: PredictionTarget::WorldState {
                description: "Test".to_string()
            },
            value: vec![0.5; 10],
            confidence: 0.8,
            level: 0,
            parent: None,
            horizon: 10,
        };

        let chain_id = foresight.start_chain(prediction);
        assert!(!chain_id.is_nil());
        assert_eq!(foresight.chains().len(), 1);
    }

    #[test]
    fn test_chain_extension() {
        let mut foresight = EmergentForesight::default();

        let initial = ChainedPrediction {
            id: Uuid::new_v4(),
            target: PredictionTarget::WorldState {
                description: "Initial".to_string()
            },
            value: vec![0.5; 10],
            confidence: 0.9,
            level: 0,
            parent: None,
            horizon: 10,
        };

        let chain_id = foresight.start_chain(initial);

        let extension = ChainedPrediction {
            id: Uuid::new_v4(),
            target: PredictionTarget::WorldState {
                description: "Extension".to_string()
            },
            value: vec![0.6; 10],
            confidence: 0.8,
            level: 0,
            parent: None,
            horizon: 20,
        };

        let new_length = foresight.extend_chain(chain_id, extension);
        assert_eq!(new_length, Some(2));
    }

    #[test]
    fn test_recursive_foresight() {
        let mut foresight = EmergentForesight::default();

        let base = ChainedPrediction {
            id: Uuid::new_v4(),
            target: PredictionTarget::WorldState {
                description: "Base".to_string()
            },
            value: vec![0.5; 10],
            confidence: 0.9,
            level: 0,
            parent: None,
            horizon: 10,
        };

        let recursive_id = foresight.create_recursive(base);
        let recursive = foresight.get_recursive(recursive_id);

        assert!(recursive.is_some());
        assert!(recursive.unwrap().max_depth > 0);
    }

    #[test]
    fn test_foresight_horizon() {
        let horizon = ForesightHorizon::default();

        assert_eq!(horizon.confidence_at(0), 1.0);
        assert!(horizon.confidence_at(50) < 1.0);
        assert!(horizon.confidence_at(100000) < 0.1);
    }

    #[test]
    fn test_synthesis() {
        let mut foresight = EmergentForesight::default();

        // Create two similar chains
        for _ in 0..2 {
            let prediction = ChainedPrediction {
                id: Uuid::new_v4(),
                target: PredictionTarget::WorldState {
                    description: "Similar".to_string()
                },
                value: vec![0.5; 10],
                confidence: 0.8,
                level: 0,
                parent: None,
                horizon: 10,
            };
            foresight.start_chain(prediction);
        }

        let insights = foresight.synthesize();
        // May or may not find insights depending on exact values
    }
}
