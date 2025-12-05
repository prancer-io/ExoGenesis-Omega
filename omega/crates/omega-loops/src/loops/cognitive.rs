//! Loop 3: Cognitive - Working memory and reasoning (~60s)

use crate::{
    LoopId, LoopInput, LoopOutput, LoopError, TemporalLoop, TickResult, Timescale,
    loops::{LoopState, timescales},
};
use async_trait::async_trait;
use std::collections::VecDeque;
use std::time::Instant;
use tokio::sync::RwLock;
use tracing::{debug, trace};

/// Cognitive loop - reasoning and working memory layer
///
/// Handles working memory management, deliberate reasoning, decision making,
/// and attention control at ~60 second intervals.
pub struct CognitiveLoop {
    state: RwLock<LoopState>,
    working_memory: RwLock<WorkingMemory>,
    reasoning_queue: RwLock<VecDeque<ReasoningTask>>,
}

#[derive(Debug, Clone)]
struct WorkingMemory {
    capacity: usize,
    items: VecDeque<MemoryItem>,
}

#[derive(Debug, Clone)]
struct MemoryItem {
    id: String,
    data: serde_json::Value,
    importance: f64,
    timestamp: std::time::SystemTime,
    access_count: u32,
}

#[derive(Debug, Clone)]
struct ReasoningTask {
    task_id: String,
    task_type: ReasoningType,
    input_data: serde_json::Value,
    priority: u8,
    created: std::time::SystemTime,
}

#[derive(Debug, Clone)]
enum ReasoningType {
    Inference,
    Planning,
    ProblemSolving,
    DecisionMaking,
}

impl WorkingMemory {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            items: VecDeque::new(),
        }
    }

    fn add(&mut self, item: MemoryItem) {
        if self.items.len() >= self.capacity {
            // Remove least important item
            if let Some(min_idx) = self.items.iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| {
                    a.importance.partial_cmp(&b.importance).unwrap()
                })
                .map(|(idx, _)| idx)
            {
                self.items.remove(min_idx);
            }
        }

        self.items.push_back(item);
    }

    fn get(&mut self, id: &str) -> Option<&mut MemoryItem> {
        self.items.iter_mut().find(|item| item.id == id).map(|item| {
            item.access_count += 1;
            item.importance = (item.importance * 0.9 + 0.1).min(1.0);
            item
        })
    }

    fn decay(&mut self) {
        for item in &mut self.items {
            item.importance *= 0.95;
        }
    }
}

impl CognitiveLoop {
    pub fn new() -> Self {
        debug!("Initializing Cognitive Loop");
        Self {
            state: RwLock::new(LoopState::new()),
            working_memory: RwLock::new(WorkingMemory::new(50)),
            reasoning_queue: RwLock::new(VecDeque::new()),
        }
    }

    /// Process reasoning tasks
    async fn process_reasoning(&self, task: &ReasoningTask) -> serde_json::Value {
        trace!("Processing {:?} reasoning task", task.task_type);

        match task.task_type {
            ReasoningType::Inference => {
                serde_json::json!({
                    "type": "inference_result",
                    "conclusions": ["derived_fact_1", "derived_fact_2"],
                    "confidence": 0.75,
                })
            }
            ReasoningType::Planning => {
                serde_json::json!({
                    "type": "plan",
                    "steps": [
                        {"step": 1, "action": "analyze"},
                        {"step": 2, "action": "decide"},
                        {"step": 3, "action": "execute"},
                    ],
                })
            }
            ReasoningType::ProblemSolving => {
                serde_json::json!({
                    "type": "solution",
                    "approach": "decomposition",
                    "sub_problems": [],
                })
            }
            ReasoningType::DecisionMaking => {
                serde_json::json!({
                    "type": "decision",
                    "choice": "option_a",
                    "reasoning": "maximizes expected value",
                })
            }
        }
    }

    /// Update working memory with new information
    async fn update_memory(&self, data: &serde_json::Value) {
        let mut memory = self.working_memory.write().await;

        let item = MemoryItem {
            id: uuid::Uuid::new_v4().to_string(),
            data: data.clone(),
            importance: 0.7,
            timestamp: std::time::SystemTime::now(),
            access_count: 1,
        };

        memory.add(item);
    }
}

impl Default for CognitiveLoop {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TemporalLoop for CognitiveLoop {
    fn id(&self) -> LoopId {
        LoopId::Cognitive
    }

    fn timescale(&self) -> Timescale {
        timescales::cognitive()
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
        drop(state);

        // Process reasoning tasks
        let mut queue = self.reasoning_queue.write().await;
        let mut processed = 0;

        while let Some(task) = queue.pop_front() {
            let _result = self.process_reasoning(&task).await;
            processed += 1;

            // Limit processing per tick
            if processed >= 10 {
                break;
            }
        }
        drop(queue);

        // Decay working memory
        let mut memory = self.working_memory.write().await;
        memory.decay();
        let memory_size = memory.items.len();
        drop(memory);

        let duration = start.elapsed();

        // Update state
        let mut state = self.state.write().await;
        state.tick(duration, processed);

        Ok(TickResult {
            loop_id: self.id(),
            duration,
            processed,
            should_continue: true,
            status: Some(format!("Memory: {}, Tasks: {}", memory_size, processed)),
        })
    }

    async fn process(&mut self, input: LoopInput) -> Result<LoopOutput, LoopError> {
        let start = Instant::now();

        trace!("Cognitive loop processing input from {:?}", input.source);

        // Add to working memory
        self.update_memory(&input.data).await;

        // Create reasoning task
        let task = ReasoningTask {
            task_id: uuid::Uuid::new_v4().to_string(),
            task_type: ReasoningType::Inference,
            input_data: input.data.clone(),
            priority: input.priority,
            created: std::time::SystemTime::now(),
        };

        // Process immediately or queue
        let result = if input.priority > 200 {
            self.process_reasoning(&task).await
        } else {
            let mut queue = self.reasoning_queue.write().await;
            queue.push_back(task);
            serde_json::json!({"queued": true})
        };

        let duration = start.elapsed();

        let memory = self.working_memory.read().await;
        let queue = self.reasoning_queue.read().await;

        Ok(LoopOutput {
            source: self.id(),
            data: serde_json::json!({
                "type": "cognitive_processed",
                "reasoning_result": result,
                "timestamp": std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis(),
            }),
            metadata: serde_json::json!({
                "processing_time_ms": duration.as_millis(),
                "working_memory_size": memory.items.len(),
                "reasoning_queue_size": queue.len(),
            }),
        })
    }

    async fn start(&mut self) -> Result<(), LoopError> {
        let mut state = self.state.write().await;
        if state.running {
            return Err(LoopError::StartFailed(
                self.id(),
                "already running".to_string(),
            ));
        }

        debug!("Starting Cognitive loop");
        state.start();
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), LoopError> {
        let mut state = self.state.write().await;
        if !state.running {
            return Ok(());
        }

        let memory = self.working_memory.read().await;
        debug!(
            "Stopping Cognitive loop - {} items in memory, {} tasks processed",
            memory.items.len(),
            state.processed_count
        );

        state.stop();

        // Clear queues
        let mut queue = self.reasoning_queue.write().await;
        queue.clear();

        Ok(())
    }

    fn is_running(&self) -> bool {
        false // Placeholder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cognitive_loop() {
        let mut loop_impl = CognitiveLoop::new();
        loop_impl.start().await.unwrap();

        let input = LoopInput {
            source: Some(LoopId::Neural),
            target: LoopId::Cognitive,
            data: serde_json::json!({"concept": "test"}),
            priority: 255,
        };

        let output = loop_impl.process(input).await.unwrap();
        assert_eq!(output.source, LoopId::Cognitive);
    }

    #[test]
    fn test_working_memory() {
        let mut memory = WorkingMemory::new(3);

        for i in 0..5 {
            memory.add(MemoryItem {
                id: format!("item_{}", i),
                data: serde_json::json!({"value": i}),
                importance: i as f64 / 5.0,
                timestamp: std::time::SystemTime::now(),
                access_count: 0,
            });
        }

        // Should only keep 3 most important
        assert_eq!(memory.items.len(), 3);
    }
}
