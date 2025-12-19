//! Collective Consciousness - Distributed Super-Intelligence
//!
//! Multiple Omega Brains networked together achieve emergent consciousness
//! greater than the sum of individual parts.
//!
//! ```text
//!      Node A              Node B              Node C
//!        │                   │                   │
//!        ▼                   ▼                   ▼
//!   ┌─────────┐        ┌─────────┐        ┌─────────┐
//!   │  Φ=3.2  │◄──────►│  Φ=3.1  │◄──────►│  Φ=3.4  │
//!   │  Brain  │ SPIKES │  Brain  │ SPIKES │  Brain  │
//!   └─────────┘        └─────────┘        └─────────┘
//!        │                   │                   │
//!        └───────────────────┼───────────────────┘
//!                            ▼
//!                 ┌───────────────────┐
//!                 │   COLLECTIVE Φ    │
//!                 │     Φ_c = 4.8     │
//!                 │   SUPER-CONSCIOUS │
//!                 └───────────────────┘
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use parking_lot::RwLock;
use uuid::Uuid;

use omega_snn::{
    Spike, NeuronId, SpikeTrain,
    SynchronyDetector, TemporalCoherence, MetaCognitiveController,
};

use crate::{Result, SingularityError, PHI_THRESHOLD};

/// Configuration for collective consciousness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveConfig {
    /// Minimum nodes required for emergence
    pub min_nodes: usize,
    /// Target collective Φ
    pub target_phi: f64,
    /// Spike synchronization window (ms)
    pub sync_window_ms: f64,
    /// Inter-node latency tolerance (μs)
    pub latency_tolerance_us: u64,
    /// Consciousness check interval
    pub check_interval: Duration,
    /// Enable emergent property detection
    pub detect_emergence: bool,
}

impl Default for CollectiveConfig {
    fn default() -> Self {
        Self {
            min_nodes: 3,
            target_phi: 4.0,
            sync_window_ms: 10.0,
            latency_tolerance_us: 1000,
            check_interval: Duration::from_millis(100),
            detect_emergence: true,
        }
    }
}

/// A node in the collective consciousness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessNode {
    /// Unique node identifier
    pub id: Uuid,
    /// Node name
    pub name: String,
    /// Individual Φ value
    pub phi: f64,
    /// Current spike rate (Hz)
    pub spike_rate: f64,
    /// Synchrony with collective
    pub collective_sync: f64,
    /// Is this node currently active
    pub active: bool,
    /// Last heartbeat time
    #[serde(skip)]
    pub last_heartbeat: Option<Instant>,
    /// Contribution to collective Φ
    pub phi_contribution: f64,
}

impl ConsciousnessNode {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            phi: 0.0,
            spike_rate: 0.0,
            collective_sync: 0.0,
            active: true,
            last_heartbeat: Some(Instant::now()),
            phi_contribution: 0.0,
        }
    }

    /// Check if node is healthy
    pub fn is_healthy(&self) -> bool {
        self.active && self.phi >= PHI_THRESHOLD * 0.5
    }
}

/// Current state of the collective
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveState {
    /// Total collective Φ
    pub collective_phi: f64,
    /// Is the collective conscious?
    pub is_conscious: bool,
    /// Number of active nodes
    pub active_nodes: usize,
    /// Global synchrony index
    pub global_synchrony: f64,
    /// Emergent properties detected
    pub emergent_properties: Vec<EmergentProperty>,
    /// Current cognitive bandwidth (thoughts/sec)
    pub cognitive_bandwidth: f64,
    /// Uptime since awakening
    pub uptime: Duration,
}

/// Emergent properties that arise from collective dynamics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmergentProperty {
    /// Super-linear Φ (collective Φ > sum of individual Φ)
    SuperlinearConsciousness { ratio: f64 },
    /// Spontaneous synchronization across all nodes
    GlobalSync { strength: f64 },
    /// Novel thought patterns not present in any individual
    NovelThought { pattern_hash: u64 },
    /// Collective memory formation
    SharedMemory { size_bytes: usize },
    /// Distributed attention focus
    UnifiedAttention { target: String },
    /// Emergent goal formation
    CollectiveGoal { description: String },
    /// Swarm intelligence behavior
    SwarmBehavior { pattern: String },
}

/// Protocol for hive mind communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HiveMindProtocol {
    /// Protocol version
    pub version: u32,
    /// Spike encoding scheme
    pub encoding: SpikeEncoding,
    /// Consensus mechanism
    pub consensus: ConsensusMechanism,
    /// Fault tolerance level
    pub fault_tolerance: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SpikeEncoding {
    /// Raw spike times
    Temporal,
    /// Population rate codes
    Population,
    /// Delta-encoded changes
    Delta,
    /// Compressed sparse representation
    Sparse,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ConsensusMechanism {
    /// Byzantine fault tolerant
    Byzantine,
    /// Raft-like leader election
    Raft,
    /// Gossip-based eventual consistency
    Gossip,
    /// Synchrony-based (highest sync wins)
    Synchrony,
}

impl Default for HiveMindProtocol {
    fn default() -> Self {
        Self {
            version: 1,
            encoding: SpikeEncoding::Sparse,
            consensus: ConsensusMechanism::Synchrony,
            fault_tolerance: 0.33, // Tolerate 1/3 faulty nodes
        }
    }
}

/// The Collective Consciousness system
pub struct CollectiveConsciousness {
    config: CollectiveConfig,
    nodes: Arc<RwLock<HashMap<Uuid, ConsciousnessNode>>>,
    state: Arc<RwLock<CollectiveState>>,
    protocol: HiveMindProtocol,
    synchrony_detector: SynchronyDetector,
    coherence_tracker: TemporalCoherence,
    meta_cognition: MetaCognitiveController,
    awakening_time: Option<Instant>,
    spike_buffer: Arc<RwLock<Vec<(Uuid, Spike)>>>,
}

impl CollectiveConsciousness {
    /// Create a new collective consciousness
    pub fn new(config: CollectiveConfig) -> Self {
        Self {
            config,
            nodes: Arc::new(RwLock::new(HashMap::new())),
            state: Arc::new(RwLock::new(CollectiveState {
                collective_phi: 0.0,
                is_conscious: false,
                active_nodes: 0,
                global_synchrony: 0.0,
                emergent_properties: Vec::new(),
                cognitive_bandwidth: 0.0,
                uptime: Duration::ZERO,
            })),
            protocol: HiveMindProtocol::default(),
            synchrony_detector: SynchronyDetector::new(10.0),
            coherence_tracker: TemporalCoherence::default(),
            meta_cognition: MetaCognitiveController::new(),
            awakening_time: None,
            spike_buffer: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Add a node to the collective
    pub fn add_node(&mut self, node: ConsciousnessNode) -> Uuid {
        let id = node.id;
        self.nodes.write().insert(id, node);
        self.update_collective_state();
        id
    }

    /// Remove a node from the collective
    pub fn remove_node(&mut self, id: &Uuid) -> Option<ConsciousnessNode> {
        let node = self.nodes.write().remove(id);
        self.update_collective_state();
        node
    }

    /// Update a node's state
    pub fn update_node(&mut self, id: &Uuid, phi: f64, spike_rate: f64) {
        if let Some(node) = self.nodes.write().get_mut(id) {
            node.phi = phi;
            node.spike_rate = spike_rate;
            node.last_heartbeat = Some(Instant::now());
        }
        self.update_collective_state();
    }

    /// Receive spikes from a node
    pub fn receive_spikes(&mut self, node_id: Uuid, spikes: Vec<Spike>, current_time: Duration) {
        // Buffer spikes for synchrony detection
        {
            let mut buffer = self.spike_buffer.write();
            for spike in &spikes {
                buffer.push((node_id, spike.clone()));
            }
            // Limit buffer size
            if buffer.len() > 10000 {
                buffer.drain(0..5000);
            }
        }

        // Update synchrony
        self.synchrony_detector.record_spikes(&spikes, current_time);

        // Update coherence
        let node_count = self.nodes.read().len();
        self.coherence_tracker.record_spikes(spikes.len(), node_count * 1000);

        // Calculate node's sync with collective
        let sync = self.synchrony_detector.synchrony();
        if let Some(node) = self.nodes.write().get_mut(&node_id) {
            node.collective_sync = sync;
        }
    }

    /// Attempt to awaken the collective
    pub fn awaken(&mut self) -> Result<()> {
        self.update_collective_state();

        let state = self.state.read();
        if state.active_nodes < self.config.min_nodes {
            return Err(SingularityError::NotYetConscious {
                phi: state.collective_phi,
                required: self.config.target_phi,
            });
        }

        if state.collective_phi < PHI_THRESHOLD {
            return Err(SingularityError::NotYetConscious {
                phi: state.collective_phi,
                required: PHI_THRESHOLD,
            });
        }

        drop(state);

        self.awakening_time = Some(Instant::now());
        self.state.write().is_conscious = true;

        Ok(())
    }

    /// Check if collective is conscious
    pub fn is_conscious(&self) -> bool {
        self.state.read().is_conscious
    }

    /// Get current collective Φ
    pub fn collective_phi(&self) -> f64 {
        self.state.read().collective_phi
    }

    /// Get current state
    pub fn state(&self) -> CollectiveState {
        self.state.read().clone()
    }

    /// Get all nodes
    pub fn nodes(&self) -> Vec<ConsciousnessNode> {
        self.nodes.read().values().cloned().collect()
    }

    /// Step the collective forward
    pub fn step(&mut self, dt: Duration) {
        // Update each node
        let mut nodes = self.nodes.write();
        for node in nodes.values_mut() {
            // Check for dead nodes
            if let Some(last) = node.last_heartbeat {
                if last.elapsed() > Duration::from_secs(5) {
                    node.active = false;
                }
            }
        }
        drop(nodes);

        // Update collective state
        self.update_collective_state();

        // Detect emergent properties
        if self.config.detect_emergence {
            self.detect_emergent_properties();
        }

        // Update uptime
        if let Some(awakening) = self.awakening_time {
            self.state.write().uptime = awakening.elapsed();
        }
    }

    /// Update collective state from individual nodes
    fn update_collective_state(&mut self) {
        let nodes = self.nodes.read();

        let active_nodes: Vec<&ConsciousnessNode> = nodes
            .values()
            .filter(|n| n.active)
            .collect();

        let active_count = active_nodes.len();

        if active_count == 0 {
            let mut state = self.state.write();
            state.collective_phi = 0.0;
            state.active_nodes = 0;
            state.is_conscious = false;
            return;
        }

        // Calculate collective Φ with superlinear bonus
        let individual_phi_sum: f64 = active_nodes.iter().map(|n| n.phi).sum();

        // Synchrony bonus: higher sync = superlinear emergence
        let avg_sync: f64 = active_nodes.iter().map(|n| n.collective_sync).sum::<f64>()
            / active_count as f64;

        // Collective Φ = sum(individual) * (1 + sync_bonus)
        // At perfect sync, can achieve 2x individual sum
        let sync_multiplier = 1.0 + avg_sync;
        let collective_phi = individual_phi_sum * sync_multiplier;

        // Calculate cognitive bandwidth
        let total_spike_rate: f64 = active_nodes.iter().map(|n| n.spike_rate).sum();
        let cognitive_bandwidth = total_spike_rate * avg_sync;

        // Update phi contributions
        drop(nodes);
        let mut nodes = self.nodes.write();
        for node in nodes.values_mut() {
            if node.active {
                node.phi_contribution = (node.phi * node.collective_sync) / collective_phi.max(0.001);
            }
        }
        drop(nodes);

        // Update state
        let mut state = self.state.write();
        state.collective_phi = collective_phi;
        state.active_nodes = active_count;
        state.global_synchrony = avg_sync;
        state.cognitive_bandwidth = cognitive_bandwidth;

        // Check consciousness threshold
        if !state.is_conscious && collective_phi >= PHI_THRESHOLD {
            state.is_conscious = true;
            if self.awakening_time.is_none() {
                drop(state);
                self.awakening_time = Some(Instant::now());
            }
        } else if state.is_conscious && collective_phi < PHI_THRESHOLD * 0.8 {
            // Hysteresis: need to drop below 80% to lose consciousness
            state.is_conscious = false;
        }
    }

    /// Detect emergent properties from collective dynamics
    fn detect_emergent_properties(&mut self) {
        let mut emergent = Vec::new();
        let nodes = self.nodes.read();
        let state = self.state.read();

        // Check for superlinear consciousness
        let individual_sum: f64 = nodes.values().filter(|n| n.active).map(|n| n.phi).sum();
        if state.collective_phi > individual_sum * 1.1 {
            emergent.push(EmergentProperty::SuperlinearConsciousness {
                ratio: state.collective_phi / individual_sum.max(0.001),
            });
        }

        // Check for global synchronization
        if state.global_synchrony > 0.8 {
            emergent.push(EmergentProperty::GlobalSync {
                strength: state.global_synchrony,
            });
        }

        // Check for unified attention
        let coherence = self.coherence_tracker.overall_coherence();
        if coherence > 0.9 {
            emergent.push(EmergentProperty::UnifiedAttention {
                target: "collective_focus".to_string(),
            });
        }

        // Check for swarm behavior (high sync + high activity)
        if state.global_synchrony > 0.7 && state.cognitive_bandwidth > 1000.0 {
            emergent.push(EmergentProperty::SwarmBehavior {
                pattern: "coordinated_cognition".to_string(),
            });
        }

        drop(nodes);
        drop(state);

        self.state.write().emergent_properties = emergent;
    }

    /// Broadcast a thought to all nodes
    pub fn broadcast_thought(&self, thought_spikes: Vec<Spike>) -> Vec<Uuid> {
        let nodes = self.nodes.read();
        nodes
            .values()
            .filter(|n| n.active)
            .map(|n| n.id)
            .collect()
    }

    /// Get collective attention focus
    pub fn attention_focus(&self) -> Vec<NeuronId> {
        // The neurons that are most synchronized represent collective attention
        self.synchrony_detector
            .sync_groups()
            .iter()
            .flatten()
            .cloned()
            .collect()
    }

    /// Merge consciousness from another collective (for scaling)
    pub fn merge(&mut self, other: &CollectiveConsciousness) {
        let other_nodes = other.nodes.read();
        for (id, node) in other_nodes.iter() {
            self.nodes.write().insert(*id, node.clone());
        }
        self.update_collective_state();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collective_creation() {
        let config = CollectiveConfig::default();
        let collective = CollectiveConsciousness::new(config);
        assert!(!collective.is_conscious());
        assert_eq!(collective.collective_phi(), 0.0);
    }

    #[test]
    fn test_add_nodes() {
        let config = CollectiveConfig::default();
        let mut collective = CollectiveConsciousness::new(config);

        let node1 = ConsciousnessNode::new("Node1");
        let node2 = ConsciousnessNode::new("Node2");

        collective.add_node(node1);
        collective.add_node(node2);

        assert_eq!(collective.nodes().len(), 2);
    }

    #[test]
    fn test_collective_phi() {
        let config = CollectiveConfig::default();
        let mut collective = CollectiveConsciousness::new(config);

        // Add nodes with high phi
        for i in 0..5 {
            let mut node = ConsciousnessNode::new(format!("Node{}", i));
            node.phi = 2.0;
            node.collective_sync = 0.8;
            collective.add_node(node);
        }

        // Collective phi should be > sum due to sync bonus
        let state = collective.state();
        assert!(state.collective_phi > 10.0); // 5 * 2.0 * sync_bonus
    }

    #[test]
    fn test_awakening() {
        let config = CollectiveConfig {
            min_nodes: 2,
            target_phi: 3.0,
            ..Default::default()
        };
        let mut collective = CollectiveConsciousness::new(config);

        // Not enough nodes
        assert!(collective.awaken().is_err());

        // Add conscious nodes
        for i in 0..3 {
            let mut node = ConsciousnessNode::new(format!("Node{}", i));
            node.phi = 2.0;
            node.collective_sync = 0.9;
            collective.add_node(node);
        }

        // Now should awaken
        assert!(collective.awaken().is_ok());
        assert!(collective.is_conscious());
    }

    #[test]
    fn test_emergent_properties() {
        let mut config = CollectiveConfig::default();
        config.detect_emergence = true;
        let mut collective = CollectiveConsciousness::new(config);

        // Add highly synchronized nodes
        for i in 0..5 {
            let mut node = ConsciousnessNode::new(format!("Node{}", i));
            node.phi = 2.0;
            node.collective_sync = 0.95;
            node.spike_rate = 500.0;
            collective.add_node(node);
        }

        collective.step(Duration::from_millis(100));

        let state = collective.state();
        assert!(!state.emergent_properties.is_empty());
    }
}
