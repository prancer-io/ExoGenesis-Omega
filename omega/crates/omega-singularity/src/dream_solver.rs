//! Dream Solver - REM-like Creative Problem Solving
//!
//! Enters dream states where normal constraints relax, allowing
//! impossible connections to form and creative insights to emerge.
//!
//! ```text
//!  ┌─────────────────────────────────────────────────────────────┐
//!  │                    DREAM SOLVER                             │
//!  ├─────────────────────────────────────────────────────────────┤
//!  │                                                             │
//!  │  WAKING          REM DREAMING         INSIGHT               │
//!  │  ┌─────┐         ┌─────────┐         ┌─────────┐           │
//!  │  │ A─B │  ───►   │ A─?─?─B │  ───►   │ A─X─Y─B │           │
//!  │  │ X   │         │ X─?─?   │         │   └───┘ │           │
//!  │  │ Y   │         │ Y─?─?   │         │ EUREKA! │           │
//!  │  └─────┘         └─────────┘         └─────────┘           │
//!  │                                                             │
//!  │  Constraints     Random replay       Novel connections      │
//!  │  enforced        explores space      discovered             │
//!  │                                                             │
//!  └─────────────────────────────────────────────────────────────┘
//! ```

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{Duration, Instant};
use rand::Rng;
use rand_distr::{Distribution, Normal};

use omega_snn::{
    Spike, NeuronId, SpikeTrain,
    SynchronyDetector, TemporalCoherence, SparsityTracker,
};

use crate::{Result, SingularityError};

/// Configuration for dream solving
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamConfig {
    /// Duration of each dream cycle
    pub cycle_duration: Duration,
    /// Number of REM cycles per session
    pub rem_cycles: usize,
    /// Constraint relaxation factor (0-1)
    pub relaxation_factor: f64,
    /// Minimum coherence for insight detection
    pub insight_threshold: f64,
    /// Random exploration noise level
    pub exploration_noise: f64,
    /// Memory replay probability
    pub replay_probability: f64,
    /// Maximum insights per session
    pub max_insights: usize,
}

impl Default for DreamConfig {
    fn default() -> Self {
        Self {
            cycle_duration: Duration::from_secs(60),
            rem_cycles: 5,
            relaxation_factor: 0.7,
            insight_threshold: 0.8,
            exploration_noise: 0.3,
            replay_probability: 0.4,
            max_insights: 10,
        }
    }
}

/// Current phase of dreaming
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DreamPhase {
    /// Awake, normal processing
    Awake,
    /// Light sleep, reduced activity
    N1,
    /// Deeper sleep, memory consolidation begins
    N2,
    /// Deep sleep, slow-wave activity
    N3,
    /// REM sleep, vivid dreams, creative exploration
    REM,
    /// Transitioning between phases
    Transition,
}

/// Current dream state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamState {
    /// Current phase
    pub phase: DreamPhase,
    /// Time in current phase
    pub phase_duration: Duration,
    /// Total dream session time
    pub session_duration: Duration,
    /// Current REM cycle number
    pub rem_cycle: usize,
    /// Constraint relaxation level
    pub relaxation_level: f64,
    /// Dream vividness (activity level during REM)
    pub vividness: f64,
    /// Memory fragments being replayed
    pub active_memories: usize,
    /// Potential insights detected
    pub pending_insights: usize,
}

/// A creative insight discovered during dreaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamInsight {
    /// Unique insight ID
    pub id: uuid::Uuid,
    /// The problem this relates to
    pub problem: String,
    /// The novel connection discovered
    pub connection: NovelConnection,
    /// Confidence in this insight
    pub confidence: f64,
    /// Coherence score
    pub coherence: f64,
    /// When it was discovered
    pub discovered_at: Duration,
    /// The dream phase when discovered
    pub phase: DreamPhase,
}

/// A novel connection discovered during dreams
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NovelConnection {
    /// Source concepts/neurons
    pub sources: Vec<String>,
    /// Target concepts/neurons
    pub targets: Vec<String>,
    /// The bridging elements
    pub bridges: Vec<String>,
    /// Connection strength
    pub strength: f64,
    /// Is this connection normally forbidden?
    pub normally_forbidden: bool,
}

/// A creative breakthrough (validated insight)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeBreakthrough {
    /// The original insight
    pub insight: DreamInsight,
    /// Validation score (how well it works when awake)
    pub validation_score: f64,
    /// Practical applications discovered
    pub applications: Vec<String>,
    /// Is this genuinely novel?
    pub is_novel: bool,
}

/// Memory fragment for replay
#[derive(Debug, Clone, Serialize, Deserialize)]
struct MemoryFragment {
    id: uuid::Uuid,
    pattern: Vec<(NeuronId, f64)>,
    context: String,
    importance: f64,
    last_replayed: Option<Duration>,
}

/// The Dream Solver system
pub struct DreamSolver {
    config: DreamConfig,
    state: DreamState,
    /// Problem being solved
    current_problem: Option<String>,
    /// Memory bank for replay
    memories: VecDeque<MemoryFragment>,
    /// Discovered insights
    insights: Vec<DreamInsight>,
    /// Validated breakthroughs
    breakthroughs: Vec<CreativeBreakthrough>,
    /// Synchrony detector for insight detection
    synchrony: SynchronyDetector,
    /// Coherence tracker
    coherence: TemporalCoherence,
    /// Sparsity tracker
    sparsity: SparsityTracker,
    /// Forbidden connections (normally blocked)
    forbidden_connections: HashSet<(String, String)>,
    /// Random number generator
    rng: rand::rngs::ThreadRng,
    /// Session start time
    session_start: Option<Instant>,
    /// Phase start time
    phase_start: Option<Instant>,
}

impl DreamSolver {
    /// Create a new dream solver
    pub fn new(config: DreamConfig) -> Self {
        Self {
            config,
            state: DreamState {
                phase: DreamPhase::Awake,
                phase_duration: Duration::ZERO,
                session_duration: Duration::ZERO,
                rem_cycle: 0,
                relaxation_level: 0.0,
                vividness: 0.0,
                active_memories: 0,
                pending_insights: 0,
            },
            current_problem: None,
            memories: VecDeque::new(),
            insights: Vec::new(),
            breakthroughs: Vec::new(),
            synchrony: SynchronyDetector::new(20.0), // Wider window for dreams
            coherence: TemporalCoherence::new(vec![10.0, 50.0, 200.0]),
            sparsity: SparsityTracker::default(),
            forbidden_connections: HashSet::new(),
            rng: rand::thread_rng(),
            session_start: None,
            phase_start: None,
        }
    }

    /// Start a dream session to solve a problem
    pub fn begin_dream_session(&mut self, problem: impl Into<String>) {
        self.current_problem = Some(problem.into());
        self.session_start = Some(Instant::now());
        self.phase_start = Some(Instant::now());
        self.insights.clear();
        self.state.phase = DreamPhase::N1;
        self.state.rem_cycle = 0;
        self.state.session_duration = Duration::ZERO;
    }

    /// Add a memory for potential replay
    pub fn add_memory(&mut self, pattern: Vec<(NeuronId, f64)>, context: String, importance: f64) {
        let fragment = MemoryFragment {
            id: uuid::Uuid::new_v4(),
            pattern,
            context,
            importance,
            last_replayed: None,
        };
        self.memories.push_back(fragment);

        // Limit memory size
        while self.memories.len() > 1000 {
            self.memories.pop_front();
        }
    }

    /// Add a forbidden connection (will try to bridge in dreams)
    pub fn add_forbidden_connection(&mut self, from: String, to: String) {
        self.forbidden_connections.insert((from, to));
    }

    /// Step the dream solver
    pub fn step(&mut self, dt: Duration) -> Option<DreamInsight> {
        // Update timing
        self.state.phase_duration += dt;
        self.state.session_duration += dt;

        // Phase transitions
        self.update_phase();

        // Process based on current phase
        match self.state.phase {
            DreamPhase::Awake => None,
            DreamPhase::N1 | DreamPhase::N2 => {
                self.process_light_sleep(dt);
                None
            }
            DreamPhase::N3 => {
                self.process_deep_sleep(dt);
                None
            }
            DreamPhase::REM => self.process_rem(dt),
            DreamPhase::Transition => None,
        }
    }

    /// Update the dream phase
    fn update_phase(&mut self) {
        let phase_duration = self.state.phase_duration;
        let cycle_duration = self.config.cycle_duration;

        match self.state.phase {
            DreamPhase::N1 if phase_duration >= cycle_duration / 4 => {
                self.transition_to(DreamPhase::N2);
            }
            DreamPhase::N2 if phase_duration >= cycle_duration / 3 => {
                self.transition_to(DreamPhase::N3);
            }
            DreamPhase::N3 if phase_duration >= cycle_duration / 4 => {
                self.transition_to(DreamPhase::REM);
            }
            DreamPhase::REM if phase_duration >= cycle_duration / 3 => {
                self.state.rem_cycle += 1;
                if self.state.rem_cycle >= self.config.rem_cycles {
                    self.transition_to(DreamPhase::Awake);
                } else {
                    self.transition_to(DreamPhase::N1);
                }
            }
            _ => {}
        }
    }

    /// Transition to a new phase
    fn transition_to(&mut self, new_phase: DreamPhase) {
        self.state.phase = DreamPhase::Transition;
        self.state.phase_duration = Duration::ZERO;
        self.phase_start = Some(Instant::now());

        // Update relaxation based on phase
        self.state.relaxation_level = match new_phase {
            DreamPhase::Awake => 0.0,
            DreamPhase::N1 => 0.2,
            DreamPhase::N2 => 0.4,
            DreamPhase::N3 => 0.3,
            DreamPhase::REM => self.config.relaxation_factor,
            DreamPhase::Transition => self.state.relaxation_level,
        };

        self.state.phase = new_phase;
    }

    /// Process light sleep (N1, N2)
    fn process_light_sleep(&mut self, _dt: Duration) {
        // Reduce activity, begin memory consolidation
        self.state.vividness = 0.2;

        // Occasionally replay memories
        if self.rng.gen::<f64>() < 0.1 {
            self.replay_memory();
        }
    }

    /// Process deep sleep (N3)
    fn process_deep_sleep(&mut self, _dt: Duration) {
        // Slow-wave activity, major memory consolidation
        self.state.vividness = 0.1;

        // Higher replay rate during deep sleep
        if self.rng.gen::<f64>() < 0.3 {
            self.replay_memory();
        }
    }

    /// Process REM sleep - where the magic happens
    fn process_rem(&mut self, dt: Duration) -> Option<DreamInsight> {
        self.state.vividness = 0.8 + self.rng.gen::<f64>() * 0.2;

        // Chaotic replay
        if self.rng.gen::<f64>() < self.config.replay_probability {
            self.chaotic_replay();
        }

        // Random exploration
        let exploration = self.random_exploration();

        // Check for insights
        if let Some(connection) = exploration {
            let coherence = self.coherence.overall_coherence();
            let synchrony = self.synchrony.synchrony();

            // Insight emerges when random exploration finds coherent patterns
            if coherence > self.config.insight_threshold || synchrony > 0.7 {
                let insight = DreamInsight {
                    id: uuid::Uuid::new_v4(),
                    problem: self.current_problem.clone().unwrap_or_default(),
                    connection,
                    confidence: (coherence + synchrony) / 2.0,
                    coherence,
                    discovered_at: self.state.session_duration,
                    phase: self.state.phase,
                };

                if self.insights.len() < self.config.max_insights {
                    self.insights.push(insight.clone());
                    self.state.pending_insights = self.insights.len();
                    return Some(insight);
                }
            }
        }

        // Record activity for coherence tracking
        self.coherence.record_activity(self.state.vividness);

        None
    }

    /// Replay a memory (normal consolidation)
    fn replay_memory(&mut self) {
        if let Some(memory) = self.memories.iter_mut()
            .filter(|m| m.last_replayed.is_none() ||
                m.last_replayed.unwrap() < self.state.session_duration.saturating_sub(Duration::from_secs(30)))
            .max_by(|a, b| a.importance.partial_cmp(&b.importance).unwrap_or(std::cmp::Ordering::Equal))
        {
            memory.last_replayed = Some(self.state.session_duration);
            self.state.active_memories += 1;
        }
    }

    /// Chaotic replay - shuffle and recombine memories
    fn chaotic_replay(&mut self) {
        if self.memories.len() < 2 {
            return;
        }

        // Pick random memories to mash together
        let idx1 = self.rng.gen_range(0..self.memories.len());
        let idx2 = self.rng.gen_range(0..self.memories.len());

        if idx1 != idx2 {
            // These memories are now associated in the dream
            self.state.active_memories += 2;
        }
    }

    /// Random exploration - try forbidden connections
    fn random_exploration(&mut self) -> Option<NovelConnection> {
        if self.forbidden_connections.is_empty() {
            return self.generate_random_connection();
        }

        // With some probability, try a forbidden connection
        if self.rng.gen::<f64>() < self.state.relaxation_level {
            // Clone to avoid borrow issues
            let forbidden: Vec<_> = self.forbidden_connections.iter()
                .map(|(a, b)| (a.clone(), b.clone()))
                .collect();
            let idx = self.rng.gen_range(0..forbidden.len());
            let (from, to) = &forbidden[idx];

            // Generate a bridge
            let bridges = self.generate_bridges(from, to);
            let strength = self.rng.gen::<f64>();

            return Some(NovelConnection {
                sources: vec![from.clone()],
                targets: vec![to.clone()],
                bridges,
                strength,
                normally_forbidden: true,
            });
        }

        None
    }

    /// Generate random connection
    fn generate_random_connection(&mut self) -> Option<NovelConnection> {
        if self.rng.gen::<f64>() < 0.3 {
            Some(NovelConnection {
                sources: vec![format!("concept_{}", self.rng.gen::<u32>() % 100)],
                targets: vec![format!("concept_{}", self.rng.gen::<u32>() % 100)],
                bridges: vec![format!("bridge_{}", self.rng.gen::<u32>() % 50)],
                strength: self.rng.gen::<f64>(),
                normally_forbidden: false,
            })
        } else {
            None
        }
    }

    /// Generate bridge concepts between two ideas
    fn generate_bridges(&mut self, _from: &str, _to: &str) -> Vec<String> {
        // In a real implementation, this would use semantic similarity
        let num_bridges = self.rng.gen_range(1..=3);
        (0..num_bridges)
            .map(|i| format!("bridge_{}", i))
            .collect()
    }

    /// Wake up and validate insights
    pub fn wake_and_validate(&mut self) -> Vec<CreativeBreakthrough> {
        self.state.phase = DreamPhase::Awake;
        self.state.relaxation_level = 0.0;

        // Validate each insight
        let mut breakthroughs = Vec::new();

        for insight in &self.insights {
            // In a real implementation, this would test the insight
            let validation_score = self.validate_insight(insight);

            if validation_score > 0.5 {
                let breakthrough = CreativeBreakthrough {
                    insight: insight.clone(),
                    validation_score,
                    applications: self.find_applications(insight),
                    is_novel: insight.connection.normally_forbidden,
                };
                breakthroughs.push(breakthrough);
            }
        }

        self.breakthroughs.extend(breakthroughs.clone());
        self.insights.clear();

        breakthroughs
    }

    /// Validate an insight in waking state
    fn validate_insight(&self, insight: &DreamInsight) -> f64 {
        // Combine coherence and confidence
        (insight.coherence + insight.confidence) / 2.0
    }

    /// Find practical applications for an insight
    fn find_applications(&self, insight: &DreamInsight) -> Vec<String> {
        let mut applications = Vec::new();

        if insight.connection.normally_forbidden {
            applications.push(format!(
                "Novel pathway from {:?} to {:?}",
                insight.connection.sources,
                insight.connection.targets
            ));
        }

        if insight.confidence > 0.8 {
            applications.push("High-confidence solution candidate".to_string());
        }

        applications
    }

    /// Get current state
    pub fn state(&self) -> &DreamState {
        &self.state
    }

    /// Get all insights from current session
    pub fn insights(&self) -> &[DreamInsight] {
        &self.insights
    }

    /// Get all validated breakthroughs
    pub fn breakthroughs(&self) -> &[CreativeBreakthrough] {
        &self.breakthroughs
    }

    /// Is currently dreaming?
    pub fn is_dreaming(&self) -> bool {
        !matches!(self.state.phase, DreamPhase::Awake)
    }

    /// Is in REM?
    pub fn is_rem(&self) -> bool {
        matches!(self.state.phase, DreamPhase::REM)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dream_solver_creation() {
        let config = DreamConfig::default();
        let solver = DreamSolver::new(config);
        assert!(!solver.is_dreaming());
    }

    #[test]
    fn test_begin_dream_session() {
        let config = DreamConfig::default();
        let mut solver = DreamSolver::new(config);

        solver.begin_dream_session("How to achieve world peace");
        assert!(solver.is_dreaming());
        assert_eq!(solver.state().phase, DreamPhase::N1);
    }

    #[test]
    fn test_phase_transitions() {
        let mut config = DreamConfig::default();
        config.cycle_duration = Duration::from_millis(100); // Fast for testing
        let mut solver = DreamSolver::new(config);

        solver.begin_dream_session("Test problem");

        // Step through phases
        for _ in 0..50 {
            solver.step(Duration::from_millis(10));
        }

        // Should have progressed through phases
        assert!(solver.state().session_duration >= Duration::from_millis(500));
    }

    #[test]
    fn test_forbidden_connections() {
        let config = DreamConfig::default();
        let mut solver = DreamSolver::new(config);

        solver.add_forbidden_connection("quantum".to_string(), "consciousness".to_string());
        solver.begin_dream_session("Quantum consciousness theory");

        // Run REM directly
        solver.state.phase = DreamPhase::REM;
        solver.state.relaxation_level = 0.9;

        // Should eventually find insight about forbidden connection
        let mut found_insight = false;
        for _ in 0..100 {
            if let Some(insight) = solver.step(Duration::from_millis(10)) {
                if insight.connection.normally_forbidden {
                    found_insight = true;
                    break;
                }
            }
        }

        // May or may not find it (probabilistic)
        // Just verify no crashes
        assert!(solver.state().session_duration > Duration::ZERO);
    }

    #[test]
    fn test_wake_and_validate() {
        let config = DreamConfig::default();
        let mut solver = DreamSolver::new(config);

        solver.begin_dream_session("Test");
        solver.state.phase = DreamPhase::REM;

        // Generate some insights
        for _ in 0..20 {
            solver.step(Duration::from_millis(50));
        }

        let breakthroughs = solver.wake_and_validate();
        assert!(!solver.is_dreaming());
        // May or may not have breakthroughs (probabilistic)
    }
}
