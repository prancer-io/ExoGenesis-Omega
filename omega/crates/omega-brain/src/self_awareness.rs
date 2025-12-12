//! Self-Awareness System - Self-contained strange loops and meta-cognition

use crate::{BrainConfig, Result};
use serde::{Deserialize, Serialize};

/// Strange loop config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrangeLoopConfig {
    pub max_depth: usize,
    pub meta_levels: usize,
    pub update_rate: f64,
    pub mirror_depth: usize,
    pub detect_paradoxes: bool,
}

/// Self state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfState {
    pub state: Vec<f64>,
}

/// Thought about thought
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtAboutThought {
    pub meta: Vec<f64>,
}

/// Self model
#[derive(Debug, Clone)]
pub struct SelfModel {
    state: Vec<f64>,
    observations: Vec<Vec<f64>>,
    confidence: f64,
    self_ref_strength: f64,
}

impl SelfModel {
    pub fn new(dim: usize) -> Self {
        Self { state: vec![0.0; dim], observations: Vec::new(), confidence: 0.5, self_ref_strength: 0.0 }
    }
    pub fn observe(&mut self, input: &[f64]) {
        self.observations.push(input.to_vec());
        if self.observations.len() > 100 { self.observations.remove(0); }
    }
    pub fn update(&mut self, output: &[f64]) {
        for (i, &v) in output.iter().enumerate() {
            if i < self.state.len() { self.state[i] = 0.9 * self.state[i] + 0.1 * v; }
        }
        if !self.observations.is_empty() {
            let last = &self.observations[self.observations.len() - 1];
            let sim = self.cosine_similarity(&self.state, last);
            self.self_ref_strength = 0.9 * self.self_ref_strength + 0.1 * sim;
            self.confidence = 0.9 * self.confidence + 0.1 * sim.abs();
        }
    }
    pub fn current_state(&self) -> SelfState { SelfState { state: self.state.clone() } }
    pub fn self_reference_strength(&self) -> f64 { self.self_ref_strength }
    pub fn confidence(&self) -> f64 { self.confidence }
    pub fn reset(&mut self) {
        self.state.fill(0.0);
        self.observations.clear();
        self.confidence = 0.5;
        self.self_ref_strength = 0.0;
    }
    fn cosine_similarity(&self, a: &[f64], b: &[f64]) -> f64 {
        let mut dot = 0.0; let mut na = 0.0; let mut nb = 0.0;
        for (&x, &y) in a.iter().zip(b.iter()) { dot += x * y; na += x * x; nb += y * y; }
        let denom = (na * nb).sqrt();
        if denom > 0.0 { dot / denom } else { 0.0 }
    }
}

/// Meta-cognition
#[derive(Debug, Clone)]
pub struct MetaCognition {
    levels: usize,
    current_level: usize,
    quality: f64,
}

impl MetaCognition {
    pub fn new(levels: usize) -> Self { Self { levels, current_level: 0, quality: 0.5 } }
    pub fn think_about(&mut self, thought: &[f64]) -> std::result::Result<ThoughtAboutThought, String> {
        self.current_level = (self.current_level + 1) % self.levels;
        let decay = 1.0 / (self.current_level + 1) as f64;
        let meta: Vec<f64> = thought.iter().map(|&x| x * decay).collect();
        self.quality = meta.iter().map(|x| x.abs()).sum::<f64>() / meta.len().max(1) as f64;
        Ok(ThoughtAboutThought { meta })
    }
    pub fn cognitive_quality(&self) -> f64 { self.quality }
    pub fn current_level(&self) -> usize { self.current_level }
    pub fn reset(&mut self) { self.current_level = 0; self.quality = 0.5; }
}

/// Strange loop engine
#[derive(Debug, Clone)]
pub struct StrangeLoopEngine {
    config: StrangeLoopConfig,
    loop_count: usize,
    dim: usize,
}

impl StrangeLoopEngine {
    pub fn with_config(config: StrangeLoopConfig) -> Self {
        Self { dim: 64, loop_count: 0, config }
    }
    pub fn process(&mut self, input: &[f64]) -> std::result::Result<Vec<f64>, String> {
        self.loop_count += 1;
        // Use dim to limit output size if needed
        let output: Vec<f64> = input.iter().take(self.dim).enumerate()
            .map(|(i, &x)| x * (1.0 / (1 + i % self.config.max_depth) as f64))
            .collect();
        Ok(output)
    }
    pub fn loop_count(&self) -> usize { self.loop_count }
    pub fn dim(&self) -> usize { self.dim }
    pub fn reset(&mut self) { self.loop_count = 0; }
}

/// Self-awareness system
pub struct SelfAwarenessSystem {
    engine: StrangeLoopEngine,
    self_model: SelfModel,
    meta_cognition: MetaCognition,
    loop_count: usize,
    self_ref_strength: f64,
    dim: usize,
}

impl SelfAwarenessSystem {
    pub fn new(config: &BrainConfig) -> Self {
        let sl_config = StrangeLoopConfig {
            max_depth: config.max_recursion,
            meta_levels: config.meta_levels,
            update_rate: config.self_update_rate,
            mirror_depth: config.mirror_depth,
            detect_paradoxes: true,
        };
        Self {
            engine: StrangeLoopEngine::with_config(sl_config),
            self_model: SelfModel::new(config.attention_dim),
            meta_cognition: MetaCognition::new(config.meta_levels),
            loop_count: 0,
            self_ref_strength: 0.0,
            dim: config.attention_dim,
        }
    }
    pub fn reflect(&mut self, input: &[f64]) -> Result<Vec<f64>> {
        let normalized: Vec<f64> = (0..self.dim).map(|i| input.get(i).copied().unwrap_or(0.0)).collect();
        let output = self.engine.process(&normalized).map_err(crate::BrainError::SelfModelError)?;
        self.self_model.observe(&normalized);
        self.self_model.update(&output);
        self.self_ref_strength = self.self_model.self_reference_strength();
        self.loop_count = self.engine.loop_count();
        Ok(output)
    }
    pub fn think_about(&mut self, thought: &[f64]) -> Result<Vec<f64>> {
        let normalized: Vec<f64> = (0..self.dim).map(|i| thought.get(i).copied().unwrap_or(0.0)).collect();
        let tat = self.meta_cognition.think_about(&normalized).map_err(crate::BrainError::SelfModelError)?;
        let output = self.engine.process(&tat.meta).map_err(crate::BrainError::SelfModelError)?;
        Ok(output)
    }
    pub fn current_self_state(&self) -> Vec<f64> { self.self_model.current_state().state }
    pub fn self_reference_strength(&self) -> f64 { self.self_ref_strength }
    pub fn loop_count(&self) -> usize { self.loop_count }
    pub fn confidence(&self) -> f64 { self.self_model.confidence() }
    pub fn cognitive_quality(&self) -> f64 { self.meta_cognition.cognitive_quality() }
    pub fn has_paradox(&self) -> bool { self.self_ref_strength > 0.95 && self.loop_count > 10 }
    pub fn meta_level_count(&self) -> usize { self.meta_cognition.current_level() }
    pub fn reset(&mut self) {
        self.engine.reset();
        self.self_model.reset();
        self.meta_cognition.reset();
        self.loop_count = 0;
        self.self_ref_strength = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_self_model() {
        let mut sm = SelfModel::new(8);
        sm.observe(&vec![0.5; 8]);
        sm.update(&vec![0.3; 8]);
        assert!(sm.confidence() >= 0.0);
    }
}
