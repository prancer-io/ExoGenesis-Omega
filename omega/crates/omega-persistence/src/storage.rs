use chrono::{DateTime, Utc};
use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;
use uuid::Uuid;

use crate::schema;

/// Custom error types for storage operations
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),
}

pub type Result<T> = std::result::Result<T, StorageError>;

/// Stored memory record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredMemory {
    pub id: String,
    pub content: String,
    pub tier: i32,
    pub importance: f64,
    pub embedding_blob: Option<Vec<u8>>,
    pub created_at: i64,
    pub last_accessed: i64,
}

/// Stored skill record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredSkill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub trigger_pattern: String,
    pub success_count: i32,
    pub last_used: Option<i64>,
    pub created_at: i64,
}

/// Stored architecture record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredArchitecture {
    pub id: String,
    pub name: String,
    pub paradigm: String,
    pub substrate: String,
    pub fitness_json: String,
    pub lineage_json: String,
    pub created_at: i64,
}

/// Stored intelligence record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredIntelligence {
    pub id: String,
    pub name: String,
    pub arch_id: String,
    pub maturity: f64,
    pub capabilities_json: String,
    pub memories_json: String,
    pub state_json: String,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Stored vector record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredVector {
    pub id: String,
    pub memory_id: String,
    pub dimensions: i32,
    pub data_blob: Vec<u8>,
}

/// Stored reflexion episode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredReflexionEpisode {
    pub id: String,
    pub memory_id: String,
    pub trigger: String,
    pub context: String,
    pub action: String,
    pub outcome: String,
    pub created_at: i64,
}

/// Stored causal edge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredCausalEdge {
    pub id: String,
    pub from_memory: String,
    pub to_memory: String,
    pub weight: f64,
    pub edge_type: String,
    pub created_at: i64,
}

/// Main storage interface for ExoGenesis Omega
pub struct OmegaStore {
    conn: Connection,
}

impl OmegaStore {
    /// Create or open a database at the given path
    ///
    /// # Arguments
    /// * `path` - Path to the SQLite database file
    ///
    /// # Returns
    /// * `Result<Self>` - New OmegaStore instance or error
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;

        // Enable foreign keys
        conn.execute("PRAGMA foreign_keys = ON;", [])?;

        // Run all schema migrations
        for schema_sql in schema::ALL_SCHEMAS {
            conn.execute(schema_sql, [])?;
        }

        Ok(Self { conn })
    }

    /// Create an in-memory database (useful for testing)
    pub fn new_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute("PRAGMA foreign_keys = ON;", [])?;

        for schema_sql in schema::ALL_SCHEMAS {
            conn.execute(schema_sql, [])?;
        }

        Ok(Self { conn })
    }

    // ===== MEMORY OPERATIONS =====

    /// Store a memory in the database
    pub fn store_memory(&self, memory: &StoredMemory) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO memories
             (id, content, tier, importance, embedding_blob, created_at, last_accessed)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                &memory.id,
                &memory.content,
                memory.tier,
                memory.importance,
                &memory.embedding_blob,
                memory.created_at,
                memory.last_accessed,
            ],
        )?;
        Ok(())
    }

    /// Retrieve a memory by ID
    pub fn get_memory(&self, id: &str) -> Result<StoredMemory> {
        let memory = self.conn.query_row(
            "SELECT id, content, tier, importance, embedding_blob, created_at, last_accessed
             FROM memories WHERE id = ?1",
            params![id],
            |row| {
                Ok(StoredMemory {
                    id: row.get(0)?,
                    content: row.get(1)?,
                    tier: row.get(2)?,
                    importance: row.get(3)?,
                    embedding_blob: row.get(4)?,
                    created_at: row.get(5)?,
                    last_accessed: row.get(6)?,
                })
            },
        ).optional()?
        .ok_or_else(|| StorageError::NotFound(format!("Memory not found: {}", id)))?;

        Ok(memory)
    }

    /// Query memories by tier
    pub fn query_memories_by_tier(&self, tier: i32) -> Result<Vec<StoredMemory>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, content, tier, importance, embedding_blob, created_at, last_accessed
             FROM memories WHERE tier = ?1 ORDER BY importance DESC"
        )?;

        let memories = stmt.query_map(params![tier], |row| {
            Ok(StoredMemory {
                id: row.get(0)?,
                content: row.get(1)?,
                tier: row.get(2)?,
                importance: row.get(3)?,
                embedding_blob: row.get(4)?,
                created_at: row.get(5)?,
                last_accessed: row.get(6)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(memories)
    }

    /// Update memory last accessed timestamp
    pub fn update_memory_access(&self, id: &str, timestamp: i64) -> Result<()> {
        let rows = self.conn.execute(
            "UPDATE memories SET last_accessed = ?1 WHERE id = ?2",
            params![timestamp, id],
        )?;

        if rows == 0 {
            return Err(StorageError::NotFound(format!("Memory not found: {}", id)));
        }

        Ok(())
    }

    // ===== SKILL OPERATIONS =====

    /// Store a skill in the database
    pub fn store_skill(&self, skill: &StoredSkill) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO skills
             (id, name, description, trigger_pattern, success_count, last_used, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                &skill.id,
                &skill.name,
                &skill.description,
                &skill.trigger_pattern,
                skill.success_count,
                skill.last_used,
                skill.created_at,
            ],
        )?;
        Ok(())
    }

    /// Retrieve a skill by ID
    pub fn get_skill(&self, id: &str) -> Result<StoredSkill> {
        let skill = self.conn.query_row(
            "SELECT id, name, description, trigger_pattern, success_count, last_used, created_at
             FROM skills WHERE id = ?1",
            params![id],
            |row| {
                Ok(StoredSkill {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    trigger_pattern: row.get(3)?,
                    success_count: row.get(4)?,
                    last_used: row.get(5)?,
                    created_at: row.get(6)?,
                })
            },
        ).optional()?
        .ok_or_else(|| StorageError::NotFound(format!("Skill not found: {}", id)))?;

        Ok(skill)
    }

    /// Get skills matching a trigger pattern
    pub fn get_skills_by_pattern(&self, pattern: &str) -> Result<Vec<StoredSkill>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, trigger_pattern, success_count, last_used, created_at
             FROM skills WHERE trigger_pattern LIKE ?1 ORDER BY success_count DESC"
        )?;

        let pattern_query = format!("%{}%", pattern);
        let skills = stmt.query_map(params![pattern_query], |row| {
            Ok(StoredSkill {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                trigger_pattern: row.get(3)?,
                success_count: row.get(4)?,
                last_used: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(skills)
    }

    /// Increment skill success count
    pub fn increment_skill_success(&self, id: &str, timestamp: i64) -> Result<()> {
        let rows = self.conn.execute(
            "UPDATE skills SET success_count = success_count + 1, last_used = ?1 WHERE id = ?2",
            params![timestamp, id],
        )?;

        if rows == 0 {
            return Err(StorageError::NotFound(format!("Skill not found: {}", id)));
        }

        Ok(())
    }

    // ===== ARCHITECTURE OPERATIONS =====

    /// Store an architecture in the database
    pub fn store_architecture(&self, arch: &StoredArchitecture) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO architectures
             (id, name, paradigm, substrate, fitness_json, lineage_json, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                &arch.id,
                &arch.name,
                &arch.paradigm,
                &arch.substrate,
                &arch.fitness_json,
                &arch.lineage_json,
                arch.created_at,
            ],
        )?;
        Ok(())
    }

    /// Retrieve an architecture by ID
    pub fn get_architecture(&self, id: &str) -> Result<StoredArchitecture> {
        let arch = self.conn.query_row(
            "SELECT id, name, paradigm, substrate, fitness_json, lineage_json, created_at
             FROM architectures WHERE id = ?1",
            params![id],
            |row| {
                Ok(StoredArchitecture {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    paradigm: row.get(2)?,
                    substrate: row.get(3)?,
                    fitness_json: row.get(4)?,
                    lineage_json: row.get(5)?,
                    created_at: row.get(6)?,
                })
            },
        ).optional()?
        .ok_or_else(|| StorageError::NotFound(format!("Architecture not found: {}", id)))?;

        Ok(arch)
    }

    /// Get all architectures by paradigm
    pub fn get_architectures_by_paradigm(&self, paradigm: &str) -> Result<Vec<StoredArchitecture>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, paradigm, substrate, fitness_json, lineage_json, created_at
             FROM architectures WHERE paradigm = ?1 ORDER BY created_at DESC"
        )?;

        let archs = stmt.query_map(params![paradigm], |row| {
            Ok(StoredArchitecture {
                id: row.get(0)?,
                name: row.get(1)?,
                paradigm: row.get(2)?,
                substrate: row.get(3)?,
                fitness_json: row.get(4)?,
                lineage_json: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(archs)
    }

    // ===== INTELLIGENCE OPERATIONS =====

    /// Store an intelligence in the database
    pub fn store_intelligence(&self, intel: &StoredIntelligence) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO intelligences
             (id, name, arch_id, maturity, capabilities_json, memories_json, state_json, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                &intel.id,
                &intel.name,
                &intel.arch_id,
                intel.maturity,
                &intel.capabilities_json,
                &intel.memories_json,
                &intel.state_json,
                intel.created_at,
                intel.updated_at,
            ],
        )?;
        Ok(())
    }

    /// Retrieve an intelligence by ID
    pub fn get_intelligence(&self, id: &str) -> Result<StoredIntelligence> {
        let intel = self.conn.query_row(
            "SELECT id, name, arch_id, maturity, capabilities_json, memories_json, state_json, created_at, updated_at
             FROM intelligences WHERE id = ?1",
            params![id],
            |row| {
                Ok(StoredIntelligence {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    arch_id: row.get(2)?,
                    maturity: row.get(3)?,
                    capabilities_json: row.get(4)?,
                    memories_json: row.get(5)?,
                    state_json: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                })
            },
        ).optional()?
        .ok_or_else(|| StorageError::NotFound(format!("Intelligence not found: {}", id)))?;

        Ok(intel)
    }

    /// Get intelligences by architecture ID
    pub fn get_intelligences_by_arch(&self, arch_id: &str) -> Result<Vec<StoredIntelligence>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, arch_id, maturity, capabilities_json, memories_json, state_json, created_at, updated_at
             FROM intelligences WHERE arch_id = ?1 ORDER BY maturity DESC"
        )?;

        let intels = stmt.query_map(params![arch_id], |row| {
            Ok(StoredIntelligence {
                id: row.get(0)?,
                name: row.get(1)?,
                arch_id: row.get(2)?,
                maturity: row.get(3)?,
                capabilities_json: row.get(4)?,
                memories_json: row.get(5)?,
                state_json: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(intels)
    }

    // ===== VECTOR OPERATIONS =====

    /// Store a vector embedding
    pub fn store_vector(&self, vector: &StoredVector) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO vectors (id, memory_id, dimensions, data_blob)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                &vector.id,
                &vector.memory_id,
                vector.dimensions,
                &vector.data_blob,
            ],
        )?;
        Ok(())
    }

    /// Get vector by memory ID
    pub fn get_vector_by_memory(&self, memory_id: &str) -> Result<StoredVector> {
        let vector = self.conn.query_row(
            "SELECT id, memory_id, dimensions, data_blob FROM vectors WHERE memory_id = ?1",
            params![memory_id],
            |row| {
                Ok(StoredVector {
                    id: row.get(0)?,
                    memory_id: row.get(1)?,
                    dimensions: row.get(2)?,
                    data_blob: row.get(3)?,
                })
            },
        ).optional()?
        .ok_or_else(|| StorageError::NotFound(format!("Vector not found for memory: {}", memory_id)))?;

        Ok(vector)
    }

    // ===== REFLEXION OPERATIONS =====

    /// Store a reflexion episode
    pub fn store_reflexion(&self, episode: &StoredReflexionEpisode) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO reflexion_episodes
             (id, memory_id, trigger, context, action, outcome, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                &episode.id,
                &episode.memory_id,
                &episode.trigger,
                &episode.context,
                &episode.action,
                &episode.outcome,
                episode.created_at,
            ],
        )?;
        Ok(())
    }

    /// Get reflexion episodes by memory ID
    pub fn get_reflexions_by_memory(&self, memory_id: &str) -> Result<Vec<StoredReflexionEpisode>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, memory_id, trigger, context, action, outcome, created_at
             FROM reflexion_episodes WHERE memory_id = ?1 ORDER BY created_at DESC"
        )?;

        let episodes = stmt.query_map(params![memory_id], |row| {
            Ok(StoredReflexionEpisode {
                id: row.get(0)?,
                memory_id: row.get(1)?,
                trigger: row.get(2)?,
                context: row.get(3)?,
                action: row.get(4)?,
                outcome: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(episodes)
    }

    // ===== CAUSAL EDGE OPERATIONS =====

    /// Store a causal edge
    pub fn store_causal_edge(&self, edge: &StoredCausalEdge) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO causal_edges
             (id, from_memory, to_memory, weight, edge_type, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                &edge.id,
                &edge.from_memory,
                &edge.to_memory,
                edge.weight,
                &edge.edge_type,
                edge.created_at,
            ],
        )?;
        Ok(())
    }

    /// Get outgoing causal edges from a memory
    pub fn get_causal_edges_from(&self, memory_id: &str) -> Result<Vec<StoredCausalEdge>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, from_memory, to_memory, weight, edge_type, created_at
             FROM causal_edges WHERE from_memory = ?1 ORDER BY weight DESC"
        )?;

        let edges = stmt.query_map(params![memory_id], |row| {
            Ok(StoredCausalEdge {
                id: row.get(0)?,
                from_memory: row.get(1)?,
                to_memory: row.get(2)?,
                weight: row.get(3)?,
                edge_type: row.get(4)?,
                created_at: row.get(5)?,
            })
        })?
        .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(edges)
    }

    // ===== BACKUP OPERATIONS =====

    /// Backup the database to a specified path
    pub fn backup(&self, backup_path: &str) -> Result<()> {
        let mut backup_conn = Connection::open(backup_path)?;
        let backup = rusqlite::backup::Backup::new(&self.conn, &mut backup_conn)?;
        backup.run_to_completion(5, std::time::Duration::from_millis(250), None)?;
        Ok(())
    }

    /// Get database statistics
    pub fn get_statistics(&self) -> Result<DatabaseStatistics> {
        let memory_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM memories",
            [],
            |row| row.get(0),
        )?;

        let skill_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM skills",
            [],
            |row| row.get(0),
        )?;

        let architecture_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM architectures",
            [],
            |row| row.get(0),
        )?;

        let intelligence_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM intelligences",
            [],
            |row| row.get(0),
        )?;

        let causal_edge_count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM causal_edges",
            [],
            |row| row.get(0),
        )?;

        Ok(DatabaseStatistics {
            memory_count,
            skill_count,
            architecture_count,
            intelligence_count,
            causal_edge_count,
        })
    }
}

/// Database statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseStatistics {
    pub memory_count: i64,
    pub skill_count: i64,
    pub architecture_count: i64,
    pub intelligence_count: i64,
    pub causal_edge_count: i64,
}
