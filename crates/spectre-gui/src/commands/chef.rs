use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tauri::command;

/// Chef operation request from the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChefRequest {
    pub operation: String,
    pub input: String,
    pub args: Option<HashMap<String, String>>,
}

/// Chef operation result.
#[derive(Debug, Serialize)]
pub struct ChefResult {
    pub operation: String,
    pub output: String,
    pub success: bool,
}

/// Execute a CyberChef operation. Stub - will be wired in Sprint 5.6.
#[command]
pub async fn execute_chef(request: ChefRequest) -> Result<ChefResult, String> {
    tracing::info!(operation = %request.operation, "Chef operation (stub)");
    Ok(ChefResult {
        operation: request.operation,
        output: String::new(),
        success: true,
    })
}

/// List available CyberChef operations. Stub.
#[command]
pub async fn list_chef_operations() -> Result<Vec<String>, String> {
    Ok(vec![
        "From_Base64".to_string(),
        "To_Base64".to_string(),
        "From_Hex".to_string(),
        "To_Hex".to_string(),
        "URL_Decode".to_string(),
        "URL_Encode".to_string(),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_chef_stub() {
        let request = ChefRequest {
            operation: "From_Base64".to_string(),
            input: "SGVsbG8=".to_string(),
            args: None,
        };
        let result = execute_chef(request).await;
        assert!(result.is_ok());
        let chef_result = result.expect("Test assertion failed");
        assert_eq!(chef_result.operation, "From_Base64");
        assert!(chef_result.success);
    }

    #[tokio::test]
    async fn test_list_chef_operations_stub() {
        let result = list_chef_operations().await;
        assert!(result.is_ok());
        let ops = result.expect("Test assertion failed");
        assert!(!ops.is_empty());
        assert!(ops.contains(&"From_Base64".to_string()));
    }
}
