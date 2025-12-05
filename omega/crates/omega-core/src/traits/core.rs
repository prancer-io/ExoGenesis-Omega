use async_trait::async_trait;
use crate::types::{
    Intelligence, IntelligenceId, IntelligenceStatus,
    Memory, MemoryId, MemoryTier, MemoryQuery,
    TemporalLoop, LoopId, LoopType, LoopCycle, CycleInput, CycleOutput,
    Capability, Architecture,
};
use std::error::Error;

/// Core trait for intelligence management
#[async_trait]
pub trait IntelligenceManager: Send + Sync {
    /// Create a new intelligence instance
    async fn create_intelligence(
        &mut self,
        name: String,
        architecture: Architecture,
    ) -> Result<Intelligence, Box<dyn Error>>;

    /// Get an intelligence by ID
    async fn get_intelligence(
        &self,
        id: &IntelligenceId,
    ) -> Result<Option<Intelligence>, Box<dyn Error>>;

    /// Update intelligence status
    async fn update_status(
        &mut self,
        id: &IntelligenceId,
        status: IntelligenceStatus,
    ) -> Result<(), Box<dyn Error>>;

    /// Add a capability to an intelligence
    async fn add_capability(
        &mut self,
        id: &IntelligenceId,
        capability: Capability,
    ) -> Result<(), Box<dyn Error>>;

    /// List all intelligences
    async fn list_intelligences(&self) -> Result<Vec<Intelligence>, Box<dyn Error>>;

    /// Delete an intelligence
    async fn delete_intelligence(&mut self, id: &IntelligenceId) -> Result<(), Box<dyn Error>>;
}

/// Core trait for memory tier management
#[async_trait]
pub trait MemoryManager: Send + Sync {
    /// Store a memory in a specific tier
    async fn store_memory(&mut self, memory: Memory) -> Result<MemoryId, Box<dyn Error>>;

    /// Retrieve a memory by ID
    async fn get_memory(&self, id: &MemoryId) -> Result<Option<Memory>, Box<dyn Error>>;

    /// Query memories across tiers
    async fn query_memories(&self, query: MemoryQuery) -> Result<Vec<Memory>, Box<dyn Error>>;

    /// Update memory metadata (importance, confidence, etc.)
    async fn update_memory(&mut self, id: &MemoryId, memory: Memory)
        -> Result<(), Box<dyn Error>>;

    /// Delete a memory
    async fn delete_memory(&mut self, id: &MemoryId) -> Result<(), Box<dyn Error>>;

    /// Consolidate memories across tiers (e.g., short-term to long-term)
    async fn consolidate_memories(
        &mut self,
        from_tier: MemoryTier,
        to_tier: MemoryTier,
    ) -> Result<usize, Box<dyn Error>>;

    /// Prune expired memories
    async fn prune_expired(&mut self) -> Result<usize, Box<dyn Error>>;

    /// Get memory statistics by tier
    async fn get_tier_stats(&self, tier: MemoryTier) -> Result<MemoryTierStats, Box<dyn Error>>;
}

#[derive(Debug, Clone)]
pub struct MemoryTierStats {
    pub tier: MemoryTier,
    pub total_memories: usize,
    pub total_size_bytes: usize,
    pub average_importance: f64,
    pub average_confidence: f64,
    pub oldest_memory: Option<String>,
    pub newest_memory: Option<String>,
}

/// Core trait for temporal loop management
#[async_trait]
pub trait LoopManager: Send + Sync {
    /// Create a new temporal loop
    async fn create_loop(
        &mut self,
        loop_type: LoopType,
        name: String,
        description: String,
    ) -> Result<TemporalLoop, Box<dyn Error>>;

    /// Get a loop by ID
    async fn get_loop(&self, id: &LoopId) -> Result<Option<TemporalLoop>, Box<dyn Error>>;

    /// Start a new cycle in a loop
    async fn start_cycle(
        &mut self,
        loop_id: &LoopId,
        input: CycleInput,
    ) -> Result<String, Box<dyn Error>>;

    /// Complete the current cycle
    async fn complete_cycle(
        &mut self,
        loop_id: &LoopId,
        output: CycleOutput,
    ) -> Result<(), Box<dyn Error>>;

    /// Get the current cycle of a loop
    async fn get_current_cycle(
        &self,
        loop_id: &LoopId,
    ) -> Result<Option<LoopCycle>, Box<dyn Error>>;

    /// Get loop history
    async fn get_loop_history(
        &self,
        loop_id: &LoopId,
        limit: Option<usize>,
    ) -> Result<Vec<LoopCycle>, Box<dyn Error>>;

    /// List all loops
    async fn list_loops(&self) -> Result<Vec<TemporalLoop>, Box<dyn Error>>;

    /// Delete a loop
    async fn delete_loop(&mut self, id: &LoopId) -> Result<(), Box<dyn Error>>;
}

/// Trait for intelligence evolution
#[async_trait]
pub trait EvolutionEngine: Send + Sync {
    /// Evaluate intelligence fitness
    async fn evaluate_fitness(&self, intelligence: &Intelligence) -> Result<f64, Box<dyn Error>>;

    /// Generate variations of an architecture
    async fn generate_variations(
        &self,
        architecture: &Architecture,
        count: usize,
    ) -> Result<Vec<Architecture>, Box<dyn Error>>;

    /// Select best architectures from a population
    async fn select_best(
        &self,
        population: Vec<Intelligence>,
        count: usize,
    ) -> Result<Vec<Intelligence>, Box<dyn Error>>;

    /// Crossover two architectures
    async fn crossover(
        &self,
        parent1: &Architecture,
        parent2: &Architecture,
    ) -> Result<Architecture, Box<dyn Error>>;

    /// Mutate an architecture
    async fn mutate(&self, architecture: &Architecture) -> Result<Architecture, Box<dyn Error>>;
}

/// Trait for capability discovery and integration
#[async_trait]
pub trait CapabilityDiscovery: Send + Sync {
    /// Discover available capabilities
    async fn discover_capabilities(&self) -> Result<Vec<Capability>, Box<dyn Error>>;

    /// Check if a capability is compatible with an architecture
    async fn is_compatible(
        &self,
        capability: &Capability,
        architecture: &Architecture,
    ) -> Result<bool, Box<dyn Error>>;

    /// Integrate a capability into an intelligence
    async fn integrate_capability(
        &mut self,
        intelligence_id: &IntelligenceId,
        capability: Capability,
    ) -> Result<(), Box<dyn Error>>;

    /// Remove a capability from an intelligence
    async fn remove_capability(
        &mut self,
        intelligence_id: &IntelligenceId,
        capability_id: &str,
    ) -> Result<(), Box<dyn Error>>;
}
