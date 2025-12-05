//! Loop Executors - Execute cycles for each loop type

use omega_core::{
    LoopType, CycleInput, CycleOutput, LoopManager,
};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, trace};

use crate::coordinator::LoopCoordinator;
use crate::processors::*;

/// Executes cycles for a specific loop type
pub struct LoopExecutor {
    loop_type: LoopType,
    processor: Box<dyn CycleProcessor>,
    running: bool,
}

impl LoopExecutor {
    pub fn new(loop_type: LoopType) -> Self {
        let processor: Box<dyn CycleProcessor> = match loop_type {
            LoopType::Reflexive => Box::new(ReflexiveProcessor::new()),
            LoopType::Reactive => Box::new(ReactiveProcessor::new()),
            LoopType::Adaptive => Box::new(AdaptiveProcessor::new()),
            LoopType::Deliberative => Box::new(DeliberativeProcessor::new()),
            LoopType::Evolutionary => Box::new(EvolutionaryProcessor::new()),
            LoopType::Transformative => Box::new(TransformativeProcessor::new()),
            LoopType::Transcendent => Box::new(TranscendentProcessor::new()),
        };

        Self {
            loop_type,
            processor,
            running: false,
        }
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        debug!("Starting {:?} executor", self.loop_type);
        self.running = true;
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        debug!("Stopping {:?} executor", self.loop_type);
        self.running = false;
        Ok(())
    }

    pub async fn execute_cycle(
        &mut self,
        coordinator: Arc<RwLock<LoopCoordinator>>,
        input: CycleInput,
    ) -> Result<CycleOutput, Box<dyn Error>> {
        trace!("Executing {:?} cycle", self.loop_type);

        // Start the cycle
        let mut coord = coordinator.write().await;
        let cycle_id = coord.start_cycle_by_type(self.loop_type, input.clone()).await?;
        drop(coord);

        // Process the cycle
        let output = self.processor.process(input).await?;

        // Complete the cycle
        let mut coord = coordinator.write().await;
        if let Some(temporal_loop) = coord.get_loop_by_type(self.loop_type).await? {
            coord.complete_cycle(&temporal_loop.id, output.clone()).await?;
        }

        debug!("Completed {:?} cycle {}", self.loop_type, cycle_id);
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_executor_creation() {
        let executor = LoopExecutor::new(LoopType::Reflexive);
        assert_eq!(executor.loop_type, LoopType::Reflexive);
    }

    #[tokio::test]
    async fn test_executor_lifecycle() {
        let mut executor = LoopExecutor::new(LoopType::Reactive);

        executor.start().await.unwrap();
        assert!(executor.running);

        executor.stop().await.unwrap();
        assert!(!executor.running);
    }
}
