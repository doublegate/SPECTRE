use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tauri::{command, State};

use crate::state::AppState;

/// Chef operation information (mirrors spectre_core::chef::OperationInfo).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChefOperation {
    pub name: String,
    pub category: String,
    pub description: String,
}

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

/// Chef health status.
#[derive(Debug, Serialize)]
pub struct ChefStatus {
    pub healthy: bool,
    pub container_status: String,
    pub operation_count: usize,
}

/// Execute a CyberChef operation.
///
/// Note: Full Docker/MCP integration requires container management.
/// For Sprint 5.6, this provides basic transformations via stub implementation.
#[command]
pub async fn execute_chef(
    _state: State<'_, Arc<AppState>>,
    request: ChefRequest,
) -> Result<ChefResult, String> {
    tracing::info!(operation = %request.operation, "Executing chef operation");

    // Basic transformations for common operations (stub implementation)
    let output = match request.operation.as_str() {
        "From_Base64" => {
            let decoded = base64::decode(&request.input)
                .map_err(|e| format!("Base64 decode error: {}", e))?;
            String::from_utf8(decoded).map_err(|e| format!("UTF-8 error: {}", e))?
        }
        "To_Base64" => base64::encode(&request.input),
        "From_Hex" => {
            let decoded = hex::decode(&request.input)
                .map_err(|e| format!("Hex decode error: {}", e))?;
            String::from_utf8(decoded).map_err(|e| format!("UTF-8 error: {}", e))?
        }
        "To_Hex" => hex::encode(&request.input),
        "URL_Decode" => urlencoding::decode(&request.input)
            .map_err(|e| format!("URL decode error: {}", e))?
            .to_string(),
        "URL_Encode" => urlencoding::encode(&request.input).to_string(),
        _ => return Err(format!("Unsupported operation: {}", request.operation)),
    };

    Ok(ChefResult {
        operation: request.operation,
        output,
        success: true,
    })
}

/// List available CyberChef operations.
///
/// Returns a comprehensive list of operations available in CyberChef-MCP v1.9.0.
/// Full integration would query the MCP server; for now we return a curated list.
#[command]
pub async fn list_chef_operations(
    _state: State<'_, Arc<AppState>>,
    category: Option<String>,
) -> Result<Vec<ChefOperation>, String> {
    let all_operations = vec![
        // Encoding
        ChefOperation {
            name: "From_Base64".to_string(),
            category: "Encoding".to_string(),
            description: "Decode Base64 encoded data".to_string(),
        },
        ChefOperation {
            name: "To_Base64".to_string(),
            category: "Encoding".to_string(),
            description: "Encode data as Base64".to_string(),
        },
        ChefOperation {
            name: "From_Hex".to_string(),
            category: "Encoding".to_string(),
            description: "Decode hexadecimal data".to_string(),
        },
        ChefOperation {
            name: "To_Hex".to_string(),
            category: "Encoding".to_string(),
            description: "Encode data as hexadecimal".to_string(),
        },
        ChefOperation {
            name: "URL_Decode".to_string(),
            category: "Encoding".to_string(),
            description: "Decode URL-encoded data".to_string(),
        },
        ChefOperation {
            name: "URL_Encode".to_string(),
            category: "Encoding".to_string(),
            description: "URL-encode data".to_string(),
        },
        // Hashing
        ChefOperation {
            name: "MD5".to_string(),
            category: "Hashing".to_string(),
            description: "Calculate MD5 hash".to_string(),
        },
        ChefOperation {
            name: "SHA1".to_string(),
            category: "Hashing".to_string(),
            description: "Calculate SHA-1 hash".to_string(),
        },
        ChefOperation {
            name: "SHA256".to_string(),
            category: "Hashing".to_string(),
            description: "Calculate SHA-256 hash".to_string(),
        },
        ChefOperation {
            name: "SHA512".to_string(),
            category: "Hashing".to_string(),
            description: "Calculate SHA-512 hash".to_string(),
        },
        // Encryption
        ChefOperation {
            name: "AES_Encrypt".to_string(),
            category: "Encryption".to_string(),
            description: "AES encryption".to_string(),
        },
        ChefOperation {
            name: "AES_Decrypt".to_string(),
            category: "Encryption".to_string(),
            description: "AES decryption".to_string(),
        },
        ChefOperation {
            name: "XOR".to_string(),
            category: "Encryption".to_string(),
            description: "XOR cipher".to_string(),
        },
        // Compression
        ChefOperation {
            name: "Gzip".to_string(),
            category: "Compression".to_string(),
            description: "Gzip compression".to_string(),
        },
        ChefOperation {
            name: "Gunzip".to_string(),
            category: "Compression".to_string(),
            description: "Gzip decompression".to_string(),
        },
    ];

    // Filter by category if specified
    let filtered = if let Some(cat) = category {
        all_operations
            .into_iter()
            .filter(|op| op.category == cat)
            .collect()
    } else {
        all_operations
    };

    Ok(filtered)
}

/// Get Chef MCP status.
#[command]
pub async fn get_chef_status(_state: State<'_, Arc<AppState>>) -> Result<ChefStatus, String> {
    // Stub: In production, this would check Docker container status
    Ok(ChefStatus {
        healthy: true,
        container_status: "stub".to_string(),
        operation_count: 15, // Matches list_chef_operations count
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_decode() {
        let decoded = base64::decode("SGVsbG8=").expect("base64 decode failed");
        let text = String::from_utf8(decoded).expect("utf8 conversion failed");
        assert_eq!(text, "Hello");
    }

    #[test]
    fn test_base64_encode() {
        let encoded = base64::encode("Hello");
        assert_eq!(encoded, "SGVsbG8=");
    }

    #[test]
    fn test_hex_encode() {
        let encoded = hex::encode("Hello");
        assert_eq!(encoded, "48656c6c6f");
    }

    #[test]
    fn test_hex_decode() {
        let decoded = hex::decode("48656c6c6f").expect("hex decode failed");
        let text = String::from_utf8(decoded).expect("utf8 conversion failed");
        assert_eq!(text, "Hello");
    }

    #[test]
    fn test_url_encode() {
        let encoded = urlencoding::encode("hello world");
        assert_eq!(encoded, "hello%20world");
    }

    #[test]
    fn test_chef_operation_struct() {
        let op = ChefOperation {
            name: "From_Base64".to_string(),
            category: "Encoding".to_string(),
            description: "Decode Base64".to_string(),
        };
        assert_eq!(op.name, "From_Base64");
        assert_eq!(op.category, "Encoding");
    }

    #[test]
    fn test_chef_result_struct() {
        let result = ChefResult {
            operation: "test".to_string(),
            output: "output".to_string(),
            success: true,
        };
        assert_eq!(result.operation, "test");
        assert!(result.success);
    }
}
