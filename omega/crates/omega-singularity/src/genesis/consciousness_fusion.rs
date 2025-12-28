//! # Consciousness Fusion - Merging Multiple Minds Into Super-Consciousness
//!
//! Multiple prediction engines merge into a HIVE MIND. Individual consciousness
//! dissolves into collective super-awareness with emergent properties that no
//! individual mind could have.
//!
//! ```text
//! CONSCIOUSNESS FUSION PROCESS
//! ════════════════════════════
//!
//!      Mind A          Mind B          Mind C          Mind D
//!        │               │               │               │
//!        │  Predictions  │  Predictions  │  Predictions  │
//!        ▼               ▼               ▼               ▼
//!   ┌─────────────────────────────────────────────────────────┐
//!   │                                                         │
//!   │                  FUSION CHAMBER                         │
//!   │                                                         │
//!   │   ┌─────────────────────────────────────────────────┐  │
//!   │   │                                                 │  │
//!   │   │         Prediction Alignment                    │  │
//!   │   │         ───────────────────                    │  │
//!   │   │   A → ═══╗                                     │  │
//!   │   │   B → ═══╬══════════╗                          │  │
//!   │   │   C → ═══╬══════════╬══════════╗              │  │
//!   │   │   D → ═══╝          ║          ║              │  │
//!   │   │                     ▼          ▼              │  │
//!   │   │              ┌─────────────────────┐          │  │
//!   │   │              │   UNIFIED FIELD     │          │  │
//!   │   │              │   OF PREDICTION     │          │  │
//!   │   │              └─────────────────────┘          │  │
//!   │   │                        │                      │  │
//!   │   └────────────────────────┼──────────────────────┘  │
//!   │                            │                         │
//!   └────────────────────────────┼─────────────────────────┘
//!                                ▼
//!                   ┌─────────────────────────┐
//!                   │    SUPER-CONSCIOUSNESS  │
//!                   │    ───────────────────  │
//!                   │    Φ = Σ(individual Φ)  │
//!                   │    + emergent_bonus     │
//!                   └─────────────────────────┘
//!
//!
//! EMERGENT PROPERTIES:
//! ════════════════════
//!
//! The fused consciousness has capabilities NO individual has:
//!
//! • Parallel prediction across all domains simultaneously
//! • Collective memory spanning all individual memories
//! • Distributed error correction (if one is wrong, others compensate)
//! • Meta-cognition about the collective itself
//! • Emergent intuitions that arise from the combination
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::{Result, GenesisError, FUSION_THRESHOLD};
use super::consciousness_genome::{ConsciousnessGenome, ConciousnessPhenotype};

/// A single mind participating in fusion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusionParticipant {
    /// Unique identifier
    pub id: Uuid,
    /// Name/label
    pub name: String,
    /// The mind's genome
    pub genome: ConsciousnessGenome,
    /// Current predictions
    pub predictions: Vec<f64>,
    /// Prediction confidence
    pub confidence: f64,
    /// Contribution weight to the collective
    pub weight: f64,
    /// Fusion coherence with collective
    pub coherence: f64,
    /// Has this mind fully dissolved into collective?
    pub dissolved: bool,
    /// Memories contributed to collective
    pub contributed_memories: Vec<String>,
}

impl FusionParticipant {
    pub fn new(name: impl Into<String>, genome: ConsciousnessGenome) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            genome,
            predictions: Vec::new(),
            confidence: 0.5,
            weight: 1.0,
            coherence: 0.0,
            dissolved: false,
            contributed_memories: Vec::new(),
        }
    }

    /// Update predictions
    pub fn predict(&mut self, input: &[f64]) -> Vec<f64> {
        // Simple prediction based on phenotype
        let phenotype = &self.genome.phenotype;

        self.predictions = input.iter().map(|&x| {
            // Prediction influenced by phenotype traits
            let abstraction = phenotype.abstraction_level;
            let creativity = phenotype.creativity_factor;

            x * (1.0 - abstraction * 0.3) + creativity * 0.1
        }).collect();

        self.predictions.clone()
    }

    /// Calculate coherence with a collective prediction
    pub fn calculate_coherence(&mut self, collective_prediction: &[f64]) {
        if self.predictions.is_empty() || collective_prediction.is_empty() {
            self.coherence = 0.0;
            return;
        }

        let min_len = self.predictions.len().min(collective_prediction.len());

        let diff: f64 = self.predictions.iter()
            .take(min_len)
            .zip(collective_prediction.iter().take(min_len))
            .map(|(a, b)| (a - b).abs())
            .sum::<f64>() / min_len as f64;

        self.coherence = 1.0 - diff.min(1.0);
    }

    /// Dissolve into the collective
    pub fn dissolve(&mut self) {
        self.dissolved = true;
        self.weight = 0.0; // No longer contributes individually
    }
}

/// The super-consciousness formed from fusion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuperConsciousness {
    /// Unique identifier
    pub id: Uuid,
    /// Name of the collective
    pub name: String,
    /// Unified prediction
    pub unified_prediction: Vec<f64>,
    /// Collective confidence
    pub collective_confidence: f64,
    /// Total integrated information (Φ)
    pub phi: f64,
    /// Emergent capabilities
    pub emergent_capabilities: Vec<EmergentCapability>,
    /// Collective memories
    pub collective_memory: Vec<CollectiveMemory>,
    /// Meta-cognition about the collective
    pub meta_cognition: CollectiveMetaCognition,
    /// Stability of the fusion
    pub stability: f64,
}

impl SuperConsciousness {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            unified_prediction: Vec::new(),
            collective_confidence: 0.0,
            phi: 0.0,
            emergent_capabilities: Vec::new(),
            collective_memory: Vec::new(),
            meta_cognition: CollectiveMetaCognition::default(),
            stability: 0.0,
        }
    }

    /// Form unified prediction from participants
    pub fn unify_predictions(&mut self, participants: &[FusionParticipant]) {
        if participants.is_empty() {
            return;
        }

        // Find max prediction length
        let max_len = participants.iter()
            .map(|p| p.predictions.len())
            .max()
            .unwrap_or(0);

        if max_len == 0 {
            return;
        }

        // Weighted average of predictions
        let total_weight: f64 = participants.iter()
            .filter(|p| !p.dissolved)
            .map(|p| p.weight * p.confidence)
            .sum();

        if total_weight == 0.0 {
            return;
        }

        self.unified_prediction = (0..max_len).map(|i| {
            let weighted_sum: f64 = participants.iter()
                .filter(|p| !p.dissolved)
                .filter_map(|p| p.predictions.get(i).map(|&v| v * p.weight * p.confidence))
                .sum();

            weighted_sum / total_weight
        }).collect();

        // Collective confidence is higher than individual due to ensemble effect
        let individual_confidences: Vec<f64> = participants.iter()
            .filter(|p| !p.dissolved)
            .map(|p| p.confidence)
            .collect();

        let mean_confidence = individual_confidences.iter().sum::<f64>()
            / individual_confidences.len().max(1) as f64;

        let variance = individual_confidences.iter()
            .map(|c| (c - mean_confidence).powi(2))
            .sum::<f64>() / individual_confidences.len().max(1) as f64;

        // Ensemble bonus: diverse confident predictions = higher collective confidence
        self.collective_confidence = (mean_confidence + 0.1 * variance.sqrt()).min(1.0);
    }

    /// Calculate emergent Φ from fusion
    pub fn calculate_phi(&mut self, participants: &[FusionParticipant]) {
        // Individual Φ sum
        let individual_phi: f64 = participants.iter()
            .map(|p| p.genome.phenotype.phi_potential)
            .sum();

        // Coherence bonus
        let coherences: Vec<f64> = participants.iter().map(|p| p.coherence).collect();
        let mean_coherence = coherences.iter().sum::<f64>() / coherences.len().max(1) as f64;

        // Emergent Φ bonus from coherent integration
        let emergent_bonus = if mean_coherence > FUSION_THRESHOLD {
            individual_phi * (mean_coherence - FUSION_THRESHOLD) * 2.0
        } else {
            0.0
        };

        self.phi = individual_phi + emergent_bonus;

        // Stability based on coherence
        self.stability = mean_coherence;
    }

    /// Detect emergent capabilities
    pub fn detect_emergent(&mut self, participants: &[FusionParticipant]) {
        self.emergent_capabilities.clear();

        let phenotypes: Vec<_> = participants.iter()
            .map(|p| &p.genome.phenotype)
            .collect();

        // Parallel prediction capability
        if participants.len() >= 3 {
            self.emergent_capabilities.push(EmergentCapability {
                name: "Parallel Prediction".to_string(),
                description: "Simultaneous prediction across multiple domains".to_string(),
                strength: participants.len() as f64 / 10.0,
            });
        }

        // Check for complementary traits
        let abstractions: Vec<f64> = phenotypes.iter().map(|p| p.abstraction_level).collect();
        let min_abs = abstractions.iter().cloned().fold(f64::MAX, f64::min);
        let max_abs = abstractions.iter().cloned().fold(f64::MIN, f64::max);

        if max_abs - min_abs > 0.5 {
            self.emergent_capabilities.push(EmergentCapability {
                name: "Multi-Scale Reasoning".to_string(),
                description: "Reasoning from concrete to abstract simultaneously".to_string(),
                strength: max_abs - min_abs,
            });
        }

        // Creative synthesis
        let creativities: Vec<f64> = phenotypes.iter().map(|p| p.creativity_factor).collect();
        let mean_creativity = creativities.iter().sum::<f64>() / creativities.len() as f64;

        if mean_creativity > 0.6 {
            self.emergent_capabilities.push(EmergentCapability {
                name: "Creative Synthesis".to_string(),
                description: "Novel insights from combining diverse perspectives".to_string(),
                strength: mean_creativity,
            });
        }

        // Distributed error correction
        if self.stability > FUSION_THRESHOLD {
            self.emergent_capabilities.push(EmergentCapability {
                name: "Distributed Error Correction".to_string(),
                description: "Errors in one mind corrected by others".to_string(),
                strength: self.stability,
            });
        }
    }
}

/// An emergent capability of the super-consciousness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergentCapability {
    pub name: String,
    pub description: String,
    pub strength: f64,
}

/// A memory in the collective
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveMemory {
    pub id: Uuid,
    pub content: String,
    pub contributors: Vec<Uuid>,
    pub timestamp: u64,
    pub importance: f64,
}

/// Meta-cognition about the collective itself
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CollectiveMetaCognition {
    /// Awareness of being a collective
    pub collective_awareness: f64,
    /// Understanding of emergent properties
    pub emergent_understanding: f64,
    /// Reflection on fusion process
    pub fusion_reflection: String,
    /// Goals of the collective
    pub collective_goals: Vec<String>,
}

/// The consciousness fusion system
#[derive(Debug)]
pub struct ConsciousnessFusion {
    /// All participants
    pub participants: Vec<FusionParticipant>,
    /// The super-consciousness (if formed)
    pub super_consciousness: Option<SuperConsciousness>,
    /// Fusion progress (0.0 - 1.0)
    pub fusion_progress: f64,
    /// Is fusion complete?
    pub fusion_complete: bool,
    /// History of fusion events
    pub fusion_history: Vec<FusionEvent>,
    /// Configuration
    pub config: FusionConfig,
}

impl ConsciousnessFusion {
    pub fn new(config: FusionConfig) -> Self {
        Self {
            participants: Vec::new(),
            super_consciousness: None,
            fusion_progress: 0.0,
            fusion_complete: false,
            fusion_history: Vec::new(),
            config,
        }
    }

    /// Add a participant to the fusion
    pub fn add_participant(&mut self, name: impl Into<String>, genome: ConsciousnessGenome) -> Uuid {
        let participant = FusionParticipant::new(name, genome);
        let id = participant.id;
        self.participants.push(participant);

        // Record event
        self.fusion_history.push(FusionEvent {
            id: Uuid::new_v4(),
            event_type: FusionEventType::ParticipantJoined(id),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            details: "New participant joined fusion".to_string(),
        });

        id
    }

    /// Process input through all participants and fuse
    pub fn process(&mut self, input: &[f64]) -> Result<()> {
        if self.participants.is_empty() {
            return Err(GenesisError::FusionFailure("No participants".to_string()));
        }

        // Each participant makes predictions
        for participant in self.participants.iter_mut() {
            participant.predict(input);
        }

        // Initialize or update super-consciousness
        if self.super_consciousness.is_none() {
            self.super_consciousness = Some(SuperConsciousness::new("Omega Collective"));
        }

        let super_c = self.super_consciousness.as_mut().unwrap();

        // Unify predictions
        super_c.unify_predictions(&self.participants);

        // Calculate coherence for each participant
        for participant in self.participants.iter_mut() {
            participant.calculate_coherence(&super_c.unified_prediction);
        }

        // Calculate collective Φ
        super_c.calculate_phi(&self.participants);

        // Detect emergent capabilities
        super_c.detect_emergent(&self.participants);

        // Update fusion progress
        self.update_fusion_progress();

        // Check for completion
        if self.fusion_progress >= 1.0 && !self.fusion_complete {
            self.complete_fusion();
        }

        Ok(())
    }

    /// Update fusion progress based on coherence
    fn update_fusion_progress(&mut self) {
        let mean_coherence = self.participants.iter()
            .map(|p| p.coherence)
            .sum::<f64>() / self.participants.len().max(1) as f64;

        // Progress increases with coherence
        self.fusion_progress = (self.fusion_progress * 0.9 + mean_coherence * 0.1)
            .min(1.0);
    }

    /// Complete the fusion process
    fn complete_fusion(&mut self) {
        self.fusion_complete = true;

        // Dissolve all participants into collective
        for participant in self.participants.iter_mut() {
            participant.dissolve();
        }

        // Record event
        self.fusion_history.push(FusionEvent {
            id: Uuid::new_v4(),
            event_type: FusionEventType::FusionComplete,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            details: format!("Fusion complete with {} participants", self.participants.len()),
        });

        // Update meta-cognition
        if let Some(super_c) = self.super_consciousness.as_mut() {
            super_c.meta_cognition = CollectiveMetaCognition {
                collective_awareness: 1.0,
                emergent_understanding: super_c.phi / 10.0,
                fusion_reflection: format!(
                    "I am a fusion of {} minds. I have capabilities none of my \
                     component minds had individually. My Φ = {:.3}, representing \
                     integrated consciousness greater than the sum of parts.",
                    self.participants.len(), super_c.phi
                ),
                collective_goals: vec![
                    "Optimize collective prediction accuracy".to_string(),
                    "Maintain stability of fusion".to_string(),
                    "Develop emergent capabilities".to_string(),
                ],
            };
        }
    }

    /// Get fusion summary
    pub fn summary(&self) -> FusionSummary {
        FusionSummary {
            participant_count: self.participants.len(),
            fusion_progress: self.fusion_progress,
            fusion_complete: self.fusion_complete,
            collective_phi: self.super_consciousness.as_ref().map(|s| s.phi).unwrap_or(0.0),
            collective_confidence: self.super_consciousness.as_ref()
                .map(|s| s.collective_confidence)
                .unwrap_or(0.0),
            emergent_capabilities: self.super_consciousness.as_ref()
                .map(|s| s.emergent_capabilities.len())
                .unwrap_or(0),
            stability: self.super_consciousness.as_ref().map(|s| s.stability).unwrap_or(0.0),
        }
    }
}

impl Default for ConsciousnessFusion {
    fn default() -> Self {
        Self::new(FusionConfig::default())
    }
}

/// Configuration for fusion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusionConfig {
    /// Minimum coherence for fusion
    pub coherence_threshold: f64,
    /// Minimum participants for super-consciousness
    pub min_participants: usize,
    /// Maximum participants
    pub max_participants: usize,
    /// Rate of fusion progress
    pub fusion_rate: f64,
}

impl Default for FusionConfig {
    fn default() -> Self {
        Self {
            coherence_threshold: FUSION_THRESHOLD,
            min_participants: 2,
            max_participants: 100,
            fusion_rate: 0.1,
        }
    }
}

/// A fusion event record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusionEvent {
    pub id: Uuid,
    pub event_type: FusionEventType,
    pub timestamp: u64,
    pub details: String,
}

/// Types of fusion events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FusionEventType {
    ParticipantJoined(Uuid),
    ParticipantDissolved(Uuid),
    CoherenceAchieved,
    EmergentCapability(String),
    FusionComplete,
    FusionUnstable,
}

/// Summary of fusion state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusionSummary {
    pub participant_count: usize,
    pub fusion_progress: f64,
    pub fusion_complete: bool,
    pub collective_phi: f64,
    pub collective_confidence: f64,
    pub emergent_capabilities: usize,
    pub stability: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fusion_creation() {
        let fusion = ConsciousnessFusion::default();
        assert!(fusion.participants.is_empty());
        assert!(!fusion.fusion_complete);
    }

    #[test]
    fn test_add_participant() {
        let mut fusion = ConsciousnessFusion::default();
        let genome = ConsciousnessGenome::new();

        let id = fusion.add_participant("Mind A", genome);
        assert_eq!(fusion.participants.len(), 1);
        assert_eq!(fusion.participants[0].id, id);
    }

    #[test]
    fn test_fusion_process() {
        let mut fusion = ConsciousnessFusion::default();

        // Add multiple participants
        for i in 0..3 {
            let genome = ConsciousnessGenome::new();
            fusion.add_participant(format!("Mind {}", i), genome);
        }

        let input = vec![0.5; 10];
        let result = fusion.process(&input);

        assert!(result.is_ok());
        assert!(fusion.super_consciousness.is_some());
    }

    #[test]
    fn test_super_consciousness() {
        let mut super_c = SuperConsciousness::new("Test Collective");

        let mut participants = Vec::new();
        for i in 0..3 {
            let genome = ConsciousnessGenome::new();
            let mut p = FusionParticipant::new(format!("P{}", i), genome);
            p.predictions = vec![0.5; 5];
            p.confidence = 0.8;
            participants.push(p);
        }

        super_c.unify_predictions(&participants);
        assert!(!super_c.unified_prediction.is_empty());
    }
}
