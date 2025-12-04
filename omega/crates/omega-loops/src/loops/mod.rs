//! Temporal Loop Implementations

pub mod quantum;
pub mod neural;
pub mod cognitive;
pub mod learning;
pub mod developmental;
pub mod evolutionary;
pub mod cosmic;

pub use quantum::QuantumLoop;
pub use neural::NeuralLoop;
pub use cognitive::CognitiveLoop;
pub use learning::LearningLoop;
pub use developmental::DevelopmentalLoop;
pub use evolutionary::EvolutionaryLoop;
pub use cosmic::CosmicLoop;

use crate::{LoopId, LoopInput, LoopOutput, LoopError, TickResult, Timescale};
use std::time::Duration;

/// Base structure for loop state management
#[derive(Debug, Clone)]
pub struct LoopState {
    /// Whether the loop is running
    pub running: bool,

    /// Total ticks processed
    pub tick_count: u64,

    /// Total items processed
    pub processed_count: u64,

    /// Last processing duration
    pub last_duration: Option<Duration>,
}

impl Default for LoopState {
    fn default() -> Self {
        Self {
            running: false,
            tick_count: 0,
            processed_count: 0,
            last_duration: None,
        }
    }
}

impl LoopState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn tick(&mut self, duration: Duration, processed: usize) {
        self.tick_count += 1;
        self.processed_count += processed as u64;
        self.last_duration = Some(duration);
    }
}

/// Helper to create standard timescales
pub mod timescales {
    use super::*;

    pub fn quantum() -> Timescale {
        Timescale::new(
            Duration::from_micros(500),
            Duration::from_millis(1),
            Duration::from_millis(5),
        )
    }

    pub fn neural() -> Timescale {
        Timescale::new(
            Duration::from_millis(50),
            Duration::from_millis(100),
            Duration::from_millis(500),
        )
    }

    pub fn cognitive() -> Timescale {
        Timescale::new(
            Duration::from_secs(30),
            Duration::from_secs(60),
            Duration::from_secs(300),
        )
    }

    pub fn learning() -> Timescale {
        Timescale::new(
            Duration::from_secs(1800),  // 30 min
            Duration::from_secs(3600),  // 1 hour
            Duration::from_secs(14400), // 4 hours
        )
    }

    pub fn developmental() -> Timescale {
        Timescale::new(
            Duration::from_secs(43200),  // 12 hours
            Duration::from_secs(86400),  // 24 hours
            Duration::from_secs(259200), // 3 days
        )
    }

    pub fn evolutionary() -> Timescale {
        Timescale::new(
            Duration::from_secs(604800),  // 1 week
            Duration::from_secs(2592000), // 30 days
            Duration::from_secs(7776000), // 90 days
        )
    }

    pub fn cosmic() -> Timescale {
        Timescale::new(
            Duration::from_secs(15552000),  // 180 days
            Duration::from_secs(31536000),  // 1 year
            Duration::from_secs(157680000), // 5 years
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_state() {
        let mut state = LoopState::new();
        assert!(!state.running);

        state.start();
        assert!(state.running);

        state.tick(Duration::from_millis(1), 5);
        assert_eq!(state.tick_count, 1);
        assert_eq!(state.processed_count, 5);

        state.stop();
        assert!(!state.running);
    }

    #[test]
    fn test_timescales() {
        let quantum = timescales::quantum();
        assert_eq!(quantum.typical, Duration::from_millis(1));

        let cosmic = timescales::cosmic();
        assert_eq!(cosmic.typical, Duration::from_secs(31536000));
    }
}
