//! Loop 1: Quantum - Immediate sensory processing (~1ms)

use crate::{
    LoopId, LoopInput, LoopOutput, LoopError, TemporalLoop, TickResult, Timescale,
    loops::{LoopState, timescales},
};
use async_trait::async_trait;
use std::time::Instant;
use tokio::sync::RwLock;
use tracing::{debug, trace};

/// Quantum loop - fastest processing layer
///
/// Handles immediate sensory processing, reflex responses, and
/// high-frequency pattern matching at ~1ms intervals.
pub struct QuantumLoop {
    state: RwLock<LoopState>,
    buffer: RwLock<Vec<serde_json::Value>>,
}

impl QuantumLoop {
    pub fn new() -> Self {
        debug!("Initializing Quantum Loop");
        Self {
            state: RwLock::new(LoopState::new()),
            buffer: RwLock::new(Vec::new()),
        }
    }

    /// Process raw sensory input
    async fn process_sensory(&self, data: &serde_json::Value) -> Result<serde_json::Value, LoopError> {
        trace!("Processing sensory data in Quantum loop");

        // Simulate immediate pattern detection
        // In a real system, this would involve:
        // - Signal filtering
        // - Edge detection
        // - Threshold comparison
        // - Immediate reflex triggers

        Ok(serde_json::json!({
            "type": "sensory_processed",
            "features": {
                "raw_input": data,
                "detected_patterns": [],
                "threshold_crossings": 0,
            },
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        }))
    }

    /// Detect high-frequency patterns
    async fn detect_patterns(&self, buffer: &[serde_json::Value]) -> Vec<String> {
        // In real implementation, would use signal processing
        // to detect repeating patterns, anomalies, etc.

        if buffer.len() > 10 {
            vec!["high_frequency_activity".to_string()]
        } else {
            vec![]
        }
    }
}

impl Default for QuantumLoop {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TemporalLoop for QuantumLoop {
    fn id(&self) -> LoopId {
        LoopId::Quantum
    }

    fn timescale(&self) -> Timescale {
        timescales::quantum()
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

        // Process buffered data
        let mut buffer = self.buffer.write().await;
        let processed = buffer.len();

        // Detect patterns in buffer
        let patterns = self.detect_patterns(&buffer).await;

        // Clear old data (keep only recent)
        if buffer.len() > 100 {
            buffer.drain(0..50);
        }
        drop(buffer);

        let duration = start.elapsed();

        // Update state
        let mut state = self.state.write().await;
        state.tick(duration, processed);

        Ok(TickResult {
            loop_id: self.id(),
            duration,
            processed,
            should_continue: true,
            status: if patterns.is_empty() {
                None
            } else {
                Some(format!("Patterns: {:?}", patterns))
            },
        })
    }

    async fn process(&mut self, input: LoopInput) -> Result<LoopOutput, LoopError> {
        let start = Instant::now();

        trace!("Quantum loop processing input from {:?}", input.source);

        // Process the sensory data
        let processed = self.process_sensory(&input.data).await?;

        // Add to buffer for pattern detection
        let mut buffer = self.buffer.write().await;
        buffer.push(processed.clone());

        let duration = start.elapsed();

        Ok(LoopOutput {
            source: self.id(),
            data: processed,
            metadata: serde_json::json!({
                "processing_time_us": duration.as_micros(),
                "buffer_size": buffer.len(),
                "priority": input.priority,
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

        debug!("Starting Quantum loop");
        state.start();
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), LoopError> {
        let mut state = self.state.write().await;
        if !state.running {
            return Ok(());
        }

        debug!(
            "Stopping Quantum loop - processed {} items in {} ticks",
            state.processed_count, state.tick_count
        );

        state.stop();

        // Clear buffer
        let mut buffer = self.buffer.write().await;
        buffer.clear();

        Ok(())
    }

    fn is_running(&self) -> bool {
        // Note: This is a synchronous method, so we can't await
        // In production, would use a sync primitive or return Future
        false // Placeholder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_quantum_loop_lifecycle() {
        let mut loop_impl = QuantumLoop::new();

        loop_impl.start().await.unwrap();

        let input = LoopInput {
            source: None,
            target: LoopId::Quantum,
            data: serde_json::json!({"value": 42}),
            priority: 255,
        };

        let output = loop_impl.process(input).await.unwrap();
        assert_eq!(output.source, LoopId::Quantum);

        loop_impl.stop().await.unwrap();
    }

    #[tokio::test]
    async fn test_quantum_tick() {
        let mut loop_impl = QuantumLoop::new();
        loop_impl.start().await.unwrap();

        let result = loop_impl.tick().await.unwrap();
        assert_eq!(result.loop_id, LoopId::Quantum);
        assert!(result.should_continue);
    }
}
