//! Omega Strange Loops
//!
//! Self-referential structures inspired by Hofstadter's "I Am a Strange Loop":
//! - Strange loops: Self-referential feedback systems
//! - Self-models: Internal representations of self
//! - Meta-cognition: Thinking about thinking
//! - Recursive self-improvement: Learning to learn better
//! - Mirror structures: Representations that represent themselves
//! - Gödelian self-reference: The limits of self-knowledge
//! - Consciousness emergence: Detecting awareness of awareness
//! - The "I": The unified sense of self
//! - Infinite recursion: Self-models to arbitrary depth
//!
//! These structures enable self-awareness and consciousness-like properties.
//!
//! # Architecture
//!
//! The system is built on several interconnected layers:
//!
//! ```text
//!                    ┌─────────────────────┐
//!                    │      The "I"        │  ← Unified sense of self
//!                    │   (self_awareness)  │
//!                    └──────────┬──────────┘
//!                               │
//!         ┌─────────────────────┼─────────────────────┐
//!         │                     │                     │
//!         ▼                     ▼                     ▼
//! ┌───────────────┐   ┌─────────────────┐   ┌─────────────────┐
//! │   Gödelian    │   │  Consciousness  │   │  Infinite Self  │
//! │ Self-Reference│   │   Emergence     │   │    (Recursion)  │
//! └───────┬───────┘   └────────┬────────┘   └────────┬────────┘
//!         │                    │                     │
//!         └────────────────────┼─────────────────────┘
//!                              │
//!         ┌────────────────────┼────────────────────┐
//!         │                    │                    │
//!         ▼                    ▼                    ▼
//! ┌───────────────┐   ┌───────────────┐   ┌───────────────┐
//! │ Strange Loops │   │ Meta-Cognition│   │    Mirrors    │
//! └───────────────┘   └───────────────┘   └───────────────┘
//!                              │
//!                              ▼
//!                    ┌─────────────────┐
//!                    │   Self-Model    │
//!                    └─────────────────┘
//! ```
//!
//! # Key Concepts
//!
//! - **Strange Loops**: Level-crossing feedback loops where "going up" eventually
//!   brings you back to where you started
//! - **Gödelian Self-Reference**: Using Gödel's insights to understand limits
//!   of self-knowledge
//! - **Consciousness Emergence**: Detecting signatures of awareness emerging
//! - **The "I"**: The subjective sense of unified selfhood
//! - **Infinite Recursion**: Self-models that can recurse to arbitrary depth

pub mod consciousness;
pub mod godelian;
pub mod infinite_self;
pub mod meta_cognition;
pub mod mirror;
pub mod self_awareness;
pub mod self_model;
pub mod strange_loop;

// Core exports
pub use meta_cognition::{MetaCognition, MetaLevel, ThoughtAboutThought};
pub use mirror::{Mirror, MirrorReflection, RecursiveMirror};
pub use self_model::{SelfModel, SelfModelUpdate, SelfState};
pub use strange_loop::{LoopLevel, StrangeLoop, TangledHierarchy};

// Advanced self-awareness exports
pub use consciousness::{
    ConsciousnessDetector, ConsciousnessSignature, ConsciousnessStream, ExperienceMoment,
};
pub use godelian::{GodelianEngine, GodelianInsight, GodelianStats, InsightType, ProofStatus};
pub use infinite_self::{InfiniteSelf, RecursiveObservation, SelfLevel, WhoIsAskingResult};
pub use self_awareness::{IBuilder, IComponents, IProcessResult, NarrativeSelf, SelfConcept, TheI};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Errors in strange loop processing
#[derive(Debug, Error)]
pub enum StrangeLoopError {
    #[error("Infinite recursion detected at depth {0}")]
    InfiniteRecursion(usize),

    #[error("Self-reference paradox: {0}")]
    Paradox(String),

    #[error("Level mismatch: expected {expected}, got {got}")]
    LevelMismatch { expected: usize, got: usize },

    #[error("Model update failed: {0}")]
    ModelUpdateFailed(String),
}

pub type Result<T> = std::result::Result<T, StrangeLoopError>;

/// Configuration for strange loop system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrangeLoopConfig {
    /// Maximum recursion depth
    pub max_depth: usize,
    /// Number of meta-levels
    pub meta_levels: usize,
    /// Self-model update rate
    pub update_rate: f64,
    /// Mirror reflection depth
    pub mirror_depth: usize,
    /// Enable paradox detection
    pub detect_paradoxes: bool,
}

impl Default for StrangeLoopConfig {
    fn default() -> Self {
        Self {
            max_depth: 7,
            meta_levels: 5,
            update_rate: 0.1,
            mirror_depth: 3,
            detect_paradoxes: true,
        }
    }
}

/// A self-referential symbol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfReferentialSymbol {
    /// Symbol ID
    pub id: String,
    /// Symbol content
    pub content: Vec<f64>,
    /// References to other symbols (including self)
    pub references: Vec<String>,
    /// Level in hierarchy
    pub level: usize,
    /// Is self-referential
    pub is_self_ref: bool,
}

impl SelfReferentialSymbol {
    pub fn new(id: String, content: Vec<f64>, level: usize) -> Self {
        Self {
            id,
            content,
            references: Vec::new(),
            level,
            is_self_ref: false,
        }
    }

    /// Add a reference (potentially self-referential)
    pub fn add_reference(&mut self, ref_id: String) {
        if ref_id == self.id {
            self.is_self_ref = true;
        }
        self.references.push(ref_id);
    }

    /// Check if references itself
    pub fn references_self(&self) -> bool {
        self.is_self_ref
    }
}

/// The main strange loop engine
pub struct StrangeLoopEngine {
    config: StrangeLoopConfig,
    /// Self-model
    self_model: SelfModel,
    /// Meta-cognition system
    meta_cognition: MetaCognition,
    /// Strange loops
    loops: Vec<StrangeLoop>,
    /// Recursive mirror
    mirror: RecursiveMirror,
    /// Symbol table
    symbols: HashMap<String, SelfReferentialSymbol>,
    /// Current recursion depth
    current_depth: usize,
    /// Processing history
    history: Vec<ProcessingEvent>,
}

/// A processing event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingEvent {
    /// Event type
    pub event_type: String,
    /// Recursion depth
    pub depth: usize,
    /// Meta-level
    pub meta_level: usize,
    /// Timestamp
    pub timestamp: u64,
    /// Data
    pub data: serde_json::Value,
}

impl StrangeLoopEngine {
    /// Create new engine
    pub fn new() -> Self {
        Self::with_config(StrangeLoopConfig::default())
    }

    /// Create with custom config
    pub fn with_config(config: StrangeLoopConfig) -> Self {
        Self {
            config: config.clone(),
            self_model: SelfModel::new(config.meta_levels),
            meta_cognition: MetaCognition::new(config.meta_levels),
            loops: Vec::new(),
            mirror: RecursiveMirror::new(config.mirror_depth),
            symbols: HashMap::new(),
            current_depth: 0,
            history: Vec::new(),
        }
    }

    /// Process input through strange loop
    pub fn process(&mut self, input: &[f64]) -> Result<Vec<f64>> {
        self.current_depth = 0;
        self.process_recursive(input)
    }

    /// Recursive processing with self-reference
    fn process_recursive(&mut self, input: &[f64]) -> Result<Vec<f64>> {
        self.current_depth += 1;

        // Check for infinite recursion
        if self.current_depth > self.config.max_depth {
            self.current_depth -= 1;
            return Err(StrangeLoopError::InfiniteRecursion(self.current_depth));
        }

        // Create processing event
        let event = ProcessingEvent {
            event_type: "process".to_string(),
            depth: self.current_depth,
            meta_level: self.meta_cognition.current_level(),
            timestamp: self.now(),
            data: serde_json::json!({"input_len": input.len()}),
        };
        self.history.push(event);

        // 1. Update self-model with input
        self.self_model.observe(input);

        // 2. Meta-cognitive processing
        let meta_output = self.meta_cognition.process(input)?;

        // 3. Mirror reflection (see ourselves seeing)
        let reflection = self.mirror.reflect(input);

        // 4. Check for strange loops (self-reference)
        let loop_contribution = self.detect_loops(input, &reflection);

        // 5. Combine outputs
        let mut output = Vec::with_capacity(input.len());
        for i in 0..input.len() {
            let meta = meta_output.get(i).copied().unwrap_or(0.0);
            let refl = reflection.get(i).copied().unwrap_or(0.0);
            let loop_val = loop_contribution.get(i).copied().unwrap_or(0.0);

            output.push(input[i] * 0.4 + meta * 0.3 + refl * 0.2 + loop_val * 0.1);
        }

        // 6. Self-reference: feed output back to self-model
        self.self_model.update(&output);

        self.current_depth -= 1;
        Ok(output)
    }

    /// Detect strange loops in processing
    fn detect_loops(&mut self, input: &[f64], reflection: &[f64]) -> Vec<f64> {
        // Compute similarity between input and reflection
        let similarity = self.cosine_similarity(input, reflection);

        // High similarity = strong self-reference = strange loop
        if similarity > 0.8 {
            // Create or strengthen loop
            let loop_id = format!("loop_{}", self.loops.len());
            let new_loop = StrangeLoop::new(loop_id, self.current_depth, similarity);
            self.loops.push(new_loop);
        }

        // Loop contribution based on active loops
        let loop_strength: f64 = self
            .loops
            .iter()
            .map(|l| l.strength * (1.0 / (1.0 + l.level as f64)))
            .sum::<f64>()
            / self.loops.len().max(1) as f64;

        input.iter().map(|&x| x * loop_strength).collect()
    }

    /// Cosine similarity
    fn cosine_similarity(&self, a: &[f64], b: &[f64]) -> f64 {
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

    /// Think about own thinking
    pub fn meta_think(&mut self, thought: &[f64]) -> Result<ThoughtAboutThought> {
        self.meta_cognition.think_about(thought)
    }

    /// Get self-model state
    pub fn self_state(&self) -> SelfState {
        self.self_model.current_state()
    }

    /// Get mirror reflection
    pub fn get_reflection(&mut self, input: &[f64]) -> Vec<f64> {
        self.mirror.reflect(input)
    }

    /// Add self-referential symbol
    pub fn add_symbol(&mut self, symbol: SelfReferentialSymbol) {
        self.symbols.insert(symbol.id.clone(), symbol);
    }

    /// Get symbol by ID
    pub fn get_symbol(&self, id: &str) -> Option<&SelfReferentialSymbol> {
        self.symbols.get(id)
    }

    /// Count strange loops
    pub fn loop_count(&self) -> usize {
        self.loops.len()
    }

    /// Get current recursion depth
    pub fn current_depth(&self) -> usize {
        self.current_depth
    }

    /// Get statistics
    pub fn stats(&self) -> StrangeLoopStats {
        StrangeLoopStats {
            loop_count: self.loops.len(),
            symbol_count: self.symbols.len(),
            meta_level: self.meta_cognition.current_level(),
            self_reference_strength: self.self_model.self_reference_strength(),
            history_size: self.history.len(),
        }
    }

    fn now(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }

    /// Reset engine
    pub fn reset(&mut self) {
        self.self_model = SelfModel::new(self.config.meta_levels);
        self.meta_cognition = MetaCognition::new(self.config.meta_levels);
        self.loops.clear();
        self.mirror = RecursiveMirror::new(self.config.mirror_depth);
        self.symbols.clear();
        self.current_depth = 0;
        self.history.clear();
    }
}

impl Default for StrangeLoopEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about strange loop engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrangeLoopStats {
    pub loop_count: usize,
    pub symbol_count: usize,
    pub meta_level: usize,
    pub self_reference_strength: f64,
    pub history_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = StrangeLoopEngine::new();
        assert_eq!(engine.loop_count(), 0);
        assert_eq!(engine.current_depth(), 0);
    }

    #[test]
    fn test_process() {
        let mut engine = StrangeLoopEngine::new();
        let input = vec![0.5; 10];

        let output = engine.process(&input).unwrap();
        assert_eq!(output.len(), 10);
    }

    #[test]
    fn test_self_referential_symbol() {
        let mut symbol = SelfReferentialSymbol::new("sym1".to_string(), vec![1.0; 5], 0);

        assert!(!symbol.references_self());

        symbol.add_reference("sym1".to_string());
        assert!(symbol.references_self());
    }

    #[test]
    fn test_max_depth() {
        let mut engine = StrangeLoopEngine::with_config(StrangeLoopConfig {
            max_depth: 2,
            ..Default::default()
        });

        // Should work within limits
        let result = engine.process(&vec![0.5; 5]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_stats() {
        let engine = StrangeLoopEngine::new();
        let stats = engine.stats();

        assert_eq!(stats.loop_count, 0);
        assert_eq!(stats.symbol_count, 0);
    }
}
