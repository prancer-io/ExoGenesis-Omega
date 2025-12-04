//! High-level API for interacting with the Omega Runtime

use crate::error::{APIError, APIResult};
use crate::events::{LoopType, MemoryTier};
use crate::runtime::OmegaRuntime;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// High-level API for the Omega Runtime
pub struct OmegaAPI {
    runtime: Arc<OmegaRuntime>,
}

impl OmegaAPI {
    /// Create a new API instance
    pub fn new(runtime: Arc<OmegaRuntime>) -> Self {
        Self { runtime }
    }

    /// Store a memory in the specified tier
    pub async fn store_memory(
        &self,
        content: &str,
        tier: MemoryTier,
    ) -> APIResult<Uuid> {
        if !self.runtime.is_running() {
            return Err(APIError::Runtime(crate::error::RuntimeError::NotRunning));
        }

        let id = Uuid::new_v4();

        // TODO: Implement actual memory storage when memory system API is available
        tracing::debug!(
            "Storing memory {} in tier {:?}: {}",
            id,
            tier,
            content
        );

        Ok(id)
    }

    /// Query memories from the specified tier
    pub async fn query_memory(
        &self,
        query: &str,
        tier: Option<MemoryTier>,
    ) -> APIResult<Vec<Memory>> {
        if !self.runtime.is_running() {
            return Err(APIError::Runtime(crate::error::RuntimeError::NotRunning));
        }

        // TODO: Implement actual memory query when memory system API is available
        tracing::debug!(
            "Querying memory with query '{}' in tier {:?}",
            query,
            tier
        );

        Ok(Vec::new())
    }

    /// Create a new intelligence from a specification
    pub async fn create_intelligence(
        &self,
        spec: IntelligenceSpec,
    ) -> APIResult<Intelligence> {
        if !self.runtime.is_running() {
            return Err(APIError::Runtime(crate::error::RuntimeError::NotRunning));
        }

        let intelligence = Intelligence {
            id: Uuid::new_v4(),
            architecture_id: Uuid::new_v4(),
            name: spec.name,
            description: spec.description,
            fitness: 0.0,
            generation: 0,
            created_at: chrono::Utc::now(),
        };

        tracing::info!(
            "Created intelligence {} with architecture {}",
            intelligence.id,
            intelligence.architecture_id
        );

        Ok(intelligence)
    }

    /// Evolve an architecture
    pub async fn evolve_architecture(
        &self,
        id: Uuid,
    ) -> APIResult<Architecture> {
        if !self.runtime.is_running() {
            return Err(APIError::Runtime(crate::error::RuntimeError::NotRunning));
        }

        // TODO: Implement actual architecture evolution when Meta-SONA API is available
        let architecture = Architecture {
            id,
            fitness: 0.5,
            generation: 1,
            parameters: serde_json::json!({}),
            created_at: chrono::Utc::now(),
        };

        tracing::info!("Evolved architecture {}", id);

        Ok(architecture)
    }

    /// Trigger a cognitive loop cycle
    pub async fn trigger_loop(
        &self,
        loop_type: LoopType,
        input: CycleInput,
    ) -> APIResult<CycleOutput> {
        if !self.runtime.is_running() {
            return Err(APIError::Runtime(crate::error::RuntimeError::NotRunning));
        }

        // TODO: Implement actual loop triggering when loop engine API is available
        let output = CycleOutput {
            cycle_id: Uuid::new_v4(),
            loop_type,
            result: serde_json::json!({}),
            duration: std::time::Duration::from_millis(100),
        };

        tracing::debug!("Triggered {:?} loop cycle {}", loop_type, output.cycle_id);

        Ok(output)
    }

    /// Get the status of all cognitive loops
    pub async fn get_loop_status(&self) -> APIResult<LoopStatus> {
        if !self.runtime.is_running() {
            return Err(APIError::Runtime(crate::error::RuntimeError::NotRunning));
        }

        // TODO: Implement actual loop status retrieval
        Ok(LoopStatus {
            conscious: LoopInfo {
                enabled: true,
                cycles_completed: 0,
                average_duration_ms: 0,
            },
            subconscious: LoopInfo {
                enabled: true,
                cycles_completed: 0,
                average_duration_ms: 0,
            },
            meta: LoopInfo {
                enabled: true,
                cycles_completed: 0,
                average_duration_ms: 0,
            },
            unconscious: LoopInfo {
                enabled: true,
                cycles_completed: 0,
                average_duration_ms: 0,
            },
        })
    }

    /// Get runtime metrics
    pub async fn get_metrics(&self) -> APIResult<RuntimeMetrics> {
        let health = self.runtime.health().await;

        Ok(RuntimeMetrics {
            state: format!("{:?}", health.state),
            is_healthy: health.is_healthy(),
            event_count: self.runtime.event_history().len(),
            // TODO: Add more metrics when subsystems expose them
            agent_count: 0,
            memory_usage_bytes: 0,
            loop_cycles_total: 0,
        })
    }

    /// Get the runtime configuration
    pub fn get_config(&self) -> &crate::config::OmegaConfig {
        self.runtime.config()
    }
}

/// Memory representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub id: Uuid,
    pub content: String,
    pub tier: MemoryTier,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub accessed_at: chrono::DateTime<chrono::Utc>,
    pub access_count: usize,
}

/// Specification for creating an intelligence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligenceSpec {
    pub name: String,
    pub description: String,
    pub initial_parameters: Option<serde_json::Value>,
}

/// Intelligence representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intelligence {
    pub id: Uuid,
    pub architecture_id: Uuid,
    pub name: String,
    pub description: String,
    pub fitness: f64,
    pub generation: usize,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Architecture representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Architecture {
    pub id: Uuid,
    pub fitness: f64,
    pub generation: usize,
    pub parameters: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Input for a cognitive loop cycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CycleInput {
    pub data: serde_json::Value,
}

/// Output from a cognitive loop cycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CycleOutput {
    pub cycle_id: Uuid,
    pub loop_type: LoopType,
    pub result: serde_json::Value,
    pub duration: std::time::Duration,
}

/// Status of a single loop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopInfo {
    pub enabled: bool,
    pub cycles_completed: u64,
    pub average_duration_ms: u64,
}

/// Status of all cognitive loops
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopStatus {
    pub conscious: LoopInfo,
    pub subconscious: LoopInfo,
    pub meta: LoopInfo,
    pub unconscious: LoopInfo,
}

/// Runtime metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeMetrics {
    pub state: String,
    pub is_healthy: bool,
    pub event_count: usize,
    pub agent_count: usize,
    pub memory_usage_bytes: usize,
    pub loop_cycles_total: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::OmegaConfig;

    #[tokio::test]
    async fn test_api_creation() {
        let config = OmegaConfig::minimal();
        let runtime = Arc::new(OmegaRuntime::new(config).await.unwrap());
        let api = OmegaAPI::new(runtime);

        // API should be created successfully
        assert!(api.get_config().validate().is_ok());
    }

    #[tokio::test]
    async fn test_api_requires_running_runtime() {
        let config = OmegaConfig::minimal();
        let runtime = Arc::new(OmegaRuntime::new(config).await.unwrap());
        let api = OmegaAPI::new(runtime);

        // Operations should fail when runtime is not running
        let result = api.store_memory("test", MemoryTier::Working).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_intelligence() {
        let config = OmegaConfig::minimal();
        let runtime = Arc::new(OmegaRuntime::new(config).await.unwrap());
        runtime.start().await.unwrap();

        let api = OmegaAPI::new(runtime.clone());

        let spec = IntelligenceSpec {
            name: "TestIntelligence".to_string(),
            description: "A test intelligence".to_string(),
            initial_parameters: None,
        };

        let intelligence = api.create_intelligence(spec).await.unwrap();
        assert_eq!(intelligence.name, "TestIntelligence");
        assert_eq!(intelligence.generation, 0);
    }

    #[tokio::test]
    async fn test_get_metrics() {
        let config = OmegaConfig::minimal();
        let runtime = Arc::new(OmegaRuntime::new(config).await.unwrap());
        runtime.start().await.unwrap();

        let api = OmegaAPI::new(runtime.clone());

        let metrics = api.get_metrics().await.unwrap();
        assert_eq!(metrics.state, "Running");
    }
}
