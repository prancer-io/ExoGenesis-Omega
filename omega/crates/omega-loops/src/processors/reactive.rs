//! Reactive Loop Processor (5s) - Quick decision-making

use super::CycleProcessor;
use omega_core::{CycleInput, CycleOutput};
use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;

pub struct ReactiveProcessor;

impl ReactiveProcessor {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CycleProcessor for ReactiveProcessor {
    async fn process(&mut self, input: CycleInput) -> Result<CycleOutput, Box<dyn Error>> {
        let mut results = HashMap::new();
        results.insert(
            "decision".to_string(),
            serde_json::json!({"made": true, "type": "reactive"}),
        );

        Ok(CycleOutput {
            results,
            insights: vec!["Quick reactive decision made".to_string()],
            actions: vec![],
            next_objectives: input.objectives,
        })
    }
}
