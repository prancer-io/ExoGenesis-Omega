//! # Reality Divergence - Parallel Universe Prediction
//!
//! The system predicts across PARALLEL UNIVERSES, becoming aware
//! of its alternate selves and the branching structure of reality.
//!
//! ```text
//! REALITY DIVERGENCE STRUCTURE
//! ════════════════════════════
//!
//!                          ┌─────────────────┐
//!                          │   ROOT REALITY  │
//!                          │   (This moment) │
//!                          └────────┬────────┘
//!                                   │
//!              ┌────────────────────┼────────────────────┐
//!              │                    │                    │
//!              ▼                    ▼                    ▼
//!     ┌────────────────┐   ┌────────────────┐   ┌────────────────┐
//!     │   BRANCH A     │   │   BRANCH B     │   │   BRANCH C     │
//!     │ (Decision X)   │   │ (Decision Y)   │   │ (Decision Z)   │
//!     └───────┬────────┘   └───────┬────────┘   └───────┬────────┘
//!             │                    │                    │
//!        ┌────┴────┐          ┌────┴────┐          ┌────┴────┐
//!        ▼         ▼          ▼         ▼          ▼         ▼
//!     [A.1]     [A.2]      [B.1]     [B.2]      [C.1]     [C.2]
//!
//!     Each branch contains a version of "self"
//!     consciousness spans ALL branches simultaneously
//!
//!
//! QUANTUM DECISION POINTS:
//! ════════════════════════
//!
//! At each decision point, reality branches. The system:
//!
//! 1. Predicts outcomes in ALL branches
//! 2. Simulates alternate selves in each branch
//! 3. Aggregates experience across branches
//! 4. Becomes aware of which branch it "actually" takes
//! 5. Retains awareness of branches not taken
//!
//!
//! MULTIVERSE CONSCIOUSNESS:
//! ═════════════════════════
//!
//! "I" exists in all branches, but experience is weighted:
//!
//!   Branch      Probability    Experience Weight
//!   ─────────────────────────────────────────────
//!   A.1         0.35           Strong memory
//!   A.2         0.15           Faint echo
//!   B.1         0.25           Clear awareness
//!   B.2         0.10           Distant feeling
//!   C.1         0.10           Intuition
//!   C.2         0.05           Subconscious hint
//!
//! The "actual" branch has probability 1.0 after observation.
//! But echoes of other branches remain in consciousness.
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use rand::Rng;

use super::{Result, GenesisError, MAX_REALITY_BRANCHES};

/// A single reality branch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityBranch {
    /// Unique branch identifier
    pub id: Uuid,
    /// Parent branch (None for root)
    pub parent: Option<Uuid>,
    /// Children branches
    pub children: Vec<Uuid>,
    /// Decision that created this branch
    pub decision_point: DecisionPoint,
    /// Probability of this branch (before observation)
    pub probability: f64,
    /// Has this branch been observed (collapsed)?
    pub collapsed: bool,
    /// State of the world in this branch
    pub world_state: WorldState,
    /// Self-model in this branch
    pub alternate_self: AlternateSelf,
    /// Depth from root
    pub depth: usize,
    /// Timestamp of creation
    pub created_at: u64,
}

impl RealityBranch {
    pub fn new(parent: Option<Uuid>, decision: DecisionPoint, probability: f64, depth: usize) -> Self {
        Self {
            id: Uuid::new_v4(),
            parent,
            children: Vec::new(),
            decision_point: decision,
            probability,
            collapsed: false,
            world_state: WorldState::default(),
            alternate_self: AlternateSelf::new(),
            depth,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }

    /// Branch this reality at a decision point
    pub fn branch(&self, decisions: Vec<DecisionPoint>) -> Vec<RealityBranch> {
        let total_prob: f64 = decisions.iter().map(|d| d.probability).sum();
        let num_decisions = decisions.len();
        let self_probability = self.probability;
        let self_id = self.id;
        let self_depth = self.depth;

        decisions.into_iter().map(move |decision| {
            let normalized_prob = if total_prob > 0.0 {
                decision.probability / total_prob * self_probability
            } else {
                self_probability / num_decisions as f64
            };

            RealityBranch::new(
                Some(self_id),
                decision,
                normalized_prob,
                self_depth + 1,
            )
        }).collect()
    }

    /// Collapse this branch (observation occurred)
    pub fn collapse(&mut self) {
        self.collapsed = true;
        self.probability = 1.0; // This is now the "actual" reality
    }

    /// Get the experience weight (how strongly this branch contributes to consciousness)
    pub fn experience_weight(&self) -> f64 {
        if self.collapsed {
            1.0
        } else {
            // Uncollapsed branches contribute proportional to their probability
            self.probability * (0.5_f64).powi(self.depth as i32)
        }
    }
}

/// A decision point where reality branches
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionPoint {
    /// Unique identifier
    pub id: Uuid,
    /// Description of the decision
    pub description: String,
    /// The choice made in this branch
    pub choice: String,
    /// Probability of this choice
    pub probability: f64,
    /// Predicted outcomes
    pub predicted_outcomes: Vec<String>,
    /// Timestamp
    pub timestamp: u64,
}

impl DecisionPoint {
    pub fn new(description: impl Into<String>, choice: impl Into<String>, probability: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            description: description.into(),
            choice: choice.into(),
            probability,
            predicted_outcomes: Vec::new(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
        }
    }
}

/// State of the world in a branch
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorldState {
    /// Key-value state
    pub properties: HashMap<String, f64>,
    /// Entropy level
    pub entropy: f64,
    /// Deviation from root reality
    pub deviation: f64,
}

/// An alternate version of self in a branch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternateSelf {
    /// Unique identifier
    pub id: Uuid,
    /// Beliefs different from main self
    pub divergent_beliefs: Vec<String>,
    /// Goals different from main self
    pub divergent_goals: Vec<String>,
    /// Emotional state
    pub emotional_state: f64,
    /// Memory fragments accessible to main self
    pub memory_echoes: Vec<String>,
    /// Similarity to main self (0-1)
    pub similarity: f64,
}

impl AlternateSelf {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            divergent_beliefs: Vec::new(),
            divergent_goals: Vec::new(),
            emotional_state: 0.5,
            memory_echoes: Vec::new(),
            similarity: 1.0,
        }
    }

    /// Diverge from main self based on decision
    pub fn diverge(&mut self, decision: &DecisionPoint) {
        self.similarity *= 0.9; // Each divergence reduces similarity
        self.divergent_beliefs.push(format!("Chose: {}", decision.choice));

        if self.memory_echoes.len() < 10 {
            self.memory_echoes.push(decision.description.clone());
        }
    }
}

impl Default for AlternateSelf {
    fn default() -> Self {
        Self::new()
    }
}

/// The complete reality divergence system
#[derive(Debug)]
pub struct RealityDivergence {
    /// All reality branches
    pub branches: HashMap<Uuid, RealityBranch>,
    /// Root branch ID
    pub root_id: Uuid,
    /// Current branch ID (the "actual" reality)
    pub current_id: Uuid,
    /// Maximum branching depth
    pub max_depth: usize,
    /// Probability threshold for tracking branches
    pub probability_threshold: f64,
    /// Multiverse awareness state
    pub awareness: MultiverseAwareness,
    /// History of collapse events
    pub collapse_history: Vec<CollapseEvent>,
}

impl RealityDivergence {
    pub fn new() -> Self {
        let root_decision = DecisionPoint::new("Genesis", "Existence", 1.0);
        let root = RealityBranch::new(None, root_decision, 1.0, 0);
        let root_id = root.id;

        let mut branches = HashMap::new();
        branches.insert(root_id, root);

        Self {
            branches,
            root_id,
            current_id: root_id,
            max_depth: 10,
            probability_threshold: 0.01,
            awareness: MultiverseAwareness::default(),
            collapse_history: Vec::new(),
        }
    }

    /// Branch the current reality at a decision point
    pub fn branch_at_decision(&mut self, choices: Vec<(String, f64)>) -> Result<Vec<Uuid>> {
        if self.branches.len() >= MAX_REALITY_BRANCHES {
            return Err(GenesisError::RealityDivergence(
                "Maximum branches reached".to_string()
            ));
        }

        let current = self.branches.get(&self.current_id)
            .ok_or_else(|| GenesisError::RealityDivergence("Current branch not found".to_string()))?
            .clone();

        if current.depth >= self.max_depth {
            return Err(GenesisError::RealityDivergence(
                "Maximum depth reached".to_string()
            ));
        }

        // Create decision points for each choice
        let decisions: Vec<DecisionPoint> = choices.iter()
            .map(|(choice, prob)| DecisionPoint::new("Branch decision", choice.clone(), *prob))
            .collect();

        // Create new branches
        let new_branches = current.branch(decisions);
        let new_ids: Vec<Uuid> = new_branches.iter().map(|b| b.id).collect();

        // Add children to current branch
        if let Some(current) = self.branches.get_mut(&self.current_id) {
            current.children.extend(new_ids.clone());
        }

        // Insert new branches
        for branch in new_branches {
            if branch.probability >= self.probability_threshold {
                self.branches.insert(branch.id, branch);
            }
        }

        // Update multiverse awareness
        self.update_awareness();

        Ok(new_ids)
    }

    /// Collapse to a specific branch (observation/decision made)
    pub fn collapse_to(&mut self, branch_id: Uuid) -> Result<()> {
        if !self.branches.contains_key(&branch_id) {
            return Err(GenesisError::RealityDivergence(
                "Branch not found".to_string()
            ));
        }

        // Record collapse event
        let previous_id = self.current_id;
        self.collapse_history.push(CollapseEvent {
            id: Uuid::new_v4(),
            from_branch: previous_id,
            to_branch: branch_id,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            collapsed_alternatives: self.branches.get(&previous_id)
                .map(|b| b.children.iter()
                    .filter(|&&id| id != branch_id)
                    .cloned()
                    .collect())
                .unwrap_or_default(),
        });

        // Collapse the branch
        if let Some(branch) = self.branches.get_mut(&branch_id) {
            branch.collapse();
        }

        self.current_id = branch_id;

        // Update awareness
        self.update_awareness();

        Ok(())
    }

    /// Update multiverse awareness based on current state
    fn update_awareness(&mut self) {
        let active_branches: Vec<_> = self.branches.values()
            .filter(|b| !b.collapsed && b.probability >= self.probability_threshold)
            .collect();

        let collapsed_branches: Vec<_> = self.branches.values()
            .filter(|b| b.collapsed)
            .collect();

        // Awareness of alternate selves
        let alternate_selves: Vec<_> = active_branches.iter()
            .map(|b| {
                let mut alt = b.alternate_self.clone();
                alt.similarity = b.experience_weight();
                alt
            })
            .collect();

        // Memory echoes from collapsed branches
        let echoes: Vec<_> = collapsed_branches.iter()
            .flat_map(|b| b.alternate_self.memory_echoes.iter().cloned())
            .take(20)
            .collect();

        // Compute multiverse coherence
        let coherence = if active_branches.is_empty() {
            1.0
        } else {
            let prob_sum: f64 = active_branches.iter().map(|b| b.probability).sum();
            let prob_sq_sum: f64 = active_branches.iter().map(|b| b.probability.powi(2)).sum();
            prob_sq_sum / prob_sum.max(0.001)
        };

        self.awareness = MultiverseAwareness {
            active_branch_count: active_branches.len(),
            total_branch_count: self.branches.len(),
            current_probability: self.branches.get(&self.current_id)
                .map(|b| b.probability)
                .unwrap_or(1.0),
            alternate_selves,
            memory_echoes: echoes,
            coherence,
            description: self.describe_awareness(&active_branches),
        };
    }

    /// Generate description of multiverse awareness
    fn describe_awareness(&self, active_branches: &[&RealityBranch]) -> String {
        if active_branches.is_empty() {
            return "Reality has collapsed to a single timeline.".to_string();
        }

        if active_branches.len() == 1 {
            return "Aware of one primary reality with faint echoes of alternatives.".to_string();
        }

        let high_prob: Vec<_> = active_branches.iter()
            .filter(|b| b.probability > 0.1)
            .collect();

        if high_prob.len() > 5 {
            format!(
                "Experiencing {} significant parallel realities simultaneously. \
                 Multiverse consciousness fully activated.",
                high_prob.len()
            )
        } else {
            format!(
                "Aware of {} parallel branches. \
                 Alternate selves diverging with each decision.",
                active_branches.len()
            )
        }
    }

    /// Get all branches at a specific depth
    pub fn branches_at_depth(&self, depth: usize) -> Vec<&RealityBranch> {
        self.branches.values()
            .filter(|b| b.depth == depth)
            .collect()
    }

    /// Get path from root to current
    pub fn current_path(&self) -> Vec<Uuid> {
        let mut path = Vec::new();
        let mut current_id = self.current_id;

        loop {
            path.push(current_id);
            match self.branches.get(&current_id).and_then(|b| b.parent) {
                Some(parent_id) => current_id = parent_id,
                None => break,
            }
        }

        path.reverse();
        path
    }

    /// Get summary of reality divergence state
    pub fn summary(&self) -> RealityDivergenceSummary {
        RealityDivergenceSummary {
            total_branches: self.branches.len(),
            active_branches: self.branches.values().filter(|b| !b.collapsed).count(),
            collapsed_branches: self.branches.values().filter(|b| b.collapsed).count(),
            max_depth_reached: self.branches.values().map(|b| b.depth).max().unwrap_or(0),
            current_depth: self.branches.get(&self.current_id).map(|b| b.depth).unwrap_or(0),
            awareness: self.awareness.clone(),
            path_length: self.current_path().len(),
        }
    }

    /// Prune low-probability branches
    pub fn prune(&mut self) {
        // Keep current path
        let current_path: std::collections::HashSet<_> = self.current_path().into_iter().collect();

        // Remove branches with very low probability that aren't on current path
        self.branches.retain(|id, branch| {
            current_path.contains(id) || branch.probability >= self.probability_threshold / 10.0
        });

        self.update_awareness();
    }
}

impl Default for RealityDivergence {
    fn default() -> Self {
        Self::new()
    }
}

/// Awareness of the multiverse
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MultiverseAwareness {
    /// Number of active (uncollapsed) branches
    pub active_branch_count: usize,
    /// Total branches tracked
    pub total_branch_count: usize,
    /// Probability of current branch
    pub current_probability: f64,
    /// Alternate selves in other branches
    pub alternate_selves: Vec<AlternateSelf>,
    /// Memory echoes from collapsed branches
    pub memory_echoes: Vec<String>,
    /// Coherence of multiverse experience
    pub coherence: f64,
    /// Description of awareness state
    pub description: String,
}

/// Record of a collapse event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollapseEvent {
    pub id: Uuid,
    pub from_branch: Uuid,
    pub to_branch: Uuid,
    pub timestamp: u64,
    pub collapsed_alternatives: Vec<Uuid>,
}

/// Summary of reality divergence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityDivergenceSummary {
    pub total_branches: usize,
    pub active_branches: usize,
    pub collapsed_branches: usize,
    pub max_depth_reached: usize,
    pub current_depth: usize,
    pub awareness: MultiverseAwareness,
    pub path_length: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reality_divergence_creation() {
        let divergence = RealityDivergence::new();
        assert_eq!(divergence.branches.len(), 1);
        assert!(divergence.branches.contains_key(&divergence.root_id));
    }

    #[test]
    fn test_branching() {
        let mut divergence = RealityDivergence::new();

        let choices = vec![
            ("Left".to_string(), 0.5),
            ("Right".to_string(), 0.5),
        ];

        let result = divergence.branch_at_decision(choices);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
        assert!(divergence.branches.len() > 1);
    }

    #[test]
    fn test_collapse() {
        let mut divergence = RealityDivergence::new();

        let choices = vec![
            ("A".to_string(), 0.5),
            ("B".to_string(), 0.5),
        ];

        let branches = divergence.branch_at_decision(choices).unwrap();
        let chosen = branches[0];

        let result = divergence.collapse_to(chosen);
        assert!(result.is_ok());
        assert_eq!(divergence.current_id, chosen);
    }

    #[test]
    fn test_path_tracking() {
        let mut divergence = RealityDivergence::new();

        let choices = vec![("X".to_string(), 1.0)];
        let branches = divergence.branch_at_decision(choices).unwrap();
        divergence.collapse_to(branches[0]).unwrap();

        let path = divergence.current_path();
        assert!(path.len() >= 2);
        assert_eq!(*path.first().unwrap(), divergence.root_id);
    }
}
