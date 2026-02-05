//! Error types for SPECTRE
//!
//! This module defines the error types used throughout the SPECTRE system.

use thiserror::Error;

/// Result type alias using SpectreError
pub type Result<T> = std::result::Result<T, SpectreError>;

/// Main error type for SPECTRE operations
#[derive(Error, Debug)]
pub enum SpectreError {
    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    /// Scan error
    #[error("Scan error: {0}")]
    Scan(#[from] ScanError),

    /// Chef/CyberChef error
    #[error("Chef error: {0}")]
    Chef(#[from] ChefError),

    /// Communications error
    #[error("Communications error: {0}")]
    Comms(#[from] CommsError),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Invalid input error
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Operation not supported
    #[error("Operation not supported: {0}")]
    NotSupported(String),

    /// External tool error
    #[error("External tool error: {0}")]
    ExternalTool(String),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),

    /// Job error
    #[error("Job error: {0}")]
    Job(#[from] JobError),

    /// Pipeline error
    #[error("Pipeline error: {0}")]
    Pipeline(#[from] PipelineError),

    /// Campaign error
    #[error("Campaign error: {0}")]
    Campaign(#[from] CampaignError),

    /// Plugin error
    #[error("Plugin error: {0}")]
    Plugin(#[from] PluginError),

    /// Target error
    #[error("Target error: {0}")]
    Target(#[from] TargetError),

    /// Orchestration error
    #[error("Orchestration error: {0}")]
    Orchestration(#[from] OrchestrationError),

    /// Workflow error
    #[error("Workflow error: {0}")]
    Workflow(#[from] WorkflowError),

    /// Recipe error
    #[error("Recipe error: {0}")]
    Recipe(#[from] RecipeError),

    /// Report error
    #[error("Report error: {0}")]
    Report(#[from] ReportError),

    /// Export error
    #[error("Export error: {0}")]
    Export(#[from] ExportError),
}

/// Configuration-related errors
#[derive(Error, Debug)]
pub enum ConfigError {
    /// Config file not found
    #[error("Configuration file not found: {path}")]
    NotFound {
        /// Path that was searched
        path: String,
    },

    /// Config parse error
    #[error("Failed to parse configuration: {message}")]
    ParseError {
        /// Parse error message
        message: String,
        /// Source of the error
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Invalid configuration value
    #[error("Invalid configuration value for '{key}': {message}")]
    InvalidValue {
        /// Configuration key
        key: String,
        /// Error message
        message: String,
    },

    /// Missing required configuration
    #[error("Missing required configuration: {key}")]
    MissingRequired {
        /// Configuration key
        key: String,
    },

    /// Environment variable error
    #[error("Environment variable error: {0}")]
    EnvVar(String),
}

/// Scan-related errors
#[derive(Error, Debug)]
pub enum ScanError {
    /// Invalid target specification
    #[error("Invalid target: {0}")]
    InvalidTarget(String),

    /// Invalid port specification
    #[error("Invalid port specification: {0}")]
    InvalidPort(String),

    /// Permission denied (e.g., raw sockets)
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Network error during scan
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Scan timeout
    #[error("Scan timeout after {seconds} seconds")]
    Timeout {
        /// Timeout duration
        seconds: u64,
    },

    /// Scanner not available
    #[error("Scanner not available: {0}")]
    NotAvailable(String),

    /// Script error
    #[error("Script error: {0}")]
    ScriptError(String),
}

/// CyberChef-related errors
#[derive(Error, Debug)]
pub enum ChefError {
    /// MCP connection error
    #[error("MCP connection error: {0}")]
    ConnectionError(String),

    /// Docker error
    #[error("Docker error: {0}")]
    DockerError(String),

    /// Invalid operation
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    /// Operation execution error
    #[error("Operation failed: {operation}: {message}")]
    OperationFailed {
        /// Operation name
        operation: String,
        /// Error message
        message: String,
    },

    /// Recipe error
    #[error("Recipe error: {0}")]
    RecipeError(String),

    /// Input error
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

/// Communications-related errors
#[derive(Error, Debug)]
pub enum CommsError {
    /// Identity not found
    #[error("Identity not found: {0}")]
    IdentityNotFound(String),

    /// Peer not found
    #[error("Peer not found: {0}")]
    PeerNotFound(String),

    /// Key error
    #[error("Key error: {0}")]
    KeyError(String),

    /// Encryption error
    #[error("Encryption error: {0}")]
    EncryptionError(String),

    /// Decryption error
    #[error("Decryption error: {0}")]
    DecryptionError(String),

    /// Connection error
    #[error("Connection error: {0}")]
    ConnectionError(String),

    /// Authentication failed
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    /// Timeout
    #[error("Operation timed out")]
    Timeout,

    /// Message too large
    #[error("Message too large: {size} bytes (max: {max})")]
    MessageTooLarge {
        /// Actual size
        size: usize,
        /// Maximum allowed
        max: usize,
    },
}

/// Target management errors
#[derive(Error, Debug)]
pub enum TargetError {
    /// Invalid CIDR specification
    #[error("Invalid CIDR: {0}")]
    InvalidCidr(String),

    /// Target out of scope
    #[error("Target out of scope: {0}")]
    OutOfScope(String),

    /// DNS resolution failure
    #[error("DNS resolution failed for {host}: {message}")]
    DnsResolution {
        /// Hostname that failed resolution
        host: String,
        /// Error message
        message: String,
    },

    /// Target file parse error
    #[error("Failed to parse target file: {0}")]
    FileParseError(String),
}

/// Job orchestration errors
#[derive(Error, Debug)]
pub enum JobError {
    /// Job not found
    #[error("Job not found: {0}")]
    NotFound(String),

    /// Invalid state transition
    #[error("Invalid state transition from {from} to {to}")]
    InvalidTransition {
        /// Current state
        from: String,
        /// Requested state
        to: String,
    },

    /// Job cancelled
    #[error("Job cancelled: {0}")]
    Cancelled(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    /// Manager shutdown
    #[error("Job manager is shutting down")]
    ManagerShutdown,
}

/// Pipeline errors
#[derive(Error, Debug)]
pub enum PipelineError {
    /// Stage execution error
    #[error("Stage '{stage}' failed: {message}")]
    StageError {
        /// Stage name
        stage: String,
        /// Error message
        message: String,
    },

    /// Pipeline configuration error
    #[error("Pipeline configuration error: {0}")]
    ConfigError(String),

    /// No stages configured
    #[error("Pipeline has no stages configured")]
    NoStages,
}

/// Campaign management errors
#[derive(Error, Debug)]
pub enum CampaignError {
    /// Campaign not found
    #[error("Campaign not found: {0}")]
    NotFound(String),

    /// Database error
    #[error("Database error: {0}")]
    DatabaseError(String),

    /// Invalid campaign state
    #[error("Invalid campaign state: {0}")]
    InvalidState(String),

    /// Artifact error
    #[error("Artifact error: {0}")]
    ArtifactError(String),

    /// Export/import error
    #[error("Export/import error: {0}")]
    ExportError(String),
}

/// Plugin system errors
#[derive(Error, Debug)]
pub enum PluginError {
    /// Plugin not found
    #[error("Plugin not found: {0}")]
    NotFound(String),

    /// Lua runtime error
    #[error("Lua runtime error: {0}")]
    RuntimeError(String),

    /// Plugin sandbox violation
    #[error("Sandbox violation: {0}")]
    SandboxViolation(String),

    /// Invalid plugin manifest
    #[error("Invalid plugin manifest: {0}")]
    InvalidManifest(String),

    /// Permission denied
    #[error("Plugin permission denied: {0}")]
    PermissionDenied(String),

    /// Resource limit exceeded
    #[error("Resource limit exceeded: {0}")]
    ResourceLimit(String),
}

/// Orchestration errors
#[derive(Error, Debug)]
pub enum OrchestrationError {
    /// Scan chain error
    #[error("Scan chain error: {0}")]
    ChainError(String),

    /// Template not found
    #[error("Scan template not found: {0}")]
    TemplateNotFound(String),

    /// Invalid schedule expression
    #[error("Invalid schedule: {0}")]
    InvalidSchedule(String),

    /// Checkpoint error
    #[error("Checkpoint error: {0}")]
    CheckpointError(String),

    /// Profile error
    #[error("Profile error: {0}")]
    ProfileError(String),
}

/// Workflow errors
#[derive(Error, Debug)]
pub enum WorkflowError {
    /// Parse error in workflow definition
    #[error("Workflow parse error: {0}")]
    ParseError(String),

    /// Step execution error
    #[error("Workflow step '{step}' failed: {message}")]
    StepError {
        /// Step name
        step: String,
        /// Error message
        message: String,
    },

    /// Variable not found
    #[error("Variable not found: {0}")]
    VariableNotFound(String),

    /// Condition evaluation error
    #[error("Condition error: {0}")]
    ConditionError(String),

    /// Template not found
    #[error("Workflow template not found: {0}")]
    TemplateNotFound(String),

    /// Persistence error
    #[error("Workflow persistence error: {0}")]
    PersistenceError(String),
}

/// Recipe errors
#[derive(Error, Debug)]
pub enum RecipeError {
    /// Recipe not found
    #[error("Recipe not found: {0}")]
    NotFound(String),

    /// Invalid recipe format
    #[error("Invalid recipe format: {0}")]
    InvalidFormat(String),

    /// Validation error
    #[error("Recipe validation error: {0}")]
    ValidationError(String),

    /// Storage error
    #[error("Recipe storage error: {0}")]
    StorageError(String),

    /// Version error
    #[error("Recipe version error: {0}")]
    VersionError(String),
}

/// Report errors
#[derive(Error, Debug)]
pub enum ReportError {
    /// Template error
    #[error("Report template error: {0}")]
    TemplateError(String),

    /// Generation error
    #[error("Report generation error: {0}")]
    GenerationError(String),

    /// Invalid report configuration
    #[error("Invalid report configuration: {0}")]
    InvalidConfig(String),
}

/// Export errors
#[derive(Error, Debug)]
pub enum ExportError {
    /// Format error
    #[error("Export format error: {0}")]
    FormatError(String),

    /// Template error
    #[error("Export template error: {0}")]
    TemplateError(String),

    /// Scheduling error
    #[error("Export scheduling error: {0}")]
    SchedulingError(String),

    /// IO error during export
    #[error("Export IO error: {0}")]
    IoError(String),
}

impl From<toml::de::Error> for SpectreError {
    fn from(err: toml::de::Error) -> Self {
        Self::Config(ConfigError::ParseError {
            message: err.to_string(),
            source: Some(Box::new(err)),
        })
    }
}

impl From<serde_json::Error> for SpectreError {
    fn from(err: serde_json::Error) -> Self {
        Self::Serialization(err.to_string())
    }
}

impl From<toml::ser::Error> for SpectreError {
    fn from(err: toml::ser::Error) -> Self {
        Self::Serialization(err.to_string())
    }
}

impl From<base64::DecodeError> for SpectreError {
    fn from(err: base64::DecodeError) -> Self {
        Self::Comms(CommsError::KeyError(err.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = SpectreError::InvalidInput("test".to_string());
        assert_eq!(err.to_string(), "Invalid input: test");
    }

    #[test]
    fn test_config_error() {
        let err = ConfigError::NotFound {
            path: "/etc/spectre.toml".to_string(),
        };
        assert!(err.to_string().contains("/etc/spectre.toml"));
    }

    #[test]
    fn test_scan_error() {
        let err = ScanError::InvalidTarget("bad target".to_string());
        assert!(err.to_string().contains("bad target"));
    }

    #[test]
    fn test_comms_error_timeout() {
        let err = CommsError::Timeout;
        assert_eq!(err.to_string(), "Operation timed out");
    }
}
