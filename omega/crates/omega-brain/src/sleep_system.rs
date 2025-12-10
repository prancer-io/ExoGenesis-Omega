//! Sleep System - Self-contained sleep stages and circadian rhythm

use crate::{BrainConfig, Result};
use serde::{Deserialize, Serialize};

/// Sleep stages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SleepStage { Awake, N1, N2, N3, REM }

impl SleepStage {
    pub fn name(&self) -> &str {
        match self { Self::Awake => "Awake", Self::N1 => "N1", Self::N2 => "N2", Self::N3 => "N3", Self::REM => "REM" }
    }
}

/// Sleep controller
#[derive(Debug, Clone)]
pub struct SleepController {
    stage: SleepStage,
    cycle_position: usize,
    cycle_length: usize,
    sws_ratio: f64,
    rem_ratio: f64,
}

impl SleepController {
    pub fn new() -> Self {
        Self { stage: SleepStage::Awake, cycle_position: 0, cycle_length: 1000, sws_ratio: 0.6, rem_ratio: 0.25 }
    }
    pub fn start_sleep(&mut self) { self.stage = SleepStage::N1; self.cycle_position = 0; }
    pub fn end_sleep(&mut self) { self.stage = SleepStage::Awake; }
    pub fn advance(&mut self) -> SleepStage {
        self.cycle_position += 1;
        let p = self.cycle_position as f64 / self.cycle_length as f64;
        self.stage = if p < 0.1 { SleepStage::N1 }
            else if p < 0.3 { SleepStage::N2 }
            else if p < 0.3 + self.sws_ratio * 0.5 { SleepStage::N3 }
            else if p < 1.0 - self.rem_ratio { SleepStage::N2 }
            else { SleepStage::REM };
        self.stage
    }
    pub fn current_stage(&self) -> SleepStage { self.stage }
    pub fn should_wake(&self) -> bool { self.cycle_position >= self.cycle_length }
    pub fn sleep_depth(&self) -> f64 {
        match self.stage { SleepStage::Awake => 0.0, SleepStage::N1 => 0.2, SleepStage::N2 => 0.4, SleepStage::N3 => 0.9, SleepStage::REM => 0.3 }
    }
}

impl Default for SleepController {
    fn default() -> Self { Self::new() }
}

/// Circadian rhythm
#[derive(Debug, Clone)]
pub struct CircadianRhythm {
    phase: f64,
}

impl CircadianRhythm {
    pub fn new() -> Self { Self { phase: 0.5 } }
    pub fn current_phase(&self) -> f64 { self.phase }
    pub fn advance(&mut self, hours: f64) { self.phase = (self.phase + hours / 24.0) % 1.0; }
    pub fn should_sleep(&self) -> bool { self.phase > 0.75 || self.phase < 0.25 }
}

impl Default for CircadianRhythm {
    fn default() -> Self { Self::new() }
}

/// Sleep output
#[derive(Debug, Clone)]
pub struct SleepOutput {
    pub is_sws: bool,
    pub is_rem: bool,
    pub replay_count: usize,
    pub depth: f64,
    pub stage_name: String,
}

/// Sleep system
pub struct SleepSystem {
    controller: SleepController,
    circadian: CircadianRhythm,
    is_sleeping: bool,
    cycles_completed: usize,
}

impl SleepSystem {
    pub fn new(_config: &BrainConfig) -> Self {
        Self { controller: SleepController::new(), circadian: CircadianRhythm::new(), is_sleeping: false, cycles_completed: 0 }
    }
    pub fn should_sleep(&self) -> bool { self.is_sleeping }
    pub fn initiate_sleep(&mut self) -> Result<()> {
        if !self.is_sleeping { self.is_sleeping = true; self.controller.start_sleep(); }
        Ok(())
    }
    pub fn wake_up(&mut self) -> Result<()> {
        if self.is_sleeping { self.is_sleeping = false; self.controller.end_sleep(); self.cycles_completed += 1; }
        Ok(())
    }
    pub fn process_cycle(&mut self) -> Result<SleepOutput> {
        if !self.is_sleeping {
            return Ok(SleepOutput { is_sws: false, is_rem: false, replay_count: 0, depth: 0.0, stage_name: "Awake".to_string() });
        }
        let stage = self.controller.advance();
        let (is_sws, is_rem, replay_count, depth) = match stage {
            SleepStage::N1 => (false, false, 0, 0.2),
            SleepStage::N2 => (false, false, 5, 0.4),
            SleepStage::N3 => (true, false, 20, 0.9),
            SleepStage::REM => (false, true, 10, 0.3),
            SleepStage::Awake => (false, false, 0, 0.0),
        };
        if self.controller.should_wake() { self.wake_up()?; }
        Ok(SleepOutput { is_sws, is_rem, replay_count, depth, stage_name: stage.name().to_string() })
    }
    pub fn current_stage_name(&self) -> Option<String> {
        if self.is_sleeping { Some(self.controller.current_stage().name().to_string()) } else { None }
    }
    pub fn is_rem(&self) -> bool { self.is_sleeping && self.controller.current_stage() == SleepStage::REM }
    pub fn is_sws(&self) -> bool { self.is_sleeping && self.controller.current_stage() == SleepStage::N3 }
    pub fn sleep_depth(&self) -> f64 { if !self.is_sleeping { 0.0 } else { self.controller.sleep_depth() } }
    pub fn cycles_completed(&self) -> usize { self.cycles_completed }
    pub fn circadian_phase(&self) -> f64 { self.circadian.current_phase() }
    pub fn update_circadian(&mut self, hours: f64) { self.circadian.advance(hours); }
    pub fn circadian_suggests_sleep(&self) -> bool { self.circadian.should_sleep() }
    pub fn reset(&mut self) {
        self.controller = SleepController::new();
        self.circadian = CircadianRhythm::new();
        self.is_sleeping = false;
        self.cycles_completed = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sleep_controller() {
        let mut ctrl = SleepController::new();
        ctrl.start_sleep();
        assert_eq!(ctrl.current_stage(), SleepStage::N1);
    }
}
