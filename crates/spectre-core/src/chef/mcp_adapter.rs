//! Real MCP client adapter for CyberChef-MCP
//!
//! This module implements a JSON-RPC 2.0 over stdio transport to communicate
//! with the CyberChef-MCP Docker container. The MCP (Model Context Protocol)
//! uses newline-delimited JSON messages over stdin/stdout of the container process.
//!
//! # Architecture
//!
//! ```text
//! McpChefClient
//!   -> McpTransport (manages subprocess stdio pipes)
//!     -> docker run -i --rm <image>  (the CyberChef-MCP container)
//! ```
//!
//! The transport spawns `docker run -i --rm <image>` as a subprocess with piped
//! stdin/stdout, then sends JSON-RPC requests and reads responses line by line.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::{oneshot, Mutex};
use tracing::{debug, info, instrument, warn};

use super::{ChefClient, HealthStatus, OperationArg, OperationHelp, OperationInfo, Recipe};
use crate::error::ChefError;

// ---------------------------------------------------------------------------
// JSON-RPC 2.0 message types
// ---------------------------------------------------------------------------

/// A JSON-RPC 2.0 request (with an id) or notification (without an id).
#[derive(Debug, Clone, Serialize)]
pub struct JsonRpcRequest {
    /// JSON-RPC version string (always "2.0")
    pub jsonrpc: &'static str,
    /// Request ID (None for notifications)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    /// Method name
    pub method: String,
    /// Optional parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

/// A JSON-RPC 2.0 response envelope. The server sends either `result` or `error`.
#[derive(Debug, Clone, Deserialize)]
pub struct JsonRpcResponse {
    /// JSON-RPC version string (always "2.0")
    #[allow(dead_code)]
    pub jsonrpc: String,
    /// Request ID this response corresponds to (None for notifications)
    pub id: Option<u64>,
    /// Successful result value
    pub result: Option<serde_json::Value>,
    /// Error object if the request failed
    pub error: Option<JsonRpcError>,
}

/// JSON-RPC 2.0 error object.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsonRpcError {
    /// Numeric error code
    pub code: i64,
    /// Human-readable message
    pub message: String,
    /// Optional structured data
    pub data: Option<serde_json::Value>,
}

impl std::fmt::Display for JsonRpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JSON-RPC error {}: {}", self.code, self.message)
    }
}

// ---------------------------------------------------------------------------
// MCP protocol types
// ---------------------------------------------------------------------------

/// MCP initialize result
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct McpInitializeResult {
    protocol_version: String,
    #[allow(dead_code)]
    capabilities: serde_json::Value,
    server_info: McpServerInfo,
}

/// MCP server info from initialization
#[derive(Debug, Deserialize)]
struct McpServerInfo {
    name: String,
    version: String,
}

/// MCP tool definition returned by tools/list
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct McpTool {
    name: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    input_schema: Option<serde_json::Value>,
}

/// MCP tools/list result
#[derive(Debug, Deserialize)]
struct McpToolsListResult {
    tools: Vec<McpTool>,
}

/// MCP content item from a tool call result
#[derive(Debug, Deserialize)]
struct McpContentItem {
    #[serde(rename = "type")]
    content_type: String,
    #[serde(default)]
    text: Option<String>,
}

/// MCP tools/call result
#[derive(Debug, Deserialize)]
struct McpToolCallResult {
    content: Vec<McpContentItem>,
    #[serde(default, rename = "isError")]
    is_error: bool,
}

// ---------------------------------------------------------------------------
// Operation name conversion helpers
// ---------------------------------------------------------------------------

/// Convert a SPECTRE operation name to the MCP tool name.
///
/// SPECTRE uses underscore-separated PascalCase like `From_Base64` or `AES_Encrypt`.
/// MCP tools are named `cyberchef_` + snake_case: `cyberchef_from_base64`, `cyberchef_aes_encrypt`.
///
/// Conversion rules:
/// 1. Replace `_` with ` ` to get words
/// 2. Lowercase all words
/// 3. Join with `_`
/// 4. Prepend `cyberchef_`
pub fn spectre_op_to_mcp_tool(op: &str) -> String {
    let lower = op.to_lowercase();
    format!("cyberchef_{}", lower)
}

/// Convert an MCP tool name back to SPECTRE operation format.
///
/// `cyberchef_from_base64` -> `From_Base64`
///
/// Conversion rules:
/// 1. Strip the `cyberchef_` prefix
/// 2. Split on `_`
/// 3. Title-case each word
/// 4. Join with `_`
pub fn mcp_tool_to_spectre_op(tool: &str) -> String {
    let without_prefix = tool.strip_prefix("cyberchef_").unwrap_or(tool);
    without_prefix
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => {
                    let upper: String = c.to_uppercase().collect();
                    format!("{}{}", upper, chars.as_str())
                },
            }
        })
        .collect::<Vec<_>>()
        .join("_")
}

// ---------------------------------------------------------------------------
// JSON-RPC message construction/parsing helpers
// ---------------------------------------------------------------------------

/// Build a JSON-RPC 2.0 request (with id).
pub fn build_request(id: u64, method: &str, params: Option<serde_json::Value>) -> JsonRpcRequest {
    JsonRpcRequest {
        jsonrpc: "2.0",
        id: Some(id),
        method: method.to_string(),
        params,
    }
}

/// Build a JSON-RPC 2.0 notification (no id).
pub fn build_notification(method: &str, params: Option<serde_json::Value>) -> JsonRpcRequest {
    JsonRpcRequest {
        jsonrpc: "2.0",
        id: None,
        method: method.to_string(),
        params,
    }
}

/// Serialize a request to a newline-delimited JSON line.
pub fn serialize_message(request: &JsonRpcRequest) -> crate::Result<String> {
    let json = serde_json::to_string(request)?;
    Ok(format!("{json}\n"))
}

/// Deserialize a JSON line into a response.
pub fn deserialize_response(line: &str) -> crate::Result<JsonRpcResponse> {
    serde_json::from_str(line.trim()).map_err(|e| {
        crate::SpectreError::Chef(ChefError::ConnectionError(format!(
            "Failed to parse JSON-RPC response: {e}"
        )))
    })
}

/// Build the MCP initialize request params.
#[allow(clippy::disallowed_methods)] // json! macro internally uses unwrap on infallible ops
pub fn build_initialize_params(client_name: &str, client_version: &str) -> serde_json::Value {
    serde_json::json!({
        "protocolVersion": "2024-11-05",
        "capabilities": {},
        "clientInfo": {
            "name": client_name,
            "version": client_version
        }
    })
}

// ---------------------------------------------------------------------------
// McpTransport — manages the stdio pipe to the Docker container
// ---------------------------------------------------------------------------

/// Pending request waiting for a response.
struct PendingRequest {
    sender: oneshot::Sender<JsonRpcResponse>,
}

/// Transport layer managing JSON-RPC communication over Docker subprocess stdio.
///
/// Spawns `docker run -i --rm <image>` with piped stdin/stdout and manages
/// request/response matching via numeric IDs and oneshot channels.
pub struct McpTransport {
    /// Handle to write JSON lines to the subprocess stdin.
    stdin: Arc<Mutex<tokio::process::ChildStdin>>,
    /// Monotonically increasing request ID counter.
    next_id: AtomicU64,
    /// Pending requests awaiting responses, keyed by request ID.
    pending: Arc<Mutex<HashMap<u64, PendingRequest>>>,
    /// The child process handle (kept alive for the transport lifetime).
    _child: Arc<Mutex<Child>>,
    /// Background reader task handle.
    _reader_task: tokio::task::JoinHandle<()>,
}

impl McpTransport {
    /// Spawn a Docker container and set up the JSON-RPC transport.
    ///
    /// This launches `docker run -i --rm <image>` as a child process, takes
    /// ownership of its stdin/stdout, and spawns a background task to read
    /// response lines and dispatch them to waiting callers.
    ///
    /// Environment variables passed to the container control v1.9.0 features:
    /// - `ENABLE_WORKERS`: enables worker thread pool for CPU-intensive ops
    /// - `CYBERCHEF_WORKER_MAX_THREADS`: max worker threads
    /// - `ENABLE_STREAMING`: enables streaming progress for large operations
    pub async fn spawn(docker_image: &str, env_vars: &[(&str, &str)]) -> crate::Result<Self> {
        info!(image = %docker_image, "Spawning CyberChef-MCP container via docker run");

        let mut cmd = Command::new("docker");
        cmd.arg("run").arg("-i").arg("--rm");

        // Pass environment variables to the container
        for (key, value) in env_vars {
            cmd.arg("-e").arg(format!("{key}={value}"));
        }

        let mut child = cmd
            .arg(docker_image)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .kill_on_drop(true)
            .spawn()
            .map_err(|e| {
                crate::SpectreError::Chef(ChefError::DockerError(format!(
                    "Failed to spawn docker process: {e}"
                )))
            })?;

        let stdin = child.stdin.take().ok_or_else(|| {
            crate::SpectreError::Chef(ChefError::DockerError(
                "Failed to capture subprocess stdin".to_string(),
            ))
        })?;

        let stdout = child.stdout.take().ok_or_else(|| {
            crate::SpectreError::Chef(ChefError::DockerError(
                "Failed to capture subprocess stdout".to_string(),
            ))
        })?;

        let pending: Arc<Mutex<HashMap<u64, PendingRequest>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let pending_clone = Arc::clone(&pending);

        // Spawn a background task that reads stdout lines and dispatches responses
        let reader_task = tokio::spawn(async move {
            let mut reader = BufReader::new(stdout);
            let mut line = String::new();

            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) => {
                        // EOF — subprocess exited
                        debug!("MCP transport: subprocess stdout EOF");
                        break;
                    },
                    Ok(_) => {
                        let trimmed = line.trim();
                        if trimmed.is_empty() {
                            continue;
                        }

                        match serde_json::from_str::<JsonRpcResponse>(trimmed) {
                            Ok(response) => {
                                if let Some(id) = response.id {
                                    let mut map = pending_clone.lock().await;
                                    if let Some(req) = map.remove(&id) {
                                        let _ = req.sender.send(response);
                                    } else {
                                        warn!(id = id, "Received response for unknown request ID");
                                    }
                                } else {
                                    // Server-initiated notification (no id) — log and ignore
                                    debug!(line = %trimmed, "MCP server notification");
                                }
                            },
                            Err(e) => {
                                warn!(
                                    error = %e,
                                    line = %trimmed,
                                    "Failed to parse line from MCP server"
                                );
                            },
                        }
                    },
                    Err(e) => {
                        warn!(error = %e, "Error reading from MCP subprocess stdout");
                        break;
                    },
                }
            }
        });

        Ok(Self {
            stdin: Arc::new(Mutex::new(stdin)),
            next_id: AtomicU64::new(1),
            pending,
            _child: Arc::new(Mutex::new(child)),
            _reader_task: reader_task,
        })
    }

    /// Send a JSON-RPC request and wait for the matching response.
    pub async fn send_request(
        &self,
        method: &str,
        params: Option<serde_json::Value>,
        timeout: Duration,
    ) -> crate::Result<serde_json::Value> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let request = build_request(id, method, params);
        let message = serialize_message(&request)?;

        // Register the pending request before sending
        let (tx, rx) = oneshot::channel();
        {
            let mut map = self.pending.lock().await;
            map.insert(id, PendingRequest { sender: tx });
        }

        // Write to stdin
        {
            let mut stdin = self.stdin.lock().await;
            stdin.write_all(message.as_bytes()).await.map_err(|e| {
                crate::SpectreError::Chef(ChefError::ConnectionError(format!(
                    "Failed to write to MCP subprocess stdin: {e}"
                )))
            })?;
            stdin.flush().await.map_err(|e| {
                crate::SpectreError::Chef(ChefError::ConnectionError(format!(
                    "Failed to flush MCP subprocess stdin: {e}"
                )))
            })?;
        }

        // Wait for response with timeout
        let response = tokio::time::timeout(timeout, rx).await.map_err(|_| {
            crate::SpectreError::Chef(ChefError::ConnectionError(format!(
                "Timeout waiting for response to {method} (request id={id})"
            )))
        })?;

        let response = response.map_err(|_| {
            crate::SpectreError::Chef(ChefError::ConnectionError(
                "Response channel closed — MCP subprocess may have exited".to_string(),
            ))
        })?;

        // Check for JSON-RPC error
        if let Some(err) = response.error {
            return Err(crate::SpectreError::Chef(ChefError::OperationFailed {
                operation: method.to_string(),
                message: err.to_string(),
            }));
        }

        response.result.ok_or_else(|| {
            crate::SpectreError::Chef(ChefError::ConnectionError(
                "JSON-RPC response contained neither result nor error".to_string(),
            ))
        })
    }

    /// Send a JSON-RPC notification (fire-and-forget, no response expected).
    pub async fn send_notification(
        &self,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> crate::Result<()> {
        let notification = build_notification(method, params);
        let message = serialize_message(&notification)?;

        let mut stdin = self.stdin.lock().await;
        stdin.write_all(message.as_bytes()).await.map_err(|e| {
            crate::SpectreError::Chef(ChefError::ConnectionError(format!(
                "Failed to write notification to MCP subprocess stdin: {e}"
            )))
        })?;
        stdin.flush().await.map_err(|e| {
            crate::SpectreError::Chef(ChefError::ConnectionError(format!(
                "Failed to flush MCP subprocess stdin: {e}"
            )))
        })?;

        Ok(())
    }

    /// Check if the subprocess is still alive.
    pub async fn is_alive(&self) -> bool {
        let mut child = self._child.lock().await;
        match child.try_wait() {
            Ok(None) => true,     // Still running
            Ok(Some(_)) => false, // Exited
            Err(_) => false,
        }
    }
}

// ---------------------------------------------------------------------------
// McpChefClient — real MCP client implementing ChefClient
// ---------------------------------------------------------------------------

/// Real MCP client that communicates with CyberChef-MCP via JSON-RPC over Docker stdio.
///
/// This client spawns a Docker container running CyberChef-MCP, performs the MCP
/// initialization handshake, and then proxies all `ChefClient` operations as MCP
/// tool calls.
pub struct McpChefClient {
    transport: McpTransport,
    timeout: Duration,
    #[allow(dead_code)]
    server_name: String,
    #[allow(dead_code)]
    server_version: String,
    protocol_version: String,
}

#[allow(clippy::disallowed_methods)] // json! macro internally uses unwrap on infallible ops
impl McpChefClient {
    /// Connect to a CyberChef-MCP Docker container.
    ///
    /// This spawns `docker run -i --rm <docker_image>` and performs the MCP
    /// initialization handshake (initialize request + initialized notification).
    ///
    /// The `config` parameter controls v1.9.0 features passed as environment
    /// variables to the container: worker thread pool, streaming, and transport.
    #[instrument(skip_all, fields(image = %docker_image))]
    pub async fn connect(
        docker_image: &str,
        timeout_secs: u64,
        config: &crate::config::ChefConfig,
    ) -> crate::Result<Self> {
        let timeout = Duration::from_secs(timeout_secs);

        info!("Connecting to CyberChef-MCP via Docker stdio");

        // Build environment variables for v1.9.0 features
        let enable_workers = config.enable_workers.to_string();
        let worker_threads = config.worker_threads.to_string();
        let enable_streaming = config.enable_streaming.to_string();
        let env_vars: Vec<(&str, &str)> = vec![
            ("ENABLE_WORKERS", &enable_workers),
            ("CYBERCHEF_WORKER_MAX_THREADS", &worker_threads),
            ("ENABLE_STREAMING", &enable_streaming),
        ];

        // Spawn the Docker subprocess
        let transport = McpTransport::spawn(docker_image, &env_vars).await?;

        // Perform MCP initialization handshake
        let init_params = build_initialize_params("spectre", env!("CARGO_PKG_VERSION"));
        let init_result = transport
            .send_request("initialize", Some(init_params), timeout)
            .await?;

        let init: McpInitializeResult = serde_json::from_value(init_result).map_err(|e| {
            crate::SpectreError::Chef(ChefError::ConnectionError(format!(
                "Failed to parse MCP initialize response: {e}"
            )))
        })?;

        info!(
            server = %init.server_info.name,
            version = %init.server_info.version,
            protocol = %init.protocol_version,
            "MCP handshake complete"
        );

        // Send the initialized notification
        transport
            .send_notification("notifications/initialized", Some(serde_json::json!({})))
            .await?;

        Ok(Self {
            transport,
            timeout,
            server_name: init.server_info.name,
            server_version: init.server_info.version,
            protocol_version: init.protocol_version,
        })
    }
}

#[async_trait]
#[allow(clippy::disallowed_methods)] // json! macro internally uses unwrap on infallible ops
impl ChefClient for McpChefClient {
    #[instrument(skip(self, input, args), fields(operation = %operation, input_len = input.len()))]
    async fn execute(
        &self,
        operation: &str,
        input: &[u8],
        args: &HashMap<String, String>,
    ) -> crate::Result<Vec<u8>> {
        info!("Executing CyberChef operation via MCP");

        let tool_name = spectre_op_to_mcp_tool(operation);
        let input_str = String::from_utf8_lossy(input);

        // Build the arguments object for the tool call
        let mut tool_args = serde_json::Map::new();
        tool_args.insert(
            "input".to_string(),
            serde_json::Value::String(input_str.into_owned()),
        );
        for (k, v) in args {
            tool_args.insert(k.clone(), serde_json::Value::String(v.clone()));
        }

        let params = serde_json::json!({
            "name": tool_name,
            "arguments": tool_args
        });

        let result = self
            .transport
            .send_request("tools/call", Some(params), self.timeout)
            .await?;

        let call_result: McpToolCallResult = serde_json::from_value(result).map_err(|e| {
            crate::SpectreError::Chef(ChefError::OperationFailed {
                operation: operation.to_string(),
                message: format!("Failed to parse tool call result: {e}"),
            })
        })?;

        if call_result.is_error {
            let error_text = call_result
                .content
                .iter()
                .filter_map(|c| c.text.as_deref())
                .collect::<Vec<_>>()
                .join("\n");
            return Err(crate::SpectreError::Chef(ChefError::OperationFailed {
                operation: operation.to_string(),
                message: error_text,
            }));
        }

        // Extract text content from the response
        let output = call_result
            .content
            .iter()
            .filter(|c| c.content_type == "text")
            .filter_map(|c| c.text.as_deref())
            .collect::<Vec<_>>()
            .join("");

        info!(output_len = output.len(), "MCP operation complete");
        Ok(output.into_bytes())
    }

    #[instrument(skip(self, input), fields(recipe = %recipe.name, operations = recipe.operations.len()))]
    async fn execute_recipe(&self, recipe: &Recipe, input: &[u8]) -> crate::Result<Vec<u8>> {
        info!("Executing CyberChef recipe via MCP");

        let input_str = String::from_utf8_lossy(input);

        // Build the recipe array for the cyberchef_bake tool
        let recipe_ops: Vec<serde_json::Value> = recipe
            .operations
            .iter()
            .map(|op| {
                let mut op_obj = serde_json::Map::new();
                op_obj.insert("op".to_string(), serde_json::Value::String(op.name.clone()));
                let args_obj: serde_json::Map<String, serde_json::Value> = op
                    .args
                    .iter()
                    .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                    .collect();
                op_obj.insert("args".to_string(), serde_json::Value::Object(args_obj));
                serde_json::Value::Object(op_obj)
            })
            .collect();

        let params = serde_json::json!({
            "name": "cyberchef_bake",
            "arguments": {
                "input": input_str,
                "recipe": recipe_ops
            }
        });

        let result = self
            .transport
            .send_request("tools/call", Some(params), self.timeout)
            .await?;

        let call_result: McpToolCallResult = serde_json::from_value(result).map_err(|e| {
            crate::SpectreError::Chef(ChefError::OperationFailed {
                operation: format!("recipe:{}", recipe.name),
                message: format!("Failed to parse bake result: {e}"),
            })
        })?;

        if call_result.is_error {
            let error_text = call_result
                .content
                .iter()
                .filter_map(|c| c.text.as_deref())
                .collect::<Vec<_>>()
                .join("\n");
            return Err(crate::SpectreError::Chef(ChefError::OperationFailed {
                operation: format!("recipe:{}", recipe.name),
                message: error_text,
            }));
        }

        let output = call_result
            .content
            .iter()
            .filter(|c| c.content_type == "text")
            .filter_map(|c| c.text.as_deref())
            .collect::<Vec<_>>()
            .join("");

        info!(output_len = output.len(), "MCP recipe complete");
        Ok(output.into_bytes())
    }

    async fn health_check(&self) -> crate::Result<HealthStatus> {
        // Check if the transport subprocess is alive
        let alive = self.transport.is_alive().await;
        if !alive {
            return Ok(HealthStatus {
                healthy: false,
                container_status: "exited".to_string(),
                mcp_version: self.protocol_version.clone(),
                operation_count: 0,
            });
        }

        // Try to list tools to verify the connection is working
        let ops = self.list_operations(None, None).await;
        let (healthy, op_count) = match ops {
            Ok(ops) => (true, ops.len()),
            Err(_) => (false, 0),
        };

        Ok(HealthStatus {
            healthy,
            container_status: "running".to_string(),
            mcp_version: self.protocol_version.clone(),
            operation_count: op_count,
        })
    }

    async fn list_operations(
        &self,
        category: Option<&str>,
        search: Option<&str>,
    ) -> crate::Result<Vec<OperationInfo>> {
        let result = self
            .transport
            .send_request("tools/list", Some(serde_json::json!({})), self.timeout)
            .await?;

        let tools_result: McpToolsListResult = serde_json::from_value(result).map_err(|e| {
            crate::SpectreError::Chef(ChefError::ConnectionError(format!(
                "Failed to parse tools/list response: {e}"
            )))
        })?;

        let mut operations: Vec<OperationInfo> = tools_result
            .tools
            .into_iter()
            .filter(|t| t.name.starts_with("cyberchef_"))
            .map(|t| {
                let spectre_name = mcp_tool_to_spectre_op(&t.name);
                // Attempt to derive a category from the tool name or description
                let cat = derive_category(&spectre_name, t.description.as_deref());
                OperationInfo {
                    name: spectre_name,
                    category: cat,
                    description: t.description.unwrap_or_default(),
                }
            })
            .collect();

        // Apply filters
        if let Some(cat) = category {
            let cat_lower = cat.to_lowercase();
            operations.retain(|op| op.category.to_lowercase().contains(&cat_lower));
        }

        if let Some(s) = search {
            let search_lower = s.to_lowercase();
            operations.retain(|op| {
                op.name.to_lowercase().contains(&search_lower)
                    || op.description.to_lowercase().contains(&search_lower)
            });
        }

        Ok(operations)
    }

    async fn operation_help(&self, operation: &str) -> crate::Result<OperationHelp> {
        // Use the cyberchef_search tool to get operation details
        let tool_name = "cyberchef_search";
        let params = serde_json::json!({
            "name": tool_name,
            "arguments": {
                "query": operation
            }
        });

        let result = self
            .transport
            .send_request("tools/call", Some(params), self.timeout)
            .await;

        match result {
            Ok(value) => {
                // Parse the search result
                let call_result: McpToolCallResult =
                    serde_json::from_value(value).map_err(|e| {
                        crate::SpectreError::Chef(ChefError::OperationFailed {
                            operation: operation.to_string(),
                            message: format!("Failed to parse search result: {e}"),
                        })
                    })?;

                let text = call_result
                    .content
                    .iter()
                    .filter(|c| c.content_type == "text")
                    .filter_map(|c| c.text.as_deref())
                    .collect::<Vec<_>>()
                    .join("\n");

                Ok(OperationHelp {
                    name: operation.to_string(),
                    category: derive_category(operation, None),
                    description: text,
                    args: vec![OperationArg {
                        name: "input".to_string(),
                        arg_type: "string".to_string(),
                        description: "Input data to process".to_string(),
                        default: None,
                        required: true,
                    }],
                    examples: vec![format!("spectre chef {operation} --input \"data\"")],
                })
            },
            Err(_) => {
                // Fallback: return basic help if search tool is not available
                Ok(OperationHelp {
                    name: operation.to_string(),
                    category: derive_category(operation, None),
                    description: format!("CyberChef operation: {operation}"),
                    args: vec![OperationArg {
                        name: "input".to_string(),
                        arg_type: "string".to_string(),
                        description: "Input data to process".to_string(),
                        default: None,
                        required: true,
                    }],
                    examples: vec![format!("spectre chef {operation} --input \"data\"")],
                })
            },
        }
    }

    async fn worker_stats(&self) -> crate::Result<super::WorkerStats> {
        let params = serde_json::json!({
            "name": "cyberchef_worker_stats",
            "arguments": {}
        });

        let result = self
            .transport
            .send_request("tools/call", Some(params), self.timeout)
            .await?;

        let call_result: McpToolCallResult = serde_json::from_value(result).map_err(|e| {
            crate::SpectreError::Chef(ChefError::OperationFailed {
                operation: "worker_stats".to_string(),
                message: format!("Failed to parse worker stats result: {e}"),
            })
        })?;

        let text = call_result
            .content
            .iter()
            .filter(|c| c.content_type == "text")
            .filter_map(|c| c.text.as_deref())
            .collect::<Vec<_>>()
            .join("");

        let stats: serde_json::Value = serde_json::from_str(&text).map_err(|e| {
            crate::SpectreError::Chef(ChefError::OperationFailed {
                operation: "worker_stats".to_string(),
                message: format!("Failed to parse worker stats JSON: {e}"),
            })
        })?;

        let enabled = stats
            .get("enabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        Ok(super::WorkerStats {
            enabled,
            threads: stats
                .get("threads")
                .and_then(|v| v.as_u64())
                .map(|v| v as usize),
            completed: stats.get("completed").and_then(|v| v.as_u64()),
            waiting: stats
                .get("waiting")
                .and_then(|v| v.as_u64())
                .map(|v| v as usize),
            utilization: stats.get("utilization").and_then(|v| v.as_f64()),
            message: stats
                .get("message")
                .and_then(|v| v.as_str())
                .map(String::from),
        })
    }
}

/// Derive a category for an operation based on its name or description.
///
/// This heuristic categorizes operations by looking at common patterns in
/// CyberChef operation names.
fn derive_category(name: &str, description: Option<&str>) -> String {
    let lower = name.to_lowercase();
    let desc_lower = description.map(|d| d.to_lowercase()).unwrap_or_default();

    if lower.contains("base64")
        || lower.contains("hex")
        || lower.contains("binary")
        || lower.contains("decimal")
        || lower.contains("octal")
        || desc_lower.contains("encode")
        || desc_lower.contains("decode")
    {
        return "Data Format".to_string();
    }

    if lower.contains("aes")
        || lower.contains("des")
        || lower.contains("rsa")
        || lower.contains("encrypt")
        || lower.contains("decrypt")
        || lower.contains("cipher")
        || lower.contains("blowfish")
        || lower.contains("rc4")
    {
        return "Encryption".to_string();
    }

    if lower.contains("md5")
        || lower.contains("sha")
        || lower.contains("hash")
        || lower.contains("hmac")
        || lower.contains("crc")
        || lower.contains("checksum")
    {
        return "Hashing".to_string();
    }

    if lower.contains("url")
        || lower.contains("http")
        || lower.contains("uri")
        || lower.contains("punycode")
    {
        return "URL".to_string();
    }

    // Compression must be checked before Networking because "gzip" contains "ip"
    if lower.contains("compress")
        || lower.contains("decompress")
        || lower.contains("gzip")
        || lower.contains("zlib")
        || lower.contains("bzip")
        || lower.contains("zip")
    {
        return "Compression".to_string();
    }

    if lower.contains("ip")
        || lower.contains("mac")
        || lower.contains("dns")
        || lower.contains("whois")
    {
        return "Networking".to_string();
    }

    if lower.contains("regex")
        || lower.contains("find")
        || lower.contains("replace")
        || lower.contains("split")
        || lower.contains("filter")
    {
        return "Search".to_string();
    }

    "Other".to_string()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Operation name conversion tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_spectre_op_to_mcp_tool_base64() {
        assert_eq!(
            spectre_op_to_mcp_tool("From_Base64"),
            "cyberchef_from_base64"
        );
    }

    #[test]
    fn test_spectre_op_to_mcp_tool_aes() {
        assert_eq!(
            spectre_op_to_mcp_tool("AES_Encrypt"),
            "cyberchef_aes_encrypt"
        );
    }

    #[test]
    fn test_spectre_op_to_mcp_tool_url_decode() {
        assert_eq!(spectre_op_to_mcp_tool("URL_Decode"), "cyberchef_url_decode");
    }

    #[test]
    fn test_spectre_op_to_mcp_tool_single_word() {
        assert_eq!(spectre_op_to_mcp_tool("MD5"), "cyberchef_md5");
    }

    #[test]
    fn test_spectre_op_to_mcp_tool_sha256() {
        assert_eq!(spectre_op_to_mcp_tool("SHA256"), "cyberchef_sha256");
    }

    #[test]
    fn test_spectre_op_to_mcp_tool_to_hex() {
        assert_eq!(spectre_op_to_mcp_tool("To_Hex"), "cyberchef_to_hex");
    }

    #[test]
    fn test_spectre_op_to_mcp_tool_three_words() {
        assert_eq!(
            spectre_op_to_mcp_tool("To_Hex_Dump"),
            "cyberchef_to_hex_dump"
        );
    }

    #[test]
    fn test_mcp_tool_to_spectre_op_base64() {
        assert_eq!(
            mcp_tool_to_spectre_op("cyberchef_from_base64"),
            "From_Base64"
        );
    }

    #[test]
    fn test_mcp_tool_to_spectre_op_aes() {
        assert_eq!(
            mcp_tool_to_spectre_op("cyberchef_aes_encrypt"),
            "Aes_Encrypt"
        );
    }

    #[test]
    fn test_mcp_tool_to_spectre_op_md5() {
        assert_eq!(mcp_tool_to_spectre_op("cyberchef_md5"), "Md5");
    }

    #[test]
    fn test_mcp_tool_to_spectre_op_no_prefix() {
        assert_eq!(mcp_tool_to_spectre_op("some_tool"), "Some_Tool");
    }

    #[test]
    fn test_mcp_tool_to_spectre_op_url_decode() {
        assert_eq!(mcp_tool_to_spectre_op("cyberchef_url_decode"), "Url_Decode");
    }

    #[test]
    fn test_roundtrip_simple() {
        let original = "To_Base64";
        let mcp = spectre_op_to_mcp_tool(original);
        let back = mcp_tool_to_spectre_op(&mcp);
        // Note: roundtrip is not perfect for mixed case like "AES_Encrypt" -> "Aes_Encrypt"
        // but single-case words roundtrip correctly
        assert_eq!(back, "To_Base64");
    }

    // -----------------------------------------------------------------------
    // JSON-RPC message construction tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_build_request_with_params() {
        let req = build_request(1, "initialize", Some(serde_json::json!({"key": "value"})));
        assert_eq!(req.jsonrpc, "2.0");
        assert_eq!(req.id, Some(1));
        assert_eq!(req.method, "initialize");
        assert!(req.params.is_some());
    }

    #[test]
    fn test_build_request_without_params() {
        let req = build_request(42, "tools/list", None);
        assert_eq!(req.id, Some(42));
        assert_eq!(req.method, "tools/list");
        assert!(req.params.is_none());
    }

    #[test]
    fn test_build_notification() {
        let notif = build_notification("notifications/initialized", Some(serde_json::json!({})));
        assert_eq!(notif.jsonrpc, "2.0");
        assert!(notif.id.is_none());
        assert_eq!(notif.method, "notifications/initialized");
    }

    #[test]
    fn test_serialize_message_ends_with_newline() {
        let req = build_request(1, "test", None);
        let msg = serialize_message(&req).unwrap();
        assert!(msg.ends_with('\n'));
    }

    #[test]
    fn test_serialize_message_valid_json() {
        let req = build_request(1, "test", Some(serde_json::json!({"a": 1})));
        let msg = serialize_message(&req).unwrap();
        let trimmed = msg.trim();
        let parsed: serde_json::Value = serde_json::from_str(trimmed).unwrap();
        assert_eq!(parsed["jsonrpc"], "2.0");
        assert_eq!(parsed["id"], 1);
        assert_eq!(parsed["method"], "test");
    }

    #[test]
    fn test_serialize_notification_no_id() {
        let notif = build_notification("test", None);
        let msg = serialize_message(&notif).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(msg.trim()).unwrap();
        assert!(parsed.get("id").is_none());
    }

    // -----------------------------------------------------------------------
    // JSON-RPC response parsing tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_deserialize_success_response() {
        let json = r#"{"jsonrpc":"2.0","id":1,"result":{"tools":[]}}"#;
        let resp = deserialize_response(json).unwrap();
        assert_eq!(resp.id, Some(1));
        assert!(resp.result.is_some());
        assert!(resp.error.is_none());
    }

    #[test]
    fn test_deserialize_error_response() {
        let json =
            r#"{"jsonrpc":"2.0","id":2,"error":{"code":-32601,"message":"Method not found"}}"#;
        let resp = deserialize_response(json).unwrap();
        assert_eq!(resp.id, Some(2));
        assert!(resp.result.is_none());
        let err = resp.error.unwrap();
        assert_eq!(err.code, -32601);
        assert_eq!(err.message, "Method not found");
    }

    #[test]
    fn test_deserialize_response_with_whitespace() {
        let json = "  {\"jsonrpc\":\"2.0\",\"id\":1,\"result\":null}  \n";
        let resp = deserialize_response(json).unwrap();
        assert_eq!(resp.id, Some(1));
    }

    #[test]
    fn test_deserialize_response_invalid_json() {
        let json = "not valid json";
        let result = deserialize_response(json);
        assert!(result.is_err());
    }

    // -----------------------------------------------------------------------
    // MCP initialize params test
    // -----------------------------------------------------------------------

    #[test]
    fn test_build_initialize_params() {
        let params = build_initialize_params("spectre", "0.4.5");
        assert_eq!(params["protocolVersion"], "2024-11-05");
        assert_eq!(params["clientInfo"]["name"], "spectre");
        assert_eq!(params["clientInfo"]["version"], "0.4.5");
        assert!(params["capabilities"].is_object());
    }

    // -----------------------------------------------------------------------
    // MCP tool call request format test
    // -----------------------------------------------------------------------

    #[test]
    fn test_tool_call_request_format() {
        let id = 3;
        let tool_name = spectre_op_to_mcp_tool("To_Base64");
        let params = serde_json::json!({
            "name": tool_name,
            "arguments": {
                "input": "Hello"
            }
        });
        let req = build_request(id, "tools/call", Some(params));
        let msg = serialize_message(&req).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(msg.trim()).unwrap();

        assert_eq!(parsed["method"], "tools/call");
        assert_eq!(parsed["params"]["name"], "cyberchef_to_base64");
        assert_eq!(parsed["params"]["arguments"]["input"], "Hello");
    }

    // -----------------------------------------------------------------------
    // MCP tool call response parsing test
    // -----------------------------------------------------------------------

    #[test]
    fn test_parse_tool_call_result() {
        let json = serde_json::json!({
            "content": [
                {"type": "text", "text": "SGVsbG8="}
            ]
        });
        let result: McpToolCallResult = serde_json::from_value(json).unwrap();
        assert_eq!(result.content.len(), 1);
        assert_eq!(result.content[0].content_type, "text");
        assert_eq!(result.content[0].text.as_deref(), Some("SGVsbG8="));
        assert!(!result.is_error);
    }

    #[test]
    fn test_parse_tool_call_error_result() {
        let json = serde_json::json!({
            "content": [
                {"type": "text", "text": "Invalid input"}
            ],
            "isError": true
        });
        let result: McpToolCallResult = serde_json::from_value(json).unwrap();
        assert!(result.is_error);
        assert_eq!(result.content[0].text.as_deref(), Some("Invalid input"));
    }

    #[test]
    fn test_parse_tool_call_multiple_content() {
        let json = serde_json::json!({
            "content": [
                {"type": "text", "text": "part1"},
                {"type": "text", "text": "part2"}
            ]
        });
        let result: McpToolCallResult = serde_json::from_value(json).unwrap();
        assert_eq!(result.content.len(), 2);
    }

    // -----------------------------------------------------------------------
    // MCP tools/list response parsing test
    // -----------------------------------------------------------------------

    #[test]
    fn test_parse_tools_list_result() {
        let json = serde_json::json!({
            "tools": [
                {
                    "name": "cyberchef_to_base64",
                    "description": "Encode data as Base64",
                    "inputSchema": {"type": "object"}
                },
                {
                    "name": "cyberchef_from_base64",
                    "description": "Decode Base64 data"
                }
            ]
        });
        let result: McpToolsListResult = serde_json::from_value(json).unwrap();
        assert_eq!(result.tools.len(), 2);
        assert_eq!(result.tools[0].name, "cyberchef_to_base64");
        assert_eq!(
            result.tools[0].description.as_deref(),
            Some("Encode data as Base64")
        );
    }

    // -----------------------------------------------------------------------
    // MCP initialize response parsing test
    // -----------------------------------------------------------------------

    #[test]
    fn test_parse_initialize_result() {
        let json = serde_json::json!({
            "protocolVersion": "2024-11-05",
            "capabilities": {"tools": {}},
            "serverInfo": {
                "name": "cyberchef-mcp",
                "version": "1.8.0"
            }
        });
        let result: McpInitializeResult = serde_json::from_value(json).unwrap();
        assert_eq!(result.protocol_version, "2024-11-05");
        assert_eq!(result.server_info.name, "cyberchef-mcp");
        assert_eq!(result.server_info.version, "1.8.0");
    }

    // -----------------------------------------------------------------------
    // Error handling tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_json_rpc_error_display() {
        let err = JsonRpcError {
            code: -32601,
            message: "Method not found".to_string(),
            data: None,
        };
        assert_eq!(err.to_string(), "JSON-RPC error -32601: Method not found");
    }

    #[test]
    fn test_json_rpc_error_with_data() {
        let err = JsonRpcError {
            code: -32000,
            message: "Server error".to_string(),
            data: Some(serde_json::json!({"detail": "something"})),
        };
        assert_eq!(err.to_string(), "JSON-RPC error -32000: Server error");
        assert!(err.data.is_some());
    }

    // -----------------------------------------------------------------------
    // Category derivation tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_derive_category_base64() {
        assert_eq!(derive_category("From_Base64", None), "Data Format");
    }

    #[test]
    fn test_derive_category_hex() {
        assert_eq!(derive_category("To_Hex", None), "Data Format");
    }

    #[test]
    fn test_derive_category_aes() {
        assert_eq!(derive_category("AES_Encrypt", None), "Encryption");
    }

    #[test]
    fn test_derive_category_md5() {
        assert_eq!(derive_category("MD5", None), "Hashing");
    }

    #[test]
    fn test_derive_category_sha256() {
        assert_eq!(derive_category("SHA256", None), "Hashing");
    }

    #[test]
    fn test_derive_category_url() {
        assert_eq!(derive_category("URL_Decode", None), "URL");
    }

    #[test]
    fn test_derive_category_unknown() {
        assert_eq!(derive_category("FooBar", None), "Other");
    }

    #[test]
    fn test_derive_category_from_description() {
        assert_eq!(
            derive_category("Custom_Op", Some("Encode the data")),
            "Data Format"
        );
    }

    #[test]
    fn test_derive_category_gzip() {
        assert_eq!(derive_category("Gzip_Compress", None), "Compression");
    }

    #[test]
    fn test_derive_category_ip_address() {
        assert_eq!(derive_category("Parse_IP_Range", None), "Networking");
    }

    #[test]
    fn test_derive_category_regex() {
        assert_eq!(derive_category("Regex_Extract", None), "Search");
    }

    // -----------------------------------------------------------------------
    // Message framing tests (newline-delimited JSON)
    // -----------------------------------------------------------------------

    #[test]
    fn test_message_framing_single_line() {
        let req = build_request(1, "test", None);
        let msg = serialize_message(&req).unwrap();
        // Must be exactly one line (content + newline)
        let lines: Vec<&str> = msg.lines().collect();
        assert_eq!(lines.len(), 1);
        assert!(!lines[0].is_empty());
    }

    #[test]
    fn test_message_framing_no_embedded_newlines() {
        let req = build_request(
            1,
            "tools/call",
            Some(serde_json::json!({
                "name": "cyberchef_to_base64",
                "arguments": {"input": "line1\nline2"}
            })),
        );
        let msg = serialize_message(&req).unwrap();
        // The JSON encoding of \n should be \\n in the serialized output,
        // so there should be exactly one trailing newline
        let count = msg.chars().filter(|c| *c == '\n').count();
        assert_eq!(count, 1, "Message should have exactly one newline at end");
    }

    // -----------------------------------------------------------------------
    // Full handshake message sequence test
    // -----------------------------------------------------------------------

    #[test]
    fn test_full_initialization_sequence() {
        // Step 1: Client sends initialize request
        let init_params = build_initialize_params("spectre", "0.4.5");
        let init_req = build_request(1, "initialize", Some(init_params));
        let init_msg = serialize_message(&init_req).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(init_msg.trim()).unwrap();
        assert_eq!(parsed["method"], "initialize");
        assert_eq!(parsed["id"], 1);
        assert_eq!(parsed["params"]["protocolVersion"], "2024-11-05");

        // Step 2: Server responds (simulated)
        let server_response = r#"{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2024-11-05","capabilities":{"tools":{}},"serverInfo":{"name":"cyberchef-mcp","version":"1.8.0"}}}"#;
        let resp = deserialize_response(server_response).unwrap();
        assert_eq!(resp.id, Some(1));
        let init_result: McpInitializeResult =
            serde_json::from_value(resp.result.unwrap()).unwrap();
        assert_eq!(init_result.server_info.name, "cyberchef-mcp");

        // Step 3: Client sends initialized notification
        let notif = build_notification("notifications/initialized", Some(serde_json::json!({})));
        let notif_msg = serialize_message(&notif).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(notif_msg.trim()).unwrap();
        assert_eq!(parsed["method"], "notifications/initialized");
        assert!(parsed.get("id").is_none());
    }

    // -----------------------------------------------------------------------
    // v1.9.0 worker stats tool call tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_worker_stats_tool_call_format() {
        let params = serde_json::json!({
            "name": "cyberchef_worker_stats",
            "arguments": {}
        });
        let req = build_request(5, "tools/call", Some(params));
        let msg = serialize_message(&req).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(msg.trim()).unwrap();

        assert_eq!(parsed["method"], "tools/call");
        assert_eq!(parsed["params"]["name"], "cyberchef_worker_stats");
    }

    #[test]
    fn test_parse_worker_stats_enabled_response() {
        let json = serde_json::json!({
            "content": [{
                "type": "text",
                "text": "{\"enabled\":true,\"threads\":4,\"completed\":100,\"waiting\":2,\"utilization\":0.75}"
            }]
        });
        let result: McpToolCallResult = serde_json::from_value(json).unwrap();
        assert!(!result.is_error);

        let text = result.content[0].text.as_deref().unwrap();
        let stats: serde_json::Value = serde_json::from_str(text).unwrap();
        assert_eq!(stats["enabled"], true);
        assert_eq!(stats["threads"], 4);
        assert_eq!(stats["completed"], 100);
    }

    #[test]
    fn test_parse_worker_stats_disabled_response() {
        let json = serde_json::json!({
            "content": [{
                "type": "text",
                "text": "{\"enabled\":false,\"message\":\"Worker pool is not enabled. Set ENABLE_WORKERS=true to enable.\"}"
            }]
        });
        let result: McpToolCallResult = serde_json::from_value(json).unwrap();
        let text = result.content[0].text.as_deref().unwrap();
        let stats: serde_json::Value = serde_json::from_str(text).unwrap();
        assert_eq!(stats["enabled"], false);
        assert!(stats["message"]
            .as_str()
            .unwrap()
            .contains("ENABLE_WORKERS"));
    }

    #[test]
    fn test_handshake_with_v190_server() {
        // v1.9.0 server responds with version 1.9.0
        let server_response = r#"{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2024-11-05","capabilities":{"tools":{}},"serverInfo":{"name":"cyberchef-mcp","version":"1.9.0"}}}"#;
        let resp = deserialize_response(server_response).unwrap();
        let init_result: McpInitializeResult =
            serde_json::from_value(resp.result.unwrap()).unwrap();
        assert_eq!(init_result.server_info.version, "1.9.0");
    }
}
