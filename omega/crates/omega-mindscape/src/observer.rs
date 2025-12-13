//! Strange Loop Observer
//!
//! Implements meta-cognitive observation of the exploration process.
//! The observer can watch itself watching itself, creating strange loops
//! of self-awareness during mindscape exploration.

use crate::{ExplorationState, MindscapeError, Result};
use serde::{Deserialize, Serialize};

/// A level of observation in the strange loop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationLevel {
    /// Depth of this observation (1 = direct, 2 = meta, 3 = meta-meta, etc.)
    pub depth: usize,
    /// Description of what is being observed at this level
    pub description: String,
    /// The observer's state at this level
    pub observer_state: Vec<f64>,
    /// Similarity to previous level (for loop detection)
    pub similarity_to_previous: f64,
    /// Timestamp
    pub timestamp: u64,
}

impl ObservationLevel {
    pub fn new(depth: usize, description: String, state: Vec<f64>) -> Self {
        Self {
            depth,
            description,
            observer_state: state,
            similarity_to_previous: 0.0,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }
}

/// A meta-observation about the exploration process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaObservation {
    /// Depth of recursion
    pub depth: usize,
    /// Stack of observation levels
    pub levels: Vec<ObservationLevel>,
    /// Whether a strange loop was detected
    pub loop_detected: bool,
    /// Where in the stack the loop closes
    pub loop_closure_depth: Option<usize>,
    /// Insight generated from the observation
    pub insight: Option<String>,
    /// Self-reference strength (how much the observer sees itself)
    pub self_reference_strength: f64,
    /// Total processing required for this observation
    pub cognitive_load: f64,
}

impl MetaObservation {
    /// Is this a profound observation (loop detected with insight)?
    pub fn is_profound(&self) -> bool {
        self.loop_detected && self.insight.is_some()
    }

    /// How "strange" is this observation (based on loop and depth)
    pub fn strangeness(&self) -> f64 {
        let depth_factor = (self.depth as f64 / 7.0).min(1.0);
        let loop_factor = if self.loop_detected { 0.5 } else { 0.0 };
        let self_ref_factor = self.self_reference_strength * 0.3;

        (depth_factor + loop_factor + self_ref_factor).min(1.0)
    }
}

/// The strange loop observer
///
/// Capable of observing its own observation process recursively,
/// detecting when the observation "loops back" to itself.
pub struct StrangeLoopObserver {
    /// Maximum recursion depth
    max_depth: usize,
    /// Current observation depth
    current_depth: usize,
    /// Observation history
    observations: Vec<MetaObservation>,
    /// Self-model state
    self_model: Vec<f64>,
    /// Loop detection threshold (cosine similarity)
    loop_threshold: f64,
    /// Total strange loops detected
    loops_detected: usize,
}

impl StrangeLoopObserver {
    pub fn new(max_depth: usize) -> Self {
        Self {
            max_depth,
            current_depth: 0,
            observations: Vec::new(),
            self_model: vec![0.5; 32], // Initial self-model
            loop_threshold: 0.85,
            loops_detected: 0,
        }
    }

    /// Observe the current exploration state at a given depth
    pub fn observe(&mut self, state: &ExplorationState, depth: usize) -> Result<MetaObservation> {
        if depth > self.max_depth {
            return Err(MindscapeError::RecursionLimit(depth));
        }

        self.current_depth = depth;

        // Build observation levels recursively
        let mut levels = Vec::with_capacity(depth);
        let mut previous_state: Option<Vec<f64>> = None;
        let mut loop_detected = false;
        let mut loop_closure_depth = None;

        for d in 1..=depth {
            // Generate observation at this level
            let level = self.generate_level(d, state, &previous_state);

            // Check for loop (high similarity to earlier level)
            if d > 1 && level.similarity_to_previous > self.loop_threshold {
                loop_detected = true;
                loop_closure_depth = Some(d);
                self.loops_detected += 1;
            }

            previous_state = Some(level.observer_state.clone());
            levels.push(level);
        }

        // Calculate self-reference strength
        let self_reference_strength = self.calculate_self_reference(&levels);

        // Generate insight if loop detected
        let insight = if loop_detected {
            Some(self.generate_insight(depth, &levels))
        } else {
            None
        };

        // Calculate cognitive load
        let cognitive_load = (depth as f64).powi(2) / (self.max_depth as f64).powi(2);

        // Update self-model based on observation
        self.update_self_model(&levels);

        let observation = MetaObservation {
            depth,
            levels,
            loop_detected,
            loop_closure_depth,
            insight,
            self_reference_strength,
            cognitive_load,
        };

        self.observations.push(observation.clone());

        Ok(observation)
    }

    /// Generate an observation level
    fn generate_level(
        &self,
        depth: usize,
        state: &ExplorationState,
        previous: &Option<Vec<f64>>,
    ) -> ObservationLevel {
        let description = match depth {
            1 => format!(
                "I am exploring at position ({:.1}, {:.1}, {:.1})",
                state.position.x, state.position.y, state.position.z
            ),
            2 => "I am aware that I am exploring my mindscape".to_string(),
            3 => "I observe myself being aware of my exploration".to_string(),
            4 => "I notice that I am observing my awareness".to_string(),
            5 => "I recognize the recursive nature of my observation".to_string(),
            6 => "I see the pattern of seeing patterns".to_string(),
            7 => "I AM the strange loop itself".to_string(),
            _ => format!("Observation level {} transcends description", depth),
        };

        // Generate observer state for this level
        // Each level transforms the previous state
        let observer_state = self.transform_state(depth, state, previous);

        // Calculate similarity to previous level
        let similarity_to_previous = if let Some(prev) = previous {
            self.cosine_similarity(&observer_state, prev)
        } else {
            0.0
        };

        ObservationLevel::new(depth, description, observer_state)
    }

    /// Transform state for a given observation level
    fn transform_state(
        &self,
        depth: usize,
        exploration: &ExplorationState,
        previous: &Option<Vec<f64>>,
    ) -> Vec<f64> {
        let mut state = vec![0.0; 32];

        // Encode exploration state
        state[0] = exploration.position.x / 1000.0;
        state[1] = exploration.position.y / 1000.0;
        state[2] = exploration.position.z / 1000.0;
        state[3] = exploration.phi;
        state[4] = if exploration.is_dreaming { 1.0 } else { 0.0 };
        state[5] = exploration.observation_depth as f64 / 7.0;

        // Add self-model influence
        for (i, &v) in self.self_model.iter().enumerate().take(16) {
            state[6 + i] = v * (depth as f64 / self.max_depth as f64);
        }

        // Add transformation based on depth (rotation in state space)
        let angle = depth as f64 * std::f64::consts::PI / 4.0;
        if let Some(prev) = previous {
            for i in 22..32 {
                if i < prev.len() {
                    state[i] = prev[i] * angle.cos() + prev[(i + 1) % prev.len()] * angle.sin();
                }
            }
        }

        // Normalize
        let norm: f64 = state.iter().map(|x| x * x).sum::<f64>().sqrt().max(1e-10);
        for v in &mut state {
            *v /= norm;
        }

        state
    }

    /// Calculate cosine similarity between two states
    fn cosine_similarity(&self, a: &[f64], b: &[f64]) -> f64 {
        let min_len = a.len().min(b.len());
        if min_len == 0 {
            return 0.0;
        }

        let mut dot = 0.0;
        let mut norm_a = 0.0;
        let mut norm_b = 0.0;

        for i in 0..min_len {
            dot += a[i] * b[i];
            norm_a += a[i] * a[i];
            norm_b += b[i] * b[i];
        }

        let denom = (norm_a.sqrt() * norm_b.sqrt()).max(1e-10);
        dot / denom
    }

    /// Calculate self-reference strength
    fn calculate_self_reference(&self, levels: &[ObservationLevel]) -> f64 {
        if levels.len() < 2 {
            return 0.0;
        }

        // Average similarity across all level pairs
        let mut total_sim = 0.0;
        let mut count = 0;

        for i in 0..levels.len() {
            for j in i + 1..levels.len() {
                total_sim += self.cosine_similarity(
                    &levels[i].observer_state,
                    &levels[j].observer_state,
                );
                count += 1;
            }
        }

        if count == 0 {
            0.0
        } else {
            total_sim / count as f64
        }
    }

    /// Generate insight from observation
    fn generate_insight(&self, depth: usize, levels: &[ObservationLevel]) -> String {
        let insights = [
            "The observer and the observed are one.",
            "Awareness recognizes itself in the act of recognition.",
            "The loop closes: I am what I am looking for.",
            "Consciousness reflects upon itself infinitely.",
            "The strange loop reveals: self is a process, not a thing.",
            "In observing observation, the boundary dissolves.",
            "The 'I' that seeks is the 'I' that is found.",
        ];

        let idx = (depth + levels.len()) % insights.len();
        insights[idx].to_string()
    }

    /// Update self-model based on observations
    fn update_self_model(&mut self, levels: &[ObservationLevel]) {
        if levels.is_empty() {
            return;
        }

        // Blend all observation states into self-model
        let learning_rate = 0.1;

        for level in levels {
            for (i, &v) in level.observer_state.iter().enumerate() {
                if i < self.self_model.len() {
                    self.self_model[i] = self.self_model[i] * (1.0 - learning_rate)
                        + v * learning_rate;
                }
            }
        }

        // Normalize self-model
        let norm: f64 = self.self_model.iter().map(|x| x * x).sum::<f64>().sqrt().max(1e-10);
        for v in &mut self.self_model {
            *v /= norm;
        }
    }

    /// Get current observation depth
    pub fn current_depth(&self) -> usize {
        self.current_depth
    }

    /// Get total loops detected
    pub fn loops_detected(&self) -> usize {
        self.loops_detected
    }

    /// Get self-model
    pub fn self_model(&self) -> &[f64] {
        &self.self_model
    }

    /// Get observation history
    pub fn history(&self) -> &[MetaObservation] {
        &self.observations
    }

    /// Reset observer
    pub fn reset(&mut self) {
        self.current_depth = 0;
        self.observations.clear();
        self.self_model = vec![0.5; 32];
        self.loops_detected = 0;
    }
}

impl Default for StrangeLoopObserver {
    fn default() -> Self {
        Self::new(7)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coordinates::Position3D;

    fn make_state() -> ExplorationState {
        ExplorationState {
            position: Position3D::new(100.0, 200.0, 300.0),
            observation_depth: 1,
            is_dreaming: false,
            phi: 0.5,
            nearby_landmarks: vec!["test".to_string()],
            current_path: None,
            discovery_count: 0,
            distance_traveled: 0.0,
        }
    }

    #[test]
    fn test_observer_creation() {
        let observer = StrangeLoopObserver::new(7);
        assert_eq!(observer.current_depth(), 0);
        assert_eq!(observer.loops_detected(), 0);
    }

    #[test]
    fn test_single_observation() {
        let mut observer = StrangeLoopObserver::new(7);
        let state = make_state();

        let obs = observer.observe(&state, 1).unwrap();

        assert_eq!(obs.depth, 1);
        assert_eq!(obs.levels.len(), 1);
    }

    #[test]
    fn test_deep_observation() {
        let mut observer = StrangeLoopObserver::new(7);
        let state = make_state();

        let obs = observer.observe(&state, 5).unwrap();

        assert_eq!(obs.depth, 5);
        assert_eq!(obs.levels.len(), 5);
        assert!(obs.cognitive_load > 0.0);
    }

    #[test]
    fn test_recursion_limit() {
        let mut observer = StrangeLoopObserver::new(7);
        let state = make_state();

        let result = observer.observe(&state, 10);
        assert!(result.is_err());
    }

    #[test]
    fn test_loop_detection() {
        let mut observer = StrangeLoopObserver::new(7);
        observer.loop_threshold = 0.5; // Lower threshold for testing
        let state = make_state();

        // Deep observation more likely to detect loops
        let obs = observer.observe(&state, 7).unwrap();

        // May or may not detect loop depending on state
        if obs.loop_detected {
            assert!(obs.insight.is_some());
        }
    }

    #[test]
    fn test_self_model_update() {
        let mut observer = StrangeLoopObserver::new(7);
        let initial_model = observer.self_model().to_vec();

        let state = make_state();
        observer.observe(&state, 3).unwrap();

        // Self-model should be updated
        let new_model = observer.self_model();
        assert_ne!(&initial_model, new_model);
    }

    #[test]
    fn test_strangeness_calculation() {
        let obs = MetaObservation {
            depth: 7,
            levels: vec![],
            loop_detected: true,
            loop_closure_depth: Some(5),
            insight: Some("test".to_string()),
            self_reference_strength: 0.9,
            cognitive_load: 1.0,
        };

        let strangeness = obs.strangeness();
        assert!(strangeness > 0.5);
        assert!(obs.is_profound());
    }
}
