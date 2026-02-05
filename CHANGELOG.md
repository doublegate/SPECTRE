# Changelog

All notable changes to SPECTRE will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- GUI application with Tauri 2.0 — Phase 5
- MCP server implementation — Phase 6

## [0.4.2] - 2026-02-04

### Added

#### TUI Subcommand — CLI-to-TUI Integration

- **`spectre tui` subcommand** (`crates/spectre-cli/src/commands/tui.rs`):
  - New CLI entry point to launch the SPECTRE TUI dashboard directly from the CLI binary
  - Loads SPECTRE configuration via standard `load_config()` pipeline (file discovery, env vars, CLI override)
  - Invokes `spectre_tui::run()` to start the async event loop with terminal initialization
  - Visible alias `ui` for convenience (`spectre ui`)
  - Added `spectre-tui` as a dependency of `spectre-cli` for cross-crate integration

### Changed
- CLI subcommand count: 12 → 13 (added `tui`)
- CLI test count: 43 → 44 (added `test_tui_args_default`)
- CLI source file count: 17 → 18 (added `commands/tui.rs`)
- `spectre-cli/Cargo.toml`: Added `spectre-tui` path dependency
- `spectre-cli/src/commands/mod.rs`: Registered `Tui` variant in `Commands` enum with execution routing

### Technical Details
- 119 Rust source files across 5 crates
- 865 tests total: 44 CLI + 538 core unit + 235 TUI + 3 doc-tests + 45 integration
- Zero clippy warnings at all lint levels (standard, pedantic, nursery)
- No new workspace dependencies — uses existing `spectre-tui` workspace member

## [0.4.1] - 2026-02-04

### Changed

#### Technical Debt Remediation — Comprehensive Code Quality Pass

**Clippy pedantic/nursery compliance** (569 warnings resolved to 0):

- **Blocking I/O fixes**: Replaced 3 `std::fs` calls with `tokio::fs` in async functions (`campaign.rs:183,195`, `pipeline.rs:61`) to prevent executor thread starvation
- **String allocation optimization**: Replaced 98 `format_push_string` instances across `export/csv.rs`, `report/html.rs`, `report/markdown.rs`, and `results/output.rs` with `std::fmt::Write` / `write!()` patterns to eliminate intermediate `String` allocations
- **`use_self` pattern**: Applied `Self::` instead of explicit type names across 108 instances in enum/struct impl blocks throughout spectre-core
- **`const fn` promotion**: Added `const` qualifier to simple getters, constructors, and state-check methods across campaign/state, job/state, orchestration, workflow, recipe, and perf modules
- **Lock scope tightening**: Addressed `significant_drop_tightening` warnings in `job/manager.rs` with targeted `#[allow]` annotations (lock scopes are minimal by design due to async RwLock semantics)
- **Redundant closure elimination**: Simplified 22 closures to method references (e.g., `|s| s.as_str()` → `String::as_str`)
- **Primitive sort optimization**: Replaced 7 `.sort()` calls on `Vec<u16>` with `.sort_unstable()` for better performance
- **Match arm consolidation**: Combined 12 identical match arm bodies using `|` patterns
- **Debug formatting cleanup**: Replaced 11 `{:?}` format specifiers with `{}`/`.display()` where `Display` is available
- **Cast safety annotations**: Added justified `#[allow(clippy::cast_*)]` on 67 instances in statistics/metrics code with explanatory comments
- **Misc pedantic fixes**: `if_not_else` (6), `semicolon_if_nothing_returned` (6), `needless_raw_string_hashes` (2), `default_trait_access` (2), `needless_pass_by_value` (2), `needless_continue` (2), `explicit_iter_loop` (2), `or_fun_call` (2), `trivially_copy_pass_by_ref` (2), `case_sensitive_file_extension_comparisons` (2), `unnested_or_patterns` (2), `needless_collect` (1), `branches_sharing_code` (1), `redundant_clone` (1)

**Documentation coverage**: Added `///` doc comments to previously undocumented public items across CLI arg structs, module re-exports, and core library types

**Property-based testing**: Added `proptest` to dev-dependencies; added property-based tests for `scan/parser.rs` (target/port parsing round-trips) and `target/scope.rs` (scope enforcement invariants)

**CI enhancement**: Added `cargo-tarpaulin` coverage tracking job to `.github/workflows/ci.yml`

### Technical Details
- 91 files changed, +731 insertions, -490 deletions
- 864 tests total (8 new proptest tests): 43 CLI + 538 core unit + 235 TUI + 3 doc-tests + 45 integration
- Zero warnings at all clippy lint levels: standard (`-D warnings`), pedantic (`-W clippy::pedantic`), nursery (`-W clippy::nursery`)
- Zero `cargo doc` warnings with `RUSTDOCFLAGS="-D warnings"`
- New dev-dependency: `proptest` (property-based testing framework)
- No API or behavioral changes — pure code quality improvements

## [0.4.0] - 2026-02-04

### Added

#### Phase 4 Complete: Operation SPECTER — Advanced Features

**6 new modules** (43 new source files) and **3 plugin extensions** in `spectre-core`:

- **Scan Orchestration** (`orchestration/`, 7 files):
  - `ScanChain` and `ScanChainBuilder` for chaining multiple scans with step conditions
  - `StepCondition` enum (Always, PortOpen, ServiceDetected, HostUp, Custom) for conditional scan execution
  - `ScanTemplate` with built-in library: quick-recon, full-audit, stealth-enum, web-focused, infrastructure
  - `ScanSchedule` with cron-like expression parsing (minute/hour/day-of-month/month/day-of-week)
  - `ScanProfile` and `ScanProfileStore` for user-defined scan configurations
  - `AdaptiveTiming` with loss-rate monitoring and automatic timing template adjustment
  - `Checkpoint` system for persisting and resuming interrupted scans

- **Workflow Automation** (`workflow/`, 7 files):
  - `WorkflowDefinition` DSL with steps, variables, conditionals, loops, retry logic
  - `WorkflowParser` supporting YAML, JSON, and TOML input formats
  - Async `WorkflowExecutor` with step-by-step execution, variable capture, progress tracking
  - `VariableStore` with string interpolation and `{{variable}}` template substitution
  - `StepCondition` evaluation against scan results for conditional branching
  - Built-in workflow templates: recon-to-report, vuln-scan-chain, full-campaign
  - `WorkflowPersistence` for save/load/list/delete of workflow definitions

- **Recipe Management** (`recipe/`, 6 files):
  - `Recipe` struct with operations, metadata, versioning, and tags
  - `RecipeStorage` for file-based recipe persistence with JSON serialization
  - `RecipeFormat` supporting JSON, YAML, and TOML import/export
  - `RecipeSearch` with name matching, tag filtering, and operation type queries
  - `RecipeValidator` with structural, semantic, and operation name validation
  - Built-in recipe library: base64-decode, hex-to-ascii, url-decode, hash-identify, extract-urls, defang-urls

- **Report Generation** (`report/`, 5 files):
  - `ReportData` with `from_results()` for automatic summary, risk scoring, and findings aggregation
  - `ReportTemplate` with configurable title, author, sections, and CSS customization
  - `HtmlReportGenerator` producing standalone HTML with embedded CSS, severity color-coding, sortable tables
  - `MarkdownReportGenerator` with structured headers, findings tables, and statistics sections
  - `ExecutiveSummary` with risk score calculation (0-100), risk rating (Critical/High/Medium/Low), severity breakdown, top affected hosts

- **Export Formats** (`export/`, 5 files):
  - `CsvExporter` with three export modes: scan results, host summary, and findings
  - `ExportTemplate` engine with `{{variable}}` substitution for custom export formats
  - `ExportScheduler` for periodic automated exports with configurable intervals
  - `IncrementalExport` tracking export state (last timestamp, count, hash) for delta exports
  - State persistence for incremental export checkpoints

- **Performance Optimization** (`perf/`, 4 files):
  - Generic `LruCache<K, V>` with configurable capacity and O(1) get/insert via HashMap + VecDeque
  - `ConnectionPool<T>` with async acquire/release, configurable max connections, idle limits, and timeouts
  - `PerfMetrics` with async-safe timing stats (min/avg/max/p95/p99) and counter operations
  - `MetricStats` computation with percentile calculation

- **Advanced Plugin System** (`plugin/` extensions, 3 new files):
  - `PluginRegistry` with register/unregister/search-by-tag/search-by-name, dependency resolution via DFS (circular dependency detection), semver version comparison, JSON persistence
  - `HookManager` with 10 lifecycle events (PreScan, PostScan, PreAnalysis, PostAnalysis, PreReport, PostReport, PreExport, PostExport, OnError, OnComplete), priority-ordered hook execution, per-plugin enable/disable
  - `PluginTemplateGenerator` for scaffolding 5 plugin types (Basic, Scanner, Report, Workflow, Analysis) with generated `plugin.toml` manifest and type-specific `init.lua` script

- **Integration Tests** (6 test files, 45 tests):
  - `integration_orchestration.rs` (9 tests): Scan chain builder, template library, scheduling, profile store, checkpoint save/load, adaptive timing, step conditions
  - `integration_workflow.rs` (9 tests): YAML/JSON parsing, execution, variable propagation, conditional skipping, loop execution, template execution, persistence, variable substitution
  - `integration_report.rs` (7 tests): Executive summary, HTML pipeline, Markdown pipeline, empty data, template sections, risk scoring, top affected hosts
  - `integration_export.rs` (8 tests): CSV results/hosts/findings, custom template rendering, incremental export flow, state persistence, reset, variable substitution
  - `integration_plugin.rs` (6 tests): Registry lifecycle, persistence, hook system, hook context data flow, template generation, version comparison
  - `integration_perf.rs` (6 tests): LRU cache, async metrics, connection pool, pool exhaustion, multiple metrics

### Changed
- Workspace version: 0.3.0 -> 0.4.0
- `spectre-core/src/lib.rs`: Added 6 new module declarations (orchestration, workflow, recipe, report, export, perf) — now 18 public modules
- `spectre-core/src/error.rs`: Added 5 new error variants (OrchestrationError, WorkflowError, RecipeError, ReportError, ExportError) — now 20+ total
- `spectre-core/src/plugin/mod.rs`: Added 3 new submodules (registry, hooks, template) with public re-exports

### Technical Details
- 118 Rust source files, ~31,000 lines of code
- 856 tests total: 43 (spectre-cli) + 530 (spectre-core unit) + 235 (spectre-tui) + 3 (spectre-mcp doc-tests) + 45 (integration tests)
- Zero clippy warnings with `-D warnings`
- All formatting passes `cargo fmt --all --check`
- New module test breakdown: orchestration (49), workflow (40), recipe (47), report (30), export (38), perf (31), plugin/registry (18), plugin/hooks (12), plugin/template (10), integration (45)
- No new workspace dependencies — all new modules built using existing deps (serde, tokio, chrono, etc.)
- Minimum supported Rust version: 1.88

## [0.3.0] - 2026-02-04

### Added

#### Phase 3 Complete: Operation PHANTOM — TUI Dashboard

**spectre-tui crate** (18 source files, 235 tests):

- **Core TUI framework** (`app.rs`, `event.rs`, `terminal.rs`, `tui.rs`):
  - `App` struct with centralized state management and event dispatch
  - Async `EventHandler` using tokio tasks for crossterm polling with configurable tick rate
  - Terminal initialization/restoration with panic hook for safe cleanup
  - Main `run()` entry point with async event loop targeting 60 FPS

- **Layout and panels** (`layout.rs`, `panels/`):
  - `PanelManager` with 4 layout modes: Grid (2x2), Wide (side-by-side), Tall (stacked), Focus (single maximized)
  - `PanelId` enum: Recon, Analysis, Comms, Campaign
  - `Panel` trait with `render()` and `on_key()` for consistent panel behavior
  - Header bar with SPECTRE title, campaign name, and help shortcut
  - Status bar with quick action shortcuts and system status indicators

- **Recon panel** (`panels/recon.rs`, `scan_state.rs`):
  - Real-time scan progress display with ratatui `Gauge` widget
  - Scan metrics: target, scan type, progress %, rate (pps), hosts scanned/total, open ports, services
  - ETA calculation based on elapsed time and completion percentage
  - Scrollable results table with port, state, service, and version columns
  - Port state color coding (green=open, red=closed, yellow=filtered)
  - Integration with `spectre_core::job::JobEvent` for live updates
  - Result filtering by IP address, port number, service name
  - Result sorting by IP, open port count, scan time

- **Analysis panel** (`panels/analysis.rs`):
  - CyberChef recipe status display with progress bar
  - Output preview with scrollable text area
  - Recipe name, input source, processing speed, and ETA display

- **Comms panel** (`panels/comms.rs`):
  - Peer connection table with protocol, status, and TX/RX bandwidth
  - Transfer queue display with direction (upload/download), progress, and status indicators
  - Connection status summary (identity, peer count, online status)

- **Campaign panel** (`panels/campaign.rs`):
  - Campaign phase timeline visualization using `CampaignPhase` state machine
  - Phase progress indicators (completed/active/pending markers)
  - Campaign metrics: name, status, duration, phase, objectives
  - Timeline event log with timestamps

- **Command input system** (`command.rs`):
  - 11 TUI commands: `scan`, `chef`, `send`, `campaign`, `set`, `theme`, `export`, `clear`, `layout`, `help`, `quit`
  - Command parsing with argument extraction
  - Command history with up/down arrow navigation
  - Tab completion for command names
  - Cursor movement (Home/End, left/right, Backspace/Delete)
  - Error display for invalid commands with timeout auto-clear

- **Keyboard shortcuts** (`keybindings.rs`):
  - Global shortcuts: F1 (help), F2-F5 (panel focus), F10/q (quit), Tab/Shift+Tab (cycle panels)
  - Vim-style navigation: j/k (up/down), h/l (left/right), g/G (top/bottom), Ctrl+d/u (page down/up)
  - Command mode: `:` to enter, Esc to cancel, Enter to execute
  - Command palette: `/` for search/fuzzy command access
  - Panel-specific shortcuts routed based on focused panel
  - `AppAction` enum for decoupled key-to-action mapping

- **Theme system** (`theme.rs`):
  - `Theme` struct with 12 configurable colors (bg, fg, primary, secondary, accent, success, warning, error, muted, border, border_focused, highlight)
  - 5 built-in themes:
    - `dark` (default): Dark background with green accents
    - `light`: Light background with dark text and blue accents
    - `tactical`: Military-style green on black
    - `matrix`: Bright green text on black (Matrix-inspired)
    - `hacker`: Amber text on dark background
  - Runtime theme switching via `:theme <name>` or `:set theme <name>`
  - Style builder methods for consistent widget styling

- **Shared widgets** (`widgets/`):
  - `HelpOverlay`: Centered modal with keyboard shortcuts grouped by category (Navigation, Panels, Commands, Within Panels)
  - `StatusBar`: Header rendering (title + campaign) and footer rendering (shortcuts or command input)

### Changed
- Workspace version: 0.2.0 -> 0.3.0
- spectre-tui: Replaced placeholder `lib.rs` with 12 module declarations and public API re-exports
- spectre-tui `Cargo.toml`: Added `chrono` workspace dependency

### Technical Details
- 75 Rust source files, ~20,800 lines of code
- 505 tests total: 43 (spectre-cli) + 224 (spectre-core) + 235 (spectre-tui) + 3 (doc-tests)
- Zero clippy warnings with `-D warnings`
- All formatting passes `cargo fmt --all --check`
- TUI test breakdown: app (28), layout (30), theme (25), command (52), keybindings (18), scan_state (32), panels/recon (15), panels/analysis (10), panels/comms (10), panels/campaign (10), widgets (5)
- Rendering tests use `ratatui::backend::TestBackend` for headless verification
- Async runtime: tokio (full features)
- Minimum supported Rust version: 1.88

## [0.2.0] - 2026-02-04

### Added

#### Phase 2 Complete: Operation NIGHTFALL — Core Orchestration

**spectre-cli crate** (17 source files, 43 tests):
- 3 new subcommands (12 total): `campaign`, `pipeline`, `plugin`
- `campaign` command with 7 subcommands: `create`, `status`, `list`, `advance`, `export`, `import`, `archive`
- `pipeline` command with 3 subcommands: `run`, `list`, `show`
- `plugin` command with 3 subcommands: `list`, `info`, `run`

**spectre-core crate** (38 source files, 224 tests, 3 doc-tests):
- **Target management** (`target/`, 40 tests):
  - `EnhancedTarget` struct with priority scoring, status tracking, metadata
  - `TargetQueue` with BinaryHeap-based priority ordering
  - `ScopeEnforcer` with allow/block lists, CIDR range support
  - Target file parser supporting line-delimited, CSV, and Nmap-format inputs
  - CIDR expansion to individual addresses
  - Async DNS resolution with configurable concurrency
  - Target deduplication and validation

- **Job orchestration** (`job/`, 35 tests):
  - `ScanJob` struct with full state machine: Created → Queued → Running → Paused → Complete/Failed/Cancelled
  - `JobManager` with configurable concurrency limits
  - `CancellationToken`-based job cancellation
  - Event broadcasting via tokio broadcast channels
  - Job progress tracking with percentage and ETA estimation
  - Job persistence and recovery support

- **Results aggregation** (`results/`, 23 tests):
  - `Finding` struct with severity levels (Critical, High, Medium, Low, Info)
  - Host-centric result grouping with service discovery
  - JSON output format with pretty-printing
  - Nmap-compatible XML output generation (quick-xml)
  - Greppable output format for scripting
  - `ResultStats` with port distribution, service summaries, OS statistics
  - Finding deduplication and merging

- **Data pipeline** (`pipeline/`, 17 tests):
  - Composable pipeline stages: Scan → Analysis → Filter → Output
  - `PipelineBuilder` fluent API for stage composition
  - Async pipeline execution with stage-level error handling
  - Pipeline execution metrics (duration per stage, total throughput)
  - Named pipeline definitions for reuse
  - Stage-level progress callbacks

- **Campaign management** (`campaign/`, 28 tests):
  - `Campaign` struct with multi-phase lifecycle
  - `CampaignPhase` state machine: Planning → Recon → Analysis → Exploitation → PostExploitation → Reporting → Complete
  - `Artifact` struct with SHA-256 hash verification
  - `CampaignStore` with SQLite persistence (rusqlite, bundled)
  - Campaign CRUD operations with SQL schema management
  - Phase advancement with validation rules
  - Campaign export/import for sharing between operators
  - Campaign archival with metadata preservation

- **Plugin system** (`plugin/`, 30 tests):
  - Lua 5.4 sandbox via mlua (vendored, async, send, serialize)
  - `PluginManifest` from `plugin.toml` with metadata, permissions, dependencies
  - Sandboxed environment with restricted standard library access
  - `spectre.*` Lua API: `spectre.log()`, `spectre.scan()`, `spectre.chef()`, `spectre.send()`
  - Permission model: network, filesystem, exec, with configurable grants
  - Resource limits: memory ceiling, execution timeout, output buffering
  - Plugin discovery from configured plugin directories

- **Error handling** (`error.rs`):
  - 5 new error variants: `Target(TargetError)`, `Job(JobError)`, `Pipeline(PipelineError)`, `Campaign(CampaignError)`, `Plugin(PluginError)`
  - Each new domain has specialized sub-error enum with thiserror derive

**Workspace dependencies added** (`Cargo.toml`):
- rusqlite (bundled): SQLite for campaign persistence
- mlua (lua54, vendored, async, send, serialize): Lua 5.4 plugin runtime
- uuid (v4): Unique identifiers for jobs and campaigns
- tokio-util: Cancellation tokens for job management
- quick-xml: Nmap-compatible XML output generation

### Changed
- Extended `SpectreError` enum from 8 to 13 variants covering all new domains
- Updated `lib.rs` from 6 to 12 public modules
- CLI `commands/mod.rs` updated with 3 new subcommand routes (12 total)

### Technical Details
- 58 Rust source files, ~14,500 lines of code
- 270 tests total: 43 (spectre-cli) + 224 (spectre-core) + 3 (doc-tests)
- Zero clippy warnings with `-D warnings`
- All formatting passes `cargo fmt --all --check`
- Core module test breakdown: target (40), job (35), plugin (30), campaign (28), results (23), scan (19), pipeline (17), config (11), comms (9), chef (7), error (4), logging (1)
- Async runtime: tokio (full features)
- Minimum supported Rust version: 1.88

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
| v0.4.0 | **Operation SPECTER** | Advanced features, workflows, reporting |
| v0.5.0 | **Operation SHADOW** | Visual campaign planning, collaboration |
| v1.0.0 | **Operation GENESIS** | Production release - full platform capability |

---

[Unreleased]: https://github.com/doublegate/SPECTRE/compare/v0.4.2...HEAD
[0.4.2]: https://github.com/doublegate/SPECTRE/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/doublegate/SPECTRE/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/doublegate/SPECTRE/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/doublegate/SPECTRE/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/doublegate/SPECTRE/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/doublegate/SPECTRE/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/doublegate/SPECTRE/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/doublegate/SPECTRE/releases/tag/v0.1.0
