//! Error types for the Omega Runtime

use thiserror::Error;

/// Errors that can occur during runtime operations
#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Initialization error: {0}")]
    Initialization(String),

    #[error("Shutdown error: {0}")]
    Shutdown(String),

    #[error("State transition error: current={current}, attempted={attempted}")]
    InvalidStateTransition { current: String, attempted: String },

    #[error("Component error - {component}: {error}")]
    Component { component: String, error: String },

    #[error("AgentDB error: {0}")]
    AgentDB(String),

    #[error("Memory error: {0}")]
    Memory(String),

    #[error("Loop engine error: {0}")]
    LoopEngine(String),

    #[error("Meta-SONA error: {0}")]
    MetaSONA(String),

    #[error("Event bus error: {0}")]
    EventBus(String),

    #[error("API error: {0}")]
    API(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Runtime is not initialized")]
    NotInitialized,

    #[error("Runtime is already running")]
    AlreadyRunning,

    #[error("Runtime is not running")]
    NotRunning,

    #[error("Operation timeout")]
    Timeout,

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Errors that can occur during configuration
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid configuration: {0}")]
    Invalid(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Validation error: {0}")]
    Validation(String),
}

/// Errors that can occur during API operations
#[derive(Debug, Error)]
pub enum APIError {
    #[error("Runtime error: {0}")]
    Runtime(#[from] RuntimeError),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Operation not supported: {0}")]
    NotSupported(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type for runtime operations
pub type RuntimeResult<T> = Result<T, RuntimeError>;

/// Result type for configuration operations
pub type ConfigResult<T> = Result<T, ConfigError>;

/// Result type for API operations
pub type APIResult<T> = Result<T, APIError>;
