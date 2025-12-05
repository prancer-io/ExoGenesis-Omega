//! Runtime configuration for the Omega system

use crate::error::{ConfigError, ConfigResult};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Configuration for the AgentDB subsystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDBConfig {
    /// Maximum number of concurrent agents
    pub max_agents: usize,
    /// Agent pruning threshold
    pub prune_threshold: usize,
    /// Enable agent persistence
    pub enable_persistence: bool,
    /// Persistence directory
    pub persistence_dir: Option<String>,
}

impl Default for AgentDBConfig {
    fn default() -> Self {
        Self {
            max_agents: 10000,
            prune_threshold: 8000,
            enable_persistence: true,
            persistence_dir: Some("data/agentdb".to_string()),
        }
    }
}

/// Configuration for the Memory subsystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// Working memory capacity (in items)
    pub working_capacity: usize,
    /// Short-term memory capacity
    pub short_term_capacity: usize,
    /// Long-term memory capacity
    pub long_term_capacity: usize,
    /// Consolidation interval (in seconds)
    pub consolidation_interval: u64,
    /// Enable compression
    pub enable_compression: bool,
    /// Memory persistence directory
    pub persistence_dir: Option<String>,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            working_capacity: 1000,
            short_term_capacity: 10000,
            long_term_capacity: 1000000,
            consolidation_interval: 300, // 5 minutes
            enable_compression: true,
            persistence_dir: Some("data/memory".to_string()),
        }
    }
}

/// Configuration for the Loops subsystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopsConfig {
    /// Enable conscious loop
    pub enable_conscious: bool,
    /// Conscious loop interval (ms)
    pub conscious_interval: u64,
    /// Enable subconscious loop
    pub enable_subconscious: bool,
    /// Subconscious loop interval (ms)
    pub subconscious_interval: u64,
    /// Enable meta loop
    pub enable_meta: bool,
    /// Meta loop interval (ms)
    pub meta_interval: u64,
    /// Enable unconscious loop
    pub enable_unconscious: bool,
    /// Unconscious loop interval (ms)
    pub unconscious_interval: u64,
}

impl Default for LoopsConfig {
    fn default() -> Self {
        Self {
            enable_conscious: true,
            conscious_interval: 100,
            enable_subconscious: true,
            subconscious_interval: 500,
            enable_meta: true,
            meta_interval: 1000,
            enable_unconscious: true,
            unconscious_interval: 5000,
        }
    }
}

/// Configuration for the Meta-SONA subsystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaSONAConfig {
    /// Population size for evolution
    pub population_size: usize,
    /// Number of generations
    pub generations: usize,
    /// Mutation rate (0.0 to 1.0)
    pub mutation_rate: f64,
    /// Crossover rate (0.0 to 1.0)
    pub crossover_rate: f64,
    /// Enable neural architecture search
    pub enable_nas: bool,
    /// Enable self-modification
    pub enable_self_modification: bool,
    /// Intelligence cache size
    pub cache_size: usize,
}

impl Default for MetaSONAConfig {
    fn default() -> Self {
        Self {
            population_size: 100,
            generations: 50,
            mutation_rate: 0.1,
            crossover_rate: 0.7,
            enable_nas: true,
            enable_self_modification: true,
            cache_size: 1000,
        }
    }
}

/// Main configuration for the Omega Runtime
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmegaConfig {
    /// AgentDB configuration
    pub agentdb: AgentDBConfig,
    /// Memory configuration
    pub memory: MemoryConfig,
    /// Loops configuration
    pub loops: LoopsConfig,
    /// Meta-SONA configuration
    pub meta_sona: MetaSONAConfig,
    /// Enable event logging
    pub enable_event_logging: bool,
    /// Enable metrics collection
    pub enable_metrics: bool,
    /// Runtime data directory
    pub data_dir: String,
}

impl Default for OmegaConfig {
    fn default() -> Self {
        Self {
            agentdb: AgentDBConfig::default(),
            memory: MemoryConfig::default(),
            loops: LoopsConfig::default(),
            meta_sona: MetaSONAConfig::default(),
            enable_event_logging: true,
            enable_metrics: true,
            data_dir: "data/omega".to_string(),
        }
    }
}

impl OmegaConfig {
    /// Create configuration from a file
    pub fn from_file(path: &Path) -> ConfigResult<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| ConfigError::FileNotFound(format!("{}: {}", path.display(), e)))?;

        let config: OmegaConfig = serde_json::from_str(&content)
            .map_err(|e| ConfigError::Parse(format!("Failed to parse config: {}", e)))?;

        config.validate()?;
        Ok(config)
    }

    /// Save configuration to a file
    pub fn to_file(&self, path: &Path) -> ConfigResult<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Validate the configuration
    pub fn validate(&self) -> ConfigResult<()> {
        // Validate AgentDB config
        if self.agentdb.max_agents == 0 {
            return Err(ConfigError::Validation(
                "max_agents must be greater than 0".to_string(),
            ));
        }
        if self.agentdb.prune_threshold >= self.agentdb.max_agents {
            return Err(ConfigError::Validation(
                "prune_threshold must be less than max_agents".to_string(),
            ));
        }

        // Validate Memory config
        if self.memory.working_capacity == 0 {
            return Err(ConfigError::Validation(
                "working_capacity must be greater than 0".to_string(),
            ));
        }
        if self.memory.consolidation_interval == 0 {
            return Err(ConfigError::Validation(
                "consolidation_interval must be greater than 0".to_string(),
            ));
        }

        // Validate Loops config
        if self.loops.conscious_interval == 0 {
            return Err(ConfigError::Validation(
                "conscious_interval must be greater than 0".to_string(),
            ));
        }

        // Validate Meta-SONA config
        if self.meta_sona.population_size == 0 {
            return Err(ConfigError::Validation(
                "population_size must be greater than 0".to_string(),
            ));
        }
        if !(0.0..=1.0).contains(&self.meta_sona.mutation_rate) {
            return Err(ConfigError::Validation(
                "mutation_rate must be between 0.0 and 1.0".to_string(),
            ));
        }
        if !(0.0..=1.0).contains(&self.meta_sona.crossover_rate) {
            return Err(ConfigError::Validation(
                "crossover_rate must be between 0.0 and 1.0".to_string(),
            ));
        }

        Ok(())
    }

    /// Create a minimal configuration for testing
    pub fn minimal() -> Self {
        Self {
            agentdb: AgentDBConfig {
                max_agents: 100,
                prune_threshold: 80,
                enable_persistence: false,
                persistence_dir: None,
            },
            memory: MemoryConfig {
                working_capacity: 100,
                short_term_capacity: 1000,
                long_term_capacity: 10000,
                consolidation_interval: 60,
                enable_compression: false,
                persistence_dir: None,
            },
            loops: LoopsConfig {
                enable_conscious: true,
                conscious_interval: 1000,
                enable_subconscious: false,
                subconscious_interval: 5000,
                enable_meta: false,
                meta_interval: 10000,
                enable_unconscious: false,
                unconscious_interval: 30000,
            },
            meta_sona: MetaSONAConfig {
                population_size: 10,
                generations: 5,
                mutation_rate: 0.1,
                crossover_rate: 0.7,
                enable_nas: false,
                enable_self_modification: false,
                cache_size: 100,
            },
            enable_event_logging: false,
            enable_metrics: false,
            data_dir: "test_data".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_is_valid() {
        let config = OmegaConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_minimal_config_is_valid() {
        let config = OmegaConfig::minimal();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_max_agents() {
        let mut config = OmegaConfig::default();
        config.agentdb.max_agents = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_prune_threshold() {
        let mut config = OmegaConfig::default();
        config.agentdb.prune_threshold = config.agentdb.max_agents + 1;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_mutation_rate() {
        let mut config = OmegaConfig::default();
        config.meta_sona.mutation_rate = 1.5;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_serialization() {
        let config = OmegaConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: OmegaConfig = serde_json::from_str(&json).unwrap();
        assert!(deserialized.validate().is_ok());
    }
}
