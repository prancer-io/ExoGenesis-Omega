//! Attention System - Self-contained attention mechanisms

use crate::{BrainConfig, Result};
use serde::{Deserialize, Serialize};

/// Attention mechanism types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttentionType { ScaledDotProduct, Flash, Linear, Sparse, Hyperbolic, Graph }

/// Working memory
#[derive(Debug, Clone)]
pub struct WorkingMemory {
    items: Vec<(Vec<f64>, f64)>,
    capacity: usize,
    dim: usize,
}

impl WorkingMemory {
    pub fn new(capacity: usize, dim: usize) -> Self {
        Self { items: Vec::with_capacity(capacity), capacity, dim }
    }
    pub fn dim(&self) -> usize { self.dim }
    pub fn store(&mut self, content: &[f64]) {
        if self.items.len() >= self.capacity {
            self.items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            self.items.remove(0);
        }
        self.items.push((content.to_vec(), 1.0));
    }
    pub fn gate(&self, input: &[f64]) -> Vec<f64> {
        let mut result = input.to_vec();
        for (item, strength) in &self.items {
            for (i, &v) in item.iter().enumerate() {
                if i < result.len() { result[i] += v * strength * 0.1; }
            }
        }
        result
    }
    pub fn contents(&self) -> Vec<Vec<f64>> { self.items.iter().map(|(v, _)| v.clone()).collect() }
    pub fn clear(&mut self) { self.items.clear(); }
}

/// Attention controller
#[derive(Debug, Clone)]
pub struct AttentionController {
    num_heads: usize,
    dim: usize,
    top_down: f64,
    bottom_up: f64,
    bias: Vec<f64>,
    mechanism: AttentionType,
}

impl AttentionController {
    pub fn new(num_heads: usize, dim: usize, top_down: f64, bottom_up: f64) -> Self {
        Self { num_heads, dim, top_down, bottom_up, bias: vec![0.0; dim], mechanism: AttentionType::ScaledDotProduct }
    }
    pub fn attend(&self, input: &[f64]) -> Vec<f64> {
        (0..self.dim).map(|i| {
            let v = input.get(i).copied().unwrap_or(0.0);
            let b = self.bias.get(i).copied().unwrap_or(0.0);
            (v * self.bottom_up + b * self.top_down).tanh()
        }).collect()
    }
    pub fn set_top_down_bias(&mut self, bias: &[f64]) {
        for (i, &b) in bias.iter().enumerate() { if i < self.dim { self.bias[i] = b; } }
    }
    pub fn set_mechanism(&mut self, m: AttentionType) { self.mechanism = m; }
    pub fn num_heads(&self) -> usize { self.num_heads }
    pub fn top_down_strength(&self) -> f64 { self.top_down }
    pub fn bottom_up_strength(&self) -> f64 { self.bottom_up }
}

/// Attention system
pub struct AttentionSystem {
    controller: AttentionController,
    working_memory: WorkingMemory,
    current_focus: Vec<f64>,
    dim: usize,
}

impl AttentionSystem {
    pub fn new(config: &BrainConfig) -> Self {
        Self {
            controller: AttentionController::new(config.attention_heads, config.attention_dim, config.top_down_strength, config.bottom_up_strength),
            working_memory: WorkingMemory::new(config.workspace_capacity, config.attention_dim),
            current_focus: vec![0.0; config.attention_dim],
            dim: config.attention_dim,
        }
    }
    pub fn attend(&mut self, input: &[f64]) -> Result<Vec<f64>> {
        let normalized: Vec<f64> = (0..self.dim).map(|i| input.get(i).copied().unwrap_or(0.0)).collect();
        let attended = self.controller.attend(&normalized);
        let gated = self.working_memory.gate(&attended);
        for (i, &v) in gated.iter().enumerate() {
            if i < self.current_focus.len() { self.current_focus[i] = 0.8 * self.current_focus[i] + 0.2 * v; }
        }
        Ok(gated)
    }
    pub fn focus_on(&mut self, target: &[f64]) -> Result<()> {
        let normalized: Vec<f64> = (0..self.dim).map(|i| target.get(i).copied().unwrap_or(0.0)).collect();
        self.controller.set_top_down_bias(&normalized);
        self.current_focus = normalized;
        Ok(())
    }
    pub fn current_focus(&self) -> Vec<f64> { self.current_focus.clone() }
    pub fn remember(&mut self, item: &[f64]) { self.working_memory.store(item); }
    pub fn working_memory_contents(&self) -> Vec<Vec<f64>> { self.working_memory.contents() }
    pub fn clear_working_memory(&mut self) { self.working_memory.clear(); }
    pub fn attention_strength(&self) -> f64 {
        let max = self.current_focus.iter().map(|x| x.abs()).fold(0.0, f64::max);
        let mean = self.current_focus.iter().map(|x| x.abs()).sum::<f64>() / self.current_focus.len().max(1) as f64;
        if mean > 0.0 { (max / mean).min(2.0) / 2.0 } else { 0.0 }
    }
    pub fn switch_mechanism(&mut self, m: AttentionType) { self.controller.set_mechanism(m); }
    pub fn reset(&mut self) {
        self.controller = AttentionController::new(self.controller.num_heads(), self.dim, self.controller.top_down_strength(), self.controller.bottom_up_strength());
        self.working_memory.clear();
        self.current_focus = vec![0.0; self.dim];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_working_memory() {
        let mut wm = WorkingMemory::new(5, 8);
        wm.store(&vec![0.5; 8]);
        assert_eq!(wm.contents().len(), 1);
    }
}
