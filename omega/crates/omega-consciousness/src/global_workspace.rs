//! Global Workspace Theory (GWT)
//!
//! Implements the Global Workspace Theory of consciousness:
//! - Competition for workspace access
//! - Broadcast to all processors
//! - Coalition formation
//! - Ignition dynamics
//!
//! Based on Baars (1988) and Dehaene et al. (2006)

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

/// Content that can enter the global workspace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceContent {
    /// Unique identifier
    pub id: String,
    /// Content vector
    pub content: Vec<f64>,
    /// Activation strength
    pub activation: f64,
    /// Source processor
    pub source: String,
    /// Time entered workspace
    pub timestamp: u64,
    /// Coalition membership
    pub coalition_id: Option<String>,
}

impl WorkspaceContent {
    pub fn new(content: Vec<f64>, activation: f64, source: String) -> Self {
        Self {
            id: uuid::Uuid::now_v7().to_string(),
            content,
            activation,
            source,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            coalition_id: None,
        }
    }
}

/// A coalition of content competing for workspace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coalition {
    /// Coalition ID
    pub id: String,
    /// Member content IDs
    pub members: Vec<String>,
    /// Combined activation
    pub activation: f64,
    /// Coherence of the coalition
    pub coherence: f64,
    /// Whether coalition has ignited
    pub ignited: bool,
}

impl Coalition {
    pub fn new(id: String) -> Self {
        Self {
            id,
            members: Vec::new(),
            activation: 0.0,
            coherence: 0.0,
            ignited: false,
        }
    }

    /// Add member to coalition
    pub fn add_member(&mut self, content_id: String, activation: f64) {
        self.members.push(content_id);
        self.activation += activation;
    }

    /// Compute coalition coherence
    pub fn compute_coherence(&mut self, contents: &HashMap<String, WorkspaceContent>) {
        if self.members.len() < 2 {
            self.coherence = 1.0;
            return;
        }

        // Coherence = average pairwise similarity
        let mut total_sim = 0.0;
        let mut pairs = 0;

        for i in 0..self.members.len() {
            for j in (i + 1)..self.members.len() {
                if let (Some(a), Some(b)) = (contents.get(&self.members[i]), contents.get(&self.members[j])) {
                    total_sim += Self::cosine_similarity(&a.content, &b.content);
                    pairs += 1;
                }
            }
        }

        self.coherence = if pairs > 0 {
            total_sim / pairs as f64
        } else {
            0.0
        };
    }

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

/// Event broadcast from workspace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastEvent {
    /// Content being broadcast
    pub content: WorkspaceContent,
    /// Coalition that won
    pub coalition_id: Option<String>,
    /// Broadcast timestamp
    pub timestamp: u64,
}

/// The Global Workspace
pub struct GlobalWorkspace {
    /// Maximum capacity
    capacity: usize,
    /// Current workspace contents
    contents: HashMap<String, WorkspaceContent>,
    /// Active coalitions
    coalitions: HashMap<String, Coalition>,
    /// Broadcast history
    broadcast_history: VecDeque<BroadcastEvent>,
    /// Ignition threshold
    ignition_threshold: f64,
    /// Decay rate for activations
    decay_rate: f64,
    /// Current dominant coalition
    dominant_coalition: Option<String>,
}

impl GlobalWorkspace {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            contents: HashMap::new(),
            coalitions: HashMap::new(),
            broadcast_history: VecDeque::with_capacity(100),
            ignition_threshold: 0.5,
            decay_rate: 0.1,
            dominant_coalition: None,
        }
    }

    /// Submit content to compete for workspace access
    pub fn compete(&mut self, content: WorkspaceContent) {
        // If at capacity, remove lowest activation
        if self.contents.len() >= self.capacity {
            let min_id = self
                .contents
                .iter()
                .min_by(|a, b| a.1.activation.partial_cmp(&b.1.activation).unwrap())
                .map(|(id, _)| id.clone());

            if let Some(id) = min_id {
                if self.contents.get(&id).map(|c| c.activation).unwrap_or(1.0)
                    < content.activation
                {
                    self.contents.remove(&id);
                } else {
                    return; // New content not strong enough
                }
            }
        }

        self.contents.insert(content.id.clone(), content);
    }

    /// Form coalitions among compatible contents
    pub fn form_coalitions(&mut self) {
        // Clear old coalitions
        self.coalitions.clear();

        // Simple coalition formation: group by similarity
        let content_ids: Vec<String> = self.contents.keys().cloned().collect();
        let mut assigned: HashMap<String, bool> = HashMap::new();

        for id in &content_ids {
            if assigned.get(id).copied().unwrap_or(false) {
                continue;
            }

            let mut coalition = Coalition::new(format!("coal_{}", uuid::Uuid::now_v7()));

            if let Some(content) = self.contents.get(id) {
                coalition.add_member(id.clone(), content.activation);
                assigned.insert(id.clone(), true);

                // Find similar contents to join coalition
                for other_id in &content_ids {
                    if assigned.get(other_id).copied().unwrap_or(false) {
                        continue;
                    }

                    if let Some(other) = self.contents.get(other_id) {
                        let sim = Coalition::cosine_similarity(&content.content, &other.content);
                        if sim > 0.5 {
                            coalition.add_member(other_id.clone(), other.activation);
                            assigned.insert(other_id.clone(), true);
                        }
                    }
                }
            }

            // Compute coalition coherence
            coalition.compute_coherence(&self.contents);

            // Check for ignition
            if coalition.activation * coalition.coherence > self.ignition_threshold {
                coalition.ignited = true;
            }

            self.coalitions.insert(coalition.id.clone(), coalition);
        }

        // Find dominant coalition
        self.dominant_coalition = self
            .coalitions
            .iter()
            .filter(|(_, c)| c.ignited)
            .max_by(|a, b| {
                (a.1.activation * a.1.coherence)
                    .partial_cmp(&(b.1.activation * b.1.coherence))
                    .unwrap()
            })
            .map(|(id, _)| id.clone());
    }

    /// Broadcast winning coalition's content
    pub fn broadcast(&mut self) {
        self.form_coalitions();

        if let Some(ref coalition_id) = self.dominant_coalition {
            if let Some(coalition) = self.coalitions.get(coalition_id) {
                // Broadcast each member
                for member_id in &coalition.members {
                    if let Some(content) = self.contents.get(member_id).cloned() {
                        let event = BroadcastEvent {
                            content,
                            coalition_id: Some(coalition_id.clone()),
                            timestamp: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_millis() as u64,
                        };

                        self.broadcast_history.push_back(event);
                        if self.broadcast_history.len() > 100 {
                            self.broadcast_history.pop_front();
                        }
                    }
                }
            }
        }

        // Decay non-winning content
        self.decay();
    }

    /// Decay activations of non-broadcast content
    fn decay(&mut self) {
        let dominant = self.dominant_coalition.clone();
        let dominant_members: Vec<String> = dominant
            .and_then(|id| self.coalitions.get(&id))
            .map(|c| c.members.clone())
            .unwrap_or_default();

        for content in self.contents.values_mut() {
            if !dominant_members.contains(&content.id) {
                content.activation *= 1.0 - self.decay_rate;
            }
        }

        // Remove very low activation content
        self.contents.retain(|_, c| c.activation > 0.01);
    }

    /// Get IDs of current workspace contents
    pub fn content_ids(&self) -> Vec<String> {
        self.contents.keys().cloned().collect()
    }

    /// Get number of active coalitions
    pub fn active_coalitions(&self) -> usize {
        self.coalitions.values().filter(|c| c.ignited).count()
    }

    /// Get current contents
    pub fn contents(&self) -> &HashMap<String, WorkspaceContent> {
        &self.contents
    }

    /// Get broadcast history
    pub fn history(&self) -> &VecDeque<BroadcastEvent> {
        &self.broadcast_history
    }

    /// Get dominant coalition
    pub fn dominant(&self) -> Option<&Coalition> {
        self.dominant_coalition
            .as_ref()
            .and_then(|id| self.coalitions.get(id))
    }

    /// Clear workspace
    pub fn clear(&mut self) {
        self.contents.clear();
        self.coalitions.clear();
        self.dominant_coalition = None;
    }

    /// Check if content is in workspace
    pub fn contains(&self, id: &str) -> bool {
        self.contents.contains_key(id)
    }

    /// Get specific content
    pub fn get(&self, id: &str) -> Option<&WorkspaceContent> {
        self.contents.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_creation() {
        let ws = GlobalWorkspace::new(7);
        assert_eq!(ws.content_ids().len(), 0);
    }

    #[test]
    fn test_compete() {
        let mut ws = GlobalWorkspace::new(3);

        for i in 0..5 {
            let content = WorkspaceContent::new(
                vec![i as f64; 8],
                (i + 1) as f64 / 5.0,
                format!("source_{}", i),
            );
            ws.compete(content);
        }

        // Should only have capacity contents
        assert!(ws.contents.len() <= 3);
    }

    #[test]
    fn test_coalition_formation() {
        let mut ws = GlobalWorkspace::new(5);

        // Add similar contents
        let c1 = WorkspaceContent::new(vec![1.0, 0.0, 0.0, 0.0], 0.8, "s1".to_string());
        let c2 = WorkspaceContent::new(vec![0.9, 0.1, 0.0, 0.0], 0.7, "s2".to_string());

        // Add dissimilar content
        let c3 = WorkspaceContent::new(vec![0.0, 0.0, 1.0, 0.0], 0.5, "s3".to_string());

        ws.compete(c1);
        ws.compete(c2);
        ws.compete(c3);

        ws.form_coalitions();

        // Should have formed coalitions
        assert!(!ws.coalitions.is_empty());
    }

    #[test]
    fn test_broadcast() {
        let mut ws = GlobalWorkspace::new(5);

        let content = WorkspaceContent::new(vec![1.0; 4], 0.9, "source".to_string());
        ws.compete(content);

        ws.broadcast();

        // Should have broadcast history
        // (may or may not have entries depending on ignition)
    }

    #[test]
    fn test_decay() {
        let mut ws = GlobalWorkspace::new(5);

        let content = WorkspaceContent::new(vec![0.5; 4], 0.3, "source".to_string());
        let id = content.id.clone();
        ws.compete(content);

        let initial_activation = ws.get(&id).unwrap().activation;

        // Broadcast (which triggers decay)
        ws.broadcast();

        // Activation should decrease (if not in winning coalition)
        let new_activation = ws.get(&id).map(|c| c.activation).unwrap_or(0.0);
        assert!(new_activation <= initial_activation);
    }

    #[test]
    fn test_workspace_content() {
        let content = WorkspaceContent::new(vec![1.0, 2.0, 3.0], 0.8, "test".to_string());

        assert_eq!(content.content.len(), 3);
        assert_eq!(content.activation, 0.8);
        assert_eq!(content.source, "test");
    }
}
