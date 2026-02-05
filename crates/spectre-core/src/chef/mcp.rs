//! MCP client implementation for CyberChef

use std::collections::HashMap;
use std::time::Duration;

use async_trait::async_trait;
use tracing::{debug, info, instrument};

use super::{ChefClient, HealthStatus, OperationArg, OperationHelp, OperationInfo, Recipe};
use crate::error::ChefError;

/// MCP client for CyberChef-MCP
pub struct McpClient {
    #[allow(dead_code)]
    endpoint: String,
    #[allow(dead_code)]
    timeout: Duration,
    // In a real implementation, this would hold the actual MCP connection
    // For now, this is a stub that simulates MCP behavior
}

impl McpClient {
    /// Connect to the MCP server
    #[instrument(skip_all, fields(endpoint = %endpoint))]
    pub async fn connect(endpoint: &str, timeout_secs: u64) -> crate::Result<Self> {
        info!("Connecting to CyberChef-MCP");
        debug!(endpoint = %endpoint, "MCP endpoint");

        // TODO: When actual MCP SDK is integrated, establish real connection
        // For now, just store the endpoint

        Ok(Self {
            endpoint: endpoint.to_string(),
            timeout: Duration::from_secs(timeout_secs),
        })
    }
}

#[async_trait]
impl ChefClient for McpClient {
    #[instrument(skip(self, input, args), fields(operation = %operation, input_len = input.len()))]
    async fn execute(
        &self,
        operation: &str,
        input: &[u8],
        args: &HashMap<String, String>,
    ) -> crate::Result<Vec<u8>> {
        info!("Executing CyberChef operation");
        debug!(args = ?args, "Operation arguments");

        // TODO: When actual MCP SDK is integrated, send real request
        // For now, simulate some common operations

        let result = match operation {
            "From_Base64" => {
                use base64::Engine;
                base64::engine::general_purpose::STANDARD
                    .decode(input)
                    .map_err(|e| {
                        crate::SpectreError::Chef(ChefError::OperationFailed {
                            operation: operation.to_string(),
                            message: e.to_string(),
                        })
                    })?
            },
            "To_Base64" => {
                use base64::Engine;
                base64::engine::general_purpose::STANDARD
                    .encode(input)
                    .into_bytes()
            },
            "From_Hex" | "From_Hexdump" => hex::decode(input).map_err(|e| {
                crate::SpectreError::Chef(ChefError::OperationFailed {
                    operation: operation.to_string(),
                    message: e.to_string(),
                })
            })?,
            "To_Hex" | "To_Hexdump" => hex::encode(input).into_bytes(),
            "URL_Decode" => {
                let input_str = String::from_utf8_lossy(input);
                urlencoding::decode(&input_str)
                    .map_err(|e| {
                        crate::SpectreError::Chef(ChefError::OperationFailed {
                            operation: operation.to_string(),
                            message: e.to_string(),
                        })
                    })?
                    .into_owned()
                    .into_bytes()
            },
            "URL_Encode" => {
                let input_str = String::from_utf8_lossy(input);
                urlencoding::encode(&input_str).into_owned().into_bytes()
            },
            _ => {
                // For unknown operations, return input unchanged with a warning
                tracing::warn!(
                    operation = %operation,
                    "Operation not implemented in stub, returning input unchanged"
                );
                input.to_vec()
            },
        };

        info!(output_len = result.len(), "Operation complete");
        Ok(result)
    }

    #[instrument(skip(self, input), fields(recipe = %recipe.name, operations = recipe.operations.len()))]
    async fn execute_recipe(&self, recipe: &Recipe, input: &[u8]) -> crate::Result<Vec<u8>> {
        info!("Executing CyberChef recipe");

        let mut data = input.to_vec();

        for op in &recipe.operations {
            debug!(operation = %op.name, "Executing recipe step");
            data = self.execute(&op.name, &data, &op.args).await?;
        }

        info!(output_len = data.len(), "Recipe complete");
        Ok(data)
    }

    async fn health_check(&self) -> crate::Result<HealthStatus> {
        // TODO: When actual MCP SDK is integrated, perform real health check
        // For now, return a simulated healthy status

        Ok(HealthStatus {
            healthy: true,
            container_status: "running".to_string(),
            mcp_version: "1.0.0".to_string(),
            operation_count: 463,
        })
    }

    async fn list_operations(
        &self,
        category: Option<&str>,
        search: Option<&str>,
    ) -> crate::Result<Vec<OperationInfo>> {
        // TODO: When actual MCP SDK is integrated, fetch real operation list
        // For now, return a subset of common operations

        let all_operations = vec![
            OperationInfo {
                name: "From_Base64".to_string(),
                category: "Data Format".to_string(),
                description: "Decode Base64 encoded data".to_string(),
            },
            OperationInfo {
                name: "To_Base64".to_string(),
                category: "Data Format".to_string(),
                description: "Encode data as Base64".to_string(),
            },
            OperationInfo {
                name: "From_Hex".to_string(),
                category: "Data Format".to_string(),
                description: "Decode hexadecimal data".to_string(),
            },
            OperationInfo {
                name: "To_Hex".to_string(),
                category: "Data Format".to_string(),
                description: "Encode data as hexadecimal".to_string(),
            },
            OperationInfo {
                name: "URL_Decode".to_string(),
                category: "URL".to_string(),
                description: "Decode URL-encoded data".to_string(),
            },
            OperationInfo {
                name: "URL_Encode".to_string(),
                category: "URL".to_string(),
                description: "URL-encode data".to_string(),
            },
            OperationInfo {
                name: "AES_Encrypt".to_string(),
                category: "Encryption".to_string(),
                description: "Encrypt data using AES".to_string(),
            },
            OperationInfo {
                name: "AES_Decrypt".to_string(),
                category: "Encryption".to_string(),
                description: "Decrypt AES encrypted data".to_string(),
            },
            OperationInfo {
                name: "MD5".to_string(),
                category: "Hashing".to_string(),
                description: "Compute MD5 hash".to_string(),
            },
            OperationInfo {
                name: "SHA256".to_string(),
                category: "Hashing".to_string(),
                description: "Compute SHA-256 hash".to_string(),
            },
        ];

        let mut filtered = all_operations;

        // Filter by category
        if let Some(cat) = category {
            let cat_lower = cat.to_lowercase();
            filtered.retain(|op| op.category.to_lowercase().contains(&cat_lower));
        }

        // Filter by search
        if let Some(s) = search {
            let search_lower = s.to_lowercase();
            filtered.retain(|op| {
                op.name.to_lowercase().contains(&search_lower)
                    || op.description.to_lowercase().contains(&search_lower)
            });
        }

        Ok(filtered)
    }

    async fn operation_help(&self, operation: &str) -> crate::Result<OperationHelp> {
        // TODO: When actual MCP SDK is integrated, fetch real help
        // For now, return stub help

        Ok(OperationHelp {
            name: operation.to_string(),
            category: "Unknown".to_string(),
            description: format!("Help for {} operation", operation),
            args: vec![OperationArg {
                name: "input".to_string(),
                arg_type: "string".to_string(),
                description: "Input data".to_string(),
                default: None,
                required: true,
            }],
            examples: vec![format!("spectre chef {} --input \"data\"", operation)],
        })
    }

    async fn worker_stats(&self) -> crate::Result<super::WorkerStats> {
        // Stub always reports workers as disabled
        Ok(super::WorkerStats {
            enabled: false,
            threads: None,
            completed: None,
            waiting: None,
            utilization: None,
            message: Some("Stub client — worker pool not available".to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_from_base64() {
        let client = McpClient::connect("http://localhost:3001", 30)
            .await
            .unwrap();

        let input = b"SGVsbG8gV29ybGQ="; // "Hello World" in base64
        let result = client
            .execute("From_Base64", input, &HashMap::new())
            .await
            .unwrap();

        assert_eq!(result, b"Hello World");
    }

    #[tokio::test]
    async fn test_to_base64() {
        let client = McpClient::connect("http://localhost:3001", 30)
            .await
            .unwrap();

        let input = b"Hello World";
        let result = client
            .execute("To_Base64", input, &HashMap::new())
            .await
            .unwrap();

        assert_eq!(result, b"SGVsbG8gV29ybGQ=");
    }

    #[tokio::test]
    async fn test_to_hex() {
        let client = McpClient::connect("http://localhost:3001", 30)
            .await
            .unwrap();

        let input = b"AB";
        let result = client
            .execute("To_Hex", input, &HashMap::new())
            .await
            .unwrap();

        assert_eq!(result, b"4142");
    }

    #[tokio::test]
    async fn test_health_check() {
        let client = McpClient::connect("http://localhost:3001", 30)
            .await
            .unwrap();

        let health = client.health_check().await.unwrap();
        assert!(health.healthy);
    }

    #[tokio::test]
    async fn test_list_operations() {
        let client = McpClient::connect("http://localhost:3001", 30)
            .await
            .unwrap();

        let ops = client.list_operations(None, None).await.unwrap();
        assert!(!ops.is_empty());

        let filtered = client
            .list_operations(Some("Encryption"), None)
            .await
            .unwrap();
        assert!(filtered.iter().all(|op| op.category == "Encryption"));
    }

    #[tokio::test]
    async fn test_worker_stats_stub() {
        let client = McpClient::connect("http://localhost:3001", 30)
            .await
            .unwrap();

        let stats = client.worker_stats().await.unwrap();
        assert!(!stats.enabled);
        assert!(stats.message.is_some());
    }
}
