//! # omega-persistence
//!
//! SQLite-backed persistence layer for ExoGenesis Omega.
//!
//! This crate provides durable storage for:
//! - Hierarchical memory tiers
//! - Learned skills with usage tracking
//! - Evolved architectures with lineage
//! - Intelligence instances with state
//! - Causal graphs and reflexion episodes
//! - Vector embeddings for similarity search
//!
//! ## Example
//!
//! ```
//! use omega_persistence::{OmegaStore, StoredMemory};
//! use chrono::Utc;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create an in-memory database for testing
//! let store = OmegaStore::new_in_memory()?;
//!
//! // Store a memory
//! let memory = StoredMemory {
//!     id: "mem-001".to_string(),
//!     content: "First memory in the system".to_string(),
//!     tier: 1,
//!     importance: 0.95,
//!     embedding_blob: None,
//!     created_at: Utc::now().timestamp(),
//!     last_accessed: Utc::now().timestamp(),
//! };
//!
//! store.store_memory(&memory)?;
//!
//! // Retrieve it
//! let retrieved = store.get_memory("mem-001")?;
//! assert_eq!(retrieved.content, "First memory in the system");
//! assert_eq!(retrieved.tier, 1);
//! # Ok(())
//! # }
//! ```

pub mod schema;
pub mod storage;

pub use storage::{
    OmegaStore, StorageError, Result,
    StoredMemory, StoredSkill, StoredArchitecture,
    StoredIntelligence, StoredVector, StoredReflexionEpisode,
    StoredCausalEdge, DatabaseStatistics,
};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_store() -> OmegaStore {
        OmegaStore::new_in_memory().expect("Failed to create test store")
    }

    #[test]
    fn test_store_and_retrieve_memory() {
        let store = create_test_store();
        let now = Utc::now().timestamp();

        let memory = StoredMemory {
            id: "mem-test-001".to_string(),
            content: "Test memory content".to_string(),
            tier: 2,
            importance: 0.85,
            embedding_blob: Some(vec![1, 2, 3, 4]),
            created_at: now,
            last_accessed: now,
        };

        store.store_memory(&memory).unwrap();
        let retrieved = store.get_memory("mem-test-001").unwrap();

        assert_eq!(retrieved.id, "mem-test-001");
        assert_eq!(retrieved.content, "Test memory content");
        assert_eq!(retrieved.tier, 2);
        assert_eq!(retrieved.importance, 0.85);
        assert_eq!(retrieved.embedding_blob, Some(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_query_memories_by_tier() {
        let store = create_test_store();
        let now = Utc::now().timestamp();

        // Store multiple memories in different tiers
        for i in 0..5 {
            let memory = StoredMemory {
                id: format!("mem-tier-{}", i),
                content: format!("Memory {}", i),
                tier: if i < 3 { 1 } else { 2 },
                importance: 0.5 + (i as f64 * 0.1),
                embedding_blob: None,
                created_at: now,
                last_accessed: now,
            };
            store.store_memory(&memory).unwrap();
        }

        let tier1_memories = store.query_memories_by_tier(1).unwrap();
        assert_eq!(tier1_memories.len(), 3);

        let tier2_memories = store.query_memories_by_tier(2).unwrap();
        assert_eq!(tier2_memories.len(), 2);

        // Verify ordering by importance (descending)
        assert!(tier1_memories[0].importance >= tier1_memories[1].importance);
    }

    #[test]
    fn test_update_memory_access() {
        let store = create_test_store();
        let now = Utc::now().timestamp();

        let memory = StoredMemory {
            id: "mem-access-001".to_string(),
            content: "Access test".to_string(),
            tier: 1,
            importance: 0.7,
            embedding_blob: None,
            created_at: now,
            last_accessed: now,
        };

        store.store_memory(&memory).unwrap();

        let new_access_time = now + 1000;
        store.update_memory_access("mem-access-001", new_access_time).unwrap();

        let retrieved = store.get_memory("mem-access-001").unwrap();
        assert_eq!(retrieved.last_accessed, new_access_time);
    }

    #[test]
    fn test_store_and_retrieve_skill() {
        let store = create_test_store();
        let now = Utc::now().timestamp();

        let skill = StoredSkill {
            id: "skill-001".to_string(),
            name: "code_review".to_string(),
            description: "Perform code review with best practices".to_string(),
            trigger_pattern: "review.*code".to_string(),
            success_count: 5,
            last_used: Some(now),
            created_at: now,
        };

        store.store_skill(&skill).unwrap();
        let retrieved = store.get_skill("skill-001").unwrap();

        assert_eq!(retrieved.name, "code_review");
        assert_eq!(retrieved.success_count, 5);
    }

    #[test]
    fn test_get_skills_by_pattern() {
        let store = create_test_store();
        let now = Utc::now().timestamp();

        let skills = vec![
            StoredSkill {
                id: "skill-code-1".to_string(),
                name: "code_analysis".to_string(),
                description: "Analyze code quality".to_string(),
                trigger_pattern: "analyze_code".to_string(),
                success_count: 10,
                last_used: Some(now),
                created_at: now,
            },
            StoredSkill {
                id: "skill-code-2".to_string(),
                name: "code_generation".to_string(),
                description: "Generate code from spec".to_string(),
                trigger_pattern: "generate_code".to_string(),
                success_count: 7,
                last_used: Some(now),
                created_at: now,
            },
            StoredSkill {
                id: "skill-test-1".to_string(),
                name: "test_writing".to_string(),
                description: "Write unit tests".to_string(),
                trigger_pattern: "write_tests".to_string(),
                success_count: 3,
                last_used: Some(now),
                created_at: now,
            },
        ];

        for skill in &skills {
            store.store_skill(skill).unwrap();
        }

        let code_skills = store.get_skills_by_pattern("code").unwrap();
        assert_eq!(code_skills.len(), 2);

        // Verify ordering by success count (descending)
        assert_eq!(code_skills[0].success_count, 10);
        assert_eq!(code_skills[1].success_count, 7);
    }

    #[test]
    fn test_increment_skill_success() {
        let store = create_test_store();
        let now = Utc::now().timestamp();

        let skill = StoredSkill {
            id: "skill-inc-001".to_string(),
            name: "test_skill".to_string(),
            description: "Test".to_string(),
            trigger_pattern: "test".to_string(),
            success_count: 5,
            last_used: None,
            created_at: now,
        };

        store.store_skill(&skill).unwrap();

        let new_time = now + 500;
        store.increment_skill_success("skill-inc-001", new_time).unwrap();

        let retrieved = store.get_skill("skill-inc-001").unwrap();
        assert_eq!(retrieved.success_count, 6);
        assert_eq!(retrieved.last_used, Some(new_time));
    }

    #[test]
    fn test_store_and_retrieve_architecture() {
        let store = create_test_store();
        let now = Utc::now().timestamp();

        let arch = StoredArchitecture {
            id: "arch-001".to_string(),
            name: "HybridTransformer".to_string(),
            paradigm: "neural".to_string(),
            substrate: "pytorch".to_string(),
            fitness_json: r#"{"accuracy": 0.95, "speed": 0.8}"#.to_string(),
            lineage_json: r#"{"parent": null, "generation": 1}"#.to_string(),
            created_at: now,
        };

        store.store_architecture(&arch).unwrap();
        let retrieved = store.get_architecture("arch-001").unwrap();

        assert_eq!(retrieved.name, "HybridTransformer");
        assert_eq!(retrieved.paradigm, "neural");
    }

    #[test]
    fn test_get_architectures_by_paradigm() {
        let store = create_test_store();
        let now = Utc::now().timestamp();

        let archs = vec![
            StoredArchitecture {
                id: "arch-neural-1".to_string(),
                name: "Transformer".to_string(),
                paradigm: "neural".to_string(),
                substrate: "pytorch".to_string(),
                fitness_json: "{}".to_string(),
                lineage_json: "{}".to_string(),
                created_at: now,
            },
            StoredArchitecture {
                id: "arch-neural-2".to_string(),
                name: "CNN".to_string(),
                paradigm: "neural".to_string(),
                substrate: "tensorflow".to_string(),
                fitness_json: "{}".to_string(),
                lineage_json: "{}".to_string(),
                created_at: now + 100,
            },
            StoredArchitecture {
                id: "arch-symbolic-1".to_string(),
                name: "LogicNet".to_string(),
                paradigm: "symbolic".to_string(),
                substrate: "prolog".to_string(),
                fitness_json: "{}".to_string(),
                lineage_json: "{}".to_string(),
                created_at: now + 200,
            },
        ];

        for arch in &archs {
            store.store_architecture(arch).unwrap();
        }

        let neural_archs = store.get_architectures_by_paradigm("neural").unwrap();
        assert_eq!(neural_archs.len(), 2);

        // Verify ordering by created_at (descending)
        assert_eq!(neural_archs[0].name, "CNN");
        assert_eq!(neural_archs[1].name, "Transformer");
    }

    #[test]
    fn test_store_and_retrieve_intelligence() {
        let store = create_test_store();
        let now = Utc::now().timestamp();

        // First create an architecture
        let arch = StoredArchitecture {
            id: "arch-intel-001".to_string(),
            name: "TestArch".to_string(),
            paradigm: "hybrid".to_string(),
            substrate: "rust".to_string(),
            fitness_json: "{}".to_string(),
            lineage_json: "{}".to_string(),
            created_at: now,
        };
        store.store_architecture(&arch).unwrap();

        let intel = StoredIntelligence {
            id: "intel-001".to_string(),
            name: "Alpha".to_string(),
            arch_id: "arch-intel-001".to_string(),
            maturity: 0.75,
            capabilities_json: r#"["reasoning", "learning"]"#.to_string(),
            memories_json: r#"["mem-001", "mem-002"]"#.to_string(),
            state_json: r#"{"active": true}"#.to_string(),
            created_at: now,
            updated_at: now,
        };

        store.store_intelligence(&intel).unwrap();
        let retrieved = store.get_intelligence("intel-001").unwrap();

        assert_eq!(retrieved.name, "Alpha");
        assert_eq!(retrieved.maturity, 0.75);
        assert_eq!(retrieved.arch_id, "arch-intel-001");
    }

    #[test]
    fn test_get_intelligences_by_arch() {
        let store = create_test_store();
        let now = Utc::now().timestamp();

        // Create architecture
        let arch = StoredArchitecture {
            id: "arch-multi-001".to_string(),
            name: "MultiArch".to_string(),
            paradigm: "neural".to_string(),
            substrate: "pytorch".to_string(),
            fitness_json: "{}".to_string(),
            lineage_json: "{}".to_string(),
            created_at: now,
        };
        store.store_architecture(&arch).unwrap();

        // Create multiple intelligences
        for i in 0..3 {
            let intel = StoredIntelligence {
                id: format!("intel-multi-{}", i),
                name: format!("Intel-{}", i),
                arch_id: "arch-multi-001".to_string(),
                maturity: 0.5 + (i as f64 * 0.2),
                capabilities_json: "[]".to_string(),
                memories_json: "[]".to_string(),
                state_json: "{}".to_string(),
                created_at: now,
                updated_at: now,
            };
            store.store_intelligence(&intel).unwrap();
        }

        let intels = store.get_intelligences_by_arch("arch-multi-001").unwrap();
        assert_eq!(intels.len(), 3);

        // Verify ordering by maturity (descending)
        assert!(intels[0].maturity >= intels[1].maturity);
        assert!(intels[1].maturity >= intels[2].maturity);
    }

    #[test]
    fn test_store_and_retrieve_vector() {
        let store = create_test_store();
        let now = Utc::now().timestamp();

        // Create memory first
        let memory = StoredMemory {
            id: "mem-vec-001".to_string(),
            content: "Vector test".to_string(),
            tier: 1,
            importance: 0.8,
            embedding_blob: None,
            created_at: now,
            last_accessed: now,
        };
        store.store_memory(&memory).unwrap();

        // Create vector
        let vector = StoredVector {
            id: "vec-001".to_string(),
            memory_id: "mem-vec-001".to_string(),
            dimensions: 768,
            data_blob: vec![0u8; 768 * 4], // 768 floats as bytes
        };

        store.store_vector(&vector).unwrap();
        let retrieved = store.get_vector_by_memory("mem-vec-001").unwrap();

        assert_eq!(retrieved.dimensions, 768);
        assert_eq!(retrieved.data_blob.len(), 768 * 4);
    }

    #[test]
    fn test_store_and_retrieve_reflexion() {
        let store = create_test_store();
        let now = Utc::now().timestamp();

        // Create memory first
        let memory = StoredMemory {
            id: "mem-reflex-001".to_string(),
            content: "Reflexion test".to_string(),
            tier: 1,
            importance: 0.9,
            embedding_blob: None,
            created_at: now,
            last_accessed: now,
        };
        store.store_memory(&memory).unwrap();

        // Create reflexion episode
        let episode = StoredReflexionEpisode {
            id: "reflex-001".to_string(),
            memory_id: "mem-reflex-001".to_string(),
            trigger: "code_error".to_string(),
            context: "Syntax error in function".to_string(),
            action: "Fixed missing semicolon".to_string(),
            outcome: "Code compiled successfully".to_string(),
            created_at: now,
        };

        store.store_reflexion(&episode).unwrap();
        let episodes = store.get_reflexions_by_memory("mem-reflex-001").unwrap();

        assert_eq!(episodes.len(), 1);
        assert_eq!(episodes[0].trigger, "code_error");
        assert_eq!(episodes[0].outcome, "Code compiled successfully");
    }

    #[test]
    fn test_store_and_retrieve_causal_edge() {
        let store = create_test_store();
        let now = Utc::now().timestamp();

        // Create two memories
        let mem1 = StoredMemory {
            id: "mem-cause-001".to_string(),
            content: "Cause".to_string(),
            tier: 1,
            importance: 0.8,
            embedding_blob: None,
            created_at: now,
            last_accessed: now,
        };
        let mem2 = StoredMemory {
            id: "mem-effect-001".to_string(),
            content: "Effect".to_string(),
            tier: 1,
            importance: 0.7,
            embedding_blob: None,
            created_at: now,
            last_accessed: now,
        };
        store.store_memory(&mem1).unwrap();
        store.store_memory(&mem2).unwrap();

        // Create causal edge
        let edge = StoredCausalEdge {
            id: "edge-001".to_string(),
            from_memory: "mem-cause-001".to_string(),
            to_memory: "mem-effect-001".to_string(),
            weight: 0.9,
            edge_type: "causal".to_string(),
            created_at: now,
        };

        store.store_causal_edge(&edge).unwrap();
        let edges = store.get_causal_edges_from("mem-cause-001").unwrap();

        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].to_memory, "mem-effect-001");
        assert_eq!(edges[0].weight, 0.9);
    }

    #[test]
    fn test_database_statistics() {
        let store = create_test_store();
        let now = Utc::now().timestamp();

        // Add some data
        let memory = StoredMemory {
            id: "mem-stats-001".to_string(),
            content: "Stats test".to_string(),
            tier: 1,
            importance: 0.5,
            embedding_blob: None,
            created_at: now,
            last_accessed: now,
        };
        store.store_memory(&memory).unwrap();

        let skill = StoredSkill {
            id: "skill-stats-001".to_string(),
            name: "test".to_string(),
            description: "test".to_string(),
            trigger_pattern: "test".to_string(),
            success_count: 0,
            last_used: None,
            created_at: now,
        };
        store.store_skill(&skill).unwrap();

        let stats = store.get_statistics().unwrap();
        assert_eq!(stats.memory_count, 1);
        assert_eq!(stats.skill_count, 1);
        assert_eq!(stats.architecture_count, 0);
        assert_eq!(stats.intelligence_count, 0);
        assert_eq!(stats.causal_edge_count, 0);
    }

    #[test]
    fn test_backup() {
        use tempfile::NamedTempFile;

        let store = create_test_store();
        let now = Utc::now().timestamp();

        // Add some data
        let memory = StoredMemory {
            id: "mem-backup-001".to_string(),
            content: "Backup test".to_string(),
            tier: 1,
            importance: 0.8,
            embedding_blob: None,
            created_at: now,
            last_accessed: now,
        };
        store.store_memory(&memory).unwrap();

        // Create backup
        let backup_file = NamedTempFile::new().unwrap();
        let backup_path = backup_file.path().to_str().unwrap();
        store.backup(backup_path).unwrap();

        // Open backup and verify
        let backup_store = OmegaStore::new(backup_path).unwrap();
        let retrieved = backup_store.get_memory("mem-backup-001").unwrap();
        assert_eq!(retrieved.content, "Backup test");
    }

    #[test]
    fn test_not_found_errors() {
        let store = create_test_store();

        // Test memory not found
        let result = store.get_memory("nonexistent");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), StorageError::NotFound(_)));

        // Test skill not found
        let result = store.get_skill("nonexistent");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), StorageError::NotFound(_)));

        // Test architecture not found
        let result = store.get_architecture("nonexistent");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), StorageError::NotFound(_)));

        // Test intelligence not found
        let result = store.get_intelligence("nonexistent");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), StorageError::NotFound(_)));
    }
}
