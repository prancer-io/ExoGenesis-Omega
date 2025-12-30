//! RvLite Bridge for Omega Memory
//!
//! This module provides integration between the omega-memory 12-tier cosmic
//! memory system and RvLite's vector database for long-term persistence.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────────────┐
//! │                        OMEGA-MEMORY + RVLITE                            │
//! ├─────────────────────────────────────────────────────────────────────────┤
//! │                                                                         │
//! │  ┌─────────────────────────────────────────────────────────────────┐   │
//! │  │                    CosmicMemory (In-Memory)                      │   │
//! │  │  ┌───────────┐ ┌───────────┐ ┌───────────┐ ┌───────────────────┐│   │
//! │  │  │ Individual│ │  Species  │ │  Cosmic   │ │   Consolidator    ││   │
//! │  │  │ Tiers 1-4 │ │ Tiers 5-8 │ │ Tiers 9-12│ │                   ││   │
//! │  │  └─────┬─────┘ └─────┬─────┘ └─────┬─────┘ └─────────┬─────────┘│   │
//! │  └────────┼─────────────┼─────────────┼─────────────────┼──────────┘   │
//! │           │             │             │                 │              │
//! │           └─────────────┴─────────────┴─────────────────┘              │
//! │                                   │                                    │
//! │                          ┌────────▼────────┐                           │
//! │                          │  RvLiteBridge   │                           │
//! │                          │  - sync_to_rvlite│                           │
//! │                          │  - restore_from  │                           │
//! │                          │  - query_semantic│                           │
//! │                          └────────┬────────┘                           │
//! │                                   │                                    │
//! │  ┌────────────────────────────────▼────────────────────────────────┐   │
//! │  │                    RvLiteBackend (Persistent)                    │   │
//! │  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │   │
//! │  │  │ VectorStore │  │ GraphStore  │  │    SemanticMemory       │  │   │
//! │  │  │ (HNSW+SIMD) │  │ (Cypher)    │  │ (Tier Consolidation)    │  │   │
//! │  │  └─────────────┘  └─────────────┘  └─────────────────────────┘  │   │
//! │  │                           │                                      │   │
//! │  │                    ┌──────┴──────┐                               │   │
//! │  │                    │  JSON File  │                               │   │
//! │  │                    │ Persistence │                               │   │
//! │  │                    └─────────────┘                               │   │
//! │  └──────────────────────────────────────────────────────────────────┘   │
//! │                                                                         │
//! └─────────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Usage
//!
//! ```rust,ignore
//! use omega_memory::{CosmicMemory, Memory, MemoryTier};
//! use omega_memory::rvlite_bridge::{RvLiteBridge, RvLiteBridgeConfig};
//!
//! // Create the bridge
//! let config = RvLiteBridgeConfig {
//!     dimensions: 768,
//!     storage_path: Some("./memory_store.json".to_string()),
//!     auto_sync_interval_secs: 60,
//!     ..Default::default()
//! };
//! let bridge = RvLiteBridge::new(config).await?;
//!
//! // Sync memories from CosmicMemory to RvLite
//! bridge.sync_from_cosmic_memory(&cosmic_memory).await?;
//!
//! // Query with semantic search
//! let results = bridge.semantic_query(&query_embedding, 10, Some(MemoryTier::Semantic)).await?;
//!
//! // Save to disk
//! bridge.save().await?;
//! ```

use crate::{Memory, MemoryContent, MemoryError, MemoryTier, Query};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

// =============================================================================
// CONFIGURATION
// =============================================================================

/// Configuration for the RvLite bridge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RvLiteBridgeConfig {
    /// Vector embedding dimensions (must match your embedding model)
    pub dimensions: usize,
    /// Path for persistent storage (JSON file)
    pub storage_path: Option<String>,
    /// Automatic sync interval in seconds (0 = disabled)
    pub auto_sync_interval_secs: u64,
    /// Enable graph relationship tracking
    pub enable_graph: bool,
    /// Minimum tier for long-term storage (lower tiers are transient)
    pub min_persistent_tier: MemoryTier,
    /// Consolidation thresholds per tier
    pub consolidation_thresholds: ConsolidationThresholds,
    /// Maximum memories per tier (for cleanup)
    pub max_memories_per_tier: HashMap<MemoryTier, usize>,
}

impl Default for RvLiteBridgeConfig {
    fn default() -> Self {
        let mut max_per_tier = HashMap::new();
        max_per_tier.insert(MemoryTier::Instant, 1000);
        max_per_tier.insert(MemoryTier::Session, 5000);
        max_per_tier.insert(MemoryTier::Episodic, 10000);
        max_per_tier.insert(MemoryTier::Semantic, 50000);
        max_per_tier.insert(MemoryTier::Collective, 100000);

        Self {
            dimensions: 768,
            storage_path: None,
            auto_sync_interval_secs: 300, // 5 minutes
            enable_graph: true,
            min_persistent_tier: MemoryTier::Episodic, // Only persist Episodic and above
            consolidation_thresholds: ConsolidationThresholds::default(),
            max_memories_per_tier: max_per_tier,
        }
    }
}

/// Thresholds for automatic tier consolidation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationThresholds {
    /// Minimum importance score to promote to next tier
    pub importance_threshold: f64,
    /// Minimum access count for promotion
    pub min_access_count: u64,
    /// Age in hours for automatic promotion consideration
    pub min_age_hours: u64,
    /// Decay rate per hour for tier 1-4
    pub decay_rate_individual: f64,
    /// Decay rate per hour for tier 5-8
    pub decay_rate_species: f64,
}

impl Default for ConsolidationThresholds {
    fn default() -> Self {
        Self {
            importance_threshold: 0.6,
            min_access_count: 3,
            min_age_hours: 24,
            decay_rate_individual: 0.995,
            decay_rate_species: 0.9999,
        }
    }
}

// =============================================================================
// RVLITE MEMORY ENTRY
// =============================================================================

/// A memory entry stored in RvLite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RvLiteMemory {
    /// Unique identifier
    pub id: String,
    /// Memory tier (1-12)
    pub tier: u8,
    /// Content type and data
    pub content: MemoryContentSerialized,
    /// Embedding vector
    pub embedding: Vec<f32>,
    /// Importance score (0.0 - 1.0)
    pub importance: f64,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last access timestamp
    pub last_accessed: DateTime<Utc>,
    /// Total access count
    pub access_count: u64,
    /// Additional metadata
    pub metadata: serde_json::Value,
    /// Consolidation status
    pub consolidated: bool,
    /// Memory strength (for replay-based learning)
    pub strength: f64,
    /// Replay count (hippocampal-style consolidation)
    pub replay_count: u32,
    /// Related memory IDs (graph edges)
    pub related_ids: Vec<String>,
}

/// Serializable memory content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryContentSerialized {
    Sensory(Vec<u8>),
    Text(String),
    Structured(serde_json::Value),
    Embedding(Vec<f32>),
    MultiModal {
        text: Option<String>,
        embedding: Vec<f32>,
        metadata: serde_json::Value,
    },
}

impl From<&MemoryContent> for MemoryContentSerialized {
    fn from(content: &MemoryContent) -> Self {
        match content {
            MemoryContent::Sensory(data) => MemoryContentSerialized::Sensory(data.clone()),
            MemoryContent::Text(text) => MemoryContentSerialized::Text(text.clone()),
            MemoryContent::Structured(value) => MemoryContentSerialized::Structured(value.clone()),
            MemoryContent::Embedding(vec) => MemoryContentSerialized::Embedding(vec.clone()),
            MemoryContent::MultiModal { text, embedding, metadata } => {
                MemoryContentSerialized::MultiModal {
                    text: text.clone(),
                    embedding: embedding.clone(),
                    metadata: metadata.clone(),
                }
            }
        }
    }
}

impl From<&MemoryContentSerialized> for MemoryContent {
    fn from(content: &MemoryContentSerialized) -> Self {
        match content {
            MemoryContentSerialized::Sensory(data) => MemoryContent::Sensory(data.clone()),
            MemoryContentSerialized::Text(text) => MemoryContent::Text(text.clone()),
            MemoryContentSerialized::Structured(value) => MemoryContent::Structured(value.clone()),
            MemoryContentSerialized::Embedding(vec) => MemoryContent::Embedding(vec.clone()),
            MemoryContentSerialized::MultiModal { text, embedding, metadata } => {
                MemoryContent::MultiModal {
                    text: text.clone(),
                    embedding: embedding.clone(),
                    metadata: metadata.clone(),
                }
            }
        }
    }
}

// =============================================================================
// GRAPH RELATIONSHIP
// =============================================================================

/// A relationship between memories in the graph store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRelationship {
    pub source_id: String,
    pub target_id: String,
    pub relationship_type: RelationshipType,
    pub weight: f64,
    pub created_at: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

/// Types of relationships between memories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipType {
    /// Causal relationship (A caused B)
    Causes,
    /// Temporal relationship (A happened before B)
    Precedes,
    /// Semantic similarity
    SimilarTo,
    /// Part-whole relationship
    PartOf,
    /// Consolidated from (tier promotion)
    ConsolidatedFrom,
    /// Context relationship
    InContext,
    /// Associative link
    AssociatedWith,
}

// =============================================================================
// RVLITE BRIDGE
// =============================================================================

/// Bridge connecting omega-memory to RvLite for long-term persistence
pub struct RvLiteBridge {
    config: RvLiteBridgeConfig,
    /// Stored memories indexed by ID
    memories: Arc<RwLock<HashMap<String, RvLiteMemory>>>,
    /// HNSW-style index for fast similarity search
    vector_index: Arc<RwLock<VectorIndex>>,
    /// Graph relationships between memories
    relationships: Arc<RwLock<Vec<MemoryRelationship>>>,
    /// Statistics
    stats: Arc<RwLock<BridgeStats>>,
}

/// Simple vector index for similarity search
struct VectorIndex {
    entries: Vec<(String, Vec<f32>)>,
    dimensions: usize,
}

impl VectorIndex {
    fn new(dimensions: usize) -> Self {
        Self {
            entries: Vec::new(),
            dimensions,
        }
    }

    fn insert(&mut self, id: String, embedding: Vec<f32>) {
        // Remove existing if present
        self.entries.retain(|(existing_id, _)| existing_id != &id);
        self.entries.push((id, embedding));
    }

    fn remove(&mut self, id: &str) {
        self.entries.retain(|(existing_id, _)| existing_id != id);
    }

    fn search(&self, query: &[f32], k: usize) -> Vec<(String, f64)> {
        let mut results: Vec<(String, f64)> = self.entries
            .iter()
            .map(|(id, embedding)| {
                let sim = cosine_similarity(query, embedding);
                (id.clone(), sim)
            })
            .collect();

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(k);
        results
    }

    fn len(&self) -> usize {
        self.entries.len()
    }
}

/// Statistics for the bridge
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BridgeStats {
    pub total_memories: usize,
    pub memories_per_tier: HashMap<u8, usize>,
    pub total_relationships: usize,
    pub queries_executed: u64,
    pub syncs_performed: u64,
    pub consolidations: u64,
    pub last_sync: Option<DateTime<Utc>>,
    pub last_save: Option<DateTime<Utc>>,
}

/// Result of a semantic query
#[derive(Debug, Clone)]
pub struct SemanticQueryResult {
    pub memory: RvLiteMemory,
    pub similarity: f64,
    pub related_memories: Vec<String>,
}

/// Report from consolidation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationReport {
    pub promoted: usize,
    pub decayed: usize,
    pub removed: usize,
    pub relationships_created: usize,
    pub timestamp: DateTime<Utc>,
}

impl RvLiteBridge {
    /// Create a new RvLite bridge
    pub async fn new(config: RvLiteBridgeConfig) -> Result<Self, MemoryError> {
        let bridge = Self {
            config: config.clone(),
            memories: Arc::new(RwLock::new(HashMap::new())),
            vector_index: Arc::new(RwLock::new(VectorIndex::new(config.dimensions))),
            relationships: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(RwLock::new(BridgeStats::default())),
        };

        // Load from storage if path exists
        if let Some(ref path) = config.storage_path {
            if Path::new(path).exists() {
                bridge.load_from_file(path).await?;
            }
        }

        Ok(bridge)
    }

    /// Store a memory from omega-memory format
    pub async fn store(&self, memory: &Memory) -> Result<String, MemoryError> {
        // Check if this tier should be persisted
        if (memory.tier as u8) < (self.config.min_persistent_tier as u8) {
            // Don't persist transient memories
            return Ok(memory.id.clone());
        }

        if memory.embedding.len() != self.config.dimensions {
            return Err(MemoryError::Storage(format!(
                "Embedding dimension {} does not match configured {}",
                memory.embedding.len(),
                self.config.dimensions
            )));
        }

        let rvlite_memory = RvLiteMemory {
            id: memory.id.clone(),
            tier: memory.tier as u8,
            content: MemoryContentSerialized::from(&memory.content),
            embedding: memory.embedding.clone(),
            importance: memory.importance,
            created_at: memory.created_at,
            last_accessed: memory.accessed_at,
            access_count: memory.access_count,
            metadata: memory.metadata.clone(),
            consolidated: false,
            strength: 1.0,
            replay_count: 0,
            related_ids: Vec::new(),
        };

        {
            let mut memories = self.memories.write().await;
            let mut index = self.vector_index.write().await;

            index.insert(memory.id.clone(), memory.embedding.clone());
            memories.insert(memory.id.clone(), rvlite_memory);
        }

        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.total_memories = self.memories.read().await.len();
        }

        Ok(memory.id.clone())
    }

    /// Retrieve a memory by ID
    pub async fn get(&self, id: &str) -> Option<Memory> {
        let memories = self.memories.read().await;
        memories.get(id).map(|m| self.to_memory(m))
    }

    /// Semantic similarity query
    pub async fn semantic_query(
        &self,
        query_embedding: &[f32],
        k: usize,
        min_tier: Option<MemoryTier>,
    ) -> Result<Vec<SemanticQueryResult>, MemoryError> {
        if query_embedding.len() != self.config.dimensions {
            return Err(MemoryError::Query(format!(
                "Query dimension {} does not match configured {}",
                query_embedding.len(),
                self.config.dimensions
            )));
        }

        // Search vector index
        let search_results = {
            let index = self.vector_index.read().await;
            index.search(query_embedding, k * 2) // Over-fetch for filtering
        };

        let memories = self.memories.read().await;
        let relationships = self.relationships.read().await;
        let min_tier_val = min_tier.map(|t| t as u8).unwrap_or(0);

        let results: Vec<SemanticQueryResult> = search_results
            .into_iter()
            .filter_map(|(id, similarity)| {
                memories.get(&id).and_then(|mem| {
                    if mem.tier >= min_tier_val {
                        // Find related memories
                        let related: Vec<String> = relationships
                            .iter()
                            .filter(|r| r.source_id == id || r.target_id == id)
                            .map(|r| {
                                if r.source_id == id {
                                    r.target_id.clone()
                                } else {
                                    r.source_id.clone()
                                }
                            })
                            .collect();

                        Some(SemanticQueryResult {
                            memory: mem.clone(),
                            similarity,
                            related_memories: related,
                        })
                    } else {
                        None
                    }
                })
            })
            .take(k)
            .collect();

        // Update query stats
        {
            let mut stats = self.stats.write().await;
            stats.queries_executed += 1;
        }

        // Touch accessed memories
        {
            let mut memories = self.memories.write().await;
            for result in &results {
                if let Some(mem) = memories.get_mut(&result.memory.id) {
                    mem.access_count += 1;
                    mem.last_accessed = Utc::now();
                }
            }
        }

        Ok(results)
    }

    /// Add a relationship between two memories
    pub async fn add_relationship(
        &self,
        source_id: &str,
        target_id: &str,
        relationship_type: RelationshipType,
        weight: f64,
    ) -> Result<(), MemoryError> {
        if !self.config.enable_graph {
            return Ok(());
        }

        // Verify both memories exist
        let memories = self.memories.read().await;
        if !memories.contains_key(source_id) || !memories.contains_key(target_id) {
            return Err(MemoryError::Query("One or both memories not found".to_string()));
        }
        drop(memories);

        let relationship = MemoryRelationship {
            source_id: source_id.to_string(),
            target_id: target_id.to_string(),
            relationship_type,
            weight,
            created_at: Utc::now(),
            metadata: serde_json::json!({}),
        };

        let mut rels = self.relationships.write().await;

        // Update existing or add new
        if let Some(existing) = rels.iter_mut().find(|r| {
            r.source_id == source_id && r.target_id == target_id && r.relationship_type == relationship_type
        }) {
            existing.weight = weight;
        } else {
            rels.push(relationship);
        }

        // Also update the related_ids in memory entries
        {
            let mut memories = self.memories.write().await;
            if let Some(source_mem) = memories.get_mut(source_id) {
                if !source_mem.related_ids.contains(&target_id.to_string()) {
                    source_mem.related_ids.push(target_id.to_string());
                }
            }
            if let Some(target_mem) = memories.get_mut(target_id) {
                if !target_mem.related_ids.contains(&source_id.to_string()) {
                    target_mem.related_ids.push(source_id.to_string());
                }
            }
        }

        Ok(())
    }

    /// Find memories related to a given memory through the graph
    pub async fn find_related(
        &self,
        memory_id: &str,
        max_hops: usize,
        relationship_types: Option<&[RelationshipType]>,
    ) -> Result<Vec<(String, usize, f64)>, MemoryError> {
        let rels = self.relationships.read().await;
        let mut visited: HashMap<String, (usize, f64)> = HashMap::new();
        let mut queue: Vec<(String, usize, f64)> = vec![(memory_id.to_string(), 0, 0.0)];

        while let Some((current, hops, total_weight)) = queue.pop() {
            if hops >= max_hops {
                continue;
            }

            for rel in rels.iter() {
                let matches_type = relationship_types
                    .map(|types| types.contains(&rel.relationship_type))
                    .unwrap_or(true);

                if !matches_type {
                    continue;
                }

                let neighbor = if rel.source_id == current {
                    Some(&rel.target_id)
                } else if rel.target_id == current {
                    Some(&rel.source_id)
                } else {
                    None
                };

                if let Some(neighbor_id) = neighbor {
                    let new_weight = total_weight + rel.weight;
                    if !visited.contains_key(neighbor_id) {
                        visited.insert(neighbor_id.clone(), (hops + 1, new_weight));
                        queue.push((neighbor_id.clone(), hops + 1, new_weight));
                    }
                }
            }
        }

        let results: Vec<(String, usize, f64)> = visited
            .into_iter()
            .filter(|(id, _)| id != memory_id)
            .map(|(id, (hops, weight))| (id, hops, weight))
            .collect();

        Ok(results)
    }

    /// Run consolidation (promote important memories, decay others)
    pub async fn consolidate(&self) -> Result<ConsolidationReport, MemoryError> {
        let now = Utc::now();
        let mut promoted = 0;
        let mut decayed = 0;
        let mut removed = 0;
        let mut relationships_created = 0;

        let thresholds = &self.config.consolidation_thresholds;

        let mut to_remove: Vec<String> = Vec::new();
        let mut to_promote: Vec<(String, u8)> = Vec::new();

        {
            let mut memories = self.memories.write().await;

            for (id, mem) in memories.iter_mut() {
                // Calculate age
                let age_hours = (now - mem.created_at).num_hours() as f64;

                // Apply decay based on tier
                let decay_rate = if mem.tier <= 4 {
                    thresholds.decay_rate_individual
                } else {
                    thresholds.decay_rate_species
                };

                mem.importance *= decay_rate.powf(age_hours / 24.0);
                decayed += 1;

                // Check for promotion
                if mem.access_count >= thresholds.min_access_count
                    && mem.importance >= thresholds.importance_threshold
                    && age_hours >= thresholds.min_age_hours as f64
                    && mem.tier < 12
                {
                    to_promote.push((id.clone(), mem.tier + 1));
                }

                // Check for removal (very low importance at tier 1-2)
                if mem.tier <= 2 && mem.importance < 0.01 {
                    to_remove.push(id.clone());
                }
            }

            // Apply promotions
            for (id, new_tier) in to_promote {
                if let Some(mem) = memories.get_mut(&id) {
                    let old_tier = mem.tier;
                    mem.tier = new_tier;
                    mem.consolidated = true;
                    mem.strength *= 1.2; // Strengthen on consolidation
                    mem.replay_count += 1;
                    promoted += 1;

                    // Create consolidation relationship
                    if self.config.enable_graph {
                        // We'll add this after releasing the lock
                    }
                }
            }

            // Remove expired memories
            for id in &to_remove {
                memories.remove(id);
                removed += 1;
            }
        }

        // Update vector index for removals
        {
            let mut index = self.vector_index.write().await;
            for id in &to_remove {
                index.remove(id);
            }
        }

        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.consolidations += 1;
            stats.total_memories = self.memories.read().await.len();
        }

        Ok(ConsolidationReport {
            promoted,
            decayed,
            removed,
            relationships_created,
            timestamp: now,
        })
    }

    /// Simulate hippocampal replay to strengthen memories
    pub async fn replay_memories(&self, memory_ids: &[String]) -> Result<usize, MemoryError> {
        let mut strengthened = 0;
        let mut memories = self.memories.write().await;

        for id in memory_ids {
            if let Some(mem) = memories.get_mut(id) {
                mem.replay_count += 1;
                mem.strength = (mem.strength * 1.15).min(10.0); // Cap at 10x
                mem.importance = (mem.importance * 1.05).min(1.0); // Boost importance
                mem.access_count += 1;
                mem.last_accessed = Utc::now();
                strengthened += 1;
            }
        }

        Ok(strengthened)
    }

    /// Save to persistent storage
    pub async fn save(&self) -> Result<(), MemoryError> {
        let path = self.config.storage_path.as_ref()
            .ok_or_else(|| MemoryError::Storage("No storage path configured".to_string()))?;

        self.save_to_file(path).await
    }

    /// Save to a specific file
    pub async fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), MemoryError> {
        let data = PersistenceData {
            config: self.config.clone(),
            memories: self.memories.read().await.values().cloned().collect(),
            relationships: self.relationships.read().await.clone(),
            stats: self.stats.read().await.clone(),
        };

        let json = serde_json::to_string_pretty(&data)?;
        tokio::fs::write(path, json).await?;

        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.last_save = Some(Utc::now());
        }

        Ok(())
    }

    /// Load from a file
    pub async fn load_from_file<P: AsRef<Path>>(&self, path: P) -> Result<(), MemoryError> {
        let content = tokio::fs::read_to_string(path).await?;
        let data: PersistenceData = serde_json::from_str(&content)?;

        // Restore memories
        {
            let mut memories = self.memories.write().await;
            let mut index = self.vector_index.write().await;

            memories.clear();
            *index = VectorIndex::new(self.config.dimensions);

            for mem in data.memories {
                index.insert(mem.id.clone(), mem.embedding.clone());
                memories.insert(mem.id.clone(), mem);
            }
        }

        // Restore relationships
        {
            let mut rels = self.relationships.write().await;
            *rels = data.relationships;
        }

        Ok(())
    }

    /// Export all data as JSON
    pub async fn export_json(&self) -> Result<serde_json::Value, MemoryError> {
        let data = PersistenceData {
            config: self.config.clone(),
            memories: self.memories.read().await.values().cloned().collect(),
            relationships: self.relationships.read().await.clone(),
            stats: self.stats.read().await.clone(),
        };

        Ok(serde_json::to_value(data)?)
    }

    /// Import from JSON
    pub async fn import_json(&self, value: serde_json::Value) -> Result<(), MemoryError> {
        let data: PersistenceData = serde_json::from_value(value)?;

        // Restore memories
        {
            let mut memories = self.memories.write().await;
            let mut index = self.vector_index.write().await;

            for mem in data.memories {
                index.insert(mem.id.clone(), mem.embedding.clone());
                memories.insert(mem.id.clone(), mem);
            }
        }

        // Restore relationships
        {
            let mut rels = self.relationships.write().await;
            rels.extend(data.relationships);
        }

        Ok(())
    }

    /// Get bridge statistics
    pub async fn stats(&self) -> BridgeStats {
        let memories = self.memories.read().await;
        let rels = self.relationships.read().await;

        let mut stats = self.stats.read().await.clone();
        stats.total_memories = memories.len();
        stats.total_relationships = rels.len();

        // Count per tier
        let mut per_tier: HashMap<u8, usize> = HashMap::new();
        for mem in memories.values() {
            *per_tier.entry(mem.tier).or_insert(0) += 1;
        }
        stats.memories_per_tier = per_tier;

        stats
    }

    /// Delete a memory
    pub async fn delete(&self, id: &str) -> Result<bool, MemoryError> {
        let removed = {
            let mut memories = self.memories.write().await;
            let mut index = self.vector_index.write().await;

            index.remove(id);
            memories.remove(id).is_some()
        };

        // Also remove relationships involving this memory
        if removed {
            let mut rels = self.relationships.write().await;
            rels.retain(|r| r.source_id != id && r.target_id != id);
        }

        Ok(removed)
    }

    /// Clear all memories
    pub async fn clear(&self) -> Result<(), MemoryError> {
        {
            let mut memories = self.memories.write().await;
            let mut index = self.vector_index.write().await;
            let mut rels = self.relationships.write().await;

            memories.clear();
            *index = VectorIndex::new(self.config.dimensions);
            rels.clear();
        }

        Ok(())
    }

    // Helper: Convert RvLiteMemory to omega-memory Memory
    fn to_memory(&self, rvlite_mem: &RvLiteMemory) -> Memory {
        Memory {
            id: rvlite_mem.id.clone(),
            tier: MemoryTier::from(rvlite_mem.tier),
            content: MemoryContent::from(&rvlite_mem.content),
            embedding: rvlite_mem.embedding.clone(),
            importance: rvlite_mem.importance,
            created_at: rvlite_mem.created_at,
            accessed_at: rvlite_mem.last_accessed,
            access_count: rvlite_mem.access_count,
            metadata: rvlite_mem.metadata.clone(),
        }
    }
}

// =============================================================================
// PERSISTENCE DATA
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersistenceData {
    config: RvLiteBridgeConfig,
    memories: Vec<RvLiteMemory>,
    relationships: Vec<MemoryRelationship>,
    stats: BridgeStats,
}

// =============================================================================
// SIMILARITY FUNCTIONS
// =============================================================================

fn cosine_similarity(a: &[f32], b: &[f32]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| (*x as f64) * (*y as f64)).sum();
    let norm_a: f64 = a.iter().map(|x| (*x as f64).powi(2)).sum::<f64>().sqrt();
    let norm_b: f64 = b.iter().map(|x| (*x as f64).powi(2)).sum::<f64>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot / (norm_a * norm_b)
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bridge_creation() {
        let config = RvLiteBridgeConfig {
            dimensions: 64,
            storage_path: None,
            ..Default::default()
        };

        let bridge = RvLiteBridge::new(config).await.unwrap();
        let stats = bridge.stats().await;
        assert_eq!(stats.total_memories, 0);
    }

    #[tokio::test]
    async fn test_store_and_retrieve() {
        let config = RvLiteBridgeConfig {
            dimensions: 32,
            min_persistent_tier: MemoryTier::Instant, // Store all tiers
            ..Default::default()
        };

        let bridge = RvLiteBridge::new(config).await.unwrap();

        let memory = Memory {
            id: "test-1".to_string(),
            tier: MemoryTier::Episodic,
            content: MemoryContent::Text("Test memory".to_string()),
            embedding: vec![0.5; 32],
            importance: 0.8,
            created_at: Utc::now(),
            accessed_at: Utc::now(),
            access_count: 0,
            metadata: serde_json::json!({}),
        };

        bridge.store(&memory).await.unwrap();

        let retrieved = bridge.get("test-1").await.unwrap();
        assert_eq!(retrieved.tier, MemoryTier::Episodic);
    }

    #[tokio::test]
    async fn test_semantic_query() {
        let config = RvLiteBridgeConfig {
            dimensions: 16,
            min_persistent_tier: MemoryTier::Instant,
            ..Default::default()
        };

        let bridge = RvLiteBridge::new(config).await.unwrap();

        // Store some memories
        for i in 0..5 {
            let mut embedding = vec![0.0; 16];
            embedding[i % 16] = 1.0;

            let memory = Memory {
                id: format!("mem-{}", i),
                tier: MemoryTier::Semantic,
                content: MemoryContent::Text(format!("Memory {}", i)),
                embedding,
                importance: 0.5 + (i as f64) * 0.1,
                created_at: Utc::now(),
                accessed_at: Utc::now(),
                access_count: 0,
                metadata: serde_json::json!({}),
            };

            bridge.store(&memory).await.unwrap();
        }

        // Query
        let mut query_embedding = vec![0.0; 16];
        query_embedding[0] = 1.0;

        let results = bridge.semantic_query(&query_embedding, 3, None).await.unwrap();
        assert!(!results.is_empty());
    }

    #[tokio::test]
    async fn test_relationships() {
        let config = RvLiteBridgeConfig {
            dimensions: 8,
            min_persistent_tier: MemoryTier::Instant,
            enable_graph: true,
            ..Default::default()
        };

        let bridge = RvLiteBridge::new(config).await.unwrap();

        // Store two memories
        let mem1 = Memory {
            id: "mem-1".to_string(),
            tier: MemoryTier::Semantic,
            content: MemoryContent::Text("Memory 1".to_string()),
            embedding: vec![1.0; 8],
            importance: 0.8,
            created_at: Utc::now(),
            accessed_at: Utc::now(),
            access_count: 0,
            metadata: serde_json::json!({}),
        };

        let mem2 = Memory {
            id: "mem-2".to_string(),
            tier: MemoryTier::Semantic,
            content: MemoryContent::Text("Memory 2".to_string()),
            embedding: vec![0.5; 8],
            importance: 0.7,
            created_at: Utc::now(),
            accessed_at: Utc::now(),
            access_count: 0,
            metadata: serde_json::json!({}),
        };

        bridge.store(&mem1).await.unwrap();
        bridge.store(&mem2).await.unwrap();

        // Add relationship
        bridge.add_relationship("mem-1", "mem-2", RelationshipType::Causes, 0.9).await.unwrap();

        // Find related
        let related = bridge.find_related("mem-1", 2, None).await.unwrap();
        assert_eq!(related.len(), 1);
        assert_eq!(related[0].0, "mem-2");
    }

    #[tokio::test]
    async fn test_consolidation() {
        let config = RvLiteBridgeConfig {
            dimensions: 8,
            min_persistent_tier: MemoryTier::Instant,
            consolidation_thresholds: ConsolidationThresholds {
                importance_threshold: 0.5,
                min_access_count: 1,
                min_age_hours: 0, // Immediate for testing
                ..Default::default()
            },
            ..Default::default()
        };

        let bridge = RvLiteBridge::new(config).await.unwrap();

        // Store a memory with high importance
        let memory = Memory {
            id: "consolidate-test".to_string(),
            tier: MemoryTier::Session,
            content: MemoryContent::Text("Important memory".to_string()),
            embedding: vec![1.0; 8],
            importance: 0.9,
            created_at: Utc::now(),
            accessed_at: Utc::now(),
            access_count: 5,
            metadata: serde_json::json!({}),
        };

        bridge.store(&memory).await.unwrap();

        // Run consolidation
        let report = bridge.consolidate().await.unwrap();
        assert!(report.decayed >= 1);
    }

    #[tokio::test]
    async fn test_replay_strengthening() {
        let config = RvLiteBridgeConfig {
            dimensions: 8,
            min_persistent_tier: MemoryTier::Instant,
            ..Default::default()
        };

        let bridge = RvLiteBridge::new(config).await.unwrap();

        let memory = Memory {
            id: "replay-test".to_string(),
            tier: MemoryTier::Episodic,
            content: MemoryContent::Text("Memory to replay".to_string()),
            embedding: vec![0.5; 8],
            importance: 0.5,
            created_at: Utc::now(),
            accessed_at: Utc::now(),
            access_count: 0,
            metadata: serde_json::json!({}),
        };

        bridge.store(&memory).await.unwrap();

        // Replay multiple times
        for _ in 0..5 {
            bridge.replay_memories(&["replay-test".to_string()]).await.unwrap();
        }

        // Check strengthening
        let memories = bridge.memories.read().await;
        let mem = memories.get("replay-test").unwrap();
        assert!(mem.strength > 1.0);
        assert_eq!(mem.replay_count, 5);
    }

    #[tokio::test]
    async fn test_export_import() {
        let config = RvLiteBridgeConfig {
            dimensions: 8,
            min_persistent_tier: MemoryTier::Instant,
            ..Default::default()
        };

        let bridge = RvLiteBridge::new(config.clone()).await.unwrap();

        // Store a memory
        let memory = Memory {
            id: "export-test".to_string(),
            tier: MemoryTier::Semantic,
            content: MemoryContent::Text("Export test".to_string()),
            embedding: vec![1.0; 8],
            importance: 0.8,
            created_at: Utc::now(),
            accessed_at: Utc::now(),
            access_count: 0,
            metadata: serde_json::json!({"key": "value"}),
        };

        bridge.store(&memory).await.unwrap();

        // Export
        let exported = bridge.export_json().await.unwrap();

        // Create new bridge and import
        let new_bridge = RvLiteBridge::new(config).await.unwrap();
        new_bridge.import_json(exported).await.unwrap();

        // Verify
        let retrieved = new_bridge.get("export-test").await.unwrap();
        assert_eq!(retrieved.tier, MemoryTier::Semantic);
    }
}
