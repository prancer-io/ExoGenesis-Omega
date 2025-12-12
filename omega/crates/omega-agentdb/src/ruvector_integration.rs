//! RuVector Integration
//!
//! Integrates RuVector components for advanced vector database functionality:
//! - ruvector-core: HNSW indexing with SIMD acceleration
//! - ruvector-gnn: Self-learning graph neural networks
//! - ruvector-graph: Cypher-like graph queries
//!
//! This module provides a unified interface to RuVector's advanced features
//! while maintaining backward compatibility with the existing AgentDB API.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// RuVector configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuVectorConfig {
    /// Vector dimension
    pub dimension: usize,
    /// HNSW M parameter (max connections per node)
    pub hnsw_m: usize,
    /// HNSW ef_construction (search width during construction)
    pub ef_construction: usize,
    /// HNSW ef_search (search width during query)
    pub ef_search: usize,
    /// Enable GNN self-learning
    pub gnn_enabled: bool,
    /// GNN learning rate
    pub gnn_learning_rate: f64,
    /// Enable graph queries
    pub graph_enabled: bool,
    /// SIMD optimization level
    pub simd_level: SimdLevel,
}

/// SIMD optimization level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimdLevel {
    /// No SIMD (fallback)
    None,
    /// SSE4.2
    SSE42,
    /// AVX2
    AVX2,
    /// AVX-512
    AVX512,
    /// Auto-detect best available
    Auto,
}

impl Default for SimdLevel {
    fn default() -> Self {
        Self::Auto
    }
}

impl Default for RuVectorConfig {
    fn default() -> Self {
        Self {
            dimension: 1024,
            hnsw_m: 32,
            ef_construction: 200,
            ef_search: 100,
            gnn_enabled: true,
            gnn_learning_rate: 0.001,
            graph_enabled: true,
            simd_level: SimdLevel::Auto,
        }
    }
}

/// Error types for RuVector operations
#[derive(Debug, Error)]
pub enum RuVectorError {
    #[error("Index error: {0}")]
    IndexError(String),

    #[error("Query error: {0}")]
    QueryError(String),

    #[error("GNN error: {0}")]
    GNNError(String),

    #[error("Graph error: {0}")]
    GraphError(String),

    #[error("Dimension mismatch: expected {expected}, got {got}")]
    DimensionMismatch { expected: usize, got: usize },

    #[error("Not found: {0}")]
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, RuVectorError>;

/// A vector entry in the index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorEntry {
    /// Unique identifier
    pub id: String,
    /// Vector embedding
    pub embedding: Vec<f32>,
    /// Metadata
    pub metadata: serde_json::Value,
    /// Graph connections (for GNN)
    pub connections: Vec<String>,
    /// Entry timestamp
    pub timestamp: u64,
}

/// Search result from RuVector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuVectorResult {
    /// Vector ID
    pub id: String,
    /// Similarity score (0-1, higher is better)
    pub similarity: f64,
    /// Distance (lower is better)
    pub distance: f64,
    /// Metadata
    pub metadata: serde_json::Value,
    /// GNN-adjusted score (if GNN enabled)
    pub gnn_score: Option<f64>,
}

/// Graph query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQueryResult {
    /// Matched nodes
    pub nodes: Vec<String>,
    /// Path if path query
    pub path: Option<Vec<String>>,
    /// Relationship types traversed
    pub relationships: Vec<String>,
    /// Total path weight
    pub weight: f64,
}

/// GNN layer for self-learning index
#[derive(Debug, Clone)]
pub struct GNNLayer {
    /// Input dimension
    input_dim: usize,
    /// Output dimension
    output_dim: usize,
    /// Weights
    weights: Vec<Vec<f64>>,
    /// Bias
    bias: Vec<f64>,
    /// Learning rate
    learning_rate: f64,
    /// Update count
    update_count: u64,
}

impl GNNLayer {
    /// Create new GNN layer
    pub fn new(input_dim: usize, output_dim: usize, learning_rate: f64) -> Self {
        // Xavier initialization
        let scale = (2.0 / (input_dim + output_dim) as f64).sqrt();
        let weights: Vec<Vec<f64>> = (0..output_dim)
            .map(|_| {
                (0..input_dim)
                    .map(|i| (i as f64 * 0.1).sin() * scale)
                    .collect()
            })
            .collect();

        let bias = vec![0.0; output_dim];

        Self {
            input_dim,
            output_dim,
            weights,
            bias,
            learning_rate,
            update_count: 0,
        }
    }

    /// Forward pass
    pub fn forward(&self, input: &[f64]) -> Vec<f64> {
        let mut output = self.bias.clone();
        for (i, w_row) in self.weights.iter().enumerate() {
            for (j, &w) in w_row.iter().enumerate() {
                if j < input.len() {
                    output[i] += w * input[j];
                }
            }
            // ReLU activation
            output[i] = output[i].max(0.0);
        }
        output
    }

    /// Update weights based on feedback
    pub fn update(&mut self, input: &[f64], target: &[f64]) {
        // Simplified gradient update
        let output = self.forward(input);

        for (i, w_row) in self.weights.iter_mut().enumerate() {
            if i < target.len() {
                let error = target[i] - output[i];
                for (j, w) in w_row.iter_mut().enumerate() {
                    if j < input.len() {
                        *w += self.learning_rate * error * input[j];
                    }
                }
                self.bias[i] += self.learning_rate * error;
            }
        }

        self.update_count += 1;
    }

    /// Get update count
    pub fn update_count(&self) -> u64 {
        self.update_count
    }

    /// Get input dimension
    pub fn input_dim(&self) -> usize {
        self.input_dim
    }

    /// Get output dimension
    pub fn output_dim(&self) -> usize {
        self.output_dim
    }
}

/// Main RuVector index combining HNSW, GNN, and Graph capabilities
pub struct RuVectorIndex {
    /// Configuration
    config: RuVectorConfig,
    /// Vector storage
    vectors: HashMap<String, VectorEntry>,
    /// HNSW graph structure (simplified)
    hnsw_graph: Vec<Vec<(String, f32)>>,
    /// GNN layers
    gnn_layers: Vec<GNNLayer>,
    /// Graph relationships
    graph_edges: HashMap<(String, String), GraphEdge>,
    /// Vector count
    count: usize,
}

/// Graph edge with relationship type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    /// Source node
    pub source: String,
    /// Target node
    pub target: String,
    /// Relationship type
    pub relationship: String,
    /// Weight
    pub weight: f64,
    /// Properties
    pub properties: serde_json::Value,
}

impl RuVectorIndex {
    /// Create new RuVector index
    pub fn new(config: RuVectorConfig) -> Self {
        let mut gnn_layers = Vec::new();
        if config.gnn_enabled {
            // Create 2-layer GNN
            gnn_layers.push(GNNLayer::new(config.dimension, 128, config.gnn_learning_rate));
            gnn_layers.push(GNNLayer::new(128, 64, config.gnn_learning_rate));
        }

        Self {
            config,
            vectors: HashMap::new(),
            hnsw_graph: Vec::new(),
            gnn_layers,
            graph_edges: HashMap::new(),
            count: 0,
        }
    }

    /// Insert a vector
    pub fn insert(&mut self, entry: VectorEntry) -> Result<()> {
        if entry.embedding.len() != self.config.dimension {
            return Err(RuVectorError::DimensionMismatch {
                expected: self.config.dimension,
                got: entry.embedding.len(),
            });
        }

        let id = entry.id.clone();
        self.vectors.insert(id.clone(), entry);
        self.count += 1;

        // Update HNSW structure (simplified)
        self.update_hnsw_connections(&id)?;

        Ok(())
    }

    /// Search for similar vectors
    pub fn search(&self, query: &[f32], k: usize) -> Result<Vec<RuVectorResult>> {
        if query.len() != self.config.dimension {
            return Err(RuVectorError::DimensionMismatch {
                expected: self.config.dimension,
                got: query.len(),
            });
        }

        // Compute similarities for all vectors
        let mut results: Vec<RuVectorResult> = self
            .vectors
            .values()
            .map(|v| {
                let similarity = self.cosine_similarity(query, &v.embedding);
                let distance = 1.0 - similarity;

                // Apply GNN scoring if enabled
                let gnn_score = if self.config.gnn_enabled && !self.gnn_layers.is_empty() {
                    Some(self.compute_gnn_score(&v.embedding))
                } else {
                    None
                };

                RuVectorResult {
                    id: v.id.clone(),
                    similarity,
                    distance,
                    metadata: v.metadata.clone(),
                    gnn_score,
                }
            })
            .collect();

        // Sort by similarity (descending)
        results.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());
        results.truncate(k);

        Ok(results)
    }

    /// Search with GNN-enhanced ranking
    pub fn search_gnn(&self, query: &[f32], k: usize) -> Result<Vec<RuVectorResult>> {
        if !self.config.gnn_enabled {
            return self.search(query, k);
        }

        let mut results = self.search(query, k * 2)?; // Get more candidates

        // Re-rank with GNN scores
        for result in &mut results {
            if let Some(gnn_score) = result.gnn_score {
                // Combine similarity with GNN score
                result.similarity = 0.7 * result.similarity + 0.3 * gnn_score;
            }
        }

        results.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());
        results.truncate(k);

        Ok(results)
    }

    /// Execute a Cypher-like graph query
    pub fn graph_query(&self, query: &str) -> Result<Vec<GraphQueryResult>> {
        if !self.config.graph_enabled {
            return Err(RuVectorError::GraphError(
                "Graph queries not enabled".to_string(),
            ));
        }

        // Parse simple query patterns
        // Format: MATCH (a)-[r:TYPE]->(b) WHERE a.id = 'xxx' RETURN b
        let results = self.execute_graph_query(query)?;
        Ok(results)
    }

    /// Add a graph relationship
    pub fn add_relationship(
        &mut self,
        source: &str,
        target: &str,
        relationship: &str,
        weight: f64,
        properties: serde_json::Value,
    ) -> Result<()> {
        if !self.config.graph_enabled {
            return Err(RuVectorError::GraphError(
                "Graph queries not enabled".to_string(),
            ));
        }

        let edge = GraphEdge {
            source: source.to_string(),
            target: target.to_string(),
            relationship: relationship.to_string(),
            weight,
            properties,
        };

        self.graph_edges
            .insert((source.to_string(), target.to_string()), edge);

        // Update vector connections
        if let Some(v) = self.vectors.get_mut(source) {
            if !v.connections.contains(&target.to_string()) {
                v.connections.push(target.to_string());
            }
        }

        Ok(())
    }

    /// Learn from feedback (GNN training)
    pub fn learn(&mut self, query: &[f32], relevant_ids: &[String]) -> Result<()> {
        if !self.config.gnn_enabled || self.gnn_layers.is_empty() {
            return Ok(());
        }

        let query_f64: Vec<f64> = query.iter().map(|&x| x as f64).collect();

        // Create target: 1.0 for relevant, 0.0 for others
        for id in relevant_ids {
            if let Some(entry) = self.vectors.get(id) {
                let input: Vec<f64> = entry.embedding.iter().map(|&x| x as f64).collect();

                // Propagate through layers and update
                let mut current = input.clone();
                for layer in &mut self.gnn_layers {
                    let target = layer.forward(&query_f64);
                    layer.update(&current, &target);
                    current = layer.forward(&current);
                }
            }
        }

        Ok(())
    }

    /// Get vector by ID
    pub fn get(&self, id: &str) -> Option<&VectorEntry> {
        self.vectors.get(id)
    }

    /// Remove vector
    pub fn remove(&mut self, id: &str) -> bool {
        if self.vectors.remove(id).is_some() {
            self.count -= 1;
            true
        } else {
            false
        }
    }

    /// Get count
    pub fn len(&self) -> usize {
        self.count
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Get GNN stats
    pub fn gnn_stats(&self) -> GNNStats {
        let total_updates: u64 = self.gnn_layers.iter().map(|l| l.update_count()).sum();
        GNNStats {
            layers: self.gnn_layers.len(),
            total_updates,
            enabled: self.config.gnn_enabled,
        }
    }

    /// Get HNSW graph structure (neighbors for each vector)
    pub fn hnsw_graph(&self) -> &Vec<Vec<(String, f32)>> {
        &self.hnsw_graph
    }

    // === Private methods ===

    fn cosine_similarity(&self, a: &[f32], b: &[f32]) -> f64 {
        if a.len() != b.len() {
            return 0.0;
        }

        // Use simsimd for SIMD acceleration
        use simsimd::SpatialSimilarity;
        match f32::cosine(a, b) {
            Some(distance) => 1.0 - distance,
            None => {
                // Fallback
                let mut dot = 0.0;
                let mut norm_a = 0.0;
                let mut norm_b = 0.0;
                for (&x, &y) in a.iter().zip(b.iter()) {
                    dot += (x * y) as f64;
                    norm_a += (x * x) as f64;
                    norm_b += (y * y) as f64;
                }
                let denom = (norm_a * norm_b).sqrt();
                if denom > 0.0 {
                    dot / denom
                } else {
                    0.0
                }
            }
        }
    }

    fn compute_gnn_score(&self, embedding: &[f32]) -> f64 {
        if self.gnn_layers.is_empty() {
            return 0.5;
        }

        let mut current: Vec<f64> = embedding.iter().map(|&x| x as f64).collect();
        for layer in &self.gnn_layers {
            current = layer.forward(&current);
        }

        // Aggregate to single score
        let score = current.iter().sum::<f64>() / current.len().max(1) as f64;
        score.clamp(0.0, 1.0)
    }

    fn update_hnsw_connections(&mut self, _id: &str) -> Result<()> {
        // Simplified HNSW update - in production would use full HNSW algorithm
        // This is a placeholder for the ruvector-core integration
        Ok(())
    }

    fn execute_graph_query(&self, query: &str) -> Result<Vec<GraphQueryResult>> {
        // Simplified query parser
        let query_lower = query.to_lowercase();

        if query_lower.contains("match") {
            // Extract source and relationship pattern
            let results: Vec<GraphQueryResult> = self
                .graph_edges
                .values()
                .filter(|e| {
                    query_lower.contains(&e.source.to_lowercase())
                        || query_lower.contains(&e.relationship.to_lowercase())
                })
                .map(|e| GraphQueryResult {
                    nodes: vec![e.source.clone(), e.target.clone()],
                    path: Some(vec![e.source.clone(), e.target.clone()]),
                    relationships: vec![e.relationship.clone()],
                    weight: e.weight,
                })
                .collect();

            Ok(results)
        } else {
            Err(RuVectorError::QueryError(format!(
                "Unsupported query: {}",
                query
            )))
        }
    }
}

impl Default for RuVectorIndex {
    fn default() -> Self {
        Self::new(RuVectorConfig::default())
    }
}

/// GNN statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GNNStats {
    /// Number of GNN layers
    pub layers: usize,
    /// Total update count
    pub total_updates: u64,
    /// GNN enabled
    pub enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ruvector_config() {
        let config = RuVectorConfig::default();
        assert_eq!(config.dimension, 1024);
        assert!(config.gnn_enabled);
    }

    #[test]
    fn test_ruvector_index_creation() {
        let config = RuVectorConfig {
            dimension: 64,
            ..Default::default()
        };
        let index = RuVectorIndex::new(config);
        assert_eq!(index.len(), 0);
    }

    #[test]
    fn test_insert_and_search() {
        let config = RuVectorConfig {
            dimension: 8,
            gnn_enabled: false,
            ..Default::default()
        };
        let mut index = RuVectorIndex::new(config);

        let entry = VectorEntry {
            id: "test1".to_string(),
            embedding: vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            metadata: serde_json::json!({"name": "test"}),
            connections: vec![],
            timestamp: 0,
        };
        index.insert(entry).unwrap();

        let query = vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let results = index.search(&query, 1).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "test1");
        assert!(results[0].similarity > 0.99);
    }

    #[test]
    fn test_gnn_layer() {
        let mut layer = GNNLayer::new(8, 4, 0.01);

        let input = vec![1.0; 8];
        let output = layer.forward(&input);
        assert_eq!(output.len(), 4);

        let target = vec![0.5; 4];
        layer.update(&input, &target);
        assert_eq!(layer.update_count(), 1);
    }

    #[test]
    fn test_graph_relationship() {
        let config = RuVectorConfig {
            dimension: 4,
            graph_enabled: true,
            ..Default::default()
        };
        let mut index = RuVectorIndex::new(config);

        // Add entries
        for i in 0..3 {
            let entry = VectorEntry {
                id: format!("node{}", i),
                embedding: vec![i as f32; 4],
                metadata: serde_json::json!({}),
                connections: vec![],
                timestamp: 0,
            };
            index.insert(entry).unwrap();
        }

        // Add relationship
        index
            .add_relationship("node0", "node1", "RELATES_TO", 1.0, serde_json::json!({}))
            .unwrap();

        // Query
        let results = index.graph_query("MATCH (a)-[r:RELATES_TO]->(b)").unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_gnn_learning() {
        let config = RuVectorConfig {
            dimension: 8,
            gnn_enabled: true,
            ..Default::default()
        };
        let mut index = RuVectorIndex::new(config);

        let entry = VectorEntry {
            id: "learn1".to_string(),
            embedding: vec![0.5; 8],
            metadata: serde_json::json!({}),
            connections: vec![],
            timestamp: 0,
        };
        index.insert(entry).unwrap();

        // Learn from feedback
        let query = vec![0.5; 8];
        index.learn(&query, &["learn1".to_string()]).unwrap();

        let stats = index.gnn_stats();
        assert!(stats.total_updates > 0);
    }
}
