//! Loop 5: Developmental - Behavioral adaptation (~days)

use crate::{
    LoopId, LoopInput, LoopOutput, LoopError, TemporalLoop, TickResult, Timescale,
    loops::{LoopState, timescales},
};
use async_trait::async_trait;
use std::collections::HashMap;
use std::time::Instant;
use tokio::sync::RwLock;
use tracing::{debug, trace};

/// Developmental loop - behavioral adaptation layer
pub struct DevelopmentalLoop {
    state: RwLock<LoopState>,
    behaviors: RwLock<HashMap<String, Behavior>>,
}

#[derive(Debug, Clone)]
struct Behavior {
    behavior_id: String,
    effectiveness: f64,
    usage_count: u64,
}

impl DevelopmentalLoop {
    pub fn new() -> Self {
        debug!("Initializing Developmental Loop");
        Self {
            state: RwLock::new(LoopState::new()),
            behaviors: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for DevelopmentalLoop {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TemporalLoop for DevelopmentalLoop {
    fn id(&self) -> LoopId {
        LoopId::Developmental
    }

    fn timescale(&self) -> Timescale {
        timescales::developmental()
    }

    async fn tick(&mut self) -> Result<TickResult, LoopError> {
        let start = Instant::now();
        let state = self.state.read().await;

        if !state.running {
            return Ok(TickResult {
                loop_id: self.id(),
                duration: start.elapsed(),
                processed: 0,
                should_continue: false,
                status: Some("not running".to_string()),
            });
        }

        let behaviors = self.behaviors.read().await;
        let processed = behaviors.len();

        Ok(TickResult {
            loop_id: self.id(),
            duration: start.elapsed(),
            processed,
            should_continue: true,
            status: Some(format!("Behaviors: {}", processed)),
        })
    }

    async fn process(&mut self, input: LoopInput) -> Result<LoopOutput, LoopError> {
        trace!("Developmental loop processing");

        Ok(LoopOutput {
            source: self.id(),
            data: serde_json::json!({"type": "developmental_processed"}),
            metadata: serde_json::json!({}),
        })
    }

    async fn start(&mut self) -> Result<(), LoopError> {
        let mut state = self.state.write().await;
        state.start();
        debug!("Starting Developmental loop");
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), LoopError> {
        let mut state = self.state.write().await;
        state.stop();
        debug!("Stopping Developmental loop");
        Ok(())
    }

    fn is_running(&self) -> bool {
        false
    }
}
