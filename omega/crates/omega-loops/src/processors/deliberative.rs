//! Deliberative Loop Processor (60s) - Strategic planning and complex reasoning

use super::{CycleProcessor, ProcessorMetrics, ProcessorInsight, metrics_to_json, insights_to_strings};
use omega_core::{CycleInput, CycleOutput, Action, ActionType};
use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;
use std::time::{Duration, Instant};
use tracing::trace;

/// Deliberative processor for complex reasoning
/// Target latency: ~60 seconds
pub struct DeliberativeProcessor {
    /// Reasoning depth limit
    max_depth: usize,
    /// Time budget
    time_budget: Duration,
}

impl DeliberativeProcessor {
    pub fn new() -> Self {
        Self {
            max_depth: 10,
            time_budget: Duration::from_secs(60),
        }
    }

    /// Multi-step reasoning process
    async fn reason(&self, data: &HashMap<String, serde_json::Value>, context: &str) -> ReasoningChain {
        let start = Instant::now();
        let mut chain = ReasoningChain::new();

        // Step 1: Parse the problem
        let entities = self.extract_entities(data, context);
        let relations = self.extract_relations(data);
        let goal = self.extract_goal(data, context);

        chain.add_step(ReasoningStep {
            name: "parse".to_string(),
            input: serde_json::json!({ "context": context, "data": data }),
            output: serde_json::json!({
                "entities": entities,
                "relations": relations,
                "goal": goal,
            }),
            confidence: 0.9,
        });

        // Check time budget before continuing
        if start.elapsed() >= self.time_budget || chain.steps.len() >= self.max_depth {
            return chain;
        }

        // Step 2: Generate hypotheses
        let hypotheses = self.generate_hypotheses(&chain.steps[0].output, &entities);
        chain.add_step(ReasoningStep {
            name: "hypothesize".to_string(),
            input: chain.steps[0].output.clone(),
            output: serde_json::json!({ "hypotheses": hypotheses }),
            confidence: 0.75,
        });

        // Check time budget before continuing
        if start.elapsed() >= self.time_budget || chain.steps.len() >= self.max_depth {
            return chain;
        }

        // Step 3: Evaluate hypotheses
        let evaluations: Vec<_> = hypotheses.iter().map(|h| {
            let score = self.evaluate_hypothesis(h, &entities, &relations);
            serde_json::json!({
                "hypothesis": h,
                "score": score,
                "supporting_evidence": self.find_evidence(h, &entities, true),
                "counter_evidence": self.find_evidence(h, &entities, false),
            })
        }).collect();

        chain.add_step(ReasoningStep {
            name: "evaluate".to_string(),
            input: chain.steps[1].output.clone(),
            output: serde_json::json!({ "evaluations": evaluations }),
            confidence: 0.8,
        });

        // Check time budget before continuing
        if start.elapsed() >= self.time_budget || chain.steps.len() >= self.max_depth {
            return chain;
        }

        // Step 4: Select best hypothesis
        let best = evaluations.iter()
            .max_by(|a, b| {
                let score_a = a["score"].as_f64().unwrap_or(0.0);
                let score_b = b["score"].as_f64().unwrap_or(0.0);
                score_a.partial_cmp(&score_b).unwrap()
            });

        let best_score = best.as_ref().and_then(|b| b["score"].as_f64()).unwrap_or(0.0);

        chain.add_step(ReasoningStep {
            name: "conclude".to_string(),
            input: chain.steps[2].output.clone(),
            output: serde_json::json!({
                "conclusion": best,
                "confidence": best_score,
                "alternatives": evaluations.len(),
            }),
            confidence: best_score,
        });

        chain
    }

    fn extract_entities(&self, data: &HashMap<String, serde_json::Value>, context: &str) -> Vec<String> {
        let mut entities = Vec::new();

        // Extract from context
        for word in context.split_whitespace() {
            if word.len() > 3 && word.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                entities.push(word.to_string());
            }
        }

        // Extract from data keys
        for (key, value) in data {
            if key.len() > 2 {
                entities.push(key.clone());
            }

            // Extract from string values
            if let Some(s) = value.as_str() {
                for word in s.split_whitespace() {
                    if word.len() > 3 && word.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                        entities.push(word.to_string());
                    }
                }
            }
        }

        entities.sort();
        entities.dedup();
        entities
    }

    fn extract_relations(&self, data: &HashMap<String, serde_json::Value>) -> Vec<serde_json::Value> {
        let mut relations = Vec::new();

        for (key, value) in data {
            if let Some(obj) = value.as_object() {
                for (sub_key, sub_value) in obj {
                    relations.push(serde_json::json!({
                        "subject": key,
                        "predicate": sub_key,
                        "object": sub_value,
                    }));
                }
            }
        }

        relations
    }

    fn extract_goal(&self, data: &HashMap<String, serde_json::Value>, context: &str) -> Option<String> {
        // Check explicit goal
        if let Some(goal) = data.get("goal") {
            return goal.as_str().map(|s| s.to_string());
        }

        // Infer from context
        let context_lower = context.to_lowercase();
        if context_lower.contains("optimize") {
            return Some("optimize performance".to_string());
        } else if context_lower.contains("solve") {
            return Some("find solution".to_string());
        } else if context_lower.contains("analyze") {
            return Some("analyze data".to_string());
        }

        None
    }

    fn generate_hypotheses(&self, parsed: &serde_json::Value, entities: &[String]) -> Vec<String> {
        let mut hypotheses = Vec::new();

        // Generate hypotheses based on entities
        if entities.len() > 1 {
            hypotheses.push(format!("Direct relationship between {} and {}", entities[0], entities.get(1).unwrap_or(&"unknown".to_string())));
            hypotheses.push(format!("Indirect influence from {} to system", entities[0]));
            hypotheses.push(format!("Combined effect of {} elements", entities.len()));
        } else if entities.len() == 1 {
            hypotheses.push(format!("{} is the primary factor", entities[0]));
            hypotheses.push(format!("{} requires external input", entities[0]));
        } else {
            hypotheses.push("No clear entity relationships".to_string());
        }

        // Add contextual hypotheses
        if let Some(goal) = parsed.get("goal") {
            if let Some(goal_str) = goal.as_str() {
                hypotheses.push(format!("Goal '{}' can be achieved through incremental steps", goal_str));
                hypotheses.push(format!("Goal '{}' requires resource optimization", goal_str));
            }
        }

        hypotheses
    }

    fn evaluate_hypothesis(&self, hypothesis: &str, entities: &[String], relations: &[serde_json::Value]) -> f64 {
        let mut score = 0.5; // Base score

        // Boost if hypothesis mentions known entities
        for entity in entities {
            if hypothesis.contains(entity) {
                score += 0.1;
            }
        }

        // Boost based on complexity
        let word_count = hypothesis.split_whitespace().count();
        if word_count > 5 && word_count < 15 {
            score += 0.1;
        }

        // Boost if supported by relations
        if !relations.is_empty() {
            score += 0.1;
        }

        // Add some randomness to simulate uncertainty
        score += (rand::random::<f64>() - 0.5) * 0.2;

        score.min(1.0).max(0.0)
    }

    fn find_evidence(&self, hypothesis: &str, entities: &[String], supporting: bool) -> Vec<String> {
        let mut evidence = Vec::new();

        for entity in entities {
            if hypothesis.contains(entity) {
                if supporting {
                    evidence.push(format!("Entity '{}' present in hypothesis", entity));
                } else {
                    if rand::random::<f64>() < 0.3 {
                        evidence.push(format!("Entity '{}' may have alternative interpretations", entity));
                    }
                }
            }
        }

        evidence
    }
}

impl Default for DeliberativeProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
struct ReasoningChain {
    steps: Vec<ReasoningStep>,
}

#[derive(Debug, Clone)]
struct ReasoningStep {
    name: String,
    input: serde_json::Value,
    output: serde_json::Value,
    confidence: f64,
}

impl ReasoningChain {
    fn new() -> Self {
        Self { steps: Vec::new() }
    }

    fn add_step(&mut self, step: ReasoningStep) {
        self.steps.push(step);
    }

    fn overall_confidence(&self) -> f64 {
        if self.steps.is_empty() {
            return 0.0;
        }
        // Geometric mean of confidences
        self.steps.iter().map(|s| s.confidence).product::<f64>().powf(1.0 / self.steps.len() as f64)
    }
}

#[async_trait]
impl CycleProcessor for DeliberativeProcessor {
    async fn process(&mut self, input: CycleInput) -> Result<CycleOutput, Box<dyn Error>> {
        let start = Instant::now();
        trace!("Processing Deliberative cycle");

        let mut insights_vec = Vec::new();
        let mut actions = Vec::new();
        let mut results = HashMap::new();

        // Perform multi-step reasoning
        let chain = self.reason(&input.data, &input.context).await;

        let conclusion = chain.steps.last()
            .map(|s| s.output.clone())
            .unwrap_or(serde_json::Value::Null);

        let overall_confidence = chain.overall_confidence();

        // Generate insights from reasoning steps
        for step in &chain.steps {
            insights_vec.push(ProcessorInsight::new(
                &step.name,
                format!("{}: confidence {:.1}%", step.name, step.confidence * 100.0),
                step.confidence
            ));
        }

        insights_vec.push(ProcessorInsight::new(
            "reasoning_complete",
            format!("Completed {}-step reasoning with {:.1}% overall confidence",
                chain.steps.len(), overall_confidence * 100.0),
            overall_confidence
        ));

        // Create strategic actions
        actions.push(Action {
            id: uuid::Uuid::now_v7().to_string(),
            action_type: ActionType::Reflect,
            description: "Strategic plan based on deliberative reasoning".to_string(),
            parameters: {
                let mut params = HashMap::new();
                params.insert("conclusion".to_string(), conclusion.clone());
                params.insert("confidence".to_string(), serde_json::json!(overall_confidence));
                params
            },
            priority: 0.9,
        });

        let latency = start.elapsed();

        let metrics = ProcessorMetrics {
            latency,
            cpu_ms: latency.as_millis() as u64,
            memory_bytes: 10 * 1024, // Reasoning uses more memory
            io_ops: chain.steps.len(),
            success: true,
        };

        results.insert("reasoning_chain".to_string(), serde_json::json!({
            "steps": chain.steps.iter().map(|s| &s.name).collect::<Vec<_>>(),
            "step_details": chain.steps.iter().map(|s| serde_json::json!({
                "name": s.name,
                "input": s.input,
                "confidence": s.confidence,
                "output": s.output,
            })).collect::<Vec<_>>(),
        }));
        results.insert("conclusion".to_string(), conclusion);
        results.insert("overall_confidence".to_string(), serde_json::json!(overall_confidence));
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
    async fn test_deliberative_reasoning() {
        let mut processor = DeliberativeProcessor::new();
        let mut data = HashMap::new();
        data.insert("problem".to_string(), serde_json::json!("How to optimize system performance?"));
        data.insert("goal".to_string(), serde_json::json!("Improve response time"));

        let input = CycleInput {
            data,
            context: "Performance optimization task".to_string(),
            objectives: vec!["analyze".to_string(), "plan".to_string()],
        };

        let result = processor.process(input).await.unwrap();
        assert!(result.results.contains_key("reasoning_chain"));
        assert!(result.results.contains_key("conclusion"));

        let chain = &result.results["reasoning_chain"];
        let steps = chain["steps"].as_array().unwrap();
        assert!(steps.len() >= 4); // parse, hypothesize, evaluate, conclude
    }

    #[tokio::test]
    async fn test_deliberative_entity_extraction() {
        let processor = DeliberativeProcessor::new();
        let mut data = HashMap::new();
        data.insert("System".to_string(), serde_json::json!("needs optimization"));
        data.insert("Performance".to_string(), serde_json::json!("metrics"));

        let entities = processor.extract_entities(&data, "Analyze System and Performance");
        assert!(entities.contains(&"Analyze".to_string()));
        assert!(entities.contains(&"System".to_string()));
        assert!(entities.contains(&"Performance".to_string()));
    }

    #[tokio::test]
    async fn test_deliberative_hypothesis_generation() {
        let processor = DeliberativeProcessor::new();
        let entities = vec!["System".to_string(), "Performance".to_string()];
        let parsed = serde_json::json!({
            "goal": "optimize",
            "entities": entities,
        });

        let hypotheses = processor.generate_hypotheses(&parsed, &entities);
        assert!(hypotheses.len() > 0);
    }

    #[tokio::test]
    async fn test_deliberative_confidence() {
        let mut chain = ReasoningChain::new();

        chain.add_step(ReasoningStep {
            name: "step1".to_string(),
            input: serde_json::json!({}),
            output: serde_json::json!({}),
            confidence: 0.9,
        });

        chain.add_step(ReasoningStep {
            name: "step2".to_string(),
            input: serde_json::json!({}),
            output: serde_json::json!({}),
            confidence: 0.8,
        });

        let overall = chain.overall_confidence();
        assert!(overall > 0.0 && overall < 1.0);
        // Geometric mean of 0.9 and 0.8 should be around 0.8485
        assert!((overall - 0.8485).abs() < 0.01);
    }
}
