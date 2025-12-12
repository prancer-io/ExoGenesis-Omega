//! Sharp-Wave Ripple Replay
//!
//! Memory replay during rest/sleep:
//! - Sharp-wave ripples (SWRs)
//! - Sequence replay (forward and reverse)
//! - Memory consolidation
//! - Priority-based replay

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// A single replay event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayEvent {
    /// Associated memory ID
    pub memory_id: String,
    /// Pattern being replayed
    pub pattern: Vec<f64>,
    /// Timestamp of original encoding
    pub timestamp: u64,
    /// Priority for replay
    pub priority: f64,
}

impl ReplayEvent {
    pub fn new(memory_id: String, pattern: Vec<f64>) -> Self {
        Self {
            memory_id,
            pattern,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            priority: 1.0,
        }
    }
}

/// Sharp-wave ripple event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharpWaveRipple {
    /// Timestamp of ripple
    pub timestamp: u64,
    /// Sequence of patterns replayed
    pub patterns: Vec<Vec<f64>>,
    /// Duration in milliseconds
    pub duration_ms: u32,
    /// Ripple frequency (150-250 Hz)
    pub frequency_hz: f64,
}

impl SharpWaveRipple {
    /// Create new SWR
    pub fn new(patterns: Vec<Vec<f64>>) -> Self {
        Self {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            patterns,
            duration_ms: 80,
            frequency_hz: 180.0,
        }
    }

    /// Get number of patterns
    pub fn len(&self) -> usize {
        self.patterns.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.patterns.is_empty()
    }
}

/// Replay buffer for storing experiences
pub struct ReplayBuffer {
    /// Buffer of events
    events: VecDeque<ReplayEvent>,
    /// Maximum capacity
    capacity: usize,
    /// Temperature for sampling (higher = more random, lower = more greedy)
    temperature: f64,
    /// Minimum priority
    min_priority: f64,
    /// Total priority for sampling
    total_priority: f64,
}

impl ReplayBuffer {
    /// Create new replay buffer
    pub fn new(capacity: usize) -> Self {
        Self {
            events: VecDeque::with_capacity(capacity),
            capacity,
            temperature: 1.0,
            min_priority: 0.01,
            total_priority: 0.0,
        }
    }

    /// Add event to buffer
    pub fn add(&mut self, event: ReplayEvent) {
        if self.events.len() >= self.capacity {
            // Remove oldest low-priority event
            if let Some(idx) = self.find_min_priority_index() {
                if let Some(removed) = self.events.remove(idx) {
                    self.total_priority -= removed.priority;
                }
            }
        }

        self.total_priority += event.priority.max(self.min_priority);
        self.events.push_back(event);
    }

    /// Sample events uniformly
    pub fn sample(&self, n: usize) -> Vec<ReplayEvent> {
        if self.events.is_empty() {
            return Vec::new();
        }

        let mut rng = rand::thread_rng();
        let mut sampled = Vec::with_capacity(n);

        for _ in 0..n {
            let idx = rng.gen_range(0..self.events.len());
            if let Some(event) = self.events.get(idx) {
                sampled.push(event.clone());
            }
        }

        sampled
    }

    /// Sample events by priority with temperature scaling
    pub fn sample_prioritized(&self, n: usize) -> Vec<ReplayEvent> {
        if self.events.is_empty() || self.total_priority <= 0.0 {
            return Vec::new();
        }

        let mut rng = rand::thread_rng();
        let mut sampled = Vec::with_capacity(n);

        // Apply temperature to priorities (higher temp = more uniform, lower = more greedy)
        let scaled_priorities: Vec<f64> = self.events
            .iter()
            .map(|e| (e.priority.max(self.min_priority) / self.temperature).exp())
            .collect();
        let scaled_total: f64 = scaled_priorities.iter().sum();

        for _ in 0..n {
            let target = rng.gen::<f64>() * scaled_total;
            let mut cumsum = 0.0;

            for (event, &scaled_p) in self.events.iter().zip(scaled_priorities.iter()) {
                cumsum += scaled_p;
                if cumsum >= target {
                    sampled.push(event.clone());
                    break;
                }
            }
        }

        sampled
    }

    /// Set sampling temperature
    pub fn set_temperature(&mut self, temperature: f64) {
        self.temperature = temperature.max(0.01); // Prevent division by zero
    }

    /// Get current temperature
    pub fn temperature(&self) -> f64 {
        self.temperature
    }

    /// Sample a sequence (for sequence replay)
    pub fn sample_sequence(&self, length: usize) -> Vec<ReplayEvent> {
        if self.events.is_empty() {
            return Vec::new();
        }

        let mut rng = rand::thread_rng();

        // Find a starting point
        let max_start = self.events.len().saturating_sub(length);
        let start = rng.gen_range(0..=max_start);

        // Extract sequence
        self.events
            .iter()
            .skip(start)
            .take(length)
            .cloned()
            .collect()
    }

    /// Sample reverse sequence (for reverse replay)
    pub fn sample_reverse_sequence(&self, length: usize) -> Vec<ReplayEvent> {
        let mut seq = self.sample_sequence(length);
        seq.reverse();
        seq
    }

    /// Update priority of an event
    pub fn update_priority(&mut self, memory_id: &str, new_priority: f64) {
        for event in &mut self.events {
            if event.memory_id == memory_id {
                self.total_priority -= event.priority.max(self.min_priority);
                event.priority = new_priority;
                self.total_priority += event.priority.max(self.min_priority);
                break;
            }
        }
    }

    /// Decay all priorities
    pub fn decay_priorities(&mut self, factor: f64) {
        self.total_priority = 0.0;
        for event in &mut self.events {
            event.priority *= factor;
            self.total_priority += event.priority.max(self.min_priority);
        }
    }

    /// Boost priorities of recently accessed items
    pub fn boost_recent(&mut self, memory_ids: &[String], boost: f64) {
        for event in &mut self.events {
            if memory_ids.contains(&event.memory_id) {
                self.total_priority -= event.priority.max(self.min_priority);
                event.priority += boost;
                event.priority = event.priority.min(10.0);
                self.total_priority += event.priority.max(self.min_priority);
            }
        }
    }

    /// Find index of minimum priority event
    fn find_min_priority_index(&self) -> Option<usize> {
        self.events
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.priority.partial_cmp(&b.1.priority).unwrap())
            .map(|(i, _)| i)
    }

    /// Get buffer length
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Clear buffer
    pub fn clear(&mut self) {
        self.events.clear();
        self.total_priority = 0.0;
    }

    /// Get recent events
    pub fn recent(&self, n: usize) -> Vec<&ReplayEvent> {
        self.events.iter().rev().take(n).collect()
    }

    /// Get events by time range
    pub fn get_by_time(&self, start: u64, end: u64) -> Vec<&ReplayEvent> {
        self.events
            .iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .collect()
    }
}

impl Default for ReplayBuffer {
    fn default() -> Self {
        Self::new(1000)
    }
}

/// Replay scheduler for controlling when replay occurs
pub struct ReplayScheduler {
    /// Time since last replay (ms)
    time_since_replay: u64,
    /// Replay interval (ms)
    replay_interval: u64,
    /// Number of events per replay
    events_per_replay: usize,
    /// Forward replay probability
    forward_prob: f64,
    /// Whether replay is enabled
    enabled: bool,
    /// Replay count
    replay_count: u64,
}

impl ReplayScheduler {
    pub fn new() -> Self {
        Self {
            time_since_replay: 0,
            replay_interval: 5000, // 5 seconds
            events_per_replay: 10,
            forward_prob: 0.6,
            enabled: true,
            replay_count: 0,
        }
    }

    /// Update and check if replay should occur
    pub fn update(&mut self, dt_ms: u64) -> bool {
        if !self.enabled {
            return false;
        }

        self.time_since_replay += dt_ms;

        if self.time_since_replay >= self.replay_interval {
            self.time_since_replay = 0;
            self.replay_count += 1;
            true
        } else {
            false
        }
    }

    /// Trigger immediate replay
    pub fn trigger(&mut self) {
        self.time_since_replay = self.replay_interval;
    }

    /// Get replay direction
    pub fn should_replay_forward(&self) -> bool {
        rand::thread_rng().gen::<f64>() < self.forward_prob
    }

    /// Set replay parameters
    pub fn configure(&mut self, interval_ms: u64, events_per: usize, forward_prob: f64) {
        self.replay_interval = interval_ms;
        self.events_per_replay = events_per;
        self.forward_prob = forward_prob;
    }

    /// Enable/disable replay
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Get replay count
    pub fn replay_count(&self) -> u64 {
        self.replay_count
    }

    /// Get events per replay
    pub fn events_per_replay(&self) -> usize {
        self.events_per_replay
    }
}

impl Default for ReplayScheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// Experience replay for reinforcement learning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    /// State before action
    pub state: Vec<f64>,
    /// Action taken
    pub action: usize,
    /// Reward received
    pub reward: f64,
    /// Next state
    pub next_state: Vec<f64>,
    /// Whether episode ended
    pub done: bool,
    /// TD error for prioritization
    pub td_error: f64,
}

impl Experience {
    pub fn new(
        state: Vec<f64>,
        action: usize,
        reward: f64,
        next_state: Vec<f64>,
        done: bool,
    ) -> Self {
        Self {
            state,
            action,
            reward,
            next_state,
            done,
            td_error: 1.0, // Initial high priority
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replay_event() {
        let event = ReplayEvent::new("mem1".to_string(), vec![0.5; 10]);
        assert_eq!(event.memory_id, "mem1");
        assert_eq!(event.pattern.len(), 10);
    }

    #[test]
    fn test_replay_buffer() {
        let mut buffer = ReplayBuffer::new(100);

        for i in 0..50 {
            let event = ReplayEvent::new(format!("mem{}", i), vec![i as f64; 5]);
            buffer.add(event);
        }

        assert_eq!(buffer.len(), 50);
    }

    #[test]
    fn test_sample() {
        let mut buffer = ReplayBuffer::new(100);

        for i in 0..20 {
            let event = ReplayEvent::new(format!("mem{}", i), vec![i as f64; 5]);
            buffer.add(event);
        }

        let sampled = buffer.sample(5);
        assert_eq!(sampled.len(), 5);
    }

    #[test]
    fn test_prioritized_sample() {
        let mut buffer = ReplayBuffer::new(100);

        // Add low priority events
        for i in 0..10 {
            let mut event = ReplayEvent::new(format!("low{}", i), vec![0.0; 5]);
            event.priority = 0.1;
            buffer.add(event);
        }

        // Add high priority event
        let mut high = ReplayEvent::new("high".to_string(), vec![1.0; 5]);
        high.priority = 10.0;
        buffer.add(high);

        // Sample many times, high priority should appear more often
        let mut high_count = 0;
        for _ in 0..100 {
            let sampled = buffer.sample_prioritized(1);
            if sampled[0].memory_id == "high" {
                high_count += 1;
            }
        }

        assert!(high_count > 30); // Should be sampled frequently
    }

    #[test]
    fn test_sequence_replay() {
        let mut buffer = ReplayBuffer::new(100);

        for i in 0..20 {
            let event = ReplayEvent::new(format!("mem{}", i), vec![i as f64; 5]);
            buffer.add(event);
        }

        let seq = buffer.sample_sequence(5);
        assert_eq!(seq.len(), 5);

        let rev_seq = buffer.sample_reverse_sequence(5);
        assert_eq!(rev_seq.len(), 5);
    }

    #[test]
    fn test_sharp_wave_ripple() {
        let patterns = vec![vec![0.5; 10], vec![0.6; 10], vec![0.7; 10]];
        let swr = SharpWaveRipple::new(patterns);

        assert_eq!(swr.len(), 3);
        assert!(swr.frequency_hz > 100.0);
    }

    #[test]
    fn test_replay_scheduler() {
        let mut scheduler = ReplayScheduler::new();
        scheduler.configure(100, 5, 0.6);

        // Not time yet
        assert!(!scheduler.update(50));

        // Now it's time
        assert!(scheduler.update(60));
        assert_eq!(scheduler.replay_count(), 1);
    }

    #[test]
    fn test_priority_update() {
        let mut buffer = ReplayBuffer::new(100);

        let event = ReplayEvent::new("test".to_string(), vec![0.5; 5]);
        buffer.add(event);

        buffer.update_priority("test", 5.0);

        // Sample and check priority was updated
        let sampled = buffer.sample(1);
        assert!((sampled[0].priority - 5.0).abs() < 0.01);
    }
}
