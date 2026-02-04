# SPECTRE Integration Specification

**Version:** 0.1.0 | **Status:** Design Phase | **Last Updated:** 2026-02-04

---

## Overview

This document specifies how SPECTRE integrates with its three component projects: WRAITH-Protocol, ProRT-IP WarScan, and CyberChef-MCP. Each component has distinct integration methods, API contracts, and data exchange patterns.

---

## Component Summary

| Component | Version | Language | Integration Method | Primary API |
|-----------|---------|----------|-------------------|-------------|
| ProRT-IP WarScan | v1.0.0 | Rust | Library + CLI | Rust crate |
| CyberChef-MCP | v1.8.0 | Node.js | MCP Protocol | JSON-RPC/MCP |
| WRAITH-Protocol | v2.3.7 | Rust | Library + FFI | Rust crate |

---

## ProRT-IP Integration

### Integration Architecture

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                          ProRT-IP Integration                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌───────────────────┐                                                       │
│  │   SPECTRE Core    │                                                       │
│  └─────────┬─────────┘                                                       │
│            │                                                                 │
│            ▼                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                    Integration Layer                                 │    │
│  │  ┌─────────────────────────┐  ┌─────────────────────────────────┐   │    │
│  │  │   Library Integration   │  │      CLI Fallback               │   │    │
│  │  │   (prtip-core crate)    │  │   (subprocess + JSON)           │   │    │
│  │  │                         │  │                                  │   │    │
│  │  │   • Direct function     │  │   • prtip -sS ... -oJ -         │   │    │
│  │  │   • Zero-copy data      │  │   • Parsed JSON output          │   │    │
│  │  │   • Async streams       │  │   • Process management          │   │    │
│  │  └─────────────────────────┘  └─────────────────────────────────┘   │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│            │                                                                 │
│            ▼                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                      ProRT-IP Core                                   │    │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌──────────────┐   │    │
│  │  │  Scanner   │  │  Detector  │  │   Output   │  │    Plugin    │   │    │
│  │  │  Engine    │  │  Engine    │  │  Formatter │  │    System    │   │    │
│  │  └────────────┘  └────────────┘  └────────────┘  └──────────────┘   │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Library Integration (Primary)

**Cargo.toml Dependency:**
```toml
[dependencies]
prtip-core = { path = "../ProRT-IP/crates/prtip-core" }
prtip-scanner = { path = "../ProRT-IP/crates/prtip-scanner" }
prtip-detector = { path = "../ProRT-IP/crates/prtip-detector" }
```

**API Usage:**

```rust
use prtip_core::{Target, TargetSpec};
use prtip_scanner::{Scanner, ScanConfig, ScanType, ScanResult};
use prtip_detector::{ServiceDetector, OsDetector};

/// Execute a network scan with SPECTRE configuration
pub async fn execute_scan(
    targets: Vec<TargetSpec>,
    config: SpectreReconConfig,
) -> Result<Vec<ScanResult>, SpectreError> {

    // Build ProRT-IP scan configuration
    let scan_config = ScanConfig::builder()
        .scan_type(map_scan_type(config.scan_type))
        .ports(config.ports.clone())
        .rate_limit(config.rate_limit)
        .timeout(config.timeout)
        .retry_count(config.retries)
        .interface(config.interface.as_deref())
        .source_port(config.source_port)
        .ttl(config.ttl)
        .build()?;

    // Create scanner instance
    let scanner = Scanner::new(scan_config)?;

    // Execute scan with progress callback
    let results = scanner
        .scan_with_progress(targets, |progress| {
            // Forward progress to SPECTRE event bus
            emit_event(ScanProgressEvent {
                completed: progress.completed,
                total: progress.total,
                rate: progress.current_rate,
            });
        })
        .await?;

    // Optionally run service detection
    if config.service_detection {
        let detector = ServiceDetector::new(config.version_intensity);
        for result in &mut results {
            result.services = detector.detect(&result).await?;
        }
    }

    Ok(results)
}

fn map_scan_type(spectre_type: SpectreScanType) -> ScanType {
    match spectre_type {
        SpectreScanType::Syn => ScanType::Syn,
        SpectreScanType::Connect => ScanType::Connect,
        SpectreScanType::Fin => ScanType::Fin,
        SpectreScanType::Null => ScanType::Null,
        SpectreScanType::Xmas => ScanType::Xmas,
        SpectreScanType::Ack => ScanType::Ack,
        SpectreScanType::Idle => ScanType::Idle,
        SpectreScanType::Udp => ScanType::Udp,
    }
}
```

### CLI Fallback Integration

Used when library integration is unavailable (e.g., version mismatch):

```rust
use std::process::{Command, Stdio};
use tokio::process::Command as AsyncCommand;

pub async fn execute_scan_cli(
    targets: &[String],
    config: &SpectreReconConfig,
) -> Result<Vec<ScanResult>, SpectreError> {

    let mut args = vec![];

    // Map scan type to prtip flags
    args.push(match config.scan_type {
        SpectreScanType::Syn => "-sS",
        SpectreScanType::Connect => "-sT",
        SpectreScanType::Fin => "-sF",
        SpectreScanType::Null => "-sN",
        SpectreScanType::Xmas => "-sX",
        SpectreScanType::Ack => "-sA",
        SpectreScanType::Udp => "-sU",
        SpectreScanType::Idle => "-sI",
    }.to_string());

    // Add ports
    args.push("-p".to_string());
    args.push(config.ports.clone());

    // Add service detection
    if config.service_detection {
        args.push("-sV".to_string());
    }

    // JSON output to stdout
    args.push("-oJ".to_string());
    args.push("-".to_string());

    // Add targets
    args.extend(targets.iter().cloned());

    // Execute prtip
    let output = AsyncCommand::new("prtip")
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;

    if !output.status.success() {
        return Err(SpectreError::ComponentError {
            component: "prtip",
            message: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }

    // Parse JSON output
    let results: PrtipJsonOutput = serde_json::from_slice(&output.stdout)?;
    Ok(convert_prtip_results(results))
}
```

### Data Model Mapping

**ProRT-IP → SPECTRE:**

| ProRT-IP Type | SPECTRE Type | Conversion |
|---------------|--------------|------------|
| `prtip::Host` | `spectre::Target` | Direct mapping |
| `prtip::Port` | `spectre::PortInfo` | Add metadata |
| `prtip::Service` | `spectre::ServiceInfo` | Add confidence |
| `prtip::OsMatch` | `spectre::OsInfo` | Aggregate matches |

```rust
impl From<prtip::Host> for spectre::Target {
    fn from(host: prtip::Host) -> Self {
        spectre::Target {
            id: Uuid::new_v4(),
            ip: host.address,
            hostname: host.hostname,
            mac: host.mac_address,
            discovered_at: Utc::now(),
            discovered_by: ComponentId::PrortIp,
            ports: host.ports.into_iter().map(Into::into).collect(),
            os_info: host.os_matches.first().map(Into::into),
            services: host.services.into_iter().map(Into::into).collect(),
            tags: HashSet::new(),
            notes: vec![],
        }
    }
}
```

### TUI Framework Reuse

SPECTRE's TUI inherits ProRT-IP's 60 FPS rendering framework:

```rust
// Shared TUI components from ProRT-IP
use prtip_tui::{
    widgets::{PortTable, NetworkGraph, MetricsDashboard},
    layout::{MultiPaneLayout, PaneConfig},
    events::{EventHandler, KeyEvent},
};

pub struct SpectreTui {
    // Reused ProRT-IP components
    recon_panel: PortTable,
    network_graph: NetworkGraph,
    metrics: MetricsDashboard,

    // SPECTRE-specific panels
    analysis_panel: AnalysisPanel,
    comms_panel: CommsPanel,
    campaign_panel: CampaignPanel,
}
```

---

## CyberChef-MCP Integration

### Integration Architecture

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                       CyberChef-MCP Integration                              │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌───────────────────┐                                                       │
│  │   SPECTRE Core    │                                                       │
│  └─────────┬─────────┘                                                       │
│            │                                                                 │
│            ▼                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                      MCP Client                                      │    │
│  │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────┐  │    │
│  │  │   Tool Schema   │  │   Request       │  │    Response         │  │    │
│  │  │   Registry      │  │   Builder       │  │    Parser           │  │    │
│  │  └─────────────────┘  └─────────────────┘  └─────────────────────┘  │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│            │                                                                 │
│            │  MCP Protocol (JSON-RPC over stdio)                            │
│            ▼                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                    Docker Container                                  │    │
│  │  ┌─────────────────────────────────────────────────────────────┐    │    │
│  │  │                  CyberChef-MCP Server                        │    │    │
│  │  │                                                              │    │    │
│  │  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐   │    │    │
│  │  │  │ 463 Atomic   │  │   Recipe     │  │    Batch         │   │    │    │
│  │  │  │ Operations   │  │   Manager    │  │    Processor     │   │    │    │
│  │  │  └──────────────┘  └──────────────┘  └──────────────────┘   │    │    │
│  │  │                                                              │    │    │
│  │  │  Image: doublegate/cyberchef-mcp:latest                      │    │    │
│  │  │  Base:  Chainguard distroless (~90MB)                        │    │    │
│  │  └─────────────────────────────────────────────────────────────┘    │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### MCP Protocol Client

**Rust MCP Client Implementation:**

```rust
use tokio::process::{Command, Child};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use serde_json::{json, Value};

pub struct CyberChefMcpClient {
    process: Child,
    stdin: BufWriter<tokio::process::ChildStdin>,
    stdout: BufReader<tokio::process::ChildStdout>,
    request_id: AtomicU64,
    tool_schemas: HashMap<String, ToolSchema>,
}

impl CyberChefMcpClient {
    /// Connect to CyberChef-MCP Docker container
    pub async fn connect() -> Result<Self, SpectreError> {
        let mut process = Command::new("docker")
            .args(["run", "-i", "--rm", "doublegate/cyberchef-mcp:latest"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        let stdin = BufWriter::new(process.stdin.take().unwrap());
        let stdout = BufReader::new(process.stdout.take().unwrap());

        let mut client = Self {
            process,
            stdin,
            stdout,
            request_id: AtomicU64::new(1),
            tool_schemas: HashMap::new(),
        };

        // Initialize MCP session
        client.initialize().await?;

        // Fetch tool schemas
        client.fetch_tool_schemas().await?;

        Ok(client)
    }

    /// Execute a CyberChef recipe
    pub async fn bake(
        &mut self,
        input: &str,
        recipe: Vec<RecipeStep>,
    ) -> Result<String, SpectreError> {
        let request = self.build_request(
            "tools/call",
            json!({
                "name": "cyberchef_bake",
                "arguments": {
                    "input": input,
                    "recipe": recipe
                }
            }),
        );

        let response = self.send_request(request).await?;
        self.parse_result(response)
    }

    /// Execute a single operation
    pub async fn execute_operation(
        &mut self,
        operation: &str,
        input: &str,
        args: Value,
    ) -> Result<String, SpectreError> {
        let tool_name = format!("cyberchef_{}", operation.to_lowercase());

        let mut arguments = json!({ "input": input });
        if let Value::Object(map) = args {
            for (k, v) in map {
                arguments[k] = v;
            }
        }

        let request = self.build_request(
            "tools/call",
            json!({
                "name": tool_name,
                "arguments": arguments
            }),
        );

        let response = self.send_request(request).await?;
        self.parse_result(response)
    }

    /// Search for operations by keyword
    pub async fn search_operations(
        &mut self,
        query: &str,
    ) -> Result<Vec<OperationInfo>, SpectreError> {
        let request = self.build_request(
            "tools/call",
            json!({
                "name": "cyberchef_search",
                "arguments": { "query": query }
            }),
        );

        let response = self.send_request(request).await?;
        serde_json::from_value(response["result"].clone())
            .map_err(Into::into)
    }

    /// Execute batch operations
    pub async fn batch_execute(
        &mut self,
        operations: Vec<BatchOperation>,
        mode: BatchMode,
    ) -> Result<Vec<BatchResult>, SpectreError> {
        let request = self.build_request(
            "tools/call",
            json!({
                "name": "cyberchef_batch",
                "arguments": {
                    "operations": operations,
                    "mode": mode.as_str()
                }
            }),
        );

        let response = self.send_request(request).await?;
        serde_json::from_value(response["result"].clone())
            .map_err(Into::into)
    }
}
```

### Recipe Management Integration

```rust
/// SPECTRE recipe wrapper for CyberChef recipes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpectreRecipe {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub category: RecipeCategory,
    pub steps: Vec<RecipeStep>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeStep {
    pub op: String,
    #[serde(default)]
    pub args: Value,
    #[serde(default)]
    pub disabled: bool,
}

impl CyberChefMcpClient {
    /// Save a recipe via CyberChef-MCP
    pub async fn save_recipe(
        &mut self,
        recipe: &SpectreRecipe,
    ) -> Result<(), SpectreError> {
        let request = self.build_request(
            "tools/call",
            json!({
                "name": "cyberchef_recipe_create",
                "arguments": {
                    "name": recipe.name,
                    "description": recipe.description,
                    "operations": recipe.steps,
                    "tags": recipe.tags
                }
            }),
        );

        self.send_request(request).await?;
        Ok(())
    }

    /// Execute a saved recipe by name
    pub async fn execute_recipe(
        &mut self,
        recipe_name: &str,
        input: &str,
    ) -> Result<String, SpectreError> {
        let request = self.build_request(
            "tools/call",
            json!({
                "name": "cyberchef_recipe_execute",
                "arguments": {
                    "name": recipe_name,
                    "input": input
                }
            }),
        );

        let response = self.send_request(request).await?;
        self.parse_result(response)
    }
}
```

### Common Security Recipes

SPECTRE includes pre-defined recipes for common security operations:

```yaml
# configs/recipes/security-defaults.yaml

recipes:
  - name: decode-credentials
    description: Decode common credential encoding schemes
    category: forensics
    steps:
      - op: From_Base64
      - op: URL_Decode
      - op: From_Hex
        args:
          delimiter: Auto

  - name: extract-iocs
    description: Extract indicators of compromise from text
    category: threat-intel
    steps:
      - op: Extract_IP_addresses
      - op: Extract_URLs
      - op: Extract_domains
      - op: Unique
      - op: Sort

  - name: deobfuscate-powershell
    description: Common PowerShell deobfuscation chain
    category: malware-analysis
    steps:
      - op: From_Base64
      - op: Decode_text
        args:
          encoding: UTF-16LE
      - op: Generic_Code_Beautify

  - name: hash-all
    description: Generate multiple hash types
    category: forensics
    steps:
      - op: Generate_all_hashes
```

---

## WRAITH-Protocol Integration

### Integration Architecture

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                      WRAITH-Protocol Integration                             │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌───────────────────┐                                                       │
│  │   SPECTRE Core    │                                                       │
│  └─────────┬─────────┘                                                       │
│            │                                                                 │
│            ▼                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                   Integration Layer                                  │    │
│  │  ┌───────────────────────┐  ┌───────────────────────────────────┐   │    │
│  │  │  Library Integration  │  │       FFI Bridge                   │   │    │
│  │  │  (wraith-* crates)    │  │   (wraith-ffi crate)               │   │    │
│  │  └───────────────────────┘  └───────────────────────────────────┘   │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│            │                                                                 │
│            ▼                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                     WRAITH Protocol Stack                            │    │
│  │                                                                      │    │
│  │  ┌────────────────┐  ┌────────────────┐  ┌────────────────────┐     │    │
│  │  │  wraith-core   │  │ wraith-crypto  │  │ wraith-transport   │     │    │
│  │  │  (Sessions,    │  │ (Noise_XX,     │  │ (UDP, TCP, WS,     │     │    │
│  │  │   Frames)      │  │  Ratchet)      │  │  QUIC, AF_XDP)     │     │    │
│  │  └────────────────┘  └────────────────┘  └────────────────────┘     │    │
│  │                                                                      │    │
│  │  ┌────────────────┐  ┌────────────────┐  ┌────────────────────┐     │    │
│  │  │wraith-obfuscate│  │wraith-discovery│  │   wraith-files     │     │    │
│  │  │ (Elligator2,   │  │ (Kademlia DHT, │  │ (Chunking,         │     │    │
│  │  │  Mimicry)      │  │  NAT traversal)│  │  BLAKE3 tree)      │     │    │
│  │  └────────────────┘  └────────────────┘  └────────────────────┘     │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Library Integration (Primary)

**Cargo.toml Dependencies:**
```toml
[dependencies]
wraith-core = { path = "../WRAITH-Protocol/crates/wraith-core" }
wraith-crypto = { path = "../WRAITH-Protocol/crates/wraith-crypto" }
wraith-transport = { path = "../WRAITH-Protocol/crates/wraith-transport" }
wraith-obfuscation = { path = "../WRAITH-Protocol/crates/wraith-obfuscation" }
wraith-files = { path = "../WRAITH-Protocol/crates/wraith-files" }
```

**Node and Session Management:**

```rust
use wraith_core::{Node, NodeConfig, Session, SessionId};
use wraith_crypto::{Identity, KeyPair, NoiseConfig};
use wraith_transport::{Transport, TransportConfig, ProtocolMimicry};
use wraith_obfuscation::{ObfuscationConfig, PaddingMode, TimingMode};

pub struct SpectreComms {
    node: Node,
    identity: Identity,
    active_sessions: HashMap<SessionId, Session>,
}

impl SpectreComms {
    /// Initialize WRAITH node for SPECTRE communications
    pub async fn new(config: SpectreCommsConfig) -> Result<Self, SpectreError> {
        // Generate or load identity
        let identity = match config.identity_path {
            Some(path) => Identity::load_from_file(&path)?,
            None => Identity::generate()?,
        };

        // Configure node
        let node_config = NodeConfig::builder()
            .identity(identity.clone())
            .bind_address(config.bind_address)
            .transport(TransportConfig {
                protocols: vec![
                    Transport::Udp,
                    Transport::Tcp,
                    Transport::WebSocket,
                ],
                mimicry: config.protocol_mimicry,
            })
            .obfuscation(ObfuscationConfig {
                padding: config.padding_mode,
                timing: config.timing_mode,
                cover_traffic: config.cover_traffic,
            })
            .discovery(config.discovery_config)
            .build()?;

        let node = Node::new(node_config).await?;

        Ok(Self {
            node,
            identity,
            active_sessions: HashMap::new(),
        })
    }

    /// Send file to peer with encryption
    pub async fn send_file(
        &mut self,
        file_path: &Path,
        peer_id: &str,
        options: SendOptions,
    ) -> Result<TransferResult, SpectreError> {
        // Resolve peer
        let peer = self.node.resolve_peer(peer_id).await?;

        // Establish session if needed
        let session = self.get_or_create_session(&peer).await?;

        // Configure transfer
        let transfer_config = TransferConfig {
            encrypt: options.encrypt,
            compress: options.compress,
            chunk_size: options.chunk_size.unwrap_or(64 * 1024),
            mimicry: options.mimicry,
        };

        // Send file with progress callback
        let result = session
            .send_file_with_progress(
                file_path,
                transfer_config,
                |progress| {
                    emit_event(TransferProgressEvent {
                        bytes_sent: progress.bytes_sent,
                        total_bytes: progress.total_bytes,
                        speed: progress.speed,
                        eta: progress.eta,
                    });
                },
            )
            .await?;

        Ok(result)
    }

    /// Receive files from any peer
    pub async fn receive_files(
        &mut self,
        output_dir: &Path,
        options: ReceiveOptions,
    ) -> Result<(), SpectreError> {
        self.node
            .set_receive_handler(move |transfer| {
                let output_path = output_dir.join(&transfer.filename);

                // Accept transfer
                transfer.accept(output_path).await?;

                emit_event(TransferCompleteEvent {
                    filename: transfer.filename,
                    size: transfer.size,
                    peer: transfer.peer_id,
                });

                Ok(())
            })
            .await;

        // Start listening
        self.node.start_receiving().await?;
        Ok(())
    }

    /// Establish secure channel for C2 operations
    pub async fn establish_c2_channel(
        &mut self,
        operator_peer: &str,
        options: C2Options,
    ) -> Result<C2Channel, SpectreError> {
        let peer = self.node.resolve_peer(operator_peer).await?;

        // Use maximum obfuscation for C2
        let session = self.node
            .connect_with_config(
                &peer,
                SessionConfig {
                    noise_pattern: NoisePattern::XX,
                    ratchet: true,
                    obfuscation: ObfuscationConfig {
                        padding: PaddingMode::Statistical,
                        timing: TimingMode::Exponential,
                        cover_traffic: CoverTrafficConfig::enabled(),
                    },
                    mimicry: options.mimicry.unwrap_or(ProtocolMimicry::Tls13),
                },
            )
            .await?;

        Ok(C2Channel::new(session))
    }
}
```

### WRAITH-RedOps Integration

For red team operations, SPECTRE can connect to WRAITH-RedOps team server:

```rust
use wraith_redops_client::{OperatorClient, Campaign, Listener};

pub struct SpectreRedOps {
    client: OperatorClient,
}

impl SpectreRedOps {
    /// Connect to WRAITH-RedOps team server
    pub async fn connect(
        server_url: &str,
        credentials: Credentials,
    ) -> Result<Self, SpectreError> {
        let client = OperatorClient::connect(
            server_url,
            credentials,
        ).await?;

        Ok(Self { client })
    }

    /// Create new campaign on team server
    pub async fn create_campaign(
        &self,
        campaign: CampaignConfig,
    ) -> Result<Campaign, SpectreError> {
        self.client.create_campaign(campaign).await
            .map_err(Into::into)
    }

    /// List active listeners
    pub async fn list_listeners(&self) -> Result<Vec<Listener>, SpectreError> {
        self.client.list_listeners().await
            .map_err(Into::into)
    }

    /// Get implant beacons
    pub async fn get_beacons(&self) -> Result<Vec<Beacon>, SpectreError> {
        self.client.get_beacons().await
            .map_err(Into::into)
    }
}
```

### Data Model Mapping

**WRAITH → SPECTRE:**

| WRAITH Type | SPECTRE Type | Notes |
|-------------|--------------|-------|
| `wraith::PeerId` | `spectre::PeerId` | Direct alias |
| `wraith::TransferResult` | `spectre::TransferArtifact` | Add campaign context |
| `wraith::Session` | `spectre::SecureChannel` | Wrapper with state |

---

## Inter-Component Data Pipeline

### Pipeline Architecture

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                        Data Pipeline Architecture                            │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────┐                                                             │
│  │  ProRT-IP   │───────┐                                                     │
│  │  ScanResult │       │                                                     │
│  └─────────────┘       │                                                     │
│                        ▼                                                     │
│               ┌─────────────────┐                                            │
│               │   Data Router   │                                            │
│               │                 │                                            │
│               │ • Format detect │                                            │
│               │ • Schema valid  │                                            │
│               │ • Transform     │                                            │
│               │ • Route         │                                            │
│               └────────┬────────┘                                            │
│                        │                                                     │
│         ┌──────────────┼──────────────┐                                      │
│         ▼              ▼              ▼                                      │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐                            │
│  │  CyberChef  │ │   WRAITH    │ │   Report    │                            │
│  │  Analysis   │ │   Transfer  │ │   Generator │                            │
│  └─────────────┘ └─────────────┘ └─────────────┘                            │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Pipeline Implementation

```rust
pub struct DataPipeline {
    router: DataRouter,
    transformers: HashMap<String, Box<dyn Transformer>>,
}

impl DataPipeline {
    /// Route scan results to analysis
    pub async fn scan_to_analysis(
        &self,
        scan_results: Vec<ScanResult>,
        recipe: &str,
    ) -> Result<AnalysisResult, SpectreError> {
        // Extract relevant data from scan results
        let banners: Vec<String> = scan_results
            .iter()
            .flat_map(|r| r.services.iter())
            .filter_map(|s| s.banner.clone())
            .collect();

        // Route to CyberChef for analysis
        let chef_client = self.router.get_cyberchef_client()?;
        let analyzed = chef_client
            .execute_recipe(recipe, &banners.join("\n"))
            .await?;

        Ok(AnalysisResult {
            source: "scan_banners".to_string(),
            data: analyzed,
            metadata: json!({
                "banner_count": banners.len(),
                "recipe": recipe
            }),
        })
    }

    /// Route analysis results to exfiltration
    pub async fn analysis_to_exfil(
        &self,
        analysis: &AnalysisResult,
        peer_id: &str,
        options: ExfilOptions,
    ) -> Result<TransferArtifact, SpectreError> {
        // Write analysis to temp file
        let temp_file = self.write_temp_file(&analysis.data)?;

        // Route to WRAITH for secure transfer
        let comms = self.router.get_wraith_comms()?;
        let result = comms
            .send_file(&temp_file, peer_id, SendOptions {
                encrypt: true,
                mimicry: options.mimicry,
                ..Default::default()
            })
            .await?;

        // Create artifact record
        Ok(TransferArtifact {
            id: Uuid::new_v4(),
            source: analysis.source.clone(),
            transfer_result: result,
            timestamp: Utc::now(),
        })
    }
}
```

### Format Negotiation

```rust
#[derive(Debug, Clone, Copy)]
pub enum DataFormat {
    Json,
    Protobuf,
    Xml,
    Text,
    Binary,
}

impl DataRouter {
    /// Negotiate format between source and destination
    fn negotiate_format(
        &self,
        source: ComponentId,
        destination: ComponentId,
    ) -> DataFormat {
        match (source, destination) {
            // ProRT-IP outputs JSON natively
            (ComponentId::PrortIp, _) => DataFormat::Json,

            // CyberChef accepts text/binary
            (_, ComponentId::CyberChef) => DataFormat::Text,

            // WRAITH handles binary efficiently
            (_, ComponentId::Wraith) => DataFormat::Binary,

            // Default to JSON
            _ => DataFormat::Json,
        }
    }
}
```

---

## Event System

### Event Bus

```rust
use tokio::sync::broadcast;

pub struct EventBus {
    sender: broadcast::Sender<SpectreEvent>,
}

#[derive(Debug, Clone)]
pub enum SpectreEvent {
    // Scan events
    ScanStarted { campaign_id: Uuid, target_count: usize },
    ScanProgress { completed: usize, total: usize, rate: f64 },
    ScanCompleted { campaign_id: Uuid, results: usize },

    // Analysis events
    AnalysisStarted { input_size: usize, recipe: String },
    AnalysisCompleted { output_size: usize, duration_ms: u64 },

    // Transfer events
    TransferStarted { peer_id: String, filename: String },
    TransferProgress { bytes_sent: u64, total_bytes: u64 },
    TransferCompleted { peer_id: String, hash: String },

    // Campaign events
    CampaignPhaseChanged { campaign_id: Uuid, phase: CampaignPhase },
    CampaignStateChanged { campaign_id: Uuid, state: CampaignState },

    // Component health
    ComponentHealthChanged { component: ComponentId, status: HealthStatus },
}

impl EventBus {
    pub fn subscribe(&self) -> broadcast::Receiver<SpectreEvent> {
        self.sender.subscribe()
    }

    pub fn emit(&self, event: SpectreEvent) {
        let _ = self.sender.send(event);
    }
}
```

---

## Health Monitoring

### Component Health Checks

```rust
pub struct HealthMonitor {
    components: HashMap<ComponentId, Box<dyn HealthCheck>>,
}

#[async_trait]
pub trait HealthCheck {
    async fn check(&self) -> HealthStatus;
    fn component_id(&self) -> ComponentId;
}

impl HealthMonitor {
    pub async fn check_all(&self) -> HashMap<ComponentId, HealthStatus> {
        let mut results = HashMap::new();

        for (id, checker) in &self.components {
            let status = checker.check().await;
            results.insert(*id, status);
        }

        results
    }
}

// ProRT-IP health check
struct PrtipHealthCheck;

#[async_trait]
impl HealthCheck for PrtipHealthCheck {
    async fn check(&self) -> HealthStatus {
        // Try to create a scanner instance
        match Scanner::new(ScanConfig::default()) {
            Ok(_) => HealthStatus::Healthy,
            Err(e) => HealthStatus::Unhealthy(e.to_string()),
        }
    }

    fn component_id(&self) -> ComponentId {
        ComponentId::PrortIp
    }
}

// CyberChef-MCP health check
struct CyberChefHealthCheck;

#[async_trait]
impl HealthCheck for CyberChefHealthCheck {
    async fn check(&self) -> HealthStatus {
        // Check Docker container status
        let output = Command::new("docker")
            .args(["inspect", "--format", "{{.State.Running}}", "cyberchef-mcp"])
            .output()
            .await;

        match output {
            Ok(o) if o.stdout.starts_with(b"true") => HealthStatus::Healthy,
            Ok(_) => HealthStatus::Unhealthy("Container not running".to_string()),
            Err(e) => HealthStatus::Unhealthy(e.to_string()),
        }
    }

    fn component_id(&self) -> ComponentId {
        ComponentId::CyberChef
    }
}
```

---

## References

- [SYSTEM-DESIGN.md](SYSTEM-DESIGN.md) — Overall system architecture
- [INTERFACE-MODES.md](INTERFACE-MODES.md) — Interface specifications
- [ProRT-IP API](https://github.com/doublegate/ProRT-IP/blob/main/docs/00-ARCHITECTURE.md)
- [WRAITH-Protocol API](https://github.com/doublegate/WRAITH-Protocol/blob/main/docs/INTEGRATION_GUIDE.md)
- [CyberChef-MCP Tools](https://github.com/doublegate/CyberChef-MCP/blob/main/docs/guides/commands.md)
