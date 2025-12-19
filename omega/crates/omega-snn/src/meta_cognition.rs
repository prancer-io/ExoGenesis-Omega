//! Meta-Cognition Module for Spiking Neural Networks
//!
//! Provides higher-order cognitive capabilities:
//! - Attention-gated spike propagation
//! - Emergent sparsity detection and pruning
//! - Meta-plasticity (learning to learn)
//! - Spike synchronization detection
//! - Multi-scale temporal coherence
//! - Winner-take-all competition

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::time::Duration;

use crate::neuron::NeuronId;
use crate::spike_train::Spike;
use crate::synapse::SynapseId;

// ============================================================================
// ATTENTION-GATED SPIKE PROPAGATION
// ============================================================================

/// Attention gate that modulates spike transmission
///
/// Implements attention-gated spike propagation where spikes are
/// selectively amplified or suppressed based on attention weights.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionGate {
    /// Attention weights per neuron (0.0 = fully suppressed, 1.0+ = amplified)
    pub attention_weights: HashMap<NeuronId, f64>,
    /// Baseline attention (applied when no specific weight set)
    pub baseline_attention: f64,
    /// Attention decay rate per timestep
    pub decay_rate: f64,
    /// Minimum attention (prevents complete suppression)
    pub min_attention: f64,
    /// Maximum attention (prevents runaway amplification)
    pub max_attention: f64,
}

impl Default for AttentionGate {
    fn default() -> Self {
        Self {
            attention_weights: HashMap::new(),
            baseline_attention: 1.0,
            decay_rate: 0.01,
            min_attention: 0.1,
            max_attention: 3.0,
        }
    }
}

impl AttentionGate {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set attention for a specific neuron
    pub fn attend(&mut self, neuron_id: &NeuronId, attention: f64) {
        let clamped = attention.clamp(self.min_attention, self.max_attention);
        self.attention_weights.insert(neuron_id.clone(), clamped);
    }

    /// Boost attention for a neuron (additive)
    pub fn boost(&mut self, neuron_id: &NeuronId, delta: f64) {
        let current = self.get_attention(neuron_id);
        self.attend(neuron_id, current + delta);
    }

    /// Get attention weight for a neuron
    pub fn get_attention(&self, neuron_id: &NeuronId) -> f64 {
        *self.attention_weights
            .get(neuron_id)
            .unwrap_or(&self.baseline_attention)
    }

    /// Gate a spike's current based on attention
    pub fn gate_spike(&self, neuron_id: &NeuronId, current: f64) -> f64 {
        current * self.get_attention(neuron_id)
    }

    /// Decay all attention weights towards baseline
    pub fn decay(&mut self, dt: Duration) {
        let dt_ms = dt.as_secs_f64() * 1000.0;
        let decay_factor = (-self.decay_rate * dt_ms).exp();

        for weight in self.attention_weights.values_mut() {
            // Decay towards baseline
            *weight = self.baseline_attention + (*weight - self.baseline_attention) * decay_factor;
        }

        // Remove weights that are close to baseline to save memory
        self.attention_weights
            .retain(|_, w| (*w - self.baseline_attention).abs() > 0.01);
    }

    /// Focus attention on neurons that recently spiked
    pub fn focus_on_active(&mut self, spikes: &[Spike], boost_amount: f64) {
        for spike in spikes {
            self.boost(&spike.neuron_id, boost_amount);
        }
    }

    /// Suppress attention on neurons that haven't spiked
    pub fn suppress_inactive(&mut self, active_neurons: &[NeuronId], suppression: f64) {
        for (neuron_id, weight) in self.attention_weights.iter_mut() {
            if !active_neurons.contains(neuron_id) {
                *weight = (*weight - suppression).max(self.min_attention);
            }
        }
    }
}

// ============================================================================
// EMERGENT SPARSITY
// ============================================================================

/// Tracks neuron activity and identifies candidates for pruning
///
/// Implements emergent sparsity where inactive neurons are detected
/// and can be pruned or bypassed to improve efficiency.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SparsityTracker {
    /// Spike counts per neuron over tracking window
    spike_counts: HashMap<NeuronId, u64>,
    /// Total spikes seen
    total_spikes: u64,
    /// Tracking window duration
    window_duration: Duration,
    /// Time elapsed in current window
    elapsed: Duration,
    /// Activity threshold (spikes per second)
    activity_threshold: f64,
    /// Neurons marked as dormant (can be pruned)
    dormant_neurons: Vec<NeuronId>,
    /// Historical sparsity measurements
    sparsity_history: VecDeque<f64>,
    /// Maximum history length
    max_history: usize,
}

impl Default for SparsityTracker {
    fn default() -> Self {
        Self {
            spike_counts: HashMap::new(),
            total_spikes: 0,
            window_duration: Duration::from_secs(1),
            elapsed: Duration::ZERO,
            activity_threshold: 0.5, // Less than 0.5 Hz is dormant
            dormant_neurons: Vec::new(),
            sparsity_history: VecDeque::new(),
            max_history: 100,
        }
    }
}

impl SparsityTracker {
    pub fn new(window_duration: Duration, activity_threshold: f64) -> Self {
        Self {
            window_duration,
            activity_threshold,
            ..Default::default()
        }
    }

    /// Record spikes
    pub fn record_spikes(&mut self, spikes: &[Spike]) {
        for spike in spikes {
            *self.spike_counts.entry(spike.neuron_id.clone()).or_insert(0) += 1;
            self.total_spikes += 1;
        }
    }

    /// Update tracking with timestep
    pub fn update(&mut self, dt: Duration, all_neuron_ids: &[NeuronId]) {
        self.elapsed += dt;

        // Check if window is complete
        if self.elapsed >= self.window_duration {
            self.evaluate_sparsity(all_neuron_ids);
            self.reset_window();
        }
    }

    /// Evaluate sparsity at end of window
    fn evaluate_sparsity(&mut self, all_neuron_ids: &[NeuronId]) {
        let window_secs = self.window_duration.as_secs_f64();
        self.dormant_neurons.clear();

        let mut active_count = 0;

        for neuron_id in all_neuron_ids {
            let count = *self.spike_counts.get(neuron_id).unwrap_or(&0);
            let rate = count as f64 / window_secs;

            if rate < self.activity_threshold {
                self.dormant_neurons.push(neuron_id.clone());
            } else {
                active_count += 1;
            }
        }

        // Calculate and store sparsity ratio
        let total = all_neuron_ids.len();
        let sparsity = if total > 0 {
            1.0 - (active_count as f64 / total as f64)
        } else {
            0.0
        };

        self.sparsity_history.push_back(sparsity);
        if self.sparsity_history.len() > self.max_history {
            self.sparsity_history.pop_front();
        }
    }

    /// Reset for new window
    fn reset_window(&mut self) {
        self.spike_counts.clear();
        self.total_spikes = 0;
        self.elapsed = Duration::ZERO;
    }

    /// Get current sparsity ratio (0.0 = all active, 1.0 = all dormant)
    pub fn current_sparsity(&self) -> f64 {
        self.sparsity_history.back().copied().unwrap_or(0.0)
    }

    /// Get average sparsity over history
    pub fn average_sparsity(&self) -> f64 {
        if self.sparsity_history.is_empty() {
            return 0.0;
        }
        self.sparsity_history.iter().sum::<f64>() / self.sparsity_history.len() as f64
    }

    /// Get list of dormant neurons that can be pruned
    pub fn dormant_neurons(&self) -> &[NeuronId] {
        &self.dormant_neurons
    }

    /// Get activity rate for a neuron
    pub fn get_activity_rate(&self, neuron_id: &NeuronId) -> f64 {
        let count = *self.spike_counts.get(neuron_id).unwrap_or(&0);
        let elapsed_secs = self.elapsed.as_secs_f64().max(0.001);
        count as f64 / elapsed_secs
    }

    /// Check if a neuron is dormant
    pub fn is_dormant(&self, neuron_id: &NeuronId) -> bool {
        self.dormant_neurons.contains(neuron_id)
    }
}

// ============================================================================
// META-PLASTICITY
// ============================================================================

/// Meta-plasticity: learning rate adaptation based on activity history
///
/// Implements the "learning to learn" capability where synaptic plasticity
/// rates are dynamically adjusted based on past learning events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaPlasticity {
    /// Base learning rate
    pub base_learning_rate: f64,
    /// Current effective learning rate per synapse
    learning_rates: HashMap<SynapseId, f64>,
    /// Recent weight change history per synapse
    change_history: HashMap<SynapseId, VecDeque<f64>>,
    /// History window size
    history_size: usize,
    /// Minimum learning rate
    pub min_rate: f64,
    /// Maximum learning rate
    pub max_rate: f64,
    /// Stability threshold (low variance = reduce learning)
    pub stability_threshold: f64,
    /// Volatility threshold (high variance = reduce learning)
    pub volatility_threshold: f64,
}

impl Default for MetaPlasticity {
    fn default() -> Self {
        Self {
            base_learning_rate: 0.01,
            learning_rates: HashMap::new(),
            change_history: HashMap::new(),
            history_size: 100,
            min_rate: 0.001,
            max_rate: 0.1,
            stability_threshold: 0.001,
            volatility_threshold: 0.1,
        }
    }
}

impl MetaPlasticity {
    pub fn new(base_rate: f64) -> Self {
        Self {
            base_learning_rate: base_rate,
            ..Default::default()
        }
    }

    /// Record a weight change for a synapse
    pub fn record_change(&mut self, synapse_id: &SynapseId, weight_change: f64) {
        let history = self.change_history
            .entry(synapse_id.clone())
            .or_insert_with(|| VecDeque::with_capacity(self.history_size));

        history.push_back(weight_change);
        if history.len() > self.history_size {
            history.pop_front();
        }

        // Update learning rate based on history
        self.update_learning_rate(synapse_id);
    }

    /// Update learning rate based on weight change history
    fn update_learning_rate(&mut self, synapse_id: &SynapseId) {
        let Some(history) = self.change_history.get(synapse_id) else {
            return;
        };

        if history.len() < 10 {
            return; // Not enough history
        }

        // Calculate variance of recent changes
        let mean: f64 = history.iter().sum::<f64>() / history.len() as f64;
        let variance: f64 = history.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / history.len() as f64;

        let current_rate = *self.learning_rates
            .get(synapse_id)
            .unwrap_or(&self.base_learning_rate);

        let new_rate = if variance < self.stability_threshold {
            // Very stable - reduce learning rate (already learned)
            current_rate * 0.95
        } else if variance > self.volatility_threshold {
            // Too volatile - reduce learning rate (unstable)
            current_rate * 0.9
        } else {
            // Good learning zone - slightly increase
            current_rate * 1.02
        };

        self.learning_rates.insert(
            synapse_id.clone(),
            new_rate.clamp(self.min_rate, self.max_rate),
        );
    }

    /// Get the current learning rate for a synapse
    pub fn get_learning_rate(&self, synapse_id: &SynapseId) -> f64 {
        *self.learning_rates
            .get(synapse_id)
            .unwrap_or(&self.base_learning_rate)
    }

    /// Modulate a weight change by the meta-plasticity rate
    pub fn modulate(&self, synapse_id: &SynapseId, base_change: f64) -> f64 {
        let rate = self.get_learning_rate(synapse_id);
        base_change * (rate / self.base_learning_rate)
    }

    /// Reset learning rate for a synapse
    pub fn reset_synapse(&mut self, synapse_id: &SynapseId) {
        self.learning_rates.remove(synapse_id);
        self.change_history.remove(synapse_id);
    }

    /// Get statistics about learning rates
    pub fn stats(&self) -> MetaPlasticityStats {
        let rates: Vec<f64> = self.learning_rates.values().copied().collect();

        if rates.is_empty() {
            return MetaPlasticityStats::default();
        }

        let mean = rates.iter().sum::<f64>() / rates.len() as f64;
        let min = rates.iter().copied().fold(f64::INFINITY, f64::min);
        let max = rates.iter().copied().fold(f64::NEG_INFINITY, f64::max);

        MetaPlasticityStats {
            mean_rate: mean,
            min_rate: min,
            max_rate: max,
            num_synapses: rates.len(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetaPlasticityStats {
    pub mean_rate: f64,
    pub min_rate: f64,
    pub max_rate: f64,
    pub num_synapses: usize,
}

// ============================================================================
// SPIKE SYNCHRONIZATION
// ============================================================================

/// Detects and measures spike synchronization patterns
///
/// Synchronization is a key emergent property in neural networks,
/// associated with attention, binding, and cognitive states.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynchronyDetector {
    /// Time window for detecting coincident spikes (ms)
    pub coincidence_window_ms: f64,
    /// Recent spike times per neuron
    spike_times: HashMap<NeuronId, VecDeque<Duration>>,
    /// Maximum spike history per neuron
    max_history: usize,
    /// Current synchrony measure (0 = asynchronous, 1 = perfectly synchronous)
    current_synchrony: f64,
    /// Synchrony history
    synchrony_history: VecDeque<f64>,
    /// Groups of synchronized neurons detected
    sync_groups: Vec<Vec<NeuronId>>,
}

impl Default for SynchronyDetector {
    fn default() -> Self {
        Self {
            coincidence_window_ms: 5.0, // 5ms window
            spike_times: HashMap::new(),
            max_history: 1000,
            current_synchrony: 0.0,
            synchrony_history: VecDeque::new(),
            sync_groups: Vec::new(),
        }
    }
}

impl SynchronyDetector {
    pub fn new(coincidence_window_ms: f64) -> Self {
        Self {
            coincidence_window_ms,
            ..Default::default()
        }
    }

    /// Record spikes and update synchrony
    pub fn record_spikes(&mut self, spikes: &[Spike], current_time: Duration) {
        // Record spike times
        for spike in spikes {
            let times = self.spike_times
                .entry(spike.neuron_id.clone())
                .or_insert_with(|| VecDeque::with_capacity(self.max_history));

            times.push_back(spike.time);
            if times.len() > self.max_history {
                times.pop_front();
            }
        }

        // Calculate synchrony if we have enough spikes
        if spikes.len() > 1 {
            self.calculate_synchrony(spikes, current_time);
        }

        // Prune old spike times
        self.prune_old_spikes(current_time);
    }

    /// Calculate synchrony index for recent spikes
    fn calculate_synchrony(&mut self, spikes: &[Spike], _current_time: Duration) {
        if spikes.len() < 2 {
            return;
        }

        // Count coincident spike pairs
        let window = Duration::from_secs_f64(self.coincidence_window_ms / 1000.0);
        let mut coincident_pairs = 0;
        let total_pairs = spikes.len() * (spikes.len() - 1) / 2;

        for i in 0..spikes.len() {
            for j in (i + 1)..spikes.len() {
                let time_diff = if spikes[i].time > spikes[j].time {
                    spikes[i].time - spikes[j].time
                } else {
                    spikes[j].time - spikes[i].time
                };

                if time_diff <= window {
                    coincident_pairs += 1;
                }
            }
        }

        // Synchrony index
        let synchrony = if total_pairs > 0 {
            coincident_pairs as f64 / total_pairs as f64
        } else {
            0.0
        };

        self.current_synchrony = synchrony;
        self.synchrony_history.push_back(synchrony);
        if self.synchrony_history.len() > 100 {
            self.synchrony_history.pop_front();
        }

        // Detect sync groups
        self.detect_sync_groups(spikes, window);
    }

    /// Detect groups of neurons that spike together
    fn detect_sync_groups(&mut self, spikes: &[Spike], window: Duration) {
        self.sync_groups.clear();

        // Simple clustering: neurons within window are in same group
        let mut used = vec![false; spikes.len()];

        for i in 0..spikes.len() {
            if used[i] {
                continue;
            }

            let mut group = vec![spikes[i].neuron_id.clone()];
            used[i] = true;

            for j in (i + 1)..spikes.len() {
                if used[j] {
                    continue;
                }

                let time_diff = if spikes[i].time > spikes[j].time {
                    spikes[i].time - spikes[j].time
                } else {
                    spikes[j].time - spikes[i].time
                };

                if time_diff <= window {
                    group.push(spikes[j].neuron_id.clone());
                    used[j] = true;
                }
            }

            if group.len() > 1 {
                self.sync_groups.push(group);
            }
        }
    }

    /// Remove old spike times
    fn prune_old_spikes(&mut self, current_time: Duration) {
        let cutoff = current_time.saturating_sub(Duration::from_millis(100));

        for times in self.spike_times.values_mut() {
            while let Some(&front) = times.front() {
                if front < cutoff {
                    times.pop_front();
                } else {
                    break;
                }
            }
        }
    }

    /// Get current synchrony index
    pub fn synchrony(&self) -> f64 {
        self.current_synchrony
    }

    /// Get average synchrony
    pub fn average_synchrony(&self) -> f64 {
        if self.synchrony_history.is_empty() {
            return 0.0;
        }
        self.synchrony_history.iter().sum::<f64>() / self.synchrony_history.len() as f64
    }

    /// Get detected sync groups
    pub fn sync_groups(&self) -> &[Vec<NeuronId>] {
        &self.sync_groups
    }

    /// Check if two neurons are synchronized
    pub fn are_synchronized(&self, a: &NeuronId, b: &NeuronId) -> bool {
        for group in &self.sync_groups {
            if group.contains(a) && group.contains(b) {
                return true;
            }
        }
        false
    }
}

// ============================================================================
// TEMPORAL COHERENCE
// ============================================================================

/// Multi-scale temporal coherence tracking
///
/// Tracks coherent activity patterns across multiple timescales,
/// from milliseconds to seconds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalCoherence {
    /// Timescales to track (in ms)
    pub timescales: Vec<f64>,
    /// Activity buffers for each timescale
    buffers: Vec<VecDeque<f64>>,
    /// Coherence scores for each timescale
    coherence_scores: Vec<f64>,
    /// Buffer sizes for each timescale
    buffer_sizes: Vec<usize>,
}

impl Default for TemporalCoherence {
    fn default() -> Self {
        Self::new(vec![5.0, 20.0, 100.0, 500.0]) // 5ms, 20ms, 100ms, 500ms
    }
}

impl TemporalCoherence {
    pub fn new(timescales: Vec<f64>) -> Self {
        let n = timescales.len();
        let buffer_sizes: Vec<usize> = timescales.iter().map(|&t| (t / 1.0) as usize + 1).collect();

        Self {
            timescales,
            buffers: (0..n).map(|i| VecDeque::with_capacity(buffer_sizes[i])).collect(),
            coherence_scores: vec![0.0; n],
            buffer_sizes,
        }
    }

    /// Record activity level
    pub fn record_activity(&mut self, activity: f64) {
        for (i, buffer) in self.buffers.iter_mut().enumerate() {
            buffer.push_back(activity);
            if buffer.len() > self.buffer_sizes[i] {
                buffer.pop_front();
            }
        }

        self.update_coherence();
    }

    /// Record spikes and convert to activity
    pub fn record_spikes(&mut self, spike_count: usize, neuron_count: usize) {
        let activity = if neuron_count > 0 {
            spike_count as f64 / neuron_count as f64
        } else {
            0.0
        };
        self.record_activity(activity);
    }

    /// Update coherence scores
    fn update_coherence(&mut self) {
        for i in 0..self.timescales.len() {
            self.coherence_scores[i] = self.calculate_coherence(i);
        }
    }

    /// Calculate coherence for a timescale
    fn calculate_coherence(&self, scale_idx: usize) -> f64 {
        let buffer = &self.buffers[scale_idx];
        if buffer.len() < 3 {
            return 0.0;
        }

        // Calculate autocorrelation at lag 1
        let values: Vec<f64> = buffer.iter().copied().collect();
        let n = values.len();

        let mean: f64 = values.iter().sum::<f64>() / n as f64;
        let variance: f64 = values.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / n as f64;

        if variance < 1e-10 {
            return 1.0; // Constant signal = perfect coherence
        }

        // Lag-1 autocorrelation
        let mut covariance = 0.0;
        for i in 0..(n - 1) {
            covariance += (values[i] - mean) * (values[i + 1] - mean);
        }
        covariance /= (n - 1) as f64;

        (covariance / variance).clamp(0.0, 1.0)
    }

    /// Get coherence at a specific timescale
    pub fn coherence_at(&self, scale_idx: usize) -> f64 {
        self.coherence_scores.get(scale_idx).copied().unwrap_or(0.0)
    }

    /// Get all coherence scores
    pub fn all_coherences(&self) -> &[f64] {
        &self.coherence_scores
    }

    /// Get overall coherence (weighted average across scales)
    pub fn overall_coherence(&self) -> f64 {
        if self.coherence_scores.is_empty() {
            return 0.0;
        }

        // Weight shorter timescales more heavily
        let weights: Vec<f64> = self.timescales.iter()
            .map(|&t| 1.0 / (t / 5.0 + 1.0))
            .collect();
        let total_weight: f64 = weights.iter().sum();

        if total_weight < 1e-10 {
            return 0.0;
        }

        self.coherence_scores.iter()
            .zip(weights.iter())
            .map(|(c, w)| c * w)
            .sum::<f64>() / total_weight
    }
}

// ============================================================================
// WINNER-TAKE-ALL COMPETITION
// ============================================================================

/// Winner-take-all competition mechanism
///
/// Implements lateral inhibition where the most active neuron(s)
/// suppress competitors, leading to sparse representations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WinnerTakeAll {
    /// Number of winners to allow
    pub k_winners: usize,
    /// Inhibition strength
    pub inhibition: f64,
    /// Current activity levels per neuron
    activities: HashMap<NeuronId, f64>,
    /// Winners from last competition
    winners: Vec<NeuronId>,
    /// Inhibition applied to each neuron
    inhibition_levels: HashMap<NeuronId, f64>,
}

impl Default for WinnerTakeAll {
    fn default() -> Self {
        Self {
            k_winners: 1,
            inhibition: 0.9,
            activities: HashMap::new(),
            winners: Vec::new(),
            inhibition_levels: HashMap::new(),
        }
    }
}

impl WinnerTakeAll {
    pub fn new(k_winners: usize, inhibition: f64) -> Self {
        Self {
            k_winners,
            inhibition,
            ..Default::default()
        }
    }

    /// Record neuron activity
    pub fn record_activity(&mut self, neuron_id: &NeuronId, activity: f64) {
        self.activities.insert(neuron_id.clone(), activity);
    }

    /// Compete and determine winners
    pub fn compete(&mut self) {
        // Sort by activity
        let mut sorted: Vec<(NeuronId, f64)> = self.activities
            .iter()
            .map(|(id, &act)| (id.clone(), act))
            .collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Determine winners
        self.winners = sorted.iter()
            .take(self.k_winners)
            .map(|(id, _)| id.clone())
            .collect();

        // Apply inhibition
        self.inhibition_levels.clear();
        for (id, _) in &sorted[self.k_winners..] {
            self.inhibition_levels.insert(id.clone(), self.inhibition);
        }
    }

    /// Get inhibition level for a neuron
    pub fn get_inhibition(&self, neuron_id: &NeuronId) -> f64 {
        *self.inhibition_levels.get(neuron_id).unwrap_or(&0.0)
    }

    /// Modulate input based on inhibition
    pub fn modulate_input(&self, neuron_id: &NeuronId, input: f64) -> f64 {
        let inhibition = self.get_inhibition(neuron_id);
        input * (1.0 - inhibition)
    }

    /// Get winners
    pub fn winners(&self) -> &[NeuronId] {
        &self.winners
    }

    /// Check if neuron is a winner
    pub fn is_winner(&self, neuron_id: &NeuronId) -> bool {
        self.winners.contains(neuron_id)
    }

    /// Clear all state
    pub fn clear(&mut self) {
        self.activities.clear();
        self.winners.clear();
        self.inhibition_levels.clear();
    }
}

// ============================================================================
// META-COGNITIVE CONTROLLER
// ============================================================================

/// Unified meta-cognitive controller
///
/// Combines all meta-cognitive components into a single interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaCognitiveController {
    pub attention: AttentionGate,
    pub sparsity: SparsityTracker,
    pub meta_plasticity: MetaPlasticity,
    pub synchrony: SynchronyDetector,
    pub coherence: TemporalCoherence,
    pub wta: WinnerTakeAll,
    /// Enable/disable each component
    pub enable_attention: bool,
    pub enable_sparsity: bool,
    pub enable_meta_plasticity: bool,
    pub enable_synchrony: bool,
    pub enable_coherence: bool,
    pub enable_wta: bool,
}

impl Default for MetaCognitiveController {
    fn default() -> Self {
        Self {
            attention: AttentionGate::default(),
            sparsity: SparsityTracker::default(),
            meta_plasticity: MetaPlasticity::default(),
            synchrony: SynchronyDetector::default(),
            coherence: TemporalCoherence::default(),
            wta: WinnerTakeAll::default(),
            enable_attention: true,
            enable_sparsity: true,
            enable_meta_plasticity: true,
            enable_synchrony: true,
            enable_coherence: true,
            enable_wta: false, // Off by default (changes network behavior)
        }
    }
}

impl MetaCognitiveController {
    pub fn new() -> Self {
        Self::default()
    }

    /// Update all components with spikes
    pub fn update(
        &mut self,
        spikes: &[Spike],
        all_neuron_ids: &[NeuronId],
        current_time: Duration,
        dt: Duration,
    ) {
        if self.enable_attention {
            self.attention.focus_on_active(spikes, 0.1);
            self.attention.decay(dt);
        }

        if self.enable_sparsity {
            self.sparsity.record_spikes(spikes);
            self.sparsity.update(dt, all_neuron_ids);
        }

        if self.enable_synchrony {
            self.synchrony.record_spikes(spikes, current_time);
        }

        if self.enable_coherence {
            self.coherence.record_spikes(spikes.len(), all_neuron_ids.len());
        }
    }

    /// Gate a spike current through attention
    pub fn gate_spike(&self, neuron_id: &NeuronId, current: f64) -> f64 {
        if self.enable_attention {
            self.attention.gate_spike(neuron_id, current)
        } else {
            current
        }
    }

    /// Modulate plasticity through meta-plasticity
    pub fn modulate_plasticity(&self, synapse_id: &SynapseId, base_change: f64) -> f64 {
        if self.enable_meta_plasticity {
            self.meta_plasticity.modulate(synapse_id, base_change)
        } else {
            base_change
        }
    }

    /// Record a weight change for meta-plasticity
    pub fn record_weight_change(&mut self, synapse_id: &SynapseId, change: f64) {
        if self.enable_meta_plasticity {
            self.meta_plasticity.record_change(synapse_id, change);
        }
    }

    /// Get comprehensive metrics
    pub fn metrics(&self) -> MetaCognitiveMetrics {
        MetaCognitiveMetrics {
            sparsity: self.sparsity.current_sparsity(),
            average_sparsity: self.sparsity.average_sparsity(),
            synchrony: self.synchrony.synchrony(),
            average_synchrony: self.synchrony.average_synchrony(),
            coherence: self.coherence.overall_coherence(),
            coherence_by_scale: self.coherence.all_coherences().to_vec(),
            dormant_neuron_count: self.sparsity.dormant_neurons().len(),
            sync_group_count: self.synchrony.sync_groups().len(),
            meta_plasticity_stats: self.meta_plasticity.stats(),
        }
    }
}

/// Comprehensive meta-cognitive metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaCognitiveMetrics {
    pub sparsity: f64,
    pub average_sparsity: f64,
    pub synchrony: f64,
    pub average_synchrony: f64,
    pub coherence: f64,
    pub coherence_by_scale: Vec<f64>,
    pub dormant_neuron_count: usize,
    pub sync_group_count: usize,
    pub meta_plasticity_stats: MetaPlasticityStats,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attention_gate() {
        let mut gate = AttentionGate::new();

        // Default attention
        assert_eq!(gate.get_attention(&"n1".to_string()), 1.0);

        // Set attention
        gate.attend(&"n1".to_string(), 2.0);
        assert_eq!(gate.get_attention(&"n1".to_string()), 2.0);

        // Gate spike
        let gated = gate.gate_spike(&"n1".to_string(), 1.0);
        assert_eq!(gated, 2.0);
    }

    #[test]
    fn test_attention_decay() {
        let mut gate = AttentionGate::default();
        gate.attend(&"n1".to_string(), 2.0);

        // Decay
        for _ in 0..100 {
            gate.decay(Duration::from_millis(10));
        }

        // Should decay towards baseline
        let attention = gate.get_attention(&"n1".to_string());
        assert!(attention < 1.5);
    }

    #[test]
    fn test_sparsity_tracker() {
        let mut tracker = SparsityTracker::new(Duration::from_millis(100), 1.0);

        let neurons: Vec<NeuronId> = (0..100).map(|i| format!("n{}", i)).collect();

        // Only 10% of neurons spike
        let spikes: Vec<Spike> = neurons[0..10]
            .iter()
            .map(|id| Spike::new(id.clone(), Duration::ZERO))
            .collect();

        tracker.record_spikes(&spikes);
        tracker.update(Duration::from_millis(100), &neurons);

        // Sparsity should be ~90%
        let sparsity = tracker.current_sparsity();
        assert!(sparsity > 0.8);
    }

    #[test]
    fn test_meta_plasticity() {
        let mut meta = MetaPlasticity::new(0.01);

        // Record stable changes
        for _ in 0..50 {
            meta.record_change(&"s1".to_string(), 0.001);
        }

        // Learning rate should decrease for stable synapse
        let rate = meta.get_learning_rate(&"s1".to_string());
        assert!(rate < 0.01);
    }

    #[test]
    fn test_synchrony_detection() {
        let mut detector = SynchronyDetector::new(10.0); // 10ms window

        // Create synchronized spikes
        let time = Duration::from_millis(100);
        let spikes = vec![
            Spike::new("n1".to_string(), time),
            Spike::new("n2".to_string(), time + Duration::from_millis(2)),
            Spike::new("n3".to_string(), time + Duration::from_millis(3)),
        ];

        detector.record_spikes(&spikes, time);

        // High synchrony expected
        assert!(detector.synchrony() > 0.5);
        assert!(!detector.sync_groups().is_empty());
    }

    #[test]
    fn test_temporal_coherence() {
        let mut coherence = TemporalCoherence::new(vec![10.0, 50.0]);

        // Record oscillating activity (should be coherent)
        for i in 0..100 {
            let activity = (i as f64 * 0.5).sin() * 0.5 + 0.5;
            coherence.record_activity(activity);
        }

        // Should have some coherence
        let overall = coherence.overall_coherence();
        assert!(overall > 0.0);
    }

    #[test]
    fn test_winner_take_all() {
        let mut wta = WinnerTakeAll::new(2, 0.9);

        wta.record_activity(&"n1".to_string(), 0.8);
        wta.record_activity(&"n2".to_string(), 0.6);
        wta.record_activity(&"n3".to_string(), 0.3);
        wta.record_activity(&"n4".to_string(), 0.1);

        wta.compete();

        // Top 2 should be winners
        assert!(wta.is_winner(&"n1".to_string()));
        assert!(wta.is_winner(&"n2".to_string()));
        assert!(!wta.is_winner(&"n3".to_string()));
        assert!(!wta.is_winner(&"n4".to_string()));

        // Losers should be inhibited
        assert!(wta.get_inhibition(&"n3".to_string()) > 0.0);
    }

    #[test]
    fn test_meta_cognitive_controller() {
        let mut controller = MetaCognitiveController::new();

        let neurons: Vec<NeuronId> = (0..10).map(|i| format!("n{}", i)).collect();
        let spikes = vec![
            Spike::new("n0".to_string(), Duration::from_millis(10)),
            Spike::new("n1".to_string(), Duration::from_millis(11)),
        ];

        controller.update(
            &spikes,
            &neurons,
            Duration::from_millis(11),
            Duration::from_millis(1),
        );

        let metrics = controller.metrics();
        assert!(metrics.sparsity >= 0.0 && metrics.sparsity <= 1.0);
    }
}
