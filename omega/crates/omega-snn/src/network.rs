//! Spiking Neural Network Structure
//!
//! Defines the network topology and manages spike propagation.
//! Supports multiple connection topologies:
//! - Feedforward: Layer-by-layer connections
//! - AllToAll: Probabilistic random connectivity
//! - SmallWorld: Ring lattice with rewiring
//! - LocalGrid: 2D spatial connectivity

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
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

// ============================================================================
// NETWORK TOPOLOGIES
// ============================================================================

/// Network topology types for connection patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Topology {
    /// Feedforward: Layer-by-layer connections
    Feedforward {
        /// Connection probability between layers
        connection_prob: f64,
    },
    /// AllToAll: Random connectivity with given probability
    AllToAll {
        /// Connection probability
        connection_prob: f64,
    },
    /// SmallWorld: Watts-Strogatz small-world network
    SmallWorld {
        /// Number of nearest neighbors (k)
        k: usize,
        /// Rewiring probability (beta)
        beta: f64,
    },
    /// LocalGrid: 2D spatial connectivity
    LocalGrid {
        /// Width of the grid
        width: usize,
        /// Height of the grid
        height: usize,
        /// Connection radius
        radius: f64,
        /// Whether connections wrap around edges
        wrap_around: bool,
    },
    /// Custom: Manual connection specification
    Custom,
}

impl Default for Topology {
    fn default() -> Self {
        Topology::AllToAll {
            connection_prob: 0.1,
        }
    }
}

/// Small-world network builder
#[derive(Debug, Clone)]
pub struct SmallWorldBuilder {
    /// Number of neurons
    pub n: usize,
    /// Each node connected to k nearest neighbors
    pub k: usize,
    /// Rewiring probability
    pub beta: f64,
    /// Default weight for connections
    pub weight: f64,
    /// Default delay for connections
    pub delay: Duration,
}

impl SmallWorldBuilder {
    pub fn new(n: usize, k: usize, beta: f64) -> Self {
        Self {
            n,
            k,
            beta,
            weight: 0.5,
            delay: Duration::from_millis(1),
        }
    }

    /// Generate connections for a small-world network
    pub fn generate_connections(&self) -> Vec<(usize, usize, f64, Duration)> {
        let mut connections = Vec::new();
        let mut rng = rand::thread_rng();

        // Create ring lattice
        for i in 0..self.n {
            for j in 1..=(self.k / 2) {
                let target = (i + j) % self.n;

                // Rewiring with probability beta
                let actual_target = if rand::Rng::gen::<f64>(&mut rng) < self.beta {
                    // Rewire to random node (avoiding self and existing)
                    let mut new_target = rand::Rng::gen_range(&mut rng, 0..self.n);
                    while new_target == i {
                        new_target = rand::Rng::gen_range(&mut rng, 0..self.n);
                    }
                    new_target
                } else {
                    target
                };

                connections.push((i, actual_target, self.weight, self.delay));
            }
        }

        connections
    }

    /// Apply to a network with given neuron IDs
    pub fn apply_to_network(
        &self,
        network: &mut SpikingNetwork,
        neuron_ids: &[NeuronId],
    ) {
        let connections = self.generate_connections();

        for (pre_idx, post_idx, weight, delay) in connections {
            if pre_idx < neuron_ids.len() && post_idx < neuron_ids.len() {
                network.connect(
                    neuron_ids[pre_idx].clone(),
                    neuron_ids[post_idx].clone(),
                    weight,
                    delay,
                );
            }
        }
    }
}

/// 2D Grid topology builder
#[derive(Debug, Clone)]
pub struct LocalGridBuilder {
    /// Width of the grid
    pub width: usize,
    /// Height of the grid
    pub height: usize,
    /// Maximum connection radius
    pub radius: f64,
    /// Whether connections wrap around edges (toroidal)
    pub wrap_around: bool,
    /// Base weight (will be modulated by distance)
    pub base_weight: f64,
    /// Default delay
    pub delay: Duration,
    /// Weight decay with distance (0 = no decay, 1 = linear decay)
    pub distance_decay: f64,
}

impl LocalGridBuilder {
    pub fn new(width: usize, height: usize, radius: f64) -> Self {
        Self {
            width,
            height,
            radius,
            wrap_around: false,
            base_weight: 0.5,
            delay: Duration::from_millis(1),
            distance_decay: 0.5,
        }
    }

    /// Convert 1D index to 2D coordinates
    fn idx_to_coord(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }

    /// Calculate distance between two neurons
    fn distance(&self, idx1: usize, idx2: usize) -> f64 {
        let (x1, y1) = self.idx_to_coord(idx1);
        let (x2, y2) = self.idx_to_coord(idx2);

        let dx = if self.wrap_around {
            let d = (x1 as isize - x2 as isize).unsigned_abs();
            d.min(self.width - d) as f64
        } else {
            (x1 as f64 - x2 as f64).abs()
        };

        let dy = if self.wrap_around {
            let d = (y1 as isize - y2 as isize).unsigned_abs();
            d.min(self.height - d) as f64
        } else {
            (y1 as f64 - y2 as f64).abs()
        };

        (dx * dx + dy * dy).sqrt()
    }

    /// Generate connections for a 2D grid network
    pub fn generate_connections(&self) -> Vec<(usize, usize, f64, Duration)> {
        let mut connections = Vec::new();
        let n = self.width * self.height;

        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }

                let dist = self.distance(i, j);
                if dist <= self.radius {
                    // Weight decreases with distance
                    let weight_mod = 1.0 - (dist / self.radius) * self.distance_decay;
                    let weight = self.base_weight * weight_mod;

                    if weight > 0.0 {
                        connections.push((i, j, weight, self.delay));
                    }
                }
            }
        }

        connections
    }

    /// Apply to a network with given neuron IDs
    pub fn apply_to_network(
        &self,
        network: &mut SpikingNetwork,
        neuron_ids: &[NeuronId],
    ) {
        let connections = self.generate_connections();

        for (pre_idx, post_idx, weight, delay) in connections {
            if pre_idx < neuron_ids.len() && post_idx < neuron_ids.len() {
                network.connect(
                    neuron_ids[pre_idx].clone(),
                    neuron_ids[post_idx].clone(),
                    weight,
                    delay,
                );
            }
        }
    }
}

// ============================================================================
// EVENT-DRIVEN PROCESSING
// ============================================================================

/// Scheduled spike event for event-driven processing
#[derive(Debug, Clone)]
pub struct SpikeEvent {
    /// Target neuron
    pub target_neuron: NeuronId,
    /// Delivery time
    pub delivery_time: Duration,
    /// Current to deliver
    pub current: f64,
    /// Source synapse
    pub synapse_id: SynapseId,
}

impl PartialEq for SpikeEvent {
    fn eq(&self, other: &Self) -> bool {
        self.delivery_time == other.delivery_time
    }
}

impl Eq for SpikeEvent {}

impl PartialOrd for SpikeEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SpikeEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap behavior
        other.delivery_time.cmp(&self.delivery_time)
    }
}

/// Event-driven spike queue using a binary heap (priority queue)
#[derive(Debug, Clone, Default)]
pub struct EventQueue {
    /// Priority queue of pending events (min-heap by time)
    events: BinaryHeap<SpikeEvent>,
    /// Total events processed
    events_processed: u64,
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            events: BinaryHeap::new(),
            events_processed: 0,
        }
    }

    /// Schedule a new spike event
    pub fn schedule(&mut self, event: SpikeEvent) {
        self.events.push(event);
    }

    /// Get next event if its delivery time has arrived
    pub fn pop_ready(&mut self, current_time: Duration) -> Option<SpikeEvent> {
        if let Some(event) = self.events.peek() {
            if event.delivery_time <= current_time {
                self.events_processed += 1;
                return self.events.pop();
            }
        }
        None
    }

    /// Get all events ready at current time
    pub fn pop_all_ready(&mut self, current_time: Duration) -> Vec<SpikeEvent> {
        let mut ready = Vec::new();
        while let Some(event) = self.pop_ready(current_time) {
            ready.push(event);
        }
        ready
    }

    /// Get next event time (for adaptive stepping)
    pub fn next_event_time(&self) -> Option<Duration> {
        self.events.peek().map(|e| e.delivery_time)
    }

    /// Number of pending events
    pub fn pending_count(&self) -> usize {
        self.events.len()
    }

    /// Total events processed
    pub fn total_processed(&self) -> u64 {
        self.events_processed
    }

    /// Clear all pending events
    pub fn clear(&mut self) {
        self.events.clear();
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
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
    /// Pending spikes with delays (legacy)
    pending_spikes: Vec<PendingSpike>,
    /// Event-driven spike queue (priority queue)
    event_queue: EventQueue,
    /// Current simulation time
    current_time: Duration,
    /// Global plasticity modulation (from dopamine)
    plasticity_mod: f64,
    /// Global gain modulation (from norepinephrine)
    gain_mod: f64,
    /// Global encoding modulation (from acetylcholine)
    encoding_mod: f64,
    /// Use event-driven processing (vs time-stepped)
    event_driven: bool,
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
            event_queue: EventQueue::new(),
            current_time: Duration::ZERO,
            plasticity_mod: 1.0,
            gain_mod: 1.0,
            encoding_mod: 1.0,
            event_driven: false,
        }
    }

    /// Create a new network with event-driven processing enabled
    pub fn new_event_driven(config: NetworkConfig) -> Self {
        let mut network = Self::new(config);
        network.event_driven = true;
        network
    }

    /// Enable or disable event-driven processing
    pub fn set_event_driven(&mut self, enabled: bool) {
        self.event_driven = enabled;
    }

    /// Check if event-driven processing is enabled
    pub fn is_event_driven(&self) -> bool {
        self.event_driven
    }

    /// Get the event queue
    pub fn event_queue(&self) -> &EventQueue {
        &self.event_queue
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
        self.event_queue.clear();
        self.current_time = Duration::ZERO;
    }

    /// Apply small-world topology to a set of neurons
    pub fn apply_small_world(&mut self, neuron_ids: &[NeuronId], k: usize, beta: f64) {
        let builder = SmallWorldBuilder::new(neuron_ids.len(), k, beta);
        builder.apply_to_network(self, neuron_ids);
    }

    /// Apply 2D grid topology to a set of neurons
    pub fn apply_local_grid(
        &mut self,
        neuron_ids: &[NeuronId],
        width: usize,
        height: usize,
        radius: f64,
    ) {
        let builder = LocalGridBuilder::new(width, height, radius);
        builder.apply_to_network(self, neuron_ids);
    }

    /// Create a layer with small-world topology
    pub fn add_small_world_layer(
        &mut self,
        layer_id: LayerId,
        name: String,
        layer_type: LayerType,
        size: usize,
        neuron_type: NeuronType,
        k: usize,
        beta: f64,
    ) -> &Layer {
        // First add the layer
        self.add_layer(layer_id.clone(), name, layer_type, size, neuron_type);

        // Get the neuron IDs
        let neuron_ids: Vec<NeuronId> = self.layers.get(&layer_id)
            .map(|l| l.neuron_ids.clone())
            .unwrap_or_default();

        // Apply small-world topology
        self.apply_small_world(&neuron_ids, k, beta);

        self.layers.get(&layer_id).unwrap()
    }

    /// Create a layer with 2D grid topology
    pub fn add_grid_layer(
        &mut self,
        layer_id: LayerId,
        name: String,
        layer_type: LayerType,
        width: usize,
        height: usize,
        neuron_type: NeuronType,
        connection_radius: f64,
    ) -> &Layer {
        let size = width * height;

        // Add the layer
        self.add_layer(layer_id.clone(), name, layer_type, size, neuron_type);

        // Get the neuron IDs
        let neuron_ids: Vec<NeuronId> = self.layers.get(&layer_id)
            .map(|l| l.neuron_ids.clone())
            .unwrap_or_default();

        // Apply grid topology
        self.apply_local_grid(&neuron_ids, width, height, connection_radius);

        self.layers.get(&layer_id).unwrap()
    }

    /// Step using event-driven processing (more efficient for sparse activity)
    pub fn step_event_driven(&mut self, dt: Duration) -> Vec<Spike> {
        let mut spikes = Vec::new();

        // Process all events ready at current time
        let ready_events = self.event_queue.pop_all_ready(self.current_time);
        for event in ready_events {
            if let Some(neuron) = self.neurons.get(&event.target_neuron) {
                neuron.write().receive_input(event.current * self.gain_mod);
            }
        }

        // Update all neurons
        for (id, neuron) in &self.neurons {
            let spiked = neuron.write().step(dt);

            if spiked {
                spikes.push(Spike::new(id.clone(), self.current_time));

                // Schedule spikes to postsynaptic neurons via event queue
                if let Some(outgoing) = self.outgoing.get(id) {
                    for syn_id in outgoing {
                        if let Some(synapse) = self.synapses.get_mut(syn_id) {
                            synapse.step(dt, true, false);
                            let current = synapse.transmit();

                            self.event_queue.schedule(SpikeEvent {
                                target_neuron: synapse.post_neuron.clone(),
                                delivery_time: self.current_time + synapse.delay,
                                current,
                                synapse_id: syn_id.clone(),
                            });
                        }
                    }
                }
            }
        }

        self.current_time += dt;
        spikes
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

    // ========================================================================
    // TOPOLOGY TESTS
    // ========================================================================

    #[test]
    fn test_small_world_builder() {
        let builder = SmallWorldBuilder::new(10, 4, 0.0);
        let connections = builder.generate_connections();

        // With beta=0 (no rewiring), should have k/2 * n = 2 * 10 = 20 connections
        assert_eq!(connections.len(), 20);

        // All connections should be valid
        for (pre, post, weight, _) in &connections {
            assert!(*pre < 10);
            assert!(*post < 10);
            assert!(*weight > 0.0);
        }
    }

    #[test]
    fn test_small_world_with_rewiring() {
        let builder = SmallWorldBuilder::new(20, 4, 0.5);
        let connections = builder.generate_connections();

        // Should still have same number of connections
        assert_eq!(connections.len(), 40); // k/2 * n = 2 * 20

        // All connections should be valid
        for (pre, post, _, _) in &connections {
            assert!(*pre < 20);
            assert!(*post < 20);
        }
    }

    #[test]
    fn test_local_grid_builder() {
        let builder = LocalGridBuilder::new(3, 3, 1.5);
        let connections = builder.generate_connections();

        // 9 neurons, each should connect to neighbors within radius 1.5
        // Center neuron (4) should connect to all 8 neighbors
        // Corner neurons should connect to 3 neighbors
        // Edge neurons should connect to 5 neighbors
        assert!(!connections.is_empty());

        for (pre, post, weight, _) in &connections {
            assert!(*pre < 9);
            assert!(*post < 9);
            assert!(*pre != *post);
            assert!(*weight > 0.0);
        }
    }

    #[test]
    fn test_small_world_layer() {
        let mut network = SpikingNetwork::new(NetworkConfig::default());

        let layer = network.add_small_world_layer(
            "sw".to_string(),
            "Small World".to_string(),
            LayerType::Hidden,
            10,
            NeuronType::Excitatory,
            4,  // k=4
            0.2, // beta=0.2
        );

        assert_eq!(layer.size(), 10);
        assert_eq!(network.neuron_count(), 10);
        // Should have roughly k/2 * n = 20 synapses (may vary slightly due to rewiring)
        let synapse_count = network.synapse_count();
        assert!(synapse_count >= 18 && synapse_count <= 20,
            "Expected 18-20 synapses, got {}", synapse_count);
    }

    #[test]
    fn test_grid_layer() {
        let mut network = SpikingNetwork::new(NetworkConfig::default());

        let layer = network.add_grid_layer(
            "grid".to_string(),
            "Grid Layer".to_string(),
            LayerType::Hidden,
            4,  // width
            4,  // height
            NeuronType::Excitatory,
            1.5, // radius
        );

        assert_eq!(layer.size(), 16); // 4x4
        assert_eq!(network.neuron_count(), 16);
        // Should have some connections based on radius
        assert!(network.synapse_count() > 0);
    }

    // ========================================================================
    // EVENT-DRIVEN PROCESSING TESTS
    // ========================================================================

    #[test]
    fn test_event_queue() {
        let mut queue = EventQueue::new();

        // Schedule events
        queue.schedule(SpikeEvent {
            target_neuron: "n1".to_string(),
            delivery_time: Duration::from_millis(10),
            current: 1.0,
            synapse_id: "s1".to_string(),
        });
        queue.schedule(SpikeEvent {
            target_neuron: "n2".to_string(),
            delivery_time: Duration::from_millis(5),
            current: 2.0,
            synapse_id: "s2".to_string(),
        });
        queue.schedule(SpikeEvent {
            target_neuron: "n3".to_string(),
            delivery_time: Duration::from_millis(15),
            current: 0.5,
            synapse_id: "s3".to_string(),
        });

        assert_eq!(queue.pending_count(), 3);

        // Pop at t=5 should get n2
        let event = queue.pop_ready(Duration::from_millis(5));
        assert!(event.is_some());
        assert_eq!(event.unwrap().target_neuron, "n2");

        // Pop at t=10 should get n1
        let event = queue.pop_ready(Duration::from_millis(10));
        assert!(event.is_some());
        assert_eq!(event.unwrap().target_neuron, "n1");

        // Pop at t=12 should get nothing (n3 is at t=15)
        let event = queue.pop_ready(Duration::from_millis(12));
        assert!(event.is_none());

        // Pop at t=15 should get n3
        let event = queue.pop_ready(Duration::from_millis(15));
        assert!(event.is_some());
        assert_eq!(event.unwrap().target_neuron, "n3");

        assert!(queue.is_empty());
        assert_eq!(queue.total_processed(), 3);
    }

    #[test]
    fn test_event_driven_network() {
        let mut network = SpikingNetwork::new_event_driven(NetworkConfig::default());

        assert!(network.is_event_driven());

        network.add_neuron("n1".to_string(), NeuronType::Excitatory);
        network.add_neuron("n2".to_string(), NeuronType::Excitatory);
        network.connect(
            "n1".to_string(),
            "n2".to_string(),
            0.8,
            Duration::from_millis(2),
        );

        // Inject current and step using event-driven
        network.inject_current("n1".to_string(), 10.0);

        let dt = Duration::from_millis(1);
        let mut total_spikes = 0;

        for _ in 0..100 {
            let spikes = network.step_event_driven(dt);
            total_spikes += spikes.len();
            network.inject_current("n1".to_string(), 10.0);
        }

        assert!(total_spikes > 0, "Event-driven network should produce spikes");
    }

    #[test]
    fn test_event_queue_pop_all_ready() {
        let mut queue = EventQueue::new();

        // Schedule multiple events at same time
        for i in 0..5 {
            queue.schedule(SpikeEvent {
                target_neuron: format!("n{}", i),
                delivery_time: Duration::from_millis(10),
                current: 1.0,
                synapse_id: format!("s{}", i),
            });
        }

        let events = queue.pop_all_ready(Duration::from_millis(10));
        assert_eq!(events.len(), 5);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_topology_enum() {
        let feedforward = Topology::Feedforward { connection_prob: 0.5 };
        let all_to_all = Topology::AllToAll { connection_prob: 0.1 };
        let small_world = Topology::SmallWorld { k: 4, beta: 0.2 };
        let local_grid = Topology::LocalGrid {
            width: 10,
            height: 10,
            radius: 2.0,
            wrap_around: true,
        };

        // Just verify they can be created
        match feedforward {
            Topology::Feedforward { connection_prob } => assert_eq!(connection_prob, 0.5),
            _ => panic!("Wrong type"),
        }

        match all_to_all {
            Topology::AllToAll { connection_prob } => assert_eq!(connection_prob, 0.1),
            _ => panic!("Wrong type"),
        }

        match small_world {
            Topology::SmallWorld { k, beta } => {
                assert_eq!(k, 4);
                assert_eq!(beta, 0.2);
            }
            _ => panic!("Wrong type"),
        }

        match local_grid {
            Topology::LocalGrid { width, height, radius, wrap_around } => {
                assert_eq!(width, 10);
                assert_eq!(height, 10);
                assert_eq!(radius, 2.0);
                assert!(wrap_around);
            }
            _ => panic!("Wrong type"),
        }
    }
}
