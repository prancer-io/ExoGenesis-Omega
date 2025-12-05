//! Loop 4: Learning - Skill acquisition and consolidation (~hours)

use crate::{
    LoopId, LoopInput, LoopOutput, LoopError, TemporalLoop, TickResult, Timescale,
    loops::{LoopState, timescales},
};
use async_trait::async_trait;
use std::collections::HashMap;
use std::time::Instant;
use tokio::sync::RwLock;
use tracing::{debug, trace};

/// Learning loop - skill acquisition and memory consolidation layer
///
/// Handles skill learning, memory consolidation, knowledge integration,
/// and procedural learning at hour-scale intervals.
pub struct LearningLoop {
    state: RwLock<LoopState>,
    skills: RwLock<HashMap<String, Skill>>,
    experiences: RwLock<Vec<Experience>>,
    consolidation_queue: RwLock<Vec<ConsolidationTask>>,
}

#[derive(Debug, Clone)]
struct Skill {
    skill_id: String,
    skill_type: SkillType,
    proficiency: f64,
    practice_count: u64,
    last_practiced: std::time::SystemTime,
    dependencies: Vec<String>,
}

#[derive(Debug, Clone)]
enum SkillType {
    Motor,
    Cognitive,
    Social,
    Technical,
}

#[derive(Debug, Clone)]
struct Experience {
    experience_id: String,
    context: serde_json::Value,
    outcome: serde_json::Value,
    reward: f64,
    timestamp: std::time::SystemTime,
}

#[derive(Debug, Clone)]
struct ConsolidationTask {
    task_id: String,
    experiences: Vec<String>,
    target_skill: Option<String>,
    priority: u8,
}

impl LearningLoop {
    pub fn new() -> Self {
        debug!("Initializing Learning Loop");
        Self {
            state: RwLock::new(LoopState::new()),
            skills: RwLock::new(HashMap::new()),
            experiences: RwLock::new(Vec::new()),
            consolidation_queue: RwLock::new(Vec::new()),
        }
    }

    /// Consolidate experiences into skills
    async fn consolidate_experiences(&self, task: &ConsolidationTask) -> serde_json::Value {
        trace!("Consolidating {} experiences", task.experiences.len());

        let experiences = self.experiences.read().await;
        let relevant_exp: Vec<_> = experiences
            .iter()
            .filter(|e| task.experiences.contains(&e.experience_id))
            .collect();

        // Calculate average reward
        let avg_reward = if !relevant_exp.is_empty() {
            relevant_exp.iter().map(|e| e.reward).sum::<f64>() / relevant_exp.len() as f64
        } else {
            0.0
        };

        // Update skill if specified
        if let Some(skill_id) = &task.target_skill {
            let mut skills = self.skills.write().await;
            if let Some(skill) = skills.get_mut(skill_id) {
                skill.practice_count += task.experiences.len() as u64;
                skill.proficiency = (skill.proficiency + avg_reward * 0.1).min(1.0);
                skill.last_practiced = std::time::SystemTime::now();
            }
        }

        serde_json::json!({
            "type": "consolidation_result",
            "experiences_processed": task.experiences.len(),
            "average_reward": avg_reward,
            "skill_updated": task.target_skill.is_some(),
        })
    }

    /// Record new experience
    async fn record_experience(&self, data: &serde_json::Value) {
        let experience = Experience {
            experience_id: uuid::Uuid::new_v4().to_string(),
            context: data.clone(),
            outcome: serde_json::json!({}),
            reward: 0.5, // Would be computed from actual feedback
            timestamp: std::time::SystemTime::now(),
        };

        let mut experiences = self.experiences.write().await;
        experiences.push(experience);

        // Limit experience buffer
        if experiences.len() > 1000 {
            experiences.drain(0..100);
        }
    }

    /// Update or create skill
    async fn update_skill(&self, skill_id: String, skill_type: SkillType) {
        let mut skills = self.skills.write().await;

        skills
            .entry(skill_id.clone())
            .and_modify(|skill| {
                skill.practice_count += 1;
                skill.proficiency = (skill.proficiency + 0.01).min(1.0);
                skill.last_practiced = std::time::SystemTime::now();
            })
            .or_insert(Skill {
                skill_id,
                skill_type,
                proficiency: 0.1,
                practice_count: 1,
                last_practiced: std::time::SystemTime::now(),
                dependencies: vec![],
            });
    }
}

impl Default for LearningLoop {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TemporalLoop for LearningLoop {
    fn id(&self) -> LoopId {
        LoopId::Learning
    }

    fn timescale(&self) -> Timescale {
        timescales::learning()
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

        // Process consolidation tasks
        let mut queue = self.consolidation_queue.write().await;
        let mut processed = 0;

        let tasks_to_process: Vec<_> = queue.drain(..).collect();
        drop(queue);

        for task in &tasks_to_process {
            let _result = self.consolidate_experiences(task).await;
            processed += 1;
        }

        let duration = start.elapsed();

        // Update state
        let mut state = self.state.write().await;
        state.tick(duration, processed);

        let skills = self.skills.read().await;
        let experiences = self.experiences.read().await;

        Ok(TickResult {
            loop_id: self.id(),
            duration,
            processed,
            should_continue: true,
            status: Some(format!(
                "Skills: {}, Experiences: {}, Consolidated: {}",
                skills.len(),
                experiences.len(),
                processed
            )),
        })
    }

    async fn process(&mut self, input: LoopInput) -> Result<LoopOutput, LoopError> {
        let start = Instant::now();

        trace!("Learning loop processing input from {:?}", input.source);

        // Record experience
        self.record_experience(&input.data).await;

        // Extract skill information if present
        if let Some(skill_info) = input.data.get("skill") {
            if let Some(skill_id) = skill_info.get("id").and_then(|v| v.as_str()) {
                self.update_skill(skill_id.to_string(), SkillType::Cognitive).await;
            }
        }

        // Queue consolidation if enough experiences
        let experiences = self.experiences.read().await;
        let should_consolidate = experiences.len() >= 100;
        let recent_exp: Vec<_> = experiences
            .iter()
            .rev()
            .take(50)
            .map(|e| e.experience_id.clone())
            .collect();
        drop(experiences);

        if should_consolidate {
            let task = ConsolidationTask {
                task_id: uuid::Uuid::new_v4().to_string(),
                experiences: recent_exp,
                target_skill: None,
                priority: input.priority,
            };

            let mut queue = self.consolidation_queue.write().await;
            queue.push(task);
        }

        let duration = start.elapsed();

        let skills = self.skills.read().await;
        let queue = self.consolidation_queue.read().await;

        Ok(LoopOutput {
            source: self.id(),
            data: serde_json::json!({
                "type": "learning_processed",
                "experience_recorded": true,
                "consolidation_queued": should_consolidate,
                "timestamp": std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis(),
            }),
            metadata: serde_json::json!({
                "processing_time_ms": duration.as_millis(),
                "skills_learned": skills.len(),
                "consolidation_queue": queue.len(),
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

        debug!("Starting Learning loop");
        state.start();
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), LoopError> {
        let mut state = self.state.write().await;
        if !state.running {
            return Ok(());
        }

        let skills = self.skills.read().await;
        debug!(
            "Stopping Learning loop - {} skills acquired, {} experiences total",
            skills.len(),
            state.processed_count
        );

        state.stop();
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
    async fn test_learning_loop() {
        let mut loop_impl = LearningLoop::new();
        loop_impl.start().await.unwrap();

        let input = LoopInput {
            source: Some(LoopId::Cognitive),
            target: LoopId::Learning,
            data: serde_json::json!({
                "skill": {"id": "test_skill"},
                "context": "practice"
            }),
            priority: 128,
        };

        let output = loop_impl.process(input).await.unwrap();
        assert_eq!(output.source, LoopId::Learning);
    }

    #[tokio::test]
    async fn test_experience_recording() {
        let loop_impl = LearningLoop::new();

        loop_impl.record_experience(&serde_json::json!({"test": true})).await;

        let experiences = loop_impl.experiences.read().await;
        assert_eq!(experiences.len(), 1);
    }
}
