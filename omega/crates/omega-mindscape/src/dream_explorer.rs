//! Dream Explorer
//!
//! Explore the mindscape through REM sleep, discovering hidden connections
//! between memories that aren't visible during waking exploration.

use crate::coordinates::Position3D;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// A vision experienced during dream exploration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamVision {
    /// Unique vision ID
    pub id: String,
    /// Intensity of the vision (0-1)
    pub intensity: f64,
    /// Memories that appeared in the vision
    pub memory_fragments: Vec<String>,
    /// New connections discovered between memories
    pub discovered_connections: Vec<(String, String)>,
    /// Bizarreness level (how surreal)
    pub bizarreness: f64,
    /// Emotional tone (-1 to 1)
    pub emotional_tone: f64,
    /// Position in mindscape where vision occurred
    pub location: Position3D,
    /// Whether this was a lucid vision
    pub is_lucid: bool,
    /// Timestamp
    pub timestamp: u64,
}

impl DreamVision {
    /// Is this a significant vision worth recording?
    pub fn is_significant(&self) -> bool {
        !self.discovered_connections.is_empty() || self.intensity > 0.8
    }

    /// Is this a nightmare?
    pub fn is_nightmare(&self) -> bool {
        self.emotional_tone < -0.6 && self.intensity > 0.7
    }
}

/// A path through memory-space discovered in dreams
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamPath {
    /// Start memory
    pub from: String,
    /// End memory
    pub to: String,
    /// Intermediate waypoints (other memories touched)
    pub waypoints: Vec<String>,
    /// Strength of the connection (0-1)
    pub strength: f64,
    /// How the path was discovered (association type)
    pub association_type: AssociationType,
    /// Times this path has been dreamed
    pub dream_count: u32,
}

/// How memories are associated in dreams
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssociationType {
    /// Similar semantic content
    Semantic,
    /// Occurred at similar times
    Temporal,
    /// Share emotional tone
    Emotional,
    /// Visual/sensory similarity
    Perceptual,
    /// Bizarre dream-logic connection
    Surreal,
    /// Opposite/contrasting memories
    Contrast,
}

/// The dream exploration engine
pub struct DreamExplorer {
    /// Dream exploration speed multiplier
    speed_multiplier: f64,
    /// Available memories for dreaming
    memory_pool: Vec<(String, Vec<f64>)>,
    /// Current dream position in mindscape
    dream_position: Position3D,
    /// Discovered paths during dreams
    discovered_paths: Vec<DreamPath>,
    /// Current REM intensity
    rem_intensity: f64,
    /// Time in current dream (minutes)
    dream_time: f64,
    /// Visions generated
    visions: Vec<DreamVision>,
    /// Current lucidity level (0-1)
    lucidity: f64,
}

impl DreamExplorer {
    pub fn new(speed_multiplier: f64) -> Self {
        Self {
            speed_multiplier,
            memory_pool: Vec::new(),
            dream_position: Position3D::new(500.0, 500.0, 500.0),
            discovered_paths: Vec::new(),
            rem_intensity: 0.0,
            dream_time: 0.0,
            visions: Vec::new(),
            lucidity: 0.0,
        }
    }

    /// Add a memory to the dream pool
    pub fn add_memory(&mut self, label: String, embedding: Vec<f64>) {
        self.memory_pool.push((label, embedding));
    }

    /// Explore the dreamscape (called during REM)
    pub fn explore(&mut self) -> Option<DreamVision> {
        if self.memory_pool.is_empty() {
            return None;
        }

        let mut rng = rand::thread_rng();

        // Update dream state
        self.dream_time += 1.0 / self.speed_multiplier;
        self.rem_intensity = 0.5 + 0.5 * (self.dream_time * 0.1).sin().abs();

        // Random dream movement
        let dx = rng.gen_range(-50.0..50.0) * self.speed_multiplier;
        let dy = rng.gen_range(-50.0..50.0) * self.speed_multiplier;
        let dz = rng.gen_range(-20.0..20.0) * self.speed_multiplier;
        self.dream_position.x += dx;
        self.dream_position.y += dy;
        self.dream_position.z += dz;

        // Chance to generate a vision
        let vision_chance = self.rem_intensity * 0.3;
        if rng.gen::<f64>() < vision_chance {
            return Some(self.generate_vision());
        }

        None
    }

    /// Generate a dream vision
    fn generate_vision(&mut self) -> DreamVision {
        let mut rng = rand::thread_rng();

        // Sample some memories for the vision
        let num_fragments = rng.gen_range(2..=self.memory_pool.len().min(5));
        let mut memory_fragments = Vec::new();
        let mut indices: Vec<usize> = (0..self.memory_pool.len()).collect();

        for _ in 0..num_fragments {
            if indices.is_empty() {
                break;
            }
            let idx = rng.gen_range(0..indices.len());
            let mem_idx = indices.remove(idx);
            memory_fragments.push(self.memory_pool[mem_idx].0.clone());
        }

        // Discover connections between memories in this vision
        let mut discovered_connections = Vec::new();
        if memory_fragments.len() >= 2 {
            // Create connections between memories that appeared together
            for i in 0..memory_fragments.len() - 1 {
                let connection_strength = rng.gen::<f64>();
                if connection_strength > 0.5 {
                    discovered_connections.push((
                        memory_fragments[i].clone(),
                        memory_fragments[i + 1].clone(),
                    ));

                    // Also record as a discovered path
                    let path = DreamPath {
                        from: memory_fragments[i].clone(),
                        to: memory_fragments[i + 1].clone(),
                        waypoints: Vec::new(),
                        strength: connection_strength,
                        association_type: self.random_association_type(),
                        dream_count: 1,
                    };
                    self.discovered_paths.push(path);
                }
            }
        }

        let vision = DreamVision {
            id: uuid::Uuid::new_v4().to_string(),
            intensity: rng.gen_range(0.3..1.0),
            memory_fragments,
            discovered_connections,
            bizarreness: rng.gen_range(0.1..0.9),
            emotional_tone: rng.gen_range(-1.0..1.0),
            location: self.dream_position,
            is_lucid: self.lucidity > 0.5,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        };

        self.visions.push(vision.clone());
        vision
    }

    /// Get a random association type
    fn random_association_type(&self) -> AssociationType {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..6) {
            0 => AssociationType::Semantic,
            1 => AssociationType::Temporal,
            2 => AssociationType::Emotional,
            3 => AssociationType::Perceptual,
            4 => AssociationType::Surreal,
            _ => AssociationType::Contrast,
        }
    }

    /// Enter lucid dreaming mode
    pub fn become_lucid(&mut self) {
        self.lucidity = 1.0;
    }

    /// Exit lucid dreaming
    pub fn lose_lucidity(&mut self) {
        self.lucidity *= 0.9;
    }

    /// Directed exploration (lucid dreaming ability)
    /// Search for connections to a specific memory
    pub fn search_for(&mut self, target: &str) -> Option<DreamVision> {
        if self.lucidity < 0.3 {
            // Not lucid enough to direct dreams
            return self.explore();
        }

        let mut rng = rand::thread_rng();

        // Find target in memory pool
        let _target_idx = self.memory_pool.iter()
            .position(|(label, _)| label == target)?;

        // Directed dream - higher chance of finding connections to target
        let mut memory_fragments = vec![target.to_string()];

        // Add some related memories
        let num_related = rng.gen_range(1..=3);
        for _ in 0..num_related {
            let idx = rng.gen_range(0..self.memory_pool.len());
            memory_fragments.push(self.memory_pool[idx].0.clone());
        }

        // Create directed connections
        let mut discovered_connections = Vec::new();
        for i in 1..memory_fragments.len() {
            discovered_connections.push((
                memory_fragments[0].clone(),
                memory_fragments[i].clone(),
            ));

            let path = DreamPath {
                from: memory_fragments[0].clone(),
                to: memory_fragments[i].clone(),
                waypoints: Vec::new(),
                strength: rng.gen_range(0.6..1.0), // Stronger connections in lucid
                association_type: self.random_association_type(),
                dream_count: 1,
            };
            self.discovered_paths.push(path);
        }

        let vision = DreamVision {
            id: uuid::Uuid::new_v4().to_string(),
            intensity: rng.gen_range(0.7..1.0), // More intense in lucid
            memory_fragments,
            discovered_connections,
            bizarreness: rng.gen_range(0.1..0.5), // Less bizarre in lucid
            emotional_tone: rng.gen_range(0.0..1.0), // More positive in lucid
            location: self.dream_position,
            is_lucid: true,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        };

        // Lucidity fades with use
        self.lucidity *= 0.95;

        self.visions.push(vision.clone());
        Some(vision)
    }

    /// Get all discovered paths
    pub fn paths(&self) -> &[DreamPath] {
        &self.discovered_paths
    }

    /// Get all visions
    pub fn visions(&self) -> &[DreamVision] {
        &self.visions
    }

    /// Get current dream position
    pub fn position(&self) -> Position3D {
        self.dream_position
    }

    /// Get lucidity level
    pub fn lucidity(&self) -> f64 {
        self.lucidity
    }

    /// Get REM intensity
    pub fn rem_intensity(&self) -> f64 {
        self.rem_intensity
    }

    /// Reset for new dream session
    pub fn reset(&mut self) {
        self.dream_time = 0.0;
        self.rem_intensity = 0.0;
        self.lucidity = 0.0;
        self.visions.clear();
        // Keep discovered paths - they persist across sessions
    }

    /// Clear all state
    pub fn clear(&mut self) {
        self.reset();
        self.memory_pool.clear();
        self.discovered_paths.clear();
    }
}

impl Default for DreamExplorer {
    fn default() -> Self {
        Self::new(10.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dream_explorer_creation() {
        let explorer = DreamExplorer::new(10.0);
        assert_eq!(explorer.lucidity(), 0.0);
    }

    #[test]
    fn test_add_memories() {
        let mut explorer = DreamExplorer::new(10.0);

        explorer.add_memory("test1".to_string(), vec![0.5; 64]);
        explorer.add_memory("test2".to_string(), vec![0.7; 64]);

        assert_eq!(explorer.memory_pool.len(), 2);
    }

    #[test]
    fn test_exploration() {
        let mut explorer = DreamExplorer::new(10.0);

        explorer.add_memory("mem1".to_string(), vec![0.1; 64]);
        explorer.add_memory("mem2".to_string(), vec![0.2; 64]);
        explorer.add_memory("mem3".to_string(), vec![0.3; 64]);

        // Run many exploration cycles
        let mut visions_found = 0;
        for _ in 0..100 {
            if explorer.explore().is_some() {
                visions_found += 1;
            }
        }

        // Should generate some visions
        assert!(visions_found > 0);
    }

    #[test]
    fn test_lucid_dreaming() {
        let mut explorer = DreamExplorer::new(10.0);

        explorer.add_memory("target".to_string(), vec![0.5; 64]);
        explorer.add_memory("other".to_string(), vec![0.3; 64]);

        // Non-lucid search
        let vision1 = explorer.search_for("target");

        // Become lucid
        explorer.become_lucid();
        assert!(explorer.lucidity() > 0.9);

        // Lucid search should work better
        let vision2 = explorer.search_for("target");
        assert!(vision2.is_some());
        assert!(vision2.unwrap().is_lucid);
    }

    #[test]
    fn test_vision_significance() {
        let vision = DreamVision {
            id: "test".to_string(),
            intensity: 0.9,
            memory_fragments: vec!["a".to_string()],
            discovered_connections: vec![],
            bizarreness: 0.5,
            emotional_tone: 0.0,
            location: Position3D::origin(),
            is_lucid: false,
            timestamp: 0,
        };

        assert!(vision.is_significant()); // High intensity

        let nightmare = DreamVision {
            id: "test".to_string(),
            intensity: 0.9,
            memory_fragments: vec![],
            discovered_connections: vec![],
            bizarreness: 0.8,
            emotional_tone: -0.8,
            location: Position3D::origin(),
            is_lucid: false,
            timestamp: 0,
        };

        assert!(nightmare.is_nightmare());
    }
}
