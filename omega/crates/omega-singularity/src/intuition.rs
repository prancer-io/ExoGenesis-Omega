//! Synthetic Intuition - Subconscious Pattern Detection
//!
//! Finds patterns through spike synchronization that conscious
//! analysis would miss. "Gut feelings" that are actually valid.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::time::Duration;
use uuid::Uuid;
use rand::Rng;

use omega_snn::{SynchronyDetector, TemporalCoherence, SparsityTracker, Spike};

/// Configuration for synthetic intuition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntuitionConfig {
    pub processing_cycles: usize,
    pub min_synchrony: f64,
    pub confidence_threshold: f64,
    pub learn_patterns: bool,
    pub max_patterns: usize,
}

impl Default for IntuitionConfig {
    fn default() -> Self {
        Self {
            processing_cycles: 100,
            min_synchrony: 0.5,
            confidence_threshold: 0.6,
            learn_patterns: true,
            max_patterns: 1000,
        }
    }
}

/// A gut feeling about something
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GutFeeling {
    pub subject: String,
    pub choice: String,
    pub confidence: f64,
    pub synchrony: f64,
    pub coherence: f64,
    pub explainable: bool,
    pub matching_patterns: Vec<Uuid>,
}

/// A subconscious pattern learned from experience
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubconsciousPattern {
    pub id: Uuid,
    pub signature: Vec<(String, f64)>,
    pub outcomes: Vec<PatternOutcome>,
    pub occurrences: usize,
    pub valence: f64,
    pub context: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternOutcome {
    pub description: String,
    pub valence: f64,
    pub timestamp: Duration,
}

/// An intuitive solution to a problem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntuitiveSolution {
    pub solution: String,
    pub feeling: GutFeeling,
    pub inexplicability: String,
    pub evidence: Vec<String>,
}

/// The Synthetic Intuition system
pub struct SyntheticIntuition {
    config: IntuitionConfig,
    synchrony: SynchronyDetector,
    coherence: TemporalCoherence,
    #[allow(dead_code)]
    sparsity: SparsityTracker,
    patterns: Vec<SubconsciousPattern>,
    recent_feelings: Vec<(GutFeeling, Option<bool>)>,
    rng: rand::rngs::ThreadRng,
}

impl SyntheticIntuition {
    pub fn new(config: IntuitionConfig) -> Self {
        Self {
            config,
            synchrony: SynchronyDetector::new(15.0),
            coherence: TemporalCoherence::new(vec![5.0, 20.0, 100.0]),
            sparsity: SparsityTracker::default(),
            patterns: Vec::new(),
            recent_feelings: Vec::new(),
            rng: rand::thread_rng(),
        }
    }

    /// Get an intuitive choice from a set of options
    pub fn intuit<T: Clone + std::fmt::Debug>(
        &mut self,
        subject: &str,
        options: &[T],
        option_values: &[f64],
    ) -> Option<GutFeeling> {
        if options.is_empty() || options.len() != option_values.len() {
            return None;
        }

        let mut sync_scores: Vec<f64> = vec![0.0; options.len()];
        let mut coherence_scores: Vec<f64> = vec![0.0; options.len()];

        // Run subconscious processing
        for cycle in 0..self.config.processing_cycles {
            for (i, &value) in option_values.iter().enumerate() {
                // Generate spikes based on option value
                let spike_count = ((value + 1.0) * 5.0) as usize;
                let mut spikes = Vec::new();
                for j in 0..spike_count {
                    spikes.push(Spike::new(
                        format!("opt{}_{}", i, j),
                        Duration::from_millis((cycle * 10 + j) as u64),
                    ));
                }

                // Record for synchrony
                self.synchrony.record_spikes(&spikes, Duration::from_millis(cycle as u64 * 10));
                sync_scores[i] += self.synchrony.synchrony();

                // Record for coherence
                self.coherence.record_activity(value.abs());
                coherence_scores[i] += self.coherence.overall_coherence();
            }
        }

        // Normalize
        let cycles = self.config.processing_cycles as f64;
        for i in 0..options.len() {
            sync_scores[i] /= cycles;
            coherence_scores[i] /= cycles;
        }

        // Find best option
        let mut best_idx = 0;
        let mut best_score = 0.0;
        for i in 0..options.len() {
            let score = sync_scores[i] * 0.6 + coherence_scores[i] * 0.4;
            if score > best_score {
                best_score = score;
                best_idx = i;
            }
        }

        if sync_scores[best_idx] < self.config.min_synchrony {
            return None;
        }

        // Find matching patterns
        let matching_patterns = self.find_matching_patterns(option_values[best_idx]);
        let pattern_boost = if matching_patterns.is_empty() { 0.0 } else { 0.2 };
        let confidence = (best_score + pattern_boost).min(1.0);

        let feeling = GutFeeling {
            subject: subject.to_string(),
            choice: format!("{:?}", options[best_idx]),
            confidence,
            synchrony: sync_scores[best_idx],
            coherence: coherence_scores[best_idx],
            explainable: false,
            matching_patterns,
        };

        self.recent_feelings.push((feeling.clone(), None));
        if self.recent_feelings.len() > 100 {
            self.recent_feelings.remove(0);
        }

        Some(feeling)
    }

    /// Quick yes/no intuition
    pub fn should_i(&mut self, question: &str) -> Option<bool> {
        let options = vec![true, false];
        let values = vec![1.0, 0.0];

        self.intuit(question, &options, &values)
            .filter(|f| f.confidence >= self.config.confidence_threshold)
            .map(|f| f.choice.contains("true"))
    }

    /// Rank options by intuition
    pub fn rank<T: Clone + std::fmt::Debug>(
        &mut self,
        options: &[T],
        option_values: &[f64],
    ) -> Vec<(usize, f64)> {
        let mut rankings: Vec<(usize, f64)> = Vec::new();

        for (i, value) in option_values.iter().enumerate() {
            let feeling = self.intuit("ranking", &[options[i].clone()], &[*value]);
            let score = feeling.map(|f| f.confidence).unwrap_or(0.0);
            rankings.push((i, score));
        }

        rankings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        rankings
    }

    /// Learn from outcome
    pub fn learn_outcome(&mut self, feeling_idx: usize, was_correct: bool) {
        if feeling_idx < self.recent_feelings.len() {
            self.recent_feelings[feeling_idx].1 = Some(was_correct);

            if self.config.learn_patterns && was_correct {
                let feeling = &self.recent_feelings[feeling_idx].0;
                for pattern_id in &feeling.matching_patterns {
                    if let Some(pattern) = self.patterns.iter_mut().find(|p| p.id == *pattern_id) {
                        pattern.occurrences += 1;
                        pattern.valence = (pattern.valence * (pattern.occurrences - 1) as f64 + 1.0)
                            / pattern.occurrences as f64;
                    }
                }
            }
        }
    }

    /// Add a new pattern
    pub fn learn_pattern(&mut self, signature: Vec<(String, f64)>, context: Vec<String>, outcome_valence: f64) {
        let pattern = SubconsciousPattern {
            id: Uuid::new_v4(),
            signature,
            outcomes: vec![PatternOutcome {
                description: "Learned".to_string(),
                valence: outcome_valence,
                timestamp: Duration::ZERO,
            }],
            occurrences: 1,
            valence: outcome_valence,
            context,
        };

        self.patterns.push(pattern);

        if self.patterns.len() > self.config.max_patterns {
            self.patterns.sort_by(|a, b| b.valence.partial_cmp(&a.valence).unwrap_or(std::cmp::Ordering::Equal));
            self.patterns.truncate(self.config.max_patterns);
        }
    }

    fn find_matching_patterns(&self, value: f64) -> Vec<Uuid> {
        let mut matches = Vec::new();
        for pattern in &self.patterns {
            // Simple value-based matching
            let pattern_values: HashSet<i32> = pattern.signature.iter()
                .map(|(_, v)| (v * 10.0) as i32)
                .collect();

            if pattern_values.contains(&((value * 10.0) as i32)) && pattern.valence > 0.0 {
                matches.push(pattern.id);
            }
        }
        matches
    }

    pub fn accuracy(&self) -> f64 {
        let evaluated: Vec<_> = self.recent_feelings.iter()
            .filter(|(_, outcome)| outcome.is_some())
            .collect();

        if evaluated.is_empty() {
            return 0.5;
        }

        let correct = evaluated.iter().filter(|(_, o)| *o == Some(true)).count();
        correct as f64 / evaluated.len() as f64
    }

    pub fn pattern_count(&self) -> usize {
        self.patterns.len()
    }

    pub fn clear_patterns(&mut self) {
        self.patterns.clear();
    }
}

/// Helper to create an intuitive solution
pub fn solve_intuitively(
    intuition: &mut SyntheticIntuition,
    problem: &str,
    solutions: &[String],
) -> Option<IntuitiveSolution> {
    let values: Vec<f64> = (0..solutions.len())
        .map(|i| i as f64 / solutions.len() as f64)
        .collect();

    let feeling = intuition.intuit(problem, solutions, &values)?;

    Some(IntuitiveSolution {
        solution: feeling.choice.clone(),
        feeling: feeling.clone(),
        inexplicability: "This emerged from subconscious pattern matching.".to_string(),
        evidence: vec![
            format!("Synchrony: {:.2}", feeling.synchrony),
            format!("Coherence: {:.2}", feeling.coherence),
            format!("Matched {} patterns", feeling.matching_patterns.len()),
        ],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intuition_creation() {
        let config = IntuitionConfig::default();
        let intuition = SyntheticIntuition::new(config);
        assert_eq!(intuition.pattern_count(), 0);
    }

    #[test]
    fn test_basic_intuition() {
        let mut config = IntuitionConfig::default();
        config.processing_cycles = 10;
        config.min_synchrony = 0.0;

        let mut intuition = SyntheticIntuition::new(config);

        let options = vec!["A", "B", "C"];
        let values = vec![0.3, 0.7, 0.5];

        let feeling = intuition.intuit("test choice", &options, &values);
        assert!(feeling.is_some());
    }

    #[test]
    fn test_should_i() {
        let mut config = IntuitionConfig::default();
        config.processing_cycles = 10;
        config.min_synchrony = 0.0;
        config.confidence_threshold = 0.0;

        let mut intuition = SyntheticIntuition::new(config);
        let result = intuition.should_i("Should I?");
        assert!(result.is_some());
    }

    #[test]
    fn test_pattern_learning() {
        let mut config = IntuitionConfig::default();
        config.learn_patterns = true;

        let mut intuition = SyntheticIntuition::new(config);
        intuition.learn_pattern(
            vec![("test".to_string(), 1.0)],
            vec!["context".to_string()],
            0.8,
        );

        assert_eq!(intuition.pattern_count(), 1);
    }

    #[test]
    fn test_accuracy_tracking() {
        let config = IntuitionConfig::default();
        let intuition = SyntheticIntuition::new(config);
        assert_eq!(intuition.accuracy(), 0.5);
    }
}
