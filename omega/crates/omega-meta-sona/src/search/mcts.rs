//! Monte Carlo Tree Search for architecture discovery

use std::sync::{Arc, Weak};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use rand::Rng;
use omega_core::Architecture;

use crate::architecture::{ArchitectureState, NodeType, ConnectionType};
use crate::fitness::{FitnessEvaluator, EvaluationError};

#[derive(Error, Debug)]
pub enum MCTSError {
    #[error("Search failed: {0}")]
    SearchFailed(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("No valid moves available")]
    NoValidMoves,

    #[error("No valid architecture found")]
    NoValidArchitecture,

    #[error("Evaluation error: {0}")]
    EvaluationError(#[from] EvaluationError),
}

/// Configuration for MCTS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCTSConfig {
    /// UCB1 exploration constant (typically sqrt(2) ≈ 1.414)
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
            simulation_depth: 20,
            max_iterations: 100,
            parallel_simulations: 1,
        }
    }
}

/// Actions that can be taken to modify an architecture
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ArchitectureAction {
    /// Add a node of given type
    AddNode { node_type: NodeType, layer: usize },
    /// Add connection between nodes
    AddConnection { from: usize, to: usize, conn_type: ConnectionType },
    /// Set hyperparameter
    SetHyperparameter { param: String, value: f64 },
    /// Remove a node
    RemoveNode { node_id: usize },
    /// Finalize the architecture
    Finalize,
}

impl ArchitectureAction {
    /// Get valid actions for the current state
    pub fn get_valid_actions(state: &ArchitectureState) -> Vec<Self> {
        let mut actions = Vec::new();

        if state.depth >= state.max_depth {
            actions.push(ArchitectureAction::Finalize);
            return actions;
        }

        // Can add various node types
        for node_type in [NodeType::Hidden, NodeType::Attention, NodeType::Memory, NodeType::LSTM] {
            for layer in 0..=state.nodes.len().min(5) {
                actions.push(ArchitectureAction::AddNode {
                    node_type: node_type.clone(),
                    layer,
                });
            }
        }

        // Can add connections between existing nodes
        for from in 0..state.nodes.len() {
            for to in 0..state.nodes.len() {
                if from != to && !state.has_edge(&state.nodes[from], &state.nodes[to]) {
                    actions.push(ArchitectureAction::AddConnection {
                        from,
                        to,
                        conn_type: ConnectionType::Forward,
                    });
                }
            }
        }

        // Can set some hyperparameters
        if state.nodes.len() >= 2 {
            actions.push(ArchitectureAction::SetHyperparameter {
                param: "learning_rate".to_string(),
                value: 0.001,
            });
        }

        // Can finalize if we have minimum structure
        if state.nodes.len() >= 3 && state.edges.len() >= 2 {
            actions.push(ArchitectureAction::Finalize);
        }

        actions
    }

    /// Apply action to state and return new state
    pub fn apply_to_state(&self, state: &ArchitectureState) -> ArchitectureState {
        let mut new_state = state.clone();
        new_state.depth += 1;

        match self {
            ArchitectureAction::AddNode { .. } => {
                let node_id = format!("node_{}", new_state.nodes.len());
                new_state.nodes.push(node_id);
            }
            ArchitectureAction::AddConnection { from, to, .. } => {
                if *from < new_state.nodes.len() && *to < new_state.nodes.len() {
                    let from_id = new_state.nodes[*from].clone();
                    let to_id = new_state.nodes[*to].clone();
                    if !new_state.has_edge(&from_id, &to_id) {
                        new_state.edges.push((from_id, to_id));
                    }
                }
            }
            ArchitectureAction::SetHyperparameter { param, value } => {
                new_state.hyperparameters.insert(param.clone(), *value);
            }
            ArchitectureAction::RemoveNode { node_id } => {
                if *node_id < new_state.nodes.len() {
                    new_state.nodes.remove(*node_id);
                    // Also remove edges involving this node
                    new_state.edges.retain(|(f, t)| {
                        !f.starts_with(&format!("node_{}", node_id)) &&
                        !t.starts_with(&format!("node_{}", node_id))
                    });
                }
            }
            ArchitectureAction::Finalize => {
                // Mark as terminal by setting depth to max
            }
        }

        new_state
    }
}

/// MCTS Node representing a state in the architecture search tree
#[derive(Debug)]
pub struct MCTSNode {
    /// Current architecture state
    pub state: ArchitectureState,
    /// Visit count
    pub visits: u64,
    /// Total value accumulated
    pub total_value: f64,
    /// Children nodes
    pub children: Vec<Arc<RwLock<MCTSNode>>>,
    /// Parent reference
    pub parent: Option<Weak<RwLock<MCTSNode>>>,
    /// Action that led to this node
    pub action: Option<ArchitectureAction>,
    /// Is this a terminal state?
    pub is_terminal: bool,
    /// Untried actions from this state
    pub untried_actions: Vec<ArchitectureAction>,
}

impl MCTSNode {
    pub fn new(state: ArchitectureState) -> Self {
        let is_terminal = state.is_terminal();
        let untried_actions = if is_terminal {
            Vec::new()
        } else {
            ArchitectureAction::get_valid_actions(&state)
        };

        Self {
            state,
            visits: 0,
            total_value: 0.0,
            children: Vec::new(),
            parent: None,
            action: None,
            is_terminal,
            untried_actions,
        }
    }

    pub fn new_with_parent(
        state: ArchitectureState,
        parent: Weak<RwLock<MCTSNode>>,
        action: ArchitectureAction,
    ) -> Self {
        let is_terminal = state.is_terminal();
        let untried_actions = if is_terminal {
            Vec::new()
        } else {
            ArchitectureAction::get_valid_actions(&state)
        };

        Self {
            state,
            visits: 0,
            total_value: 0.0,
            children: Vec::new(),
            parent: Some(parent),
            action: Some(action),
            is_terminal,
            untried_actions,
        }
    }

    /// UCB1 value for selection
    pub fn ucb1(&self, parent_visits: u64, exploration_constant: f64) -> f64 {
        if self.visits == 0 {
            return f64::INFINITY;
        }

        let exploitation = self.total_value / self.visits as f64;
        let exploration = exploration_constant *
            ((parent_visits as f64).ln() / self.visits as f64).sqrt();

        exploitation + exploration
    }

    /// Average value
    pub fn average_value(&self) -> f64 {
        if self.visits == 0 {
            0.0
        } else {
            self.total_value / self.visits as f64
        }
    }

    /// Is fully expanded?
    pub fn is_fully_expanded(&self) -> bool {
        self.untried_actions.is_empty()
    }

    /// Is leaf node?
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

/// MCTS search engine
pub struct MCTS {
    config: MCTSConfig,
    evaluator: Arc<FitnessEvaluator>,
}

impl MCTS {
    pub fn new(config: MCTSConfig) -> Self {
        Self {
            config,
            evaluator: Arc::new(FitnessEvaluator::default()),
        }
    }

    pub fn with_evaluator(config: MCTSConfig, evaluator: FitnessEvaluator) -> Self {
        Self {
            config,
            evaluator: Arc::new(evaluator),
        }
    }

    /// Run MCTS search to find optimal architecture
    pub async fn search(&self, initial_state: ArchitectureState) -> Result<Architecture, MCTSError> {
        let root = Arc::new(RwLock::new(MCTSNode::new(initial_state)));

        for iteration in 0..self.config.max_iterations {
            // 1. Selection - traverse tree using UCB1
            let selected = self.select(root.clone());

            // 2. Expansion - add a new child node
            let expanded = self.expand(selected)?;

            // 3. Simulation - random rollout to terminal state
            let value = self.simulate(&expanded.read().state).await;

            // 4. Backpropagation - update values up the tree
            self.backpropagate(expanded, value);

            if iteration % 100 == 0 {
                tracing::debug!("MCTS iteration {}/{}", iteration, self.config.max_iterations);
            }
        }

        // Return best child of root
        self.best_child(root)
    }

    /// Select node to expand using UCB1
    fn select(&self, node: Arc<RwLock<MCTSNode>>) -> Arc<RwLock<MCTSNode>> {
        let mut current = node;
        let max_depth = 100; // Prevent infinite loops
        let mut depth = 0;

        loop {
            depth += 1;
            if depth > max_depth {
                tracing::warn!("MCTS selection reached max depth");
                return current;
            }

            let node_read = current.read();

            // If terminal or not fully expanded, return this node
            if node_read.is_terminal || !node_read.is_fully_expanded() {
                drop(node_read);
                return current;
            }

            // Select child with highest UCB1
            let parent_visits = node_read.visits;
            let best_child = node_read.children
                .iter()
                .max_by(|a, b| {
                    let a_ucb = a.read().ucb1(parent_visits, self.config.exploration_constant);
                    let b_ucb = b.read().ucb1(parent_visits, self.config.exploration_constant);
                    a_ucb.partial_cmp(&b_ucb).unwrap_or(std::cmp::Ordering::Equal)
                })
                .cloned();

            drop(node_read);

            match best_child {
                Some(child) => current = child,
                None => return current,
            }
        }
    }

    /// Expand node by adding a child
    fn expand(&self, node: Arc<RwLock<MCTSNode>>) -> Result<Arc<RwLock<MCTSNode>>, MCTSError> {
        let mut node_write = node.write();

        // If no untried actions, return the node itself
        if node_write.untried_actions.is_empty() {
            drop(node_write);
            return Ok(node);
        }

        // Pick random untried action
        let action_idx = rand::thread_rng().gen_range(0..node_write.untried_actions.len());
        let action = node_write.untried_actions.remove(action_idx);

        // Create new state by applying action
        let new_state = action.apply_to_state(&node_write.state);

        // Create child node
        let child = Arc::new(RwLock::new(MCTSNode::new_with_parent(
            new_state,
            Arc::downgrade(&node),
            action,
        )));

        node_write.children.push(child.clone());
        drop(node_write);

        Ok(child)
    }

    /// Simulate random rollout from state
    async fn simulate(&self, state: &ArchitectureState) -> f64 {
        let mut current = state.clone();
        let mut depth = 0;

        // Random rollout until terminal
        while !current.is_terminal() && depth < self.config.simulation_depth {
            let actions = ArchitectureAction::get_valid_actions(&current);
            if actions.is_empty() {
                break;
            }
            let action = &actions[rand::thread_rng().gen_range(0..actions.len())];
            current = action.apply_to_state(&current);
            depth += 1;
        }

        // Evaluate terminal state
        let architecture = self.state_to_architecture(&current);
        let fitness = self.evaluator.evaluate(&architecture).await
            .unwrap_or_else(|e| {
                tracing::warn!("Fitness evaluation failed: {}", e);
                omega_core::FitnessScore {
                    overall: 0.5,
                    capability: 0.5,
                    efficiency: 0.5,
                    alignment: 0.5,
                    novelty: 0.5,
                    confidence: 0.5,
                }
            });

        fitness.overall
    }

    /// Backpropagate value up the tree
    fn backpropagate(&self, node: Arc<RwLock<MCTSNode>>, value: f64) {
        let mut current = Some(node);

        while let Some(node_arc) = current {
            {
                let mut node_write = node_arc.write();
                node_write.visits += 1;
                node_write.total_value += value;
            }

            let parent = node_arc.read().parent.as_ref().and_then(|p| p.upgrade());
            current = parent;
        }
    }

    /// Get best child based on visit count
    fn best_child(&self, root: Arc<RwLock<MCTSNode>>) -> Result<Architecture, MCTSError> {
        let root_read = root.read();

        let best = root_read.children
            .iter()
            .max_by_key(|c| c.read().visits)
            .ok_or(MCTSError::NoValidArchitecture)?;

        let best_state = &best.read().state;
        let best_visits = best.read().visits;
        let best_value = best.read().average_value();

        tracing::info!(
            "Best architecture found: {} nodes, {} edges, visits={}, avg_value={:.3}",
            best_state.nodes.len(),
            best_state.edges.len(),
            best_visits,
            best_value
        );

        Ok(self.state_to_architecture(best_state))
    }

    /// Convert state to architecture
    fn state_to_architecture(&self, state: &ArchitectureState) -> Architecture {
        Architecture {
            id: uuid::Uuid::now_v7().to_string(),
            name: format!("MCTS-{}-nodes-{}-edges", state.nodes.len(), state.edges.len()),
            paradigm: omega_core::Paradigm::Neural,
            substrate: omega_core::SubstrateType::Digital,
            fitness: None,
            lineage: Vec::new(),
            created_at: chrono::Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ucb1_calculation() {
        let mut node = MCTSNode::new(ArchitectureState::default());
        node.visits = 10;
        node.total_value = 5.0;

        let ucb = node.ucb1(100, 1.414);
        assert!(ucb > 0.5);  // exploitation component
        assert!(ucb < 2.0);  // bounded by reasonable value
    }

    #[test]
    fn test_ucb1_unvisited() {
        let node = MCTSNode::new(ArchitectureState::default());
        let ucb = node.ucb1(100, 1.414);
        assert_eq!(ucb, f64::INFINITY);
    }

    #[test]
    fn test_average_value() {
        let mut node = MCTSNode::new(ArchitectureState::default());
        node.visits = 10;
        node.total_value = 7.5;
        assert_eq!(node.average_value(), 0.75);
    }

    #[test]
    fn test_average_value_no_visits() {
        let node = MCTSNode::new(ArchitectureState::default());
        assert_eq!(node.average_value(), 0.0);
    }

    #[test]
    fn test_architecture_actions() {
        let state = ArchitectureState::default();
        let actions = ArchitectureAction::get_valid_actions(&state);
        assert!(!actions.is_empty());
    }

    #[test]
    fn test_add_node_action() {
        let state = ArchitectureState::default();
        let action = ArchitectureAction::AddNode {
            node_type: NodeType::Hidden,
            layer: 0,
        };
        let new_state = action.apply_to_state(&state);
        assert_eq!(new_state.nodes.len(), 1);
        assert_eq!(new_state.depth, 1);
    }

    #[test]
    fn test_add_connection_action() {
        let mut state = ArchitectureState::default();
        state.nodes.push("node_0".to_string());
        state.nodes.push("node_1".to_string());

        let action = ArchitectureAction::AddConnection {
            from: 0,
            to: 1,
            conn_type: ConnectionType::Forward,
        };
        let new_state = action.apply_to_state(&state);
        assert_eq!(new_state.edges.len(), 1);
    }

    #[test]
    fn test_set_hyperparameter_action() {
        let state = ArchitectureState::default();
        let action = ArchitectureAction::SetHyperparameter {
            param: "learning_rate".to_string(),
            value: 0.001,
        };
        let new_state = action.apply_to_state(&state);
        assert_eq!(new_state.hyperparameters.get("learning_rate"), Some(&0.001));
    }

    #[test]
    fn test_node_fully_expanded() {
        let mut node = MCTSNode::new(ArchitectureState::default());
        assert!(!node.is_fully_expanded());
        node.untried_actions.clear();
        assert!(node.is_fully_expanded());
    }

    #[test]
    fn test_terminal_state() {
        let mut state = ArchitectureState::default();
        state.depth = 25; // Beyond max_depth of 20
        assert!(state.is_terminal());
    }

    #[tokio::test]
    async fn test_mcts_search() {
        let config = MCTSConfig {
            exploration_constant: 1.414,
            simulation_depth: 10,
            max_iterations: 50,
            parallel_simulations: 1,
        };

        let mcts = MCTS::new(config);
        let initial = ArchitectureState::default();

        let result = mcts.search(initial).await;
        assert!(result.is_ok());

        let architecture = result.unwrap();
        assert!(!architecture.name.is_empty());
        assert!(!architecture.id.is_empty());
    }

    #[tokio::test]
    async fn test_mcts_search_with_custom_evaluator() {
        let config = MCTSConfig {
            exploration_constant: 1.414,
            simulation_depth: 10,
            max_iterations: 30,
            parallel_simulations: 1,
        };

        let evaluator = FitnessEvaluator::with_weights(0.5, 0.2, 0.2, 0.1);
        let mcts = MCTS::with_evaluator(config, evaluator);
        let initial = ArchitectureState::default();

        let result = mcts.search(initial).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_selection() {
        let config = MCTSConfig::default();
        let mcts = MCTS::new(config);
        let root = Arc::new(RwLock::new(MCTSNode::new(ArchitectureState::default())));

        let selected = mcts.select(root);
        assert!(selected.read().visits == 0);
    }

    #[test]
    fn test_expansion() {
        let config = MCTSConfig::default();
        let mcts = MCTS::new(config);
        let node = Arc::new(RwLock::new(MCTSNode::new(ArchitectureState::default())));

        let expanded = mcts.expand(node.clone());
        assert!(expanded.is_ok());
    }

    #[tokio::test]
    async fn test_simulation() {
        let config = MCTSConfig {
            simulation_depth: 5,
            ..Default::default()
        };
        let mcts = MCTS::new(config);
        let state = ArchitectureState::default();

        let value = mcts.simulate(&state).await;
        assert!(value >= 0.0 && value <= 1.0);
    }

    #[test]
    fn test_backpropagation() {
        let config = MCTSConfig::default();
        let mcts = MCTS::new(config);

        let root = Arc::new(RwLock::new(MCTSNode::new(ArchitectureState::default())));
        let child = Arc::new(RwLock::new(MCTSNode::new_with_parent(
            ArchitectureState::default(),
            Arc::downgrade(&root),
            ArchitectureAction::Finalize,
        )));

        root.write().children.push(child.clone());

        mcts.backpropagate(child.clone(), 0.8);

        assert_eq!(child.read().visits, 1);
        assert_eq!(child.read().total_value, 0.8);
        assert_eq!(root.read().visits, 1);
        assert_eq!(root.read().total_value, 0.8);
    }
}
