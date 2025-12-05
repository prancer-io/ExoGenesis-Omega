//! Architecture types for system design and topology

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Architectural topology types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TopologyType {
    Hierarchical,
    Mesh,
    Hybrid,
    Adaptive,
}

/// System architecture configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Architecture {
    pub id: Uuid,
    pub name: String,
    pub topology: TopologyType,
    pub components: Vec<Component>,
}

/// Individual system component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub id: Uuid,
    pub name: String,
    pub component_type: String,
    pub config: serde_json::Value,
}
