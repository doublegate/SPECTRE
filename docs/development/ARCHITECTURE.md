# Codebase Architecture

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## Overview

SPECTRE is organized as a Cargo workspace with multiple crates, each with a specific responsibility. This document describes the internal architecture and how components interact.

---

## Workspace Structure

```
SPECTRE/
├── Cargo.toml              # Workspace manifest
├── crates/
│   ├── spectre-cli/        # CLI binary
│   ├── spectre-core/       # Core orchestration library
│   ├── spectre-tui/        # TUI dashboard
│   ├── spectre-gui/        # GUI application (Tauri)
│   └── spectre-mcp/        # MCP server
├── configs/                # Configuration files
├── docs/                   # Documentation
├── scripts/                # Build and utility scripts
├── templates/              # Output templates
└── tests/                  # Integration tests
```

---

## Crate Dependencies

```
                    ┌─────────────┐
                    │ spectre-cli │
                    └──────┬──────┘
                           │
         ┌─────────────────┼─────────────────┐
         │                 │                 │
         ▼                 ▼                 ▼
┌────────────────┐ ┌──────────────┐ ┌──────────────┐
│  spectre-tui   │ │ spectre-gui  │ │ spectre-mcp  │
└────────┬───────┘ └──────┬───────┘ └──────┬───────┘
         │                │                 │
         └────────────────┼─────────────────┘
                          │
                          ▼
                 ┌────────────────┐
                 │  spectre-core  │
                 └────────────────┘
```

---

## Crate Details

### spectre-cli

**Binary crate** - Main entry point for CLI usage.

```
spectre-cli/
├── src/
│   ├── main.rs             # Entry point
│   ├── cli.rs              # Argument parsing (clap)
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── scan.rs         # Scan commands
│   │   ├── chef.rs         # CyberChef commands
│   │   ├── send.rs         # WRAITH send
│   │   ├── receive.rs      # WRAITH receive
│   │   ├── campaign.rs     # Campaign management
│   │   └── status.rs       # Status display
│   └── output/
│       ├── mod.rs
│       ├── json.rs         # JSON formatter
│       ├── table.rs        # Table formatter
│       └── greppable.rs    # Greppable formatter
└── Cargo.toml
```

**Dependencies:**
- `spectre-core` - Core functionality
- `clap` - Argument parsing
- `tokio` - Async runtime

### spectre-core

**Library crate** - Core orchestration and integration logic.

```
spectre-core/
├── src/
│   ├── lib.rs              # Public API
│   ├── config/
│   │   ├── mod.rs
│   │   └── settings.rs     # Configuration management
│   ├── scan/
│   │   ├── mod.rs
│   │   ├── manager.rs      # Scan job management
│   │   ├── prtip.rs        # ProRT-IP integration
│   │   └── results.rs      # Result types
│   ├── chef/
│   │   ├── mod.rs
│   │   ├── client.rs       # MCP client
│   │   ├── operations.rs   # Operation wrappers
│   │   └── recipes.rs      # Recipe management
│   ├── comms/
│   │   ├── mod.rs
│   │   ├── wraith.rs       # WRAITH integration
│   │   ├── channel.rs      # Secure channels
│   │   └── c2.rs           # C2 functionality
│   ├── campaign/
│   │   ├── mod.rs
│   │   ├── state.rs        # Campaign state
│   │   └── workflow.rs     # Workflow engine
│   ├── models/
│   │   ├── mod.rs
│   │   ├── target.rs       # Target definitions
│   │   ├── finding.rs      # Findings/results
│   │   └── artifact.rs     # Data artifacts
│   └── plugin/
│       ├── mod.rs
│       ├── lua.rs          # Lua VM
│       └── sandbox.rs      # Sandboxing
└── Cargo.toml
```

**Key Responsibilities:**
- Configuration management
- Component orchestration
- Data pipeline between components
- Campaign state management
- Plugin system

### spectre-tui

**Library crate** - Terminal UI dashboard.

```
spectre-tui/
├── src/
│   ├── lib.rs
│   ├── app.rs              # Application state
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── layout.rs       # Panel layout
│   │   ├── panels/
│   │   │   ├── targets.rs
│   │   │   ├── results.rs
│   │   │   ├── activity.rs
│   │   │   └── status.rs
│   │   └── widgets/
│   │       ├── progress.rs
│   │       └── chart.rs
│   ├── event.rs            # Event handling
│   ├── input.rs            # Keyboard input
│   └── theme.rs            # Color themes
└── Cargo.toml
```

**Key Features:**
- 60 FPS rendering
- 4-panel layout
- Real-time updates
- Keyboard navigation
- Themeable

### spectre-gui

**Binary crate** - Tauri 2.0 desktop application.

```
spectre-gui/
├── src/
│   ├── main.rs             # Tauri entry
│   ├── commands.rs         # IPC commands
│   └── state.rs            # App state
├── src-tauri/
│   ├── Cargo.toml
│   └── tauri.conf.json
└── ui/                     # React frontend
    ├── src/
    │   ├── App.tsx
    │   ├── components/
    │   ├── pages/
    │   └── hooks/
    ├── package.json
    └── vite.config.ts
```

### spectre-mcp

**Binary crate** - MCP server for AI assistant integration.

```
spectre-mcp/
├── src/
│   ├── main.rs
│   ├── server.rs           # MCP server
│   ├── tools/
│   │   ├── mod.rs
│   │   ├── scan.rs         # Scan tools
│   │   ├── chef.rs         # Chef tools
│   │   └── comms.rs        # Comms tools
│   ├── resources.rs        # Resource definitions
│   └── prompts.rs          # Prompt templates
└── Cargo.toml
```

---

## Design Decisions

### Async-First Architecture

All I/O operations use `tokio` async runtime:

```rust
// Good: Async operation
pub async fn scan_targets(&self, targets: &[Target]) -> Result<Vec<Finding>> {
    let futures: Vec<_> = targets.iter()
        .map(|t| self.scan_single(t))
        .collect();

    futures::future::join_all(futures).await
        .into_iter()
        .collect()
}

// Avoid: Blocking in async context
pub async fn bad_example(&self) -> Result<Data> {
    std::thread::sleep(Duration::from_secs(1)); // DON'T DO THIS
    Ok(data)
}
```

### Error Handling

Use `thiserror` for library errors, `anyhow` for application errors:

```rust
// In spectre-core (library)
#[derive(Debug, thiserror::Error)]
pub enum ScanError {
    #[error("target unreachable: {0}")]
    Unreachable(String),

    #[error("scan timeout after {0:?}")]
    Timeout(Duration),

    #[error("permission denied")]
    PermissionDenied,
}

// In spectre-cli (application)
fn main() -> anyhow::Result<()> {
    let result = scan_target(target)?;
    // anyhow adds context automatically
    Ok(())
}
```

### Configuration Layering

Configuration follows precedence (lowest to highest):
1. Default values
2. System config (`/etc/spectre/spectre.toml`)
3. User config (`~/.config/spectre/spectre.toml`)
4. Project config (`./spectre.toml`)
5. Environment variables (`SPECTRE_*`)
6. Command-line arguments

```rust
pub fn load_config() -> Result<Config> {
    let mut config = Config::default();

    // Layer configurations
    if let Ok(system) = load_file("/etc/spectre/spectre.toml") {
        config.merge(system);
    }
    if let Ok(user) = load_file(dirs::config_dir()?.join("spectre/spectre.toml")) {
        config.merge(user);
    }
    if let Ok(project) = load_file("./spectre.toml") {
        config.merge(project);
    }

    // Environment variables override
    config.apply_env();

    Ok(config)
}
```

### Component Isolation

Components communicate through well-defined interfaces:

```rust
// Trait for scanner integration
pub trait Scanner: Send + Sync {
    async fn scan(&self, targets: &[Target], opts: ScanOptions) -> Result<ScanResults>;
    async fn detect_services(&self, hosts: &[Host]) -> Result<Vec<Service>>;
    fn capabilities(&self) -> ScannerCapabilities;
}

// ProRT-IP implementation
pub struct PrtipScanner { /* ... */ }

impl Scanner for PrtipScanner {
    async fn scan(&self, targets: &[Target], opts: ScanOptions) -> Result<ScanResults> {
        // Delegate to ProRT-IP
    }
}
```

---

## Data Flow

### Scan Pipeline

```
┌────────────┐    ┌─────────────┐    ┌───────────────┐    ┌────────────┐
│   Targets  │───▶│  ProRT-IP   │───▶│   CyberChef   │───▶│   Output   │
│   Input    │    │   Scanner   │    │   Analysis    │    │  Formatter │
└────────────┘    └─────────────┘    └───────────────┘    └────────────┘
                        │                    │
                        ▼                    ▼
                 ┌─────────────┐      ┌─────────────┐
                 │  scan.json  │      │ decoded.txt │
                 └─────────────┘      └─────────────┘
```

### Event Flow (TUI)

```
┌────────────┐    ┌─────────────┐    ┌───────────────┐
│  Keyboard  │───▶│   Event     │───▶│     App       │
│   Input    │    │   Handler   │    │    State      │
└────────────┘    └─────────────┘    └───────┬───────┘
                                             │
┌────────────┐    ┌─────────────┐            │
│   Screen   │◀───│    UI       │◀───────────┘
│   Render   │    │   Update    │
└────────────┘    └─────────────┘
```

---

## Threading Model

### Async Tasks

- **Main thread:** CLI parsing, TUI rendering
- **Tokio runtime:** Network I/O, file operations
- **Dedicated threads:** CPU-intensive analysis (via `spawn_blocking`)

```rust
// CPU-intensive work
let result = tokio::task::spawn_blocking(move || {
    expensive_computation(data)
}).await?;

// I/O-bound work
let data = tokio::fs::read(path).await?;
```

### Channel Communication

```rust
// Between components
let (tx, rx) = tokio::sync::mpsc::channel(100);

// Progress updates
let (progress_tx, progress_rx) = tokio::sync::watch::channel(Progress::default());

// Shutdown signal
let (shutdown_tx, shutdown_rx) = tokio::sync::broadcast::channel(1);
```

---

## Testing Strategy

See [TESTING.md](TESTING.md) for detailed testing guidelines.

**Test Organization:**
- Unit tests: In each module (`#[cfg(test)]`)
- Integration tests: `/tests/` directory
- Documentation tests: In doc comments
- Property-based tests: Using `proptest`

---

## Performance Considerations

### Memory Management

- Use `Arc<T>` for shared read-only data
- Use channels instead of shared mutable state
- Stream large files instead of loading entirely
- Use object pools for frequently allocated types

### CPU Optimization

- Profile before optimizing
- Use SIMD via libraries when applicable
- Parallelize with `rayon` for CPU-bound work
- Avoid unnecessary allocations in hot paths

---

## Security Architecture

See [../security/THREAT-MODEL.md](../security/THREAT-MODEL.md) for threat modeling.

**Key Principles:**
- Validate all external input
- Use constant-time comparisons for secrets
- Sandbox Lua plugins
- Minimize privilege requirements
