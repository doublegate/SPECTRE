# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

---

## Project Overview

**SPECTRE** (Security Platform for Encrypted Comms, Testing, Enumeration, Recon) is a unified offensive security toolkit that orchestrates three standalone components:

| Component | Role | Repository | Version | Tests |
|-----------|------|------------|---------|-------|
| **ProRT-IP** | Network reconnaissance (10M+ pps scanning) | Rust | v1.0.0 | 2,557 |
| **CyberChef-MCP** | Data analysis (463 operations via MCP) | TypeScript/Docker | v1.8.0 | 563 |
| **WRAITH-Protocol** | Secure communications (10+ Gbps E2EE) | Rust | v2.3.7 | 2,957 |

**Current Phase:** Operation BLACKOUT (v0.1.x) - Foundation/CLI skeleton **COMPLETE**

**SPECTRE Tests:** 86 (34 CLI + 51 core + 1 doc-test) | **Code:** ~7,600 lines Rust (32 files)

**Repository:** [github.com/doublegate/SPECTRE](https://github.com/doublegate/SPECTRE)

---

## Build Commands

```bash
# Build all Rust components (workspace)
cargo build --release --workspace

# Build specific crate
cargo build --release -p spectre-cli

# Pull CyberChef-MCP container
docker pull doublegate/cyberchef-mcp:latest

# Verify installation
./target/release/spectre --version
./target/release/spectre status
```

---

## Testing Commands

```bash
# Run all tests
cargo test --workspace

# Run tests with output
cargo test --workspace -- --nocapture

# Run specific crate tests
cargo test -p spectre-core

# Run tests with coverage (requires cargo-tarpaulin)
cargo tarpaulin --workspace --out Html

# Lint and format
cargo fmt --all --check
cargo clippy --workspace -- -D warnings
```

---

## Architecture

### Integration Layers

| Layer | Technology | Purpose |
|-------|------------|---------|
| CLI Orchestrator | Rust (clap) | Unified command interface |
| TUI Framework | Rust (ratatui) | Real-time dashboard (60 FPS) |
| GUI Application | Tauri 2.0, React | Visual campaign planning |
| MCP Server | Rust, MCP Protocol | AI-assisted operations |
| Data Pipeline | JSON, Protocol Buffers | Inter-component data flow |
| Plugin System | Lua 5.4 (sandboxed) | Extensibility |

### Data Flow

```
Target Network → ProRT-IP (Recon) → CyberChef (Analysis) → WRAITH (Exfil/C2)
                    ↓                    ↓                      ↓
                scan.json           decoded.txt           secure_channel
```

### Component Structure

```
crates/
├── spectre-cli/        # Unified CLI orchestrator (14 files, 34 tests)
│   └── src/
│       ├── main.rs         # Entry point, CLI parsing with clap 4
│       ├── commands/       # scan, chef, send, receive, identity, peer, status, config, completions
│       └── output/         # table (comfy-table) and json (serde_json) formatters
├── spectre-core/       # Core orchestration library (15 files, 52 tests)
│   └── src/
│       ├── config/         # TOML config with file discovery and env var support
│       ├── scan/           # Scanner trait, port/target parsing, 8 scan types
│       ├── chef/           # Chef trait, MCP client, Docker management (bollard)
│       ├── comms/          # Identity generation/storage, peer management
│       ├── error.rs        # SpectreError enum (thiserror)
│       └── logging.rs      # tracing-subscriber with env-filter
├── spectre-tui/        # TUI dashboard (planned - Phase 3)
├── spectre-gui/        # GUI application (planned - Phase 5)
└── spectre-mcp/        # MCP server (planned - Phase 6)
```

---

## Documentation Structure

```
docs/
├── architecture/       # System design documentation
│   ├── SYSTEM-DESIGN.md
│   ├── INTEGRATION-SPEC.md
│   └── INTERFACE-MODES.md
├── user-guide/         # Usage documentation
│   ├── QUICK-START.md
│   ├── CLI-REFERENCE.md
│   ├── TUI-GUIDE.md
│   └── MCP-TOOLS.md
├── integration/        # Component integration guides
│   ├── WRAITH-INTEGRATION.md
│   ├── PRTIP-INTEGRATION.md
│   └── CYBERCHEF-INTEGRATION.md
├── briefings/          # Mission briefing templates
│   ├── OPORD-template.md
│   ├── SITREP.md
│   ├── CONOP-template.md
│   └── AAR-template.md
├── development/        # Developer documentation
│   ├── SETUP.md           # Development environment setup
│   ├── ARCHITECTURE.md    # Internal architecture details
│   ├── TESTING.md         # Testing strategy and practices
│   └── DEBUGGING.md       # Debugging techniques
├── api/                # API specifications
│   ├── REST-API.md        # REST API for GUI backend
│   ├── MCP-PROTOCOL.md    # MCP protocol details
│   └── PLUGIN-API.md      # Lua plugin development
├── security/           # Security documentation
│   ├── THREAT-MODEL.md    # Threat model and mitigations
│   ├── ENCRYPTION.md      # Cryptographic implementations
│   └── OPERATIONAL-SECURITY.md  # OpSec best practices
├── deployment/         # Deployment guides
│   ├── INSTALLATION.md    # Platform installation
│   ├── CONFIGURATION.md   # Configuration reference
│   └── DOCKER.md          # Docker deployment
├── tutorials/          # Step-by-step tutorials
│   ├── FIRST-SCAN.md      # Getting started with scanning
│   ├── SECURE-CHANNEL.md  # Encrypted communications
│   ├── DATA-ANALYSIS.md   # CyberChef workflows
│   └── CAMPAIGN-PLANNING.md  # Campaign orchestration
└── reference/          # Reference materials
    ├── GLOSSARY.md        # Security terminology
    ├── FAQ.md             # Frequently asked questions
    └── TROUBLESHOOTING.md # Common issues and solutions

to-dos/                 # Sprint planning and roadmap
├── README.md           # Planning overview
├── ROADMAP.md          # Product roadmap
├── PHASE-1-FOUNDATION.md   # Operation BLACKOUT (v0.1.x)
├── PHASE-2-INTEGRATION.md  # Operation NIGHTFALL (v0.2.x)
├── PHASE-3-TUI.md          # Operation PHANTOM (v0.3.x)
├── PHASE-4-ADVANCED.md     # Operation SPECTER (v0.4.x)
├── PHASE-5-GUI.md          # Operation SHADOW (v0.5.x)
├── PHASE-6-MCP.md          # Operation WRAITH (v0.6.x)
└── PHASE-7-RELEASE.md      # Operation GENESIS (v1.0.0)
```

---

## Development Standards

- **Rust:** `cargo fmt`, `cargo clippy -- -D warnings`
- **TypeScript/JavaScript:** ESLint, Prettier
- **Commits:** [Conventional Commits](https://www.conventionalcommits.org/)
- **Testing:** TDD approach, all PRs require tests
- **Documentation:** Keep README, CHANGELOG, and docs in sync

---

## Documentation Style

This project uses military operational formatting for campaign documentation:

| Template | Purpose | Location |
|----------|---------|----------|
| OPORD | Operations order (5-paragraph campaign planning) | `docs/briefings/OPORD-template.md` |
| SITREP | Situation reports (real-time status updates) | `docs/briefings/SITREP.md` |
| CONOP | Concept of operations (detailed methodology) | `docs/briefings/CONOP-template.md` |
| AAR | After action review (post-campaign analysis) | `docs/briefings/AAR-template.md` |

---

## Requirements

- **Rust 1.88+** (WRAITH, ProRT-IP, SPECTRE CLI)
- **Node.js 22+** (CyberChef-MCP)
- **Docker** (recommended for CyberChef-MCP)
- **Linux kernel 6.2+** (recommended for AF_XDP/io_uring performance)
- **libpcap** (Linux/macOS) or **Npcap** (Windows) for raw packet access

---

## Key Files

| File | Purpose |
|------|---------|
| `README.md` | Project overview and quick start |
| `CHANGELOG.md` | Version history and release notes |
| `CLAUDE.md` | AI assistant guidance (this file) |
| `Cargo.toml` | Workspace manifest |
| `configs/spectre.toml` | Main configuration file |
| `CONTRIBUTING.md` | Contribution guidelines |
| `SECURITY.md` | Security policy and vulnerability reporting |
| `to-dos/ROADMAP.md` | Product roadmap and timeline |

---

## Sprint Planning

Development is organized into 7 phases (56 sprints total):

| Phase | Codename | Version | Focus |
|-------|----------|---------|-------|
| 1 | Operation BLACKOUT | v0.1.x | Foundation - CLI skeleton, component integration **COMPLETE** |
| 2 | Operation NIGHTFALL | v0.2.x | Data pipeline, scan-to-analysis automation |
| 3 | Operation PHANTOM | v0.3.x | TUI dashboard (60 FPS), real-time visualization |
| 4 | Operation SPECTER | v0.4.x | Advanced features, workflows, plugins |
| 5 | Operation SHADOW | v0.5.x | GUI application (Tauri 2.0) |
| 6 | Operation WRAITH | v0.6.x | MCP server implementation |
| 7 | Operation GENESIS | v1.0.0 | Production release |

See `to-dos/` directory for detailed sprint planning.

---

## CLI Commands Reference

```bash
# Core commands
spectre scan [flags] <targets>    # Network scanning (ProRT-IP interface)
spectre chef [operation] [flags]  # Data analysis (CyberChef-MCP interface)
spectre send [flags]              # Secure send (WRAITH interface)
spectre receive [flags]           # Secure receive (WRAITH interface)
spectre identity <subcommand>     # Cryptographic identity management
spectre peer <subcommand>         # Trusted peer management
spectre status                    # Component health checks
spectre config <subcommand>       # Configuration management
spectre completions <shell>       # Shell completion generation

# Key scan flags (Nmap-compatible)
-S / --syn       # SYN scan (stealth)
-T / --connect   # TCP Connect scan
-U / --udp       # UDP scan
-A / --ack       # ACK scan
-F / --fin       # FIN scan
-X / --xmas      # Xmas scan
-N / --null       # Null scan
-p / --ports     # Port specification
-t / --timing    # Timing template (T0-T5)
-o / --output    # Output format (table, json, csv, yaml)

# Global flags
-v               # Verbose (-v info, -vv debug, -vvv trace)
-q               # Quiet mode
--log-file       # Log to file
--config         # Override config file path
```

---

## License

Multi-license project: SPECTRE CLI (MIT), WRAITH (MIT), ProRT-IP (GPLv3), CyberChef-MCP (Apache 2.0)
