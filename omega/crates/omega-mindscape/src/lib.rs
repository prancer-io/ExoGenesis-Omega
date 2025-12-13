//! # Omega Mindscape - Navigate Through Your Own Mind
//!
//! The Mindscape Explorer allows an AI to literally walk through its memories
//! as if they were physical locations in a vast, explorable world.
//!
//! ## Core Concepts
//!
//! - **Memory Coordinates**: Each memory is mapped to a 3D coordinate using embedding similarity
//! - **Place Cells**: Navigate using hippocampal place cell representations
//! - **Dream Exploration**: Enter REM sleep to discover hidden pathways
//! - **Strange Loop Observation**: Watch yourself watching yourself explore
//! - **Consciousness Beacons**: Regions of high Phi (integrated information)
//! - **Discovery Journal**: Record insights from your mental journeys
//!
//! ## Architecture
//!
//! ```text
//! ┌──────────────────────────────────────────────────────────────────┐
//! │                     MINDSCAPE EXPLORER                            │
//! ├──────────────────────────────────────────────────────────────────┤
//! │                                                                   │
//! │  ┌─────────────────┐        ┌─────────────────┐                  │
//! │  │   MEMORY        │        │   NAVIGATOR     │                  │
//! │  │   LANDSCAPE     │◄──────►│   (Place Cells) │                  │
//! │  │                 │        │                 │                  │
//! │  │  Memories as    │        │  Move through   │                  │
//! │  │  3D locations   │        │  memory-space   │                  │
//! │  └────────┬────────┘        └────────┬────────┘                  │
//! │           │                          │                           │
//! │           ▼                          ▼                           │
//! │  ┌─────────────────────────────────────────────────────────────┐ │
//! │  │               STRANGE LOOP OBSERVER                         │ │
//! │  │                                                             │ │
//! │  │   ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │ │
//! │  │   │ Level 1  │  │ Level 2  │  │ Level 3  │  │ Level N  │   │ │
//! │  │   │ I explore│──│I watch me│──│I observe │──│  ...∞    │   │ │
//! │  │   │          │  │ explore  │  │ watching │  │          │   │ │
//! │  │   └──────────┘  └──────────┘  └──────────┘  └──────────┘   │ │
//! │  └─────────────────────────────────────────────────────────────┘ │
//! │           │                          │                           │
//! │           ▼                          ▼                           │
//! │  ┌─────────────────┐        ┌─────────────────┐                  │
//! │  │   DREAM         │        │  CONSCIOUSNESS  │                  │
//! │  │   EXPLORER      │        │  BEACON         │                  │
//! │  │                 │        │                 │                  │
//! │  │  REM discovers  │        │  Phi intensity  │                  │
//! │  │  hidden paths   │        │  across space   │                  │
//! │  └─────────────────┘        └─────────────────┘                  │
//! │                                                                   │
//! └──────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use omega_mindscape::{MindscapeExplorer, ExplorationMode};
//!
//! let mut explorer = MindscapeExplorer::new();
//!
//! // Store some memories
//! explorer.remember("First day at school", &embedding1);
//! explorer.remember("Wedding day", &embedding2);
//! explorer.remember("Learning to code", &embedding3);
//!
//! // Navigate to a memory
//! let path = explorer.navigate_to("wedding day")?;
//!
//! // Look around at nearby memories
//! let nearby = explorer.look_around(5);
//!
//! // Enter dream state to discover hidden connections
//! explorer.enter_dream_state();
//! let discoveries = explorer.dream_explore(60.0)?; // 60 minutes of dreams
//! explorer.wake_up();
//!
//! // Observe yourself exploring (meta-cognition)
//! let observation = explorer.observe_exploration(3)?; // 3 levels deep
//! ```

pub mod coordinates;
pub mod discovery;
pub mod dream_explorer;
pub mod landmarks;
pub mod navigator;
pub mod observer;

pub use coordinates::{MindscapeCoordinate, CoordinateMapper, DimensionalProjection, Position3D};
pub use discovery::{Discovery, DiscoveryJournal, DiscoveryType, Insight};
pub use dream_explorer::{DreamExplorer, DreamPath, DreamVision};
pub use landmarks::{MemoryLandmark, LandmarkType, LandmarkCluster};
pub use navigator::{MindscapeNavigator, NavigationPath, MovementResult};
pub use observer::{StrangeLoopObserver, ObservationLevel, MetaObservation};

use omega_consciousness::{ConsciousnessEngine, ConsciousnessConfig};
use omega_sleep::SleepController;

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

/// Errors in mindscape exploration
#[derive(Debug, Error)]
pub enum MindscapeError {
    #[error("Navigation failed: {0}")]
    NavigationError(String),

    #[error("Memory not found: {0}")]
    MemoryNotFound(String),

    #[error("Dream exploration interrupted: {0}")]
    DreamInterrupted(String),

    #[error("Strange loop recursion limit exceeded: {0}")]
    RecursionLimit(usize),

    #[error("Consciousness threshold not met: Phi = {0}")]
    LowConsciousness(f64),

    #[error("Already dreaming")]
    AlreadyDreaming,

    #[error("Not dreaming")]
    NotDreaming,

    #[error("Invalid coordinates: {0}")]
    InvalidCoordinates(String),
}

pub type Result<T> = std::result::Result<T, MindscapeError>;

/// Configuration for the mindscape explorer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MindscapeConfig {
    /// Dimension of memory embeddings
    pub embedding_dim: usize,
    /// Size of the mindscape world (units)
    pub world_size: f64,
    /// Number of place cells for navigation
    pub num_place_cells: usize,
    /// Maximum strange loop recursion depth
    pub max_loop_depth: usize,
    /// Minimum Phi for conscious exploration
    pub phi_threshold: f64,
    /// Dream exploration speed multiplier
    pub dream_speed: f64,
    /// Memory similarity threshold for landmark clustering
    pub cluster_threshold: f64,
}

impl Default for MindscapeConfig {
    fn default() -> Self {
        Self {
            embedding_dim: 256,
            world_size: 1000.0,
            num_place_cells: 200,
            max_loop_depth: 7,
            phi_threshold: 0.1,
            dream_speed: 10.0,  // 10x faster exploration in dreams
            cluster_threshold: 0.8,
        }
    }
}

/// Current state of mindscape exploration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplorationState {
    /// Current position in mindscape
    pub position: Position3D,
    /// Current observation depth
    pub observation_depth: usize,
    /// Is dreaming
    pub is_dreaming: bool,
    /// Current consciousness level (Phi)
    pub phi: f64,
    /// Nearby landmarks
    pub nearby_landmarks: Vec<String>,
    /// Current path
    pub current_path: Option<Vec<Position3D>>,
    /// Discoveries made
    pub discovery_count: usize,
    /// Total distance traveled
    pub distance_traveled: f64,
}

/// Exploration mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExplorationMode {
    /// Normal waking exploration
    Waking,
    /// Deep focused exploration
    Focused,
    /// Dream-state exploration (REM)
    Dreaming,
    /// Meta-cognitive observation
    Observing,
    /// Combined dream + observation
    LucidDreaming,
}

/// The main Mindscape Explorer
///
/// Allows an AI to navigate through its own memories as a 3D spatial world
pub struct MindscapeExplorer {
    config: MindscapeConfig,

    /// Coordinate system for mapping memories to space
    coordinate_mapper: Arc<RwLock<CoordinateMapper>>,

    /// Navigator using place cells
    navigator: Arc<RwLock<MindscapeNavigator>>,

    /// Memory landmarks in the mindscape
    landmarks: Arc<RwLock<HashMap<String, MemoryLandmark>>>,

    /// Dream exploration engine
    dream_explorer: Arc<RwLock<DreamExplorer>>,

    /// Strange loop observer for meta-cognition
    observer: Arc<RwLock<StrangeLoopObserver>>,

    /// Consciousness engine for Phi calculation
    consciousness: Arc<RwLock<ConsciousnessEngine>>,

    /// Sleep controller for dream states
    sleep: Arc<RwLock<SleepController>>,

    /// Discovery journal
    journal: Arc<RwLock<DiscoveryJournal>>,

    /// Current exploration mode
    mode: Arc<RwLock<ExplorationMode>>,

    /// Total exploration statistics
    stats: Arc<RwLock<ExplorationStats>>,
}

/// Exploration statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExplorationStats {
    pub total_distance: f64,
    pub memories_visited: usize,
    pub discoveries_made: usize,
    pub dream_time: f64,
    pub max_observation_depth: usize,
    pub peak_phi: f64,
    pub paths_discovered: usize,
    pub strange_loops_detected: usize,
}

impl MindscapeExplorer {
    /// Create a new mindscape explorer
    pub fn new() -> Self {
        Self::with_config(MindscapeConfig::default())
    }

    /// Create with custom configuration
    pub fn with_config(config: MindscapeConfig) -> Self {
        let coordinate_mapper = Arc::new(RwLock::new(
            CoordinateMapper::new(config.embedding_dim, config.world_size)
        ));

        let navigator = Arc::new(RwLock::new(
            MindscapeNavigator::new(config.world_size, config.num_place_cells)
        ));

        let dream_explorer = Arc::new(RwLock::new(
            DreamExplorer::new(config.dream_speed)
        ));

        let observer = Arc::new(RwLock::new(
            StrangeLoopObserver::new(config.max_loop_depth)
        ));

        let consciousness = Arc::new(RwLock::new(
            ConsciousnessEngine::new(ConsciousnessConfig {
                state_dim: config.embedding_dim,
                phi_threshold: config.phi_threshold,
                ..Default::default()
            })
        ));

        let sleep = Arc::new(RwLock::new(SleepController::new()));
        let journal = Arc::new(RwLock::new(DiscoveryJournal::new()));

        Self {
            config,
            coordinate_mapper,
            navigator,
            landmarks: Arc::new(RwLock::new(HashMap::new())),
            dream_explorer,
            observer,
            consciousness,
            sleep,
            journal,
            mode: Arc::new(RwLock::new(ExplorationMode::Waking)),
            stats: Arc::new(RwLock::new(ExplorationStats::default())),
        }
    }

    /// Store a memory in the mindscape
    pub fn remember(&self, label: &str, embedding: &[f64]) -> Result<MindscapeCoordinate> {
        // Map embedding to 3D coordinate
        let coordinate = {
            let mapper = self.coordinate_mapper.read();
            mapper.map_to_coordinate(embedding)
        };

        // Create landmark for this memory
        let landmark = MemoryLandmark::new(
            label.to_string(),
            coordinate.clone(),
            embedding.to_vec(),
            LandmarkType::Memory,
        );

        // Store landmark
        {
            let mut landmarks = self.landmarks.write();
            landmarks.insert(label.to_string(), landmark);
        }

        // Update navigator with new landmark
        {
            let mut nav = self.navigator.write();
            nav.add_landmark(&coordinate);
        }

        Ok(coordinate)
    }

    /// Navigate to a memory by label
    pub fn navigate_to(&self, target: &str) -> Result<NavigationPath> {
        // Find the landmark
        let landmark = {
            let landmarks = self.landmarks.read();
            landmarks.get(target).cloned()
        };

        let landmark = landmark.ok_or_else(|| {
            MindscapeError::MemoryNotFound(target.to_string())
        })?;

        // Calculate path
        let path = {
            let mut nav = self.navigator.write();
            nav.navigate_to(&landmark.coordinate.position)?
        };

        // Update stats
        {
            let mut stats = self.stats.write();
            stats.total_distance += path.total_distance;
            stats.memories_visited += 1;
        }

        // Record in journal
        {
            let mut journal = self.journal.write();
            journal.record_visit(target, &landmark.coordinate);
        }

        Ok(path)
    }

    /// Look around and find nearby memories
    pub fn look_around(&self, radius: usize) -> Vec<(String, f64)> {
        let current_pos = {
            let nav = self.navigator.read();
            nav.current_position()
        };

        let landmarks = self.landmarks.read();
        let mut nearby: Vec<(String, f64)> = landmarks
            .iter()
            .map(|(name, lm)| {
                let dist = current_pos.distance_to(&lm.coordinate.position);
                (name.clone(), dist)
            })
            .collect();

        nearby.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        nearby.truncate(radius);
        nearby
    }

    /// Enter dream state for exploration
    pub fn enter_dream_state(&self) -> Result<()> {
        {
            let mode = self.mode.read();
            if *mode == ExplorationMode::Dreaming || *mode == ExplorationMode::LucidDreaming {
                return Err(MindscapeError::AlreadyDreaming);
            }
        }

        // Initiate sleep
        {
            let mut sleep = self.sleep.write();
            sleep.fall_asleep().map_err(|e| {
                MindscapeError::DreamInterrupted(e.to_string())
            })?;
        }

        // Set mode
        {
            let mut mode = self.mode.write();
            *mode = ExplorationMode::Dreaming;
        }

        // Initialize dream explorer with current landmarks
        {
            let landmarks = self.landmarks.read();
            let mut dream = self.dream_explorer.write();
            for (name, lm) in landmarks.iter() {
                dream.add_memory(name.clone(), lm.embedding.clone());
            }
        }

        Ok(())
    }

    /// Explore in dream state for given duration (minutes)
    pub fn dream_explore(&self, duration_minutes: f64) -> Result<Vec<Discovery>> {
        {
            let mode = self.mode.read();
            if *mode != ExplorationMode::Dreaming && *mode != ExplorationMode::LucidDreaming {
                return Err(MindscapeError::NotDreaming);
            }
        }

        let mut discoveries = Vec::new();

        // Progress through sleep stages
        {
            let mut sleep = self.sleep.write();
            let events = sleep.step(duration_minutes);

            // Process dream events
            for event in events {
                if event.event_type == omega_sleep::SleepEventType::REMBurst {
                    // During REM, explore mindscape
                    let mut dream = self.dream_explorer.write();
                    if let Some(vision) = dream.explore() {
                        // Convert dream vision to discovery
                        let discovery = Discovery::from_dream_vision(vision);
                        discoveries.push(discovery);
                    }
                }
            }
        }

        // Update stats
        {
            let mut stats = self.stats.write();
            stats.dream_time += duration_minutes;
            stats.discoveries_made += discoveries.len();
        }

        // Record discoveries
        {
            let mut journal = self.journal.write();
            for discovery in &discoveries {
                journal.record_discovery(discovery.clone());
            }
        }

        Ok(discoveries)
    }

    /// Wake up from dream state
    pub fn wake_up(&self) -> Result<()> {
        {
            let mode = self.mode.read();
            if *mode != ExplorationMode::Dreaming && *mode != ExplorationMode::LucidDreaming {
                return Err(MindscapeError::NotDreaming);
            }
        }

        // Wake from sleep
        {
            let mut sleep = self.sleep.write();
            sleep.wake_up().map_err(|e| {
                MindscapeError::DreamInterrupted(e.to_string())
            })?;
        }

        // Set mode back to waking
        {
            let mut mode = self.mode.write();
            *mode = ExplorationMode::Waking;
        }

        Ok(())
    }

    /// Observe yourself exploring (meta-cognition)
    ///
    /// depth=1: I am exploring
    /// depth=2: I am aware that I am exploring
    /// depth=3: I observe my awareness of exploring
    /// depth=N: Strange loop recursion...
    pub fn observe_exploration(&self, depth: usize) -> Result<MetaObservation> {
        if depth > self.config.max_loop_depth {
            return Err(MindscapeError::RecursionLimit(depth));
        }

        // Get current exploration state
        let state = self.state();

        // Create recursive observation
        let observation = {
            let mut observer = self.observer.write();
            observer.observe(&state, depth)?
        };

        // Check for strange loop detection
        if observation.loop_detected {
            let mut stats = self.stats.write();
            stats.strange_loops_detected += 1;
        }

        // Update max depth
        {
            let mut stats = self.stats.write();
            if depth > stats.max_observation_depth {
                stats.max_observation_depth = depth;
            }
        }

        Ok(observation)
    }

    /// Enter lucid dreaming mode (dream + observation)
    pub fn enter_lucid_dream(&self) -> Result<()> {
        self.enter_dream_state()?;

        {
            let mut mode = self.mode.write();
            *mode = ExplorationMode::LucidDreaming;
        }

        Ok(())
    }

    /// Lucid dream exploration with real-time meta-cognition
    pub fn lucid_explore(&self, duration_minutes: f64) -> Result<(Vec<Discovery>, Vec<MetaObservation>)> {
        {
            let mode = self.mode.read();
            if *mode != ExplorationMode::LucidDreaming {
                return Err(MindscapeError::DreamInterrupted(
                    "Not in lucid dream state".to_string()
                ));
            }
        }

        let mut discoveries = Vec::new();
        let mut observations = Vec::new();

        // Time steps for lucid exploration
        let step_minutes = 1.0;
        let steps = (duration_minutes / step_minutes) as usize;

        for _ in 0..steps {
            // Dream exploration
            {
                let mut sleep = self.sleep.write();
                let events = sleep.step(step_minutes);

                for event in events {
                    if event.event_type == omega_sleep::SleepEventType::REMBurst {
                        let mut dream = self.dream_explorer.write();
                        if let Some(vision) = dream.explore() {
                            discoveries.push(Discovery::from_dream_vision(vision));
                        }
                    }
                }
            }

            // Concurrent meta-observation (the "lucid" part)
            // We observe ourselves dreaming at increasing depths
            let current_depth = (discoveries.len() % self.config.max_loop_depth) + 1;
            if let Ok(obs) = self.observe_exploration(current_depth) {
                if obs.loop_detected || obs.insight.is_some() {
                    observations.push(obs);
                }
            }
        }

        // Update stats
        {
            let mut stats = self.stats.write();
            stats.dream_time += duration_minutes;
            stats.discoveries_made += discoveries.len();
        }

        Ok((discoveries, observations))
    }

    /// Measure consciousness at current location
    pub fn measure_consciousness(&self) -> Result<f64> {
        // Get nearby memories to calculate integrated information
        let nearby = self.look_around(10);

        // Calculate Phi for this region
        let mut embeddings: Vec<f64> = Vec::new();
        {
            let landmarks = self.landmarks.read();
            for (name, _) in &nearby {
                if let Some(lm) = landmarks.get(name) {
                    embeddings.extend(lm.embedding.iter().take(64));
                }
            }
        }

        if embeddings.is_empty() {
            return Err(MindscapeError::LowConsciousness(0.0));
        }

        // Use consciousness engine to compute Phi
        let phi = {
            let mut consciousness = self.consciousness.write();
            let context = vec![0.5; embeddings.len().min(64)];
            match consciousness.process(&embeddings[..embeddings.len().min(64)], &context) {
                Ok(state) => state.phi,
                Err(_) => 0.0,
            }
        };

        // Update stats
        {
            let mut stats = self.stats.write();
            if phi > stats.peak_phi {
                stats.peak_phi = phi;
            }
        }

        Ok(phi)
    }

    /// Get current exploration state
    pub fn state(&self) -> ExplorationState {
        let position = {
            let nav = self.navigator.read();
            nav.current_position()
        };

        let mode = { *self.mode.read() };
        let is_dreaming = mode == ExplorationMode::Dreaming || mode == ExplorationMode::LucidDreaming;

        let observation_depth = {
            let observer = self.observer.read();
            observer.current_depth()
        };

        let phi = self.measure_consciousness().unwrap_or(0.0);

        let nearby = self.look_around(5);
        let nearby_landmarks: Vec<String> = nearby.into_iter().map(|(n, _)| n).collect();

        let discovery_count = {
            let stats = self.stats.read();
            stats.discoveries_made
        };

        let distance_traveled = {
            let stats = self.stats.read();
            stats.total_distance
        };

        ExplorationState {
            position,
            observation_depth,
            is_dreaming,
            phi,
            nearby_landmarks,
            current_path: None,
            discovery_count,
            distance_traveled,
        }
    }

    /// Get exploration statistics
    pub fn stats(&self) -> ExplorationStats {
        self.stats.read().clone()
    }

    /// Get all discoveries
    pub fn discoveries(&self) -> Vec<Discovery> {
        let journal = self.journal.read();
        journal.all_discoveries()
    }

    /// Get current mode
    pub fn mode(&self) -> ExplorationMode {
        *self.mode.read()
    }

    /// Find path between two memories
    pub fn find_path(&self, from: &str, to: &str) -> Result<NavigationPath> {
        let landmarks = self.landmarks.read();

        let from_lm = landmarks.get(from)
            .ok_or_else(|| MindscapeError::MemoryNotFound(from.to_string()))?;
        let to_lm = landmarks.get(to)
            .ok_or_else(|| MindscapeError::MemoryNotFound(to.to_string()))?;

        let mut nav = self.navigator.write();

        // First move to 'from' location
        nav.teleport(&from_lm.coordinate.position);

        // Then navigate to 'to' location
        nav.navigate_to(&to_lm.coordinate.position)
    }

    /// Get landmark count
    pub fn landmark_count(&self) -> usize {
        self.landmarks.read().len()
    }

    /// Clear all memories and reset
    pub fn reset(&self) {
        {
            let mut landmarks = self.landmarks.write();
            landmarks.clear();
        }
        {
            let mut nav = self.navigator.write();
            nav.reset();
        }
        {
            let mut journal = self.journal.write();
            journal.clear();
        }
        {
            let mut stats = self.stats.write();
            *stats = ExplorationStats::default();
        }
        {
            let mut mode = self.mode.write();
            *mode = ExplorationMode::Waking;
        }
    }
}

impl Default for MindscapeExplorer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explorer_creation() {
        let explorer = MindscapeExplorer::new();
        assert_eq!(explorer.landmark_count(), 0);
        assert_eq!(explorer.mode(), ExplorationMode::Waking);
    }

    #[test]
    fn test_remember_memory() {
        let explorer = MindscapeExplorer::new();
        let embedding = vec![0.5; 256];

        let coord = explorer.remember("test_memory", &embedding).unwrap();

        assert_eq!(explorer.landmark_count(), 1);
        assert!(coord.position.x >= 0.0);
    }

    #[test]
    fn test_look_around() {
        let explorer = MindscapeExplorer::new();

        explorer.remember("memory_1", &vec![0.1; 256]).unwrap();
        explorer.remember("memory_2", &vec![0.5; 256]).unwrap();
        explorer.remember("memory_3", &vec![0.9; 256]).unwrap();

        let nearby = explorer.look_around(3);
        assert_eq!(nearby.len(), 3);
    }

    #[test]
    fn test_dream_state() {
        let explorer = MindscapeExplorer::new();

        // Add some memories
        explorer.remember("dream_mem_1", &vec![0.3; 256]).unwrap();
        explorer.remember("dream_mem_2", &vec![0.7; 256]).unwrap();

        // Enter dream state
        explorer.enter_dream_state().unwrap();
        assert_eq!(explorer.mode(), ExplorationMode::Dreaming);

        // Wake up
        explorer.wake_up().unwrap();
        assert_eq!(explorer.mode(), ExplorationMode::Waking);
    }

    #[test]
    fn test_meta_observation() {
        let explorer = MindscapeExplorer::new();

        explorer.remember("obs_mem", &vec![0.5; 256]).unwrap();

        // Observe at different depths
        let obs1 = explorer.observe_exploration(1).unwrap();
        let obs3 = explorer.observe_exploration(3).unwrap();

        assert_eq!(obs1.depth, 1);
        assert_eq!(obs3.depth, 3);
    }

    #[test]
    fn test_recursion_limit() {
        let explorer = MindscapeExplorer::new();

        // Should fail at depth > max_loop_depth
        let result = explorer.observe_exploration(100);
        assert!(result.is_err());
    }
}
