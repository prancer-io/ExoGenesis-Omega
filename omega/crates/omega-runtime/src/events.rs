//! Event system for the Omega Runtime

use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

/// Types of cognitive loops
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoopType {
    Conscious,
    Subconscious,
    Meta,
    Unconscious,
}

/// Memory tier levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryTier {
    Working,
    ShortTerm,
    LongTerm,
}

/// Intelligence identifier
pub type IntelligenceId = Uuid;

/// Architecture identifier
pub type ArchitectureId = Uuid;

/// Events emitted by the Omega Runtime
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OmegaEvent {
    // Lifecycle events
    SystemStarted {
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    SystemShutdown {
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    SystemPaused {
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    SystemResumed {
        timestamp: chrono::DateTime<chrono::Utc>,
    },

    // Loop events
    LoopCycleStarted {
        loop_type: LoopType,
        cycle_id: Uuid,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    LoopCycleCompleted {
        loop_type: LoopType,
        cycle_id: Uuid,
        duration: Duration,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    LoopError {
        loop_type: LoopType,
        error: String,
        timestamp: chrono::DateTime<chrono::Utc>,
    },

    // Memory events
    MemoryStored {
        tier: MemoryTier,
        id: Uuid,
        size_bytes: usize,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    MemoryRetrieved {
        tier: MemoryTier,
        id: Uuid,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    MemoryConsolidated {
        from_tier: MemoryTier,
        to_tier: MemoryTier,
        count: usize,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    MemoryEvicted {
        tier: MemoryTier,
        id: Uuid,
        timestamp: chrono::DateTime<chrono::Utc>,
    },

    // Intelligence events
    IntelligenceCreated {
        id: IntelligenceId,
        architecture_id: ArchitectureId,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    IntelligenceEvolved {
        id: IntelligenceId,
        generation: usize,
        fitness: f64,
        timestamp: chrono::DateTime<chrono::Utc>,
    },

    // Architecture events
    ArchitectureCreated {
        id: ArchitectureId,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    ArchitectureEvolved {
        id: ArchitectureId,
        fitness: f64,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    ArchitectureMutated {
        id: ArchitectureId,
        timestamp: chrono::DateTime<chrono::Utc>,
    },

    // AgentDB events
    AgentCreated {
        agent_id: Uuid,
        agent_type: String,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    AgentDestroyed {
        agent_id: Uuid,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    AgentsPruned {
        count: usize,
        timestamp: chrono::DateTime<chrono::Utc>,
    },

    // Error events
    Error {
        component: String,
        error: String,
        timestamp: chrono::DateTime<chrono::Utc>,
    },

    // Metrics events
    MetricsCollected {
        component: String,
        metrics: serde_json::Value,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
}

impl OmegaEvent {
    /// Get the event type as a string
    pub fn event_type(&self) -> &'static str {
        match self {
            OmegaEvent::SystemStarted { .. } => "system.started",
            OmegaEvent::SystemShutdown { .. } => "system.shutdown",
            OmegaEvent::SystemPaused { .. } => "system.paused",
            OmegaEvent::SystemResumed { .. } => "system.resumed",
            OmegaEvent::LoopCycleStarted { .. } => "loop.cycle.started",
            OmegaEvent::LoopCycleCompleted { .. } => "loop.cycle.completed",
            OmegaEvent::LoopError { .. } => "loop.error",
            OmegaEvent::MemoryStored { .. } => "memory.stored",
            OmegaEvent::MemoryRetrieved { .. } => "memory.retrieved",
            OmegaEvent::MemoryConsolidated { .. } => "memory.consolidated",
            OmegaEvent::MemoryEvicted { .. } => "memory.evicted",
            OmegaEvent::IntelligenceCreated { .. } => "intelligence.created",
            OmegaEvent::IntelligenceEvolved { .. } => "intelligence.evolved",
            OmegaEvent::ArchitectureCreated { .. } => "architecture.created",
            OmegaEvent::ArchitectureEvolved { .. } => "architecture.evolved",
            OmegaEvent::ArchitectureMutated { .. } => "architecture.mutated",
            OmegaEvent::AgentCreated { .. } => "agent.created",
            OmegaEvent::AgentDestroyed { .. } => "agent.destroyed",
            OmegaEvent::AgentsPruned { .. } => "agents.pruned",
            OmegaEvent::Error { .. } => "error",
            OmegaEvent::MetricsCollected { .. } => "metrics.collected",
        }
    }

    /// Get the timestamp of the event
    pub fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        match self {
            OmegaEvent::SystemStarted { timestamp } => *timestamp,
            OmegaEvent::SystemShutdown { timestamp } => *timestamp,
            OmegaEvent::SystemPaused { timestamp } => *timestamp,
            OmegaEvent::SystemResumed { timestamp } => *timestamp,
            OmegaEvent::LoopCycleStarted { timestamp, .. } => *timestamp,
            OmegaEvent::LoopCycleCompleted { timestamp, .. } => *timestamp,
            OmegaEvent::LoopError { timestamp, .. } => *timestamp,
            OmegaEvent::MemoryStored { timestamp, .. } => *timestamp,
            OmegaEvent::MemoryRetrieved { timestamp, .. } => *timestamp,
            OmegaEvent::MemoryConsolidated { timestamp, .. } => *timestamp,
            OmegaEvent::MemoryEvicted { timestamp, .. } => *timestamp,
            OmegaEvent::IntelligenceCreated { timestamp, .. } => *timestamp,
            OmegaEvent::IntelligenceEvolved { timestamp, .. } => *timestamp,
            OmegaEvent::ArchitectureCreated { timestamp, .. } => *timestamp,
            OmegaEvent::ArchitectureEvolved { timestamp, .. } => *timestamp,
            OmegaEvent::ArchitectureMutated { timestamp, .. } => *timestamp,
            OmegaEvent::AgentCreated { timestamp, .. } => *timestamp,
            OmegaEvent::AgentDestroyed { timestamp, .. } => *timestamp,
            OmegaEvent::AgentsPruned { timestamp, .. } => *timestamp,
            OmegaEvent::Error { timestamp, .. } => *timestamp,
            OmegaEvent::MetricsCollected { timestamp, .. } => *timestamp,
        }
    }
}

/// Event handler function type
pub type EventHandler = Arc<dyn Fn(&OmegaEvent) + Send + Sync>;

/// Event bus for distributing events to handlers
pub struct EventBus {
    handlers: Vec<EventHandler>,
    buffer: Vec<OmegaEvent>,
    max_buffer_size: usize,
}

impl EventBus {
    /// Create a new event bus
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
            buffer: Vec::new(),
            max_buffer_size: 1000,
        }
    }

    /// Create a new event bus with a custom buffer size
    pub fn with_buffer_size(max_buffer_size: usize) -> Self {
        Self {
            handlers: Vec::new(),
            buffer: Vec::new(),
            max_buffer_size,
        }
    }

    /// Register an event handler
    pub fn on(&mut self, handler: EventHandler) {
        self.handlers.push(handler);
    }

    /// Emit an event to all registered handlers
    pub fn emit(&mut self, event: OmegaEvent) {
        // Store in buffer
        if self.buffer.len() >= self.max_buffer_size {
            self.buffer.remove(0); // Remove oldest event
        }
        self.buffer.push(event.clone());

        // Notify all handlers
        for handler in &self.handlers {
            handler(&event);
        }
    }

    /// Get the event history buffer
    pub fn history(&self) -> &[OmegaEvent] {
        &self.buffer
    }

    /// Clear the event history buffer
    pub fn clear_history(&mut self) {
        self.buffer.clear();
    }

    /// Get the number of registered handlers
    pub fn handler_count(&self) -> usize {
        self.handlers.len()
    }

    /// Filter events by type
    pub fn filter_by_type(&self, event_type: &str) -> Vec<&OmegaEvent> {
        self.buffer
            .iter()
            .filter(|e| e.event_type() == event_type)
            .collect()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_event_bus_creation() {
        let bus = EventBus::new();
        assert_eq!(bus.handler_count(), 0);
        assert_eq!(bus.history().len(), 0);
    }

    #[test]
    fn test_event_emission() {
        let mut bus = EventBus::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        bus.on(Arc::new(move |_| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        }));

        bus.emit(OmegaEvent::SystemStarted {
            timestamp: chrono::Utc::now(),
        });

        assert_eq!(counter.load(Ordering::SeqCst), 1);
        assert_eq!(bus.history().len(), 1);
    }

    #[test]
    fn test_event_buffer_overflow() {
        let mut bus = EventBus::with_buffer_size(2);

        bus.emit(OmegaEvent::SystemStarted {
            timestamp: chrono::Utc::now(),
        });
        bus.emit(OmegaEvent::SystemPaused {
            timestamp: chrono::Utc::now(),
        });
        bus.emit(OmegaEvent::SystemResumed {
            timestamp: chrono::Utc::now(),
        });

        assert_eq!(bus.history().len(), 2);
    }

    #[test]
    fn test_event_type_filtering() {
        let mut bus = EventBus::new();

        bus.emit(OmegaEvent::SystemStarted {
            timestamp: chrono::Utc::now(),
        });
        bus.emit(OmegaEvent::SystemPaused {
            timestamp: chrono::Utc::now(),
        });
        bus.emit(OmegaEvent::SystemStarted {
            timestamp: chrono::Utc::now(),
        });

        let started_events = bus.filter_by_type("system.started");
        assert_eq!(started_events.len(), 2);
    }

    #[test]
    fn test_multiple_handlers() {
        let mut bus = EventBus::new();
        let counter1 = Arc::new(AtomicUsize::new(0));
        let counter2 = Arc::new(AtomicUsize::new(0));

        let c1 = counter1.clone();
        let c2 = counter2.clone();

        bus.on(Arc::new(move |_| {
            c1.fetch_add(1, Ordering::SeqCst);
        }));
        bus.on(Arc::new(move |_| {
            c2.fetch_add(1, Ordering::SeqCst);
        }));

        bus.emit(OmegaEvent::SystemStarted {
            timestamp: chrono::Utc::now(),
        });

        assert_eq!(counter1.load(Ordering::SeqCst), 1);
        assert_eq!(counter2.load(Ordering::SeqCst), 1);
    }
}
