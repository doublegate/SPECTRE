//! CyberChef-MCP integration module
//!
//! This module provides the interface for data transformation using CyberChef via MCP.
//!
//! # Architecture
//!
//! The chef module has a three-layer architecture:
//!
//! - **[`ChefClient`] trait** — the public interface for all CyberChef operations
//! - **[`mcp_adapter::McpChefClient`]** — real MCP JSON-RPC 2.0 client over Docker stdio
//!   (production path; spawns `docker run -i --rm <image>` and communicates via stdin/stdout)
//! - **[`McpClient`]** — stub implementation with 6 local operations for testing without Docker
//!
//! # Integration Notes
//!
//! CyberChef-MCP v1.9.0 runs as a Docker container exposing 463+ data operations via
//! the Model Context Protocol (MCP). The protocol uses JSON-RPC 2.0 over stdio: the
//! client writes JSON requests to the container's stdin and reads responses from
//! its stdout, one JSON object per line.
//!
//! ## v1.9.0 Features
//!
//! - **Worker thread pool**: CPU-intensive operations (AES, RSA, SHA, compression)
//!   can be routed to a Piscina worker pool via `ENABLE_WORKERS=true`
//! - **Streaming progress**: Operations report progress via MCP notifications
//!   when a `progressToken` is provided in request metadata
//! - **Configurable transport**: Supports stdio (default) or Streamable HTTP
//!   via `CYBERCHEF_TRANSPORT=http`
//!
//! The [`DockerManager`] handles container lifecycle (pull, start, stop) via the
//! bollard Docker API, while [`mcp_adapter::McpChefClient`] handles the MCP
//! transport and protocol.

mod docker;
mod mcp;
pub mod mcp_adapter;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub use docker::DockerManager;
pub use mcp::McpClient;
pub use mcp_adapter::McpChefClient;

use crate::config::Config;
use crate::error::ChefError;

/// Create a CyberChef MCP client (production path).
///
/// This spawns a Docker container running CyberChef-MCP and communicates with it
/// via the MCP protocol (JSON-RPC 2.0 over stdio). Requires Docker to be installed
/// and the configured image to be available.
///
/// v1.9.0 features (worker pool, streaming, transport) are controlled by
/// `ChefConfig` fields and passed as environment variables to the container.
///
/// For testing without Docker, use [`create_stub_chef_client`] instead.
pub async fn create_chef_client(config: &Config) -> crate::Result<Box<dyn ChefClient>> {
    let client =
        McpChefClient::connect(&config.chef.docker_image, config.chef.timeout, &config.chef)
            .await?;
    Ok(Box::new(client))
}

/// Create a stub CyberChef client for testing without Docker.
///
/// This returns the local stub implementation that handles 6 common operations
/// (base64, hex, URL encode/decode) without requiring a running Docker container
/// or MCP server.
pub async fn create_stub_chef_client(config: &Config) -> crate::Result<Box<dyn ChefClient>> {
    let client = McpClient::connect(&config.chef.mcp_endpoint, config.chef.timeout).await?;
    Ok(Box::new(client))
}

/// CyberChef client trait
#[async_trait::async_trait]
pub trait ChefClient: Send + Sync {
    /// Execute a single operation
    async fn execute(
        &self,
        operation: &str,
        input: &[u8],
        args: &HashMap<String, String>,
    ) -> crate::Result<Vec<u8>>;

    /// Execute a recipe (chain of operations)
    async fn execute_recipe(&self, recipe: &Recipe, input: &[u8]) -> crate::Result<Vec<u8>>;

    /// Perform health check
    async fn health_check(&self) -> crate::Result<HealthStatus>;

    /// List available operations
    async fn list_operations(
        &self,
        category: Option<&str>,
        search: Option<&str>,
    ) -> crate::Result<Vec<OperationInfo>>;

    /// Get help for an operation
    async fn operation_help(&self, operation: &str) -> crate::Result<OperationHelp>;

    /// Get worker thread pool statistics (v1.9.0)
    ///
    /// Returns pool stats if workers are enabled (`ENABLE_WORKERS=true`),
    /// or a disabled status otherwise.
    async fn worker_stats(&self) -> crate::Result<WorkerStats>;
}

/// Health status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Whether the service is healthy
    pub healthy: bool,
    /// Container status
    pub container_status: String,
    /// MCP protocol version
    pub mcp_version: String,
    /// Number of available operations
    pub operation_count: usize,
}

/// Worker thread pool statistics (v1.9.0)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerStats {
    /// Whether the worker pool is enabled
    pub enabled: bool,
    /// Number of active threads (None if disabled)
    pub threads: Option<usize>,
    /// Total completed tasks (None if disabled)
    pub completed: Option<u64>,
    /// Tasks waiting in queue (None if disabled)
    pub waiting: Option<usize>,
    /// Pool utilization ratio 0.0-1.0 (None if disabled)
    pub utilization: Option<f64>,
    /// Optional status message
    pub message: Option<String>,
}

/// Operation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationInfo {
    /// Operation name
    pub name: String,
    /// Category
    pub category: String,
    /// Description
    pub description: String,
}

/// Operation help information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationHelp {
    /// Operation name
    pub name: String,
    /// Category
    pub category: String,
    /// Full description
    pub description: String,
    /// Arguments
    pub args: Vec<OperationArg>,
    /// Usage examples
    pub examples: Vec<String>,
}

/// Operation argument information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationArg {
    /// Argument name
    pub name: String,
    /// Argument type
    pub arg_type: String,
    /// Description
    pub description: String,
    /// Default value
    pub default: Option<String>,
    /// Whether required
    pub required: bool,
}

/// Recipe containing a chain of operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    /// Recipe name
    pub name: String,
    /// Operations to execute in order
    pub operations: Vec<RecipeOperation>,
}

/// Single operation in a recipe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeOperation {
    /// Operation name
    pub name: String,
    /// Operation arguments
    #[serde(default)]
    pub args: HashMap<String, String>,
}

impl std::fmt::Display for RecipeOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.args.is_empty() {
            write!(f, "{}", self.name)
        } else {
            let args: Vec<String> = self
                .args
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            write!(f, "{}({})", self.name, args.join(", "))
        }
    }
}

impl Recipe {
    /// Create a new recipe
    pub fn new(name: &str, operations: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            operations: operations
                .into_iter()
                .map(|op| RecipeOperation {
                    name: op,
                    args: HashMap::new(),
                })
                .collect(),
        }
    }

    /// Load a recipe from file
    pub fn load_from_file(path: &Path) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path).map_err(|e| {
            crate::SpectreError::Chef(ChefError::RecipeError(format!(
                "Failed to read recipe file: {}",
                e
            )))
        })?;

        let recipe: Self = serde_json::from_str(&content)?;
        Ok(recipe)
    }

    /// Load a recipe by name from config directory
    pub fn load(name: &str, config: &Config) -> crate::Result<Self> {
        let recipe_dir = config.chef.recipe_dir.clone().unwrap_or_else(|| {
            directories::ProjectDirs::from("com", "spectre", "spectre").map_or_else(
                || PathBuf::from("~/.config/spectre/recipes"),
                |dirs| dirs.config_dir().join("recipes"),
            )
        });

        let path = recipe_dir.join(format!("{}.json", name));
        Self::load_from_file(&path)
    }

    /// Save the recipe
    pub fn save(&self, config: &Config) -> crate::Result<()> {
        let recipe_dir = config.chef.recipe_dir.clone().unwrap_or_else(|| {
            directories::ProjectDirs::from("com", "spectre", "spectre").map_or_else(
                || PathBuf::from("~/.config/spectre/recipes"),
                |dirs| dirs.config_dir().join("recipes"),
            )
        });

        std::fs::create_dir_all(&recipe_dir)?;

        let path = recipe_dir.join(format!("{}.json", self.name));
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;

        Ok(())
    }

    /// List all saved recipes
    pub fn list(config: &Config) -> crate::Result<Vec<String>> {
        let recipe_dir = config.chef.recipe_dir.clone().unwrap_or_else(|| {
            directories::ProjectDirs::from("com", "spectre", "spectre").map_or_else(
                || PathBuf::from("~/.config/spectre/recipes"),
                |dirs| dirs.config_dir().join("recipes"),
            )
        });

        if !recipe_dir.exists() {
            return Ok(Vec::new());
        }

        let mut recipes = Vec::new();
        for entry in std::fs::read_dir(recipe_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "json") {
                if let Some(name) = path.file_stem() {
                    recipes.push(name.to_string_lossy().to_string());
                }
            }
        }

        Ok(recipes)
    }

    /// Delete a recipe
    pub fn delete(name: &str, config: &Config) -> crate::Result<()> {
        let recipe_dir = config.chef.recipe_dir.clone().unwrap_or_else(|| {
            directories::ProjectDirs::from("com", "spectre", "spectre").map_or_else(
                || PathBuf::from("~/.config/spectre/recipes"),
                |dirs| dirs.config_dir().join("recipes"),
            )
        });

        let path = recipe_dir.join(format!("{}.json", name));
        std::fs::remove_file(path)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recipe_new() {
        let recipe = Recipe::new(
            "test",
            vec!["From_Base64".to_string(), "To_Hex".to_string()],
        );
        assert_eq!(recipe.name, "test");
        assert_eq!(recipe.operations.len(), 2);
    }

    #[test]
    fn test_worker_stats_disabled() {
        let stats = WorkerStats {
            enabled: false,
            threads: None,
            completed: None,
            waiting: None,
            utilization: None,
            message: Some("Worker pool is not enabled".to_string()),
        };
        assert!(!stats.enabled);
        assert!(stats.threads.is_none());
        assert!(stats.message.is_some());
    }

    #[test]
    fn test_worker_stats_enabled() {
        let stats = WorkerStats {
            enabled: true,
            threads: Some(4),
            completed: Some(100),
            waiting: Some(2),
            utilization: Some(0.75),
            message: None,
        };
        assert!(stats.enabled);
        assert_eq!(stats.threads, Some(4));
        assert_eq!(stats.completed, Some(100));
        assert_eq!(stats.utilization, Some(0.75));
    }

    #[test]
    fn test_worker_stats_serialization() {
        let stats = WorkerStats {
            enabled: true,
            threads: Some(4),
            completed: Some(50),
            waiting: Some(0),
            utilization: Some(0.5),
            message: None,
        };
        let json = serde_json::to_string(&stats).unwrap();
        let deserialized: WorkerStats = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.enabled, stats.enabled);
        assert_eq!(deserialized.threads, stats.threads);
        assert_eq!(deserialized.completed, stats.completed);
    }

    #[test]
    fn test_recipe_operation_display() {
        let op = RecipeOperation {
            name: "AES_Encrypt".to_string(),
            args: {
                let mut args = HashMap::new();
                args.insert("key".to_string(), "secret".to_string());
                args
            },
        };
        let display = format!("{}", op);
        assert!(display.contains("AES_Encrypt"));
        assert!(display.contains("key=secret"));
    }
}
