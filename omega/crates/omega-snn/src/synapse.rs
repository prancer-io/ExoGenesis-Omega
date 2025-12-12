//! Synaptic Connections and Plasticity
//!
//! Implements:
//! - Spike-Timing Dependent Plasticity (STDP)
//! - Short-Term Plasticity (facilitation and depression)
//! - Synaptic weight dynamics

use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::neuron::NeuronId;

/// Unique identifier for a synapse
pub type SynapseId = String;

/// STDP rule parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STDPParams {
    /// LTP amplitude (potentiation when pre before post)
    pub a_plus: f64,
    /// LTD amplitude (depression when post before pre)
    pub a_minus: f64,
    /// LTP time constant (ms)
    pub tau_plus: f64,
    /// LTD time constant (ms)
    pub tau_minus: f64,
    /// Maximum weight
    pub w_max: f64,
    /// Minimum weight
    pub w_min: f64,
}

impl Default for STDPParams {
    fn default() -> Self {
        Self {
            a_plus: 0.01,    // 1% potentiation per coincidence
            a_minus: 0.012,  // Slightly stronger depression (for stability)
            tau_plus: 20.0,  // 20ms potentiation window
            tau_minus: 20.0, // 20ms depression window
            w_max: 1.0,
            w_min: 0.0,
        }
    }
}

/// STDP learning rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STDPRule {
    params: STDPParams,
    /// Trace of presynaptic activity
    pre_trace: f64,
    /// Trace of postsynaptic activity
    post_trace: f64,
}

impl STDPRule {
    pub fn new(params: STDPParams) -> Self {
        Self {
            params,
            pre_trace: 0.0,
            post_trace: 0.0,
        }
    }

    /// Update traces and compute weight change
    pub fn update(
        &mut self,
        dt: Duration,
        pre_spike: bool,
        post_spike: bool,
        current_weight: f64,
    ) -> f64 {
        let dt_ms = dt.as_secs_f64() * 1000.0;

        // Decay traces
        self.pre_trace *= (-dt_ms / self.params.tau_plus).exp();
        self.post_trace *= (-dt_ms / self.params.tau_minus).exp();

        let mut dw = 0.0;

        // Pre-spike: check for LTD (post before pre)
        if pre_spike {
            // Depression: post-trace indicates recent post-spike
            dw -= self.params.a_minus * self.post_trace;
            // Update pre-trace
            self.pre_trace += 1.0;
        }

        // Post-spike: check for LTP (pre before post)
        if post_spike {
            // Potentiation: pre-trace indicates recent pre-spike
            dw += self.params.a_plus * self.pre_trace;
            // Update post-trace
            self.post_trace += 1.0;
        }

        // Apply soft bounds
        if dw > 0.0 {
            dw *= self.params.w_max - current_weight;
        } else {
            dw *= current_weight - self.params.w_min;
        }

        dw
    }

    /// Reset traces (e.g., at start of new episode)
    pub fn reset_traces(&mut self) {
        self.pre_trace = 0.0;
        self.post_trace = 0.0;
    }

    pub fn params(&self) -> &STDPParams {
        &self.params
    }
}

/// Short-term plasticity state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortTermPlasticity {
    /// Facilitation variable (1.0 = no facilitation)
    pub facilitation: f64,
    /// Depression variable (1.0 = no depression)
    pub depression: f64,
    /// Facilitation time constant (ms)
    pub tau_f: f64,
    /// Depression time constant (ms)
    pub tau_d: f64,
    /// Facilitation increment per spike
    pub f_increment: f64,
    /// Depression decrement per spike
    pub d_decrement: f64,
}

impl Default for ShortTermPlasticity {
    fn default() -> Self {
        Self {
            facilitation: 1.0,
            depression: 1.0,
            tau_f: 50.0,    // 50ms facilitation decay
            tau_d: 200.0,   // 200ms depression recovery
            f_increment: 0.2,
            d_decrement: 0.1,
        }
    }
}

impl ShortTermPlasticity {
    /// Create facilitating synapse (e.g., cortical-cortical)
    pub fn facilitating() -> Self {
        Self {
            tau_f: 100.0,
            tau_d: 500.0,
            f_increment: 0.3,
            d_decrement: 0.05,
            ..Default::default()
        }
    }

    /// Create depressing synapse (e.g., thalamocortical)
    pub fn depressing() -> Self {
        Self {
            tau_f: 20.0,
            tau_d: 100.0,
            f_increment: 0.05,
            d_decrement: 0.3,
            ..Default::default()
        }
    }

    /// Update STP state
    pub fn update(&mut self, dt: Duration, pre_spike: bool) {
        let dt_ms = dt.as_secs_f64() * 1000.0;

        // Decay towards baseline
        self.facilitation += dt_ms / self.tau_f * (1.0 - self.facilitation);
        self.depression += dt_ms / self.tau_d * (1.0 - self.depression);

        // On spike: update both
        if pre_spike {
            self.facilitation += self.f_increment;
            self.depression -= self.d_decrement * self.depression;
        }

        // Clamp values
        self.facilitation = self.facilitation.clamp(1.0, 5.0);
        self.depression = self.depression.clamp(0.1, 1.0);
    }

    /// Get effective transmission coefficient
    pub fn transmission_coefficient(&self) -> f64 {
        self.facilitation * self.depression
    }
}

/// Synaptic plasticity combining STDP and STP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynapticPlasticity {
    pub stdp: STDPRule,
    pub stp: ShortTermPlasticity,
}

impl SynapticPlasticity {
    pub fn new(stdp_params: STDPParams) -> Self {
        Self {
            stdp: STDPRule::new(stdp_params),
            stp: ShortTermPlasticity::default(),
        }
    }

    pub fn with_stp(stdp_params: STDPParams, stp: ShortTermPlasticity) -> Self {
        Self {
            stdp: STDPRule::new(stdp_params),
            stp,
        }
    }
}

/// A synapse connecting two neurons
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Synapse {
    /// Unique ID
    pub id: SynapseId,
    /// Presynaptic neuron
    pub pre_neuron: NeuronId,
    /// Postsynaptic neuron
    pub post_neuron: NeuronId,
    /// Synaptic weight
    pub weight: f64,
    /// Transmission delay (ms)
    pub delay: Duration,
    /// Plasticity mechanisms
    pub plasticity: SynapticPlasticity,
    /// Whether synapse is excitatory
    pub excitatory: bool,
}

impl Synapse {
    /// Create a new synapse
    pub fn new(
        pre_neuron: NeuronId,
        post_neuron: NeuronId,
        weight: f64,
        delay: Duration,
        excitatory: bool,
    ) -> Self {
        Self {
            id: format!("{}->{}",pre_neuron, post_neuron),
            pre_neuron,
            post_neuron,
            weight,
            delay,
            plasticity: SynapticPlasticity::new(STDPParams::default()),
            excitatory,
        }
    }

    /// Create excitatory synapse with default parameters
    pub fn excitatory(pre: NeuronId, post: NeuronId, weight: f64) -> Self {
        Self::new(pre, post, weight, Duration::from_millis(1), true)
    }

    /// Create inhibitory synapse with default parameters
    pub fn inhibitory(pre: NeuronId, post: NeuronId, weight: f64) -> Self {
        Self::new(pre, post, weight, Duration::from_millis(1), false)
    }

    /// Update synapse for one time step
    pub fn step(&mut self, dt: Duration, pre_spike: bool, post_spike: bool) {
        // Update STP
        self.plasticity.stp.update(dt, pre_spike);

        // Update STDP and modify weight
        let dw = self.plasticity.stdp.update(dt, pre_spike, post_spike, self.weight);
        self.weight += dw;

        // Clamp weight
        let params = self.plasticity.stdp.params();
        self.weight = self.weight.max(params.w_min).min(params.w_max);
    }

    /// Get effective synaptic current when presynaptic neuron fires
    pub fn transmit(&self) -> f64 {
        let sign = if self.excitatory { 1.0 } else { -1.0 };
        sign * self.weight * self.plasticity.stp.transmission_coefficient()
    }

    /// Apply neuromodulator effect to learning rate
    pub fn modulate_plasticity(&mut self, dopamine: f64, _norepinephrine: f64) {
        // Dopamine gates STDP learning
        // High dopamine: more plasticity
        // Low dopamine: less plasticity
        let modulation = 1.0 + dopamine;
        self.plasticity.stdp.params.a_plus *= modulation;
        self.plasticity.stdp.params.a_minus *= modulation;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stdp_ltp() {
        let mut stdp = STDPRule::new(STDPParams::default());

        // Pre spike, then post spike (should cause LTP)
        let dt = Duration::from_millis(1);

        // Pre spike
        let dw1 = stdp.update(dt, true, false, 0.5);

        // Small delay
        for _ in 0..10 {
            stdp.update(dt, false, false, 0.5);
        }

        // Post spike - should cause potentiation
        let dw2 = stdp.update(dt, false, true, 0.5);

        assert!(dw2 > 0.0, "Pre-before-post should cause LTP");
    }

    #[test]
    fn test_stdp_ltd() {
        let mut stdp = STDPRule::new(STDPParams::default());

        // Post spike, then pre spike (should cause LTD)
        let dt = Duration::from_millis(1);

        // Post spike
        stdp.update(dt, false, true, 0.5);

        // Small delay
        for _ in 0..10 {
            stdp.update(dt, false, false, 0.5);
        }

        // Pre spike - should cause depression
        let dw = stdp.update(dt, true, false, 0.5);

        assert!(dw < 0.0, "Post-before-pre should cause LTD");
    }

    #[test]
    fn test_stp_facilitation() {
        let mut stp = ShortTermPlasticity::facilitating();

        let initial = stp.transmission_coefficient();

        // Spike
        stp.update(Duration::from_millis(1), true);

        let after_spike = stp.transmission_coefficient();

        assert!(
            after_spike > initial,
            "Facilitation should increase transmission"
        );
    }

    #[test]
    fn test_stp_depression() {
        let mut stp = ShortTermPlasticity::depressing();

        let initial = stp.transmission_coefficient();

        // Spike
        stp.update(Duration::from_millis(1), true);

        let after_spike = stp.transmission_coefficient();

        assert!(
            after_spike < initial,
            "Depression should decrease transmission"
        );
    }

    #[test]
    fn test_synapse_creation() {
        let syn = Synapse::excitatory("n1".to_string(), "n2".to_string(), 0.5);

        assert!(syn.excitatory);
        assert!((syn.weight - 0.5).abs() < 0.01);
        assert!(syn.transmit() > 0.0);
    }

    #[test]
    fn test_inhibitory_synapse() {
        let syn = Synapse::inhibitory("n1".to_string(), "n2".to_string(), 0.5);

        assert!(!syn.excitatory);
        assert!(syn.transmit() < 0.0);
    }
}
