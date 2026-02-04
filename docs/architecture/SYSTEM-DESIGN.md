# SPECTRE System Design

**Version:** 0.1.0 | **Status:** Design Phase | **Last Updated:** 2026-02-04

---

## Executive Summary

SPECTRE is a unified offensive security platform that orchestrates three battle-tested components—WRAITH-Protocol, ProRT-IP WarScan, and CyberChef-MCP—into a cohesive operational system. This document defines the system architecture, component interactions, and technical decisions.

---

## Design Principles

### 1. Component Independence

Each integrated component remains a standalone, production-ready project:

- **No tight coupling** — Components communicate via well-defined interfaces
- **Independent versioning** — Components can be upgraded independently
- **Fallback capability** — SPECTRE degrades gracefully if a component is unavailable

### 2. Interface Plurality

Four interaction modes serve different operational contexts:

| Mode | Use Case | Primary Users |
|------|----------|---------------|
| CLI | Scripting, automation, CI/CD | Operators, scripts |
| TUI | Real-time monitoring, rapid operation | Operators |
| GUI | Visual planning, collaboration | Teams, analysts |
| MCP | AI-assisted operations | AI assistants |

### 3. Data-Centric Architecture

Unified data models enable seamless component interaction:

- **Standardized schemas** for targets, findings, artifacts
- **Format negotiation** between components (JSON, Protobuf, XML)
- **Evidence chain of custody** with cryptographic verification

### 4. Security by Default

- All inter-component communication encrypted
- No cleartext secrets in configuration
- Audit logging for compliance
- Principle of least privilege

---

## System Architecture

### High-Level Overview

```text
┌─────────────────────────────────────────────────────────────────────────────────┐
│                              SPECTRE PLATFORM                                   │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ┌────────────────────────────────────────────────────────────────────────────┐ │
│  │                         INTERFACE LAYER                                    │ │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌────────────────────────────┐  │ │
│  │  │   CLI    │  │   TUI    │  │   GUI    │  │       MCP Server           │  │ │
│  │  │  Clap    │  │ Ratatui  │  │  Tauri   │  │   Model Context Protocol   │  │ │
│  │  │          │  │  60 FPS  │  │  2.0     │  │   Stdio Transport          │  │ │
│  │  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────────────┬───────────────┘  │ │
│  │       └─────────────┴─────────────┴─────────────────────┘                  │ │
│  └────────────────────────────────────┬───────────────────────────────────────┘ │
│                                       │                                         │
│  ┌────────────────────────────────────▼───────────────────────────────────────┐ │
│  │                         ORCHESTRATION LAYER                                │ │
│  │                                                                            │ │
│  │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────────────┐ │ │
│  │  │    Campaign     │  │      Data       │  │         Workflow            │ │ │
│  │  │    Manager      │  │     Router      │  │          Engine             │ │ │
│  │  │                 │  │                 │  │                             │ │ │
│  │  │  • State mgmt   │  │  • Format conv  │  │  • DAG execution            │ │ │
│  │  │  • Phase track  │  │  • Validation   │  │  • Dependency resolution    │ │ │
│  │  │  • Timeline     │  │  • Routing      │  │  • Parallel scheduling      │ │ │
│  │  └─────────────────┘  └─────────────────┘  └─────────────────────────────┘ │ │
│  │                                                                            │ │
│  │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────────────┐ │ │
│  │  │     Config      │  │      Event      │  │          Plugin             │ │ │
│  │  │     Store       │  │       Bus       │  │          System             │ │ │
│  │  │                 │  │                 │  │                             │ │ │
│  │  │  • TOML config  │  │  • Pub/Sub      │  │  • Lua 5.4 sandbox          │ │ │
│  │  │  • Env overlay  │  │  • Async events │  │  • Custom workflows         │ │ │
│  │  │  • Validation   │  │  • Logging      │  │  • Hook points              │ │ │
│  │  └─────────────────┘  └─────────────────┘  └─────────────────────────────┘ │ │
│  └────────────────────────────────────┬───────────────────────────────────────┘ │
│                                       │                                         │
│  ┌────────────────────────────────────▼───────────────────────────────────────┐ │
│  │                         COMPONENT LAYER                                    │ │
│  │                                                                            │ │
│  │  ┌───────────────────┐  ┌───────────────────┐  ┌───────────────────────┐   │ │
│  │  │     ProRT-IP      │  │   CyberChef-MCP   │  │   WRAITH-Protocol     │   │ │
│  │  │     WarScan       │  │                   │  │                       │   │ │
│  │  ├───────────────────┤  ├───────────────────┤  ├───────────────────────┤   │ │
│  │  │ Integration Mode: │  │ Integration Mode: │  │ Integration Mode:     │   │ │
│  │  │ • Library (Rust)  │  │ • MCP Protocol    │  │ • Library (Rust)      │   │ │
│  │  │ • CLI subprocess  │  │ • Docker stdio    │  │ • FFI bindings        │   │ │
│  │  │                   │  │                   │  │ • CLI subprocess      │   │ │
│  │  └───────────────────┘  └───────────────────┘  └───────────────────────┘   │ │
│  │                                                                            │ │
│  └────────────────────────────────────────────────────────────────────────────┘ │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Layer Specifications

### Interface Layer

The interface layer provides four methods of interaction, all communicating with the same orchestration core.

#### CLI (spectre-cli)

**Technology:** Rust, clap 4.x

**Responsibilities:**
- Parse command-line arguments
- Route to appropriate component
- Format output (text, JSON, XML)
- Support shell pipelines

**Command Structure:**
```
spectre <subcommand> [options] [target]

Subcommands:
  scan      Network scanning (ProRT-IP)
  chef      Data analysis (CyberChef)
  send      File transfer (WRAITH)
  receive   File receive (WRAITH)
  campaign  Campaign management
  config    Configuration
  status    Health check
```

#### TUI (spectre-tui)

**Technology:** Rust, ratatui (forked from ProRT-IP TUI)

**Responsibilities:**
- Real-time dashboard rendering (60 FPS target)
- Multi-pane layout management
- Keyboard-driven navigation
- Live data visualization

**Architecture:**
```text
┌───────────────────────────────────────────────────────────────────────┐
│                      TUI Application                                  │
├───────────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐   ┌────────────┐ │
│  │   Event     │   │    State    │   │   Widget    │   │  Terminal  │ │
│  │   Handler   │──▶│   Manager   │──▶│  Renderer   │──▶│  Backend   │ │
│  │  (async)    │   │  (tokio)    │   │ (ratatui)   │   │ (crossterm)│ │
│  └─────────────┘   └─────────────┘   └─────────────┘   └────────────┘ │
└───────────────────────────────────────────────────────────────────────┘
```

#### GUI (spectre-gui)

**Technology:** Tauri 2.0, React, TypeScript

**Responsibilities:**
- Visual campaign planning
- Network topology visualization
- Multi-operator collaboration
- Report generation

**Architecture:**
```text
┌───────────────────────────────────────────────────────────────────┐
│                      Tauri Application                            │
├───────────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────────────┐  │
│  │                    React Frontend                           │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │  │
│  │  │   Zustand   │  │   React     │  │    TailwindCSS      │  │  │
│  │  │   State     │  │   Router    │  │    + shadcn/ui      │  │  │
│  │  └─────────────┘  └─────────────┘  └─────────────────────┘  │  │
│  └─────────────────────────────────────────────────────────────┘  │
│                              │                                    │
│                        Tauri IPC                                  │
│                              │                                    │
│  ┌─────────────────────────────────────────────────────────────┐  │
│  │                    Rust Backend                             │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │  │
│  │  │   IPC       │  │   State     │  │    SPECTRE Core     │  │  │
│  │  │   Commands  │  │   Manager   │  │    Integration      │  │  │
│  │  └─────────────┘  └─────────────┘  └─────────────────────┘  │  │
│  └─────────────────────────────────────────────────────────────┘  │
└───────────────────────────────────────────────────────────────────┘
```

#### MCP Server (spectre-mcp)

**Technology:** Rust, MCP Protocol (stdio transport)

**Responsibilities:**
- Expose SPECTRE operations as MCP tools
- Handle AI assistant requests
- Manage tool schemas and validation
- Stream results for long operations

**Tool Categories:**

| Prefix | Source | Tool Count |
|--------|--------|------------|
| `spectre_scan_*` | ProRT-IP | 12 |
| `spectre_detect_*` | ProRT-IP | 4 |
| `spectre_chef_*` | CyberChef | 463+ |
| `spectre_send/receive` | WRAITH | 4 |
| `spectre_campaign_*` | Core | 8 |

---

### Orchestration Layer

The orchestration layer coordinates all component interactions.

#### Campaign Manager

Manages operational campaigns with state machine semantics:

```text
Campaign States:
  PLANNING → READY → ACTIVE → PAUSED → COMPLETED
                        ↓
                     ABORTED

Campaign Phases:
  RECON → ANALYSIS → EXPLOITATION → EXFIL → REPORTING
```

**Data Model:**
```rust
struct Campaign {
    id: Uuid,
    name: String,
    codename: String,
    state: CampaignState,
    phase: CampaignPhase,
    targets: Vec<Target>,
    findings: Vec<Finding>,
    artifacts: Vec<Artifact>,
    timeline: Vec<Event>,
    config: CampaignConfig,
}
```

#### Data Router

Handles data format conversion and routing between components:

**Supported Formats:**
- JSON (primary interchange format)
- Protocol Buffers (high-performance binary)
- XML (Nmap compatibility)
- YAML (configuration, recipes)

**Routing Logic:**
```text
scan_result.json → [Data Router] → chef_input.json
                                 → wraith_payload.bin
                                 → report_data.json
```

#### Workflow Engine

Executes complex multi-step workflows defined as DAGs:

```yaml
# Example workflow definition
name: red-team-recon
steps:
  - id: scan
    component: prtip
    action: syn_scan
    config:
      ports: "1-1000"
      rate: 10000

  - id: analyze-banners
    component: cyberchef
    action: execute_recipe
    depends_on: [scan]
    config:
      recipe: "Extract_Strings,Find_Patterns"

  - id: exfil-results
    component: wraith
    action: send
    depends_on: [analyze-banners]
    config:
      peer: c2-server
      encrypt: true
```

#### Event Bus

Pub/Sub system for loose coupling:

```text
Event Types:
  • scan.started / scan.completed / scan.progress
  • analysis.started / analysis.completed
  • transfer.started / transfer.completed
  • campaign.phase_changed / campaign.state_changed
  • component.health_changed
```

---

### Component Layer

Each component integrates via its optimal interface.

#### ProRT-IP Integration

**Primary Method:** Rust library linkage

```rust
// Direct library usage
use prtip_core::{Scanner, ScanConfig, ScanType};

let config = ScanConfig::builder()
    .scan_type(ScanType::Syn)
    .ports("1-1000")
    .rate_limit(10000)
    .build();

let scanner = Scanner::new(config)?;
let results = scanner.scan(targets).await?;
```

**Fallback Method:** CLI subprocess

```rust
// When library unavailable
let output = Command::new("prtip")
    .args(["-sS", "-p", "1-1000", "-oJ", "-", target])
    .output()?;
```

**Data Flow:**
```text
SPECTRE → ScanConfig → ProRT-IP → ScanResults → Data Router
```

#### CyberChef-MCP Integration

**Primary Method:** MCP Protocol (stdio)

```rust
// MCP client to CyberChef server
let mcp_client = McpClient::connect_docker("cyberchef-mcp")?;

let result = mcp_client.call_tool(
    "cyberchef_bake",
    json!({
        "input": data,
        "recipe": [
            {"op": "From_Base64"},
            {"op": "Gunzip"}
        ]
    })
).await?;
```

**Docker Integration:**
```rust
// Spawn CyberChef container
let container = Docker::run("doublegate/cyberchef-mcp:latest")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()?;
```

**Data Flow:**
```text
SPECTRE → JSON/MCP → Docker Container → CyberChef → JSON Result → Data Router
```

#### WRAITH-Protocol Integration

**Primary Method:** Rust library linkage

```rust
// Direct library usage
use wraith_core::{Node, SendConfig, TransferResult};
use wraith_crypto::{Identity, KeyPair};

let identity = Identity::generate()?;
let node = Node::builder()
    .identity(identity)
    .bind("0.0.0.0:0")
    .build()
    .await?;

let result = node.send_file(
    "sensitive.db",
    peer_id,
    SendConfig::encrypted()
        .mimicry(ProtocolMimicry::Tls13)
).await?;
```

**Data Flow:**
```text
SPECTRE → File/Data → WRAITH Node → Encrypted Channel → Peer
```

---

## Data Models

### Unified Target Model

```rust
struct Target {
    id: Uuid,

    // Network identification
    ip: IpAddr,
    hostname: Option<String>,
    mac: Option<MacAddr>,

    // Discovery metadata
    discovered_at: DateTime<Utc>,
    discovered_by: ComponentId,

    // Scan results
    ports: Vec<PortInfo>,
    os_info: Option<OsInfo>,
    services: Vec<ServiceInfo>,

    // Campaign context
    tags: HashSet<String>,
    notes: Vec<Note>,
}

struct PortInfo {
    port: u16,
    protocol: Protocol,
    state: PortState,
    service: Option<ServiceInfo>,
    banner: Option<String>,
}
```

### Unified Finding Model

```rust
struct Finding {
    id: Uuid,
    target_id: Uuid,

    // Classification
    category: FindingCategory,
    severity: Severity,
    confidence: f32,  // 0.0 - 1.0

    // Details
    title: String,
    description: String,
    evidence: Vec<Evidence>,

    // Provenance
    discovered_at: DateTime<Utc>,
    discovered_by: ComponentId,
    workflow_id: Option<Uuid>,

    // References
    cve: Option<Vec<String>>,
    cwe: Option<Vec<String>>,
    mitre_attack: Option<Vec<String>>,
}
```

### Unified Artifact Model

```rust
struct Artifact {
    id: Uuid,
    campaign_id: Uuid,

    // Content
    artifact_type: ArtifactType,
    name: String,
    data: ArtifactData,

    // Integrity
    hash_sha256: String,
    hash_blake3: String,

    // Chain of custody
    created_at: DateTime<Utc>,
    created_by: ComponentId,
    transformations: Vec<Transformation>,
}
```

---

## Security Architecture

### Threat Model

**Assets Protected:**
1. Target data and findings
2. Operational security (OPSEC)
3. C2 channel integrity
4. Campaign artifacts

**Threat Categories:**
1. Network interception (MITM)
2. Component compromise
3. Data exfiltration (unauthorized)
4. Audit trail tampering

### Security Controls

| Control | Implementation |
|---------|----------------|
| Transport encryption | WRAITH protocol (XChaCha20-Poly1305) |
| At-rest encryption | SQLCipher, AES-256-GCM |
| Authentication | Ed25519 signatures, Noise_XX |
| Authorization | Role-based (operator, admin, viewer) |
| Audit logging | Append-only log with BLAKE3 hashing |
| Secret management | Environment variables, no disk storage |

### Secure Communication Channels

```text
┌──────────────────────────────────────────────────────────┐
│                    Security Layers                       │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  ┌───────────┐                           ┌───────────┐   │
│  │  SPECTRE  │◄──── WRAITH Protocol ────►│    Peer   │   │
│  │  Instance │     (E2EE, PFS, TAR)      │  Instance │   │
│  └───────────┘                           └───────────┘   │
│                                                          │
│  Encryption: XChaCha20-Poly1305 (256-bit)                │
│  Key Exchange: Noise_XX + Double Ratchet                 │
│  Identity: Ed25519 signatures                            │
│  Obfuscation: Elligator2, protocol mimicry               │
│  Post-Quantum: X25519 + ML-KEM-768 hybrid (optional)     │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

---

## Performance Targets

| Metric | Target | Component |
|--------|--------|-----------|
| CLI startup | <100ms | spectre-cli |
| TUI frame time | <16ms (60 FPS) | spectre-tui |
| Scan throughput | 10M+ pps | ProRT-IP |
| Transfer throughput | 10+ Gbps | WRAITH |
| CyberChef operation | <1s for 10MB | CyberChef-MCP |
| MCP response time | <500ms | spectre-mcp |

---

## Deployment Models

### Standalone (Single Operator)

```text
┌───────────────────────────────────────┐
│           Operator Workstation        │
│  ┌─────────────────────────────────┐  │
│  │          SPECTRE                │  │
│  │  CLI / TUI / GUI / MCP          │  │
│  │         ↓         ↓             │  │
│  │    ProRT-IP    WRAITH           │  │
│  │         ↓                       │  │
│  │    [Docker: CyberChef-MCP]      │  │
│  └─────────────────────────────────┘  │
└───────────────────────────────────────┘
```

### Team (Multi-Operator)

```text
┌───────────────────────────────────────────────────────────┐
│                        Team Server                        │
│  ┌─────────────────────────────────────────────────────┐  │
│  │               WRAITH Team Server                    │  │
│  │  (Campaign coordination, artifact storage)          │  │
│  └─────────────────────────────────────────────────────┘  │
└───────────────────────────────────────────────────────────┘
                              │
         ┌────────────────────┼────────────────────┐
         ▼                    ▼                    ▼
┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│   Operator 1    │  │   Operator 2    │  │   Operator 3    │
│   SPECTRE       │  │   SPECTRE       │  │   SPECTRE       │
│   (GUI)         │  │   (TUI)         │  │   (CLI)         │
└─────────────────┘  └─────────────────┘  └─────────────────┘
```

---

## Technology Stack Summary

| Layer | Technology | Language |
|-------|------------|----------|
| CLI | clap 4.x | Rust |
| TUI | ratatui, crossterm | Rust |
| GUI Frontend | React, TypeScript, Tailwind | TypeScript |
| GUI Backend | Tauri 2.0 | Rust |
| MCP Server | MCP Protocol | Rust |
| Core Library | tokio, serde | Rust |
| Scanning | ProRT-IP (pnet, socket2) | Rust |
| Analysis | CyberChef-MCP (Node.js) | JavaScript |
| Communications | WRAITH-Protocol | Rust |
| Configuration | TOML | - |
| Data Interchange | JSON, Protocol Buffers | - |
| Containerization | Docker (Chainguard distroless) | - |

---

## References

- [INTEGRATION-SPEC.md](INTEGRATION-SPEC.md) — Component integration details
- [INTERFACE-MODES.md](INTERFACE-MODES.md) — Interface specifications
- [ProRT-IP Architecture](https://github.com/doublegate/ProRT-IP/blob/main/docs/00-ARCHITECTURE.md)
- [WRAITH-Protocol Architecture](https://github.com/doublegate/WRAITH-Protocol/blob/main/docs/architecture/protocol-overview.md)
- [CyberChef-MCP Architecture](https://github.com/doublegate/CyberChef-MCP/blob/main/docs/architecture/architecture.md)
