//! Cycle processors for each temporal loop type

use omega_core::{CycleInput, CycleOutput};
use async_trait::async_trait;
use std::error::Error;
use std::time::Duration;

pub mod reflexive;
pub mod reactive;
pub mod adaptive;
pub mod deliberative;
pub mod evolutionary;
pub mod transformative;
pub mod transcendent;

pub use reflexive::ReflexiveProcessor;
pub use reactive::ReactiveProcessor;
pub use adaptive::AdaptiveProcessor;
pub use deliberative::DeliberativeProcessor;
pub use evolutionary::EvolutionaryProcessor;
pub use transformative::TransformativeProcessor;
pub use transcendent::TranscendentProcessor;

/// Trait for processing cycles in temporal loops
#[async_trait]
pub trait CycleProcessor: Send + Sync {
    /// Process a cycle input and produce output
    async fn process(&mut self, input: CycleInput) -> Result<CycleOutput, Box<dyn Error>>;
}

/// Internal metrics for processor performance
#[derive(Debug, Clone)]
pub struct ProcessorMetrics {
    pub latency: Duration,
    pub cpu_ms: u64,
    pub memory_bytes: usize,
    pub io_ops: usize,
    pub success: bool,
}

impl Default for ProcessorMetrics {
    fn default() -> Self {
        Self {
            latency: Duration::from_millis(0),
            cpu_ms: 0,
            memory_bytes: 0,
            io_ops: 0,
            success: true,
        }
    }
}

/// Internal insight type for processor-generated observations
#[derive(Debug, Clone)]
pub struct ProcessorInsight {
    pub category: String,
    pub content: String,
    pub confidence: f64,
}

impl ProcessorInsight {
    pub fn new(category: impl Into<String>, content: impl Into<String>, confidence: f64) -> Self {
        Self {
            category: category.into(),
            content: content.into(),
            confidence,
        }
    }
}

/// Helper to convert ProcessorMetrics to latency string for results
pub fn metrics_to_json(metrics: &ProcessorMetrics) -> serde_json::Value {
    serde_json::json!({
        "latency_ms": metrics.latency.as_millis(),
        "cpu_ms": metrics.cpu_ms,
        "memory_bytes": metrics.memory_bytes,
        "io_ops": metrics.io_ops,
        "success": metrics.success,
    })
}

/// Helper to convert insights to strings
pub fn insights_to_strings(insights: &[ProcessorInsight]) -> Vec<String> {
    insights.iter().map(|i| i.content.clone()).collect()
}
