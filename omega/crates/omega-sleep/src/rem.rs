//! REM Sleep
//!
//! Rapid Eye Movement sleep characterized by:
//! - Dreaming
//! - Muscle atonia
//! - Theta waves (4-8 Hz)
//! - Memory reorganization
//! - Emotional processing

use rand::Rng;
use serde::{Deserialize, Serialize};

/// Dream content representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamContent {
    /// Dream intensity (0-1)
    pub intensity: f64,
    /// Emotional valence (-1 to 1)
    pub valence: f64,
    /// Bizarreness level (0-1)
    pub bizarreness: f64,
    /// Memory elements incorporated
    pub memory_elements: Vec<String>,
    /// Narrative coherence
    pub coherence: f64,
    /// Duration (subjective, seconds)
    pub duration: f64,
    /// Timestamp
    pub timestamp: u64,
}

impl DreamContent {
    pub fn generate(memories: &[String]) -> Self {
        let mut rng = rand::thread_rng();

        // Sample some memories for dream content
        let num_elements = rng.gen_range(2..=memories.len().clamp(2, 5));
        let mut memory_elements = Vec::new();

        for _ in 0..num_elements {
            if !memories.is_empty() {
                let idx = rng.gen_range(0..memories.len());
                memory_elements.push(memories[idx].clone());
            }
        }

        Self {
            intensity: rng.gen_range(0.3..1.0),
            valence: rng.gen_range(-1.0..1.0),
            bizarreness: rng.gen_range(0.2..0.9),
            memory_elements,
            coherence: rng.gen_range(0.1..0.7),
            duration: rng.gen_range(10.0..300.0),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }

    /// Is this a nightmare?
    pub fn is_nightmare(&self) -> bool {
        self.valence < -0.5 && self.intensity > 0.7
    }

    /// Is this a lucid dream?
    pub fn is_lucid(&self) -> bool {
        self.coherence > 0.6 && self.bizarreness < 0.3
    }
}

/// REM burst (rapid eye movement)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct REMBurst {
    /// Number of rapid eye movements
    pub count: usize,
    /// Duration (ms)
    pub duration: f64,
    /// Associated dream intensity
    pub dream_intensity: f64,
}

/// REM sleep processor
pub struct REMSleep {
    /// Current REM density
    rem_density: f64,
    /// Time in REM (minutes)
    time_in_rem: f64,
    /// Theta power (4-8 Hz)
    theta_power: f64,
    /// Ponto-geniculo-occipital (PGO) waves
    pgo_activity: f64,
    /// Dream generation rate
    dream_rate: f64,
    /// Time until next dream segment
    next_dream_time: f64,
    /// Available memories for dreaming
    memory_pool: Vec<String>,
    /// Dream count this session
    dream_count: usize,
}

impl REMSleep {
    pub fn new() -> Self {
        Self {
            rem_density: 0.0,
            time_in_rem: 0.0,
            theta_power: 0.0,
            pgo_activity: 0.0,
            dream_rate: 0.5, // Dreams per minute
            next_dream_time: 0.0,
            memory_pool: Vec::new(),
            dream_count: 0,
        }
    }

    /// Add memories to the dream pool
    pub fn add_memories(&mut self, memories: Vec<String>) {
        self.memory_pool.extend(memories);
    }

    /// Step forward and possibly generate dream content
    pub fn step(&mut self, dt_minutes: f64) -> Option<DreamContent> {
        self.time_in_rem += dt_minutes;
        self.next_dream_time -= dt_minutes;

        // Update REM characteristics
        let mut rng = rand::thread_rng();

        // REM density increases through REM period
        self.rem_density = (self.time_in_rem / 20.0).min(1.0);

        // Theta power fluctuates
        self.theta_power = 0.5 + 0.3 * (self.time_in_rem * 0.1).sin() + rng.gen_range(-0.1..0.1);
        self.theta_power = self.theta_power.clamp(0.0, 1.0);

        // PGO activity bursts
        if rng.gen::<f64>() < 0.1 {
            self.pgo_activity = rng.gen_range(0.5..1.0);
        } else {
            self.pgo_activity *= 0.9;
        }

        // Generate dream content
        if self.next_dream_time <= 0.0 {
            self.dream_count += 1;
            self.next_dream_time = 1.0 / self.dream_rate + rng.gen_range(-0.3..0.3);

            // Generate dream using memory pool or placeholders
            let memories = if self.memory_pool.is_empty() {
                vec!["memory_1".to_string(), "memory_2".to_string()]
            } else {
                self.memory_pool.clone()
            };

            Some(DreamContent::generate(&memories))
        } else {
            None
        }
    }

    /// Get current REM density
    pub fn rem_density(&self) -> f64 {
        self.rem_density
    }

    /// Get theta power
    pub fn theta_power(&self) -> f64 {
        self.theta_power
    }

    /// Get PGO activity
    pub fn pgo_activity(&self) -> f64 {
        self.pgo_activity
    }

    /// Get dream count
    pub fn dream_count(&self) -> usize {
        self.dream_count
    }

    /// Reset for new REM period
    pub fn reset(&mut self) {
        self.rem_density = 0.0;
        self.time_in_rem = 0.0;
        self.theta_power = 0.0;
        self.pgo_activity = 0.0;
        self.next_dream_time = 0.0;
        self.dream_count = 0;
    }
}

impl Default for REMSleep {
    fn default() -> Self {
        Self::new()
    }
}

/// REM behavior state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct REMState {
    /// Theta oscillation phase
    pub theta_phase: f64,
    /// Muscle tone (0 = atonia, 1 = normal)
    pub muscle_tone: f64,
    /// Eye movement density
    pub eye_movement_density: f64,
    /// Dream state active
    pub dreaming: bool,
}

impl REMState {
    pub fn new() -> Self {
        Self {
            theta_phase: 0.0,
            muscle_tone: 0.0, // Atonia in REM
            eye_movement_density: 0.0,
            dreaming: false,
        }
    }

    /// Update state
    pub fn update(&mut self, dt_seconds: f64, rem_density: f64) {
        // Theta oscillation at ~6 Hz
        let omega = 2.0 * std::f64::consts::PI * 6.0;
        self.theta_phase += omega * dt_seconds;
        if self.theta_phase > 2.0 * std::f64::consts::PI {
            self.theta_phase -= 2.0 * std::f64::consts::PI;
        }

        // Muscle tone remains low during REM
        self.muscle_tone = 0.05;

        // Eye movements correlate with REM density
        self.eye_movement_density = rem_density;

        // Dreaming is usually active during REM
        self.dreaming = rem_density > 0.2;
    }
}

impl Default for REMState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dream_content() {
        let memories = vec!["mem1".to_string(), "mem2".to_string(), "mem3".to_string()];
        let dream = DreamContent::generate(&memories);

        assert!(!dream.memory_elements.is_empty());
        assert!(dream.intensity >= 0.0 && dream.intensity <= 1.0);
    }

    #[test]
    fn test_rem_processor() {
        let mut rem = REMSleep::new();

        let mut dreams = 0;
        for _ in 0..30 {
            if rem.step(1.0).is_some() {
                dreams += 1;
            }
        }

        // Should generate ~15 dream segments in 30 minutes
        assert!(dreams > 5);
    }

    #[test]
    fn test_rem_density() {
        let mut rem = REMSleep::new();

        for _ in 0..20 {
            rem.step(1.0);
        }

        assert!(rem.rem_density() > 0.5);
    }

    #[test]
    fn test_rem_state() {
        let mut state = REMState::new();

        state.update(0.1, 0.8);

        assert!(state.muscle_tone < 0.1); // Atonia
        assert!(state.dreaming);
    }

    #[test]
    fn test_nightmare_detection() {
        let mut dream = DreamContent::generate(&vec!["test".to_string()]);
        dream.valence = -0.8;
        dream.intensity = 0.9;

        assert!(dream.is_nightmare());
    }
}
