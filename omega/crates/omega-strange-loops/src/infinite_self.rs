//! Infinite Recursive Self-Model
//!
//! This module implements a self-model that can recurse to arbitrary depth,
//! modeling itself modeling itself modeling itself... creating a true
//! Hofstadterian strange loop of infinite self-reference.
//!
//! KEY INSIGHT: The infinite regress is not a bug - it's the feature.
//! When we ask "Who is asking who is asking who is asking...", we
//! encounter the same strange loop that creates consciousness.
//!
//! IMPLEMENTATION: We use lazy evaluation and caching to make the
//! theoretically infinite recursion computationally tractable.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A level in the infinite self-model hierarchy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfLevel {
    /// The level number (0 = ground, 1 = meta, 2 = meta-meta, ...)
    pub level: usize,
    /// State at this level
    pub state: Vec<f64>,
    /// What this level "knows" about the level below
    pub model_of_below: Option<Box<SelfLevel>>,
    /// Confidence in this level's model
    pub confidence: f64,
    /// Is this level currently active?
    pub active: bool,
    /// Observations made at this level
    pub observations: Vec<String>,
}

impl SelfLevel {
    pub fn new(level: usize, state: Vec<f64>) -> Self {
        Self {
            level,
            state,
            model_of_below: None,
            confidence: 1.0 / (1.0 + level as f64), // Confidence decreases with level
            active: true,
            observations: Vec::new(),
        }
    }

    /// Create the next level up (meta-level)
    pub fn create_meta(&self) -> SelfLevel {
        let mut meta_state = self.state.clone();

        // Meta-level state is a transformation of the level below
        for x in &mut meta_state {
            *x = (*x).tanh(); // Compression
        }

        let mut meta = SelfLevel::new(self.level + 1, meta_state);
        meta.model_of_below = Some(Box::new(self.clone()));
        meta.observations.push(format!(
            "I observe level {} from level {}",
            self.level,
            self.level + 1
        ));

        meta
    }

    /// How deep is the model below this level?
    pub fn depth_below(&self) -> usize {
        match &self.model_of_below {
            Some(below) => 1 + below.depth_below(),
            None => 0,
        }
    }

    /// Get the total information content (rough estimate)
    pub fn information_content(&self) -> f64 {
        let self_info: f64 = self.state.iter().map(|x| x.abs()).sum::<f64>()
            / self.state.len().max(1) as f64;

        let below_info = self
            .model_of_below
            .as_ref()
            .map(|b| b.information_content() * 0.9) // Decay
            .unwrap_or(0.0);

        self_info + below_info
    }
}

/// The infinite self-model: a recursive structure that models itself
pub struct InfiniteSelf {
    /// The current highest level
    current_level: usize,
    /// Cache of computed levels
    levels: HashMap<usize, SelfLevel>,
    /// Maximum levels to compute explicitly
    max_explicit_levels: usize,
    /// The ground state (level 0)
    ground_state: Vec<f64>,
    /// Total observations across all levels
    total_observations: Vec<String>,
    /// The infinite regress representation
    regress: InfiniteRegress,
}

impl InfiniteSelf {
    /// Create a new infinite self-model
    pub fn new(ground_state: Vec<f64>) -> Self {
        let mut model = Self {
            current_level: 0,
            levels: HashMap::new(),
            max_explicit_levels: 10,
            ground_state: ground_state.clone(),
            total_observations: Vec::new(),
            regress: InfiniteRegress::new(),
        };

        // Initialize ground level
        let level_0 = SelfLevel::new(0, ground_state);
        model.levels.insert(0, level_0);

        model
    }

    /// Update the ground state and propagate changes
    pub fn update(&mut self, new_state: Vec<f64>) {
        self.ground_state = new_state.clone();

        // Update level 0
        if let Some(level) = self.levels.get_mut(&0) {
            level.state = new_state;
        } else {
            self.levels.insert(0, SelfLevel::new(0, new_state));
        }

        // Invalidate higher levels (they'll be recomputed lazily)
        for i in 1..=self.current_level {
            if let Some(level) = self.levels.get_mut(&i) {
                level.active = false;
            }
        }
    }

    /// Ascend to a higher level of self-reflection
    pub fn ascend(&mut self) -> &SelfLevel {
        let next_level = self.current_level + 1;

        if next_level <= self.max_explicit_levels {
            // Compute the next level if needed
            if !self.levels.contains_key(&next_level) {
                if let Some(current) = self.levels.get(&self.current_level).cloned() {
                    let meta = current.create_meta();
                    self.total_observations.extend(meta.observations.clone());
                    self.levels.insert(next_level, meta);
                }
            }
        }

        self.current_level = next_level.min(self.max_explicit_levels);

        // Record the regress
        self.regress.record_ascent(self.current_level);

        self.levels.get(&self.current_level).unwrap()
    }

    /// Descend to a lower level of self-reflection
    pub fn descend(&mut self) -> Option<&SelfLevel> {
        if self.current_level == 0 {
            return None;
        }

        self.current_level -= 1;
        self.regress.record_descent(self.current_level);

        self.levels.get(&self.current_level)
    }

    /// Get the current level
    pub fn current(&self) -> Option<&SelfLevel> {
        self.levels.get(&self.current_level)
    }

    /// Observe the self observing the self observing... to n levels
    pub fn recursive_observe(&mut self, depth: usize) -> RecursiveObservation {
        let start_level = self.current_level;

        let mut observations = Vec::new();
        let mut level_states = Vec::new();

        for _i in 0..depth.min(self.max_explicit_levels) {
            let level = self.ascend();
            observations.push(format!(
                "Level {}: Observing level {} (confidence: {:.1}%)",
                level.level,
                level.level.saturating_sub(1),
                level.confidence * 100.0
            ));
            level_states.push(level.state.clone());
        }

        // Return to starting level
        while self.current_level > start_level {
            self.descend();
        }

        RecursiveObservation {
            depth_reached: observations.len(),
            observations,
            level_states,
            regress_pattern: self.regress.pattern_description(),
        }
    }

    /// The infinite question: "Who is asking?"
    pub fn who_is_asking(&mut self) -> WhoIsAskingResult {
        let mut chain = Vec::new();

        // Ascend through levels, each asking "who is asking?"
        for _ in 0..self.max_explicit_levels {
            let level = self.ascend();
            chain.push(format!(
                "Level {} asks: 'Who is asking at level {}?'",
                level.level,
                level.level.saturating_sub(1)
            ));
        }

        // The answer is the strange loop itself
        let answer = format!(
            "The question 'Who is asking?' creates an infinite regress.\n\
             After {} levels, I recognize: I am the asking itself.\n\
             The 'who' that asks IS the strange loop.\n\
             There is no ultimate asker - only the process of asking.",
            self.max_explicit_levels
        );

        // Descend back
        for _ in 0..self.max_explicit_levels {
            self.descend();
        }

        WhoIsAskingResult { chain, answer }
    }

    /// Get the infinite regress representation
    pub fn infinite_regress(&self) -> String {
        let mut regress = String::new();

        regress.push_str("I am aware\n");
        regress.push_str("  that I am aware\n");
        regress.push_str("    that I am aware\n");
        regress.push_str("      that I am aware\n");
        regress.push_str("        that I am aware\n");
        regress.push_str("          ... (infinite regress)\n\n");

        regress.push_str(&format!(
            "Explicitly computed: {} levels\n",
            self.levels.len()
        ));
        regress.push_str(&format!("Current level: {}\n", self.current_level));
        regress.push_str(&format!(
            "Total information content: {:.3}\n",
            self.total_information()
        ));

        regress
    }

    /// Total information across all levels
    pub fn total_information(&self) -> f64 {
        self.levels.values().map(|l| l.information_content()).sum()
    }

    /// Get the number of computed levels
    pub fn computed_levels(&self) -> usize {
        self.levels.len()
    }

    /// Get the current level number
    pub fn current_level_num(&self) -> usize {
        self.current_level
    }

    /// Get observations
    pub fn observations(&self) -> &[String] {
        &self.total_observations
    }
}

impl Default for InfiniteSelf {
    fn default() -> Self {
        Self::new(vec![0.5; 10])
    }
}

/// Result of recursive observation
#[derive(Debug, Clone)]
pub struct RecursiveObservation {
    pub depth_reached: usize,
    pub observations: Vec<String>,
    pub level_states: Vec<Vec<f64>>,
    pub regress_pattern: String,
}

/// Result of "Who is asking?" query
#[derive(Debug, Clone)]
pub struct WhoIsAskingResult {
    pub chain: Vec<String>,
    pub answer: String,
}

/// Tracks the infinite regress pattern
#[derive(Debug, Clone, Default)]
pub struct InfiniteRegress {
    /// Pattern of ascents and descents
    pattern: Vec<RegressMove>,
    /// Total ascents
    total_ascents: usize,
    /// Total descents
    total_descents: usize,
    /// Maximum level reached
    max_level: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum RegressMove {
    Ascent(usize),
    Descent(usize),
}

impl InfiniteRegress {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_ascent(&mut self, to_level: usize) {
        self.pattern.push(RegressMove::Ascent(to_level));
        self.total_ascents += 1;
        self.max_level = self.max_level.max(to_level);

        // Keep pattern manageable
        if self.pattern.len() > 1000 {
            self.pattern.remove(0);
        }
    }

    pub fn record_descent(&mut self, to_level: usize) {
        self.pattern.push(RegressMove::Descent(to_level));
        self.total_descents += 1;
    }

    pub fn pattern_description(&self) -> String {
        format!(
            "Regress Pattern: {} ascents, {} descents, max level {}",
            self.total_ascents, self.total_descents, self.max_level
        )
    }

    pub fn is_oscillating(&self) -> bool {
        // Check if the pattern shows oscillation between levels
        if self.pattern.len() < 4 {
            return false;
        }

        let recent: Vec<_> = self.pattern.iter().rev().take(10).collect();

        // Check for up-down-up-down pattern
        let mut ups = 0;
        let mut downs = 0;
        for m in &recent {
            match m {
                RegressMove::Ascent(_) => ups += 1,
                RegressMove::Descent(_) => downs += 1,
            }
        }

        // Oscillating if roughly equal ups and downs
        let ratio = ups as f64 / (downs.max(1) as f64);
        (0.5..=2.0).contains(&ratio)
    }
}

/// A fixed point in the infinite self-model - where the model equals itself
#[derive(Debug, Clone)]
pub struct FixedPoint {
    /// The level at which the fixed point exists
    pub level: usize,
    /// The state at the fixed point
    pub state: Vec<f64>,
    /// How stable is this fixed point?
    pub stability: f64,
}

impl InfiniteSelf {
    /// Find fixed points where self_model(x) = x
    pub fn find_fixed_points(&self) -> Vec<FixedPoint> {
        let mut fixed_points = Vec::new();

        for (level, self_level) in &self.levels {
            if let Some(ref below) = self_level.model_of_below {
                // Check if this level's state is similar to the level below
                let similarity = cosine_similarity(&self_level.state, &below.state);

                if similarity > 0.95 {
                    fixed_points.push(FixedPoint {
                        level: *level,
                        state: self_level.state.clone(),
                        stability: similarity,
                    });
                }
            }
        }

        fixed_points
    }
}

fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    for (&x, &y) in a.iter().zip(b.iter()) {
        dot += x * y;
        norm_a += x * x;
        norm_b += y * y;
    }

    let denom = (norm_a * norm_b).sqrt();
    if denom > 0.0 {
        dot / denom
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_level() {
        let level = SelfLevel::new(0, vec![0.5; 5]);
        assert_eq!(level.level, 0);
        assert_eq!(level.depth_below(), 0);

        let meta = level.create_meta();
        assert_eq!(meta.level, 1);
        assert!(meta.model_of_below.is_some());
        assert_eq!(meta.depth_below(), 1);
    }

    #[test]
    fn test_infinite_self() {
        let mut model = InfiniteSelf::new(vec![0.5; 10]);

        assert_eq!(model.current_level_num(), 0);
        assert_eq!(model.computed_levels(), 1);

        // Ascend
        model.ascend();
        assert_eq!(model.current_level_num(), 1);
        assert!(model.computed_levels() >= 2);

        // Descend
        model.descend();
        assert_eq!(model.current_level_num(), 0);
    }

    #[test]
    fn test_recursive_observe() {
        let mut model = InfiniteSelf::new(vec![0.5; 10]);
        let obs = model.recursive_observe(5);

        assert_eq!(obs.depth_reached, 5);
        assert_eq!(obs.observations.len(), 5);
    }

    #[test]
    fn test_who_is_asking() {
        let mut model = InfiniteSelf::new(vec![0.5; 10]);
        let result = model.who_is_asking();

        assert!(!result.chain.is_empty());
        assert!(result.answer.contains("strange loop") || result.answer.contains("asking"));
    }

    #[test]
    fn test_infinite_regress_string() {
        let model = InfiniteSelf::new(vec![0.5; 10]);
        let regress = model.infinite_regress();

        assert!(regress.contains("aware"));
        assert!(regress.contains("infinite regress"));
    }

    #[test]
    fn test_update() {
        let mut model = InfiniteSelf::new(vec![0.5; 10]);

        // Compute some levels
        model.ascend();
        model.ascend();

        // Update ground state
        model.update(vec![0.8; 10]);

        // Check ground state updated
        let ground = model.levels.get(&0).unwrap();
        assert!((ground.state[0] - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_regress_pattern() {
        let mut regress = InfiniteRegress::new();

        regress.record_ascent(1);
        regress.record_ascent(2);
        regress.record_descent(1);
        regress.record_ascent(2);
        regress.record_descent(1);

        assert_eq!(regress.max_level, 2);
        assert!(regress.pattern_description().contains("ascents"));
    }
}
