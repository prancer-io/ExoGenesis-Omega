//! Causal World Model - Understanding WHY Things Happen
//!
//! True prediction requires understanding causality, not just correlation.
//! This module implements causal inference using do-calculus and
//! interventional reasoning to answer "what if I do X?"
//!
//! ```text
//! CAUSAL WORLD MODEL
//! ══════════════════
//!
//!        ┌─────────┐
//!        │  Rain   │
//!        └────┬────┘
//!             │ causes
//!             ▼
//!    ┌────────────────┐
//!    │  Wet Ground    │◄──── (confounded)
//!    └────────┬───────┘
//!             │ causes
//!             ▼
//!    ┌────────────────┐
//!    │   Slippery     │
//!    └────────────────┘
//!
//! P(Slippery | do(Wet Ground)) ≠ P(Slippery | Wet Ground)
//! Intervention cuts the arrow from Rain to Wet Ground
//! ```

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use uuid::Uuid;

use super::Result;

/// A node in the causal graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalNode {
    /// Unique identifier
    pub id: Uuid,
    /// Name of the variable
    pub name: String,
    /// Current value
    pub value: f64,
    /// Prior probability distribution (discretized)
    pub prior: Vec<f64>,
    /// Is this an observed variable?
    pub observed: bool,
    /// Is this a latent/hidden variable?
    pub latent: bool,
    /// Structural equation coefficients (from parents)
    pub coefficients: HashMap<Uuid, f64>,
    /// Noise term
    pub noise: f64,
}

impl CausalNode {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            value: 0.0,
            prior: vec![0.5, 0.5], // Binary by default
            observed: true,
            latent: false,
            coefficients: HashMap::new(),
            noise: 0.0,
        }
    }

    /// Compute value from parents using structural equation
    pub fn compute(&mut self, parent_values: &HashMap<Uuid, f64>) {
        let mut value = self.noise;

        for (parent_id, coef) in &self.coefficients {
            if let Some(&parent_val) = parent_values.get(parent_id) {
                value += coef * parent_val;
            }
        }

        // Apply sigmoid for bounded output
        self.value = 1.0 / (1.0 + (-value).exp());
    }
}

/// An edge in the causal graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalEdge {
    /// Source node
    pub from: Uuid,
    /// Target node
    pub to: Uuid,
    /// Causal strength
    pub strength: f64,
    /// Is this a direct cause?
    pub direct: bool,
    /// Confidence in this causal relationship
    pub confidence: f64,
}

impl CausalEdge {
    pub fn new(from: Uuid, to: Uuid, strength: f64) -> Self {
        Self {
            from,
            to,
            strength,
            direct: true,
            confidence: 1.0,
        }
    }
}

/// An intervention (do-operator)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intervention {
    /// Variable to intervene on
    pub target: Uuid,
    /// Value to set
    pub value: f64,
    /// Description
    pub description: String,
}

impl Intervention {
    pub fn new(target: Uuid, value: f64, description: impl Into<String>) -> Self {
        Self {
            target,
            value,
            description: description.into(),
        }
    }
}

/// A causal query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalQuery {
    /// Query type
    pub query_type: QueryType,
    /// Target variable(s)
    pub targets: Vec<Uuid>,
    /// Conditioning variables
    pub conditions: HashMap<Uuid, f64>,
    /// Interventions
    pub interventions: Vec<Intervention>,
}

/// Types of causal queries
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum QueryType {
    /// P(Y | X) - observational
    Observational,
    /// P(Y | do(X)) - interventional
    Interventional,
    /// P(Y_x | X', Y') - counterfactual
    Counterfactual,
}

/// Result of causal inference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalInference {
    /// Query that was answered
    pub query: CausalQuery,
    /// Probability/value result
    pub result: f64,
    /// Confidence in the inference
    pub confidence: f64,
    /// Causal path used
    pub causal_path: Vec<Uuid>,
    /// Explanation
    pub explanation: String,
}

/// The Causal Graph
#[derive(Debug, Clone)]
pub struct CausalGraph {
    /// All nodes
    nodes: HashMap<Uuid, CausalNode>,
    /// All edges (parent -> children)
    edges: HashMap<Uuid, Vec<CausalEdge>>,
    /// Reverse edges (child -> parents)
    reverse_edges: HashMap<Uuid, Vec<Uuid>>,
    /// Name to ID mapping
    name_to_id: HashMap<String, Uuid>,
}

impl CausalGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            reverse_edges: HashMap::new(),
            name_to_id: HashMap::new(),
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: CausalNode) -> Uuid {
        let id = node.id;
        self.name_to_id.insert(node.name.clone(), id);
        self.nodes.insert(id, node);
        self.edges.entry(id).or_insert_with(Vec::new);
        self.reverse_edges.entry(id).or_insert_with(Vec::new);
        id
    }

    /// Add a causal edge
    pub fn add_edge(&mut self, from: Uuid, to: Uuid, strength: f64) {
        let edge = CausalEdge::new(from, to, strength);
        self.edges.entry(from).or_default().push(edge);
        self.reverse_edges.entry(to).or_default().push(from);

        // Update target node's coefficients
        if let Some(node) = self.nodes.get_mut(&to) {
            node.coefficients.insert(from, strength);
        }
    }

    /// Get node by name
    pub fn get_by_name(&self, name: &str) -> Option<&CausalNode> {
        self.name_to_id.get(name).and_then(|id| self.nodes.get(id))
    }

    /// Get node by ID
    pub fn get(&self, id: &Uuid) -> Option<&CausalNode> {
        self.nodes.get(id)
    }

    /// Get mutable node
    pub fn get_mut(&mut self, id: &Uuid) -> Option<&mut CausalNode> {
        self.nodes.get_mut(id)
    }

    /// Get parents of a node
    pub fn parents(&self, id: &Uuid) -> Vec<Uuid> {
        self.reverse_edges.get(id).cloned().unwrap_or_default()
    }

    /// Get children of a node
    pub fn children(&self, id: &Uuid) -> Vec<&CausalEdge> {
        self.edges.get(id)
            .map(|edges| edges.iter().collect())
            .unwrap_or_default()
    }

    /// Check for cycles (would make it not a DAG)
    pub fn has_cycle(&self) -> bool {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for &node in self.nodes.keys() {
            if self.has_cycle_util(node, &mut visited, &mut rec_stack) {
                return true;
            }
        }
        false
    }

    fn has_cycle_util(
        &self,
        node: Uuid,
        visited: &mut HashSet<Uuid>,
        rec_stack: &mut HashSet<Uuid>
    ) -> bool {
        if rec_stack.contains(&node) {
            return true;
        }
        if visited.contains(&node) {
            return false;
        }

        visited.insert(node);
        rec_stack.insert(node);

        for edge in self.children(&node) {
            if self.has_cycle_util(edge.to, visited, rec_stack) {
                return true;
            }
        }

        rec_stack.remove(&node);
        false
    }

    /// Topological sort
    pub fn topological_sort(&self) -> Vec<Uuid> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();

        for &node in self.nodes.keys() {
            self.topo_visit(node, &mut visited, &mut result);
        }

        result.reverse();
        result
    }

    fn topo_visit(&self, node: Uuid, visited: &mut HashSet<Uuid>, result: &mut Vec<Uuid>) {
        if visited.contains(&node) {
            return;
        }
        visited.insert(node);

        for edge in self.children(&node) {
            self.topo_visit(edge.to, visited, result);
        }

        result.push(node);
    }

    /// Find all paths between two nodes
    pub fn find_paths(&self, from: Uuid, to: Uuid) -> Vec<Vec<Uuid>> {
        let mut paths = Vec::new();
        let mut current_path = vec![from];
        self.find_paths_dfs(from, to, &mut current_path, &mut paths);
        paths
    }

    fn find_paths_dfs(
        &self,
        current: Uuid,
        target: Uuid,
        current_path: &mut Vec<Uuid>,
        paths: &mut Vec<Vec<Uuid>>
    ) {
        if current == target {
            paths.push(current_path.clone());
            return;
        }

        for edge in self.children(&current) {
            if !current_path.contains(&edge.to) {
                current_path.push(edge.to);
                self.find_paths_dfs(edge.to, target, current_path, paths);
                current_path.pop();
            }
        }
    }

    /// D-separation test
    pub fn d_separated(&self, x: Uuid, y: Uuid, z: &HashSet<Uuid>) -> bool {
        // Simplified d-separation: check if all paths are blocked
        let paths = self.find_paths(x, y);

        for path in paths {
            if !self.path_blocked(&path, z) {
                return false;
            }
        }
        true
    }

    fn path_blocked(&self, path: &[Uuid], conditioning: &HashSet<Uuid>) -> bool {
        if path.len() < 3 {
            return false; // Direct path, not blocked
        }

        // Check each triple in the path
        for i in 0..(path.len() - 2) {
            let a = path[i];
            let b = path[i + 1];
            let c = path[i + 2];

            // Check if this is a collider (a -> b <- c)
            let a_to_b = self.edges.get(&a)
                .map(|e| e.iter().any(|edge| edge.to == b))
                .unwrap_or(false);
            let c_to_b = self.edges.get(&c)
                .map(|e| e.iter().any(|edge| edge.to == b))
                .unwrap_or(false);

            let is_collider = a_to_b && c_to_b;

            if is_collider {
                // Collider: blocked unless b or descendant of b is in conditioning set
                if !conditioning.contains(&b) {
                    return true; // Blocked by collider
                }
            } else {
                // Non-collider: blocked if b is in conditioning set
                if conditioning.contains(&b) {
                    return true; // Blocked by conditioning
                }
            }
        }

        false
    }
}

impl Default for CausalGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// The Causal World Model
pub struct CausalWorldModel {
    /// The causal graph
    graph: CausalGraph,
    /// Intervention history
    intervention_history: Vec<Intervention>,
    /// Learned causal strengths
    learned_strengths: HashMap<(Uuid, Uuid), f64>,
    /// Confidence in the model
    model_confidence: f64,
}

impl CausalWorldModel {
    pub fn new() -> Self {
        Self {
            graph: CausalGraph::new(),
            intervention_history: Vec::new(),
            learned_strengths: HashMap::new(),
            model_confidence: 0.5,
        }
    }

    /// Add a variable to the world model
    pub fn add_variable(&mut self, name: impl Into<String>) -> Uuid {
        let node = CausalNode::new(name);
        self.graph.add_node(node)
    }

    /// Add a causal relationship
    pub fn add_cause(&mut self, cause: Uuid, effect: Uuid, strength: f64) {
        self.graph.add_edge(cause, effect, strength);
        self.learned_strengths.insert((cause, effect), strength);
    }

    /// Perform an intervention (do-operator)
    pub fn intervene(&mut self, intervention: Intervention) -> Result<HashMap<Uuid, f64>> {
        // Store intervention
        self.intervention_history.push(intervention.clone());

        // Get topological order
        let order = self.graph.topological_sort();

        // Propagate effects
        let mut values: HashMap<Uuid, f64> = HashMap::new();

        for &node_id in &order {
            if node_id == intervention.target {
                // Intervened variable: use intervention value, ignore parents
                values.insert(node_id, intervention.value);
            } else {
                // Non-intervened: compute from parents
                if let Some(node) = self.graph.get_mut(&node_id) {
                    node.compute(&values);
                    values.insert(node_id, node.value);
                }
            }
        }

        Ok(values)
    }

    /// Query: P(Y | X = x)
    pub fn query_observational(
        &self,
        target: Uuid,
        conditions: &HashMap<Uuid, f64>
    ) -> f64 {
        // Simple observational query
        // In a full implementation, this would use belief propagation

        if let Some(&value) = conditions.get(&target) {
            return value;
        }

        // Compute expected value given conditions
        let mut expected = 0.0;

        if let Some(node) = self.graph.get(&target) {
            expected = node.value;

            // Adjust based on conditions
            for (cond_id, cond_val) in conditions {
                if let Some(&strength) = self.learned_strengths.get(&(*cond_id, target)) {
                    expected += strength * cond_val;
                }
            }
        }

        expected.clamp(0.0, 1.0)
    }

    /// Query: P(Y | do(X = x))
    pub fn query_interventional(
        &mut self,
        target: Uuid,
        interventions: Vec<Intervention>
    ) -> Result<f64> {
        // Perform interventions
        let mut final_values = HashMap::new();

        for intervention in interventions {
            let values = self.intervene(intervention)?;
            final_values.extend(values);
        }

        Ok(final_values.get(&target).copied().unwrap_or(0.0))
    }

    /// Estimate causal effect of X on Y
    pub fn causal_effect(&mut self, cause: Uuid, effect: Uuid) -> Result<f64> {
        // Average causal effect: E[Y | do(X=1)] - E[Y | do(X=0)]

        let y_do_x1 = self.query_interventional(
            effect,
            vec![Intervention::new(cause, 1.0, "Set to 1")]
        )?;

        let y_do_x0 = self.query_interventional(
            effect,
            vec![Intervention::new(cause, 0.0, "Set to 0")]
        )?;

        Ok(y_do_x1 - y_do_x0)
    }

    /// Why did Y happen? (Causal attribution)
    pub fn why(&self, effect: Uuid) -> Vec<(Uuid, f64)> {
        let mut attributions = Vec::new();

        // Get all paths to this effect
        for &node_id in self.graph.nodes.keys() {
            if node_id != effect {
                let paths = self.graph.find_paths(node_id, effect);

                if !paths.is_empty() {
                    // Calculate total causal influence
                    let mut total_influence = 0.0;

                    for path in &paths {
                        let mut path_strength = 1.0;
                        for i in 0..(path.len() - 1) {
                            if let Some(&strength) = self.learned_strengths.get(&(path[i], path[i+1])) {
                                path_strength *= strength;
                            }
                        }
                        total_influence += path_strength;
                    }

                    if total_influence.abs() > 0.01 {
                        attributions.push((node_id, total_influence));
                    }
                }
            }
        }

        // Sort by influence magnitude
        attributions.sort_by(|a, b| b.1.abs().partial_cmp(&a.1.abs()).unwrap());
        attributions
    }

    /// What will cause Y? (Causal prediction)
    pub fn what_causes(&self, effect: Uuid) -> Vec<Uuid> {
        self.graph.parents(&effect)
    }

    /// What will Y cause? (Causal consequences)
    pub fn what_effects(&self, cause: Uuid) -> Vec<Uuid> {
        self.graph.children(&cause)
            .iter()
            .map(|e| e.to)
            .collect()
    }

    /// Is X a cause of Y?
    pub fn is_cause(&self, potential_cause: Uuid, effect: Uuid) -> bool {
        !self.graph.find_paths(potential_cause, effect).is_empty()
    }

    /// Get the causal graph
    pub fn graph(&self) -> &CausalGraph {
        &self.graph
    }

    /// Get model confidence
    pub fn confidence(&self) -> f64 {
        self.model_confidence
    }

    /// Learn causal structure from observations
    pub fn learn_from_observation(&mut self, observations: &[(Uuid, f64)]) {
        // Update node values
        for (id, value) in observations {
            if let Some(node) = self.graph.get_mut(id) {
                node.value = *value;
            }
        }

        // Update causal strengths based on correlations
        for i in 0..observations.len() {
            for j in 0..observations.len() {
                if i != j {
                    let (id_i, val_i) = &observations[i];
                    let (id_j, val_j) = &observations[j];

                    // Check if there's an edge from i to j
                    if self.learned_strengths.contains_key(&(*id_i, *id_j)) {
                        // Update strength based on correlation
                        let correlation = val_i * val_j;
                        let current = self.learned_strengths.get(&(*id_i, *id_j)).copied().unwrap_or(0.0);
                        let new_strength = 0.9 * current + 0.1 * correlation;
                        self.learned_strengths.insert((*id_i, *id_j), new_strength);
                    }
                }
            }
        }

        // Update confidence
        self.model_confidence = (self.model_confidence * 0.99 + 0.01).min(1.0);
    }
}

impl Default for CausalWorldModel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_causal_graph() {
        let mut graph = CausalGraph::new();

        let rain = graph.add_node(CausalNode::new("Rain"));
        let wet = graph.add_node(CausalNode::new("Wet Ground"));
        let slip = graph.add_node(CausalNode::new("Slippery"));

        graph.add_edge(rain, wet, 0.8);
        graph.add_edge(wet, slip, 0.9);

        assert!(!graph.has_cycle());
        assert_eq!(graph.parents(&wet), vec![rain]);
    }

    #[test]
    fn test_causal_intervention() {
        let mut model = CausalWorldModel::new();

        let x = model.add_variable("X");
        let y = model.add_variable("Y");
        model.add_cause(x, y, 0.7);

        let result = model.intervene(Intervention::new(x, 1.0, "Set X"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_causal_effect() {
        let mut model = CausalWorldModel::new();

        let treatment = model.add_variable("Treatment");
        let outcome = model.add_variable("Outcome");
        model.add_cause(treatment, outcome, 0.5);

        let effect = model.causal_effect(treatment, outcome);
        assert!(effect.is_ok());
    }

    #[test]
    fn test_topological_sort() {
        let mut graph = CausalGraph::new();

        let a = graph.add_node(CausalNode::new("A"));
        let b = graph.add_node(CausalNode::new("B"));
        let c = graph.add_node(CausalNode::new("C"));

        graph.add_edge(a, b, 1.0);
        graph.add_edge(b, c, 1.0);

        let order = graph.topological_sort();
        assert_eq!(order.len(), 3);

        // A should come before B, B before C
        let pos_a = order.iter().position(|&x| x == a).unwrap();
        let pos_b = order.iter().position(|&x| x == b).unwrap();
        let pos_c = order.iter().position(|&x| x == c).unwrap();

        assert!(pos_a < pos_b);
        assert!(pos_b < pos_c);
    }
}
