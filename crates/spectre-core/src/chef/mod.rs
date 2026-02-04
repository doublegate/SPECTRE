//! CyberChef-MCP integration module
//!
//! This module provides the interface for data transformation using CyberChef via MCP.
//!
//! # Integration Notes
//!
//! CyberChef-MCP runs as a Docker container exposing 463 data operations via the MCP protocol.
//! The client connects to the container and sends operation requests.

mod docker;
mod mcp;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub use docker::DockerManager;
pub use mcp::McpClient;

use crate::config::Config;
use crate::error::ChefError;

/// Create a CyberChef MCP client
pub async fn create_chef_client(config: &Config) -> crate::Result<Box<dyn ChefClient>> {
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

        let recipe: Recipe = serde_json::from_str(&content)?;
        Ok(recipe)
    }

    /// Load a recipe by name from config directory
    pub fn load(name: &str, config: &Config) -> crate::Result<Self> {
        let recipe_dir = config.chef.recipe_dir.clone().unwrap_or_else(|| {
            directories::ProjectDirs::from("com", "spectre", "spectre")
                .map(|dirs| dirs.config_dir().join("recipes"))
                .unwrap_or_else(|| PathBuf::from("~/.config/spectre/recipes"))
        });

        let path = recipe_dir.join(format!("{}.json", name));
        Self::load_from_file(&path)
    }

    /// Save the recipe
    pub fn save(&self, config: &Config) -> crate::Result<()> {
        let recipe_dir = config.chef.recipe_dir.clone().unwrap_or_else(|| {
            directories::ProjectDirs::from("com", "spectre", "spectre")
                .map(|dirs| dirs.config_dir().join("recipes"))
                .unwrap_or_else(|| PathBuf::from("~/.config/spectre/recipes"))
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
            directories::ProjectDirs::from("com", "spectre", "spectre")
                .map(|dirs| dirs.config_dir().join("recipes"))
                .unwrap_or_else(|| PathBuf::from("~/.config/spectre/recipes"))
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
            directories::ProjectDirs::from("com", "spectre", "spectre")
                .map(|dirs| dirs.config_dir().join("recipes"))
                .unwrap_or_else(|| PathBuf::from("~/.config/spectre/recipes"))
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
