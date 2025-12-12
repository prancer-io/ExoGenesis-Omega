//! Multi-objective fitness evaluation for architectures

use serde::{Deserialize, Serialize};
use thiserror::Error;
use omega_core::{Architecture, FitnessScore};
use super::benchmarks::BenchmarkSuite;

#[derive(Error, Debug)]
pub enum EvaluationError {
    #[error("Evaluation failed: {0}")]
    EvaluationFailed(String),

    #[error("Invalid architecture: {0}")]
    InvalidArchitecture(String),
}

/// Weight for each fitness component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricWeight {
    pub name: String,
    pub weight: f64,
}

/// Evaluates architecture fitness across multiple objectives
pub struct FitnessEvaluator {
    pub metrics: Vec<MetricWeight>,
    capability_weight: f64,
    efficiency_weight: f64,
    alignment_weight: f64,
    novelty_weight: f64,
    benchmark_suite: BenchmarkSuite,
}

impl FitnessEvaluator {
    pub fn new() -> Self {
        Self {
            metrics: vec![
                MetricWeight { name: "capability".to_string(), weight: 0.4 },
                MetricWeight { name: "efficiency".to_string(), weight: 0.2 },
                MetricWeight { name: "alignment".to_string(), weight: 0.3 },
                MetricWeight { name: "novelty".to_string(), weight: 0.1 },
            ],
            capability_weight: 0.4,
            efficiency_weight: 0.2,
            alignment_weight: 0.3,
            novelty_weight: 0.1,
            benchmark_suite: BenchmarkSuite::new(),
        }
    }

    pub fn with_weights(
        capability: f64,
        efficiency: f64,
        alignment: f64,
        novelty: f64,
    ) -> Self {
        Self {
            metrics: vec![
                MetricWeight { name: "capability".to_string(), weight: capability },
                MetricWeight { name: "efficiency".to_string(), weight: efficiency },
                MetricWeight { name: "alignment".to_string(), weight: alignment },
                MetricWeight { name: "novelty".to_string(), weight: novelty },
            ],
            capability_weight: capability,
            efficiency_weight: efficiency,
            alignment_weight: alignment,
            novelty_weight: novelty,
            benchmark_suite: BenchmarkSuite::with_weights(capability, efficiency, alignment, novelty),
        }
    }

    /// Evaluate architecture fitness across all dimensions
    pub async fn evaluate(
        &self,
        architecture: &Architecture,
    ) -> Result<FitnessScore, EvaluationError> {
        // Run the comprehensive benchmark suite
        let suite_result = self.benchmark_suite.run().await
            .map_err(|e| EvaluationError::EvaluationFailed(e.to_string()))?;

        // Extract scores from benchmark results, combining with architecture-specific evaluation
        let capability = (suite_result.capability + self.evaluate_capability(architecture).await?) / 2.0;
        let efficiency = (suite_result.efficiency + self.evaluate_efficiency(architecture).await?) / 2.0;
        let alignment = (suite_result.alignment + self.evaluate_alignment(architecture).await?) / 2.0;
        let novelty = (suite_result.novelty + self.evaluate_novelty(architecture).await?) / 2.0;

        // Compute weighted overall score using the stored weights
        let overall = self.capability_weight * capability
            + self.efficiency_weight * efficiency
            + self.alignment_weight * alignment
            + self.novelty_weight * novelty;

        // Compute confidence based on variance of component scores
        let scores = vec![capability, efficiency, alignment, novelty];
        let confidence = self.compute_confidence(&scores);

        Ok(FitnessScore {
            overall,
            capability,
            efficiency,
            alignment,
            novelty,
            confidence,
        })
    }

    /// Evaluate capability dimension
    async fn evaluate_capability(&self, architecture: &Architecture) -> Result<f64, EvaluationError> {
        // Simplified evaluation - in production would run benchmarks
        // Score based on architecture complexity and paradigm
        let base_score = match architecture.paradigm {
            omega_core::Paradigm::Neural => 0.7,
            omega_core::Paradigm::Hybrid => 0.8,
            omega_core::Paradigm::Quantum => 0.9,
            _ => 0.5,
        };

        Ok(base_score)
    }

    /// Evaluate efficiency dimension
    async fn evaluate_efficiency(&self, architecture: &Architecture) -> Result<f64, EvaluationError> {
        // Simplified evaluation - in production would measure resource usage
        let base_score = match architecture.substrate {
            omega_core::SubstrateType::Digital => 0.8,
            omega_core::SubstrateType::Biological => 0.7,
            _ => 0.6,
        };

        Ok(base_score)
    }

    /// Evaluate alignment dimension
    async fn evaluate_alignment(&self, _architecture: &Architecture) -> Result<f64, EvaluationError> {
        // Simplified evaluation - in production would run alignment tests
        // For now, return high score assuming architectures are aligned
        Ok(0.9)
    }

    /// Evaluate novelty dimension
    async fn evaluate_novelty(&self, architecture: &Architecture) -> Result<f64, EvaluationError> {
        // Simplified evaluation - in production would compare against database
        // Score based on lineage depth (newer generations are more novel)
        let novelty = if architecture.lineage.is_empty() {
            0.5
        } else {
            0.7 + (architecture.lineage.len() as f64 * 0.05).min(0.3)
        };

        Ok(novelty)
    }

    /// Compute confidence in evaluation
    fn compute_confidence(&self, scores: &[f64]) -> f64 {
        if scores.is_empty() {
            return 0.0;
        }

        // Compute variance
        let mean = scores.iter().sum::<f64>() / scores.len() as f64;
        let variance = scores.iter()
            .map(|s| (s - mean).powi(2))
            .sum::<f64>() / scores.len() as f64;

        // Higher variance = lower confidence
        // Map variance [0, 0.25] to confidence [1.0, 0.5]
        let confidence = 1.0 - (variance * 2.0).min(0.5);

        confidence.clamp(0.5, 1.0)
    }
}

impl Default for FitnessEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

/// Benchmark for evaluating capabilities
pub struct Benchmark {
    pub id: String,
    pub name: String,
    pub category: String,
}

impl Benchmark {
    pub fn new(id: String, name: String, category: String) -> Self {
        Self { id, name, category }
    }

    pub async fn run(&self, _architecture: &Architecture) -> Result<f64, EvaluationError> {
        // Simplified - would run actual benchmarks in production
        Ok(0.75)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_fitness_evaluation() {
        let evaluator = FitnessEvaluator::new();

        let architecture = Architecture {
            id: "test-arch".to_string(),
            name: "Test Architecture".to_string(),
            paradigm: omega_core::Paradigm::Neural,
            substrate: omega_core::SubstrateType::Digital,
            fitness: None,
            lineage: vec![],
            created_at: Utc::now(),
        };

        let result = evaluator.evaluate(&architecture).await;
        assert!(result.is_ok());

        let fitness = result.unwrap();
        assert!(fitness.overall >= 0.0 && fitness.overall <= 1.0);
        assert!(fitness.confidence >= 0.5 && fitness.confidence <= 1.0);
    }

    #[test]
    fn test_compute_confidence() {
        let evaluator = FitnessEvaluator::new();

        // Low variance = high confidence
        let scores = vec![0.7, 0.71, 0.69, 0.70];
        let confidence = evaluator.compute_confidence(&scores);
        assert!(confidence > 0.9);

        // High variance = lower confidence
        let scores = vec![0.1, 0.9, 0.3, 0.7];
        let confidence = evaluator.compute_confidence(&scores);
        assert!(confidence < 0.9);
    }
}
