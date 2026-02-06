# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

---

## Project Overview

**SPECTRE** (Security Platform for Encrypted Comms, Testing, Enumeration, Recon) is a unified offensive security toolkit that orchestrates three standalone components:

| Component | Role | Repository | Version | Tests |
|-----------|------|------------|---------|-------|
| **ProRT-IP** | Network reconnaissance (10M+ pps scanning) | Rust | v1.0.0 | 2,557 |
| **CyberChef-MCP** | Data analysis (463 operations via MCP) | TypeScript/Docker | v1.9.0 | 689 |
| **WRAITH-Protocol** | Secure communications (10+ Gbps E2EE) | Rust | v2.3.7 | 2,957 |

**Phase 1:** Operation BLACKOUT (v0.1.x) - Foundation/CLI skeleton **COMPLETE**
**Phase 2:** Operation NIGHTFALL (v0.2.x) - Core Orchestration **COMPLETE**
**Phase 3:** Operation PHANTOM (v0.3.x) - TUI Dashboard **COMPLETE**
**Phase 4:** Operation SPECTER (v0.4.x) - Advanced Features **COMPLETE**
**Phase 5:** Operation SHADOW (v0.5.x) - GUI Application (Tauri 2.0) **IN PROGRESS** (Sprint 5.6 complete)

**SPECTRE Tests:** 1,175 (44 CLI + 618 core + 74 GUI Rust + 121 GUI frontend + 268 TUI + 5 doc-tests + 45 integration) | **Code:** ~45,000 lines Rust + TypeScript (136 Rust files + 60 frontend files)

**Repository:** [github.com/doublegate/SPECTRE](https://github.com/doublegate/SPECTRE)

---

## Build Commands

```bash
# Build all Rust components (workspace)
cargo build --release --workspace

# Build specific crate
cargo build --release -p spectre-cli

# Build for musl or cross-compilation (vendors OpenSSL from source)
cargo build --release --target x86_64-unknown-linux-musl --features vendored-openssl

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
components/                # Git submodules (standalone component repos)
├── wraith-protocol/       # WRAITH-Protocol v2.3.7 - Secure communications
├── prtip/                 # ProRT-IP v1.0.0 - Network reconnaissance
└── cyberchef-mcp/         # CyberChef-MCP v1.9.0 - Data analysis

crates/
├── spectre-cli/        # Unified CLI orchestrator (18 files, 44 tests)
│   └── src/
│       ├── main.rs         # Entry point, CLI parsing with clap 4
│       ├── commands/       # 13 subcommands: scan, chef, send, receive, identity, peer,
│       │                   # status, config, completions, campaign, pipeline, plugin, tui
│       └── output/         # table (comfy-table) and json (serde_json) formatters
├── spectre-core/       # Core orchestration library (84 files, 618 unit + 45 integration tests)
│   ├── src/
│   │   ├── config/         # TOML config with file discovery and env var support
│   │   ├── scan/           # Scanner trait, ProRT-IP adapter, port/target parsing, 8 scan types
│   │   ├── chef/           # Chef trait, MCP adapter (JSON-RPC 2.0 over stdio), Docker management (bollard)
│   │   ├── comms/          # WRAITH adapter, identity generation/storage, peer management
│   │   ├── target/         # Priority queue, scope enforcer, CIDR expansion, DNS
│   │   ├── job/            # State machine, concurrency control, events
│   │   ├── results/        # Finding model, JSON/XML/greppable output, stats
│   │   ├── pipeline/       # Composable stages, builder API, metrics
│   │   ├── campaign/       # SQLite persistence, phases, artifacts
│   │   ├── plugin/         # Lua 5.4 sandbox, manifest, permissions, registry, hooks, templates
│   │   ├── orchestration/  # Scan chaining, templates, scheduling, profiles, adaptive timing, checkpoint
│   │   ├── workflow/       # YAML/JSON/TOML DSL, parser, async executor, variables, loops
│   │   ├── recipe/         # Storage, import/export, versioning, search, validation, built-in library
│   │   ├── report/         # HTML and Markdown generators, executive summary, risk scoring
│   │   ├── export/         # CSV exporter, custom templates, incremental export tracking
│   │   ├── perf/           # LRU cache, connection pool, performance metrics
│   │   ├── error.rs        # SpectreError enum (20+ variants, thiserror)
│   │   └── logging.rs      # tracing-subscriber with env-filter
│   └── tests/              # Integration tests (45 tests across 6 files)
├── spectre-tui/        # TUI dashboard (18 files, 268 tests)
│   └── src/
│       ├── lib.rs          # Module declarations, re-exports
│       ├── app.rs          # App struct, state management, event dispatch
│       ├── event.rs        # Async EventHandler (crossterm + tick timer)
│       ├── terminal.rs     # Terminal init/restore, panic hook
│       ├── tui.rs          # Main run() entry point
│       ├── layout.rs       # 4-panel layout, Grid/Wide/Tall/Focus modes
│       ├── theme.rs        # 5 built-in themes (dark, light, tactical, matrix, hacker)
│       ├── command.rs      # Command input, 11 commands, history, tab completion
│       ├── keybindings.rs  # Key mapping, vim-style nav, F-keys, AppAction
│       ├── scan_state.rs   # Scan progress tracking, ETA, filtering, sorting
│       ├── panels/         # Recon, Analysis, Comms, Campaign panel renderers
│       └── widgets/        # HelpOverlay, StatusBar shared widgets
├── spectre-gui/        # GUI application — Tauri 2.10 + React 19 + Tailwind CSS 4 + Zustand + D3.js 7 + Recharts 2 (13 Rust files, 74 tests + 60 frontend files, 121 tests)
│   ├── Cargo.toml          # Binary+library crate (staticlib, cdylib, rlib)
│   ├── build.rs            # tauri_build::build()
│   ├── tauri.conf.json     # Tauri config (1280x800 window, CSP, bundle)
│   ├── capabilities/       # IPC security permissions (default.json)
│   ├── icons/              # App icons (RGBA PNG, ico, icns)
│   ├── src/
│   │   ├── main.rs         # Desktop entry point
│   │   ├── lib.rs          # Tauri Builder, plugins, 22 IPC handlers
│   │   ├── state.rs        # AppState (RwLock<Config>, active_scans tracking)
│   │   ├── events.rs       # Event payloads (scan progress/result/complete/error with full host data)
│   │   └── commands/       # IPC command handlers (10 modules)
│   │       ├── status.rs       # get_version, get_status (fully wired)
│   │       ├── scan.rs         # start_scan, stop_scan, get_scan_results, get_active_scans (FULLY WIRED)
│   │       ├── chef.rs         # execute_chef, list_chef_operations, get_chef_status (stub with real encoding - Sprint 5.6)
│   │       ├── comms.rs        # get_identity, list_peers, add_peer, send_data (stub WRAITH - Sprint 5.6)
│   │       ├── campaign.rs     # create/list/get/advance/export/import/archive campaign (FULLY WIRED)
│   │       ├── config.rs       # get_config, update_config, reset_config (FULLY WIRED - Sprint 5.6)
│   │       ├── results.rs      # get_dashboard_stats, get_findings (FULLY WIRED)
│   │       ├── report.rs       # generate_report, export_data (FULLY WIRED)
│   │       └── target.rs       # parse_targets (FULLY WIRED)
│   └── frontend/           # React 19 + Vite 6 + TypeScript + D3.js 7 + Recharts 2
│       ├── package.json, vite.config.ts, tsconfig.json, index.html
│       └── src/
│           ├── components/
│           │   ├── ui/         # 12 shadcn/ui components (card, dialog, badge, button, input, select, table, separator, checkbox, radio-group, label, tabs)
│           │   ├── scan/       # NetworkTopology (D3), ScanConfigForm, ScanProgress, HostCard, ResultsTable
│           │   ├── campaign/   # CampaignCard, ObjectiveList, PhaseTimeline, TargetInput, CreateWizard
│           │   ├── dashboard/  # StatCard, SeverityChart, ServicesChart, ActivityTimeline (Sprint 5.5)
│           │   ├── reports/    # FindingsTable, FindingDetail, ExportPanel, ReportPreview (Sprint 5.5)
│           │   ├── settings/   # 8 settings tabs: General, Scan, Chef, Comms, Output, Theme, Shortcuts, About (Sprint 5.6)
│           │   ├── analysis/   # OperationBrowser, CategoryFilter, TransformPanel (Sprint 5.6)
│           │   └── comms/      # IdentityDisplay, PeerList, SendDataPanel (Sprint 5.6)
│           ├── pages/          # Dashboard (COMPLETE), Recon (COMPLETE), Campaigns (COMPLETE), Reports (COMPLETE), Settings (COMPLETE), Analysis (COMPLETE), Comms (COMPLETE)
│           ├── stores/         # scanStore (multi-scan), campaignStore, dashboardStore, uiStore
│           ├── hooks/          # useScan (event listeners), useCampaign, useDashboard, useIpc
│           └── types/          # scan.ts (complete), campaign.ts (complete), dashboard.ts (complete), config.ts (complete)
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

- **Rust:** `cargo fmt`, `cargo clippy -- -D warnings` (also clean under `-W clippy::pedantic -W clippy::nursery`)
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

- **Rust 1.92+** (WRAITH, ProRT-IP, SPECTRE CLI)
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
| 2 | Operation NIGHTFALL | v0.2.x | Core orchestration - target, job, results, pipeline, campaign, plugins **COMPLETE** |
| 3 | Operation PHANTOM | v0.3.x | TUI dashboard - 4-panel layout, 5 themes, command input, vim-style nav **COMPLETE** |
| 4 | Operation SPECTER | v0.4.x | Advanced features - orchestration, workflows, recipes, reports, exports, perf, plugins **COMPLETE** |
| 5 | Operation SHADOW | v0.5.x | GUI application (Tauri 2.0) — **Sprint 5.1 complete** |
| 6 | Operation WRAITH | v0.6.x | MCP server implementation |
| 7 | Operation GENESIS | v1.0.0 | Production release |

See `to-dos/` directory for detailed sprint planning.

---

## CLI Commands Reference

```bash
# Core commands (Phase 1)
spectre scan [flags] <targets>    # Network scanning (ProRT-IP interface)
spectre chef [operation] [flags]  # Data analysis (CyberChef-MCP interface)
spectre send [flags]              # Secure send (WRAITH interface)
spectre receive [flags]           # Secure receive (WRAITH interface)
spectre identity <subcommand>     # Cryptographic identity management
spectre peer <subcommand>         # Trusted peer management
spectre status                    # Component health checks
spectre config <subcommand>       # Configuration management
spectre completions <shell>       # Shell completion generation
spectre tui                      # Launch TUI dashboard (alias: ui)

# Orchestration commands (Phase 2)
spectre campaign create <name>    # Create a new campaign
spectre campaign status <name>    # Show campaign status
spectre campaign list             # List all campaigns
spectre campaign advance <name>   # Advance campaign to next phase
spectre campaign export <name>    # Export campaign data
spectre campaign import <file>    # Import campaign from file
spectre campaign archive <name>   # Archive completed campaign
spectre pipeline run <name>       # Execute a data pipeline
spectre pipeline list             # List available pipelines
spectre pipeline show <name>      # Show pipeline details
spectre plugin list               # List available plugins
spectre plugin info <name>        # Show plugin information
spectre plugin run <name>         # Execute a plugin

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
