//! Cognitive Cycle
//!
//! The main processing loop that coordinates all brain components:
//! Perception → Attention → Integration → Memory → Action

use crate::{BrainConfig, BrainMode, Result};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Current cognitive state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveState {
    /// Current mode
    pub mode: BrainMode,
    /// Activity level (0-1)
    pub activity_level: f64,
    /// Integration level (0-1)
    pub integration: f64,
    /// Arousal level (0-1)
    pub arousal: f64,
    /// Valence (-1 to 1)
    pub valence: f64,
    /// Current goal (if any)
    pub goal: Option<Vec<f64>>,
}

impl CognitiveState {
    pub fn new() -> Self {
        Self {
            mode: BrainMode::Awake,
            activity_level: 0.5,
            integration: 0.5,
            arousal: 0.5,
            valence: 0.0,
            goal: None,
        }
    }
}

impl Default for CognitiveState {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a processing cycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingResult {
    /// Output vector
    pub output: Vec<f64>,
    /// Consciousness level (0-1)
    pub consciousness_level: f64,
    /// Attention strength (0-1)
    pub attention_strength: f64,
    /// Was a memory encoded?
    pub memory_encoded: bool,
    /// Was a strange loop detected?
    pub strange_loop_detected: bool,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
}

/// Cycle timing statistics
#[derive(Debug, Clone)]
struct CycleStats {
    /// Processing times (ms)
    times: VecDeque<u64>,
    /// Maximum stored
    max_stored: usize,
}

impl CycleStats {
    fn new(max_stored: usize) -> Self {
        Self {
            times: VecDeque::with_capacity(max_stored),
            max_stored,
        }
    }

    fn add(&mut self, time_ms: u64) {
        if self.times.len() >= self.max_stored {
            self.times.pop_front();
        }
        self.times.push_back(time_ms);
    }

    fn average(&self) -> f64 {
        if self.times.is_empty() {
            0.0
        } else {
            self.times.iter().sum::<u64>() as f64 / self.times.len() as f64
        }
    }
}

/// Manages the cognitive processing cycle
pub struct CognitiveCycle {
    /// Current state
    state: CognitiveState,
    /// Processing history
    history: VecDeque<ProcessingResult>,
    /// Maximum history length
    max_history: usize,
    /// Cycle statistics
    stats: CycleStats,
    /// Cycle count
    cycle_count: u64,
    /// Last processing timestamp
    last_cycle_time: u64,
}

impl CognitiveCycle {
    /// Create new cognitive cycle manager
    pub fn new(_config: &BrainConfig) -> Self {
        Self {
            state: CognitiveState::new(),
            history: VecDeque::with_capacity(100),
            max_history: 100,
            stats: CycleStats::new(1000),
            cycle_count: 0,
            last_cycle_time: Self::now(),
        }
    }

    /// Complete a processing cycle
    pub fn complete_cycle(
        &mut self,
        input: &[f64],
        neural_output: &[f64],
        attended: &[f64],
        conscious: &[f64],
        memory_output: &[f64],
        self_aware_output: &[f64],
    ) -> Result<ProcessingResult> {
        let start = Self::now();

        // Compute output as weighted combination
        let output = self.combine_outputs(
            input,
            neural_output,
            attended,
            conscious,
            memory_output,
            self_aware_output,
        );

        // Update state based on processing
        self.update_state(&output, attended, conscious)?;

        // Compute metrics
        let consciousness_level = self.compute_consciousness_level(conscious);
        let attention_strength = self.compute_attention_strength(attended);
        let memory_encoded = self.detect_memory_encoding(memory_output);
        let strange_loop_detected = self.detect_strange_loop(self_aware_output);

        let end = Self::now();
        let processing_time_ms = end.saturating_sub(start);

        let result = ProcessingResult {
            output,
            consciousness_level,
            attention_strength,
            memory_encoded,
            strange_loop_detected,
            processing_time_ms,
        };

        // Store in history
        self.history.push_back(result.clone());
        if self.history.len() > self.max_history {
            self.history.pop_front();
        }

        // Update stats
        self.stats.add(processing_time_ms);
        self.cycle_count += 1;
        self.last_cycle_time = end;

        Ok(result)
    }

    /// Combine outputs from different processing stages
    fn combine_outputs(
        &self,
        input: &[f64],
        neural: &[f64],
        attended: &[f64],
        conscious: &[f64],
        memory: &[f64],
        self_aware: &[f64],
    ) -> Vec<f64> {
        let len = input.len();
        let mut output = vec![0.0; len];

        // Weights for combination
        let w_neural = 0.15;
        let w_attended = 0.20;
        let w_conscious = 0.30;
        let w_memory = 0.20;
        let w_self = 0.15;

        for i in 0..len {
            let n = neural.get(i).copied().unwrap_or(0.0);
            let a = attended.get(i).copied().unwrap_or(0.0);
            let c = conscious.get(i).copied().unwrap_or(0.0);
            let m = memory.get(i).copied().unwrap_or(0.0);
            let s = self_aware.get(i).copied().unwrap_or(0.0);

            output[i] = w_neural * n + w_attended * a + w_conscious * c + w_memory * m + w_self * s;
        }

        output
    }

    /// Update cognitive state
    fn update_state(
        &mut self,
        output: &[f64],
        attended: &[f64],
        conscious: &[f64],
    ) -> Result<()> {
        // Compute activity level
        let activity: f64 = output.iter().map(|x| x.abs()).sum::<f64>() / output.len().max(1) as f64;
        self.state.activity_level = 0.9 * self.state.activity_level + 0.1 * activity;

        // Compute integration from conscious content
        let integration: f64 =
            conscious.iter().map(|x| x.abs()).sum::<f64>() / conscious.len().max(1) as f64;
        self.state.integration = 0.9 * self.state.integration + 0.1 * integration;

        // Compute arousal from attention strength
        let arousal: f64 =
            attended.iter().map(|x| x.abs()).sum::<f64>() / attended.len().max(1) as f64;
        self.state.arousal = 0.9 * self.state.arousal + 0.1 * arousal;

        // Update mode based on activity
        self.state.mode = self.determine_mode();

        Ok(())
    }

    /// Determine current operating mode
    fn determine_mode(&self) -> BrainMode {
        if self.state.arousal > 0.8 {
            BrainMode::Focused
        } else if self.state.activity_level < 0.2 {
            BrainMode::Idle
        } else if self.state.integration > 0.7 {
            BrainMode::Creative
        } else {
            BrainMode::Awake
        }
    }

    /// Compute consciousness level
    fn compute_consciousness_level(&self, conscious: &[f64]) -> f64 {
        if conscious.is_empty() {
            return 0.0;
        }
        let mean = conscious.iter().sum::<f64>() / conscious.len() as f64;
        let variance = conscious.iter().map(|x| (x - mean).powi(2)).sum::<f64>()
            / conscious.len() as f64;

        // Higher mean and lower variance = higher consciousness
        (mean.abs() * (1.0 - variance.min(1.0))).min(1.0)
    }

    /// Compute attention strength
    fn compute_attention_strength(&self, attended: &[f64]) -> f64 {
        if attended.is_empty() {
            return 0.0;
        }
        let max = attended.iter().map(|x| x.abs()).fold(0.0, f64::max);
        let mean = attended.iter().map(|x| x.abs()).sum::<f64>() / attended.len() as f64;

        // Higher peak relative to mean = stronger focus
        if mean > 0.0 {
            (max / mean).min(2.0) / 2.0
        } else {
            0.0
        }
    }

    /// Detect if memory was encoded
    fn detect_memory_encoding(&self, memory_output: &[f64]) -> bool {
        // Memory encoded if output significantly differs from zero
        let activity: f64 =
            memory_output.iter().map(|x| x.abs()).sum::<f64>() / memory_output.len().max(1) as f64;
        activity > 0.3
    }

    /// Detect strange loop
    fn detect_strange_loop(&self, self_aware_output: &[f64]) -> bool {
        // Compare to previous outputs for self-reference
        if let Some(prev) = self.history.back() {
            let similarity = self.cosine_similarity(&prev.output, self_aware_output);
            similarity > 0.9
        } else {
            false
        }
    }

    /// Cosine similarity
    fn cosine_similarity(&self, a: &[f64], b: &[f64]) -> f64 {
        if a.len() != b.len() || a.is_empty() {
            return 0.0;
        }

        let mut dot = 0.0;
        let mut norm_a = 0.0;
        let mut norm_b = 0.0;

        for (&x, &y) in a.iter().zip(b.iter()) {
            dot += x * y;
            norm_a += x * x;
            norm_b += y * y;
        }

        let denom = (norm_a * norm_b).sqrt();
        if denom > 0.0 {
            dot / denom
        } else {
            0.0
        }
    }

    /// Get current mode
    pub fn current_mode(&self) -> BrainMode {
        self.state.mode
    }

    /// Get activity level
    pub fn activity_level(&self) -> f64 {
        self.state.activity_level
    }

    /// Get current state
    pub fn state(&self) -> &CognitiveState {
        &self.state
    }

    /// Get average processing time
    pub fn avg_processing_time(&self) -> f64 {
        self.stats.average()
    }

    /// Get cycle count
    pub fn cycle_count(&self) -> u64 {
        self.cycle_count
    }

    /// Get recent history
    pub fn recent_history(&self, n: usize) -> Vec<&ProcessingResult> {
        self.history.iter().rev().take(n).collect()
    }

    /// Set goal
    pub fn set_goal(&mut self, goal: Vec<f64>) {
        self.state.goal = Some(goal);
    }

    /// Clear goal
    pub fn clear_goal(&mut self) {
        self.state.goal = None;
    }

    /// Reset the cycle manager
    pub fn reset(&mut self) {
        self.state = CognitiveState::new();
        self.history.clear();
        self.stats = CycleStats::new(1000);
        self.cycle_count = 0;
    }

    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }
}

impl Default for CognitiveCycle {
    fn default() -> Self {
        Self::new(&BrainConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cognitive_cycle_creation() {
        let config = BrainConfig::default();
        let cycle = CognitiveCycle::new(&config);

        assert_eq!(cycle.cycle_count(), 0);
    }

    #[test]
    fn test_complete_cycle() {
        let config = BrainConfig::default();
        let mut cycle = CognitiveCycle::new(&config);

        let input = vec![0.5; 10];
        let result = cycle
            .complete_cycle(&input, &input, &input, &input, &input, &input)
            .unwrap();

        assert_eq!(result.output.len(), 10);
        assert_eq!(cycle.cycle_count(), 1);
    }

    #[test]
    fn test_mode_determination() {
        let config = BrainConfig::default();
        let cycle = CognitiveCycle::new(&config);

        assert_eq!(cycle.current_mode(), BrainMode::Awake);
    }
}
