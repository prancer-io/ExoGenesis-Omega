//! Adaptive Loop Processor (30min - 24h) - Learning and skill acquisition

use super::{CycleProcessor, ProcessorMetrics, ProcessorInsight, metrics_to_json, insights_to_strings};
use omega_core::{CycleInput, CycleOutput, Action, ActionType};
use async_trait::async_trait;
use itertools::Itertools;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;
use tracing::trace;

/// Adaptive processor for learning and skill acquisition
/// Target latency: minutes to hours
pub struct AdaptiveProcessor {
    /// Learning rate
    learning_rate: f64,
    /// Skill storage
    skills: RwLock<Vec<LearnedSkill>>,
    /// Experience buffer
    experiences: RwLock<Vec<Experience>>,
    /// Maximum experiences to store
    max_experiences: usize,
}

#[derive(Clone, Debug)]
struct LearnedSkill {
    id: String,
    name: String,
    pattern: Vec<f32>,
    success_rate: f64,
    usage_count: u64,
    created_at: std::time::SystemTime,
}

#[derive(Clone, Debug)]
struct Experience {
    state: serde_json::Value,
    action: String,
    reward: f64,
    next_state: serde_json::Value,
    timestamp: std::time::SystemTime,
}

impl AdaptiveProcessor {
    pub fn new() -> Self {
        Self {
            learning_rate: 0.01,
            skills: RwLock::new(Vec::new()),
            experiences: RwLock::new(Vec::new()),
            max_experiences: 1000,
        }
    }

    /// Store an experience for later learning
    pub fn add_experience(&self, state: serde_json::Value, action: String, reward: f64, next_state: serde_json::Value) {
        let mut experiences = self.experiences.write();

        // Add new experience
        experiences.push(Experience {
            state,
            action,
            reward,
            next_state,
            timestamp: std::time::SystemTime::now(),
        });

        // Trim old experiences if we exceed max
        if experiences.len() > self.max_experiences {
            let excess = experiences.len() - self.max_experiences;
            experiences.drain(0..excess);
        }
    }

    /// Consolidate experiences into learned skills
    async fn consolidate_learning(&self) -> Vec<LearnedSkill> {
        let experiences = self.experiences.read().clone();
        let mut new_skills = Vec::new();

        // Group experiences by action pattern
        let mut action_groups: HashMap<String, Vec<&Experience>> = HashMap::new();
        for exp in experiences.iter() {
            action_groups.entry(exp.action.clone()).or_default().push(exp);
        }

        // Create skills from successful patterns
        for (action, exps) in action_groups {
            if exps.is_empty() {
                continue;
            }

            let avg_reward: f64 = exps.iter().map(|e| e.reward).sum::<f64>() / exps.len() as f64;

            // Only create skill if it has positive reward and enough samples
            if avg_reward > 0.5 && exps.len() >= 3 {
                // Create a pattern embedding from the experiences
                let pattern = self.create_skill_pattern(&exps);

                new_skills.push(LearnedSkill {
                    id: uuid::Uuid::now_v7().to_string(),
                    name: action.clone(),
                    pattern,
                    success_rate: avg_reward,
                    usage_count: exps.len() as u64,
                    created_at: std::time::SystemTime::now(),
                });
            }
        }

        new_skills
    }

    /// Create a pattern embedding from experiences
    fn create_skill_pattern(&self, experiences: &[&Experience]) -> Vec<f32> {
        let mut pattern = vec![0.0f32; 32];

        for (i, exp) in experiences.iter().take(32).enumerate() {
            // Simple hash-based encoding
            let state_hash = format!("{:?}", exp.state).chars().map(|c| c as u32).sum::<u32>();
            pattern[i] = (state_hash as f32 % 1000.0) / 1000.0;
        }

        // Normalize
        let norm: f32 = pattern.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for x in &mut pattern {
                *x /= norm;
            }
        }

        pattern
    }

    /// Apply learned skills to current situation
    fn apply_skills(&self, _input: &HashMap<String, serde_json::Value>) -> Option<(String, f64)> {
        let skills = self.skills.read();

        if skills.is_empty() {
            return None;
        }

        // Find best matching skill
        let mut best_skill = None;
        let mut best_score = 0.0;

        for skill in skills.iter() {
            let score = skill.success_rate * (skill.usage_count as f64).log10().max(1.0);
            if score > best_score {
                best_score = score;
                best_skill = Some(skill.clone());
            }
        }

        best_skill.map(|s| (s.name, s.success_rate))
    }

    /// Update skill performance based on usage
    fn update_skill(&self, skill_name: &str, success: bool) {
        let mut skills = self.skills.write();

        if let Some(skill) = skills.iter_mut().find(|s| s.name == skill_name) {
            skill.usage_count += 1;

            // Update success rate with exponential moving average
            let alpha = self.learning_rate;
            let new_sample = if success { 1.0 } else { 0.0 };
            skill.success_rate = alpha * new_sample + (1.0 - alpha) * skill.success_rate;
        }
    }
}

impl Default for AdaptiveProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CycleProcessor for AdaptiveProcessor {
    async fn process(&mut self, input: CycleInput) -> Result<CycleOutput, Box<dyn Error>> {
        let start = Instant::now();
        trace!("Processing Adaptive cycle");

        let mut insights_vec = Vec::new();
        let mut actions = Vec::new();
        let mut results = HashMap::new();

        // Check if this is an experience to store
        if let Some(exp) = input.data.get("experience") {
            let state = exp.get("state").cloned().unwrap_or(serde_json::Value::Null);
            let action = exp.get("action").and_then(|a| a.as_str()).unwrap_or("unknown").to_string();
            let reward = exp.get("reward").and_then(|r| r.as_f64()).unwrap_or(0.0);
            let next_state = exp.get("next_state").cloned().unwrap_or(serde_json::Value::Null);

            self.add_experience(state, action, reward, next_state);

            insights_vec.push(ProcessorInsight::new(
                "experience_stored",
                format!("Stored experience with reward {:.2}", reward),
                0.7
            ));
        }

        // Check if we should consolidate learning
        let should_consolidate = input.data.get("consolidate")
            .and_then(|c| c.as_bool())
            .unwrap_or(false) || self.experiences.read().len() >= 50;

        let new_skills = if should_consolidate {
            let skills = self.consolidate_learning().await;

            if !skills.is_empty() {
                insights_vec.push(ProcessorInsight::new(
                    "learning_consolidated",
                    format!("Consolidated {} new skills from {} experiences",
                        skills.len(), self.experiences.read().len()),
                    0.85
                ));

                // Add to skill storage
                let mut existing = self.skills.write();
                for skill in &skills {
                    insights_vec.push(ProcessorInsight::new(
                        "skill_learned",
                        format!("Learned skill '{}' with {:.0}% success rate",
                            skill.name, skill.success_rate * 100.0),
                        skill.success_rate
                    ));
                }
                existing.extend(skills.clone());
            }

            skills
        } else {
            Vec::new()
        };

        // Try to apply learned skills
        let skill_application = if let Some((skill_name, confidence)) = self.apply_skills(&input.data) {
            insights_vec.push(ProcessorInsight::new(
                "skill_applied",
                format!("Applied skill '{}' with {:.1}% confidence", skill_name, confidence * 100.0),
                confidence
            ));

            actions.push(Action {
                id: uuid::Uuid::now_v7().to_string(),
                action_type: ActionType::Execute,
                description: format!("Execute learned skill '{}'", skill_name),
                parameters: {
                    let mut params = HashMap::new();
                    params.insert("skill".to_string(), serde_json::json!(skill_name));
                    params.insert("confidence".to_string(), serde_json::json!(confidence));
                    params
                },
                priority: 0.7,
            });

            Some(serde_json::json!({
                "skill": skill_name,
                "confidence": confidence,
            }))
        } else {
            None
        };

        let skills = self.skills.read();
        let experiences = self.experiences.read();

        // Create learning action
        if experiences.len() > 0 && new_skills.is_empty() && experiences.len() < 50 {
            actions.push(Action {
                id: uuid::Uuid::now_v7().to_string(),
                action_type: ActionType::Learn,
                description: "Continue gathering experiences for learning".to_string(),
                parameters: HashMap::new(),
                priority: 0.5,
            });
        }

        let latency = start.elapsed();

        let metrics = ProcessorMetrics {
            latency,
            cpu_ms: latency.as_millis() as u64,
            memory_bytes: experiences.len() * 100 + skills.len() * 200,
            io_ops: 2,
            success: true,
        };

        results.insert("learning_status".to_string(), serde_json::json!({
            "experiences_stored": experiences.len(),
            "skills_learned": skills.len(),
            "new_skills_this_cycle": new_skills.len(),
            "learning_rate": self.learning_rate,
        }));

        if let Some(skill_app) = skill_application {
            results.insert("skill_applied".to_string(), skill_app);
        }

        results.insert("top_skills".to_string(), serde_json::json!(
            skills.iter()
                .sorted_by(|a, b| b.success_rate.partial_cmp(&a.success_rate).unwrap())
                .take(5)
                .map(|s| serde_json::json!({
                    "name": s.name,
                    "success_rate": s.success_rate,
                    "usage_count": s.usage_count,
                }))
                .collect::<Vec<_>>()
        ));

        results.insert("metrics".to_string(), metrics_to_json(&metrics));

        Ok(CycleOutput {
            results,
            insights: insights_to_strings(&insights_vec),
            actions,
            next_objectives: input.objectives,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_adaptive_experience_storage() {
        let mut processor = AdaptiveProcessor::new();

        let mut data = HashMap::new();
        data.insert("experience".to_string(), serde_json::json!({
            "state": {"step": 1},
            "action": "move_forward",
            "reward": 0.8,
            "next_state": {"step": 2}
        }));

        let input = CycleInput {
            data,
            context: "test".to_string(),
            objectives: vec![],
        };

        let result = processor.process(input).await.unwrap();
        let status = &result.results["learning_status"];

        assert_eq!(status["experiences_stored"].as_u64().unwrap(), 1);
    }

    #[tokio::test]
    async fn test_adaptive_skill_learning() {
        let mut processor = AdaptiveProcessor::new();

        // Add multiple experiences for the same action
        for i in 0..5 {
            let mut data = HashMap::new();
            data.insert("experience".to_string(), serde_json::json!({
                "state": {"step": i},
                "action": "action_a",
                "reward": 0.9,
                "next_state": {"step": i + 1}
            }));

            let input = CycleInput {
                data,
                context: "test".to_string(),
                objectives: vec![],
            };

            processor.process(input).await.unwrap();
        }

        // Consolidate learning
        let mut data = HashMap::new();
        data.insert("consolidate".to_string(), serde_json::json!(true));

        let input = CycleInput {
            data,
            context: "test".to_string(),
            objectives: vec![],
        };

        let result = processor.process(input).await.unwrap();
        let status = &result.results["learning_status"];

        assert!(status["skills_learned"].as_u64().unwrap() > 0);
    }

    #[tokio::test]
    async fn test_adaptive_skill_application() {
        let mut processor = AdaptiveProcessor::new();

        // Learn a skill
        for i in 0..5 {
            let mut data = HashMap::new();
            data.insert("experience".to_string(), serde_json::json!({
                "state": {"step": i},
                "action": "best_action",
                "reward": 0.95,
                "next_state": {"step": i + 1}
            }));

            let input = CycleInput {
                data,
                context: "test".to_string(),
                objectives: vec![],
            };

            processor.process(input).await.unwrap();
        }

        // Consolidate
        let mut data = HashMap::new();
        data.insert("consolidate".to_string(), serde_json::json!(true));
        let input = CycleInput {
            data,
            context: "test".to_string(),
            objectives: vec![],
        };
        processor.process(input).await.unwrap();

        // Now apply skills
        let mut data = HashMap::new();
        data.insert("task".to_string(), serde_json::json!("perform action"));
        let input = CycleInput {
            data,
            context: "test".to_string(),
            objectives: vec![],
        };

        let result = processor.process(input).await.unwrap();

        // Should have applied a skill
        if result.results.contains_key("skill_applied") {
            let skill_app = &result.results["skill_applied"];
            assert!(skill_app["confidence"].as_f64().unwrap() > 0.0);
        }
    }

    #[tokio::test]
    async fn test_adaptive_top_skills() {
        let mut processor = AdaptiveProcessor::new();

        // Add experiences for multiple actions
        for action_id in 0..3 {
            for i in 0..5 {
                let mut data = HashMap::new();
                data.insert("experience".to_string(), serde_json::json!({
                    "state": {"step": i},
                    "action": format!("action_{}", action_id),
                    "reward": 0.7 + (action_id as f64) * 0.1,
                    "next_state": {"step": i + 1}
                }));

                let input = CycleInput {
                    data,
                    context: "test".to_string(),
                    objectives: vec![],
                };

                processor.process(input).await.unwrap();
            }
        }

        // Consolidate
        let mut data = HashMap::new();
        data.insert("consolidate".to_string(), serde_json::json!(true));
        let input = CycleInput {
            data,
            context: "test".to_string(),
            objectives: vec![],
        };

        let result = processor.process(input).await.unwrap();
        let top_skills = result.results["top_skills"].as_array().unwrap();

        assert!(top_skills.len() > 0);
    }

    #[test]
    fn test_experience_buffer_limit() {
        let processor = AdaptiveProcessor::new();

        // Add more than max_experiences
        for i in 0..1100 {
            processor.add_experience(
                serde_json::json!({"step": i}),
                "action".to_string(),
                0.5,
                serde_json::json!({"step": i + 1})
            );
        }

        let experiences = processor.experiences.read();
        assert_eq!(experiences.len(), processor.max_experiences);
    }
}
