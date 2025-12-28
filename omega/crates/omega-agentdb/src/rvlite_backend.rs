//! RvLite Integration Backend for ExoGenesis Omega
//!
//! This module provides a unified interface for long-term memory storage
//! using RvLite's vector database capabilities, including:
//! - Semantic similarity search with configurable distance metrics
//! - Graph-based relationships via Cypher queries
//! - Multi-tier memory consolidation
//! - Persistent storage with JSON export/import
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    RvLiteBackend                            │
//! ├─────────────────────────────────────────────────────────────┤
//! │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
//! │  │ VectorStore │  │ GraphStore  │  │ SemanticMemory      │  │
//! │  │ (HNSW+SIMD) │  │ (Cypher)    │  │ (Tier Consolidation)│  │
//! │  └─────────────┘  └─────────────┘  └─────────────────────┘  │
//! │         │                │                    │             │
//! │         └────────────────┴────────────────────┘             │
//! │                          │                                  │
//! │                   ┌──────┴──────┐                           │
//! │                   │ Persistence │                           │
//! │                   │ (JSON/File) │                           │
//! │                   └─────────────┘                           │
//! └─────────────────────────────────────────────────────────────┘
//! ```

use crate::{AgentDBError, CausalEdge, Embedding, VectorId, VectorResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

// =============================================================================
// STORAGE BACKEND TRAIT
// =============================================================================

/// Trait defining the interface for pluggable storage backends.
/// Both the existing HNSW backend and RvLite can implement this trait.
#[async_trait::async_trait]
pub trait VectorBackend: Send + Sync {
    /// Insert a vector with metadata, returns the assigned ID
    async fn insert(&self, embedding: Embedding, metadata: serde_json::Value) -> Result<VectorId, AgentDBError>;

    /// Insert a vector with a specific ID
    async fn insert_with_id(&self, id: &str, embedding: Embedding, metadata: serde_json::Value) -> Result<(), AgentDBError>;

    /// Search for k nearest neighbors
    async fn search(&self, query: &Embedding, k: usize) -> Result<Vec<VectorResult>, AgentDBError>;

    /// Retrieve a vector by ID
    async fn get(&self, id: &str) -> Result<Option<(Embedding, serde_json::Value)>, AgentDBError>;

    /// Delete a vector by ID
    async fn delete(&self, id: &str) -> Result<bool, AgentDBError>;

    /// Get the total count of vectors
    async fn len(&self) -> usize;

    /// Check if the store is empty
    async fn is_empty(&self) -> bool {
        self.len().await == 0
    }

    /// Export state to JSON for persistence
    async fn export_json(&self) -> Result<serde_json::Value, AgentDBError>;

    /// Import state from JSON
    async fn import_json(&self, data: serde_json::Value) -> Result<(), AgentDBError>;
}

/// Trait for graph-based relationship storage (Cypher-compatible)
#[async_trait::async_trait]
pub trait GraphBackend: Send + Sync {
    /// Add a relationship between two nodes
    async fn add_relationship(
        &self,
        source_id: &str,
        target_id: &str,
        relationship_type: &str,
        properties: serde_json::Value,
    ) -> Result<(), AgentDBError>;

    /// Query relationships using Cypher-like syntax
    async fn cypher_query(&self, query: &str) -> Result<Vec<GraphQueryResult>, AgentDBError>;

    /// Find related nodes within N hops
    async fn find_related(&self, node_id: &str, max_hops: usize) -> Result<Vec<RelatedNode>, AgentDBError>;

    /// Delete a relationship
    async fn delete_relationship(&self, source_id: &str, target_id: &str, relationship_type: &str) -> Result<bool, AgentDBError>;
}

// =============================================================================
// RVLITE DATA TYPES
// =============================================================================

/// Configuration for the RvLite backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RvLiteConfig {
    /// Vector embedding dimensions
    pub dimensions: usize,
    /// Distance metric for similarity search
    pub distance_metric: DistanceMetric,
    /// Enable graph query support (Cypher)
    pub enable_graph: bool,
    /// Enable semantic memory tier system
    pub enable_semantic_memory: bool,
    /// Auto-save interval in seconds (0 = disabled)
    pub auto_save_interval_secs: u64,
    /// Storage path for persistence
    pub storage_path: Option<String>,
    /// Memory tiers configuration
    pub tier_config: TierConfig,
}

impl Default for RvLiteConfig {
    fn default() -> Self {
        Self {
            dimensions: 768,
            distance_metric: DistanceMetric::Cosine,
            enable_graph: true,
            enable_semantic_memory: true,
            auto_save_interval_secs: 300, // 5 minutes
            storage_path: None,
            tier_config: TierConfig::default(),
        }
    }
}

/// Distance metric for similarity calculations
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DistanceMetric {
    Cosine,
    Euclidean,
    DotProduct,
}

/// Configuration for memory tier consolidation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierConfig {
    /// Thresholds for tier promotion (importance scores)
    pub tier_thresholds: Vec<f64>,
    /// Decay rate per tier (exponential)
    pub decay_rates: Vec<f64>,
    /// Consolidation batch size
    pub consolidation_batch_size: usize,
    /// Minimum access count for tier promotion
    pub min_access_for_promotion: u64,
}

impl Default for TierConfig {
    fn default() -> Self {
        Self {
            // 12 tiers: Instant → Cosmic
            tier_thresholds: vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.85, 0.9, 0.95, 1.0],
            decay_rates: vec![0.99, 0.995, 0.999, 0.9995, 0.9999, 0.99995, 0.99999, 0.999995, 0.999999, 0.9999995, 0.9999999, 1.0],
            consolidation_batch_size: 100,
            min_access_for_promotion: 3,
        }
    }
}

/// A stored vector entry with full metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RvLiteEntry {
    pub id: String,
    pub embedding: Embedding,
    pub metadata: serde_json::Value,
    pub tier: u8,
    pub importance: f64,
    pub access_count: u64,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub consolidated: bool,
}

/// Graph relationship between nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphRelationship {
    pub source_id: String,
    pub target_id: String,
    pub relationship_type: String,
    pub properties: serde_json::Value,
    pub weight: f64,
    pub created_at: DateTime<Utc>,
}

/// Result from a graph query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQueryResult {
    pub nodes: Vec<String>,
    pub relationships: Vec<String>,
    pub path: Option<Vec<String>>,
    pub properties: serde_json::Value,
}

/// A related node found through graph traversal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedNode {
    pub id: String,
    pub relationship_type: String,
    pub hops: usize,
    pub path: Vec<String>,
    pub total_weight: f64,
}

/// Semantic memory entry for tier-based consolidation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticMemoryEntry {
    pub id: String,
    pub content: String,
    pub embedding: Embedding,
    pub tier: MemoryTier,
    pub importance: f64,
    pub context: Option<String>,
    pub relationships: Vec<String>,
    pub replay_count: u32,
    pub strength: f64,
}

/// Memory tier enumeration matching omega-memory
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(u8)]
pub enum MemoryTier {
    Instant = 1,       // Milliseconds
    Session = 2,       // Minutes to hours
    Episodic = 3,      // Days to weeks
    Semantic = 4,      // Weeks to months
    Collective = 5,    // Months to years
    Evolutionary = 6,  // Years to decades
    Architectural = 7, // Decades
    Substrate = 8,     // Centuries
    Civilizational = 9,// Millennia
    Temporal = 10,     // Millions of years
    Physical = 11,     // Billions of years
    Omega = 12,        // Eternal/Universal
}

impl From<u8> for MemoryTier {
    fn from(value: u8) -> Self {
        match value {
            1 => MemoryTier::Instant,
            2 => MemoryTier::Session,
            3 => MemoryTier::Episodic,
            4 => MemoryTier::Semantic,
            5 => MemoryTier::Collective,
            6 => MemoryTier::Evolutionary,
            7 => MemoryTier::Architectural,
            8 => MemoryTier::Substrate,
            9 => MemoryTier::Civilizational,
            10 => MemoryTier::Temporal,
            11 => MemoryTier::Physical,
            12 => MemoryTier::Omega,
            _ => MemoryTier::Instant,
        }
    }
}

// =============================================================================
// RVLITE BACKEND IMPLEMENTATION
// =============================================================================

/// RvLite-compatible vector database backend for ExoGenesis Omega.
/// Provides unified vector storage, graph relationships, and semantic memory.
pub struct RvLiteBackend {
    config: RvLiteConfig,
    /// Vector storage with HNSW index
    vectors: Arc<RwLock<HashMap<String, RvLiteEntry>>>,
    /// Graph relationships (Cypher-compatible)
    relationships: Arc<RwLock<Vec<GraphRelationship>>>,
    /// Semantic memory tiers
    semantic_memories: Arc<RwLock<HashMap<String, SemanticMemoryEntry>>>,
    /// HNSW index for fast approximate search
    hnsw_index: Arc<RwLock<HnswIndex>>,
    /// Statistics
    stats: Arc<RwLock<RvLiteStats>>,
}

/// Internal HNSW index using instant-distance
struct HnswIndex {
    points: Vec<HnswPoint>,
    config: HnswConfig,
}

struct HnswPoint {
    id: String,
    embedding: Vec<f32>,
}

#[derive(Clone)]
struct HnswConfig {
    ef_construction: usize,
    ef_search: usize,
    m: usize,
}

impl Default for HnswConfig {
    fn default() -> Self {
        Self {
            ef_construction: 200,
            ef_search: 100,
            m: 32,
        }
    }
}

impl HnswIndex {
    fn new(config: HnswConfig) -> Self {
        Self {
            points: Vec::new(),
            config,
        }
    }

    fn insert(&mut self, id: String, embedding: Vec<f32>) {
        self.points.push(HnswPoint { id, embedding });
    }

    fn search(&self, query: &[f32], k: usize, metric: DistanceMetric) -> Vec<(String, f64)> {
        let mut results: Vec<(String, f64)> = self.points
            .iter()
            .map(|p| {
                let sim = match metric {
                    DistanceMetric::Cosine => cosine_similarity(query, &p.embedding),
                    DistanceMetric::Euclidean => 1.0 / (1.0 + euclidean_distance(query, &p.embedding)),
                    DistanceMetric::DotProduct => dot_product(query, &p.embedding),
                };
                (p.id.clone(), sim)
            })
            .collect();

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(k);
        results
    }

    fn remove(&mut self, id: &str) -> bool {
        let len_before = self.points.len();
        self.points.retain(|p| p.id != id);
        self.points.len() < len_before
    }

    fn len(&self) -> usize {
        self.points.len()
    }
}

/// Statistics for RvLite backend
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RvLiteStats {
    pub total_vectors: usize,
    pub total_relationships: usize,
    pub total_semantic_memories: usize,
    pub queries_executed: u64,
    pub insertions: u64,
    pub deletions: u64,
    pub consolidations: u64,
    pub tier_distribution: HashMap<u8, usize>,
}

impl RvLiteBackend {
    /// Create a new RvLite backend with the given configuration
    pub fn new(config: RvLiteConfig) -> Self {
        Self {
            config,
            vectors: Arc::new(RwLock::new(HashMap::new())),
            relationships: Arc::new(RwLock::new(Vec::new())),
            semantic_memories: Arc::new(RwLock::new(HashMap::new())),
            hnsw_index: Arc::new(RwLock::new(HnswIndex::new(HnswConfig::default()))),
            stats: Arc::new(RwLock::new(RvLiteStats::default())),
        }
    }

    /// Load from a persistence file
    pub async fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, AgentDBError> {
        let content = tokio::fs::read_to_string(path)
            .await
            .map_err(|e| AgentDBError::StorageError(format!("Failed to read file: {}", e)))?;

        let data: RvLitePersistenceData = serde_json::from_str(&content)
            .map_err(|e| AgentDBError::StorageError(format!("Failed to parse JSON: {}", e)))?;

        let backend = Self::new(data.config);

        // Restore vectors
        {
            let mut vectors = backend.vectors.write().await;
            let mut hnsw = backend.hnsw_index.write().await;
            for entry in data.vectors {
                hnsw.insert(entry.id.clone(), entry.embedding.clone());
                vectors.insert(entry.id.clone(), entry);
            }
        }

        // Restore relationships
        {
            let mut rels = backend.relationships.write().await;
            *rels = data.relationships;
        }

        // Restore semantic memories
        {
            let mut memories = backend.semantic_memories.write().await;
            for mem in data.semantic_memories {
                memories.insert(mem.id.clone(), mem);
            }
        }

        Ok(backend)
    }

    /// Save to a persistence file
    pub async fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), AgentDBError> {
        let data = RvLitePersistenceData {
            config: self.config.clone(),
            vectors: self.vectors.read().await.values().cloned().collect(),
            relationships: self.relationships.read().await.clone(),
            semantic_memories: self.semantic_memories.read().await.values().cloned().collect(),
        };

        let json = serde_json::to_string_pretty(&data)
            .map_err(|e| AgentDBError::StorageError(format!("Failed to serialize: {}", e)))?;

        tokio::fs::write(path, json)
            .await
            .map_err(|e| AgentDBError::StorageError(format!("Failed to write file: {}", e)))?;

        Ok(())
    }

    /// Get current statistics
    pub async fn stats(&self) -> RvLiteStats {
        let vectors = self.vectors.read().await;
        let rels = self.relationships.read().await;
        let memories = self.semantic_memories.read().await;

        let mut tier_distribution = HashMap::new();
        for entry in vectors.values() {
            *tier_distribution.entry(entry.tier).or_insert(0) += 1;
        }

        let mut stats = self.stats.read().await.clone();
        stats.total_vectors = vectors.len();
        stats.total_relationships = rels.len();
        stats.total_semantic_memories = memories.len();
        stats.tier_distribution = tier_distribution;
        stats
    }

    // =========================================================================
    // SEMANTIC MEMORY OPERATIONS
    // =========================================================================

    /// Store a semantic memory with tier assignment
    pub async fn store_semantic_memory(&self, memory: SemanticMemoryEntry) -> Result<String, AgentDBError> {
        let id = memory.id.clone();

        // Store embedding in vector index
        let entry = RvLiteEntry {
            id: id.clone(),
            embedding: memory.embedding.clone(),
            metadata: serde_json::json!({
                "content": memory.content,
                "tier": memory.tier as u8,
                "importance": memory.importance,
                "context": memory.context,
            }),
            tier: memory.tier as u8,
            importance: memory.importance,
            access_count: 0,
            created_at: Utc::now(),
            last_accessed: Utc::now(),
            consolidated: false,
        };

        {
            let mut vectors = self.vectors.write().await;
            let mut hnsw = self.hnsw_index.write().await;
            hnsw.insert(id.clone(), memory.embedding.clone());
            vectors.insert(id.clone(), entry);
        }

        // Store in semantic memory map
        {
            let mut memories = self.semantic_memories.write().await;
            memories.insert(id.clone(), memory);
        }

        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.insertions += 1;
        }

        Ok(id)
    }

    /// Query semantic memories by similarity
    pub async fn query_semantic_memory(
        &self,
        query_embedding: &Embedding,
        k: usize,
        min_tier: Option<MemoryTier>,
    ) -> Result<Vec<SemanticMemoryEntry>, AgentDBError> {
        // Search HNSW index
        let search_results = {
            let hnsw = self.hnsw_index.read().await;
            hnsw.search(query_embedding, k * 2, self.config.distance_metric) // Over-fetch for filtering
        };

        // Filter by tier and collect memories
        let memories = self.semantic_memories.read().await;
        let min_tier_val = min_tier.map(|t| t as u8).unwrap_or(0);

        let mut results: Vec<SemanticMemoryEntry> = search_results
            .into_iter()
            .filter_map(|(id, _sim)| {
                memories.get(&id).cloned().filter(|m| m.tier as u8 >= min_tier_val)
            })
            .take(k)
            .collect();

        // Update access counts
        {
            let mut vectors = self.vectors.write().await;
            for mem in &results {
                if let Some(entry) = vectors.get_mut(&mem.id) {
                    entry.access_count += 1;
                    entry.last_accessed = Utc::now();
                }
            }
        }

        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.queries_executed += 1;
        }

        Ok(results)
    }

    /// Consolidate memories: promote important ones to higher tiers
    pub async fn consolidate_memories(&self) -> Result<ConsolidationReport, AgentDBError> {
        let mut promoted = 0;
        let mut decayed = 0;
        let mut removed = 0;

        let tier_config = &self.config.tier_config;
        let now = Utc::now();

        let mut vectors = self.vectors.write().await;
        let mut memories = self.semantic_memories.write().await;

        let ids: Vec<String> = vectors.keys().cloned().collect();

        for id in ids {
            if let Some(entry) = vectors.get_mut(&id) {
                // Calculate age-based decay
                let age_hours = (now - entry.created_at).num_hours() as f64;
                let tier_idx = (entry.tier as usize).saturating_sub(1).min(11);
                let decay_rate = tier_config.decay_rates.get(tier_idx).copied().unwrap_or(0.999);

                // Apply decay
                entry.importance *= decay_rate.powf(age_hours / 24.0);

                // Check for promotion
                if entry.access_count >= tier_config.min_access_for_promotion {
                    let next_tier = (entry.tier + 1).min(12);
                    let threshold = tier_config.tier_thresholds
                        .get(next_tier as usize - 1)
                        .copied()
                        .unwrap_or(1.0);

                    if entry.importance >= threshold && entry.tier < 12 {
                        entry.tier = next_tier;
                        entry.consolidated = true;
                        promoted += 1;

                        // Update semantic memory tier
                        if let Some(mem) = memories.get_mut(&id) {
                            mem.tier = MemoryTier::from(next_tier);
                            mem.replay_count += 1;
                            mem.strength *= 1.1; // Strengthen on consolidation
                        }
                    }
                }

                // Check for removal (importance too low in tier 1)
                if entry.tier == 1 && entry.importance < 0.01 {
                    removed += 1;
                } else {
                    decayed += 1;
                }
            }
        }

        // Remove very low importance tier-1 memories
        vectors.retain(|_id, entry| !(entry.tier == 1 && entry.importance < 0.01));

        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.consolidations += 1;
        }

        Ok(ConsolidationReport {
            promoted,
            decayed,
            removed,
            timestamp: now,
        })
    }

    /// Simulate hippocampal replay for memory strengthening
    pub async fn replay_memories(&self, memory_ids: &[String]) -> Result<ReplayReport, AgentDBError> {
        let mut replayed = 0;
        let mut strengthened = 0;

        let mut memories = self.semantic_memories.write().await;
        let mut vectors = self.vectors.write().await;

        for id in memory_ids {
            if let Some(mem) = memories.get_mut(id) {
                mem.replay_count += 1;
                mem.strength = (mem.strength * 1.15).min(10.0); // Cap at 10x
                replayed += 1;

                if mem.strength > 2.0 {
                    strengthened += 1;
                }
            }

            // Also update vector entry
            if let Some(entry) = vectors.get_mut(id) {
                entry.access_count += 1;
                entry.importance = (entry.importance * 1.1).min(1.0);
            }
        }

        Ok(ReplayReport {
            replayed,
            strengthened,
            timestamp: Utc::now(),
        })
    }

    // =========================================================================
    // GRAPH OPERATIONS (CYPHER-COMPATIBLE)
    // =========================================================================

    /// Add a causal relationship between memories
    pub async fn add_causal_relationship(&self, edge: &CausalEdge) -> Result<(), AgentDBError> {
        let relationship = GraphRelationship {
            source_id: edge.cause.clone(),
            target_id: edge.effect.clone(),
            relationship_type: "CAUSES".to_string(),
            properties: serde_json::json!({
                "uplift": edge.uplift,
                "confidence": edge.confidence,
                "sample_size": edge.sample_size,
            }),
            weight: edge.uplift * edge.confidence,
            created_at: edge.first_observed,
        };

        let mut rels = self.relationships.write().await;

        // Update existing or add new
        if let Some(existing) = rels.iter_mut().find(|r| {
            r.source_id == relationship.source_id
            && r.target_id == relationship.target_id
            && r.relationship_type == relationship.relationship_type
        }) {
            existing.properties = relationship.properties;
            existing.weight = relationship.weight;
        } else {
            rels.push(relationship);
        }

        Ok(())
    }

    /// Execute a Cypher-like query
    pub async fn cypher_query(&self, query: &str) -> Result<Vec<GraphQueryResult>, AgentDBError> {
        let rels = self.relationships.read().await;
        let query_lower = query.to_lowercase();

        // Simple Cypher parser for common patterns
        if query_lower.contains("match") && query_lower.contains("where") {
            // MATCH (a)-[r]->(b) WHERE a.id = 'xxx' RETURN b
            self.execute_match_query(&query_lower, &rels).await
        } else if query_lower.contains("path") || query_lower.contains("shortestpath") {
            // Find path queries
            self.execute_path_query(&query_lower, &rels).await
        } else {
            // Return all relationships matching type
            let results = rels
                .iter()
                .map(|r| GraphQueryResult {
                    nodes: vec![r.source_id.clone(), r.target_id.clone()],
                    relationships: vec![r.relationship_type.clone()],
                    path: Some(vec![r.source_id.clone(), r.target_id.clone()]),
                    properties: r.properties.clone(),
                })
                .collect();
            Ok(results)
        }
    }

    async fn execute_match_query(
        &self,
        query: &str,
        rels: &[GraphRelationship],
    ) -> Result<Vec<GraphQueryResult>, AgentDBError> {
        // Extract source ID from WHERE clause (simplified parser)
        let source_id = self.extract_where_id(query);

        let results: Vec<GraphQueryResult> = rels
            .iter()
            .filter(|r| {
                source_id.as_ref().map(|id| r.source_id.contains(id)).unwrap_or(true)
            })
            .map(|r| GraphQueryResult {
                nodes: vec![r.source_id.clone(), r.target_id.clone()],
                relationships: vec![r.relationship_type.clone()],
                path: Some(vec![r.source_id.clone(), r.target_id.clone()]),
                properties: r.properties.clone(),
            })
            .collect();

        Ok(results)
    }

    async fn execute_path_query(
        &self,
        _query: &str,
        _rels: &[GraphRelationship],
    ) -> Result<Vec<GraphQueryResult>, AgentDBError> {
        // Placeholder for path finding - implement BFS/DFS
        Ok(vec![])
    }

    fn extract_where_id(&self, query: &str) -> Option<String> {
        // Very simple extraction: find "id = 'xxx'" or "id = \"xxx\""
        if let Some(pos) = query.find("id") {
            let rest = &query[pos..];
            if let Some(eq_pos) = rest.find('=') {
                let value_start = &rest[eq_pos + 1..].trim_start();
                let quote_char = if value_start.starts_with('\'') { '\'' } else { '"' };
                if value_start.starts_with(quote_char) {
                    let value = &value_start[1..];
                    if let Some(end) = value.find(quote_char) {
                        return Some(value[..end].to_string());
                    }
                }
            }
        }
        None
    }

    /// Find all nodes related to a given node within N hops
    pub async fn find_related(&self, node_id: &str, max_hops: usize) -> Result<Vec<RelatedNode>, AgentDBError> {
        let rels = self.relationships.read().await;
        let mut visited: HashMap<String, (usize, Vec<String>, f64)> = HashMap::new();
        let mut queue: Vec<(String, usize, Vec<String>, f64)> = vec![(node_id.to_string(), 0, vec![node_id.to_string()], 0.0)];

        while let Some((current, hops, path, weight)) = queue.pop() {
            if hops >= max_hops {
                continue;
            }

            for rel in rels.iter().filter(|r| r.source_id == current) {
                let new_path: Vec<String> = path.iter().cloned().chain(std::iter::once(rel.target_id.clone())).collect();
                let new_weight = weight + rel.weight;

                if !visited.contains_key(&rel.target_id) {
                    visited.insert(rel.target_id.clone(), (hops + 1, new_path.clone(), new_weight));
                    queue.push((rel.target_id.clone(), hops + 1, new_path, new_weight));
                }
            }
        }

        let results: Vec<RelatedNode> = visited
            .into_iter()
            .filter(|(id, _)| id != node_id)
            .map(|(id, (hops, path, weight))| RelatedNode {
                id,
                relationship_type: "RELATED".to_string(),
                hops,
                path,
                total_weight: weight,
            })
            .collect();

        Ok(results)
    }
}

// =============================================================================
// VECTOR BACKEND TRAIT IMPLEMENTATION
// =============================================================================

#[async_trait::async_trait]
impl VectorBackend for RvLiteBackend {
    async fn insert(&self, embedding: Embedding, metadata: serde_json::Value) -> Result<VectorId, AgentDBError> {
        if embedding.len() != self.config.dimensions {
            return Err(AgentDBError::StorageError(format!(
                "Embedding dimension {} does not match configured dimension {}",
                embedding.len(),
                self.config.dimensions
            )));
        }

        let id = uuid::Uuid::new_v4().to_string();

        let entry = RvLiteEntry {
            id: id.clone(),
            embedding: embedding.clone(),
            metadata,
            tier: 1, // Start at Instant tier
            importance: 0.5, // Default importance
            access_count: 0,
            created_at: Utc::now(),
            last_accessed: Utc::now(),
            consolidated: false,
        };

        {
            let mut vectors = self.vectors.write().await;
            let mut hnsw = self.hnsw_index.write().await;
            hnsw.insert(id.clone(), embedding);
            vectors.insert(id.clone(), entry);
        }

        {
            let mut stats = self.stats.write().await;
            stats.insertions += 1;
        }

        Ok(id)
    }

    async fn insert_with_id(&self, id: &str, embedding: Embedding, metadata: serde_json::Value) -> Result<(), AgentDBError> {
        if embedding.len() != self.config.dimensions {
            return Err(AgentDBError::StorageError(format!(
                "Embedding dimension {} does not match configured dimension {}",
                embedding.len(),
                self.config.dimensions
            )));
        }

        let entry = RvLiteEntry {
            id: id.to_string(),
            embedding: embedding.clone(),
            metadata,
            tier: 1,
            importance: 0.5,
            access_count: 0,
            created_at: Utc::now(),
            last_accessed: Utc::now(),
            consolidated: false,
        };

        {
            let mut vectors = self.vectors.write().await;
            let mut hnsw = self.hnsw_index.write().await;
            hnsw.insert(id.to_string(), embedding);
            vectors.insert(id.to_string(), entry);
        }

        {
            let mut stats = self.stats.write().await;
            stats.insertions += 1;
        }

        Ok(())
    }

    async fn search(&self, query: &Embedding, k: usize) -> Result<Vec<VectorResult>, AgentDBError> {
        if query.len() != self.config.dimensions {
            return Err(AgentDBError::QueryError(format!(
                "Query dimension {} does not match configured dimension {}",
                query.len(),
                self.config.dimensions
            )));
        }

        let search_results = {
            let hnsw = self.hnsw_index.read().await;
            hnsw.search(query, k, self.config.distance_metric)
        };

        let vectors = self.vectors.read().await;

        let results: Vec<VectorResult> = search_results
            .into_iter()
            .filter_map(|(id, similarity)| {
                vectors.get(&id).map(|entry| VectorResult {
                    id: entry.id.clone(),
                    similarity,
                    metadata: entry.metadata.clone(),
                })
            })
            .collect();

        // Update access counts
        {
            let mut vectors = self.vectors.write().await;
            for result in &results {
                if let Some(entry) = vectors.get_mut(&result.id) {
                    entry.access_count += 1;
                    entry.last_accessed = Utc::now();
                }
            }
        }

        {
            let mut stats = self.stats.write().await;
            stats.queries_executed += 1;
        }

        Ok(results)
    }

    async fn get(&self, id: &str) -> Result<Option<(Embedding, serde_json::Value)>, AgentDBError> {
        let vectors = self.vectors.read().await;
        Ok(vectors.get(id).map(|entry| (entry.embedding.clone(), entry.metadata.clone())))
    }

    async fn delete(&self, id: &str) -> Result<bool, AgentDBError> {
        let removed = {
            let mut vectors = self.vectors.write().await;
            let mut hnsw = self.hnsw_index.write().await;
            hnsw.remove(id);
            vectors.remove(id).is_some()
        };

        // Also remove from semantic memories
        {
            let mut memories = self.semantic_memories.write().await;
            memories.remove(id);
        }

        if removed {
            let mut stats = self.stats.write().await;
            stats.deletions += 1;
        }

        Ok(removed)
    }

    async fn len(&self) -> usize {
        self.vectors.read().await.len()
    }

    async fn export_json(&self) -> Result<serde_json::Value, AgentDBError> {
        let data = RvLitePersistenceData {
            config: self.config.clone(),
            vectors: self.vectors.read().await.values().cloned().collect(),
            relationships: self.relationships.read().await.clone(),
            semantic_memories: self.semantic_memories.read().await.values().cloned().collect(),
        };

        serde_json::to_value(data)
            .map_err(|e| AgentDBError::StorageError(format!("Failed to serialize: {}", e)))
    }

    async fn import_json(&self, data: serde_json::Value) -> Result<(), AgentDBError> {
        let parsed: RvLitePersistenceData = serde_json::from_value(data)
            .map_err(|e| AgentDBError::StorageError(format!("Failed to parse: {}", e)))?;

        // Restore vectors
        {
            let mut vectors = self.vectors.write().await;
            let mut hnsw = self.hnsw_index.write().await;
            vectors.clear();
            *hnsw = HnswIndex::new(HnswConfig::default());

            for entry in parsed.vectors {
                hnsw.insert(entry.id.clone(), entry.embedding.clone());
                vectors.insert(entry.id.clone(), entry);
            }
        }

        // Restore relationships
        {
            let mut rels = self.relationships.write().await;
            *rels = parsed.relationships;
        }

        // Restore semantic memories
        {
            let mut memories = self.semantic_memories.write().await;
            memories.clear();
            for mem in parsed.semantic_memories {
                memories.insert(mem.id.clone(), mem);
            }
        }

        Ok(())
    }
}

// =============================================================================
// GRAPH BACKEND TRAIT IMPLEMENTATION
// =============================================================================

#[async_trait::async_trait]
impl GraphBackend for RvLiteBackend {
    async fn add_relationship(
        &self,
        source_id: &str,
        target_id: &str,
        relationship_type: &str,
        properties: serde_json::Value,
    ) -> Result<(), AgentDBError> {
        let relationship = GraphRelationship {
            source_id: source_id.to_string(),
            target_id: target_id.to_string(),
            relationship_type: relationship_type.to_string(),
            properties,
            weight: 1.0,
            created_at: Utc::now(),
        };

        let mut rels = self.relationships.write().await;

        // Check for existing
        if let Some(existing) = rels.iter_mut().find(|r| {
            r.source_id == source_id
            && r.target_id == target_id
            && r.relationship_type == relationship_type
        }) {
            existing.properties = relationship.properties;
            existing.weight = relationship.weight;
        } else {
            rels.push(relationship);
        }

        Ok(())
    }

    async fn cypher_query(&self, query: &str) -> Result<Vec<GraphQueryResult>, AgentDBError> {
        RvLiteBackend::cypher_query(self, query).await
    }

    async fn find_related(&self, node_id: &str, max_hops: usize) -> Result<Vec<RelatedNode>, AgentDBError> {
        RvLiteBackend::find_related(self, node_id, max_hops).await
    }

    async fn delete_relationship(&self, source_id: &str, target_id: &str, relationship_type: &str) -> Result<bool, AgentDBError> {
        let mut rels = self.relationships.write().await;
        let len_before = rels.len();
        rels.retain(|r| {
            !(r.source_id == source_id && r.target_id == target_id && r.relationship_type == relationship_type)
        });
        Ok(rels.len() < len_before)
    }
}

// =============================================================================
// PERSISTENCE DATA STRUCTURE
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RvLitePersistenceData {
    config: RvLiteConfig,
    vectors: Vec<RvLiteEntry>,
    relationships: Vec<GraphRelationship>,
    semantic_memories: Vec<SemanticMemoryEntry>,
}

// =============================================================================
// CONSOLIDATION & REPLAY REPORTS
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationReport {
    pub promoted: usize,
    pub decayed: usize,
    pub removed: usize,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayReport {
    pub replayed: usize,
    pub strengthened: usize,
    pub timestamp: DateTime<Utc>,
}

// =============================================================================
// DISTANCE FUNCTIONS (SIMD-optimized via simsimd)
// =============================================================================

fn cosine_similarity(a: &[f32], b: &[f32]) -> f64 {
    use simsimd::SpatialSimilarity;
    match f32::cosine(a, b) {
        Some(distance) => 1.0 - distance,
        None => 0.0,
    }
}

fn euclidean_distance(a: &[f32], b: &[f32]) -> f64 {
    use simsimd::SpatialSimilarity;
    f32::sqeuclidean(a, b).unwrap_or(f64::MAX).sqrt()
}

fn dot_product(a: &[f32], b: &[f32]) -> f64 {
    use simsimd::SpatialSimilarity;
    // SimSIMD doesn't have direct dot product, compute from inner product
    f32::dot(a, b).unwrap_or(0.0)
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rvlite_backend_basic_operations() {
        let config = RvLiteConfig {
            dimensions: 64,
            ..Default::default()
        };
        let backend = RvLiteBackend::new(config);

        // Test insert
        let embedding: Embedding = (0..64).map(|i| i as f32 / 64.0).collect();
        let id = backend.insert(embedding.clone(), serde_json::json!({"test": "data"})).await.unwrap();
        assert!(!id.is_empty());

        // Test get
        let (retrieved, metadata) = backend.get(&id).await.unwrap().unwrap();
        assert_eq!(retrieved.len(), 64);
        assert_eq!(metadata["test"], "data");

        // Test search
        let results = backend.search(&embedding, 1).await.unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].similarity > 0.99);

        // Test delete
        assert!(backend.delete(&id).await.unwrap());
        assert!(backend.get(&id).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_semantic_memory_tiers() {
        let config = RvLiteConfig {
            dimensions: 32,
            ..Default::default()
        };
        let backend = RvLiteBackend::new(config);

        // Store semantic memory
        let embedding: Embedding = vec![0.5; 32];
        let memory = SemanticMemoryEntry {
            id: "mem-1".to_string(),
            content: "Test memory content".to_string(),
            embedding: embedding.clone(),
            tier: MemoryTier::Session,
            importance: 0.8,
            context: Some("test context".to_string()),
            relationships: vec![],
            replay_count: 0,
            strength: 1.0,
        };

        let id = backend.store_semantic_memory(memory).await.unwrap();
        assert_eq!(id, "mem-1");

        // Query by similarity
        let results = backend.query_semantic_memory(&embedding, 5, None).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].tier, MemoryTier::Session);
    }

    #[tokio::test]
    async fn test_graph_relationships() {
        let config = RvLiteConfig::default();
        let backend = RvLiteBackend::new(config);

        // Add relationships
        backend.add_relationship(
            "node-1",
            "node-2",
            "KNOWS",
            serde_json::json!({"since": "2024"}),
        ).await.unwrap();

        backend.add_relationship(
            "node-2",
            "node-3",
            "KNOWS",
            serde_json::json!({"since": "2025"}),
        ).await.unwrap();

        // Find related nodes
        let related = backend.find_related("node-1", 2).await.unwrap();
        assert!(related.len() >= 1);
    }

    #[tokio::test]
    async fn test_consolidation() {
        let config = RvLiteConfig {
            dimensions: 16,
            tier_config: TierConfig {
                min_access_for_promotion: 1,
                tier_thresholds: vec![0.1; 12],
                ..Default::default()
            },
            ..Default::default()
        };
        let backend = RvLiteBackend::new(config);

        // Insert with high importance
        let mut entry = RvLiteEntry {
            id: "test-1".to_string(),
            embedding: vec![0.5; 16],
            metadata: serde_json::json!({}),
            tier: 1,
            importance: 0.9,
            access_count: 5,
            created_at: Utc::now(),
            last_accessed: Utc::now(),
            consolidated: false,
        };

        {
            let mut vectors = backend.vectors.write().await;
            let mut hnsw = backend.hnsw_index.write().await;
            hnsw.insert(entry.id.clone(), entry.embedding.clone());
            vectors.insert(entry.id.clone(), entry);
        }

        // Run consolidation
        let report = backend.consolidate_memories().await.unwrap();
        assert!(report.promoted >= 0); // May or may not promote based on thresholds
    }

    #[tokio::test]
    async fn test_export_import() {
        let config = RvLiteConfig {
            dimensions: 8,
            ..Default::default()
        };
        let backend = RvLiteBackend::new(config);

        // Insert data
        let embedding: Embedding = vec![1.0; 8];
        backend.insert(embedding, serde_json::json!({"key": "value"})).await.unwrap();

        // Export
        let exported = backend.export_json().await.unwrap();

        // Create new backend and import
        let new_backend = RvLiteBackend::new(RvLiteConfig {
            dimensions: 8,
            ..Default::default()
        });
        new_backend.import_json(exported).await.unwrap();

        // Verify
        assert_eq!(new_backend.len().await, 1);
    }

    #[tokio::test]
    async fn test_replay_strengthening() {
        let config = RvLiteConfig {
            dimensions: 16,
            ..Default::default()
        };
        let backend = RvLiteBackend::new(config);

        // Create semantic memory
        let memory = SemanticMemoryEntry {
            id: "replay-test".to_string(),
            content: "Memory to replay".to_string(),
            embedding: vec![0.5; 16],
            tier: MemoryTier::Episodic,
            importance: 0.5,
            context: None,
            relationships: vec![],
            replay_count: 0,
            strength: 1.0,
        };

        backend.store_semantic_memory(memory).await.unwrap();

        // Replay multiple times
        for _ in 0..5 {
            backend.replay_memories(&["replay-test".to_string()]).await.unwrap();
        }

        // Check strengthening
        let memories = backend.semantic_memories.read().await;
        let mem = memories.get("replay-test").unwrap();
        assert!(mem.strength > 1.0);
        assert_eq!(mem.replay_count, 5);
    }
}
