//! Mirror Structures
//!
//! Self-referential reflections:
//! - Seeing ourselves seeing
//! - Recursive mirrors
//! - Representation of representations

use serde::{Deserialize, Serialize};

/// A reflection in the mirror
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirrorReflection {
    /// Original content
    pub original: Vec<f64>,
    /// Reflected content
    pub reflection: Vec<f64>,
    /// Depth of reflection (how many times reflected)
    pub depth: usize,
    /// Fidelity of reflection (0-1)
    pub fidelity: f64,
}

impl MirrorReflection {
    pub fn new(original: Vec<f64>, depth: usize) -> Self {
        // Reflection with some distortion
        let reflection: Vec<f64> = original
            .iter()
            .enumerate()
            .map(|(i, &x)| {
                let distortion = 1.0 - (depth as f64 * 0.1).min(0.5);
                x * distortion + (i as f64 / original.len() as f64) * (1.0 - distortion)
            })
            .collect();

        let fidelity = 1.0 - (depth as f64 * 0.1).min(0.9);

        Self {
            original,
            reflection,
            depth,
            fidelity,
        }
    }

    /// Compare reflection to original
    pub fn similarity(&self) -> f64 {
        if self.original.len() != self.reflection.len() || self.original.is_empty() {
            return 0.0;
        }

        let mut dot = 0.0;
        let mut norm_a = 0.0;
        let mut norm_b = 0.0;

        for (&o, &r) in self.original.iter().zip(self.reflection.iter()) {
            dot += o * r;
            norm_a += o * o;
            norm_b += r * r;
        }

        let denom = (norm_a * norm_b).sqrt();
        if denom > 0.0 {
            dot / denom
        } else {
            0.0
        }
    }
}

/// A simple mirror
#[derive(Debug, Clone)]
pub struct Mirror {
    /// Reflection delay (cognitive processing time)
    pub delay: f64,
    /// Distortion factor
    pub distortion: f64,
    /// Last reflection
    last_reflection: Option<MirrorReflection>,
}

impl Mirror {
    pub fn new() -> Self {
        Self {
            delay: 0.1,
            distortion: 0.05,
            last_reflection: None,
        }
    }

    /// Reflect input
    pub fn reflect(&mut self, input: &[f64]) -> Vec<f64> {
        let reflection = MirrorReflection::new(input.to_vec(), 1);
        let result = reflection.reflection.clone();
        self.last_reflection = Some(reflection);
        result
    }

    /// Get last reflection
    pub fn last_reflection(&self) -> Option<&MirrorReflection> {
        self.last_reflection.as_ref()
    }
}

impl Default for Mirror {
    fn default() -> Self {
        Self::new()
    }
}

/// Recursive mirror (mirrors reflecting mirrors)
pub struct RecursiveMirror {
    /// Maximum recursion depth
    max_depth: usize,
    /// Reflections at each depth
    reflections: Vec<MirrorReflection>,
    /// Distortion per level (0.0-1.0)
    distortion_per_level: f64,
}

impl RecursiveMirror {
    /// Create new recursive mirror
    pub fn new(max_depth: usize) -> Self {
        Self {
            max_depth,
            reflections: Vec::new(),
            distortion_per_level: 0.1,
        }
    }

    /// Reflect recursively
    pub fn reflect(&mut self, input: &[f64]) -> Vec<f64> {
        self.reflections.clear();

        let mut current = input.to_vec();

        for depth in 0..self.max_depth {
            // Apply per-level distortion
            let distorted: Vec<f64> = current.iter()
                .enumerate()
                .map(|(i, &x)| {
                    let noise = (i as f64 * 0.1).sin() * self.distortion_per_level;
                    x * (1.0 - self.distortion_per_level) + noise
                })
                .collect();

            let reflection = MirrorReflection::new(distorted, depth);
            current = reflection.reflection.clone();
            self.reflections.push(reflection);

            // Early termination if too distorted
            if current.iter().all(|&x| x.abs() < 0.01) {
                break;
            }
        }

        // Return the final reflection
        current
    }

    /// Set distortion per level
    pub fn set_distortion(&mut self, distortion: f64) {
        self.distortion_per_level = distortion.max(0.0).min(1.0);
    }

    /// Get distortion per level
    pub fn distortion_per_level(&self) -> f64 {
        self.distortion_per_level
    }

    /// Get reflection at depth
    pub fn reflection_at_depth(&self, depth: usize) -> Option<&MirrorReflection> {
        self.reflections.get(depth)
    }

    /// Get all reflections
    pub fn all_reflections(&self) -> &[MirrorReflection] {
        &self.reflections
    }

    /// Get similarity decay across depths
    pub fn similarity_decay(&self) -> Vec<f64> {
        self.reflections.iter().map(|r| r.similarity()).collect()
    }

    /// Get total fidelity (product of all fidelities)
    pub fn total_fidelity(&self) -> f64 {
        self.reflections
            .iter()
            .map(|r| r.fidelity)
            .product::<f64>()
    }

    /// Get depth reached
    pub fn depth_reached(&self) -> usize {
        self.reflections.len()
    }
}

impl Default for RecursiveMirror {
    fn default() -> Self {
        Self::new(5)
    }
}

/// Mirror of mirrors (meta-mirror)
pub struct MetaMirror {
    /// Primary mirror
    primary: RecursiveMirror,
    /// Secondary mirror (reflects the reflection process)
    secondary: RecursiveMirror,
    /// Meta-reflections (reflection of reflection patterns)
    meta_reflections: Vec<Vec<f64>>,
}

impl MetaMirror {
    pub fn new(depth: usize) -> Self {
        Self {
            primary: RecursiveMirror::new(depth),
            secondary: RecursiveMirror::new(depth / 2 + 1),
            meta_reflections: Vec::new(),
        }
    }

    /// Reflect and observe the reflection
    pub fn reflect_and_observe(&mut self, input: &[f64]) -> Vec<f64> {
        // Primary reflection
        let primary_result = self.primary.reflect(input);

        // Reflect the reflection (meta-level)
        let meta_result = self.secondary.reflect(&primary_result);

        // Store meta-pattern (difference between levels)
        let meta_pattern: Vec<f64> = primary_result
            .iter()
            .zip(meta_result.iter())
            .map(|(&p, &m)| p - m)
            .collect();
        self.meta_reflections.push(meta_pattern);

        // Limit meta-reflection storage
        if self.meta_reflections.len() > 100 {
            self.meta_reflections.remove(0);
        }

        meta_result
    }

    /// Get meta-reflection patterns
    pub fn meta_patterns(&self) -> &[Vec<f64>] {
        &self.meta_reflections
    }

    /// Get average meta-pattern
    pub fn average_meta_pattern(&self) -> Vec<f64> {
        if self.meta_reflections.is_empty() {
            return Vec::new();
        }

        let dim = self.meta_reflections[0].len();
        let mut avg = vec![0.0; dim];

        for pattern in &self.meta_reflections {
            for (i, &v) in pattern.iter().enumerate() {
                if i < dim {
                    avg[i] += v;
                }
            }
        }

        let n = self.meta_reflections.len() as f64;
        for v in &mut avg {
            *v /= n;
        }

        avg
    }
}

impl Default for MetaMirror {
    fn default() -> Self {
        Self::new(5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mirror_reflection() {
        let original = vec![1.0, 0.5, 0.3, 0.8];
        let reflection = MirrorReflection::new(original.clone(), 0);

        assert_eq!(reflection.original.len(), 4);
        assert!(reflection.similarity() > 0.5);
    }

    #[test]
    fn test_simple_mirror() {
        let mut mirror = Mirror::new();

        let input = vec![0.5, 0.3, 0.8];
        let reflected = mirror.reflect(&input);

        assert_eq!(reflected.len(), 3);
        assert!(mirror.last_reflection().is_some());
    }

    #[test]
    fn test_recursive_mirror() {
        let mut mirror = RecursiveMirror::new(5);

        let input = vec![1.0, 0.5, 0.3, 0.8];
        let result = mirror.reflect(&input);

        assert_eq!(result.len(), 4);
        assert!(mirror.depth_reached() > 0);
    }

    #[test]
    fn test_similarity_decay() {
        let mut mirror = RecursiveMirror::new(5);

        let input = vec![1.0; 10];
        mirror.reflect(&input);

        let decay = mirror.similarity_decay();

        // Similarity should generally decrease with depth
        if decay.len() >= 2 {
            // First reflection should be more similar than last
            assert!(decay[0] >= decay[decay.len() - 1] * 0.5);
        }
    }

    #[test]
    fn test_meta_mirror() {
        let mut mirror = MetaMirror::new(4);

        let input = vec![0.5; 8];
        let result = mirror.reflect_and_observe(&input);

        assert_eq!(result.len(), 8);
        assert!(!mirror.meta_patterns().is_empty());
    }

    #[test]
    fn test_fidelity() {
        let mut mirror = RecursiveMirror::new(5);

        let input = vec![1.0; 5];
        mirror.reflect(&input);

        let fidelity = mirror.total_fidelity();
        assert!(fidelity > 0.0 && fidelity <= 1.0);
    }
}
