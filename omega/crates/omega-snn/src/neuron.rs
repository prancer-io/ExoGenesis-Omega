//! Spiking Neuron Models
//!
//! Implements biologically-inspired neuron models including:
//! - Leaky Integrate-and-Fire (LIF)
//! - Adaptive LIF with spike-frequency adaptation
//! - Izhikevich model (future)

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Unique identifier for a neuron
pub type NeuronId = String;

/// Type of neuron (affects connectivity and dynamics)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NeuronType {
    /// Excitatory pyramidal neuron (glutamatergic)
    Excitatory,
    /// Inhibitory interneuron (GABAergic)
    Inhibitory,
    /// Sensory input neuron
    Sensory,
    /// Motor output neuron
    Motor,
    /// Modulatory neuron (dopaminergic, etc.)
    Modulatory,
}

impl NeuronType {
    /// Whether this neuron type produces excitatory output
    pub fn is_excitatory(&self) -> bool {
        matches!(self, NeuronType::Excitatory | NeuronType::Sensory | NeuronType::Motor)
    }

    /// Typical ratio in cortex (80% excitatory, 20% inhibitory)
    pub fn cortical_ratio(&self) -> f64 {
        match self {
            NeuronType::Excitatory => 0.80,
            NeuronType::Inhibitory => 0.20,
            _ => 0.0,
        }
    }
}

/// Current state of a neuron
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuronState {
    /// Membrane potential (mV)
    pub membrane_potential: f64,
    /// Whether neuron is in refractory period
    pub refractory: bool,
    /// Time remaining in refractory period
    pub refractory_remaining: Duration,
    /// Adaptation current (for adaptive neurons)
    pub adaptation: f64,
    /// Input current accumulator
    pub input_current: f64,
    /// Time since last spike
    pub time_since_spike: Duration,
}

impl Default for NeuronState {
    fn default() -> Self {
        Self {
            membrane_potential: -70.0, // Resting potential in mV
            refractory: false,
            refractory_remaining: Duration::ZERO,
            adaptation: 0.0,
            input_current: 0.0,
            time_since_spike: Duration::from_secs(1000), // Large value = no recent spike
        }
    }
}

/// Trait for all spiking neuron models
pub trait SpikingNeuron: Send + Sync {
    /// Get the neuron's unique ID
    fn id(&self) -> &NeuronId;

    /// Get the neuron type
    fn neuron_type(&self) -> NeuronType;

    /// Get current state
    fn state(&self) -> &NeuronState;

    /// Get mutable state
    fn state_mut(&mut self) -> &mut NeuronState;

    /// Update neuron for one time step, returns true if spike occurred
    fn step(&mut self, dt: Duration) -> bool;

    /// Receive input current from synapse
    fn receive_input(&mut self, current: f64);

    /// Reset after spike
    fn reset(&mut self);

    /// Check if neuron can spike (not in refractory)
    fn can_spike(&self) -> bool {
        !self.state().refractory
    }

    /// Get membrane potential
    fn membrane_potential(&self) -> f64 {
        self.state().membrane_potential
    }
}

/// Parameters for Leaky Integrate-and-Fire neuron
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LIFParams {
    /// Membrane time constant (ms)
    pub tau_m: f64,
    /// Membrane resistance (MOhm)
    pub r_m: f64,
    /// Resting potential (mV)
    pub v_rest: f64,
    /// Threshold potential (mV)
    pub v_thresh: f64,
    /// Reset potential (mV)
    pub v_reset: f64,
    /// Refractory period (ms)
    pub t_ref: f64,
}

impl Default for LIFParams {
    fn default() -> Self {
        Self {
            tau_m: 20.0,      // 20ms membrane time constant
            r_m: 10.0,        // 10 MOhm membrane resistance
            v_rest: -70.0,    // -70mV resting potential
            v_thresh: -55.0,  // -55mV threshold
            v_reset: -75.0,   // -75mV reset (hyperpolarized)
            t_ref: 2.0,       // 2ms refractory period
        }
    }
}

/// Leaky Integrate-and-Fire neuron
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LIFNeuron {
    id: NeuronId,
    neuron_type: NeuronType,
    params: LIFParams,
    state: NeuronState,
}

impl LIFNeuron {
    /// Create a new LIF neuron
    pub fn new(id: NeuronId, neuron_type: NeuronType, params: LIFParams) -> Self {
        Self {
            id,
            neuron_type,
            state: NeuronState {
                membrane_potential: params.v_rest,
                ..Default::default()
            },
            params,
        }
    }

    /// Create with default parameters
    pub fn with_defaults(id: NeuronId, neuron_type: NeuronType) -> Self {
        Self::new(id, neuron_type, LIFParams::default())
    }

    /// Get parameters
    pub fn params(&self) -> &LIFParams {
        &self.params
    }
}

impl SpikingNeuron for LIFNeuron {
    fn id(&self) -> &NeuronId {
        &self.id
    }

    fn neuron_type(&self) -> NeuronType {
        self.neuron_type
    }

    fn state(&self) -> &NeuronState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut NeuronState {
        &mut self.state
    }

    fn step(&mut self, dt: Duration) -> bool {
        let dt_ms = dt.as_secs_f64() * 1000.0;

        // Update refractory period
        if self.state.refractory {
            if self.state.refractory_remaining > dt {
                self.state.refractory_remaining -= dt;
                self.state.time_since_spike += dt;
                self.state.input_current = 0.0;
                return false;
            } else {
                self.state.refractory = false;
                self.state.refractory_remaining = Duration::ZERO;
            }
        }

        // LIF dynamics: tau_m * dV/dt = -(V - V_rest) + R_m * I
        let dv = dt_ms / self.params.tau_m
            * (-(self.state.membrane_potential - self.params.v_rest)
                + self.params.r_m * self.state.input_current);

        self.state.membrane_potential += dv;
        self.state.time_since_spike += dt;

        // Clear input current after processing
        self.state.input_current = 0.0;

        // Check for spike
        if self.state.membrane_potential >= self.params.v_thresh {
            self.reset();
            return true;
        }

        false
    }

    fn receive_input(&mut self, current: f64) {
        self.state.input_current += current;
    }

    fn reset(&mut self) {
        self.state.membrane_potential = self.params.v_reset;
        self.state.refractory = true;
        self.state.refractory_remaining = Duration::from_secs_f64(self.params.t_ref / 1000.0);
        self.state.time_since_spike = Duration::ZERO;
    }
}

/// Adaptive LIF neuron with spike-frequency adaptation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveLIFNeuron {
    base: LIFNeuron,
    /// Adaptation time constant (ms)
    tau_w: f64,
    /// Subthreshold adaptation
    a: f64,
    /// Spike-triggered adaptation
    b: f64,
}

impl AdaptiveLIFNeuron {
    /// Create a new adaptive LIF neuron
    pub fn new(id: NeuronId, neuron_type: NeuronType, params: LIFParams) -> Self {
        Self {
            base: LIFNeuron::new(id, neuron_type, params),
            tau_w: 100.0,  // 100ms adaptation time constant
            a: 0.01,       // Subthreshold adaptation
            b: 0.5,        // Spike-triggered adaptation
        }
    }

    /// Create with custom adaptation parameters
    pub fn with_adaptation(
        id: NeuronId,
        neuron_type: NeuronType,
        params: LIFParams,
        tau_w: f64,
        a: f64,
        b: f64,
    ) -> Self {
        Self {
            base: LIFNeuron::new(id, neuron_type, params),
            tau_w,
            a,
            b,
        }
    }
}

impl SpikingNeuron for AdaptiveLIFNeuron {
    fn id(&self) -> &NeuronId {
        self.base.id()
    }

    fn neuron_type(&self) -> NeuronType {
        self.base.neuron_type()
    }

    fn state(&self) -> &NeuronState {
        self.base.state()
    }

    fn state_mut(&mut self) -> &mut NeuronState {
        self.base.state_mut()
    }

    fn step(&mut self, dt: Duration) -> bool {
        let dt_ms = dt.as_secs_f64() * 1000.0;

        // Update adaptation current
        let v = self.base.state.membrane_potential;
        let v_rest = self.base.params.v_rest;
        let w = self.base.state.adaptation;

        // dw/dt = (a*(V - V_rest) - w) / tau_w
        let dw = dt_ms / self.tau_w * (self.a * (v - v_rest) - w);
        self.base.state.adaptation += dw;

        // Subtract adaptation from input current
        self.base.state.input_current -= self.base.state.adaptation;

        // Run base LIF dynamics
        let spiked = self.base.step(dt);

        // Spike-triggered adaptation
        if spiked {
            self.base.state.adaptation += self.b;
        }

        spiked
    }

    fn receive_input(&mut self, current: f64) {
        self.base.receive_input(current);
    }

    fn reset(&mut self) {
        self.base.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lif_neuron_creation() {
        let neuron = LIFNeuron::with_defaults("n1".to_string(), NeuronType::Excitatory);

        assert_eq!(neuron.id(), "n1");
        assert_eq!(neuron.neuron_type(), NeuronType::Excitatory);
        assert!((neuron.membrane_potential() - (-70.0)).abs() < 0.01);
    }

    #[test]
    fn test_lif_neuron_spike() {
        let mut neuron = LIFNeuron::with_defaults("n1".to_string(), NeuronType::Excitatory);

        // Inject strong current
        neuron.receive_input(10.0);

        // Step until spike
        let dt = Duration::from_millis(1);
        let mut spiked = false;
        for _ in 0..100 {
            if neuron.step(dt) {
                spiked = true;
                break;
            }
            neuron.receive_input(10.0);
        }

        assert!(spiked);
        assert!(neuron.state().refractory);
    }

    #[test]
    fn test_lif_refractory_period() {
        let mut neuron = LIFNeuron::with_defaults("n1".to_string(), NeuronType::Excitatory);

        // Force spike
        neuron.state_mut().membrane_potential = -50.0;
        let spiked = neuron.step(Duration::from_millis(1));
        assert!(spiked);

        // Should be refractory
        assert!(neuron.state().refractory);
        assert!(!neuron.can_spike());

        // Inject current during refractory - should not spike
        neuron.receive_input(100.0);
        let spiked = neuron.step(Duration::from_millis(1));
        assert!(!spiked);
    }

    #[test]
    fn test_neuron_type_properties() {
        assert!(NeuronType::Excitatory.is_excitatory());
        assert!(!NeuronType::Inhibitory.is_excitatory());
        assert!(NeuronType::Sensory.is_excitatory());
    }

    #[test]
    fn test_adaptive_lif_adaptation() {
        let mut neuron = AdaptiveLIFNeuron::new(
            "n1".to_string(),
            NeuronType::Excitatory,
            LIFParams::default(),
        );

        // Initial adaptation should be zero
        assert!((neuron.state().adaptation - 0.0).abs() < 0.01);

        // Inject current and step
        let dt = Duration::from_millis(1);
        for _ in 0..10 {
            neuron.receive_input(5.0);
            neuron.step(dt);
        }

        // Adaptation should have changed
        // (depends on parameters and whether spike occurred)
    }
}
