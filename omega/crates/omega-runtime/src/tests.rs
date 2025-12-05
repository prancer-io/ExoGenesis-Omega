//! Integration tests for the Omega Runtime

#[cfg(test)]
mod integration_tests {
    use crate::api::{CycleInput, IntelligenceSpec, OmegaAPI};
    use crate::config::OmegaConfig;
    use crate::events::{EventBus, LoopType, MemoryTier, OmegaEvent};
    use crate::runtime::{OmegaRuntime, RuntimeState};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_full_runtime_lifecycle() {
        // Create runtime with minimal config
        let config = OmegaConfig::minimal();
        let runtime = OmegaRuntime::new(config).await.unwrap();

        // Initial state should be uninitialized
        assert_eq!(runtime.state(), RuntimeState::Uninitialized);

        // Start the runtime
        runtime.start().await.unwrap();
        assert_eq!(runtime.state(), RuntimeState::Running);
        assert!(runtime.is_running());

        // Pause the runtime
        runtime.pause().await.unwrap();
        assert_eq!(runtime.state(), RuntimeState::Paused);
        assert!(runtime.is_paused());

        // Resume the runtime
        runtime.resume().await.unwrap();
        assert_eq!(runtime.state(), RuntimeState::Running);

        // Stop the runtime
        runtime.stop().await.unwrap();
        assert_eq!(runtime.state(), RuntimeState::Stopped);
        assert!(!runtime.is_running());
    }

    #[tokio::test]
    async fn test_event_bus_integration() {
        let mut config = OmegaConfig::minimal();
        config.enable_event_logging = true;  // Enable event logging for this test
        let runtime = OmegaRuntime::new(config).await.unwrap();

        // Register event handler
        let event_count = Arc::new(AtomicUsize::new(0));
        let counter = event_count.clone();

        runtime.on_event(Arc::new(move |_event| {
            counter.fetch_add(1, Ordering::SeqCst);
        }));

        // Start runtime (should emit SystemStarted event)
        runtime.start().await.unwrap();

        // Give handlers time to process
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // At least one event should have been emitted
        assert!(event_count.load(Ordering::SeqCst) > 0);

        // Check event history
        let history = runtime.event_history();
        assert!(history.iter().any(|e| matches!(e, OmegaEvent::SystemStarted { .. })));

        runtime.stop().await.unwrap();
    }

    #[tokio::test]
    async fn test_config_validation() {
        // Valid minimal config
        let config = OmegaConfig::minimal();
        assert!(config.validate().is_ok());

        // Valid default config
        let config = OmegaConfig::default();
        assert!(config.validate().is_ok());

        // Invalid config: max_agents = 0
        let mut config = OmegaConfig::minimal();
        config.agentdb.max_agents = 0;
        assert!(config.validate().is_err());

        // Invalid config: mutation_rate > 1.0
        let mut config = OmegaConfig::minimal();
        config.meta_sona.mutation_rate = 1.5;
        assert!(config.validate().is_err());
    }

    #[tokio::test]
    async fn test_subsystem_access() {
        let config = OmegaConfig::minimal();
        let runtime = OmegaRuntime::new(config).await.unwrap();
        runtime.start().await.unwrap();

        // Access all subsystems
        let _agentdb = runtime.agentdb();
        let _memory = runtime.memory();
        let _loops = runtime.loops();
        let _meta_sona = runtime.meta_sona();

        // All subsystems should be accessible
        runtime.stop().await.unwrap();
    }

    #[tokio::test]
    async fn test_api_memory_operations() {
        let config = OmegaConfig::minimal();
        let runtime = Arc::new(OmegaRuntime::new(config).await.unwrap());
        runtime.start().await.unwrap();

        let api = OmegaAPI::new(runtime.clone());

        // Store memory
        let memory_id = api
            .store_memory("test content", MemoryTier::Working)
            .await
            .unwrap();
        assert!(!memory_id.is_nil());

        // Query memory
        let memories = api
            .query_memory("test", Some(MemoryTier::Working))
            .await
            .unwrap();

        // Note: Current implementation returns empty vec,
        // this will change when memory system is fully integrated
        assert!(memories.is_empty());

        runtime.stop().await.unwrap();
    }

    #[tokio::test]
    async fn test_api_intelligence_operations() {
        let config = OmegaConfig::minimal();
        let runtime = Arc::new(OmegaRuntime::new(config).await.unwrap());
        runtime.start().await.unwrap();

        let api = OmegaAPI::new(runtime.clone());

        // Create intelligence
        let spec = IntelligenceSpec {
            name: "TestAgent".to_string(),
            description: "A test intelligence".to_string(),
            initial_parameters: Some(serde_json::json!({
                "learning_rate": 0.001,
                "layers": [64, 32, 16]
            })),
        };

        let intelligence = api.create_intelligence(spec).await.unwrap();
        assert_eq!(intelligence.name, "TestAgent");
        assert_eq!(intelligence.generation, 0);
        assert!(!intelligence.id.is_nil());

        // Evolve architecture
        let evolved = api
            .evolve_architecture(intelligence.architecture_id)
            .await
            .unwrap();
        assert_eq!(evolved.id, intelligence.architecture_id);

        runtime.stop().await.unwrap();
    }

    #[tokio::test]
    async fn test_api_loop_operations() {
        let config = OmegaConfig::minimal();
        let runtime = Arc::new(OmegaRuntime::new(config).await.unwrap());
        runtime.start().await.unwrap();

        let api = OmegaAPI::new(runtime.clone());

        // Trigger conscious loop
        let input = CycleInput {
            data: serde_json::json!({ "input": "test data" }),
        };

        let output = api
            .trigger_loop(LoopType::Conscious, input)
            .await
            .unwrap();
        assert_eq!(output.loop_type, LoopType::Conscious);
        assert!(!output.cycle_id.is_nil());

        // Get loop status
        let status = api.get_loop_status().await.unwrap();
        assert!(status.conscious.enabled);

        runtime.stop().await.unwrap();
    }

    #[tokio::test]
    async fn test_api_metrics() {
        let config = OmegaConfig::minimal();
        let runtime = Arc::new(OmegaRuntime::new(config).await.unwrap());
        runtime.start().await.unwrap();

        let api = OmegaAPI::new(runtime.clone());

        // Get metrics
        let metrics = api.get_metrics().await.unwrap();
        assert_eq!(metrics.state, "Running");
        assert!(metrics.is_healthy);

        runtime.stop().await.unwrap();
    }

    #[tokio::test]
    async fn test_concurrent_operations() {
        let config = OmegaConfig::minimal();
        let runtime = Arc::new(OmegaRuntime::new(config).await.unwrap());
        runtime.start().await.unwrap();

        let api = Arc::new(OmegaAPI::new(runtime.clone()));

        // Spawn multiple concurrent operations
        let mut handles = vec![];

        for i in 0..10 {
            let api_clone = api.clone();
            let handle = tokio::spawn(async move {
                let spec = IntelligenceSpec {
                    name: format!("Agent{}", i),
                    description: format!("Test agent {}", i),
                    initial_parameters: None,
                };
                api_clone.create_intelligence(spec).await
            });
            handles.push(handle);
        }

        // Wait for all operations to complete
        for handle in handles {
            let result = handle.await.unwrap();
            assert!(result.is_ok());
        }

        runtime.stop().await.unwrap();
    }

    #[tokio::test]
    async fn test_error_handling() {
        let config = OmegaConfig::minimal();
        let runtime = Arc::new(OmegaRuntime::new(config).await.unwrap());
        // Don't start the runtime

        let api = OmegaAPI::new(runtime.clone());

        // Operations should fail when runtime is not running
        assert!(api.store_memory("test", MemoryTier::Working).await.is_err());
        assert!(api.query_memory("test", None).await.is_err());
        assert!(api.get_loop_status().await.is_err());

        // Invalid state transitions
        let result = runtime.pause().await;
        assert!(result.is_err());
    }

    #[test]
    fn test_event_types() {
        let event = OmegaEvent::SystemStarted {
            timestamp: chrono::Utc::now(),
        };
        assert_eq!(event.event_type(), "system.started");

        let event = OmegaEvent::LoopCycleStarted {
            loop_type: LoopType::Conscious,
            cycle_id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
        };
        assert_eq!(event.event_type(), "loop.cycle.started");

        let event = OmegaEvent::MemoryStored {
            tier: MemoryTier::Working,
            id: uuid::Uuid::new_v4(),
            size_bytes: 1024,
            timestamp: chrono::Utc::now(),
        };
        assert_eq!(event.event_type(), "memory.stored");
    }

    #[test]
    fn test_event_bus_filtering() {
        let mut bus = EventBus::new();

        // Emit different event types
        bus.emit(OmegaEvent::SystemStarted {
            timestamp: chrono::Utc::now(),
        });
        bus.emit(OmegaEvent::SystemPaused {
            timestamp: chrono::Utc::now(),
        });
        bus.emit(OmegaEvent::SystemStarted {
            timestamp: chrono::Utc::now(),
        });

        // Filter by type
        let started_events = bus.filter_by_type("system.started");
        assert_eq!(started_events.len(), 2);

        let paused_events = bus.filter_by_type("system.paused");
        assert_eq!(paused_events.len(), 1);
    }

    #[tokio::test]
    async fn test_health_check() {
        let config = OmegaConfig::minimal();
        let runtime = OmegaRuntime::new(config).await.unwrap();

        // Health before starting
        let health = runtime.health().await;
        assert_eq!(health.state, RuntimeState::Uninitialized);

        // Start and check health
        runtime.start().await.unwrap();
        let health = runtime.health().await;
        assert_eq!(health.state, RuntimeState::Running);
        assert!(health.is_healthy());

        runtime.stop().await.unwrap();
    }
}
