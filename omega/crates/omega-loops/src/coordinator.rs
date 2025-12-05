//! Loop Coordinator - Implements LoopManager trait for coordinating all temporal loops

use omega_core::{
    LoopType, TemporalLoop, LoopId, CycleInput, CycleOutput, CycleMetrics,
    LoopManager, LoopCycle,
};
use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Coordinates all 7 temporal loops and their interactions
pub struct LoopCoordinator {
    loops: RwLock<HashMap<LoopId, TemporalLoop>>,
    loop_type_index: RwLock<HashMap<LoopType, LoopId>>,
}

impl LoopCoordinator {
    pub fn new() -> Self {
        info!("Initializing Loop Coordinator");
        Self {
            loops: RwLock::new(HashMap::new()),
            loop_type_index: RwLock::new(HashMap::new()),
        }
    }

    /// Get a loop by its type (convenience method)
    pub async fn get_loop_by_type(&self, loop_type: LoopType) -> Result<Option<TemporalLoop>, Box<dyn Error>> {
        let index = self.loop_type_index.read().await;
        if let Some(loop_id) = index.get(&loop_type) {
            self.get_loop(loop_id).await
        } else {
            Ok(None)
        }
    }

    /// Start a cycle in a specific loop type (convenience method)
    pub async fn start_cycle_by_type(
        &mut self,
        loop_type: LoopType,
        input: CycleInput,
    ) -> Result<String, Box<dyn Error>> {
        let index = self.loop_type_index.read().await;
        if let Some(loop_id) = index.get(&loop_type).cloned() {
            drop(index);
            self.start_cycle(&loop_id, input).await
        } else {
            Err(format!("Loop type {:?} not found", loop_type).into())
        }
    }
}

impl Default for LoopCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LoopManager for LoopCoordinator {
    async fn create_loop(
        &mut self,
        loop_type: LoopType,
        name: String,
        description: String,
    ) -> Result<TemporalLoop, Box<dyn Error>> {
        debug!("Creating {:?} loop: {}", loop_type, name);

        let temporal_loop = TemporalLoop::new(loop_type, name, description);
        let loop_id = temporal_loop.id.clone();

        let mut loops = self.loops.write().await;
        let mut index = self.loop_type_index.write().await;

        loops.insert(loop_id.clone(), temporal_loop.clone());
        index.insert(loop_type, loop_id);

        Ok(temporal_loop)
    }

    async fn get_loop(&self, id: &LoopId) -> Result<Option<TemporalLoop>, Box<dyn Error>> {
        let loops = self.loops.read().await;
        Ok(loops.get(id).cloned())
    }

    async fn start_cycle(
        &mut self,
        loop_id: &LoopId,
        input: CycleInput,
    ) -> Result<String, Box<dyn Error>> {
        let mut loops = self.loops.write().await;

        if let Some(temporal_loop) = loops.get_mut(loop_id) {
            let cycle_id = temporal_loop.start_cycle(input);
            debug!("Started cycle {} in loop {}", cycle_id, loop_id);
            Ok(cycle_id)
        } else {
            Err(format!("Loop {} not found", loop_id).into())
        }
    }

    async fn complete_cycle(
        &mut self,
        loop_id: &LoopId,
        output: CycleOutput,
    ) -> Result<(), Box<dyn Error>> {
        let mut loops = self.loops.write().await;

        if let Some(temporal_loop) = loops.get_mut(loop_id) {
            // Calculate metrics from the output
            let metrics = CycleMetrics {
                duration: chrono::Duration::zero(), // Would be calculated from actual timing
                success: true,
                quality: 0.8,
                efficiency: 0.7,
                novelty: 0.5,
                alignment: 0.9,
            };

            temporal_loop.complete_cycle(output, metrics);
            debug!("Completed cycle in loop {}", loop_id);
            Ok(())
        } else {
            Err(format!("Loop {} not found", loop_id).into())
        }
    }

    async fn get_current_cycle(
        &self,
        loop_id: &LoopId,
    ) -> Result<Option<LoopCycle>, Box<dyn Error>> {
        let loops = self.loops.read().await;

        if let Some(temporal_loop) = loops.get(loop_id) {
            Ok(temporal_loop.current_cycle.clone())
        } else {
            Ok(None)
        }
    }

    async fn get_loop_history(
        &self,
        loop_id: &LoopId,
        limit: Option<usize>,
    ) -> Result<Vec<LoopCycle>, Box<dyn Error>> {
        let loops = self.loops.read().await;

        if let Some(temporal_loop) = loops.get(loop_id) {
            let _history_ids = if let Some(lim) = limit {
                temporal_loop.history.iter().rev().take(lim).cloned().collect()
            } else {
                temporal_loop.history.clone()
            };

            // In a full implementation, would fetch actual cycle data
            // For now, return empty vec as cycles are not persisted
            Ok(vec![])
        } else {
            Err(format!("Loop {} not found", loop_id).into())
        }
    }

    async fn list_loops(&self) -> Result<Vec<TemporalLoop>, Box<dyn Error>> {
        let loops = self.loops.read().await;
        Ok(loops.values().cloned().collect())
    }

    async fn delete_loop(&mut self, id: &LoopId) -> Result<(), Box<dyn Error>> {
        let mut loops = self.loops.write().await;
        let mut index = self.loop_type_index.write().await;

        if let Some(temporal_loop) = loops.remove(id) {
            index.remove(&temporal_loop.loop_type);
            info!("Deleted loop {}", id);
            Ok(())
        } else {
            warn!("Attempted to delete non-existent loop {}", id);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_coordinator_create_loop() {
        let mut coord = LoopCoordinator::new();

        let result = coord.create_loop(
            LoopType::Reflexive,
            "Test Loop".to_string(),
            "Test Description".to_string(),
        ).await;

        assert!(result.is_ok());
        let temporal_loop = result.unwrap();
        assert_eq!(temporal_loop.loop_type, LoopType::Reflexive);
    }

    #[tokio::test]
    async fn test_coordinator_cycle_management() {
        let mut coord = LoopCoordinator::new();

        let temporal_loop = coord.create_loop(
            LoopType::Reactive,
            "Reactive Loop".to_string(),
            "Quick responses".to_string(),
        ).await.unwrap();

        let input = CycleInput {
            data: HashMap::new(),
            context: "test context".to_string(),
            objectives: vec!["objective 1".to_string()],
        };

        let cycle_id = coord.start_cycle(&temporal_loop.id, input).await.unwrap();
        assert!(!cycle_id.is_empty());

        let current = coord.get_current_cycle(&temporal_loop.id).await.unwrap();
        assert!(current.is_some());
    }

    #[tokio::test]
    async fn test_list_loops() {
        let mut coord = LoopCoordinator::new();

        for loop_type in &[LoopType::Reflexive, LoopType::Reactive, LoopType::Adaptive] {
            coord.create_loop(
                *loop_type,
                format!("{:?} Loop", loop_type),
                "description".to_string(),
            ).await.unwrap();
        }

        let loops = coord.list_loops().await.unwrap();
        assert_eq!(loops.len(), 3);
    }
}
