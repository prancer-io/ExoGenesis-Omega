//! Spiking Neural Network Structure
//!
//! Defines the network topology and manages spike propagation.

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use crate::neuron::{LIFNeuron, NeuronId, NeuronType, SpikingNeuron, LIFParams};
use crate::spike_train::{Spike, SpikeTrain};
use crate::synapse::{Synapse, SynapseId};

/// Layer identifier
pub type LayerId = String;

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Default neuron parameters
    pub neuron_params: LIFParams,
    /// Default excitatory:inhibitory ratio
    pub ei_ratio: f64,
    /// Default connection probability
    pub connection_prob: f64,
    /// Default synaptic weight
    pub default_weight: f64,
    /// Default synaptic delay
    pub default_delay: Duration,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            neuron_params: LIFParams::default(),
            ei_ratio: 0.8, // 80% excitatory
            connection_prob: 0.1,
            default_weight: 0.5,
            default_delay: Duration::from_millis(1),
        }
    }
}

/// A layer of neurons
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub id: LayerId,
    pub name: String,
    pub neuron_ids: Vec<NeuronId>,
    pub layer_type: LayerType,
}

/// Type of layer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayerType {
    Input,
    Hidden,
    Output,
    Recurrent,
}

impl Layer {
    pub fn new(id: LayerId, name: String, layer_type: LayerType) -> Self {
        Self {
            id,
            name,
            neuron_ids: Vec::new(),
            layer_type,
        }
    }

    pub fn size(&self) -> usize {
        self.neuron_ids.len()
    }
}

/// Pending spike with delay
#[derive(Debug, Clone)]
struct PendingSpike {
    synapse_id: SynapseId,
    delivery_time: Duration,
    current: f64,
}

/// The spiking neural network
pub struct SpikingNetwork {
    config: NetworkConfig,
    /// All neurons in the network
    neurons: HashMap<NeuronId, Arc<RwLock<Box<dyn SpikingNeuron>>>>,
    /// All synapses
    synapses: HashMap<SynapseId, Synapse>,
    /// Layers
    layers: HashMap<LayerId, Layer>,
    /// Outgoing synapses for each neuron
    outgoing: HashMap<NeuronId, Vec<SynapseId>>,
    /// Incoming synapses for each neuron
    incoming: HashMap<NeuronId, Vec<SynapseId>>,
    /// Pending spikes with delays
    pending_spikes: Vec<PendingSpike>,
    /// Current simulation time
    current_time: Duration,
    /// Global plasticity modulation (from dopamine)
    plasticity_mod: f64,
    /// Global gain modulation (from norepinephrine)
    gain_mod: f64,
    /// Global encoding modulation (from acetylcholine)
    encoding_mod: f64,
}

impl SpikingNetwork {
    /// Create a new empty network
    pub fn new(config: NetworkConfig) -> Self {
        Self {
            config,
            neurons: HashMap::new(),
            synapses: HashMap::new(),
            layers: HashMap::new(),
            outgoing: HashMap::new(),
            incoming: HashMap::new(),
            pending_spikes: Vec::new(),
            current_time: Duration::ZERO,
            plasticity_mod: 1.0,
            gain_mod: 1.0,
            encoding_mod: 1.0,
        }
    }

    /// Add a neuron to the network
    pub fn add_neuron(&mut self, id: NeuronId, neuron_type: NeuronType) -> NeuronId {
        let neuron = LIFNeuron::new(id.clone(), neuron_type, self.config.neuron_params.clone());
        self.neurons.insert(
            id.clone(),
            Arc::new(RwLock::new(Box::new(neuron) as Box<dyn SpikingNeuron>)),
        );
        self.outgoing.insert(id.clone(), Vec::new());
        self.incoming.insert(id.clone(), Vec::new());
        id
    }

    /// Add a layer of neurons
    pub fn add_layer(
        &mut self,
        layer_id: LayerId,
        name: String,
        layer_type: LayerType,
        size: usize,
        neuron_type: NeuronType,
    ) -> &Layer {
        let mut layer = Layer::new(layer_id.clone(), name, layer_type);

        for i in 0..size {
            let neuron_id = format!("{}_{}", layer_id, i);
            self.add_neuron(neuron_id.clone(), neuron_type);
            layer.neuron_ids.push(neuron_id);
        }

        self.layers.insert(layer_id.clone(), layer);
        self.layers.get(&layer_id).unwrap()
    }

    /// Connect two neurons
    pub fn connect(
        &mut self,
        pre: NeuronId,
        post: NeuronId,
        weight: f64,
        delay: Duration,
    ) -> Option<SynapseId> {
        if !self.neurons.contains_key(&pre) || !self.neurons.contains_key(&post) {
            return None;
        }

        let pre_type = self.neurons.get(&pre).unwrap().read().neuron_type();
        let excitatory = pre_type.is_excitatory();

        let synapse = Synapse::new(pre.clone(), post.clone(), weight, delay, excitatory);
        let synapse_id = synapse.id.clone();

        self.synapses.insert(synapse_id.clone(), synapse);
        self.outgoing.get_mut(&pre).unwrap().push(synapse_id.clone());
        self.incoming.get_mut(&post).unwrap().push(synapse_id.clone());

        Some(synapse_id)
    }

    /// Connect two layers with given probability
    pub fn connect_layers(
        &mut self,
        pre_layer: &LayerId,
        post_layer: &LayerId,
        prob: f64,
        weight: f64,
    ) {
        let pre_ids = self.layers.get(pre_layer).map(|l| l.neuron_ids.clone());
        let post_ids = self.layers.get(post_layer).map(|l| l.neuron_ids.clone());

        if let (Some(pre_ids), Some(post_ids)) = (pre_ids, post_ids) {
            for pre in &pre_ids {
                for post in &post_ids {
                    if rand::random::<f64>() < prob {
                        self.connect(
                            pre.clone(),
                            post.clone(),
                            weight,
                            self.config.default_delay,
                        );
                    }
                }
            }
        }
    }

    /// Step the network forward by dt
    pub fn step(&mut self, dt: Duration) -> Vec<Spike> {
        let mut spikes = Vec::new();

        // Deliver pending spikes
        let mut to_deliver = Vec::new();
        self.pending_spikes.retain(|ps| {
            if ps.delivery_time <= self.current_time {
                to_deliver.push(ps.clone());
                false
            } else {
                true
            }
        });

        for ps in to_deliver {
            if let Some(synapse) = self.synapses.get(&ps.synapse_id) {
                if let Some(neuron) = self.neurons.get(&synapse.post_neuron) {
                    neuron.write().receive_input(ps.current * self.gain_mod);
                }
            }
        }

        // Update all neurons
        for (id, neuron) in &self.neurons {
            let spiked = neuron.write().step(dt);

            if spiked {
                spikes.push(Spike::new(id.clone(), self.current_time));

                // Schedule spikes to postsynaptic neurons
                if let Some(outgoing) = self.outgoing.get(id) {
                    for syn_id in outgoing {
                        if let Some(synapse) = self.synapses.get_mut(syn_id) {
                            // Update synapse plasticity
                            synapse.step(dt, true, false);

                            let current = synapse.transmit();
                            let delivery_time = self.current_time + synapse.delay;

                            self.pending_spikes.push(PendingSpike {
                                synapse_id: syn_id.clone(),
                                delivery_time,
                                current,
                            });
                        }
                    }
                }
            }
        }

        // Update time
        self.current_time += dt;

        spikes
    }

    /// Apply STDP based on spike timing
    pub fn apply_stdp(&mut self, spikes: &[Spike], _current_time: Duration) {
        // For each spike, update synapses
        for spike in spikes {
            // Update incoming synapses (this neuron is post)
            if let Some(incoming) = self.incoming.get(&spike.neuron_id) {
                for syn_id in incoming {
                    if let Some(synapse) = self.synapses.get_mut(syn_id) {
                        // Post-spike: apply STDP
                        let dt = Duration::from_millis(1);
                        let dw = synapse.plasticity.stdp.update(dt, false, true, synapse.weight);
                        synapse.weight = (synapse.weight + dw * self.plasticity_mod).clamp(0.0, 1.0);
                    }
                }
            }
        }
    }

    /// Inject current into a neuron
    pub fn inject_current(&mut self, neuron_id: NeuronId, current: f64) {
        if let Some(neuron) = self.neurons.get(&neuron_id) {
            neuron.write().receive_input(current);
        }
    }

    /// Inject current into all neurons in a layer
    pub fn inject_layer_current(&mut self, layer_id: &LayerId, current: f64) {
        // Collect neuron IDs first to avoid borrow conflict
        let neuron_ids: Vec<NeuronId> = if let Some(layer) = self.layers.get(layer_id) {
            layer.neuron_ids.clone()
        } else {
            return;
        };

        for neuron_id in neuron_ids {
            self.inject_current(neuron_id, current);
        }
    }

    /// Collect spike trains from spikes
    pub fn collect_spike_trains(&self, spikes: &[Spike]) -> Vec<SpikeTrain> {
        let mut trains: HashMap<NeuronId, SpikeTrain> = HashMap::new();

        for spike in spikes {
            trains
                .entry(spike.neuron_id.clone())
                .or_insert_with(|| SpikeTrain::new(spike.neuron_id.clone()))
                .add_spike(spike.time);
        }

        trains.into_values().collect()
    }

    /// Get neuron count
    pub fn neuron_count(&self) -> usize {
        self.neurons.len()
    }

    /// Get synapse count
    pub fn synapse_count(&self) -> usize {
        self.synapses.len()
    }

    /// Modulate plasticity (dopamine effect)
    pub fn modulate_plasticity(&mut self, level: f64) {
        self.plasticity_mod = 0.5 + level; // Range: 0.5 to 1.5
    }

    /// Modulate gain (norepinephrine effect)
    pub fn modulate_gain(&mut self, level: f64) {
        self.gain_mod = 0.5 + level; // Range: 0.5 to 1.5
    }

    /// Modulate encoding (acetylcholine effect)
    pub fn modulate_encoding(&mut self, level: f64) {
        self.encoding_mod = 0.5 + level;
    }

    /// Get layer by ID
    pub fn get_layer(&self, id: &LayerId) -> Option<&Layer> {
        self.layers.get(id)
    }

    /// Get all layer IDs
    pub fn layer_ids(&self) -> Vec<LayerId> {
        self.layers.keys().cloned().collect()
    }

    /// Get current time
    pub fn current_time(&self) -> Duration {
        self.current_time
    }

    /// Reset network state
    pub fn reset(&mut self) {
        for neuron in self.neurons.values() {
            neuron.write().reset();
        }
        self.pending_spikes.clear();
        self.current_time = Duration::ZERO;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_creation() {
        let network = SpikingNetwork::new(NetworkConfig::default());
        assert_eq!(network.neuron_count(), 0);
        assert_eq!(network.synapse_count(), 0);
    }

    #[test]
    fn test_add_neuron() {
        let mut network = SpikingNetwork::new(NetworkConfig::default());

        network.add_neuron("n1".to_string(), NeuronType::Excitatory);
        network.add_neuron("n2".to_string(), NeuronType::Inhibitory);

        assert_eq!(network.neuron_count(), 2);
    }

    #[test]
    fn test_add_layer() {
        let mut network = SpikingNetwork::new(NetworkConfig::default());

        let layer = network.add_layer(
            "input".to_string(),
            "Input Layer".to_string(),
            LayerType::Input,
            10,
            NeuronType::Excitatory,
        );

        assert_eq!(layer.size(), 10);
        assert_eq!(network.neuron_count(), 10);
    }

    #[test]
    fn test_connect_neurons() {
        let mut network = SpikingNetwork::new(NetworkConfig::default());

        network.add_neuron("n1".to_string(), NeuronType::Excitatory);
        network.add_neuron("n2".to_string(), NeuronType::Excitatory);

        let syn_id = network.connect(
            "n1".to_string(),
            "n2".to_string(),
            0.5,
            Duration::from_millis(1),
        );

        assert!(syn_id.is_some());
        assert_eq!(network.synapse_count(), 1);
    }

    #[test]
    fn test_network_step() {
        let mut network = SpikingNetwork::new(NetworkConfig::default());

        network.add_neuron("n1".to_string(), NeuronType::Excitatory);
        network.add_neuron("n2".to_string(), NeuronType::Excitatory);
        network.connect(
            "n1".to_string(),
            "n2".to_string(),
            0.8,
            Duration::from_millis(1),
        );

        // Inject strong current to cause spike
        network.inject_current("n1".to_string(), 10.0);

        let dt = Duration::from_millis(1);
        let mut total_spikes = 0;

        for _ in 0..100 {
            let spikes = network.step(dt);
            total_spikes += spikes.len();
            network.inject_current("n1".to_string(), 10.0);
        }

        assert!(total_spikes > 0, "Should have some spikes");
    }

    #[test]
    fn test_layer_connection() {
        let mut network = SpikingNetwork::new(NetworkConfig::default());

        network.add_layer(
            "input".to_string(),
            "Input".to_string(),
            LayerType::Input,
            5,
            NeuronType::Excitatory,
        );

        network.add_layer(
            "output".to_string(),
            "Output".to_string(),
            LayerType::Output,
            5,
            NeuronType::Excitatory,
        );

        network.connect_layers(&"input".to_string(), &"output".to_string(), 1.0, 0.5);

        // With 100% connection probability, should have 25 synapses (5x5)
        assert_eq!(network.synapse_count(), 25);
    }
}
