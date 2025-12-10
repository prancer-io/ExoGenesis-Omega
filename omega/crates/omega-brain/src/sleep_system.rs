//! Sleep System Integration
//!
//! Integrates omega-sleep for:
//! - Sleep stage cycling (N1, N2, N3/SWS, REM)
//! - Memory consolidation
//! - Circadian rhythm

use crate::{BrainConfig, BrainError, Result};
use omega_sleep::{CircadianRhythm, SleepController, SleepStage, ConsolidationCycle};

/// Output from sleep processing
#[derive(Debug, Clone)]
pub struct SleepOutput {
    /// Is in slow wave sleep
    pub is_sws: bool,
    /// Is in REM sleep
    pub is_rem: bool,
    /// Number of memories to replay
    pub replay_count: usize,
    /// Sleep depth (0-1)
    pub depth: f64,
    /// Current stage name
    pub stage_name: String,
}

/// Sleep system wrapping sleep controller
pub struct SleepSystem {
    /// Sleep controller
    controller: SleepController,
    /// Circadian rhythm
    circadian: CircadianRhythm,
    /// Is currently sleeping
    is_sleeping: bool,
    /// Sleep cycles completed
    cycles_completed: usize,
    /// Total sleep time (in processing cycles)
    total_sleep_cycles: u64,
}

impl SleepSystem {
    /// Create new sleep system
    pub fn new(config: &BrainConfig) -> Self {
        let controller = SleepController::new(
            config.sleep_cycle_length,
            config.sws_ratio,
            config.rem_ratio,
        );

        let circadian = CircadianRhythm::new();

        Self {
            controller,
            circadian,
            is_sleeping: false,
            cycles_completed: 0,
            total_sleep_cycles: 0,
        }
    }

    /// Check if brain should sleep
    pub fn should_sleep(&self) -> bool {
        self.is_sleeping
    }

    /// Initiate sleep
    pub fn initiate_sleep(&mut self) -> Result<()> {
        if self.is_sleeping {
            return Ok(()); // Already sleeping
        }

        self.is_sleeping = true;
        self.controller.start_sleep();
        Ok(())
    }

    /// Wake up from sleep
    pub fn wake_up(&mut self) -> Result<()> {
        if !self.is_sleeping {
            return Ok(()); // Already awake
        }

        self.is_sleeping = false;
        self.controller.end_sleep();
        self.cycles_completed += 1;
        Ok(())
    }

    /// Process a sleep cycle
    pub fn process_cycle(&mut self) -> Result<SleepOutput> {
        if !self.is_sleeping {
            return Ok(SleepOutput {
                is_sws: false,
                is_rem: false,
                replay_count: 0,
                depth: 0.0,
                stage_name: "Awake".to_string(),
            });
        }

        // Advance sleep stage
        let stage = self.controller.advance();
        self.total_sleep_cycles += 1;

        // Determine replay parameters based on stage
        let (is_sws, is_rem, replay_count, depth) = match stage {
            SleepStage::N1 => (false, false, 0, 0.2),
            SleepStage::N2 => (false, false, 5, 0.4),
            SleepStage::N3 => (true, false, 20, 0.9),
            SleepStage::REM => (false, true, 10, 0.3),
            SleepStage::Awake => (false, false, 0, 0.0),
        };

        // Check if should naturally wake
        if self.controller.should_wake() {
            self.wake_up()?;
        }

        Ok(SleepOutput {
            is_sws,
            is_rem,
            replay_count,
            depth,
            stage_name: stage.name().to_string(),
        })
    }

    /// Get current stage name
    pub fn current_stage_name(&self) -> Option<String> {
        if self.is_sleeping {
            Some(self.controller.current_stage().name().to_string())
        } else {
            None
        }
    }

    /// Is in REM sleep
    pub fn is_rem(&self) -> bool {
        self.is_sleeping && self.controller.current_stage() == SleepStage::REM
    }

    /// Is in SWS
    pub fn is_sws(&self) -> bool {
        self.is_sleeping && self.controller.current_stage() == SleepStage::N3
    }

    /// Get sleep depth
    pub fn sleep_depth(&self) -> f64 {
        if !self.is_sleeping {
            return 0.0;
        }
        self.controller.sleep_depth()
    }

    /// Get cycles completed
    pub fn cycles_completed(&self) -> usize {
        self.cycles_completed
    }

    /// Get circadian phase
    pub fn circadian_phase(&self) -> f64 {
        self.circadian.current_phase()
    }

    /// Update circadian rhythm
    pub fn update_circadian(&mut self, hours: f64) {
        self.circadian.advance(hours);
    }

    /// Check if circadian rhythm suggests sleep
    pub fn circadian_suggests_sleep(&self) -> bool {
        self.circadian.should_sleep()
    }

    /// Reset the sleep system
    pub fn reset(&mut self) {
        self.controller = SleepController::new(1000, 0.6, 0.25);
        self.circadian = CircadianRhythm::new();
        self.is_sleeping = false;
        self.cycles_completed = 0;
        self.total_sleep_cycles = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sleep_system_creation() {
        let config = BrainConfig::default();
        let system = SleepSystem::new(&config);

        assert!(!system.should_sleep());
    }

    #[test]
    fn test_initiate_sleep() {
        let config = BrainConfig::default();
        let mut system = SleepSystem::new(&config);

        system.initiate_sleep().unwrap();
        assert!(system.should_sleep());
    }

    #[test]
    fn test_wake_up() {
        let config = BrainConfig::default();
        let mut system = SleepSystem::new(&config);

        system.initiate_sleep().unwrap();
        system.wake_up().unwrap();

        assert!(!system.should_sleep());
        assert_eq!(system.cycles_completed(), 1);
    }

    #[test]
    fn test_process_cycle() {
        let config = BrainConfig::minimal();
        let mut system = SleepSystem::new(&config);

        system.initiate_sleep().unwrap();
        let output = system.process_cycle().unwrap();

        assert!(!output.stage_name.is_empty());
    }

    #[test]
    fn test_circadian() {
        let config = BrainConfig::default();
        let mut system = SleepSystem::new(&config);

        let initial_phase = system.circadian_phase();
        system.update_circadian(6.0); // Advance 6 hours

        // Phase should have changed
        assert_ne!(system.circadian_phase(), initial_phase);
    }
}
