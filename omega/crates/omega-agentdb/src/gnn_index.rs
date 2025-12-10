//! Self-Learning Graph Neural Network Index
//!
//! Adaptive indexing structure that learns optimal navigation:
//! - Graph-based structure with learnable edge weights
//! - Self-organizing based on query patterns
//! - Adaptive routing for efficient search
//! - Online learning from search results

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

use crate::simd_ops::{self, DistanceMetric};

/// Node in the GNN index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GNNNode {
    /// Node ID
    pub id: String,
    /// Feature vector
    pub embedding: Vec<f32>,
    /// Metadata
    pub metadata: serde_json::Value,
    /// Outgoing edges with learned weights
    pub edges: Vec<GNNEdge>,
    /// Degree centrality (number of times visited)
    pub visit_count: u64,
    /// Average time to find relevant results through this node
    pub avg_search_quality: f64,
}

impl GNNNode {
    pub fn new(id: String, embedding: Vec<f32>, metadata: serde_json::Value) -> Self {
        Self {
            id,
            embedding,
            metadata,
            edges: Vec::new(),
            visit_count: 0,
            avg_search_quality: 0.5,
        }
    }
}

/// Edge in the GNN with learnable weight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GNNEdge {
    /// Target node ID
    pub target: String,
    /// Edge weight (learned)
    pub weight: f32,
    /// Times traversed
    pub traversal_count: u64,
    /// Success rate (led to good results)
    pub success_rate: f64,
}

impl GNNEdge {
    pub fn new(target: String, initial_weight: f32) -> Self {
        Self {
            target,
            weight: initial_weight,
            traversal_count: 0,
            success_rate: 0.5,
        }
    }
}

/// Query history for learning
#[derive(Debug, Clone)]
struct QueryRecord {
    query: Vec<f32>,
    path: Vec<String>,
    found_results: Vec<String>,
    quality_score: f64,
}

/// Self-learning GNN index configuration
#[derive(Debug, Clone)]
pub struct GNNConfig {
    /// Maximum edges per node
    pub max_edges: usize,
    /// Number of entry points
    pub num_entry_points: usize,
    /// Learning rate for edge weights
    pub learning_rate: f32,
    /// Exploration vs exploitation
    pub exploration_rate: f32,
    /// History size for learning
    pub history_size: usize,
    /// Distance metric
    pub metric: DistanceMetric,
}

impl Default for GNNConfig {
    fn default() -> Self {
        Self {
            max_edges: 32,
            num_entry_points: 8,
            learning_rate: 0.1,
            exploration_rate: 0.1,
            history_size: 1000,
            metric: DistanceMetric::Cosine,
        }
    }
}

/// Search result from GNN index
#[derive(Debug, Clone)]
pub struct GNNSearchResult {
    pub id: String,
    pub similarity: f32,
    pub metadata: serde_json::Value,
    pub path_length: usize,
}

/// Self-Learning GNN Index
pub struct GNNIndex {
    config: GNNConfig,
    /// All nodes
    nodes: HashMap<String, GNNNode>,
    /// Entry point node IDs
    entry_points: Vec<String>,
    /// Query history for learning
    query_history: VecDeque<QueryRecord>,
    /// Dimension of embeddings
    dimension: Option<usize>,
}

impl GNNIndex {
    /// Create new GNN index
    pub fn new(config: GNNConfig) -> Self {
        Self {
            config,
            nodes: HashMap::new(),
            entry_points: Vec::new(),
            query_history: VecDeque::with_capacity(1000),
            dimension: None,
        }
    }

    /// Insert a vector into the index
    pub fn insert(&mut self, id: String, embedding: Vec<f32>, metadata: serde_json::Value) {
        // Set dimension on first insert
        if self.dimension.is_none() {
            self.dimension = Some(embedding.len());
        }

        let node = GNNNode::new(id.clone(), embedding.clone(), metadata);
        self.nodes.insert(id.clone(), node);

        // Connect to existing nodes
        self.connect_new_node(&id, &embedding);

        // Update entry points
        self.update_entry_points(&id);
    }

    /// Connect a new node to existing nodes
    fn connect_new_node(&mut self, new_id: &str, new_embedding: &[f32]) {
        if self.nodes.len() <= 1 {
            return;
        }

        // Find nearest nodes to connect to
        let mut distances: Vec<(String, f32)> = self
            .nodes
            .iter()
            .filter(|(id, _)| *id != new_id)
            .map(|(id, node)| {
                let dist = self.config.metric.compute(new_embedding, &node.embedding);
                (id.clone(), dist)
            })
            .collect();

        distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        // Connect to nearest neighbors (bidirectional)
        let num_connections = self.config.max_edges.min(distances.len());

        for (target_id, dist) in distances.iter().take(num_connections) {
            let weight = 1.0 / (1.0 + dist);

            // Add edge from new node
            if let Some(new_node) = self.nodes.get_mut(new_id) {
                new_node.edges.push(GNNEdge::new(target_id.clone(), weight));
            }

            // Add reverse edge
            if let Some(target_node) = self.nodes.get_mut(target_id) {
                if target_node.edges.len() < self.config.max_edges {
                    target_node.edges.push(GNNEdge::new(new_id.to_string(), weight));
                }
            }
        }
    }

    /// Update entry points
    fn update_entry_points(&mut self, new_id: &str) {
        if self.entry_points.len() < self.config.num_entry_points {
            self.entry_points.push(new_id.to_string());
        } else if rand::thread_rng().gen::<f32>() < 0.1 {
            // Occasionally add new entry points
            let idx = rand::thread_rng().gen_range(0..self.entry_points.len());
            self.entry_points[idx] = new_id.to_string();
        }
    }

    /// Search for k nearest neighbors
    pub fn search(&mut self, query: &[f32], k: usize) -> Vec<GNNSearchResult> {
        if self.nodes.is_empty() {
            return Vec::new();
        }

        let mut visited: HashSet<String> = HashSet::new();
        let mut candidates: Vec<(String, f32, usize)> = Vec::new();
        let mut path: Vec<String> = Vec::new();

        // Start from entry points
        for entry_id in &self.entry_points.clone() {
            if let Some(node) = self.nodes.get(entry_id) {
                let sim = self
                    .config
                    .metric
                    .to_similarity(self.config.metric.compute(query, &node.embedding));
                candidates.push((entry_id.clone(), sim, 0));
            }
        }

        // Greedy search with exploration
        let max_hops = 100;
        let mut current_best = f32::MIN;

        for _ in 0..max_hops {
            if candidates.is_empty() {
                break;
            }

            // Sort by similarity
            candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

            // Pick next node (with exploration)
            let idx = if rand::thread_rng().gen::<f32>() < self.config.exploration_rate
                && candidates.len() > 1
            {
                rand::thread_rng().gen_range(0..candidates.len().min(3))
            } else {
                0
            };

            let (current_id, current_sim, depth) = candidates.remove(idx);

            if visited.contains(&current_id) {
                continue;
            }

            visited.insert(current_id.clone());
            path.push(current_id.clone());

            // Update visit count
            if let Some(node) = self.nodes.get_mut(&current_id) {
                node.visit_count += 1;
            }

            // Check for improvement
            if current_sim > current_best {
                current_best = current_sim;
            } else if depth > 10 {
                // Stop if not improving
                break;
            }

            // Expand neighbors
            if let Some(node) = self.nodes.get(&current_id) {
                for edge in &node.edges {
                    if !visited.contains(&edge.target) {
                        if let Some(target_node) = self.nodes.get(&edge.target) {
                            let sim = self.config.metric.to_similarity(
                                self.config.metric.compute(query, &target_node.embedding),
                            );

                            // Weight by edge strength
                            let weighted_sim = sim * edge.weight;
                            candidates.push((edge.target.clone(), weighted_sim, depth + 1));
                        }
                    }
                }
            }
        }

        // Collect results from visited nodes
        let mut results: Vec<GNNSearchResult> = visited
            .iter()
            .filter_map(|id| {
                self.nodes.get(id).map(|node| {
                    let sim = self
                        .config
                        .metric
                        .to_similarity(self.config.metric.compute(query, &node.embedding));
                    GNNSearchResult {
                        id: id.clone(),
                        similarity: sim,
                        metadata: node.metadata.clone(),
                        path_length: path.iter().position(|p| p == id).unwrap_or(0),
                    }
                })
            })
            .collect();

        // Sort by similarity and take top k
        results.sort_by(|a, b| {
            b.similarity
                .partial_cmp(&a.similarity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        results.truncate(k);

        // Record query for learning
        let found_ids: Vec<String> = results.iter().map(|r| r.id.clone()).collect();
        let quality = results.first().map(|r| r.similarity as f64).unwrap_or(0.0);

        self.record_query(query.to_vec(), path, found_ids, quality);

        results
    }

    /// Record query for learning
    fn record_query(
        &mut self,
        query: Vec<f32>,
        path: Vec<String>,
        found_results: Vec<String>,
        quality_score: f64,
    ) {
        let record = QueryRecord {
            query,
            path,
            found_results,
            quality_score,
        };

        self.query_history.push_back(record);
        if self.query_history.len() > self.config.history_size {
            self.query_history.pop_front();
        }
    }

    /// Learn from query history to improve navigation
    pub fn learn(&mut self) {
        let history: Vec<QueryRecord> = self.query_history.iter().cloned().collect();

        for record in history {
            // Update edges along successful paths
            if record.quality_score > 0.5 {
                self.reinforce_path(&record.path, record.quality_score as f32);
            } else {
                self.weaken_path(&record.path, record.quality_score as f32);
            }

            // Update node quality scores
            for node_id in &record.path {
                if let Some(node) = self.nodes.get_mut(node_id) {
                    node.avg_search_quality = 0.9 * node.avg_search_quality
                        + 0.1 * record.quality_score;
                }
            }
        }

        // Prune weak edges
        self.prune_edges();

        // Add new edges based on query patterns
        self.add_shortcut_edges();
    }

    /// Reinforce edges along a successful path
    fn reinforce_path(&mut self, path: &[String], quality: f32) {
        for i in 0..path.len().saturating_sub(1) {
            let from = &path[i];
            let to = &path[i + 1];

            if let Some(node) = self.nodes.get_mut(from) {
                for edge in &mut node.edges {
                    if &edge.target == to {
                        edge.weight += self.config.learning_rate * quality;
                        edge.weight = edge.weight.min(5.0);
                        edge.traversal_count += 1;
                        edge.success_rate =
                            0.9 * edge.success_rate + 0.1 * quality as f64;
                    }
                }
            }
        }
    }

    /// Weaken edges along a poor path
    fn weaken_path(&mut self, path: &[String], quality: f32) {
        for i in 0..path.len().saturating_sub(1) {
            let from = &path[i];
            let to = &path[i + 1];

            if let Some(node) = self.nodes.get_mut(from) {
                for edge in &mut node.edges {
                    if &edge.target == to {
                        edge.weight -= self.config.learning_rate * (1.0 - quality);
                        edge.weight = edge.weight.max(0.1);
                        edge.traversal_count += 1;
                        edge.success_rate =
                            0.9 * edge.success_rate + 0.1 * quality as f64;
                    }
                }
            }
        }
    }

    /// Prune weak or unused edges
    fn prune_edges(&mut self) {
        for node in self.nodes.values_mut() {
            node.edges.retain(|e| e.weight > 0.2 || e.traversal_count < 10);
        }
    }

    /// Add shortcut edges based on frequent paths
    fn add_shortcut_edges(&mut self) {
        // Find frequently co-occurring result pairs
        let mut pair_counts: HashMap<(String, String), u32> = HashMap::new();

        for record in &self.query_history {
            if record.quality_score > 0.7 {
                for i in 0..record.found_results.len() {
                    for j in (i + 1)..record.found_results.len() {
                        let pair = if record.found_results[i] < record.found_results[j] {
                            (
                                record.found_results[i].clone(),
                                record.found_results[j].clone(),
                            )
                        } else {
                            (
                                record.found_results[j].clone(),
                                record.found_results[i].clone(),
                            )
                        };
                        *pair_counts.entry(pair).or_insert(0) += 1;
                    }
                }
            }
        }

        // Add edges for frequent pairs
        for ((a, b), count) in pair_counts {
            if count > 5 {
                // Check if edge exists
                let has_edge_a = self
                    .nodes
                    .get(&a)
                    .map(|n| n.edges.iter().any(|e| e.target == b))
                    .unwrap_or(false);

                if !has_edge_a {
                    if let Some(node) = self.nodes.get_mut(&a) {
                        if node.edges.len() < self.config.max_edges {
                            node.edges.push(GNNEdge::new(b.clone(), 1.0));
                        }
                    }
                }
            }
        }
    }

    /// Get node by ID
    pub fn get(&self, id: &str) -> Option<&GNNNode> {
        self.nodes.get(id)
    }

    /// Remove node by ID
    pub fn remove(&mut self, id: &str) -> bool {
        if self.nodes.remove(id).is_some() {
            // Remove from entry points
            self.entry_points.retain(|e| e != id);

            // Remove edges pointing to this node
            for node in self.nodes.values_mut() {
                node.edges.retain(|e| e.target != id);
            }

            true
        } else {
            false
        }
    }

    /// Get number of nodes
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Get graph statistics
    pub fn stats(&self) -> GNNStats {
        let total_edges: usize = self.nodes.values().map(|n| n.edges.len()).sum();
        let avg_edges = if self.nodes.is_empty() {
            0.0
        } else {
            total_edges as f64 / self.nodes.len() as f64
        };

        let avg_quality = if self.nodes.is_empty() {
            0.0
        } else {
            self.nodes.values().map(|n| n.avg_search_quality).sum::<f64>()
                / self.nodes.len() as f64
        };

        GNNStats {
            node_count: self.nodes.len(),
            edge_count: total_edges,
            avg_edges_per_node: avg_edges,
            entry_point_count: self.entry_points.len(),
            query_history_size: self.query_history.len(),
            avg_node_quality: avg_quality,
        }
    }

    /// Clear all data
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.entry_points.clear();
        self.query_history.clear();
        self.dimension = None;
    }
}

impl Default for GNNIndex {
    fn default() -> Self {
        Self::new(GNNConfig::default())
    }
}

/// Statistics about the GNN index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GNNStats {
    pub node_count: usize,
    pub edge_count: usize,
    pub avg_edges_per_node: f64,
    pub entry_point_count: usize,
    pub query_history_size: usize,
    pub avg_node_quality: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gnn_index_creation() {
        let index = GNNIndex::new(GNNConfig::default());
        assert!(index.is_empty());
    }

    #[test]
    fn test_gnn_insert_and_search() {
        let mut index = GNNIndex::new(GNNConfig::default());

        // Insert vectors
        for i in 0..50 {
            let embedding: Vec<f32> = (0..64).map(|j| ((i * j) % 100) as f32 / 100.0).collect();
            index.insert(
                format!("vec_{}", i),
                embedding,
                serde_json::json!({"index": i}),
            );
        }

        assert_eq!(index.len(), 50);

        // Search
        let query: Vec<f32> = (0..64).map(|j| ((25 * j) % 100) as f32 / 100.0).collect();
        let results = index.search(&query, 5);

        assert!(!results.is_empty());
        assert!(results.len() <= 5);
    }

    #[test]
    fn test_gnn_learning() {
        let mut index = GNNIndex::new(GNNConfig {
            max_edges: 8,
            num_entry_points: 4,
            learning_rate: 0.2,
            ..Default::default()
        });

        // Insert vectors
        for i in 0..20 {
            let embedding: Vec<f32> = (0..32).map(|j| (i + j) as f32 / 50.0).collect();
            index.insert(format!("v{}", i), embedding, serde_json::json!({}));
        }

        // Perform multiple searches to build history
        for i in 0..10 {
            let query: Vec<f32> = (0..32).map(|j| (i * 2 + j) as f32 / 50.0).collect();
            index.search(&query, 3);
        }

        // Learn from queries
        index.learn();

        // Stats should reflect learning
        let stats = index.stats();
        assert!(stats.query_history_size > 0);
    }

    #[test]
    fn test_gnn_remove() {
        let mut index = GNNIndex::new(GNNConfig::default());

        for i in 0..10 {
            let embedding = vec![i as f32; 16];
            index.insert(format!("v{}", i), embedding, serde_json::json!({}));
        }

        assert_eq!(index.len(), 10);
        assert!(index.remove("v5"));
        assert_eq!(index.len(), 9);
        assert!(!index.remove("v5"));
    }

    #[test]
    fn test_gnn_get() {
        let mut index = GNNIndex::new(GNNConfig::default());

        let embedding = vec![1.0, 2.0, 3.0];
        index.insert(
            "test".to_string(),
            embedding.clone(),
            serde_json::json!({"key": "value"}),
        );

        let node = index.get("test");
        assert!(node.is_some());
        assert_eq!(node.unwrap().embedding, embedding);
    }

    #[test]
    fn test_gnn_stats() {
        let mut index = GNNIndex::new(GNNConfig::default());

        for i in 0..30 {
            let embedding: Vec<f32> = (0..16).map(|j| ((i + j) % 16) as f32).collect();
            index.insert(format!("v{}", i), embedding, serde_json::json!({}));
        }

        let stats = index.stats();
        assert_eq!(stats.node_count, 30);
        assert!(stats.edge_count > 0);
        assert!(stats.avg_edges_per_node > 0.0);
    }
}
