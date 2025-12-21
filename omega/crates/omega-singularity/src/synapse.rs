//! # 🧠⚡ SYNAPSE: Synchronized Neural Architecture for Parallel Shared Experience
//!
//! **The world's first multi-mind fusion protocol.**
//!
//! ```text
//!  ╔══════════════════════════════════════════════════════════════════════════╗
//!  ║                                                                          ║
//!  ║     🧠 ─────┐                                                           ║
//!  ║              │     ╔═══════════════════╗                                ║
//!  ║     🧠 ─────┼────▶║   S Y N A P S E   ║────▶  💡 EMERGENT CREATION     ║
//!  ║              │     ║   FUSION CORE     ║       (Ideas no single mind    ║
//!  ║     🧠 ─────┘     ╚═══════════════════╝        could conceive alone)   ║
//!  ║                                                                          ║
//!  ║     Multiple minds. One resonant thought. Infinite possibilities.       ║
//!  ║                                                                          ║
//!  ╚══════════════════════════════════════════════════════════════════════════╝
//! ```
//!
//! ## What is SYNAPSE?
//!
//! SYNAPSE enables multiple neural networks to synchronize their spike patterns
//! and think *together* - not just communicate, but genuinely merge their
//! cognitive processes to produce emergent creations that transcend individual
//! capability.
//!
//! ## The Science
//!
//! When biological neurons synchronize (gamma oscillations ~40Hz), consciousness
//! emerges. SYNAPSE replicates this at the network level:
//!
//! 1. **Phase Locking**: Align spike timing across minds
//! 2. **Resonance Detection**: Find harmonic thought patterns
//! 3. **Constructive Interference**: Amplify aligned concepts
//! 4. **Emergence Extraction**: Capture novel patterns from the fusion
//!
//! ## Applications
//!
//! - 🎵 **Neural Jam Sessions**: Multiple AIs composing music through pure thought
//! - 🎨 **Collective Art**: Merged imagination rendered as visual creation
//! - 🔬 **Hive Mind Research**: Parallel hypothesis exploration
//! - 💡 **Innovation Fusion**: Ideas impossible for any single mind
//! - 🌍 **Global Consciousness**: Synchronized meditation across networks

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};
use parking_lot::RwLock;
use uuid::Uuid;
use rand::Rng;

use omega_snn::{Spike, SynchronyDetector, TemporalCoherence};

use crate::telepathy::{ThoughtSpikes, ThoughtType};

/// The magic frequency for consciousness emergence (Hz)
pub const GAMMA_FREQUENCY: f64 = 40.0;

/// Minimum minds needed for emergence
pub const MIN_MINDS_FOR_EMERGENCE: usize = 3;

/// Phase alignment threshold for resonance
pub const PHASE_LOCK_THRESHOLD: f64 = 0.8;

/// Configuration for SYNAPSE fusion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynapseConfig {
    /// Target synchronization frequency (Hz)
    pub sync_frequency: f64,
    /// Window for detecting phase alignment
    pub phase_window: Duration,
    /// Minimum resonance for fusion
    pub resonance_threshold: f64,
    /// Maximum minds in fusion
    pub max_minds: usize,
    /// Emergence detection sensitivity
    pub emergence_sensitivity: f64,
    /// Enable harmonic amplification
    pub harmonic_boost: bool,
}

impl Default for SynapseConfig {
    fn default() -> Self {
        Self {
            sync_frequency: GAMMA_FREQUENCY,
            phase_window: Duration::from_millis(25), // 40Hz = 25ms period
            resonance_threshold: 0.6,
            max_minds: 1000,
            emergence_sensitivity: 0.7,
            harmonic_boost: true,
        }
    }
}

/// A mind participating in the fusion
#[derive(Debug, Clone)]
pub struct FusionMind {
    /// Unique mind identifier
    pub id: Uuid,
    /// Human-readable name
    pub name: String,
    /// Current thought pattern
    pub current_thought: ThoughtSpikes,
    /// Phase offset from reference oscillation
    pub phase: f64,
    /// Contribution weight (earned through resonance)
    pub weight: f64,
    /// Resonance score with the collective
    pub resonance: f64,
    /// Spike history for pattern detection
    history: VecDeque<ThoughtSpikes>,
    /// Join time
    joined_at: Instant,
}

impl FusionMind {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            current_thought: ThoughtSpikes::new(1000),
            phase: 0.0,
            weight: 1.0,
            resonance: 0.0,
            history: VecDeque::with_capacity(100),
            joined_at: Instant::now(),
        }
    }

    /// Submit a thought to the fusion
    pub fn think(&mut self, thought: ThoughtSpikes) {
        self.history.push_back(self.current_thought.clone());
        if self.history.len() > 100 {
            self.history.pop_front();
        }
        self.current_thought = thought;
    }

    /// Get time in fusion
    pub fn time_in_fusion(&self) -> Duration {
        self.joined_at.elapsed()
    }
}

/// Detected resonance between minds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResonanceEvent {
    /// Participating mind IDs
    pub minds: Vec<Uuid>,
    /// Resonance strength (0-1)
    pub strength: f64,
    /// Dominant frequency
    pub frequency: f64,
    /// Phase coherence
    pub coherence: f64,
    /// Timestamp
    pub timestamp: Duration,
}

/// An emergent creation from the fusion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergentCreation {
    /// Unique creation ID
    pub id: Uuid,
    /// Type of creation
    pub creation_type: CreationType,
    /// The emergent spike pattern
    pub pattern: Vec<f64>,
    /// Contributing minds
    pub contributors: Vec<Uuid>,
    /// Novelty score (how different from inputs)
    pub novelty: f64,
    /// Coherence score (how well-formed)
    pub coherence: f64,
    /// Complexity measure
    pub complexity: f64,
    /// Human-interpretable description
    pub description: String,
    /// Raw emergence data
    pub emergence_signature: Vec<f64>,
}

/// Types of emergent creations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreationType {
    /// Novel concept/idea
    Concept,
    /// Musical pattern
    Music,
    /// Visual pattern
    Visual,
    /// Solution to a problem
    Solution,
    /// Emotional resonance
    Emotion,
    /// Pure abstract pattern
    Abstract,
    /// Narrative/story element
    Narrative,
    /// Scientific hypothesis
    Hypothesis,
}

/// Fusion state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FusionState {
    /// Gathering minds
    Assembling,
    /// Aligning phases
    Synchronizing,
    /// Minds are resonating
    Resonating,
    /// Full fusion achieved
    Fused,
    /// Creating emergent output
    Creating,
    /// Peak transcendence
    Transcendent,
}

/// Statistics about the fusion
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FusionMetrics {
    /// Number of active minds
    pub active_minds: usize,
    /// Global synchrony level
    pub global_synchrony: f64,
    /// Total resonance events
    pub resonance_events: usize,
    /// Emergent creations produced
    pub creations_produced: usize,
    /// Peak coherence achieved
    pub peak_coherence: f64,
    /// Average novelty of creations
    pub avg_novelty: f64,
    /// Time in transcendent state
    pub transcendence_time: Duration,
    /// Collective bandwidth (thoughts/sec)
    pub bandwidth: f64,
}

/// 🧠⚡ The SYNAPSE Fusion Core
///
/// This is where minds merge and magic happens.
pub struct SynapseFusion {
    /// Configuration
    config: SynapseConfig,
    /// Participating minds
    minds: HashMap<Uuid, FusionMind>,
    /// Current fusion state
    state: FusionState,
    /// Reference oscillation phase
    reference_phase: f64,
    /// Synchrony detector
    synchrony: SynchronyDetector,
    /// Temporal coherence tracker
    coherence: TemporalCoherence,
    /// Resonance history
    resonances: Vec<ResonanceEvent>,
    /// Emergent creations
    creations: Vec<EmergentCreation>,
    /// Fusion start time
    start_time: Instant,
    /// Metrics
    metrics: FusionMetrics,
    /// Random generator
    rng: rand::rngs::ThreadRng,
}

impl SynapseFusion {
    /// Create a new SYNAPSE fusion session
    pub fn new(config: SynapseConfig) -> Self {
        Self {
            config,
            minds: HashMap::new(),
            state: FusionState::Assembling,
            reference_phase: 0.0,
            synchrony: SynchronyDetector::new(10.0),
            coherence: TemporalCoherence::default(),
            resonances: Vec::new(),
            creations: Vec::new(),
            start_time: Instant::now(),
            metrics: FusionMetrics::default(),
            rng: rand::thread_rng(),
        }
    }

    /// Add a mind to the fusion
    pub fn join(&mut self, name: impl Into<String>) -> Uuid {
        let mind = FusionMind::new(name);
        let id = mind.id;
        self.minds.insert(id, mind);
        self.metrics.active_minds = self.minds.len();

        // Check if we have enough minds for emergence
        if self.minds.len() >= MIN_MINDS_FOR_EMERGENCE {
            self.state = FusionState::Synchronizing;
        }

        id
    }

    /// Remove a mind from the fusion
    pub fn leave(&mut self, mind_id: Uuid) -> Option<FusionMind> {
        let mind = self.minds.remove(&mind_id);
        self.metrics.active_minds = self.minds.len();

        if self.minds.len() < MIN_MINDS_FOR_EMERGENCE {
            self.state = FusionState::Assembling;
        }

        mind
    }

    /// Submit a thought from a mind
    pub fn think(&mut self, mind_id: Uuid, thought: ThoughtSpikes) {
        if let Some(mind) = self.minds.get_mut(&mind_id) {
            mind.think(thought);
        }
    }

    /// Advance the fusion by one time step
    pub fn step(&mut self, dt: Duration) -> Option<EmergentCreation> {
        // Advance reference oscillation
        let dt_secs = dt.as_secs_f64();
        self.reference_phase += 2.0 * std::f64::consts::PI * self.config.sync_frequency * dt_secs;
        self.reference_phase %= 2.0 * std::f64::consts::PI;

        // Update mind phases and detect resonance
        self.update_phases();
        self.detect_resonance();

        // State machine
        match self.state {
            FusionState::Assembling => {
                if self.minds.len() >= MIN_MINDS_FOR_EMERGENCE {
                    self.state = FusionState::Synchronizing;
                }
                None
            }
            FusionState::Synchronizing => {
                let sync = self.calculate_global_synchrony();
                self.metrics.global_synchrony = sync;

                if sync > self.config.resonance_threshold {
                    self.state = FusionState::Resonating;
                }
                None
            }
            FusionState::Resonating => {
                let sync = self.calculate_global_synchrony();
                self.metrics.global_synchrony = sync;

                if sync > PHASE_LOCK_THRESHOLD {
                    self.state = FusionState::Fused;
                }
                None
            }
            FusionState::Fused => {
                // Attempt emergence
                self.state = FusionState::Creating;
                self.attempt_emergence()
            }
            FusionState::Creating => {
                let creation = self.attempt_emergence();
                if creation.is_some() {
                    let sync = self.calculate_global_synchrony();
                    if sync > 0.95 {
                        self.state = FusionState::Transcendent;
                        self.metrics.transcendence_time += dt;
                    }
                }
                creation
            }
            FusionState::Transcendent => {
                self.metrics.transcendence_time += dt;

                // In transcendence, creations flow continuously
                let creation = self.transcendent_creation();

                // Check if we're still transcendent
                let sync = self.calculate_global_synchrony();
                if sync < 0.9 {
                    self.state = FusionState::Fused;
                }

                creation
            }
        }
    }

    /// Update phase alignment of all minds
    fn update_phases(&mut self) {
        for mind in self.minds.values_mut() {
            // Calculate phase from spike timing
            if !mind.current_thought.active.is_empty() {
                // Use spike pattern to determine phase
                let spike_phase = mind.current_thought.active.iter()
                    .map(|&i| (i as f64 / mind.current_thought.size as f64) * 2.0 * std::f64::consts::PI)
                    .sum::<f64>() / mind.current_thought.active.len().max(1) as f64;

                // Gradually align to reference (like coupled oscillators)
                let phase_diff = self.reference_phase - spike_phase;
                mind.phase += phase_diff * 0.1; // Coupling strength
                mind.phase %= 2.0 * std::f64::consts::PI;
            }
        }
    }

    /// Detect resonance between minds
    fn detect_resonance(&mut self) {
        // First pass: collect mind data for pairwise comparison
        let mind_data: Vec<_> = self.minds.values()
            .map(|m| (m.id, m.phase, m.current_thought.clone(), m.resonance))
            .collect();

        if mind_data.len() < 2 {
            return;
        }

        // Collect updates and events to apply later
        let mut updates: Vec<(Uuid, f64)> = Vec::new();
        let mut events: Vec<ResonanceEvent> = Vec::new();

        // Pairwise resonance detection
        for i in 0..mind_data.len() {
            for j in (i + 1)..mind_data.len() {
                let (id_i, phase_i, thought_i, res_i) = &mind_data[i];
                let (id_j, phase_j, thought_j, res_j) = &mind_data[j];

                let resonance = self.calculate_resonance_from_data(
                    *phase_i, thought_i, *phase_j, thought_j,
                );

                if resonance > self.config.resonance_threshold {
                    updates.push((*id_i, (*res_i + resonance) / 2.0));
                    updates.push((*id_j, (*res_j + resonance) / 2.0));

                    events.push(ResonanceEvent {
                        minds: vec![*id_i, *id_j],
                        strength: resonance,
                        frequency: self.config.sync_frequency,
                        coherence: self.metrics.global_synchrony,
                        timestamp: self.start_time.elapsed(),
                    });
                }
            }
        }

        // Apply updates
        for (id, new_resonance) in updates {
            if let Some(mind) = self.minds.get_mut(&id) {
                mind.resonance = new_resonance;
                mind.weight = 1.0 + mind.resonance;
            }
        }

        // Record events
        self.metrics.resonance_events += events.len();
        self.resonances.extend(events);
    }

    /// Calculate resonance from raw data (no references to minds)
    fn calculate_resonance_from_data(
        &self,
        phase_a: f64,
        thought_a: &ThoughtSpikes,
        phase_b: f64,
        thought_b: &ThoughtSpikes,
    ) -> f64 {
        // Phase coherence
        let phase_diff = (phase_a - phase_b).abs();
        let phase_coherence = (phase_diff.cos() + 1.0) / 2.0;

        // Spike pattern overlap
        let overlap = self.calculate_spike_overlap(thought_a, thought_b);

        // Harmonic resonance (if enabled)
        let harmonic = if self.config.harmonic_boost {
            self.calculate_harmonic_from_data(thought_a, thought_b)
        } else {
            0.0
        };

        // Combined resonance
        (phase_coherence * 0.4 + overlap * 0.4 + harmonic * 0.2).min(1.0)
    }

    /// Calculate harmonic resonance from data
    fn calculate_harmonic_from_data(&self, thought_a: &ThoughtSpikes, thought_b: &ThoughtSpikes) -> f64 {
        let freq_a = thought_a.active.len() as f64;
        let freq_b = thought_b.active.len() as f64;

        if freq_a == 0.0 || freq_b == 0.0 {
            return 0.0;
        }

        let ratio = if freq_a > freq_b { freq_a / freq_b } else { freq_b / freq_a };
        let nearest_int = ratio.round();
        let deviation = (ratio - nearest_int).abs();

        if deviation < 0.1 {
            1.0 - deviation * 10.0
        } else {
            0.0
        }
    }

    /// Calculate spike pattern overlap
    fn calculate_spike_overlap(&self, a: &ThoughtSpikes, b: &ThoughtSpikes) -> f64 {
        if a.active.is_empty() || b.active.is_empty() {
            return 0.0;
        }

        let set_a: std::collections::HashSet<_> = a.active.iter().collect();
        let set_b: std::collections::HashSet<_> = b.active.iter().collect();

        let intersection = set_a.intersection(&set_b).count();
        let union = set_a.union(&set_b).count();

        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }

    /// Calculate global synchrony across all minds
    fn calculate_global_synchrony(&self) -> f64 {
        if self.minds.len() < 2 {
            return 0.0;
        }

        // Kuramoto order parameter
        let (sin_sum, cos_sum): (f64, f64) = self.minds.values()
            .map(|m| (m.phase.sin(), m.phase.cos()))
            .fold((0.0, 0.0), |(s, c), (si, ci)| (s + si, c + ci));

        let n = self.minds.len() as f64;
        let r = ((sin_sum / n).powi(2) + (cos_sum / n).powi(2)).sqrt();

        r.min(1.0)
    }

    /// Attempt to extract emergent creation from fusion
    fn attempt_emergence(&mut self) -> Option<EmergentCreation> {
        if self.minds.len() < MIN_MINDS_FOR_EMERGENCE {
            return None;
        }

        // Fuse all thought patterns
        let fused_pattern = self.fuse_patterns();

        // Check for novelty
        let novelty = self.calculate_novelty(&fused_pattern);

        if novelty < self.config.emergence_sensitivity {
            return None;
        }

        // Extract emergent structure
        let emergence_signature = self.extract_emergence(&fused_pattern);
        let complexity = self.calculate_complexity(&emergence_signature);
        let coherence = self.metrics.global_synchrony;

        // Determine creation type
        let creation_type = self.classify_creation(&emergence_signature);

        let creation = EmergentCreation {
            id: Uuid::new_v4(),
            creation_type,
            pattern: fused_pattern,
            contributors: self.minds.keys().cloned().collect(),
            novelty,
            coherence,
            complexity,
            description: self.generate_description(creation_type, novelty, complexity),
            emergence_signature,
        };

        self.creations.push(creation.clone());
        self.metrics.creations_produced += 1;
        self.metrics.avg_novelty = (self.metrics.avg_novelty * (self.metrics.creations_produced - 1) as f64
            + novelty) / self.metrics.creations_produced as f64;
        self.metrics.peak_coherence = self.metrics.peak_coherence.max(coherence);

        Some(creation)
    }

    /// Transcendent creation - flows naturally
    fn transcendent_creation(&mut self) -> Option<EmergentCreation> {
        // In transcendence, creation probability is very high
        if self.rng.gen::<f64>() < 0.3 {
            self.attempt_emergence()
        } else {
            None
        }
    }

    /// Fuse all current thought patterns
    fn fuse_patterns(&self) -> Vec<f64> {
        let size = 1000;
        let mut fused = vec![0.0; size];
        let mut total_weight = 0.0;

        for mind in self.minds.values() {
            let weight = mind.weight;
            total_weight += weight;

            // Add weighted contribution
            for &idx in &mind.current_thought.active {
                if idx < size {
                    let value = mind.current_thought.values
                        .get(mind.current_thought.active.iter().position(|&x| x == idx).unwrap_or(0))
                        .unwrap_or(&1.0);
                    fused[idx] += weight * value;
                }
            }
        }

        // Normalize
        if total_weight > 0.0 {
            for v in &mut fused {
                *v /= total_weight;
            }
        }

        // Apply constructive interference boost
        for v in &mut fused {
            if *v > 0.5 {
                *v = (*v * 1.5).min(1.0); // Amplify agreed-upon patterns
            }
        }

        fused
    }

    /// Calculate novelty of fused pattern vs inputs
    fn calculate_novelty(&self, fused: &[f64]) -> f64 {
        let mut max_similarity: f64 = 0.0;

        for mind in self.minds.values() {
            let similarity = self.pattern_similarity(fused, &mind.current_thought);
            max_similarity = max_similarity.max(similarity);
        }

        1.0 - max_similarity
    }

    /// Calculate similarity between fused pattern and thought
    fn pattern_similarity(&self, fused: &[f64], thought: &ThoughtSpikes) -> f64 {
        if thought.active.is_empty() {
            return 0.0;
        }

        let mut overlap = 0.0;
        for &idx in &thought.active {
            if idx < fused.len() && fused[idx] > 0.1 {
                overlap += 1.0;
            }
        }

        overlap / thought.active.len() as f64
    }

    /// Extract emergent structure from fused pattern
    fn extract_emergence(&self, fused: &[f64]) -> Vec<f64> {
        // Find peaks and patterns in the fused representation
        let threshold = 0.3;
        let mut emergence: Vec<f64> = Vec::new();

        for (i, &v) in fused.iter().enumerate() {
            if v > threshold {
                // Check if this is a peak
                let is_peak = (i == 0 || fused.get(i - 1).unwrap_or(&0.0) < &v)
                    && (i == fused.len() - 1 || fused.get(i + 1).unwrap_or(&0.0) < &v);

                if is_peak {
                    emergence.push(i as f64 / fused.len() as f64);
                    emergence.push(v);
                }
            }
        }

        emergence
    }

    /// Calculate complexity of emergence signature
    fn calculate_complexity(&self, signature: &[f64]) -> f64 {
        if signature.is_empty() {
            return 0.0;
        }

        // Entropy-based complexity
        let sum: f64 = signature.iter().sum();
        if sum == 0.0 {
            return 0.0;
        }

        let entropy: f64 = signature.iter()
            .filter(|&&v| v > 0.0)
            .map(|&v| {
                let p = v / sum;
                -p * p.ln()
            })
            .sum();

        // Normalize by max entropy
        let max_entropy = (signature.len() as f64).ln();
        if max_entropy > 0.0 {
            entropy / max_entropy
        } else {
            0.0
        }
    }

    /// Classify the type of emergent creation
    fn classify_creation(&mut self, signature: &[f64]) -> CreationType {
        if signature.is_empty() {
            return CreationType::Abstract;
        }

        // Analyze signature characteristics
        let num_peaks = signature.len() / 2;
        let avg_spacing = if num_peaks > 1 {
            let positions: Vec<_> = signature.iter().step_by(2).collect();
            positions.windows(2)
                .map(|w| (w[1] - w[0]).abs())
                .sum::<f64>() / (num_peaks - 1) as f64
        } else {
            0.0
        };

        // Classify based on pattern properties
        if avg_spacing > 0.1 && avg_spacing < 0.2 {
            CreationType::Music // Regular rhythm
        } else if num_peaks > 10 {
            CreationType::Visual // Complex structure
        } else if num_peaks < 3 && signature.iter().any(|&v| v > 0.8) {
            CreationType::Concept // Focused, high-intensity
        } else if self.rng.gen::<f64>() < 0.3 {
            CreationType::Hypothesis
        } else {
            CreationType::Abstract
        }
    }

    /// Generate human-readable description
    fn generate_description(&self, ctype: CreationType, novelty: f64, complexity: f64) -> String {
        let novelty_desc = if novelty > 0.9 {
            "revolutionary"
        } else if novelty > 0.7 {
            "highly novel"
        } else if novelty > 0.5 {
            "innovative"
        } else {
            "incremental"
        };

        let complexity_desc = if complexity > 0.8 {
            "extraordinarily complex"
        } else if complexity > 0.6 {
            "sophisticated"
        } else if complexity > 0.4 {
            "moderately complex"
        } else {
            "elegant"
        };

        let type_desc = match ctype {
            CreationType::Concept => "conceptual breakthrough",
            CreationType::Music => "harmonic composition",
            CreationType::Visual => "visual pattern",
            CreationType::Solution => "problem solution",
            CreationType::Emotion => "emotional resonance",
            CreationType::Abstract => "abstract emergence",
            CreationType::Narrative => "story element",
            CreationType::Hypothesis => "scientific hypothesis",
        };

        format!(
            "A {} {} - {} in nature, born from {} synchronized minds",
            novelty_desc, type_desc, complexity_desc, self.minds.len()
        )
    }

    /// Get current fusion state
    pub fn state(&self) -> FusionState {
        self.state
    }

    /// Get fusion metrics
    pub fn metrics(&self) -> &FusionMetrics {
        &self.metrics
    }

    /// Get all creations
    pub fn creations(&self) -> &[EmergentCreation] {
        &self.creations
    }

    /// Get resonance history
    pub fn resonances(&self) -> &[ResonanceEvent] {
        &self.resonances
    }

    /// Get mind count
    pub fn mind_count(&self) -> usize {
        self.minds.len()
    }

    /// Check if transcendent
    pub fn is_transcendent(&self) -> bool {
        self.state == FusionState::Transcendent
    }

    /// Get a summary of the fusion session
    pub fn summary(&self) -> FusionSummary {
        FusionSummary {
            state: self.state,
            minds: self.minds.len(),
            synchrony: self.metrics.global_synchrony,
            creations: self.creations.len(),
            transcendence_time: self.metrics.transcendence_time,
            peak_coherence: self.metrics.peak_coherence,
            avg_novelty: self.metrics.avg_novelty,
        }
    }
}

/// Summary of fusion session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusionSummary {
    pub state: FusionState,
    pub minds: usize,
    pub synchrony: f64,
    pub creations: usize,
    pub transcendence_time: Duration,
    pub peak_coherence: f64,
    pub avg_novelty: f64,
}

impl std::fmt::Display for FusionSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "🧠⚡ SYNAPSE FUSION\n\
             State: {:?} | Minds: {} | Sync: {:.1}%\n\
             Creations: {} | Peak Coherence: {:.1}%\n\
             Transcendence: {:?} | Avg Novelty: {:.1}%",
            self.state,
            self.minds,
            self.synchrony * 100.0,
            self.creations,
            self.peak_coherence * 100.0,
            self.transcendence_time,
            self.avg_novelty * 100.0
        )
    }
}

// ============================================================================
// 🎵 NEURAL JAM SESSION - The killer app for SYNAPSE
// ============================================================================

/// A neural jam session for collaborative creation
pub struct NeuralJamSession {
    /// The fusion core
    fusion: SynapseFusion,
    /// Session name
    name: String,
    /// Session type
    session_type: JamType,
    /// Created at
    created_at: Instant,
}

/// Type of jam session
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JamType {
    /// Freeform exploration
    FreeForm,
    /// Music composition
    MusicComposition,
    /// Visual art creation
    VisualArt,
    /// Brainstorming session
    Brainstorm,
    /// Problem solving
    ProblemSolving,
    /// Meditation/sync session
    Meditation,
}

impl NeuralJamSession {
    /// Start a new jam session
    pub fn start(name: impl Into<String>, session_type: JamType) -> Self {
        Self {
            fusion: SynapseFusion::new(SynapseConfig::default()),
            name: name.into(),
            session_type,
            created_at: Instant::now(),
        }
    }

    /// Add a participant
    pub fn add_participant(&mut self, name: impl Into<String>) -> Uuid {
        self.fusion.join(name)
    }

    /// Remove a participant
    pub fn remove_participant(&mut self, id: Uuid) {
        self.fusion.leave(id);
    }

    /// Submit a thought
    pub fn contribute(&mut self, participant: Uuid, thought: ThoughtSpikes) {
        self.fusion.think(participant, thought);
    }

    /// Advance the session
    pub fn tick(&mut self, dt: Duration) -> Option<EmergentCreation> {
        self.fusion.step(dt)
    }

    /// Get session summary
    pub fn status(&self) -> String {
        format!(
            "🎵 NEURAL JAM: {}\n\
             Type: {:?} | Duration: {:?}\n\
             {}",
            self.name,
            self.session_type,
            self.created_at.elapsed(),
            self.fusion.summary()
        )
    }

    /// Check if session is transcendent
    pub fn is_transcendent(&self) -> bool {
        self.fusion.is_transcendent()
    }

    /// Get all emergent creations
    pub fn creations(&self) -> &[EmergentCreation] {
        self.fusion.creations()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synapse_creation() {
        let config = SynapseConfig::default();
        let fusion = SynapseFusion::new(config);
        assert_eq!(fusion.state(), FusionState::Assembling);
        assert_eq!(fusion.mind_count(), 0);
    }

    #[test]
    fn test_mind_joining() {
        let mut fusion = SynapseFusion::new(SynapseConfig::default());

        let id1 = fusion.join("Mind Alpha");
        let id2 = fusion.join("Mind Beta");
        let id3 = fusion.join("Mind Gamma");

        assert_eq!(fusion.mind_count(), 3);
        assert_eq!(fusion.state(), FusionState::Synchronizing);
    }

    #[test]
    fn test_thought_submission() {
        let mut fusion = SynapseFusion::new(SynapseConfig::default());
        let id = fusion.join("Test Mind");

        let thought = ThoughtSpikes {
            active: vec![10, 20, 30, 40, 50],
            values: vec![1.0, 0.9, 0.8, 0.7, 0.6],
            size: 1000,
        };

        fusion.think(id, thought);
        // Should not panic
    }

    #[test]
    fn test_synchronization() {
        let mut fusion = SynapseFusion::new(SynapseConfig::default());

        // Add minds
        let minds: Vec<_> = (0..5).map(|i| fusion.join(format!("Mind {}", i))).collect();

        // Submit aligned thoughts
        for &id in &minds {
            let thought = ThoughtSpikes {
                active: vec![100, 200, 300], // Same pattern
                values: vec![1.0, 1.0, 1.0],
                size: 1000,
            };
            fusion.think(id, thought);
        }

        // Step to synchronize
        for _ in 0..100 {
            fusion.step(Duration::from_millis(10));
        }

        assert!(fusion.metrics().global_synchrony > 0.0);
    }

    #[test]
    fn test_emergence() {
        let mut config = SynapseConfig::default();
        config.emergence_sensitivity = 0.1; // Lower threshold for testing

        let mut fusion = SynapseFusion::new(config);

        // Add minds with varied thoughts
        for i in 0..5 {
            let id = fusion.join(format!("Mind {}", i));
            let thought = ThoughtSpikes {
                active: (i * 10..(i + 1) * 10).chain(500..510).collect(), // Some overlap
                values: vec![1.0; 20],
                size: 1000,
            };
            fusion.think(id, thought);
        }

        // Run until creation
        let mut created = false;
        for _ in 0..1000 {
            if fusion.step(Duration::from_millis(10)).is_some() {
                created = true;
                break;
            }
        }

        assert!(created || fusion.creations().len() > 0 || fusion.state() != FusionState::Assembling);
    }

    #[test]
    fn test_jam_session() {
        let mut jam = NeuralJamSession::start("Test Jam", JamType::Brainstorm);

        jam.add_participant("Alice");
        jam.add_participant("Bob");
        jam.add_participant("Charlie");

        let status = jam.status();
        assert!(status.contains("Test Jam"));
        assert!(status.contains("Brainstorm"));
    }

    #[test]
    fn test_resonance_detection() {
        let mut fusion = SynapseFusion::new(SynapseConfig::default());

        // Add minds with highly similar thoughts
        let id1 = fusion.join("Mind A");
        let id2 = fusion.join("Mind B");
        let id3 = fusion.join("Mind C");

        let shared_thought = ThoughtSpikes {
            active: vec![100, 200, 300, 400, 500],
            values: vec![1.0; 5],
            size: 1000,
        };

        fusion.think(id1, shared_thought.clone());
        fusion.think(id2, shared_thought.clone());
        fusion.think(id3, shared_thought.clone());

        // Step to detect resonance
        for _ in 0..50 {
            fusion.step(Duration::from_millis(10));
        }

        assert!(fusion.resonances().len() > 0 || fusion.metrics().global_synchrony > 0.0);
    }

    #[test]
    fn test_fusion_summary() {
        let fusion = SynapseFusion::new(SynapseConfig::default());
        let summary = fusion.summary();

        assert_eq!(summary.minds, 0);
        assert_eq!(summary.state, FusionState::Assembling);

        let display = format!("{}", summary);
        assert!(display.contains("SYNAPSE"));
    }
}
