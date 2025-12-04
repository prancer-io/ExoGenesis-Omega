//! Loop 2: Neural - Pattern recognition and motor control (~100ms)

use crate::{
    LoopId, LoopInput, LoopOutput, LoopError, TemporalLoop, TickResult, Timescale,
    loops::{LoopState, timescales},
};
use async_trait::async_trait;
use std::collections::HashMap;
use std::time::Instant;
use tokio::sync::RwLock;
use tracing::{debug, trace};

/// Neural loop - pattern recognition layer
///
/// Handles pattern recognition, motor control coordination, and
/// learned response activation at ~100ms intervals.
pub struct NeuralLoop {
    state: RwLock<LoopState>,
    patterns: RwLock<HashMap<String, PatternInfo>>,
    activations: RwLock<Vec<Activation>>,
}

#[derive(Debug, Clone)]
struct PatternInfo {
    pattern_id: String,
    activation_count: u64,
    confidence: f64,
    last_seen: std::time::SystemTime,
}

#[derive(Debug, Clone)]
struct Activation {
    pattern_id: String,
    strength: f64,
    timestamp: std::time::SystemTime,
}

impl NeuralLoop {
    pub fn new() -> Self {
        debug!("Initializing Neural Loop");
        Self {
            state: RwLock::new(LoopState::new()),
            patterns: RwLock::new(HashMap::new()),
            activations: RwLock::new(Vec::new()),
        }
    }

    /// Recognize patterns from quantum layer input
    async fn recognize_patterns(&self, data: &serde_json::Value) -> Vec<String> {
        trace!("Recognizing patterns in Neural loop");

        let mut recognized = Vec::new();

        // Simple pattern recognition
        // In real system would use neural networks
        if let Some(obj) = data.as_object() {
            if obj.contains_key("features") {
                recognized.push("feature_vector".to_string());
            }
            if obj.get("type").and_then(|v| v.as_str()) == Some("sensory_processed") {
                recognized.push("sensory_input".to_string());
            }
        }

        // Update pattern database
        let mut patterns = self.patterns.write().await;
        for pattern_id in &recognized {
            patterns
                .entry(pattern_id.clone())
                .and_modify(|info| {
                    info.activation_count += 1;
                    info.confidence = (info.confidence * 0.9 + 0.1).min(1.0);
                    info.last_seen = std::time::SystemTime::now();
                })
                .or_insert(PatternInfo {
                    pattern_id: pattern_id.clone(),
                    activation_count: 1,
                    confidence: 0.5,
                    last_seen: std::time::SystemTime::now(),
                });
        }

        recognized
    }

    /// Generate motor control commands
    async fn generate_motor_control(&self, patterns: &[String]) -> Option<serde_json::Value> {
        if patterns.is_empty() {
            return None;
        }

        // Simulate motor command generation
        Some(serde_json::json!({
            "type": "motor_command",
            "commands": patterns.iter().map(|p| {
                serde_json::json!({
                    "pattern": p,
                    "action": "respond",
                    "strength": 0.8,
                })
            }).collect::<Vec<_>>(),
        }))
    }
}

impl Default for NeuralLoop {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TemporalLoop for NeuralLoop {
    fn id(&self) -> LoopId {
        LoopId::Neural
    }

    fn timescale(&self) -> Timescale {
        timescales::neural()
    }

    async fn tick(&mut self) -> Result<TickResult, LoopError> {
        let start = Instant::now();
        let state = self.state.read().await;

        if !state.running {
            return Ok(TickResult {
                loop_id: self.id(),
                duration: start.elapsed(),
                processed: 0,
                should_continue: false,
                status: Some("not running".to_string()),
            });
        }
        drop(state);

        // Process recent activations
        let mut activations = self.activations.write().await;
        let processed = activations.len();

        // Decay old activations
        activations.retain(|act| {
            act.timestamp
                .elapsed()
                .map(|d| d.as_secs() < 1)
                .unwrap_or(false)
        });

        let duration = start.elapsed();

        // Update state
        let mut state = self.state.write().await;
        state.tick(duration, processed);

        Ok(TickResult {
            loop_id: self.id(),
            duration,
            processed,
            should_continue: true,
            status: Some(format!("Active patterns: {}", processed)),
        })
    }

    async fn process(&mut self, input: LoopInput) -> Result<LoopOutput, LoopError> {
        let start = Instant::now();

        trace!("Neural loop processing input from {:?}", input.source);

        // Recognize patterns
        let patterns = self.recognize_patterns(&input.data).await;

        // Generate motor control if needed
        let motor_control = self.generate_motor_control(&patterns).await;

        // Record activations
        let mut activations = self.activations.write().await;
        for pattern_id in &patterns {
            activations.push(Activation {
                pattern_id: pattern_id.clone(),
                strength: 0.8,
                timestamp: std::time::SystemTime::now(),
            });
        }

        let duration = start.elapsed();

        // Get pattern stats
        let pattern_stats = self.patterns.read().await;
        let total_patterns = pattern_stats.len();

        Ok(LoopOutput {
            source: self.id(),
            data: serde_json::json!({
                "type": "neural_processed",
                "recognized_patterns": patterns,
                "motor_control": motor_control,
                "timestamp": std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis(),
            }),
            metadata: serde_json::json!({
                "processing_time_ms": duration.as_millis(),
                "patterns_recognized": patterns.len(),
                "total_patterns_known": total_patterns,
                "active_activations": activations.len(),
            }),
        })
    }

    async fn start(&mut self) -> Result<(), LoopError> {
        let mut state = self.state.write().await;
        if state.running {
            return Err(LoopError::StartFailed(
                self.id(),
                "already running".to_string(),
            ));
        }

        debug!("Starting Neural loop");
        state.start();
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), LoopError> {
        let mut state = self.state.write().await;
        if !state.running {
            return Ok(());
        }

        let patterns = self.patterns.read().await;
        debug!(
            "Stopping Neural loop - {} patterns learned, {} items processed",
            patterns.len(),
            state.processed_count
        );

        state.stop();

        // Clear activations
        let mut activations = self.activations.write().await;
        activations.clear();

        Ok(())
    }

    fn is_running(&self) -> bool {
        false // Placeholder - would need sync primitive
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_neural_loop_lifecycle() {
        let mut loop_impl = NeuralLoop::new();

        loop_impl.start().await.unwrap();

        let input = LoopInput {
            source: Some(LoopId::Quantum),
            target: LoopId::Neural,
            data: serde_json::json!({
                "type": "sensory_processed",
                "features": {"test": true}
            }),
            priority: 200,
        };

        let output = loop_impl.process(input).await.unwrap();
        assert_eq!(output.source, LoopId::Neural);

        loop_impl.stop().await.unwrap();
    }

    #[tokio::test]
    async fn test_pattern_recognition() {
        let loop_impl = NeuralLoop::new();

        let data = serde_json::json!({
            "type": "sensory_processed",
            "features": {}
        });

        let patterns = loop_impl.recognize_patterns(&data).await;
        assert!(!patterns.is_empty());
    }
}
