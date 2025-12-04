//! Cycle processors for each temporal loop type

use omega_core::{CycleInput, CycleOutput};
use async_trait::async_trait;
use std::error::Error;

pub mod reflexive;
pub mod reactive;
pub mod adaptive;
pub mod deliberative;
pub mod evolutionary;
pub mod transformative;
pub mod transcendent;

pub use reflexive::ReflexiveProcessor;
pub use reactive::ReactiveProcessor;
pub use adaptive::AdaptiveProcessor;
pub use deliberative::DeliberativeProcessor;
pub use evolutionary::EvolutionaryProcessor;
pub use transformative::TransformativeProcessor;
pub use transcendent::TranscendentProcessor;

/// Trait for processing cycles in temporal loops
#[async_trait]
pub trait CycleProcessor: Send + Sync {
    /// Process a cycle input and produce output
    async fn process(&mut self, input: CycleInput) -> Result<CycleOutput, Box<dyn Error>>;
}
