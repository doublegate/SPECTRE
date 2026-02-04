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

**Current Phase:** Operation BLACKOUT (v0.1.0) - Foundation/CLI skeleton

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
├── spectre-cli/        # Unified CLI orchestrator
├── spectre-core/       # Core orchestration library
├── spectre-tui/        # TUI dashboard (ratatui)
├── spectre-gui/        # GUI application (Tauri 2.0)
└── spectre-mcp/        # MCP server implementation
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
└── briefings/          # Mission briefing templates
    ├── OPORD-template.md
    ├── SITREP.md
    ├── CONOP-template.md
    └── AAR-template.md
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

---

## License

Multi-license project: SPECTRE CLI (MIT), WRAITH (MIT), ProRT-IP (GPLv3), CyberChef-MCP (Apache 2.0)
