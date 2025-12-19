//! # Omega SNN - Spiking Neural Network Substrate
//!
//! Biologically-inspired spiking neural network implementation for ExoGenesis Omega.
//! Provides the foundational neural dynamics layer that enables brain-like computation.
//!
//! ## Features
//!
//! - **Leaky Integrate-and-Fire (LIF) Neurons**: Biologically plausible neuron model
//! - **Spike-Timing Dependent Plasticity (STDP)**: Hebbian learning based on spike timing
//! - **Neuromodulation**: Dopamine, norepinephrine, serotonin, acetylcholine
//! - **Short-Term Plasticity**: Synaptic facilitation and depression
//! - **Population Coding**: Sparse distributed representations
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    SPIKING NEURAL NETWORK                    │
//! ├─────────────────────────────────────────────────────────────┤
//! │                                                              │
//! │  ┌──────────────────────────────────────────────────────┐  │
//! │  │              NEUROMODULATOR SYSTEM                    │  │
//! │  │  Dopamine │ Norepinephrine │ Serotonin │ Acetylcholine│  │
//! │  └──────────────────────────────────────────────────────┘  │
//! │                          ↓                                   │
//! │  ┌──────────────────────────────────────────────────────┐  │
//! │  │                  NEURAL POPULATIONS                   │  │
//! │  │  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐ │  │
//! │  │  │Excitatory│  │Inhibitory│  │ Sensory │  │  Motor  │ │  │
//! │  │  │ Neurons │  │ Neurons │  │ Neurons │  │ Neurons │ │  │
//! │  │  └─────────┘  └─────────┘  └─────────┘  └─────────┘ │  │
//! │  └──────────────────────────────────────────────────────┘  │
//! │                          ↓                                   │
//! │  ┌──────────────────────────────────────────────────────┐  │
//! │  │                 SYNAPTIC CONNECTIONS                  │  │
//! │  │        STDP │ Short-Term Plasticity │ Weights        │  │
//! │  └──────────────────────────────────────────────────────┘  │
//! │                          ↓                                   │
//! │  ┌──────────────────────────────────────────────────────┐  │
//! │  │                   SPIKE TRAINS                        │  │
//! │  │           Temporal coding │ Rate coding               │  │
//! │  └──────────────────────────────────────────────────────┘  │
//! │                                                              │
//! └─────────────────────────────────────────────────────────────┘
//! ```

pub mod neuron;
pub mod synapse;
pub mod neuromodulators;
pub mod network;
pub mod spike_train;
pub mod population;
pub mod encoding;

pub use neuron::{
    SpikingNeuron, NeuronId, NeuronState, NeuronType,
    LIFNeuron, LIFParams, AdaptiveLIFNeuron,
    // Izhikevich neuron model
    IzhikevichNeuron, IzhikevichParams, IzhikevichType, IzhikevichState,
};
pub use synapse::{
    Synapse, SynapseId, SynapticPlasticity,
    STDPRule, STDPParams, ShortTermPlasticity,
};
pub use neuromodulators::{
    NeuromodulatorSystem, Neuromodulator, NeuromodulatorType,
    DopamineState, NorepinephrineState, SerotoninState, AcetylcholineState,
};
pub use network::{
    SpikingNetwork, NetworkConfig, Layer, LayerId, LayerType,
    // Network topologies
    Topology, SmallWorldBuilder, LocalGridBuilder,
    // Event-driven processing
    EventQueue, SpikeEvent,
};
pub use spike_train::{SpikeTrain, Spike, SpikeAnalysis, SpikeBuffer};
pub use population::{NeuralPopulation, PopulationActivity, SparseCode};
pub use encoding::{
    // Spike encoding methods
    RateEncoder, TemporalEncoder, TemporalEncodingType,
    DeltaEncoder, PopulationEncoder, SparseSpikes, MultiEncoder,
};

use std::time::Duration;
use thiserror::Error;

/// Time step for simulation (1ms default, matching biological timescales)
pub const DEFAULT_DT: Duration = Duration::from_micros(1000);

/// Errors that can occur in the SNN module
#[derive(Error, Debug)]
pub enum SNNError {
    #[error("Neuron not found: {0}")]
    NeuronNotFound(NeuronId),

    #[error("Synapse not found: {0}")]
    SynapseNotFound(SynapseId),

    #[error("Invalid connection: {0}")]
    InvalidConnection(String),

    #[error("Simulation error: {0}")]
    SimulationError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

pub type Result<T> = std::result::Result<T, SNNError>;

/// Main SNN engine that orchestrates all components
pub struct SNNEngine {
    network: SpikingNetwork,
    neuromodulators: NeuromodulatorSystem,
    time: Duration,
    dt: Duration,
}

impl SNNEngine {
    /// Create a new SNN engine with default configuration
    pub fn new(config: NetworkConfig) -> Self {
        Self {
            network: SpikingNetwork::new(config),
            neuromodulators: NeuromodulatorSystem::new(),
            time: Duration::ZERO,
            dt: DEFAULT_DT,
        }
    }

    /// Step the simulation forward by one time step
    pub fn step(&mut self) -> Vec<Spike> {
        // 1. Apply neuromodulator effects to all synapses
        self.neuromodulators.modulate_network(&mut self.network);

        // 2. Propagate spikes through network
        let spikes = self.network.step(self.dt);

        // 3. Apply STDP learning based on spike timing
        self.network.apply_stdp(&spikes, self.time);

        // 4. Update neuromodulator levels based on activity
        self.neuromodulators.update(self.dt, &spikes);

        // 5. Advance time
        self.time += self.dt;

        spikes
    }

    /// Run simulation for specified duration
    pub fn run(&mut self, duration: Duration) -> Vec<SpikeTrain> {
        let steps = (duration.as_micros() / self.dt.as_micros()) as usize;
        let mut all_spikes: Vec<Spike> = Vec::new();

        for _ in 0..steps {
            let spikes = self.step();
            all_spikes.extend(spikes);
        }

        // Group spikes by neuron
        self.network.collect_spike_trains(&all_spikes)
    }

    /// Inject current into specific neurons
    pub fn inject_current(&mut self, neuron_id: NeuronId, current: f64) {
        self.network.inject_current(neuron_id, current);
    }

    /// Set neuromodulator level
    pub fn set_neuromodulator(&mut self, modulator: NeuromodulatorType, level: f64) {
        self.neuromodulators.set_level(modulator, level);
    }

    /// Get current network state
    pub fn state(&self) -> NetworkState {
        NetworkState {
            time: self.time,
            neuron_count: self.network.neuron_count(),
            synapse_count: self.network.synapse_count(),
            neuromodulators: self.neuromodulators.levels(),
        }
    }

    /// Access the underlying network
    pub fn network(&self) -> &SpikingNetwork {
        &self.network
    }

    /// Access the underlying network mutably
    pub fn network_mut(&mut self) -> &mut SpikingNetwork {
        &mut self.network
    }
}

/// Current state of the network
#[derive(Debug, Clone)]
pub struct NetworkState {
    pub time: Duration,
    pub neuron_count: usize,
    pub synapse_count: usize,
    pub neuromodulators: NeuromodulatorLevels,
}

/// Current levels of all neuromodulators
#[derive(Debug, Clone, Default)]
pub struct NeuromodulatorLevels {
    pub dopamine: f64,
    pub norepinephrine: f64,
    pub serotonin: f64,
    pub acetylcholine: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snn_engine_creation() {
        let config = NetworkConfig::default();
        let engine = SNNEngine::new(config);

        let state = engine.state();
        assert_eq!(state.time, Duration::ZERO);
    }

    #[test]
    fn test_snn_engine_step() {
        let config = NetworkConfig::default();
        let mut engine = SNNEngine::new(config);

        let spikes = engine.step();
        assert_eq!(engine.state().time, DEFAULT_DT);
    }
}
