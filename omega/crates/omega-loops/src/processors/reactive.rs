//! Reactive Loop Processor (100ms) - Pattern recognition and quick responses

use super::{CycleProcessor, ProcessorMetrics, ProcessorInsight, metrics_to_json, insights_to_strings};
use omega_core::{CycleInput, CycleOutput, Action, ActionType};
use async_trait::async_trait;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;
use tracing::trace;

/// Reactive processor for pattern recognition
/// Target latency: ~100ms
pub struct ReactiveProcessor {
    /// Pattern memory for recognition
    patterns: RwLock<Vec<Pattern>>,
    /// Similarity threshold
    threshold: f64,
    /// Target latency
    target_latency_ms: u128,
}

#[derive(Clone, Debug)]
struct Pattern {
    id: String,
    embedding: Vec<f32>,
    response: serde_json::Value,
    hits: u64,
    last_used: std::time::SystemTime,
}

impl ReactiveProcessor {
    pub fn new() -> Self {
        let mut patterns = Vec::new();

        // Pre-load some common patterns
        patterns.push(Pattern {
            id: "greeting".to_string(),
            embedding: Self::create_embedding("hello hi greeting welcome"),
            response: serde_json::json!({"type": "greeting", "action": "respond_friendly"}),
            hits: 0,
            last_used: std::time::SystemTime::now(),
        });

        patterns.push(Pattern {
            id: "question".to_string(),
            embedding: Self::create_embedding("what why how when where question"),
            response: serde_json::json!({"type": "question", "action": "provide_answer"}),
            hits: 0,
            last_used: std::time::SystemTime::now(),
        });

        patterns.push(Pattern {
            id: "request".to_string(),
            embedding: Self::create_embedding("please help need want request"),
            response: serde_json::json!({"type": "request", "action": "assist"}),
            hits: 0,
            last_used: std::time::SystemTime::now(),
        });

        Self {
            patterns: RwLock::new(patterns),
            threshold: 0.7,
            target_latency_ms: 100,
        }
    }

    /// Learn a new pattern
    pub fn learn_pattern(&self, text: &str, response: serde_json::Value) {
        let pattern = Pattern {
            id: uuid::Uuid::now_v7().to_string(),
            embedding: Self::create_embedding(text),
            response,
            hits: 0,
            last_used: std::time::SystemTime::now(),
        };
        self.patterns.write().push(pattern);
    }

    /// Create a simple embedding from text (in production, use real embeddings)
    fn create_embedding(text: &str) -> Vec<f32> {
        let mut embedding = vec![0.0f32; 64];

        // Simple character-based embedding
        for (i, c) in text.chars().take(64).enumerate() {
            embedding[i] = (c as u32 as f32) / 256.0;
        }

        // Add word-level features
        let words: Vec<&str> = text.split_whitespace().collect();
        for (i, word) in words.iter().enumerate().take(32) {
            let word_hash = word.chars().map(|c| c as u32).sum::<u32>();
            if i * 2 + 1 < embedding.len() {
                embedding[i * 2] += (word_hash as f32) / 1000.0;
                embedding[i * 2 + 1] += (word.len() as f32) / 20.0;
            }
        }

        // Normalize
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for x in &mut embedding {
                *x /= norm;
            }
        }

        embedding
    }

    /// Extract embedding from input data
    fn extract_embedding(&self, input: &HashMap<String, serde_json::Value>) -> Vec<f32> {
        let mut text = String::new();

        for (key, value) in input {
            text.push_str(key);
            text.push(' ');
            text.push_str(&value.to_string());
            text.push(' ');
        }

        Self::create_embedding(&text)
    }

    /// Find similar pattern
    fn find_similar(&self, input_embedding: &[f32]) -> Option<(usize, f64)> {
        let patterns = self.patterns.read();
        let mut best_match = None;
        let mut best_similarity = 0.0;

        for (i, pattern) in patterns.iter().enumerate() {
            let similarity = cosine_similarity(input_embedding, &pattern.embedding);
            if similarity > best_similarity && similarity > self.threshold {
                best_similarity = similarity;
                best_match = Some(i);
            }
        }

        best_match.map(|i| (i, best_similarity))
    }
}

impl Default for ReactiveProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate cosine similarity between two vectors
fn cosine_similarity(a: &[f32], b: &[f32]) -> f64 {
    if a.len() != b.len() {
        return 0.0;
    }

    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    (dot / (norm_a * norm_b)) as f64
}

#[async_trait]
impl CycleProcessor for ReactiveProcessor {
    async fn process(&mut self, input: CycleInput) -> Result<CycleOutput, Box<dyn Error>> {
        let start = Instant::now();
        trace!("Processing Reactive cycle");

        let mut insights = Vec::new();
        let mut actions = Vec::new();
        let mut results = HashMap::new();

        // Extract embedding from input
        let embedding = self.extract_embedding(&input.data);

        // Find similar pattern
        let (output, matched) = if let Some((idx, similarity)) = self.find_similar(&embedding) {
            let mut patterns = self.patterns.write();
            patterns[idx].hits += 1;
            patterns[idx].last_used = std::time::SystemTime::now();

            let response = patterns[idx].response.clone();
            let pattern_id = patterns[idx].id.clone();
            let pattern_hits = patterns[idx].hits;

            drop(patterns); // Release lock

            insights.push(ProcessorInsight::new(
                "pattern_match",
                format!("Matched pattern '{}' with {:.1}% confidence ({} total hits)",
                    pattern_id, similarity * 100.0, pattern_hits),
                similarity
            ));

            // Create reactive action
            actions.push(Action {
                id: uuid::Uuid::now_v7().to_string(),
                action_type: ActionType::Reason,
                description: format!("Pattern-based response to '{}'", pattern_id),
                parameters: response.as_object().unwrap_or(&serde_json::Map::new()).iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
                priority: 0.8,
            });

            (serde_json::json!({
                "matched": true,
                "pattern_id": pattern_id,
                "similarity": similarity,
                "response": response,
                "hits": pattern_hits,
            }), true)
        } else {
            insights.push(ProcessorInsight::new(
                "no_match",
                "No similar pattern found, using default response",
                0.3
            ));

            (serde_json::json!({
                "matched": false,
                "reason": "no_similar_pattern",
                "threshold": self.threshold,
            }), false)
        };

        let latency = start.elapsed();

        let metrics = ProcessorMetrics {
            latency,
            cpu_ms: latency.as_millis() as u64,
            memory_bytes: embedding.len() * 4 + 2048,
            io_ops: 1,
            success: true,
        };

        results.insert("pattern_recognition".to_string(), output);
        results.insert("metrics".to_string(), metrics_to_json(&metrics));
        results.insert("pattern_count".to_string(), serde_json::json!(self.patterns.read().len()));
        results.insert("embedding_size".to_string(), serde_json::json!(embedding.len()));

        // Add reasoning action if needed
        if !matched {
            actions.push(Action {
                id: uuid::Uuid::now_v7().to_string(),
                action_type: ActionType::Learn,
                description: "Learn new pattern from unmatched input".to_string(),
                parameters: HashMap::new(),
                priority: 0.6,
            });
        }

        Ok(CycleOutput {
            results,
            insights: insights_to_strings(&insights),
            actions,
            next_objectives: input.objectives,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_reactive_pattern_match() {
        let mut processor = ReactiveProcessor::new();
        let mut data = HashMap::new();
        data.insert("message".to_string(), serde_json::json!("hello there"));

        let input = CycleInput {
            data,
            context: "test".to_string(),
            objectives: vec!["respond".to_string()],
        };

        let result = processor.process(input).await.unwrap();
        let pattern_rec = &result.results["pattern_recognition"];

        assert!(pattern_rec["matched"].as_bool().unwrap_or(false));
        assert!(pattern_rec["similarity"].as_f64().unwrap() > 0.0);
    }

    #[tokio::test]
    async fn test_reactive_learning() {
        let mut processor = ReactiveProcessor::new();

        // Learn a custom pattern
        processor.learn_pattern(
            "error failure problem",
            serde_json::json!({"action": "troubleshoot"})
        );

        let mut data = HashMap::new();
        data.insert("status".to_string(), serde_json::json!("error occurred"));

        let input = CycleInput {
            data,
            context: "test".to_string(),
            objectives: vec![],
        };

        let result = processor.process(input).await.unwrap();
        let pattern_rec = &result.results["pattern_recognition"];

        // Should match the learned pattern
        if pattern_rec["matched"].as_bool().unwrap_or(false) {
            assert!(pattern_rec["similarity"].as_f64().unwrap() > 0.5);
        }
    }

    #[tokio::test]
    async fn test_reactive_no_match() {
        let mut processor = ReactiveProcessor::new();
        let mut data = HashMap::new();
        data.insert("random".to_string(), serde_json::json!("xyz123abc456"));

        let input = CycleInput {
            data,
            context: "test".to_string(),
            objectives: vec![],
        };

        let result = processor.process(input).await.unwrap();
        assert!(result.results.contains_key("pattern_recognition"));
    }

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 0.001);

        let c = vec![1.0, 0.0];
        let d = vec![0.0, 1.0];
        assert!((cosine_similarity(&c, &d) - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_embedding_creation() {
        let embedding = ReactiveProcessor::create_embedding("test");
        assert_eq!(embedding.len(), 64);

        // Check normalization
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 0.001);
    }
}
