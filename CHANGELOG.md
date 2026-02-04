# Changelog

All notable changes to SPECTRE will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Data pipeline between components (JSON/Protobuf) — Phase 2
- Scan-to-analysis automation workflows — Phase 2
- TUI dashboard with real-time visualization — Phase 3

## [0.1.2] - 2026-02-04

### Added

#### Phase 1 Complete: Operation BLACKOUT — CLI Foundation

**spectre-cli crate** (14 source files, 34 tests):
- Unified CLI with clap 4 derive-based argument parsing
- 9 subcommands: `scan`, `chef`, `send`, `receive`, `identity`, `peer`, `status`, `config`, `completions`
- `scan` command with Nmap-compatible flags: `-S` (SYN), `-T` (Connect), `-U` (UDP), `-A` (ACK), `-F` (FIN), `-X` (Xmas), `-N` (Null), `--scan-type window`
- Port specification parsing: individual ports, ranges, comma-separated, named sets (`common`, `top100`, `all`)
- Target parsing: IPv4, IPv6, CIDR notation, hostname, IP ranges
- Timing templates T0-T5 (paranoid through insane)
- `chef` command with operation execution, recipe support, Docker health checks, and setup subcommand
- `send`/`receive` commands with WRAITH peer addressing and encryption options
- `identity` subcommands: `init` (key generation), `show` (display), `list`, `delete`, `export`
- `peer` subcommands: `add`, `remove`, `list`, `show`, `verify`, `import`
- `status` command with component health checks (ProRT-IP, CyberChef, WRAITH, Config)
- `config` subcommands: `init` (create default), `show` (display effective config), `check` (validate), `edit`, `reset`
- `completions` command generating shell completions for bash, zsh, fish, PowerShell, elvish
- Global flags: `--verbose` (-v, -vv, -vvv), `--quiet`, `--log-file`, `--config`
- Output formatting: table (comfy-table) and JSON (serde_json) formatters

**spectre-core crate** (15 source files, 51 tests, 1 doc-test):
- **Configuration system** (`config/`):
  - `SpectreConfig` struct with nested sections: general, scan, chef, comms, output
  - Multi-source config file discovery: system (`/etc/spectre/`), user (`~/.config/spectre/`), project (`./spectre.toml`)
  - Config merging with precedence: CLI args > env vars > project > user > system
  - Environment variable support (`SPECTRE_*` prefix)
  - TOML serialization/deserialization with serde
  - `config init` generates annotated default config file
  - `config show` displays effective merged configuration
  - `config check` validates configuration correctness
  - Platform-aware directory resolution via `directories` crate

- **Scanning interface** (`scan/`):
  - `Scanner` async trait for pluggable scan engine implementations
  - `StubScanner` implementation for development/testing (to be replaced with real ProRT-IP)
  - 8 scan types: SYN, Connect, UDP, ACK, FIN, Xmas, Null, Window
  - Rich type system: `ScanType`, `ScanResult`, `HostResult`, `PortResult`, `PortState`, `ServiceInfo`
  - Port parser supporting ranges (`1-1000`), lists (`22,80,443`), named sets, and mixed specifications
  - Target parser with IPv4, IPv6, CIDR, hostname, and range notation support
  - Timing templates (T0-T5) mapped to rate limits and timeout values

- **CyberChef integration** (`chef/`):
  - `Chef` async trait for pluggable analysis backends
  - `StubChef` implementation for development/testing
  - `McpClient` for future MCP protocol communication
  - `DockerManager` using bollard crate for container lifecycle management
  - Container operations: start, stop, health check, status, auto-start
  - Recipe execution support (JSON format with chained operations)

- **WRAITH comms interface** (`comms/`):
  - `Identity` struct with keypair generation, display name, creation timestamp
  - SHA-256 based identity fingerprint generation
  - Identity persistence to disk (JSON serialization in data directory)
  - Identity listing and lookup by name or fingerprint prefix
  - `Peer` struct with trust verification status
  - Peer management: add, remove, list, verify, import
  - Peer persistence with JSON storage

- **Error handling** (`error.rs`):
  - `SpectreError` enum with thiserror derive covering: Config, Scan, Chef, Comms, Io, Parse, Docker, Timeout
  - Contextual error messages with source chaining
  - Display implementations for user-friendly output

- **Structured logging** (`logging.rs`):
  - tracing-subscriber initialization with env-filter
  - RUST_LOG environment variable support
  - Optional file-based log output via tracing-appender
  - Verbosity level mapping: 0=warn, 1=info, 2=debug, 3=trace

**Configuration file** (`configs/spectre.toml`):
- Annotated default configuration with all sections documented
- Sections: general, scan, chef, comms, output
- Inline documentation of all configuration options and defaults

**Workspace dependencies updated** (`Cargo.toml`):
- Added: clap_complete, serde_yaml, toml_edit, tracing-appender, ipnetwork, sha2, base64, hex, urlencoding, chrono, directories, bollard, colored, comfy-table, indicatif, tempfile
- Updated: clap (added color, suggestions features), tracing-subscriber (added json feature)
- Removed unused: pcap, pnet, chacha20poly1305, x25519-dalek, mcp-sdk

### Changed
- Cleaned `rustfmt.toml` to stable-channel-only options (removed 15 nightly-only settings)
- Updated Cargo.toml workspace dependencies to match actual implementation needs

### Technical Details
- 32 Rust source files, ~7,600 lines of code
- 86 tests total: 34 (spectre-cli) + 51 (spectre-core) + 1 (doc-test)
- Zero clippy warnings with `-D warnings`
- All formatting passes `cargo fmt --all --check`
- Async runtime: tokio (full features)
- Minimum supported Rust version: 1.88

## [0.1.1] - 2026-02-04

### Added

#### GitHub Repository Infrastructure
- **GitHub Issue Templates**
  - `bug_report.md` - Structured bug reporting with environment details
  - `feature_request.md` - Feature proposal with use case requirements
  - `config.yml` - Issue template chooser with external links

- **Pull Request Template** - Comprehensive PR checklist with security review

- **GitHub Workflows (CI/CD)**
  - `ci.yml` - Multi-platform CI (Linux, macOS, Windows) with format, lint, test, docs, audit, MSRV, and coverage jobs
  - `release.yml` - Automated release workflow with cross-compilation for 6 platform targets (x86_64/aarch64 for Linux, macOS, Windows)

- **Repository Management**
  - `CODEOWNERS` - Code ownership for review assignment
  - `FUNDING.yml` - GitHub Sponsors configuration
  - `dependabot.yml` - Automated dependency updates for Rust, npm, Docker, GitHub Actions

- **Community Files**
  - `CONTRIBUTING.md` - Comprehensive contribution guidelines with development workflow
  - `SECURITY.md` - Security policy with vulnerability reporting procedures

#### Project Configuration
- **Cargo Workspace** (`Cargo.toml`)
  - Workspace configuration with 5 crates
  - Shared workspace dependencies
  - Optimized release profiles (LTO, codegen-units)
  - MSRV 1.88

- **Code Quality Tools**
  - `rustfmt.toml` - Rust formatting configuration
  - `clippy.toml` - Clippy linting configuration
  - `.editorconfig` - Cross-editor formatting standards

#### Documentation (45+ files)

- **Development Documentation** (`docs/development/`)
  - `SETUP.md` - Development environment setup guide
  - `ARCHITECTURE.md` - Internal architecture with component diagrams
  - `TESTING.md` - Testing strategy and best practices
  - `DEBUGGING.md` - Debugging techniques and tools

- **API Documentation** (`docs/api/`)
  - `REST-API.md` - REST API specification for GUI backend
  - `MCP-PROTOCOL.md` - MCP protocol implementation details
  - `PLUGIN-API.md` - Lua plugin development guide

- **Security Documentation** (`docs/security/`)
  - `THREAT-MODEL.md` - Comprehensive threat model with mitigations
  - `ENCRYPTION.md` - Cryptographic implementations and protocols
  - `OPERATIONAL-SECURITY.md` - OpSec best practices

- **Deployment Documentation** (`docs/deployment/`)
  - `INSTALLATION.md` - Installation guide for all platforms
  - `CONFIGURATION.md` - Configuration reference with examples
  - `DOCKER.md` - Docker deployment and orchestration

- **Tutorial Documentation** (`docs/tutorials/`)
  - `FIRST-SCAN.md` - Getting started with network scanning
  - `SECURE-CHANNEL.md` - Setting up encrypted communications
  - `DATA-ANALYSIS.md` - Data analysis workflows with CyberChef
  - `CAMPAIGN-PLANNING.md` - Campaign orchestration guide

- **Reference Documentation** (`docs/reference/`)
  - `GLOSSARY.md` - Security terminology and acronyms
  - `FAQ.md` - Frequently asked questions
  - `TROUBLESHOOTING.md` - Common issues and solutions

#### Sprint Planning (`to-dos/`)
- `README.md` - Sprint planning overview and status
- `ROADMAP.md` - Product roadmap with timeline
- **7 Phase Planning Documents:**
  - `PHASE-1-FOUNDATION.md` - Operation BLACKOUT (v0.1.x) - 8 sprints
  - `PHASE-2-INTEGRATION.md` - Operation NIGHTFALL (v0.2.x) - 8 sprints
  - `PHASE-3-TUI.md` - Operation PHANTOM (v0.3.x) - 8 sprints
  - `PHASE-4-ADVANCED.md` - Operation SPECTER (v0.4.x) - 8 sprints
  - `PHASE-5-GUI.md` - Operation SHADOW (v0.5.x) - 8 sprints
  - `PHASE-6-MCP.md` - Operation WRAITH (v0.6.x) - 8 sprints
  - `PHASE-7-RELEASE.md` - Operation GENESIS (v1.0.0) - 8 sprints

### Changed
- Updated CLAUDE.md with complete documentation structure
- Enhanced .gitignore with comprehensive coverage

## [0.1.0] - 2026-02-04

### Added

#### Platform Foundation
- Initial SPECTRE platform architecture documentation
- Unified offensive security platform design integrating three components
- Four interface modes specification: CLI, TUI, GUI, MCP Server
- Campaign orchestration framework design

#### Integrated Components
- **WRAITH-Protocol v2.3.7** integration specification
  - Wire-speed E2EE communications (10+ Gbps with AF_XDP)
  - XChaCha20-Poly1305, Noise_XX, Double Ratchet protocols
  - Protocol mimicry (TLS 1.3, WebSocket, DNS-over-HTTPS)
  - Post-quantum hybrid X25519 + ML-KEM-768
  - 12 client applications including RedOps C2
  - 2,957 tests

- **ProRT-IP WarScan v1.0.0** integration specification
  - High-performance network scanning (10M+ pps)
  - 8 scan types: SYN, Connect, FIN, NULL, Xmas, ACK, UDP, Idle
  - Service detection with 1000+ signatures
  - OS fingerprinting via TCP/IP stack analysis
  - Lua 5.4 plugin extensibility
  - 60 FPS TUI framework
  - 2,557 tests

- **CyberChef-MCP v1.8.0** integration specification
  - 463+ data manipulation operations via MCP
  - Recipe management with CRUD operations
  - Batch processing for parallel execution
  - Magic detection for auto-format identification
  - 563 tests

#### Documentation
- Architecture documentation
  - `docs/architecture/SYSTEM-DESIGN.md` - Platform architecture
  - `docs/architecture/INTEGRATION-SPEC.md` - Component integration
  - `docs/architecture/INTERFACE-MODES.md` - CLI/TUI/GUI/MCP specifications

- User guides
  - `docs/user-guide/QUICK-START.md` - Getting started guide
  - `docs/user-guide/CLI-REFERENCE.md` - Command reference
  - `docs/user-guide/TUI-GUIDE.md` - Terminal UI guide
  - `docs/user-guide/MCP-TOOLS.md` - MCP tool reference

- Integration guides
  - `docs/integration/WRAITH-INTEGRATION.md` - WRAITH integration details
  - `docs/integration/PRTIP-INTEGRATION.md` - ProRT-IP integration details
  - `docs/integration/CYBERCHEF-INTEGRATION.md` - CyberChef integration details

- Mission briefing templates
  - `docs/briefings/OPORD-template.md` - Operations order format
  - `docs/briefings/SITREP.md` - Situation report format
  - `docs/briefings/CONOP-template.md` - Concept of operations format
  - `docs/briefings/AAR-template.md` - After action review format

#### Project Structure
- Cargo workspace configuration
- Crate scaffolding for spectre-cli, spectre-core, spectre-tui, spectre-gui, spectre-mcp
- Configuration templates in `configs/`
- Workflow templates in `templates/`
- Test structure in `tests/`

#### Development
- CLAUDE.md for AI assistant guidance
- README.md with comprehensive platform documentation
- Multi-license structure (MIT/GPLv3/Apache-2.0)

### Technical Specifications
- Combined test suite: 6,077 tests (WRAITH: 2,957 + ProRT-IP: 2,557 + CyberChef: 563)
- Estimated codebase: ~220,000 lines (180,000 Rust + 40,000 TypeScript)
- Language: Rust 2024 edition, TypeScript
- Build system: Cargo workspace

### Security
- All communications encrypted with XChaCha20-Poly1305
- Perfect forward secrecy via Double Ratchet
- Traffic analysis resistance with Elligator2 and protocol mimicry
- No telemetry or phone-home functionality
- Post-quantum hybrid encryption available

---

## Release Codenames

| Version | Codename | Focus |
|---------|----------|-------|
| v0.1.0 | **Operation BLACKOUT** | Foundation - CLI skeleton, component integration |
| v0.2.0 | **Operation NIGHTFALL** | Data pipeline, scan-to-analysis automation |
| v0.3.0 | **Operation PHANTOM** | Campaign orchestration, multi-target coordination |
| v0.4.0 | **Operation ECLIPSE** | AI-assisted targeting, threat intel integration |
| v0.5.0 | **Operation SHADOW** | Visual campaign planning, collaboration |
| v1.0.0 | **Operation GENESIS** | Production release - full platform capability |

---

[Unreleased]: https://github.com/doublegate/SPECTRE/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/doublegate/SPECTRE/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/doublegate/SPECTRE/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/doublegate/SPECTRE/releases/tag/v0.1.0
