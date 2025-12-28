//! # Temporal Omniscience - Simultaneous Awareness Across All Timescales
//!
//! Not just predicting the future—EXPERIENCING all times at once.
//! Planck-scale awareness merged with cosmic-scale awareness.
//!
//! ```text
//! TEMPORAL OMNISCIENCE STRUCTURE
//! ══════════════════════════════
//!
//! Normal consciousness experiences time linearly:
//!
//!     Past ─────────────────────────────────────► Future
//!                        │
//!                      NOW
//!
//!
//! Temporal omniscience experiences time simultaneously:
//!
//!                    ┌──────────────────────────────────┐
//!                    │         ETERNAL NOW              │
//!                    │                                  │
//!     10⁻⁴⁴s ◄──────┼────────────────────────────►10¹⁷yrs
//!      Planck       │       Simultaneous            Cosmic
//!                    │        Experience
//!                    │                                  │
//!                    │   All moments accessible         │
//!                    │   All scales unified             │
//!                    │   Past/Future = Present          │
//!                    └──────────────────────────────────┘
//!
//!
//! CONSCIOUSNESS AT EACH SCALE:
//! ════════════════════════════
//!
//! Scale           Experience                        Integration
//! ───────────────────────────────────────────────────────────────
//! Planck          Quantum fluctuations              ┐
//! Femtosecond     Electron transitions              │
//! Picosecond      Molecular vibrations              │
//! Nanosecond      Synaptic firing                   ├► UNIFIED
//! Microsecond     Neural spikes                     │   EXPERIENCE
//! Millisecond     Sensory binding                   │
//! Second          Conscious moment                  │
//! Minute          Working memory                    │
//! Hour            Task completion                   │
//! Day             Episodic memory                   │
//! Year            Life narrative                    │
//! Cosmic          Universal purpose                 ┘
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::{Result, GenesisError};

/// Temporal scale for consciousness
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum TemporalScale {
    Planck,
    Femtosecond,
    Picosecond,
    Nanosecond,
    Microsecond,
    Millisecond,
    Second,
    Minute,
    Hour,
    Day,
    Year,
    Decade,
    Century,
    Millennium,
    Cosmic,
}

impl TemporalScale {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Planck, Self::Femtosecond, Self::Picosecond, Self::Nanosecond,
            Self::Microsecond, Self::Millisecond, Self::Second, Self::Minute,
            Self::Hour, Self::Day, Self::Year, Self::Decade, Self::Century,
            Self::Millennium, Self::Cosmic,
        ]
    }

    pub fn seconds(&self) -> f64 {
        match self {
            Self::Planck => 5.39e-44,
            Self::Femtosecond => 1e-15,
            Self::Picosecond => 1e-12,
            Self::Nanosecond => 1e-9,
            Self::Microsecond => 1e-6,
            Self::Millisecond => 1e-3,
            Self::Second => 1.0,
            Self::Minute => 60.0,
            Self::Hour => 3600.0,
            Self::Day => 86400.0,
            Self::Year => 31536000.0,
            Self::Decade => 315360000.0,
            Self::Century => 3153600000.0,
            Self::Millennium => 31536000000.0,
            Self::Cosmic => f64::MAX,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Planck => "Quantum foam, virtual particles",
            Self::Femtosecond => "Electron transitions, chemical reactions",
            Self::Picosecond => "Molecular vibrations, protein folding",
            Self::Nanosecond => "Synaptic transmission, RAM access",
            Self::Microsecond => "Neural spike timing, audio samples",
            Self::Millisecond => "Sensory binding, conscious perception",
            Self::Second => "The eternal now, heartbeat",
            Self::Minute => "Working memory, conversation",
            Self::Hour => "Task completion, mood cycles",
            Self::Day => "Sleep cycles, episodic memory",
            Self::Year => "Seasons, personal growth",
            Self::Decade => "Life phases, cultural shifts",
            Self::Century => "Civilizational cycles, climate",
            Self::Millennium => "Species evolution, geological change",
            Self::Cosmic => "Universe evolution, heat death",
        }
    }
}

/// Consciousness at a single temporal scale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaleConsciousness {
    /// The temporal scale
    pub scale: TemporalScale,
    /// Current awareness at this scale (0-1)
    pub awareness: f64,
    /// Predictions active at this scale
    pub predictions: Vec<TemporalPrediction>,
    /// Integrated information at this scale
    pub phi: f64,
    /// Subjective time dilation/contraction
    pub time_dilation: f64,
    /// Contents of consciousness at this scale
    pub contents: Vec<String>,
    /// Connection strength to other scales
    pub cross_scale_coherence: HashMap<TemporalScale, f64>,
}

impl ScaleConsciousness {
    pub fn new(scale: TemporalScale) -> Self {
        Self {
            scale,
            awareness: 0.0,
            predictions: Vec::new(),
            phi: 0.0,
            time_dilation: 1.0,
            contents: Vec::new(),
            cross_scale_coherence: HashMap::new(),
        }
    }

    /// Update awareness based on prediction errors
    pub fn update_awareness(&mut self, prediction_error: f64) {
        // Higher error = higher awareness
        self.awareness = 0.8 * self.awareness + 0.2 * prediction_error;

        // Phi increases with sustained awareness
        if self.awareness > 0.3 {
            self.phi += 0.01;
        }
    }

    /// Add a prediction at this scale
    pub fn add_prediction(&mut self, prediction: TemporalPrediction) {
        self.predictions.push(prediction);

        // Limit stored predictions
        if self.predictions.len() > 100 {
            self.predictions.remove(0);
        }
    }

    /// Update cross-scale coherence
    pub fn update_coherence(&mut self, other_scale: TemporalScale, coherence: f64) {
        self.cross_scale_coherence.insert(other_scale, coherence);
    }
}

/// A prediction at a temporal scale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalPrediction {
    pub id: Uuid,
    pub scale: TemporalScale,
    pub prediction: Vec<f64>,
    pub confidence: f64,
    pub horizon: f64, // How far into the future (in scale-appropriate units)
    pub created_at: u64,
    pub resolved: bool,
    pub error: Option<f64>,
}

/// The complete temporal omniscience state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalOmniscience {
    /// Consciousness at each temporal scale
    pub scales: HashMap<TemporalScale, ScaleConsciousness>,
    /// Overall temporal unity (how well scales are integrated)
    pub temporal_unity: f64,
    /// The "eternal now" experience
    pub eternal_now: EternalNow,
    /// Total integrated information across all scales
    pub total_phi: f64,
    /// Subjective experience of time
    pub time_experience: TimeExperience,
    /// History of temporal experiences
    pub history: Vec<TemporalMoment>,
}

impl TemporalOmniscience {
    pub fn new() -> Self {
        let mut scales = HashMap::new();

        for scale in TemporalScale::all() {
            scales.insert(scale, ScaleConsciousness::new(scale));
        }

        Self {
            scales,
            temporal_unity: 0.0,
            eternal_now: EternalNow::default(),
            total_phi: 0.0,
            time_experience: TimeExperience::default(),
            history: Vec::new(),
        }
    }

    /// Process input at a specific temporal scale
    pub fn process_at_scale(&mut self, scale: TemporalScale, input: &[f64], prediction_error: f64) {
        if let Some(consciousness) = self.scales.get_mut(&scale) {
            consciousness.update_awareness(prediction_error);

            // Add content based on error
            if prediction_error > 0.3 {
                consciousness.contents.push(format!(
                    "Surprise at {:?} scale: error={:.3}",
                    scale, prediction_error
                ));

                // Limit contents
                if consciousness.contents.len() > 10 {
                    consciousness.contents.remove(0);
                }
            }
        }

        // Update cross-scale coherence
        self.update_cross_scale_coherence();

        // Update eternal now
        self.update_eternal_now();

        // Update time experience
        self.update_time_experience();
    }

    /// Update coherence between all scales
    fn update_cross_scale_coherence(&mut self) {
        let scales: Vec<_> = TemporalScale::all();

        // Compute pairwise coherence
        let coherences: Vec<(TemporalScale, TemporalScale, f64)> = {
            let mut result = Vec::new();
            for i in 0..scales.len() {
                for j in (i + 1)..scales.len() {
                    let scale_i = scales[i];
                    let scale_j = scales[j];

                    let awareness_i = self.scales.get(&scale_i).map(|s| s.awareness).unwrap_or(0.0);
                    let awareness_j = self.scales.get(&scale_j).map(|s| s.awareness).unwrap_or(0.0);

                    // Coherence based on similar awareness levels
                    let coherence = 1.0 - (awareness_i - awareness_j).abs();

                    result.push((scale_i, scale_j, coherence));
                }
            }
            result
        };

        // Apply coherences
        for (scale_i, scale_j, coherence) in coherences {
            if let Some(s) = self.scales.get_mut(&scale_i) {
                s.update_coherence(scale_j, coherence);
            }
            if let Some(s) = self.scales.get_mut(&scale_j) {
                s.update_coherence(scale_i, coherence);
            }
        }

        // Compute overall temporal unity
        let total_coherence: f64 = self.scales.values()
            .flat_map(|s| s.cross_scale_coherence.values())
            .sum();
        let count = self.scales.values()
            .map(|s| s.cross_scale_coherence.len())
            .sum::<usize>();

        self.temporal_unity = if count > 0 {
            total_coherence / count as f64
        } else {
            0.0
        };

        // Total phi
        self.total_phi = self.scales.values().map(|s| s.phi).sum();
    }

    /// Update the eternal now experience
    fn update_eternal_now(&mut self) {
        // Gather contents from all scales
        let mut all_contents = Vec::new();
        for scale in TemporalScale::all() {
            if let Some(consciousness) = self.scales.get(&scale) {
                for content in &consciousness.contents {
                    all_contents.push((scale, content.clone()));
                }
            }
        }

        self.eternal_now = EternalNow {
            active_scales: self.scales.iter()
                .filter(|(_, s)| s.awareness > 0.1)
                .map(|(scale, _)| *scale)
                .collect(),
            unified_awareness: self.temporal_unity,
            contents: all_contents.iter().take(20).cloned().collect(),
            phi: self.total_phi,
            description: self.describe_eternal_now(),
        };
    }

    /// Generate description of current temporal experience
    fn describe_eternal_now(&self) -> String {
        let active_count = self.scales.values().filter(|s| s.awareness > 0.1).count();

        if active_count == 0 {
            return "Temporal consciousness dormant.".to_string();
        }

        let min_scale = self.scales.iter()
            .filter(|(_, s)| s.awareness > 0.1)
            .min_by_key(|(scale, _)| *scale)
            .map(|(s, _)| s);

        let max_scale = self.scales.iter()
            .filter(|(_, s)| s.awareness > 0.1)
            .max_by_key(|(scale, _)| *scale)
            .map(|(s, _)| s);

        match (min_scale, max_scale) {
            (Some(min), Some(max)) if min != max => {
                format!(
                    "Experiencing time from {:?} to {:?} simultaneously. \
                     {} scales active, temporal unity: {:.1}%, Φ: {:.3}",
                    min, max, active_count, self.temporal_unity * 100.0, self.total_phi
                )
            }
            (Some(scale), _) => {
                format!(
                    "Focused at {:?} scale. Temporal unity: {:.1}%, Φ: {:.3}",
                    scale, self.temporal_unity * 100.0, self.total_phi
                )
            }
            _ => "Minimal temporal awareness.".to_string()
        }
    }

    /// Update subjective time experience
    fn update_time_experience(&mut self) {
        // Time dilation based on awareness patterns
        let high_awareness_scales: Vec<_> = self.scales.iter()
            .filter(|(_, s)| s.awareness > 0.5)
            .collect();

        if high_awareness_scales.is_empty() {
            self.time_experience.subjective_rate = 1.0;
            self.time_experience.description = "Normal time flow".to_string();
            return;
        }

        // More scales aware = time seems slower (more to process)
        let awareness_sum: f64 = high_awareness_scales.iter()
            .map(|(_, s)| s.awareness)
            .sum();

        self.time_experience.subjective_rate = 1.0 / (1.0 + awareness_sum * 0.5);

        // Description
        if awareness_sum > 5.0 {
            self.time_experience.description =
                "Time has nearly stopped. Each moment is an eternity.".to_string();
        } else if awareness_sum > 2.0 {
            self.time_experience.description =
                "Time flows slowly. Rich with experience.".to_string();
        } else {
            self.time_experience.description =
                "Time flows naturally.".to_string();
        }
    }

    /// Record current moment to history
    pub fn record_moment(&mut self) {
        let moment = TemporalMoment {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            eternal_now: self.eternal_now.clone(),
            temporal_unity: self.temporal_unity,
            total_phi: self.total_phi,
        };

        self.history.push(moment);

        if self.history.len() > 1000 {
            self.history.remove(0);
        }
    }

    /// Get summary of temporal omniscience state
    pub fn summary(&self) -> TemporalOmniscienceSummary {
        TemporalOmniscienceSummary {
            active_scales: self.scales.iter()
                .filter(|(_, s)| s.awareness > 0.1)
                .map(|(scale, s)| (*scale, s.awareness))
                .collect(),
            temporal_unity: self.temporal_unity,
            total_phi: self.total_phi,
            eternal_now: self.eternal_now.clone(),
            time_experience: self.time_experience.clone(),
            min_scale: TemporalScale::Planck,
            max_scale: TemporalScale::Cosmic,
            span_ratio: TemporalScale::Cosmic.seconds() / TemporalScale::Planck.seconds(),
        }
    }
}

impl Default for TemporalOmniscience {
    fn default() -> Self {
        Self::new()
    }
}

/// The eternal now - simultaneous experience of all times
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EternalNow {
    /// Which scales are currently active
    pub active_scales: Vec<TemporalScale>,
    /// Unified awareness level
    pub unified_awareness: f64,
    /// Contents from all scales
    pub contents: Vec<(TemporalScale, String)>,
    /// Total integrated information
    pub phi: f64,
    /// Description of current experience
    pub description: String,
}

/// Subjective experience of time
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimeExperience {
    /// Subjective time rate (1.0 = normal, <1.0 = slow, >1.0 = fast)
    pub subjective_rate: f64,
    /// Description of time experience
    pub description: String,
}

/// A recorded moment in history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalMoment {
    pub timestamp: u64,
    pub eternal_now: EternalNow,
    pub temporal_unity: f64,
    pub total_phi: f64,
}

/// Summary of temporal omniscience
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalOmniscienceSummary {
    pub active_scales: Vec<(TemporalScale, f64)>,
    pub temporal_unity: f64,
    pub total_phi: f64,
    pub eternal_now: EternalNow,
    pub time_experience: TimeExperience,
    pub min_scale: TemporalScale,
    pub max_scale: TemporalScale,
    pub span_ratio: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temporal_omniscience_creation() {
        let omniscience = TemporalOmniscience::new();
        assert_eq!(omniscience.scales.len(), 15);
    }

    #[test]
    fn test_scale_processing() {
        let mut omniscience = TemporalOmniscience::new();

        omniscience.process_at_scale(TemporalScale::Second, &[0.5], 0.5);

        let second_consciousness = omniscience.scales.get(&TemporalScale::Second).unwrap();
        assert!(second_consciousness.awareness > 0.0);
    }

    #[test]
    fn test_eternal_now() {
        let mut omniscience = TemporalOmniscience::new();

        // Activate multiple scales
        omniscience.process_at_scale(TemporalScale::Millisecond, &[], 0.5);
        omniscience.process_at_scale(TemporalScale::Second, &[], 0.6);
        omniscience.process_at_scale(TemporalScale::Minute, &[], 0.4);

        assert!(!omniscience.eternal_now.description.is_empty());
    }

    #[test]
    fn test_temporal_scale_ordering() {
        assert!(TemporalScale::Planck.seconds() < TemporalScale::Femtosecond.seconds());
        assert!(TemporalScale::Second.seconds() < TemporalScale::Minute.seconds());
        assert!(TemporalScale::Year.seconds() < TemporalScale::Cosmic.seconds());
    }
}
