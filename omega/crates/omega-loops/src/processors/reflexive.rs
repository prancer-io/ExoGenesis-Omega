//! Reflexive Loop Processor (<1ms) - Immediate pattern-triggered responses

use super::{CycleProcessor, ProcessorMetrics, ProcessorInsight, metrics_to_json, insights_to_strings};
use omega_core::{CycleInput, CycleOutput, Action, ActionType};
use async_trait::async_trait;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;
use tracing::{trace, warn};

/// Reflexive processor for instant pattern-triggered responses
/// Target latency: <1ms
pub struct ReflexiveProcessor {
    /// Pattern -> Response mappings (pre-compiled for speed)
    reflexes: RwLock<HashMap<String, serde_json::Value>>,
    /// Pattern matching threshold
    threshold: f64,
    /// Target latency
    target_latency_ms: u128,
}

impl ReflexiveProcessor {
    pub fn new() -> Self {
        let mut reflexes = HashMap::new();

        // Pre-load common reflexes for instant response
        reflexes.insert("danger".to_string(), serde_json::json!({
            "action": "evade",
            "priority": "immediate",
            "response": "defensive_posture"
        }));
        reflexes.insert("opportunity".to_string(), serde_json::json!({
            "action": "capture",
            "priority": "high",
            "response": "engage"
        }));
        reflexes.insert("query".to_string(), serde_json::json!({
            "action": "respond",
            "priority": "normal",
            "response": "acknowledge"
        }));
        reflexes.insert("error".to_string(), serde_json::json!({
            "action": "recover",
            "priority": "high",
            "response": "fallback"
        }));
        reflexes.insert("stimulus".to_string(), serde_json::json!({
            "action": "orient",
            "priority": "medium",
            "response": "attention"
        }));

        Self {
            reflexes: RwLock::new(reflexes),
            threshold: 0.8,
            target_latency_ms: 1,
        }
    }

    /// Add a new reflex pattern
    pub fn add_reflex(&self, pattern: String, response: serde_json::Value) {
        self.reflexes.write().insert(pattern, response);
    }

    /// Match input against known patterns
    fn match_pattern(&self, input: &HashMap<String, serde_json::Value>) -> Option<(String, f64, serde_json::Value)> {
        let reflexes = self.reflexes.read();

        // Check data fields for pattern matches
        for (key, value) in input {
            let value_str = value.to_string().to_lowercase();

            for (pattern, response) in reflexes.iter() {
                if value_str.contains(pattern) || key.to_lowercase().contains(pattern) {
                    return Some((pattern.clone(), 1.0, response.clone()));
                }
            }
        }

        // Fuzzy matching could be added here for more sophisticated matching
        None
    }
}

impl Default for ReflexiveProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CycleProcessor for ReflexiveProcessor {
    async fn process(&mut self, input: CycleInput) -> Result<CycleOutput, Box<dyn Error>> {
        let start = Instant::now();
        trace!("Processing Reflexive cycle");

        let mut insights = Vec::new();
        let mut actions = Vec::new();
        let mut results = HashMap::new();

        // Pattern matching
        let (output, pattern_matched) = if let Some((pattern, confidence, response)) = self.match_pattern(&input.data) {
            insights.push(ProcessorInsight::new(
                "pattern_match",
                format!("Matched reflex pattern '{}' with {:.0}% confidence", pattern, confidence * 100.0),
                confidence
            ));

            // Create reflex action
            actions.push(Action {
                id: uuid::Uuid::now_v7().to_string(),
                action_type: ActionType::Execute,
                description: format!("Reflex response to '{}'", pattern),
                parameters: response.as_object().unwrap_or(&serde_json::Map::new()).iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
                priority: 1.0,
            });

            (serde_json::json!({
                "matched": true,
                "pattern": pattern,
                "confidence": confidence,
                "response": response,
            }), true)
        } else {
            insights.push(ProcessorInsight::new(
                "no_match",
                "No reflex pattern matched, passing through",
                0.5
            ));

            (serde_json::json!({
                "matched": false,
                "action": "pass_through",
                "reason": "no_pattern_match"
            }), false)
        };

        let latency = start.elapsed();

        // Warn if we exceeded target latency
        if latency.as_millis() > self.target_latency_ms {
            warn!(
                "Reflexive processor exceeded target latency: {}ms > {}ms",
                latency.as_millis(),
                self.target_latency_ms
            );
        }

        // Build metrics
        let metrics = ProcessorMetrics {
            latency,
            cpu_ms: latency.as_micros() as u64 / 1000,
            memory_bytes: 1024, // Minimal memory for reflex lookup
            io_ops: 0,
            success: true,
        };

        results.insert("reflex_response".to_string(), output);
        results.insert("metrics".to_string(), metrics_to_json(&metrics));
        results.insert("pattern_matched".to_string(), serde_json::json!(pattern_matched));
        results.insert("reflex_count".to_string(), serde_json::json!(self.reflexes.read().len()));

        // Add perception action if not already responding
        if !pattern_matched {
            actions.push(Action {
                id: uuid::Uuid::now_v7().to_string(),
                action_type: ActionType::Perceive,
                description: "Continue sensory monitoring".to_string(),
                parameters: HashMap::new(),
                priority: 0.5,
            });
        }

        Ok(CycleOutput {
            results,
            insights: insights_to_strings(&insights),
            actions,
            next_objectives: input.objectives,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_reflexive_pattern_match() {
        let mut processor = ReflexiveProcessor::new();
        let mut data = HashMap::new();
        data.insert("event".to_string(), serde_json::json!("danger detected"));

        let input = CycleInput {
            data,
            context: "test".to_string(),
            objectives: vec!["respond".to_string()],
        };

        let result = processor.process(input).await.unwrap();
        assert!(result.results.contains_key("reflex_response"));

        let reflex_response = &result.results["reflex_response"];
        assert_eq!(reflex_response["matched"].as_bool().unwrap(), true);
        assert_eq!(reflex_response["pattern"].as_str().unwrap(), "danger");

        // Check latency
        let metrics = &result.results["metrics"];
        assert!(metrics["latency_ms"].as_u64().unwrap() < 100); // Should be very fast
    }

    #[tokio::test]
    async fn test_reflexive_no_match() {
        let mut processor = ReflexiveProcessor::new();
        let mut data = HashMap::new();
        data.insert("unknown".to_string(), serde_json::json!("random data"));

        let input = CycleInput {
            data,
            context: "test".to_string(),
            objectives: vec![],
        };

        let result = processor.process(input).await.unwrap();

        let reflex_response = &result.results["reflex_response"];
        assert_eq!(reflex_response["matched"].as_bool().unwrap(), false);
    }

    #[tokio::test]
    async fn test_add_custom_reflex() {
        let mut processor = ReflexiveProcessor::new();

        // Add custom reflex
        processor.add_reflex(
            "custom_pattern".to_string(),
            serde_json::json!({"action": "custom_response"})
        );

        let mut data = HashMap::new();
        data.insert("test".to_string(), serde_json::json!("custom_pattern trigger"));

        let input = CycleInput {
            data,
            context: "test".to_string(),
            objectives: vec![],
        };

        let result = processor.process(input).await.unwrap();
        let reflex_response = &result.results["reflex_response"];
        assert_eq!(reflex_response["matched"].as_bool().unwrap(), true);
        assert_eq!(reflex_response["pattern"].as_str().unwrap(), "custom_pattern");
    }
}
