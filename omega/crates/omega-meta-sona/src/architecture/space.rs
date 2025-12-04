//! Architecture space definition and constraints

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Defines the space of all possible valid architectures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureSpace {
    pub dimensions: usize,
    pub constraints: Vec<Constraint>,
    pub size_constraints: SizeConstraints,
    pub resource_constraints: ResourceConstraints,
}

impl ArchitectureSpace {
    pub fn new(dimensions: usize) -> Self {
        Self {
            dimensions,
            constraints: Vec::new(),
            size_constraints: SizeConstraints::default(),
            resource_constraints: ResourceConstraints::default(),
        }
    }

    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }

    pub fn is_valid(&self, _encoding: &[f64]) -> bool {
        // Validate architecture against constraints
        true
    }
}

/// Constraints on architecture structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Constraint {
    MinNodes(usize),
    MaxNodes(usize),
    RequiredNodeType(String),
    MaxDepth(usize),
    ResourceBudget(f64),
}

/// Size constraints for architectures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeConstraints {
    pub min_nodes: usize,
    pub max_nodes: usize,
    pub min_edges: usize,
    pub max_edges: usize,
    pub min_depth: usize,
    pub max_depth: usize,
    pub max_width: usize,
}

impl Default for SizeConstraints {
    fn default() -> Self {
        Self {
            min_nodes: 1,
            max_nodes: 1000,
            min_edges: 0,
            max_edges: 10000,
            min_depth: 1,
            max_depth: 100,
            max_width: 1000,
        }
    }
}

/// Resource constraints for architectures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConstraints {
    pub max_parameters: u64,
    pub max_flops: u64,
    pub max_memory_bytes: u64,
    pub max_latency: Duration,
}

impl Default for ResourceConstraints {
    fn default() -> Self {
        Self {
            max_parameters: 1_000_000_000, // 1B parameters
            max_flops: 1_000_000_000_000,   // 1T FLOPs
            max_memory_bytes: 10_000_000_000, // 10GB
            max_latency: Duration::from_secs(1),
        }
    }
}

/// State of a partial architecture during search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureState {
    pub nodes: Vec<String>,
    pub edges: Vec<(String, String)>,
    pub hyperparameters: std::collections::HashMap<String, f64>,
}

impl ArchitectureState {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            hyperparameters: std::collections::HashMap::new(),
        }
    }

    pub fn is_complete(&self) -> bool {
        !self.nodes.is_empty() && !self.edges.is_empty()
    }
}
