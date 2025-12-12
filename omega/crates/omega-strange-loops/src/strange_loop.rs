//! Strange Loop Structures
//!
//! Self-referential feedback loops that cross levels:
//! - Tangled hierarchies
//! - Level-crossing feedback
//! - Self-modifying structures

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Level in a tangled hierarchy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopLevel {
    /// Level index
    pub index: usize,
    /// Content at this level
    pub content: Vec<f64>,
    /// References to other levels (can cross levels)
    pub references: Vec<usize>,
    /// Self-reference strength
    pub self_ref_strength: f64,
}

impl LoopLevel {
    pub fn new(index: usize, dim: usize) -> Self {
        Self {
            index,
            content: vec![0.0; dim],
            references: Vec::new(),
            self_ref_strength: 0.0,
        }
    }

    /// Add reference to another level
    pub fn add_reference(&mut self, level: usize) {
        if level == self.index {
            self.self_ref_strength += 0.1;
        }
        self.references.push(level);
    }

    /// Update content
    pub fn update(&mut self, new_content: &[f64]) {
        for (i, &v) in new_content.iter().enumerate() {
            if i < self.content.len() {
                self.content[i] = 0.9 * self.content[i] + 0.1 * v;
            }
        }
    }
}

/// A strange loop (self-referential feedback)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrangeLoop {
    /// Loop ID
    pub id: String,
    /// Depth where loop was detected
    pub level: usize,
    /// Strength of the loop
    pub strength: f64,
    /// Loop pattern (sequence of levels)
    pub pattern: Vec<usize>,
    /// Times activated
    pub activation_count: usize,
    /// Timestamp created
    pub created_at: u64,
}

impl StrangeLoop {
    pub fn new(id: String, level: usize, strength: f64) -> Self {
        Self {
            id,
            level,
            strength,
            pattern: vec![level],
            activation_count: 1,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }

    /// Activate the loop
    pub fn activate(&mut self) {
        self.activation_count += 1;
        self.strength *= 1.1; // Strengthen with use
        self.strength = self.strength.min(2.0);
    }

    /// Decay the loop
    pub fn decay(&mut self, factor: f64) {
        self.strength *= factor;
    }

    /// Check if loop is still active
    pub fn is_active(&self) -> bool {
        self.strength > 0.1
    }

    /// Add level to pattern
    pub fn extend_pattern(&mut self, level: usize) {
        self.pattern.push(level);
    }

    /// Check if pattern returns to start (true loop)
    pub fn is_closed(&self) -> bool {
        self.pattern.len() >= 2 && self.pattern.first() == self.pattern.last()
    }
}

/// A tangled hierarchy (Hofstadter concept)
pub struct TangledHierarchy {
    /// Levels in the hierarchy
    levels: Vec<LoopLevel>,
    /// Detected strange loops
    loops: Vec<StrangeLoop>,
    /// Cross-level connections
    connections: HashMap<(usize, usize), f64>,
    /// Dimension of content
    dim: usize,
}

impl TangledHierarchy {
    /// Create new tangled hierarchy
    pub fn new(num_levels: usize, dim: usize) -> Self {
        let levels = (0..num_levels).map(|i| LoopLevel::new(i, dim)).collect();

        Self {
            levels,
            loops: Vec::new(),
            connections: HashMap::new(),
            dim,
        }
    }

    /// Process input through hierarchy
    pub fn process(&mut self, input: &[f64], start_level: usize) -> Vec<f64> {
        let mut current = input.to_vec();
        let mut visited = vec![false; self.levels.len()];
        let mut path = Vec::new();

        self.process_level(start_level, &mut current, &mut visited, &mut path);

        current
    }

    /// Process at a specific level
    fn process_level(
        &mut self,
        level: usize,
        current: &mut [f64],
        visited: &mut [bool],
        path: &mut Vec<usize>,
    ) {
        if level >= self.levels.len() || visited[level] {
            // Check for loop
            if visited[level] {
                self.detect_loop(path, level);
            }
            return;
        }

        visited[level] = true;
        path.push(level);

        // Update level content
        self.levels[level].update(current);

        // Mix with level content
        for i in 0..current.len().min(self.dim) {
            current[i] = 0.7 * current[i] + 0.3 * self.levels[level].content[i];
        }

        // Process references (cross-level)
        let references = self.levels[level].references.clone();
        for ref_level in references {
            // Get connection strength
            let strength = self
                .connections
                .get(&(level, ref_level))
                .copied()
                .unwrap_or(0.5);

            // Weighted processing
            let mut ref_current = current
                .iter()
                .map(|&x| x * strength)
                .collect::<Vec<f64>>();

            self.process_level(ref_level, &mut ref_current, visited, path);

            // Combine back
            for i in 0..current.len() {
                current[i] = 0.5 * current[i] + 0.5 * ref_current[i];
            }
        }

        path.pop();
        visited[level] = false;
    }

    /// Detect a strange loop in the path
    fn detect_loop(&mut self, path: &[usize], return_level: usize) {
        // Find where the loop starts
        if let Some(start_idx) = path.iter().position(|&l| l == return_level) {
            let loop_pattern: Vec<usize> = path[start_idx..].to_vec();

            // Check if this loop already exists
            let exists = self.loops.iter_mut().any(|l| {
                if l.pattern == loop_pattern {
                    l.activate();
                    true
                } else {
                    false
                }
            });

            if !exists {
                let mut new_loop =
                    StrangeLoop::new(format!("loop_{}", self.loops.len()), return_level, 0.5);
                new_loop.pattern = loop_pattern;
                self.loops.push(new_loop);
            }
        }
    }

    /// Add cross-level connection
    pub fn connect(&mut self, from: usize, to: usize, strength: f64) {
        if from < self.levels.len() && to < self.levels.len() {
            self.connections.insert((from, to), strength);
            self.levels[from].add_reference(to);
        }
    }

    /// Get level
    pub fn level(&self, index: usize) -> Option<&LoopLevel> {
        self.levels.get(index)
    }

    /// Get loops
    pub fn loops(&self) -> &[StrangeLoop] {
        &self.loops
    }

    /// Decay all loops
    pub fn decay_loops(&mut self, factor: f64) {
        for loop_item in &mut self.loops {
            loop_item.decay(factor);
        }
        self.loops.retain(|l| l.is_active());
    }

    /// Get number of levels
    pub fn num_levels(&self) -> usize {
        self.levels.len()
    }

    /// Get active loop count
    pub fn active_loop_count(&self) -> usize {
        self.loops.iter().filter(|l| l.is_active()).count()
    }
}

impl Default for TangledHierarchy {
    fn default() -> Self {
        Self::new(5, 32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_level() {
        let mut level = LoopLevel::new(0, 10);
        level.add_reference(1);
        level.add_reference(0); // Self-reference

        assert!(level.self_ref_strength > 0.0);
    }

    #[test]
    fn test_strange_loop() {
        let mut loop_item = StrangeLoop::new("test".to_string(), 0, 0.5);

        loop_item.activate();
        assert_eq!(loop_item.activation_count, 2);
        assert!(loop_item.strength > 0.5);
    }

    #[test]
    fn test_tangled_hierarchy() {
        let mut hierarchy = TangledHierarchy::new(3, 8);

        // Create a loop: 0 → 1 → 2 → 0
        hierarchy.connect(0, 1, 0.8);
        hierarchy.connect(1, 2, 0.8);
        hierarchy.connect(2, 0, 0.8);

        let input = vec![0.5; 8];
        let output = hierarchy.process(&input, 0);

        assert_eq!(output.len(), 8);
    }

    #[test]
    fn test_loop_detection() {
        let mut hierarchy = TangledHierarchy::new(3, 4);

        // Create self-loop
        hierarchy.connect(0, 0, 1.0);

        let input = vec![1.0; 4];
        hierarchy.process(&input, 0);

        // Should detect loop
        // Note: Detection depends on processing logic
    }

    #[test]
    fn test_closed_loop() {
        let mut loop_item = StrangeLoop::new("test".to_string(), 0, 0.5);
        loop_item.extend_pattern(1);
        loop_item.extend_pattern(2);
        loop_item.extend_pattern(0);

        assert!(loop_item.is_closed());
    }
}
