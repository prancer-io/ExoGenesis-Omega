//! Memory System - Self-contained hippocampal memory implementation

use crate::{BrainConfig, Result};
use crate::sleep_system::SleepOutput;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Memory entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub pattern: Vec<f64>,
    pub importance: f64,
}

/// Replay buffer
#[derive(Debug, Clone)]
pub struct ReplayBuffer {
    memories: Vec<Memory>,
    capacity: usize,
}

impl ReplayBuffer {
    pub fn new(capacity: usize) -> Self { Self { memories: Vec::with_capacity(capacity), capacity } }
    pub fn add(&mut self, pattern: Vec<f64>, importance: f64) {
        if self.memories.len() >= self.capacity {
            self.memories.sort_by(|a, b| a.importance.partial_cmp(&b.importance).unwrap());
            self.memories.remove(0);
        }
        self.memories.push(Memory { pattern, importance });
    }
    pub fn sample(&self, n: usize) -> Vec<Memory> {
        self.memories.iter().rev().take(n).cloned().collect()
    }
    pub fn clear(&mut self) { self.memories.clear(); }
}

/// Hippocampus - pattern separation and completion
#[derive(Debug, Clone)]
pub struct Hippocampus {
    patterns: HashMap<String, Memory>,
    dim: usize,
    threshold: f64,
    next_id: usize,
}

impl Hippocampus {
    pub fn new(dim: usize, _ca3_size: usize, threshold: f64) -> Self {
        Self { patterns: HashMap::new(), dim, threshold, next_id: 0 }
    }
    pub fn encode(&mut self, pattern: &[f64], importance: f64) -> String {
        let id = format!("mem_{}", self.next_id);
        self.next_id += 1;
        self.patterns.insert(id.clone(), Memory { pattern: pattern.to_vec(), importance });
        id
    }
    pub fn retrieve(&self, cue: &[f64]) -> Option<Memory> {
        let mut best: Option<(&Memory, f64)> = None;
        for mem in self.patterns.values() {
            let sim = cosine_similarity(cue, &mem.pattern);
            if sim > self.threshold {
                if best.is_none() || sim > best.unwrap().1 { best = Some((mem, sim)); }
            }
        }
        best.map(|(m, _)| m.clone())
    }
    pub fn strengthen(&mut self, pattern: &[f64], amount: f64) {
        for mem in self.patterns.values_mut() {
            let sim = cosine_similarity(pattern, &mem.pattern);
            if sim > 0.8 { mem.importance = (mem.importance + amount).min(1.0); }
        }
    }
    pub fn strengthen_association(&mut self, a: &[f64], b: &[f64], amount: f64) {
        self.strengthen(a, amount * 0.5);
        self.strengthen(b, amount * 0.5);
    }
    pub fn consolidate_all(&mut self) -> usize {
        let count = self.patterns.len();
        for mem in self.patterns.values_mut() { mem.importance = (mem.importance * 1.1).min(1.0); }
        count
    }
}

/// Cosine similarity (free function to avoid borrow conflicts)
fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    let mut dot = 0.0; let mut na = 0.0; let mut nb = 0.0;
    for (&x, &y) in a.iter().zip(b.iter()) { dot += x * y; na += x * x; nb += y * y; }
    let denom = (na * nb).sqrt();
    if denom > 0.0 { dot / denom } else { 0.0 }
}

/// Memory system
pub struct MemorySystem {
    hippocampus: Hippocampus,
    replay_buffer: ReplayBuffer,
    consolidation_count: usize,
    memory_count: usize,
    dim: usize,
}

impl MemorySystem {
    pub fn new(config: &BrainConfig) -> Self {
        Self {
            hippocampus: Hippocampus::new(config.pattern_dim, config.ca3_size, config.consolidation_threshold),
            replay_buffer: ReplayBuffer::new(config.replay_buffer_size),
            consolidation_count: 0,
            memory_count: 0,
            dim: config.pattern_dim,
        }
    }
    pub fn process(&mut self, content: &[f64]) -> Result<Vec<f64>> {
        let normalized: Vec<f64> = (0..self.dim).map(|i| content.get(i).copied().unwrap_or(0.0)).collect();
        if let Some(retrieved) = self.hippocampus.retrieve(&normalized) {
            Ok(normalized.iter().zip(retrieved.pattern.iter()).map(|(&a, &b)| 0.6 * a + 0.4 * b).collect())
        } else {
            let sig = normalized.iter().map(|x| x.abs()).sum::<f64>() / self.dim.max(1) as f64;
            if sig > 0.3 { self.encode(&normalized, sig)?; }
            Ok(normalized)
        }
    }
    pub fn encode(&mut self, content: &[f64], importance: f64) -> Result<()> {
        let normalized: Vec<f64> = (0..self.dim).map(|i| content.get(i).copied().unwrap_or(0.0)).collect();
        self.hippocampus.encode(&normalized, importance);
        self.memory_count += 1;
        self.replay_buffer.add(normalized, importance);
        Ok(())
    }
    pub fn retrieve(&self, cue: &[f64]) -> Result<Option<Vec<f64>>> {
        let normalized: Vec<f64> = (0..self.dim).map(|i| cue.get(i).copied().unwrap_or(0.0)).collect();
        Ok(self.hippocampus.retrieve(&normalized).map(|m| m.pattern))
    }
    pub fn consolidate_slow_wave(&mut self, output: &SleepOutput) -> Result<()> {
        for mem in self.replay_buffer.sample(output.replay_count) {
            self.hippocampus.strengthen(&mem.pattern, 0.1);
        }
        self.consolidation_count += 1;
        Ok(())
    }
    pub fn consolidate_rem(&mut self, output: &SleepOutput) -> Result<()> {
        let mems = self.replay_buffer.sample(output.replay_count);
        for i in 0..mems.len().saturating_sub(1) {
            self.hippocampus.strengthen_association(&mems[i].pattern, &mems[i + 1].pattern, 0.05);
        }
        Ok(())
    }
    pub fn force_consolidation(&mut self) -> Result<usize> {
        let count = self.hippocampus.consolidate_all();
        self.consolidation_count += 1;
        Ok(count)
    }
    pub fn consolidation_ratio(&self) -> f64 {
        if self.memory_count == 0 { 0.0 } else { self.consolidation_count as f64 / self.memory_count as f64 }
    }
    pub fn memory_count(&self) -> usize { self.memory_count }
    pub fn reset(&mut self) {
        self.hippocampus = Hippocampus::new(self.dim, 500, 0.7);
        self.replay_buffer.clear();
        self.consolidation_count = 0;
        self.memory_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hippocampus() {
        let mut hc = Hippocampus::new(8, 100, 0.5);
        hc.encode(&vec![0.5; 8], 1.0);
        assert!(hc.retrieve(&vec![0.5; 8]).is_some());
    }
}
