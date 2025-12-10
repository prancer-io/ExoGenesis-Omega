//! Working Memory with Gating
//!
//! Implements capacity-limited working memory (7±2 items) with:
//! - Input gate: What enters WM
//! - Output gate: What influences processing
//! - Forget gate: What gets cleared
//! - Active maintenance via rehearsal

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// A single item in working memory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingMemoryItem {
    /// Unique ID
    pub id: String,
    /// Content vector
    pub content: Vec<f64>,
    /// Activation level (decays over time)
    pub activation: f64,
    /// Importance/priority score
    pub importance: f64,
    /// Time entered WM
    #[serde(skip)]
    pub entered_at: Option<Instant>,
    /// Number of times rehearsed
    pub rehearsal_count: u32,
    /// Associated context
    pub context: Option<Vec<f64>>,
}

impl WorkingMemoryItem {
    pub fn new(content: Vec<f64>, importance: f64) -> Self {
        Self {
            id: uuid::Uuid::now_v7().to_string(),
            content,
            activation: 1.0,
            importance,
            entered_at: Some(Instant::now()),
            rehearsal_count: 0,
            context: None,
        }
    }

    pub fn with_context(content: Vec<f64>, importance: f64, context: Vec<f64>) -> Self {
        let mut item = Self::new(content, importance);
        item.context = Some(context);
        item
    }

    /// Get age since entry
    pub fn age(&self) -> Duration {
        self.entered_at
            .map(|t| t.elapsed())
            .unwrap_or(Duration::ZERO)
    }

    /// Rehearse item (boosts activation)
    pub fn rehearse(&mut self) {
        self.activation = (self.activation + 0.3).min(1.0);
        self.rehearsal_count += 1;
    }

    /// Decay activation over time
    pub fn decay(&mut self, rate: f64) {
        self.activation *= 1.0 - rate;
    }
}

/// Working memory gate state
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WMGate {
    /// Gate openness (0.0 to 1.0)
    pub openness: f64,
    /// Threshold for passing
    pub threshold: f64,
}

impl Default for WMGate {
    fn default() -> Self {
        Self {
            openness: 0.5,
            threshold: 0.3,
        }
    }
}

impl WMGate {
    /// Check if item passes gate
    pub fn passes(&self, value: f64) -> bool {
        value * self.openness > self.threshold
    }

    /// Open gate more
    pub fn open(&mut self, amount: f64) {
        self.openness = (self.openness + amount).min(1.0);
    }

    /// Close gate more
    pub fn close(&mut self, amount: f64) {
        self.openness = (self.openness - amount).max(0.0);
    }
}

/// Working Memory system
pub struct WorkingMemory {
    /// Capacity (typically 7±2)
    capacity: usize,
    /// Items in WM
    items: VecDeque<WorkingMemoryItem>,
    /// Input gate
    pub input_gate: WMGate,
    /// Output gate
    pub output_gate: WMGate,
    /// Forget gate
    pub forget_gate: WMGate,
    /// Decay rate per update
    decay_rate: f64,
    /// Minimum activation to retain
    min_activation: f64,
}

impl WorkingMemory {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            items: VecDeque::with_capacity(capacity),
            input_gate: WMGate::default(),
            output_gate: WMGate::default(),
            forget_gate: WMGate {
                openness: 0.3,
                threshold: 0.2,
            },
            decay_rate: 0.05,
            min_activation: 0.1,
        }
    }

    /// Try to store item (may fail if full and not important enough)
    pub fn try_store(&mut self, item: WorkingMemoryItem) -> bool {
        // Check input gate
        if !self.input_gate.passes(item.importance) {
            return false;
        }

        // If full, try to replace lowest activation item
        if self.items.len() >= self.capacity {
            let min_idx = self
                .items
                .iter()
                .enumerate()
                .min_by(|a, b| a.1.activation.partial_cmp(&b.1.activation).unwrap())
                .map(|(i, _)| i);

            if let Some(idx) = min_idx {
                if self.items[idx].activation < item.importance {
                    self.items.remove(idx);
                } else {
                    return false; // Can't replace, all items more important
                }
            }
        }

        self.items.push_back(item);
        true
    }

    /// Force store (always succeeds, may evict)
    pub fn store(&mut self, item: WorkingMemoryItem) {
        if self.items.len() >= self.capacity {
            // Evict oldest/lowest activation
            let min_idx = self
                .items
                .iter()
                .enumerate()
                .min_by(|a, b| {
                    let score_a = a.1.activation * 0.5 + (1.0 / (a.1.age().as_secs_f64() + 1.0)) * 0.5;
                    let score_b = b.1.activation * 0.5 + (1.0 / (b.1.age().as_secs_f64() + 1.0)) * 0.5;
                    score_a.partial_cmp(&score_b).unwrap()
                })
                .map(|(i, _)| i)
                .unwrap_or(0);

            self.items.remove(min_idx);
        }

        self.items.push_back(item);
    }

    /// Get item by ID
    pub fn get(&self, id: &str) -> Option<&WorkingMemoryItem> {
        self.items.iter().find(|item| item.id == id)
    }

    /// Get mutable item by ID
    pub fn get_mut(&mut self, id: &str) -> Option<&mut WorkingMemoryItem> {
        self.items.iter_mut().find(|item| item.id == id)
    }

    /// Retrieve item content (applies output gate)
    pub fn retrieve(&self, id: &str) -> Option<Vec<f64>> {
        self.items
            .iter()
            .find(|item| item.id == id && self.output_gate.passes(item.activation))
            .map(|item| item.content.clone())
    }

    /// Get all items above output gate threshold
    pub fn active_items(&self) -> Vec<&WorkingMemoryItem> {
        self.items
            .iter()
            .filter(|item| self.output_gate.passes(item.activation))
            .collect()
    }

    /// Update WM state (decay, forget)
    pub fn update(&mut self) {
        // Decay all items
        for item in &mut self.items {
            item.decay(self.decay_rate);
        }

        // Apply forget gate
        let forget_threshold = self.forget_gate.openness * self.min_activation;
        self.items.retain(|item| {
            item.activation >= forget_threshold || item.importance > 0.8
        });
    }

    /// Rehearse specific item
    pub fn rehearse(&mut self, id: &str) {
        if let Some(item) = self.get_mut(id) {
            item.rehearse();
        }
    }

    /// Rehearse all items (maintenance)
    pub fn rehearse_all(&mut self) {
        for item in &mut self.items {
            item.rehearse();
        }
    }

    /// Clear all items
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Get current number of items
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Check if full
    pub fn is_full(&self) -> bool {
        self.items.len() >= self.capacity
    }

    /// Get capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get all items
    pub fn items(&self) -> &VecDeque<WorkingMemoryItem> {
        &self.items
    }

    /// Find items similar to query
    pub fn find_similar(&self, query: &[f64], threshold: f64) -> Vec<&WorkingMemoryItem> {
        self.items
            .iter()
            .filter(|item| {
                let sim = Self::cosine_similarity(&item.content, query);
                sim > threshold
            })
            .collect()
    }

    /// Get most activated item
    pub fn most_active(&self) -> Option<&WorkingMemoryItem> {
        self.items
            .iter()
            .max_by(|a, b| a.activation.partial_cmp(&b.activation).unwrap())
    }

    /// Helper: cosine similarity
    fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
        let mut dot = 0.0;
        let mut norm_a = 0.0;
        let mut norm_b = 0.0;

        for (&x, &y) in a.iter().zip(b.iter()) {
            dot += x * y;
            norm_a += x * x;
            norm_b += y * y;
        }

        norm_a = norm_a.sqrt();
        norm_b = norm_b.sqrt();

        if norm_a > 0.0 && norm_b > 0.0 {
            dot / (norm_a * norm_b)
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_working_memory_creation() {
        let wm = WorkingMemory::new(7);
        assert_eq!(wm.capacity(), 7);
        assert!(wm.is_empty());
    }

    #[test]
    fn test_store_and_retrieve() {
        let mut wm = WorkingMemory::new(7);

        let item = WorkingMemoryItem::new(vec![1.0, 2.0, 3.0], 0.8);
        let id = item.id.clone();

        wm.store(item);

        assert_eq!(wm.len(), 1);
        assert!(wm.get(&id).is_some());
    }

    #[test]
    fn test_capacity_limit() {
        let mut wm = WorkingMemory::new(3);

        for i in 0..5 {
            let item = WorkingMemoryItem::new(vec![i as f64], 0.5);
            wm.store(item);
        }

        assert_eq!(wm.len(), 3); // Capped at capacity
    }

    #[test]
    fn test_input_gate() {
        let mut wm = WorkingMemory::new(7);
        wm.input_gate.openness = 1.0;  // Full openness so value * 1.0 = value
        wm.input_gate.threshold = 0.5;

        // Low importance - should fail (0.3 * 1.0 = 0.3 < 0.5)
        let item1 = WorkingMemoryItem::new(vec![1.0], 0.3);
        assert!(!wm.try_store(item1));

        // High importance - should succeed (0.8 * 1.0 = 0.8 > 0.5)
        let item2 = WorkingMemoryItem::new(vec![2.0], 0.8);
        assert!(wm.try_store(item2));
    }

    #[test]
    fn test_decay() {
        let mut wm = WorkingMemory::new(7);

        let item = WorkingMemoryItem::new(vec![1.0], 0.8);
        let id = item.id.clone();
        wm.store(item);

        let initial_activation = wm.get(&id).unwrap().activation;

        wm.update();

        let new_activation = wm.get(&id).unwrap().activation;
        assert!(new_activation < initial_activation);
    }

    #[test]
    fn test_rehearsal() {
        let mut wm = WorkingMemory::new(7);

        let item = WorkingMemoryItem::new(vec![1.0], 0.5);
        let id = item.id.clone();
        wm.store(item);

        // Decay
        for _ in 0..5 {
            wm.update();
        }

        let before_rehearsal = wm.get(&id).unwrap().activation;
        wm.rehearse(&id);
        let after_rehearsal = wm.get(&id).unwrap().activation;

        assert!(after_rehearsal > before_rehearsal);
    }

    #[test]
    fn test_find_similar() {
        let mut wm = WorkingMemory::new(7);

        wm.store(WorkingMemoryItem::new(vec![1.0, 0.0, 0.0], 0.8));
        wm.store(WorkingMemoryItem::new(vec![0.0, 1.0, 0.0], 0.8));
        wm.store(WorkingMemoryItem::new(vec![0.9, 0.1, 0.0], 0.8));

        let similar = wm.find_similar(&[1.0, 0.0, 0.0], 0.8);
        assert!(similar.len() >= 1); // At least the exact match
    }
}
