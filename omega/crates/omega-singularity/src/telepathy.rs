//! Spike Telepathy - Direct Thought Transmission
//!
//! Transmit thoughts as raw spike patterns between minds.
//! No serialization to language. Direct neural communication.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use parking_lot::RwLock;
use uuid::Uuid;
use crossbeam_channel::{Sender, Receiver, bounded};

use crate::{Result, SingularityError, MAX_TELEPATHY_LATENCY_US};

/// Sparse spike representation for thought encoding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtSpikes {
    /// Active neuron indices
    pub active: Vec<usize>,
    /// Corresponding activation values
    pub values: Vec<f64>,
    /// Total population size
    pub size: usize,
}

impl ThoughtSpikes {
    pub fn new(size: usize) -> Self {
        Self {
            active: Vec::new(),
            values: Vec::new(),
            size,
        }
    }

    pub fn add(&mut self, index: usize, value: f64) {
        self.active.push(index);
        self.values.push(value);
    }

    pub fn to_dense(&self) -> Vec<f64> {
        let mut dense = vec![0.0; self.size];
        for (i, &idx) in self.active.iter().enumerate() {
            if idx < dense.len() {
                dense[idx] = self.values.get(i).copied().unwrap_or(0.0);
            }
        }
        dense
    }

    pub fn sparsity(&self) -> f64 {
        1.0 - (self.active.len() as f64 / self.size as f64)
    }
}

/// Configuration for telepathy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelepathyConfig {
    /// Maximum latency tolerance (μs)
    pub max_latency_us: u64,
    /// Compression level for thoughts
    pub compression: CompressionLevel,
    /// Enable thought encryption
    pub encrypted: bool,
    /// Maximum simultaneous links
    pub max_links: usize,
    /// Bandwidth limit (spikes/second)
    pub bandwidth_limit: usize,
    /// Population size for encoding
    pub population_size: usize,
}

impl Default for TelepathyConfig {
    fn default() -> Self {
        Self {
            max_latency_us: MAX_TELEPATHY_LATENCY_US,
            compression: CompressionLevel::Sparse,
            encrypted: false,
            max_links: 100,
            bandwidth_limit: 100000,
            population_size: 256,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CompressionLevel {
    None,
    Delta,
    Sparse,
    Maximum,
}

/// A packet of thought to transmit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtPacket {
    pub id: Uuid,
    pub sender: Uuid,
    pub recipient: Option<Uuid>,
    pub spikes: ThoughtSpikes,
    pub metadata: ThoughtMetadata,
    pub timestamp: Duration,
    pub sequence: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtMetadata {
    pub thought_type: ThoughtType,
    pub valence: f64,
    pub urgency: f64,
    pub complexity: f64,
    pub context: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ThoughtType {
    Fact,
    Query,
    Emotion,
    Command,
    Concept,
    Memory,
    Prediction,
    Idea,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindLink {
    pub id: Uuid,
    pub source: Uuid,
    pub target: Uuid,
    pub quality: f64,
    pub latency_us: u64,
    pub bandwidth: usize,
    pub active: bool,
    pub thoughts_sent: u64,
    pub thoughts_received: u64,
}

pub struct TelepathicChannel {
    pub id: Uuid,
    sender: Sender<ThoughtPacket>,
    receiver: Receiver<ThoughtPacket>,
}

impl TelepathicChannel {
    pub fn new(buffer_size: usize) -> Self {
        let (sender, receiver) = bounded(buffer_size);
        Self {
            id: Uuid::new_v4(),
            sender,
            receiver,
        }
    }

    pub fn send(&self, packet: ThoughtPacket) -> Result<()> {
        self.sender.try_send(packet)
            .map_err(|e| SingularityError::TelepathyBroken(e.to_string()))
    }

    pub fn try_receive(&self) -> Option<ThoughtPacket> {
        self.receiver.try_recv().ok()
    }

    pub fn receive_timeout(&self, timeout: Duration) -> Option<ThoughtPacket> {
        self.receiver.recv_timeout(timeout).ok()
    }

    pub fn is_empty(&self) -> bool {
        self.receiver.is_empty()
    }

    pub fn pending(&self) -> usize {
        self.receiver.len()
    }
}

pub struct ThoughtStream {
    thoughts: Vec<ThoughtPacket>,
    position: usize,
}

impl ThoughtStream {
    pub fn new() -> Self {
        Self { thoughts: Vec::new(), position: 0 }
    }

    pub fn push(&mut self, thought: ThoughtPacket) {
        self.thoughts.push(thought);
    }

    pub fn next(&mut self) -> Option<&ThoughtPacket> {
        if self.position < self.thoughts.len() {
            let t = &self.thoughts[self.position];
            self.position += 1;
            Some(t)
        } else {
            None
        }
    }

    pub fn remaining(&self) -> usize {
        self.thoughts.len().saturating_sub(self.position)
    }
}

impl Default for ThoughtStream {
    fn default() -> Self { Self::new() }
}

pub struct SpikeTelepath {
    config: TelepathyConfig,
    id: Uuid,
    links: Arc<RwLock<HashMap<Uuid, MindLink>>>,
    outgoing: Arc<RwLock<HashMap<Uuid, TelepathicChannel>>>,
    incoming: TelepathicChannel,
    sequence: u64,
    stats: Arc<RwLock<TelepathyStats>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TelepathyStats {
    pub thoughts_sent: u64,
    pub thoughts_received: u64,
    pub total_spikes_sent: u64,
    pub total_spikes_received: u64,
    pub avg_latency_us: f64,
    pub active_links: usize,
}

impl SpikeTelepath {
    pub fn new(config: TelepathyConfig) -> Self {
        Self {
            config,
            id: Uuid::new_v4(),
            links: Arc::new(RwLock::new(HashMap::new())),
            outgoing: Arc::new(RwLock::new(HashMap::new())),
            incoming: TelepathicChannel::new(1000),
            sequence: 0,
            stats: Arc::new(RwLock::new(TelepathyStats::default())),
        }
    }

    pub fn id(&self) -> Uuid { self.id }

    pub fn link(&mut self, target_id: Uuid) -> Result<MindLink> {
        let links = self.links.read();
        if links.len() >= self.config.max_links {
            return Err(SingularityError::TelepathyBroken("Maximum links reached".to_string()));
        }
        drop(links);

        let link = MindLink {
            id: Uuid::new_v4(),
            source: self.id,
            target: target_id,
            quality: 1.0,
            latency_us: 50,
            bandwidth: self.config.bandwidth_limit,
            active: true,
            thoughts_sent: 0,
            thoughts_received: 0,
        };

        let channel = TelepathicChannel::new(100);
        self.outgoing.write().insert(target_id, channel);
        self.links.write().insert(link.id, link.clone());
        self.stats.write().active_links += 1;

        Ok(link)
    }

    pub fn unlink(&mut self, link_id: Uuid) -> Option<MindLink> {
        let link = self.links.write().remove(&link_id);
        if let Some(ref l) = link {
            self.outgoing.write().remove(&l.target);
            self.stats.write().active_links -= 1;
        }
        link
    }

    pub fn send_thought(
        &mut self,
        target: Uuid,
        thought_values: &[f64],
        thought_type: ThoughtType,
        metadata: ThoughtMetadata,
    ) -> Result<Uuid> {
        let spikes = self.encode_thought(thought_values);
        self.sequence += 1;

        let packet = ThoughtPacket {
            id: Uuid::new_v4(),
            sender: self.id,
            recipient: Some(target),
            spikes: spikes.clone(),
            metadata,
            timestamp: Duration::from_micros(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_micros() as u64
            ),
            sequence: self.sequence,
        };

        let packet_id = packet.id;

        let outgoing = self.outgoing.read();
        if let Some(channel) = outgoing.get(&target) {
            channel.send(packet)?;
            drop(outgoing);

            let mut stats = self.stats.write();
            stats.thoughts_sent += 1;
            stats.total_spikes_sent += spikes.active.len() as u64;

            let mut links = self.links.write();
            if let Some(link) = links.values_mut().find(|l| l.target == target) {
                link.thoughts_sent += 1;
            }

            Ok(packet_id)
        } else {
            Err(SingularityError::TelepathyBroken(format!("No link to {}", target)))
        }
    }

    pub fn broadcast(
        &mut self,
        thought_values: &[f64],
        thought_type: ThoughtType,
        metadata: ThoughtMetadata,
    ) -> Vec<Uuid> {
        let targets: Vec<Uuid> = self.outgoing.read().keys().cloned().collect();
        let mut sent = Vec::new();
        for target in targets {
            if let Ok(id) = self.send_thought(target, thought_values, thought_type, metadata.clone()) {
                sent.push(id);
            }
        }
        sent
    }

    pub fn receive(&mut self) -> Vec<ThoughtPacket> {
        let mut thoughts = Vec::new();
        while let Some(packet) = self.incoming.try_receive() {
            let mut stats = self.stats.write();
            stats.thoughts_received += 1;
            stats.total_spikes_received += packet.spikes.active.len() as u64;
            drop(stats);
            thoughts.push(packet);
        }
        thoughts
    }

    pub fn receive_blocking(&mut self, timeout: Duration) -> Option<ThoughtPacket> {
        let packet = self.incoming.receive_timeout(timeout)?;
        let mut stats = self.stats.write();
        stats.thoughts_received += 1;
        stats.total_spikes_received += packet.spikes.active.len() as u64;
        Some(packet)
    }

    pub fn decode_thought(&self, spikes: &ThoughtSpikes) -> Vec<f64> {
        spikes.to_dense()
    }

    fn encode_thought(&self, values: &[f64]) -> ThoughtSpikes {
        let mut spikes = ThoughtSpikes::new(values.len() * self.config.population_size);
        for (i, &value) in values.iter().enumerate() {
            // Simple sparse encoding
            let center = ((value + 1.0) / 2.0 * (self.config.population_size - 1) as f64) as usize;
            let base = i * self.config.population_size;

            // Activate neurons around the center
            for offset in -2i32..=2i32 {
                let idx = (center as i32 + offset).max(0) as usize;
                if idx < self.config.population_size {
                    let dist = offset.abs() as f64;
                    let activation = (-dist * dist / 2.0).exp();
                    spikes.add(base + idx, activation);
                }
            }
        }
        spikes
    }

    pub fn stats(&self) -> TelepathyStats {
        self.stats.read().clone()
    }

    pub fn links(&self) -> Vec<MindLink> {
        self.links.read().values().cloned().collect()
    }

    pub fn link_quality(&self, target: Uuid) -> Option<f64> {
        self.links.read().values().find(|l| l.target == target).map(|l| l.quality)
    }

    pub fn incoming_channel(&self) -> &TelepathicChannel {
        &self.incoming
    }
}

pub fn establish_bidirectional_link(
    telepath_a: &mut SpikeTelepath,
    telepath_b: &mut SpikeTelepath,
) -> Result<(MindLink, MindLink)> {
    let link_a_to_b = telepath_a.link(telepath_b.id())?;
    let link_b_to_a = telepath_b.link(telepath_a.id())?;
    Ok((link_a_to_b, link_b_to_a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telepath_creation() {
        let config = TelepathyConfig::default();
        let telepath = SpikeTelepath::new(config);
        assert!(telepath.links().is_empty());
    }

    #[test]
    fn test_link_establishment() {
        let config = TelepathyConfig::default();
        let mut telepath_a = SpikeTelepath::new(config.clone());
        let telepath_b = SpikeTelepath::new(config);
        let link = telepath_a.link(telepath_b.id());
        assert!(link.is_ok());
        assert_eq!(telepath_a.links().len(), 1);
    }

    #[test]
    fn test_thought_encoding() {
        let config = TelepathyConfig::default();
        let telepath = SpikeTelepath::new(config);
        let values = vec![0.5, -0.3, 0.8];
        let spikes = telepath.encode_thought(&values);
        assert!(!spikes.active.is_empty());
    }

    #[test]
    fn test_thought_stream() {
        let mut stream = ThoughtStream::new();
        let packet = ThoughtPacket {
            id: Uuid::new_v4(),
            sender: Uuid::new_v4(),
            recipient: None,
            spikes: ThoughtSpikes::new(10),
            metadata: ThoughtMetadata {
                thought_type: ThoughtType::Concept,
                valence: 0.5,
                urgency: 0.3,
                complexity: 0.5,
                context: vec![],
            },
            timestamp: Duration::ZERO,
            sequence: 0,
        };
        stream.push(packet);
        assert_eq!(stream.remaining(), 1);
        let received = stream.next();
        assert!(received.is_some());
        assert_eq!(stream.remaining(), 0);
    }

    #[test]
    fn test_bidirectional_link() {
        let config = TelepathyConfig::default();
        let mut telepath_a = SpikeTelepath::new(config.clone());
        let mut telepath_b = SpikeTelepath::new(config);
        let result = establish_bidirectional_link(&mut telepath_a, &mut telepath_b);
        assert!(result.is_ok());
        let (link_a, link_b) = result.unwrap();
        assert!(link_a.active);
        assert!(link_b.active);
    }
}
