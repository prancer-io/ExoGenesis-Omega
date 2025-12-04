//! Monte Carlo Tree Search for architecture discovery

use std::sync::{Arc, Weak};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use rand::Rng;
use omega_core::Architecture;

use crate::architecture::ArchitectureState;

#[derive(Error, Debug)]
pub enum MCTSError {
    #[error("Search failed: {0}")]
    SearchFailed(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("No valid moves available")]
    NoValidMoves,
}

/// Configuration for MCTS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCTSConfig {
    /// UCB1 exploration constant
    pub exploration_constant: f64,

    /// Depth of simulation rollouts
    pub simulation_depth: usize,

    /// Maximum number of search iterations
    pub max_iterations: usize,

    /// Number of parallel simulations
    pub parallel_simulations: usize,
}

impl Default for MCTSConfig {
    fn default() -> Self {
        Self {
            exploration_constant: 1.414, // sqrt(2)
            simulation_depth: 20, // Reduced for testing
            max_iterations: 100, // Reduced for testing
            parallel_simulations: 1, // Reduced for testing
        }
    }
}

/// A node in the MCTS search tree
#[derive(Debug)]
pub struct MCTSNode {
    pub state: ArchitectureState,
    pub visits: u64,
    pub value: f64,
    pub children: Vec<Arc<RwLock<MCTSNode>>>,
    pub parent: Option<Weak<RwLock<MCTSNode>>>,
    pub untried_actions: Vec<Action>,
}

impl MCTSNode {
    pub fn new(state: ArchitectureState, parent: Option<Weak<RwLock<MCTSNode>>>) -> Self {
        Self {
            state,
            visits: 0,
            value: 0.0,
            children: Vec::new(),
            parent,
            untried_actions: Vec::new(),
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub fn is_fully_expanded(&self) -> bool {
        self.untried_actions.is_empty()
    }

    pub fn ucb_value(&self, exploration_constant: f64, parent_visits: u64) -> f64 {
        if self.visits == 0 {
            return f64::INFINITY;
        }

        let exploitation = self.value / self.visits as f64;
        let exploration = exploration_constant *
            ((parent_visits as f64).ln() / self.visits as f64).sqrt();

        exploitation + exploration
    }
}

/// Actions that can be taken to modify an architecture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    AddNode { node_type: String },
    AddEdge { from: String, to: String },
    SetHyperparameter { name: String, value: f64 },
    Finalize,
}

/// MCTS search engine
pub struct MCTS {
    config: MCTSConfig,
    root: Arc<RwLock<MCTSNode>>,
}

impl MCTS {
    pub fn new(config: MCTSConfig, initial_state: ArchitectureState) -> Self {
        let root = Arc::new(RwLock::new(MCTSNode::new(initial_state, None)));
        Self { config, root }
    }

    /// Run MCTS search to find optimal architecture
    pub async fn search(&self, _root_state: ArchitectureState) -> Result<Architecture, MCTSError> {
        for iteration in 0..self.config.max_iterations {
            // Selection: traverse tree using UCB1
            let selected = self.select();

            // Expansion: add new child node
            let expanded = self.expand(selected)?;

            // Simulation: rollout to terminal state
            let value = self.simulate(&expanded).await;

            // Backpropagation: update statistics
            self.backpropagate(expanded, value);

            if iteration % 1000 == 0 {
                tracing::debug!("MCTS iteration {}/{}", iteration, self.config.max_iterations);
            }
        }

        // Extract best architecture from search tree
        self.extract_best()
    }

    /// Select most promising node using UCB1
    fn select(&self) -> Arc<RwLock<MCTSNode>> {
        let mut current = self.root.clone();
        let max_depth = 100; // Prevent infinite loops
        let mut depth = 0;

        loop {
            depth += 1;
            if depth > max_depth {
                tracing::warn!("MCTS selection reached max depth");
                return current;
            }

            let node = current.read();

            if node.is_leaf() || !node.is_fully_expanded() {
                drop(node);
                return current;
            }

            let parent_visits = node.visits;
            let children = node.children.clone();
            drop(node);

            if children.is_empty() {
                return current;
            }

            // Find child with highest UCB value
            let best_child = children.iter()
                .max_by(|a, b| {
                    let a_val = a.read().ucb_value(self.config.exploration_constant, parent_visits);
                    let b_val = b.read().ucb_value(self.config.exploration_constant, parent_visits);
                    a_val.partial_cmp(&b_val).unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap()
                .clone();

            current = best_child;
        }
    }

    /// Expand node with a new child
    fn expand(&self, node: Arc<RwLock<MCTSNode>>) -> Result<Arc<RwLock<MCTSNode>>, MCTSError> {
        let mut node_write = node.write();

        // Get valid actions for current state
        let actions = self.get_valid_actions(&node_write.state);

        if actions.is_empty() {
            return Err(MCTSError::NoValidMoves);
        }

        // Select random untried action
        let action = actions[rand::thread_rng().gen_range(0..actions.len())].clone();

        // Apply action to create new state
        let new_state = self.apply_action(&node_write.state, &action);

        // Create child node
        let child = Arc::new(RwLock::new(MCTSNode::new(
            new_state,
            Some(Arc::downgrade(&node)),
        )));

        node_write.children.push(child.clone());

        Ok(child)
    }

    /// Simulate random rollout from state
    async fn simulate(&self, node: &Arc<RwLock<MCTSNode>>) -> f64 {
        let state = node.read().state.clone();

        // Fast random rollout to terminal state
        let mut current_state = state;
        let mut depth = 0;

        while !current_state.is_complete() && depth < self.config.simulation_depth {
            let actions = self.get_valid_actions(&current_state);
            if actions.is_empty() {
                break;
            }

            let action = &actions[rand::thread_rng().gen_range(0..actions.len())];
            current_state = self.apply_action(&current_state, action);
            depth += 1;
        }

        // Evaluate terminal state (simplified - would call fitness evaluator)
        if current_state.is_complete() {
            rand::thread_rng().gen_range(0.0..1.0)
        } else {
            0.0
        }
    }

    /// Backpropagate value up the tree
    fn backpropagate(&self, node: Arc<RwLock<MCTSNode>>, value: f64) {
        let mut current = Some(node);

        while let Some(node_arc) = current {
            let mut node_write = node_arc.write();
            node_write.visits += 1;
            node_write.value += value;

            current = node_write.parent.as_ref().and_then(|weak| weak.upgrade());
        }
    }

    /// Extract best architecture from search tree
    fn extract_best(&self) -> Result<Architecture, MCTSError> {
        let root = self.root.read();

        // Select child with highest visit count
        let best_child = root.children.iter()
            .max_by_key(|child| child.read().visits)
            .ok_or_else(|| MCTSError::SearchFailed("No children found".to_string()))?;

        let _state = &best_child.read().state;

        // Convert state to architecture (simplified - would use state in production)
        Ok(Architecture {
            id: uuid::Uuid::now_v7().to_string(),
            name: "MCTS-generated".to_string(),
            paradigm: omega_core::Paradigm::Neural,
            substrate: omega_core::SubstrateType::Digital,
            fitness: None,
            lineage: vec![],
            created_at: chrono::Utc::now(),
        })
    }

    fn get_valid_actions(&self, state: &ArchitectureState) -> Vec<Action> {
        let mut actions = Vec::new();

        // Can always add nodes up to a limit
        if state.nodes.len() < 100 {
            actions.push(Action::AddNode {
                node_type: "hidden".to_string(),
            });
        }

        // Can add edges between existing nodes
        if state.nodes.len() >= 2 {
            actions.push(Action::AddEdge {
                from: state.nodes[0].clone(),
                to: state.nodes[state.nodes.len() - 1].clone(),
            });
        }

        // Can finalize if state is valid
        if state.is_complete() {
            actions.push(Action::Finalize);
        }

        actions
    }

    fn apply_action(&self, state: &ArchitectureState, action: &Action) -> ArchitectureState {
        let mut new_state = state.clone();

        match action {
            Action::AddNode { node_type } => {
                new_state.nodes.push(node_type.clone());
            }
            Action::AddEdge { from, to } => {
                new_state.edges.push((from.clone(), to.clone()));
            }
            Action::SetHyperparameter { name, value } => {
                new_state.hyperparameters.insert(name.clone(), *value);
            }
            Action::Finalize => {
                // Mark as complete
            }
        }

        new_state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mcts_search() {
        let config = MCTSConfig {
            max_iterations: 100,
            ..Default::default()
        };

        let initial_state = ArchitectureState::new();
        let mcts = MCTS::new(config, initial_state.clone());

        let result = mcts.search(initial_state).await;
        assert!(result.is_ok());
    }
}
