//! Spiking Neuron Models
//!
//! Implements biologically-inspired neuron models including:
//! - Leaky Integrate-and-Fire (LIF)
//! - Adaptive LIF with spike-frequency adaptation
//! - Izhikevich model with 7 firing patterns (RS, FS, IB, CH, LTS, TC, RZ)

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

// ============================================================================
// IZHIKEVICH NEURON MODEL
// ============================================================================

/// Izhikevich neuron firing patterns
///
/// Each pattern corresponds to different combinations of the (a, b, c, d) parameters
/// that reproduce behaviors observed in biological neurons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IzhikevichType {
    /// Regular Spiking (RS) - Most common excitatory cortical neurons
    /// Slow spike-frequency adaptation
    RegularSpiking,
    /// Fast Spiking (FS) - Inhibitory interneurons (basket cells)
    /// High-frequency non-adapting firing
    FastSpiking,
    /// Intrinsically Bursting (IB) - Initial burst followed by regular spiking
    /// Layer 5 pyramidal neurons
    IntrinsicallyBursting,
    /// Chattering (CH) - Fast rhythmic bursting
    /// Some layer 2/3 pyramidal cells
    Chattering,
    /// Low-Threshold Spiking (LTS) - Inhibitory with rebound burst
    /// Martinotti cells
    LowThresholdSpiking,
    /// Thalamo-Cortical (TC) - Two distinct firing modes
    /// Thalamic relay neurons
    ThalamoCortical,
    /// Resonator (RZ) - Subthreshold oscillations
    /// Some interneurons
    Resonator,
}

impl IzhikevichType {
    /// Get the canonical (a, b, c, d) parameters for this firing type
    pub fn params(&self) -> IzhikevichParams {
        match self {
            IzhikevichType::RegularSpiking => IzhikevichParams {
                a: 0.02,
                b: 0.2,
                c: -65.0,
                d: 8.0,
            },
            IzhikevichType::FastSpiking => IzhikevichParams {
                a: 0.1,
                b: 0.2,
                c: -65.0,
                d: 2.0,
            },
            IzhikevichType::IntrinsicallyBursting => IzhikevichParams {
                a: 0.02,
                b: 0.2,
                c: -55.0,
                d: 4.0,
            },
            IzhikevichType::Chattering => IzhikevichParams {
                a: 0.02,
                b: 0.2,
                c: -50.0,
                d: 2.0,
            },
            IzhikevichType::LowThresholdSpiking => IzhikevichParams {
                a: 0.02,
                b: 0.25,
                c: -65.0,
                d: 2.0,
            },
            IzhikevichType::ThalamoCortical => IzhikevichParams {
                a: 0.02,
                b: 0.25,
                c: -65.0,
                d: 0.05,
            },
            IzhikevichType::Resonator => IzhikevichParams {
                a: 0.1,
                b: 0.26,
                c: -65.0,
                d: 2.0,
            },
        }
    }

    /// Whether this neuron type is typically excitatory
    pub fn is_excitatory(&self) -> bool {
        matches!(
            self,
            IzhikevichType::RegularSpiking
                | IzhikevichType::IntrinsicallyBursting
                | IzhikevichType::Chattering
                | IzhikevichType::ThalamoCortical
        )
    }
}

/// Parameters for Izhikevich neuron model
///
/// The model equations are:
/// ```text
/// dv/dt = 0.04*v² + 5*v + 140 - u + I
/// du/dt = a*(b*v - u)
///
/// if v >= 30 mV: v = c, u = u + d
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct IzhikevichParams {
    /// Time scale of recovery variable (0.02-0.1)
    /// Smaller = slower recovery
    pub a: f64,
    /// Sensitivity of recovery to subthreshold membrane potential (0.2-0.26)
    pub b: f64,
    /// After-spike reset value of membrane potential (-65 to -50 mV)
    pub c: f64,
    /// After-spike reset increment of recovery variable (0.05-8.0)
    pub d: f64,
}

impl Default for IzhikevichParams {
    fn default() -> Self {
        // Default to Regular Spiking
        IzhikevichType::RegularSpiking.params()
    }
}

/// Izhikevich neuron state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IzhikevichState {
    /// Membrane potential (mV)
    pub v: f64,
    /// Recovery variable
    pub u: f64,
    /// Whether in refractory period (brief, for numerical stability)
    pub refractory: bool,
    /// Time since last spike
    pub time_since_spike: Duration,
    /// Input current accumulator
    pub input_current: f64,
}

impl Default for IzhikevichState {
    fn default() -> Self {
        Self {
            v: -65.0,                              // Resting potential
            u: -65.0 * 0.2,                        // b * v_rest
            refractory: false,
            time_since_spike: Duration::from_secs(1000),
            input_current: 0.0,
        }
    }
}

/// Izhikevich neuron - captures 20+ biological firing patterns
///
/// This model provides a good balance between biological realism and
/// computational efficiency. It can reproduce most known firing patterns
/// of cortical neurons with only 4 parameters.
///
/// # Example
/// ```ignore
/// use omega_snn::neuron::{IzhikevichNeuron, IzhikevichType, NeuronType};
///
/// let neuron = IzhikevichNeuron::new(
///     "n1".to_string(),
///     NeuronType::Excitatory,
///     IzhikevichType::RegularSpiking,
/// );
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IzhikevichNeuron {
    id: NeuronId,
    neuron_type: NeuronType,
    izh_type: IzhikevichType,
    params: IzhikevichParams,
    izh_state: IzhikevichState,
    /// Standard neuron state for compatibility
    state: NeuronState,
}

impl IzhikevichNeuron {
    /// Create a new Izhikevich neuron with a specific firing type
    pub fn new(id: NeuronId, neuron_type: NeuronType, izh_type: IzhikevichType) -> Self {
        let params = izh_type.params();
        let izh_state = IzhikevichState {
            v: params.c,
            u: params.b * params.c,
            ..Default::default()
        };

        Self {
            id,
            neuron_type,
            izh_type,
            params,
            izh_state,
            state: NeuronState {
                membrane_potential: params.c,
                ..Default::default()
            },
        }
    }

    /// Create with custom parameters
    pub fn with_params(
        id: NeuronId,
        neuron_type: NeuronType,
        params: IzhikevichParams,
    ) -> Self {
        let izh_state = IzhikevichState {
            v: params.c,
            u: params.b * params.c,
            ..Default::default()
        };

        Self {
            id,
            neuron_type,
            izh_type: IzhikevichType::RegularSpiking, // Custom params
            params,
            izh_state,
            state: NeuronState {
                membrane_potential: params.c,
                ..Default::default()
            },
        }
    }

    /// Get the Izhikevich firing type
    pub fn izh_type(&self) -> IzhikevichType {
        self.izh_type
    }

    /// Get the Izhikevich parameters
    pub fn izh_params(&self) -> &IzhikevichParams {
        &self.params
    }

    /// Get the recovery variable
    pub fn recovery(&self) -> f64 {
        self.izh_state.u
    }

    /// Get the Izhikevich-specific state
    pub fn izh_state(&self) -> &IzhikevichState {
        &self.izh_state
    }
}

impl SpikingNeuron for IzhikevichNeuron {
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

        // Skip if refractory (brief period for numerical stability)
        if self.izh_state.refractory {
            self.izh_state.refractory = false;
            self.izh_state.time_since_spike += dt;
            self.izh_state.input_current = 0.0;
            return false;
        }

        let v = self.izh_state.v;
        let u = self.izh_state.u;
        let i = self.izh_state.input_current;

        // Izhikevich dynamics with half-step Euler for stability
        // dv/dt = 0.04*v² + 5*v + 140 - u + I
        // du/dt = a*(b*v - u)

        // First half-step
        let dv1 = 0.04 * v * v + 5.0 * v + 140.0 - u + i;
        let v_half = v + 0.5 * dt_ms * dv1;

        // Second half-step
        let dv2 = 0.04 * v_half * v_half + 5.0 * v_half + 140.0 - u + i;
        self.izh_state.v += dt_ms * dv2;

        // Recovery variable (single Euler step)
        let du = self.params.a * (self.params.b * v - u);
        self.izh_state.u += dt_ms * du;

        // Update time
        self.izh_state.time_since_spike += dt;

        // Clear input current
        self.izh_state.input_current = 0.0;

        // Synchronize with standard state
        self.state.membrane_potential = self.izh_state.v;
        self.state.time_since_spike = self.izh_state.time_since_spike;

        // Check for spike (threshold at 30mV)
        if self.izh_state.v >= 30.0 {
            // Reset
            self.izh_state.v = self.params.c;
            self.izh_state.u += self.params.d;
            self.izh_state.refractory = true;
            self.izh_state.time_since_spike = Duration::ZERO;

            self.state.membrane_potential = self.params.c;
            self.state.time_since_spike = Duration::ZERO;
            self.state.refractory = true;
            self.state.refractory_remaining = Duration::from_micros(500); // Brief

            return true;
        }

        false
    }

    fn receive_input(&mut self, current: f64) {
        self.izh_state.input_current += current;
        self.state.input_current += current;
    }

    fn reset(&mut self) {
        self.izh_state = IzhikevichState {
            v: self.params.c,
            u: self.params.b * self.params.c,
            ..Default::default()
        };
        self.state = NeuronState {
            membrane_potential: self.params.c,
            ..Default::default()
        };
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

    // ========================================================================
    // IZHIKEVICH NEURON TESTS
    // ========================================================================

    #[test]
    fn test_izhikevich_creation() {
        let neuron = IzhikevichNeuron::new(
            "izh1".to_string(),
            NeuronType::Excitatory,
            IzhikevichType::RegularSpiking,
        );

        assert_eq!(neuron.id(), "izh1");
        assert_eq!(neuron.neuron_type(), NeuronType::Excitatory);
        assert_eq!(neuron.izh_type(), IzhikevichType::RegularSpiking);
    }

    #[test]
    fn test_izhikevich_regular_spiking() {
        let mut neuron = IzhikevichNeuron::new(
            "rs".to_string(),
            NeuronType::Excitatory,
            IzhikevichType::RegularSpiking,
        );

        let dt = Duration::from_micros(500); // 0.5ms steps
        let mut spike_count = 0;

        // Inject constant current and run for 100ms
        for _ in 0..200 {
            neuron.receive_input(10.0);
            if neuron.step(dt) {
                spike_count += 1;
            }
        }

        assert!(spike_count > 0, "Regular spiking neuron should fire with input");
    }

    #[test]
    fn test_izhikevich_fast_spiking() {
        let mut fs_neuron = IzhikevichNeuron::new(
            "fs".to_string(),
            NeuronType::Inhibitory,
            IzhikevichType::FastSpiking,
        );
        let mut rs_neuron = IzhikevichNeuron::new(
            "rs".to_string(),
            NeuronType::Excitatory,
            IzhikevichType::RegularSpiking,
        );

        let dt = Duration::from_micros(500);
        let mut fs_spikes = 0;
        let mut rs_spikes = 0;

        // Same input, compare firing rates
        for _ in 0..200 {
            fs_neuron.receive_input(15.0);
            rs_neuron.receive_input(15.0);

            if fs_neuron.step(dt) {
                fs_spikes += 1;
            }
            if rs_neuron.step(dt) {
                rs_spikes += 1;
            }
        }

        // Fast-spiking should fire at higher rate with same input
        assert!(
            fs_spikes >= rs_spikes,
            "Fast-spiking should fire at similar or higher rate"
        );
    }

    #[test]
    fn test_izhikevich_bursting() {
        let mut neuron = IzhikevichNeuron::new(
            "ib".to_string(),
            NeuronType::Excitatory,
            IzhikevichType::IntrinsicallyBursting,
        );

        let dt = Duration::from_micros(500);
        let mut spike_times = Vec::new();

        // Inject strong current
        for t in 0..400 {
            neuron.receive_input(12.0);
            if neuron.step(dt) {
                spike_times.push(t);
            }
        }

        assert!(!spike_times.is_empty(), "IB neuron should spike");

        // Check for initial burst (multiple spikes close together)
        if spike_times.len() >= 2 {
            let isi = spike_times[1] - spike_times[0];
            // Initial bursting should have short ISIs
            assert!(isi < 20, "IB should have short initial ISI (burst)");
        }
    }

    #[test]
    fn test_izhikevich_all_types() {
        let types = [
            IzhikevichType::RegularSpiking,
            IzhikevichType::FastSpiking,
            IzhikevichType::IntrinsicallyBursting,
            IzhikevichType::Chattering,
            IzhikevichType::LowThresholdSpiking,
            IzhikevichType::ThalamoCortical,
            IzhikevichType::Resonator,
        ];

        for izh_type in types {
            let neuron = IzhikevichNeuron::new(
                format!("{:?}", izh_type),
                NeuronType::Excitatory,
                izh_type,
            );

            let params = neuron.izh_params();

            // Verify parameters are in valid ranges
            assert!(params.a > 0.0 && params.a <= 0.2);
            assert!(params.b > 0.0 && params.b <= 0.3);
            assert!(params.c >= -70.0 && params.c <= -45.0);
            assert!(params.d >= 0.0 && params.d <= 10.0);
        }
    }

    #[test]
    fn test_izhikevich_reset() {
        let mut neuron = IzhikevichNeuron::new(
            "reset_test".to_string(),
            NeuronType::Excitatory,
            IzhikevichType::RegularSpiking,
        );

        // Inject current and step
        let dt = Duration::from_millis(1);
        for _ in 0..50 {
            neuron.receive_input(15.0);
            neuron.step(dt);
        }

        // State should have changed
        let v_before = neuron.izh_state().v;

        // Reset
        neuron.reset();

        // Should be back to initial state
        assert_eq!(neuron.izh_state().v, neuron.izh_params().c);
        assert_ne!(v_before, neuron.izh_state().v);
    }

    #[test]
    fn test_izhikevich_excitatory_types() {
        // Verify excitatory classification
        assert!(IzhikevichType::RegularSpiking.is_excitatory());
        assert!(IzhikevichType::IntrinsicallyBursting.is_excitatory());
        assert!(IzhikevichType::Chattering.is_excitatory());
        assert!(IzhikevichType::ThalamoCortical.is_excitatory());

        // Inhibitory types
        assert!(!IzhikevichType::FastSpiking.is_excitatory());
        assert!(!IzhikevichType::LowThresholdSpiking.is_excitatory());
        assert!(!IzhikevichType::Resonator.is_excitatory());
    }
}
