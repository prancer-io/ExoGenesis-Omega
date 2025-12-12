//! Architecture space representation for META-SONA
//!
//! This module defines how META-SONA represents and manipulates neural architectures
//! in the search space.

pub mod space;
pub mod encoding;

pub use space::*;
pub use encoding::*;

use serde::{Deserialize, Serialize};

/// Unique identifier for an architecture node
pub type NodeId = String;

/// Types of nodes that can appear in a computational graph
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeType {
    Input,
    Hidden,
    Output,
    Attention,
    Transformer,
    LSTM,
    GRU,
    Memory,
    Routing,
    Custom(String),
}

/// Parameters for a compute node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameters {
    pub values: std::collections::HashMap<String, f64>,
}

impl Default for Parameters {
    fn default() -> Self {
        Self::new()
    }
}

impl Parameters {
    pub fn new() -> Self {
        Self {
            values: std::collections::HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: f64) {
        self.values.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<f64> {
        self.values.get(key).copied()
    }
}

/// Connection between nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub from: NodeId,
    pub to: NodeId,
    pub weight: f64,
    pub connection_type: ConnectionType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectionType {
    Forward,
    Recurrent,
    Residual,
    Attention,
}

/// A node in the architecture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureNode {
    pub id: NodeId,
    pub node_type: NodeType,
    pub parameters: Parameters,
    pub connections: Vec<Connection>,
}

impl ArchitectureNode {
    pub fn new(id: NodeId, node_type: NodeType) -> Self {
        Self {
            id,
            node_type,
            parameters: Parameters::new(),
            connections: Vec::new(),
        }
    }
}

/// Computational graph representing an architecture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputationalGraph {
    pub nodes: Vec<ArchitectureNode>,
    pub edges: Vec<Connection>,
}

impl Default for ComputationalGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl ComputationalGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: ArchitectureNode) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: Connection) {
        self.edges.push(edge);
    }

    pub fn is_valid(&self) -> bool {
        // Check for cycles, disconnected components, etc.
        !self.nodes.is_empty() && !self.edges.is_empty()
    }
}
