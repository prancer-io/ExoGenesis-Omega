//! Main runtime orchestrator for the Omega system

use crate::config::OmegaConfig;
use crate::error::{RuntimeError, RuntimeResult};
use crate::events::{EventBus, EventHandler, OmegaEvent};
use omega_agentdb::{AgentDB, AgentDBConfig};
use omega_loops::LoopEngine;
use omega_memory::CosmicMemory;
use omega_meta_sona::MetaSONA;
use parking_lot::RwLock;
use std::sync::Arc;
use tracing::{debug, info};

/// Runtime state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeState {
    /// Runtime has not been initialized
    Uninitialized,
    /// Runtime is currently initializing
    Initializing,
    /// Runtime is running normally
    Running,
    /// Runtime is paused
    Paused,
    /// Runtime is shutting down
    ShuttingDown,
    /// Runtime has stopped
    Stopped,
}

impl RuntimeState {
    /// Check if a state transition is valid
    pub fn can_transition_to(&self, target: RuntimeState) -> bool {
        use RuntimeState::*;
        matches!(
            (self, target),
            (Uninitialized, Initializing)
                | (Initializing, Running)
                | (Initializing, Stopped)
                | (Running, Paused)
                | (Running, ShuttingDown)
                | (Paused, Running)
                | (Paused, ShuttingDown)
                | (ShuttingDown, Stopped)
        )
    }

    /// Get a human-readable description of the state
    pub fn description(&self) -> &'static str {
        match self {
            RuntimeState::Uninitialized => "uninitialized",
            RuntimeState::Initializing => "initializing",
            RuntimeState::Running => "running",
            RuntimeState::Paused => "paused",
            RuntimeState::ShuttingDown => "shutting down",
            RuntimeState::Stopped => "stopped",
        }
    }
}

/// Main runtime orchestrator for the Omega system
pub struct OmegaRuntime {
    config: OmegaConfig,
    agentdb: Arc<AgentDB>,
    memory: Arc<CosmicMemory>,
    loops: Arc<LoopEngine>,
    meta_sona: Arc<MetaSONA>,
    event_bus: Arc<RwLock<EventBus>>,
    state: Arc<RwLock<RuntimeState>>,
}

impl OmegaRuntime {
    /// Create a new OmegaRuntime instance
    pub async fn new(config: OmegaConfig) -> RuntimeResult<Self> {
        info!("Creating OmegaRuntime with configuration");

        // Validate configuration
        config.validate().map_err(|e| {
            RuntimeError::Config(format!("Invalid configuration: {}", e))
        })?;

        // Create subsystems with async constructors where needed
        let agentdb = Arc::new(
            AgentDB::new(AgentDBConfig::default())
                .await
                .map_err(|e| RuntimeError::AgentDB(e.to_string()))?
        );

        let memory = Arc::new(
            CosmicMemory::new()
                .await
                .map_err(|e| RuntimeError::Memory(e.to_string()))?
        );

        let loops = Arc::new(LoopEngine::new());
        let meta_sona = Arc::new(MetaSONA::new());

        let event_bus = if config.enable_event_logging {
            Arc::new(RwLock::new(EventBus::new()))
        } else {
            Arc::new(RwLock::new(EventBus::with_buffer_size(0)))
        };

        Ok(Self {
            config,
            agentdb,
            memory,
            loops,
            meta_sona,
            event_bus,
            state: Arc::new(RwLock::new(RuntimeState::Uninitialized)),
        })
    }

    /// Start the runtime and all subsystems
    pub async fn start(&self) -> RuntimeResult<()> {
        self.transition_state(RuntimeState::Initializing)?;

        info!("Starting OmegaRuntime");

        // Subsystems are initialized during construction
        // Here we just transition to running state
        debug!("AgentDB ready");
        debug!("Memory system ready");
        debug!("Loop engine ready");
        debug!("Meta-SONA ready");

        self.transition_state(RuntimeState::Running)?;

        self.emit_event(OmegaEvent::SystemStarted {
            timestamp: chrono::Utc::now(),
        });

        info!("OmegaRuntime started successfully");
        Ok(())
    }

    /// Stop the runtime and shutdown all subsystems
    pub async fn stop(&self) -> RuntimeResult<()> {
        self.transition_state(RuntimeState::ShuttingDown)?;

        info!("Stopping OmegaRuntime");

        // Subsystems will be dropped when runtime is dropped
        // Here we just transition to stopped state
        debug!("Meta-SONA stopping");
        debug!("Loop engine stopping");
        debug!("Memory system stopping");
        debug!("AgentDB stopping");

        self.transition_state(RuntimeState::Stopped)?;

        self.emit_event(OmegaEvent::SystemShutdown {
            timestamp: chrono::Utc::now(),
        });

        info!("OmegaRuntime stopped");
        Ok(())
    }

    /// Pause the runtime
    pub async fn pause(&self) -> RuntimeResult<()> {
        self.transition_state(RuntimeState::Paused)?;

        info!("Pausing OmegaRuntime");

        self.emit_event(OmegaEvent::SystemPaused {
            timestamp: chrono::Utc::now(),
        });

        Ok(())
    }

    /// Resume the runtime from paused state
    pub async fn resume(&self) -> RuntimeResult<()> {
        self.transition_state(RuntimeState::Running)?;

        info!("Resuming OmegaRuntime");

        self.emit_event(OmegaEvent::SystemResumed {
            timestamp: chrono::Utc::now(),
        });

        Ok(())
    }

    /// Get reference to AgentDB
    pub fn agentdb(&self) -> &AgentDB {
        &self.agentdb
    }

    /// Get reference to Memory system
    pub fn memory(&self) -> &CosmicMemory {
        &self.memory
    }

    /// Get reference to Loop engine
    pub fn loops(&self) -> &LoopEngine {
        &self.loops
    }

    /// Get reference to Meta-SONA
    pub fn meta_sona(&self) -> &MetaSONA {
        &self.meta_sona
    }

    /// Get the current runtime configuration
    pub fn config(&self) -> &OmegaConfig {
        &self.config
    }

    /// Get the current runtime state
    pub fn state(&self) -> RuntimeState {
        *self.state.read()
    }

    /// Check if runtime is running
    pub fn is_running(&self) -> bool {
        *self.state.read() == RuntimeState::Running
    }

    /// Check if runtime is paused
    pub fn is_paused(&self) -> bool {
        *self.state.read() == RuntimeState::Paused
    }

    /// Register an event handler
    pub fn on_event(&self, handler: EventHandler) {
        self.event_bus.write().on(handler);
    }

    /// Get event history
    pub fn event_history(&self) -> Vec<OmegaEvent> {
        self.event_bus.read().history().to_vec()
    }

    /// Clear event history
    pub fn clear_event_history(&self) {
        self.event_bus.write().clear_history();
    }

    /// Transition to a new state
    fn transition_state(&self, new_state: RuntimeState) -> RuntimeResult<()> {
        let mut state = self.state.write();

        if !state.can_transition_to(new_state) {
            return Err(RuntimeError::InvalidStateTransition {
                current: state.description().to_string(),
                attempted: new_state.description().to_string(),
            });
        }

        debug!("State transition: {} -> {}", state.description(), new_state.description());
        *state = new_state;
        Ok(())
    }

    /// Emit an event to the event bus
    fn emit_event(&self, event: OmegaEvent) {
        if self.config.enable_event_logging {
            self.event_bus.write().emit(event);
        }
    }

    /// Get runtime health status
    pub async fn health(&self) -> RuntimeHealth {
        RuntimeHealth {
            state: self.state(),
            agentdb_healthy: true, // TODO: implement health checks
            memory_healthy: true,
            loops_healthy: true,
            meta_sona_healthy: true,
        }
    }
}

/// Runtime health status
#[derive(Debug, Clone)]
pub struct RuntimeHealth {
    pub state: RuntimeState,
    pub agentdb_healthy: bool,
    pub memory_healthy: bool,
    pub loops_healthy: bool,
    pub meta_sona_healthy: bool,
}

impl RuntimeHealth {
    /// Check if all subsystems are healthy
    pub fn is_healthy(&self) -> bool {
        self.agentdb_healthy
            && self.memory_healthy
            && self.loops_healthy
            && self.meta_sona_healthy
            && self.state == RuntimeState::Running
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_transitions() {
        use RuntimeState::*;

        assert!(Uninitialized.can_transition_to(Initializing));
        assert!(Initializing.can_transition_to(Running));
        assert!(Running.can_transition_to(Paused));
        assert!(Paused.can_transition_to(Running));
        assert!(Running.can_transition_to(ShuttingDown));
        assert!(ShuttingDown.can_transition_to(Stopped));

        assert!(!Uninitialized.can_transition_to(Running));
        assert!(!Running.can_transition_to(Stopped));
        assert!(!Stopped.can_transition_to(Running));
    }

    #[tokio::test]
    async fn test_runtime_creation() {
        let config = OmegaConfig::minimal();
        let runtime = OmegaRuntime::new(config).await;
        assert!(runtime.is_ok());
    }

    #[tokio::test]
    async fn test_runtime_lifecycle() {
        let config = OmegaConfig::minimal();
        let runtime = OmegaRuntime::new(config).await.unwrap();

        assert_eq!(runtime.state(), RuntimeState::Uninitialized);

        runtime.start().await.unwrap();
        assert_eq!(runtime.state(), RuntimeState::Running);
        assert!(runtime.is_running());

        runtime.pause().await.unwrap();
        assert_eq!(runtime.state(), RuntimeState::Paused);
        assert!(runtime.is_paused());

        runtime.resume().await.unwrap();
        assert_eq!(runtime.state(), RuntimeState::Running);

        runtime.stop().await.unwrap();
        assert_eq!(runtime.state(), RuntimeState::Stopped);
    }

    #[tokio::test]
    async fn test_invalid_state_transition() {
        let config = OmegaConfig::minimal();
        let runtime = OmegaRuntime::new(config).await.unwrap();

        // Cannot pause when not running
        let result = runtime.pause().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_event_emission() {
        let mut config = OmegaConfig::minimal();
        config.enable_event_logging = true;  // Enable event logging for this test
        let runtime = OmegaRuntime::new(config).await.unwrap();

        runtime.start().await.unwrap();

        let history = runtime.event_history();
        assert!(history.iter().any(|e| matches!(e, OmegaEvent::SystemStarted { .. })));
    }
}
