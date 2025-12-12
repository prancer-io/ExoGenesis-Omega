//! Neuromodulator System
//!
//! Implements the major neuromodulatory systems:
//! - Dopamine (DA): Reward, motivation, learning
//! - Norepinephrine (NE): Arousal, attention, uncertainty
//! - Serotonin (5-HT): Mood, behavioral inhibition
//! - Acetylcholine (ACh): Attention, memory encoding

use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::network::SpikingNetwork;
use crate::spike_train::Spike;
use crate::NeuromodulatorLevels;

/// Type of neuromodulator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NeuromodulatorType {
    Dopamine,
    Norepinephrine,
    Serotonin,
    Acetylcholine,
}

/// Base trait for neuromodulator states
pub trait Neuromodulator: Send + Sync {
    /// Get the type of this neuromodulator
    fn modulator_type(&self) -> NeuromodulatorType;

    /// Get current tonic (baseline) level
    fn tonic_level(&self) -> f64;

    /// Get current phasic (burst) level
    fn phasic_level(&self) -> f64;

    /// Get combined effective level
    fn effective_level(&self) -> f64 {
        self.tonic_level() + self.phasic_level()
    }

    /// Update state for one time step
    fn update(&mut self, dt: Duration);

    /// Trigger phasic burst
    fn trigger_burst(&mut self, magnitude: f64);

    /// Set tonic level
    fn set_tonic(&mut self, level: f64);
}

/// Dopamine state - reward prediction error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DopamineState {
    /// Tonic (baseline) level
    pub tonic: f64,
    /// Phasic (burst) level
    pub phasic: f64,
    /// Phasic decay time constant (ms)
    pub tau_phasic: f64,
    /// Value prediction (for TD learning)
    pub value_prediction: f64,
    /// Actual reward received
    pub reward: f64,
}

impl Default for DopamineState {
    fn default() -> Self {
        Self {
            tonic: 0.5,       // Baseline dopamine
            phasic: 0.0,      // No burst
            tau_phasic: 200.0, // 200ms phasic decay
            value_prediction: 0.0,
            reward: 0.0,
        }
    }
}

impl DopamineState {
    /// Compute reward prediction error (TD error)
    pub fn compute_rpe(&self, reward: f64, next_value: f64, gamma: f64) -> f64 {
        reward + gamma * next_value - self.value_prediction
    }

    /// Update with reward signal
    pub fn signal_reward(&mut self, reward: f64, next_value: f64) {
        self.reward = reward;
        let rpe = self.compute_rpe(reward, next_value, 0.99);

        // Positive RPE -> dopamine burst
        // Negative RPE -> dopamine dip
        if rpe > 0.0 {
            self.trigger_burst(rpe.min(1.0));
        } else {
            self.phasic = (self.phasic + rpe).max(-0.5);
        }
    }
}

impl Neuromodulator for DopamineState {
    fn modulator_type(&self) -> NeuromodulatorType {
        NeuromodulatorType::Dopamine
    }

    fn tonic_level(&self) -> f64 {
        self.tonic
    }

    fn phasic_level(&self) -> f64 {
        self.phasic
    }

    fn update(&mut self, dt: Duration) {
        let dt_ms = dt.as_secs_f64() * 1000.0;

        // Decay phasic towards zero
        self.phasic *= (-dt_ms / self.tau_phasic).exp();
    }

    fn trigger_burst(&mut self, magnitude: f64) {
        self.phasic = (self.phasic + magnitude).min(1.0);
    }

    fn set_tonic(&mut self, level: f64) {
        self.tonic = level.clamp(0.0, 1.0);
    }
}

/// Norepinephrine state - arousal and uncertainty
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NorepinephrineState {
    /// Tonic (baseline) level
    pub tonic: f64,
    /// Phasic (burst) level
    pub phasic: f64,
    /// Phasic decay time constant (ms)
    pub tau_phasic: f64,
    /// Uncertainty/surprise signal
    pub uncertainty: f64,
    /// Exploration vs exploitation balance
    pub exploration_bonus: f64,
}

impl Default for NorepinephrineState {
    fn default() -> Self {
        Self {
            tonic: 0.3,
            phasic: 0.0,
            tau_phasic: 100.0,
            uncertainty: 0.0,
            exploration_bonus: 0.0,
        }
    }
}

impl NorepinephrineState {
    /// Signal unexpected event (drives exploration)
    pub fn signal_surprise(&mut self, surprise: f64) {
        self.uncertainty = surprise;
        self.trigger_burst(surprise.min(1.0));

        // High uncertainty -> more exploration
        self.exploration_bonus = surprise * 0.5;
    }

    /// Get gain modulation factor
    pub fn gain_modulation(&self) -> f64 {
        // NE increases neural gain (sensitivity)
        1.0 + self.effective_level()
    }
}

impl Neuromodulator for NorepinephrineState {
    fn modulator_type(&self) -> NeuromodulatorType {
        NeuromodulatorType::Norepinephrine
    }

    fn tonic_level(&self) -> f64 {
        self.tonic
    }

    fn phasic_level(&self) -> f64 {
        self.phasic
    }

    fn update(&mut self, dt: Duration) {
        let dt_ms = dt.as_secs_f64() * 1000.0;

        self.phasic *= (-dt_ms / self.tau_phasic).exp();
        self.uncertainty *= 0.99; // Slow decay
        self.exploration_bonus *= 0.995;
    }

    fn trigger_burst(&mut self, magnitude: f64) {
        self.phasic = (self.phasic + magnitude).min(1.0);
    }

    fn set_tonic(&mut self, level: f64) {
        self.tonic = level.clamp(0.0, 1.0);
    }
}

/// Serotonin state - mood and inhibition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerotoninState {
    /// Tonic (baseline) level
    pub tonic: f64,
    /// Phasic level
    pub phasic: f64,
    /// Decay time constant (ms)
    pub tau_phasic: f64,
    /// Inhibition strength
    pub inhibition: f64,
    /// Patience/waiting ability
    pub patience: f64,
}

impl Default for SerotoninState {
    fn default() -> Self {
        Self {
            tonic: 0.5,
            phasic: 0.0,
            tau_phasic: 500.0, // Slower dynamics
            inhibition: 0.5,
            patience: 0.5,
        }
    }
}

impl SerotoninState {
    /// Get behavioral inhibition factor
    pub fn inhibition_factor(&self) -> f64 {
        self.effective_level() * self.inhibition
    }

    /// Modulate impulsivity (high 5-HT = less impulsive)
    pub fn impulsivity(&self) -> f64 {
        1.0 - self.effective_level()
    }
}

impl Neuromodulator for SerotoninState {
    fn modulator_type(&self) -> NeuromodulatorType {
        NeuromodulatorType::Serotonin
    }

    fn tonic_level(&self) -> f64 {
        self.tonic
    }

    fn phasic_level(&self) -> f64 {
        self.phasic
    }

    fn update(&mut self, dt: Duration) {
        let dt_ms = dt.as_secs_f64() * 1000.0;

        self.phasic *= (-dt_ms / self.tau_phasic).exp();
    }

    fn trigger_burst(&mut self, magnitude: f64) {
        self.phasic = (self.phasic + magnitude).min(1.0);
    }

    fn set_tonic(&mut self, level: f64) {
        self.tonic = level.clamp(0.0, 1.0);
    }
}

/// Acetylcholine state - attention and memory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcetylcholineState {
    /// Tonic (baseline) level
    pub tonic: f64,
    /// Phasic level
    pub phasic: f64,
    /// Decay time constant (ms)
    pub tau_phasic: f64,
    /// Attention signal
    pub attention: f64,
    /// Memory encoding strength
    pub encoding_strength: f64,
}

impl Default for AcetylcholineState {
    fn default() -> Self {
        Self {
            tonic: 0.4,
            phasic: 0.0,
            tau_phasic: 300.0,
            attention: 0.5,
            encoding_strength: 0.5,
        }
    }
}

impl AcetylcholineState {
    /// Signal attention focus
    pub fn focus_attention(&mut self, target_salience: f64) {
        self.attention = target_salience;
        self.trigger_burst(target_salience * 0.5);
    }

    /// Get memory encoding modulation
    pub fn encoding_modulation(&self) -> f64 {
        // High ACh = better encoding
        1.0 + self.effective_level()
    }

    /// Get retrieval modulation (inverse of encoding)
    pub fn retrieval_modulation(&self) -> f64 {
        // Low ACh = better retrieval (less interference)
        2.0 - self.effective_level()
    }
}

impl Neuromodulator for AcetylcholineState {
    fn modulator_type(&self) -> NeuromodulatorType {
        NeuromodulatorType::Acetylcholine
    }

    fn tonic_level(&self) -> f64 {
        self.tonic
    }

    fn phasic_level(&self) -> f64 {
        self.phasic
    }

    fn update(&mut self, dt: Duration) {
        let dt_ms = dt.as_secs_f64() * 1000.0;

        self.phasic *= (-dt_ms / self.tau_phasic).exp();
        self.attention *= 0.99;
    }

    fn trigger_burst(&mut self, magnitude: f64) {
        self.phasic = (self.phasic + magnitude).min(1.0);
    }

    fn set_tonic(&mut self, level: f64) {
        self.tonic = level.clamp(0.0, 1.0);
    }
}

/// Complete neuromodulator system
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NeuromodulatorSystem {
    pub dopamine: DopamineState,
    pub norepinephrine: NorepinephrineState,
    pub serotonin: SerotoninState,
    pub acetylcholine: AcetylcholineState,
}

impl NeuromodulatorSystem {
    pub fn new() -> Self {
        Self::default()
    }

    /// Update all neuromodulators
    pub fn update(&mut self, dt: Duration, spikes: &[Spike]) {
        // Update based on activity levels
        let activity = spikes.len() as f64 / 100.0; // Normalize

        // High activity can trigger NE (arousal)
        if activity > 0.5 {
            self.norepinephrine.trigger_burst(activity * 0.1);
        }

        // Update all modulators
        self.dopamine.update(dt);
        self.norepinephrine.update(dt);
        self.serotonin.update(dt);
        self.acetylcholine.update(dt);
    }

    /// Get current levels for all modulators
    pub fn levels(&self) -> NeuromodulatorLevels {
        NeuromodulatorLevels {
            dopamine: self.dopamine.effective_level(),
            norepinephrine: self.norepinephrine.effective_level(),
            serotonin: self.serotonin.effective_level(),
            acetylcholine: self.acetylcholine.effective_level(),
        }
    }

    /// Set level for a specific modulator
    pub fn set_level(&mut self, modulator: NeuromodulatorType, level: f64) {
        match modulator {
            NeuromodulatorType::Dopamine => self.dopamine.set_tonic(level),
            NeuromodulatorType::Norepinephrine => self.norepinephrine.set_tonic(level),
            NeuromodulatorType::Serotonin => self.serotonin.set_tonic(level),
            NeuromodulatorType::Acetylcholine => self.acetylcholine.set_tonic(level),
        }
    }

    /// Apply neuromodulator effects to network
    pub fn modulate_network(&self, network: &mut SpikingNetwork) {
        let da = self.dopamine.effective_level();
        let ne = self.norepinephrine.effective_level();
        let ach = self.acetylcholine.effective_level();

        // Modulate plasticity via dopamine
        network.modulate_plasticity(da);

        // Modulate gain via norepinephrine
        network.modulate_gain(ne);

        // Modulate encoding via acetylcholine
        network.modulate_encoding(ach);
    }

    /// Signal reward (updates dopamine)
    pub fn signal_reward(&mut self, reward: f64) {
        self.dopamine.signal_reward(reward, 0.0);
    }

    /// Signal surprise (updates norepinephrine)
    pub fn signal_surprise(&mut self, surprise: f64) {
        self.norepinephrine.signal_surprise(surprise);
    }

    /// Focus attention (updates acetylcholine)
    pub fn focus_attention(&mut self, salience: f64) {
        self.acetylcholine.focus_attention(salience);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dopamine_rpe() {
        let mut da = DopamineState::default();

        // Positive surprise (better than expected)
        da.value_prediction = 0.5;
        da.signal_reward(1.0, 0.0);

        assert!(da.phasic > 0.0, "Positive RPE should cause burst");
    }

    #[test]
    fn test_norepinephrine_surprise() {
        let mut ne = NorepinephrineState::default();

        let initial = ne.effective_level();
        ne.signal_surprise(0.8);

        assert!(ne.effective_level() > initial);
        assert!(ne.exploration_bonus > 0.0);
    }

    #[test]
    fn test_acetylcholine_attention() {
        let mut ach = AcetylcholineState::default();

        ach.focus_attention(0.9);

        assert!(ach.attention > 0.5);
        assert!(ach.phasic > 0.0);
    }

    #[test]
    fn test_neuromodulator_system() {
        let mut system = NeuromodulatorSystem::new();

        let levels = system.levels();
        assert!(levels.dopamine > 0.0);
        assert!(levels.norepinephrine > 0.0);

        system.signal_reward(1.0);
        let new_levels = system.levels();
        assert!(new_levels.dopamine > levels.dopamine);
    }

    #[test]
    fn test_modulator_decay() {
        let mut da = DopamineState::default();
        da.trigger_burst(0.5);

        let initial = da.phasic;
        da.update(Duration::from_millis(100));

        assert!(da.phasic < initial, "Phasic should decay over time");
    }
}
