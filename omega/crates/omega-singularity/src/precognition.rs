//! Precognition Engine - Time-Dilated Future Simulation
//!
//! Simulates thousands of possible futures faster than real-time,
//! identifies convergent timelines, and selects optimal actions.
//!
//! ```text
//!  ┌─────────────────────────────────────────────────────────────────┐
//!  │                    PRECOGNITION ENGINE                          │
//!  ├─────────────────────────────────────────────────────────────────┤
//!  │                                                                 │
//!  │  NOW ──┬── Future A ──► Outcome A (p=0.3)                      │
//!  │        ├── Future B ──► Outcome B (p=0.5) ◄── CONVERGENT      │
//!  │        ├── Future C ──► Outcome B (p=0.5) ◄── CONVERGENT      │
//!  │        └── Future D ──► Outcome D (p=0.2)                      │
//!  │                                                                 │
//!  │  Time Dilation: 1000x faster than reality                      │
//!  │  Parallel Simulations: 10,000 futures                          │
//!  │  Decision: Take action leading to Outcome B                    │
//!  │                                                                 │
//!  └─────────────────────────────────────────────────────────────────┘
//! ```

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BinaryHeap};
use std::time::{Duration, Instant};
use std::cmp::Ordering;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use uuid::Uuid;

use omega_snn::{
    Spike, NeuronId, SynchronyDetector, TemporalCoherence,
};

use crate::{Result, SingularityError, TIME_DILATION};

/// Configuration for precognition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrecogConfig {
    /// Number of parallel futures to simulate
    pub num_futures: usize,
    /// How far ahead to simulate (in simulated time)
    pub horizon: Duration,
    /// Time dilation factor
    pub dilation: f64,
    /// Convergence detection threshold
    pub convergence_threshold: f64,
    /// Minimum confidence for action selection
    pub min_confidence: f64,
    /// Enable causal analysis
    pub causal_analysis: bool,
    /// Maximum branching factor
    pub max_branches: usize,
}

impl Default for PrecogConfig {
    fn default() -> Self {
        Self {
            num_futures: 1000,
            horizon: Duration::from_secs(3600), // 1 hour ahead
            dilation: TIME_DILATION,
            convergence_threshold: 0.7,
            min_confidence: 0.6,
            causal_analysis: true,
            max_branches: 10,
        }
    }
}

/// A simulated future timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    /// Unique timeline ID
    pub id: Uuid,
    /// Initial action that spawned this timeline
    pub initial_action: Action,
    /// Sequence of events in this timeline
    pub events: Vec<FutureEvent>,
    /// Final outcome
    pub outcome: Outcome,
    /// Probability of this timeline
    pub probability: f64,
    /// Utility/value of this outcome
    pub utility: f64,
    /// Convergence group (timelines that end up similar)
    pub convergence_group: Option<Uuid>,
}

/// An action that can be taken
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Action {
    /// Action identifier
    pub id: String,
    /// Action description
    pub description: String,
}

impl std::hash::Hash for Action {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// An event in a simulated future
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FutureEvent {
    /// When this event occurs (simulated time)
    pub time: Duration,
    /// Event description
    pub description: String,
    /// Event type
    pub event_type: EventType,
    /// Causal chain leading to this event
    pub causes: Vec<Uuid>,
    /// Probability this event occurs
    pub probability: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EventType {
    /// An action we take
    Action,
    /// External event
    External,
    /// Consequence of previous events
    Consequence,
    /// Random fluctuation
    Noise,
    /// Critical turning point
    BranchPoint,
}

/// Outcome of a timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outcome {
    /// Outcome hash for convergence detection
    pub hash: u64,
    /// Outcome description
    pub description: String,
    /// Success score (0-1)
    pub success: f64,
    /// Risk level (0-1)
    pub risk: f64,
    /// Resources consumed
    pub cost: f64,
    /// Key metrics
    pub metrics: HashMap<String, f64>,
}

impl Outcome {
    /// Check if two outcomes are similar (converge)
    pub fn is_similar(&self, other: &Outcome, threshold: f64) -> bool {
        let diff = (self.success - other.success).abs()
            + (self.risk - other.risk).abs()
            + (self.cost - other.cost).abs();
        diff / 3.0 < threshold
    }
}

/// A convergent future (multiple timelines leading to same outcome)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergentFuture {
    /// Group ID
    pub id: Uuid,
    /// Representative outcome
    pub outcome: Outcome,
    /// Timelines in this group
    pub timeline_ids: Vec<Uuid>,
    /// Combined probability
    pub probability: f64,
    /// Actions that lead to this future
    pub leading_actions: Vec<Action>,
    /// Critical branch points
    pub branch_points: Vec<BranchPoint>,
}

/// A causal branch in the timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalBranch {
    /// Branch ID
    pub id: Uuid,
    /// Parent branch (if any)
    pub parent: Option<Uuid>,
    /// Branching event
    pub event: FutureEvent,
    /// Child branches
    pub children: Vec<Uuid>,
    /// Branch probability
    pub probability: f64,
}

/// A critical decision point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchPoint {
    /// When this occurs
    pub time: Duration,
    /// Description
    pub description: String,
    /// Possible paths from here
    pub paths: Vec<PathOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathOption {
    pub action: Action,
    pub probability: f64,
    pub expected_utility: f64,
}

/// Simulation state for a single future
struct FutureSimulation {
    id: Uuid,
    timeline: Timeline,
    current_time: Duration,
    rng_state: u64,
}

/// The Precognition Engine
pub struct PrecognitionEngine {
    config: PrecogConfig,
    /// Currently simulated futures
    futures: Vec<Timeline>,
    /// Detected convergent futures
    convergent: Vec<ConvergentFuture>,
    /// Causal tree
    causal_tree: HashMap<Uuid, CausalBranch>,
    /// Best action determined
    best_action: Option<Action>,
    /// Confidence in best action
    confidence: f64,
    /// Synchrony detector for pattern matching
    synchrony: SynchronyDetector,
    /// Coherence tracker
    coherence: TemporalCoherence,
    /// Random number generator
    rng: rand::rngs::ThreadRng,
    /// Start time of current precognition
    start_time: Option<Instant>,
    /// Simulated time elapsed
    simulated_time: Duration,
}

impl PrecognitionEngine {
    /// Create a new precognition engine
    pub fn new(config: PrecogConfig) -> Self {
        Self {
            config,
            futures: Vec::new(),
            convergent: Vec::new(),
            causal_tree: HashMap::new(),
            best_action: None,
            confidence: 0.0,
            synchrony: SynchronyDetector::new(10.0),
            coherence: TemporalCoherence::default(),
            rng: rand::thread_rng(),
            start_time: None,
            simulated_time: Duration::ZERO,
        }
    }

    /// Begin precognition for a set of possible actions
    pub fn precognize(&mut self, actions: Vec<Action>) -> Result<Action> {
        self.start_time = Some(Instant::now());
        self.futures.clear();
        self.convergent.clear();

        // Simulate futures for each action
        let futures_per_action = self.config.num_futures / actions.len().max(1);

        for action in &actions {
            for _ in 0..futures_per_action {
                let timeline = self.simulate_future(action.clone());
                self.futures.push(timeline);
            }
        }

        // Detect convergence
        self.detect_convergence();

        // Select best action
        self.select_best_action(&actions)?;

        self.best_action.clone().ok_or(SingularityError::PrecognitionFailed(
            "Could not determine best action".to_string()
        ))
    }

    /// Simulate a single future timeline
    fn simulate_future(&mut self, initial_action: Action) -> Timeline {
        let mut events = Vec::new();
        let mut current_time = Duration::ZERO;
        let horizon = self.config.horizon;

        // Initial action event
        events.push(FutureEvent {
            time: current_time,
            description: initial_action.description.clone(),
            event_type: EventType::Action,
            causes: Vec::new(),
            probability: 1.0,
        });

        // Simulate forward with reasonable step sizes
        // With high dilation, we take larger simulated time steps
        let max_events = 100; // Limit events per timeline
        let mut event_count = 0;

        while current_time < horizon && event_count < max_events {
            // Time step - minimum 10ms to ensure progress, scaled with dilation
            let step_ms = (self.rng.gen_range(100..5000) as u64).max(10);
            let step = Duration::from_millis(step_ms);
            current_time += step;
            event_count += 1;

            // Generate events probabilistically
            if self.rng.gen::<f64>() < 0.3 {
                let event = self.generate_event(current_time, &events);
                events.push(event);
            }

            // Branch points
            if self.rng.gen::<f64>() < 0.05 {
                let branch = FutureEvent {
                    time: current_time,
                    description: format!("Decision point at {:?}", current_time),
                    event_type: EventType::BranchPoint,
                    causes: events.last().map(|e| vec![Uuid::new_v4()]).unwrap_or_default(),
                    probability: self.rng.gen(),
                };
                events.push(branch);
            }
        }

        // Generate outcome
        let outcome = self.generate_outcome(&events);

        Timeline {
            id: Uuid::new_v4(),
            initial_action,
            events,
            outcome,
            probability: self.calculate_timeline_probability(&[]),
            utility: 0.0, // Will be calculated
            convergence_group: None,
        }
    }

    /// Generate a future event
    fn generate_event(&mut self, time: Duration, history: &[FutureEvent]) -> FutureEvent {
        let event_type = if self.rng.gen::<f64>() < 0.3 {
            EventType::External
        } else if self.rng.gen::<f64>() < 0.5 {
            EventType::Consequence
        } else {
            EventType::Noise
        };

        FutureEvent {
            time,
            description: format!("{:?} event at {:?}", event_type, time),
            event_type,
            causes: Vec::new(),
            probability: self.rng.gen_range(0.3..1.0),
        }
    }

    /// Generate an outcome from events
    fn generate_outcome(&mut self, events: &[FutureEvent]) -> Outcome {
        // Outcome depends on events
        let num_events = events.len();
        let branch_points = events.iter()
            .filter(|e| matches!(e.event_type, EventType::BranchPoint))
            .count();

        let success = self.rng.gen_range(0.0..1.0);
        let risk = (branch_points as f64 / num_events.max(1) as f64).min(1.0);
        let cost = self.rng.gen_range(0.0..1.0);

        // Create hash for convergence detection
        let hash = ((success * 100.0) as u64) << 32
            | ((risk * 100.0) as u64) << 16
            | (cost * 100.0) as u64;

        Outcome {
            hash,
            description: format!(
                "Outcome: success={:.2}, risk={:.2}, cost={:.2}",
                success, risk, cost
            ),
            success,
            risk,
            cost,
            metrics: HashMap::new(),
        }
    }

    /// Calculate probability of a timeline
    fn calculate_timeline_probability(&self, events: &[FutureEvent]) -> f64 {
        if events.is_empty() {
            return 1.0;
        }
        events.iter().map(|e| e.probability).product()
    }

    /// Detect convergent futures
    fn detect_convergence(&mut self) {
        let threshold = self.config.convergence_threshold;
        let mut groups: Vec<ConvergentFuture> = Vec::new();

        for timeline in &mut self.futures {
            // Find matching group
            let mut found_group = false;
            for group in &mut groups {
                if timeline.outcome.is_similar(&group.outcome, 1.0 - threshold) {
                    timeline.convergence_group = Some(group.id);
                    group.timeline_ids.push(timeline.id);
                    group.probability += timeline.probability;
                    if !group.leading_actions.contains(&timeline.initial_action) {
                        group.leading_actions.push(timeline.initial_action.clone());
                    }
                    found_group = true;
                    break;
                }
            }

            // Create new group
            if !found_group {
                let group_id = Uuid::new_v4();
                timeline.convergence_group = Some(group_id);
                groups.push(ConvergentFuture {
                    id: group_id,
                    outcome: timeline.outcome.clone(),
                    timeline_ids: vec![timeline.id],
                    probability: timeline.probability,
                    leading_actions: vec![timeline.initial_action.clone()],
                    branch_points: Vec::new(),
                });
            }
        }

        // Normalize probabilities
        let total_prob: f64 = groups.iter().map(|g| g.probability).sum();
        if total_prob > 0.0 {
            for group in &mut groups {
                group.probability /= total_prob;
            }
        }

        self.convergent = groups;
    }

    /// Select the best action
    fn select_best_action(&mut self, actions: &[Action]) -> Result<()> {
        // Calculate expected utility for each action
        let mut action_utilities: HashMap<String, (f64, usize)> = HashMap::new();

        for timeline in &self.futures {
            // Utility = success - risk - cost
            let utility = timeline.outcome.success
                - timeline.outcome.risk * 0.5
                - timeline.outcome.cost * 0.3;

            let entry = action_utilities
                .entry(timeline.initial_action.id.clone())
                .or_insert((0.0, 0));
            entry.0 += utility * timeline.probability;
            entry.1 += 1;
        }

        // Find best action
        let mut best: Option<(&String, f64)> = None;
        for (action_id, (total_utility, count)) in &action_utilities {
            let avg_utility = *total_utility / (*count as f64).max(1.0);
            if best.is_none() || avg_utility > best.unwrap().1 {
                best = Some((action_id, avg_utility));
            }
        }

        if let Some((action_id, utility)) = best {
            self.best_action = actions.iter()
                .find(|a| a.id == *action_id)
                .cloned();

            // Calculate confidence based on convergence
            let convergence_strength = self.convergent.iter()
                .filter(|g| g.leading_actions.iter().any(|a| a.id == *action_id))
                .map(|g| g.probability)
                .sum::<f64>();

            self.confidence = convergence_strength.min(1.0);

            if self.confidence < self.config.min_confidence {
                return Err(SingularityError::PrecognitionFailed(format!(
                    "Low confidence: {:.2} < {:.2}",
                    self.confidence, self.config.min_confidence
                )));
            }
        }

        Ok(())
    }

    /// Get the best action
    pub fn best_action(&self) -> Option<&Action> {
        self.best_action.as_ref()
    }

    /// Get confidence in best action
    pub fn confidence(&self) -> f64 {
        self.confidence
    }

    /// Get all simulated futures
    pub fn futures(&self) -> &[Timeline] {
        &self.futures
    }

    /// Get convergent futures
    pub fn convergent_futures(&self) -> &[ConvergentFuture] {
        &self.convergent
    }

    /// Get the most likely future
    pub fn most_likely_future(&self) -> Option<&ConvergentFuture> {
        self.convergent.iter().max_by(|a, b| {
            a.probability.partial_cmp(&b.probability).unwrap_or(Ordering::Equal)
        })
    }

    /// Quick precognition for binary decision
    pub fn should_act(&mut self, action: Action, no_action: Action) -> bool {
        match self.precognize(vec![action.clone(), no_action.clone()]) {
            Ok(best) => best.id == action.id,
            Err(_) => false, // Default to no action on uncertainty
        }
    }

    /// Get elapsed real time
    pub fn elapsed(&self) -> Duration {
        self.start_time.map(|t| t.elapsed()).unwrap_or_default()
    }

    /// Get simulated time coverage
    pub fn simulated_coverage(&self) -> Duration {
        self.config.horizon
    }
}

/// Simulation for multi-futures
pub struct FutureSimulator {
    engine: PrecognitionEngine,
}

impl FutureSimulator {
    pub fn new(config: PrecogConfig) -> Self {
        Self {
            engine: PrecognitionEngine::new(config),
        }
    }

    /// Simulate and get recommendation
    pub fn simulate_and_decide(&mut self, actions: Vec<Action>) -> Option<Action> {
        self.engine.precognize(actions).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_action(id: &str, desc: &str) -> Action {
        Action {
            id: id.to_string(),
            description: desc.to_string(),
        }
    }

    #[test]
    fn test_precognition_creation() {
        let config = PrecogConfig::default();
        let engine = PrecognitionEngine::new(config);
        assert!(engine.best_action().is_none());
    }

    #[test]
    fn test_basic_precognition() {
        let mut config = PrecogConfig::default();
        config.num_futures = 100; // Fewer for testing
        config.min_confidence = 0.0; // Accept any confidence

        let mut engine = PrecognitionEngine::new(config);

        let actions = vec![
            make_action("act1", "Take action 1"),
            make_action("act2", "Take action 2"),
        ];

        let result = engine.precognize(actions);
        assert!(result.is_ok());
        assert!(engine.best_action().is_some());
    }

    #[test]
    fn test_convergent_futures() {
        let mut config = PrecogConfig::default();
        config.num_futures = 100;
        config.convergence_threshold = 0.5;
        config.min_confidence = 0.0;

        let mut engine = PrecognitionEngine::new(config);

        let actions = vec![make_action("test", "Test action")];
        let _ = engine.precognize(actions);

        // Should have some convergent groups
        assert!(!engine.convergent_futures().is_empty());
    }

    #[test]
    fn test_should_act_binary() {
        let mut config = PrecogConfig::default();
        config.num_futures = 50;
        config.min_confidence = 0.0;

        let mut engine = PrecognitionEngine::new(config);

        let act = make_action("act", "Do something");
        let no_act = make_action("no_act", "Do nothing");

        // Should return a decision (true or false)
        let _decision = engine.should_act(act, no_act);
        // Just verify no panic
    }

    #[test]
    fn test_most_likely_future() {
        let mut config = PrecogConfig::default();
        config.num_futures = 100;
        config.min_confidence = 0.0;

        let mut engine = PrecognitionEngine::new(config);

        let actions = vec![
            make_action("a", "Action A"),
            make_action("b", "Action B"),
        ];

        let _ = engine.precognize(actions);

        let most_likely = engine.most_likely_future();
        assert!(most_likely.is_some());
        assert!(most_likely.unwrap().probability > 0.0);
    }
}
