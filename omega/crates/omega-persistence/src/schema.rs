/// SQL schema constants for the ExoGenesis Omega persistence layer
/// Schema for the memories table
/// Stores hierarchical memory tiers with embeddings and metadata
pub const MEMORIES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS memories (
    id TEXT PRIMARY KEY,
    content TEXT NOT NULL,
    tier INTEGER NOT NULL,
    importance REAL NOT NULL,
    embedding_blob BLOB,
    created_at INTEGER NOT NULL,
    last_accessed INTEGER NOT NULL
);
"#;

/// Index on memory tier for fast tier-based queries
pub const MEMORIES_TIER_INDEX: &str = r#"
CREATE INDEX IF NOT EXISTS idx_memories_tier ON memories(tier);
"#;

/// Index on importance for priority-based retrieval
pub const MEMORIES_IMPORTANCE_INDEX: &str = r#"
CREATE INDEX IF NOT EXISTS idx_memories_importance ON memories(importance DESC);
"#;

/// Schema for the vectors table
/// Stores high-dimensional vector embeddings for similarity search
pub const VECTORS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS vectors (
    id TEXT PRIMARY KEY,
    memory_id TEXT NOT NULL,
    dimensions INTEGER NOT NULL,
    data_blob BLOB NOT NULL,
    FOREIGN KEY(memory_id) REFERENCES memories(id) ON DELETE CASCADE
);
"#;

/// Index on memory_id for fast vector lookups
pub const VECTORS_MEMORY_INDEX: &str = r#"
CREATE INDEX IF NOT EXISTS idx_vectors_memory ON vectors(memory_id);
"#;

/// Schema for the reflexion_episodes table
/// Stores reflexive learning episodes linking triggers to outcomes
pub const REFLEXION_EPISODES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS reflexion_episodes (
    id TEXT PRIMARY KEY,
    memory_id TEXT NOT NULL,
    trigger TEXT NOT NULL,
    context TEXT NOT NULL,
    action TEXT NOT NULL,
    outcome TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY(memory_id) REFERENCES memories(id) ON DELETE CASCADE
);
"#;

/// Index on memory_id for episode retrieval
pub const REFLEXION_MEMORY_INDEX: &str = r#"
CREATE INDEX IF NOT EXISTS idx_reflexion_memory ON reflexion_episodes(memory_id);
"#;

/// Schema for the skills table
/// Stores learned skills with trigger patterns and usage statistics
pub const SKILLS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS skills (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    trigger_pattern TEXT NOT NULL,
    success_count INTEGER NOT NULL DEFAULT 0,
    last_used INTEGER,
    created_at INTEGER NOT NULL
);
"#;

/// Index on trigger pattern for pattern matching
pub const SKILLS_PATTERN_INDEX: &str = r#"
CREATE INDEX IF NOT EXISTS idx_skills_pattern ON skills(trigger_pattern);
"#;

/// Index on success count for selecting best skills
pub const SKILLS_SUCCESS_INDEX: &str = r#"
CREATE INDEX IF NOT EXISTS idx_skills_success ON skills(success_count DESC);
"#;

/// Schema for the causal_edges table
/// Stores causal relationships between memories
pub const CAUSAL_EDGES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS causal_edges (
    id TEXT PRIMARY KEY,
    from_memory TEXT NOT NULL,
    to_memory TEXT NOT NULL,
    weight REAL NOT NULL,
    edge_type TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY(from_memory) REFERENCES memories(id) ON DELETE CASCADE,
    FOREIGN KEY(to_memory) REFERENCES memories(id) ON DELETE CASCADE
);
"#;

/// Index on from_memory for graph traversal
pub const CAUSAL_FROM_INDEX: &str = r#"
CREATE INDEX IF NOT EXISTS idx_causal_from ON causal_edges(from_memory);
"#;

/// Index on to_memory for reverse graph traversal
pub const CAUSAL_TO_INDEX: &str = r#"
CREATE INDEX IF NOT EXISTS idx_causal_to ON causal_edges(to_memory);
"#;

/// Schema for the architectures table
/// Stores evolved architectures with paradigm and fitness metadata
pub const ARCHITECTURES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS architectures (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    paradigm TEXT NOT NULL,
    substrate TEXT NOT NULL,
    fitness_json TEXT NOT NULL,
    lineage_json TEXT NOT NULL,
    created_at INTEGER NOT NULL
);
"#;

/// Index on paradigm for filtering architectures by type
pub const ARCHITECTURES_PARADIGM_INDEX: &str = r#"
CREATE INDEX IF NOT EXISTS idx_architectures_paradigm ON architectures(paradigm);
"#;

/// Schema for the intelligences table
/// Stores intelligence instances with capabilities and state
pub const INTELLIGENCES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS intelligences (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    arch_id TEXT NOT NULL,
    maturity REAL NOT NULL,
    capabilities_json TEXT NOT NULL,
    memories_json TEXT NOT NULL,
    state_json TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY(arch_id) REFERENCES architectures(id) ON DELETE RESTRICT
);
"#;

/// Index on architecture id for finding intelligences by architecture
pub const INTELLIGENCES_ARCH_INDEX: &str = r#"
CREATE INDEX IF NOT EXISTS idx_intelligences_arch ON intelligences(arch_id);
"#;

/// Index on maturity for selecting mature intelligences
pub const INTELLIGENCES_MATURITY_INDEX: &str = r#"
CREATE INDEX IF NOT EXISTS idx_intelligences_maturity ON intelligences(maturity DESC);
"#;

/// Full schema initialization sequence
pub const ALL_SCHEMAS: &[&str] = &[
    MEMORIES_TABLE,
    MEMORIES_TIER_INDEX,
    MEMORIES_IMPORTANCE_INDEX,
    VECTORS_TABLE,
    VECTORS_MEMORY_INDEX,
    REFLEXION_EPISODES_TABLE,
    REFLEXION_MEMORY_INDEX,
    SKILLS_TABLE,
    SKILLS_PATTERN_INDEX,
    SKILLS_SUCCESS_INDEX,
    CAUSAL_EDGES_TABLE,
    CAUSAL_FROM_INDEX,
    CAUSAL_TO_INDEX,
    ARCHITECTURES_TABLE,
    ARCHITECTURES_PARADIGM_INDEX,
    INTELLIGENCES_TABLE,
    INTELLIGENCES_ARCH_INDEX,
    INTELLIGENCES_MATURITY_INDEX,
];
