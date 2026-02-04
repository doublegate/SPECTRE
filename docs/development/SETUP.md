# Development Environment Setup

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## Prerequisites

### Required Software

| Software | Version | Purpose |
|----------|---------|---------|
| Rust | 1.88+ | Primary language |
| Cargo | 1.88+ | Package manager |
| Git | 2.40+ | Version control |
| Docker | 24+ | CyberChef-MCP container |

### Platform-Specific Dependencies

#### Linux (Debian/Ubuntu)

```bash
# Build essentials
sudo apt-get update
sudo apt-get install -y build-essential pkg-config

# Network libraries (for ProRT-IP)
sudo apt-get install -y libpcap-dev

# Optional: musl for static linking
sudo apt-get install -y musl-tools
```

#### Linux (Fedora/RHEL)

```bash
sudo dnf install -y gcc make pkg-config
sudo dnf install -y libpcap-devel
```

#### Linux (Arch)

```bash
sudo pacman -S base-devel pkg-config
sudo pacman -S libpcap
```

#### macOS

```bash
# Xcode command line tools
xcode-select --install

# Homebrew packages
brew install libpcap
```

#### Windows

1. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
2. Install [Npcap](https://npcap.com/) with SDK option
3. Set environment variables:
   ```powershell
   $env:LIB = "C:\npcap-sdk\Lib\x64"
   $env:PATH += ";C:\npcap-sdk\Lib\x64"
   ```

---

## Rust Installation

### Using rustup (Recommended)

```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Source the environment
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### Required Components

```bash
# Formatting
rustup component add rustfmt

# Linting
rustup component add clippy

# Documentation
rustup component add rust-docs

# Optional: Language server
rustup component add rust-analyzer
```

### Additional Cargo Tools

```bash
# Security audit
cargo install cargo-audit

# Code coverage
cargo install cargo-tarpaulin

# Watch mode for development
cargo install cargo-watch

# Benchmarking
cargo install cargo-criterion

# License checking
cargo install cargo-deny
```

---

## Clone and Build

### Clone the Repository

```bash
# Clone with submodules
git clone --recursive https://github.com/doublegate/SPECTRE.git
cd SPECTRE
```

### Build the Workspace

```bash
# Debug build (faster compilation)
cargo build --workspace

# Release build (optimized)
cargo build --workspace --release

# Build specific crate
cargo build -p spectre-cli
cargo build -p spectre-core
```

### Run Tests

```bash
# All tests
cargo test --workspace

# With output
cargo test --workspace -- --nocapture

# Specific crate
cargo test -p spectre-core

# Single test
cargo test test_name
```

### Run the CLI

```bash
# Debug build
cargo run -- --help

# Release build
cargo run --release -- --help

# Specific command
cargo run -- status
cargo run -- scan -sS 127.0.0.1
```

---

## IDE Setup

### VS Code

#### Recommended Extensions

- `rust-analyzer` - Rust language support
- `crates` - Dependency management
- `Even Better TOML` - TOML support
- `Error Lens` - Inline error display
- `GitLens` - Git integration

#### settings.json

```json
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.checkOnSave.extraArgs": ["--", "-D", "warnings"],
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.tabSize": 4
  },
  "files.watcherExclude": {
    "**/target/**": true
  }
}
```

#### launch.json (Debugging)

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug SPECTRE CLI",
      "cargo": {
        "args": ["build", "--bin=spectre", "--package=spectre-cli"],
        "filter": {
          "name": "spectre",
          "kind": "bin"
        }
      },
      "args": ["--help"],
      "cwd": "${workspaceFolder}"
    },
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug Unit Tests",
      "cargo": {
        "args": ["test", "--no-run", "--lib", "--package=spectre-core"],
        "filter": {
          "kind": "lib"
        }
      },
      "cwd": "${workspaceFolder}"
    }
  ]
}
```

### IntelliJ IDEA / CLion

1. Install the **Rust** plugin
2. Open the project root (with Cargo.toml)
3. Configure:
   - Settings > Languages & Frameworks > Rust > Rustfmt: Enable "Run rustfmt on save"
   - Settings > Languages & Frameworks > Rust > External Linters: Enable Clippy

### Neovim

Using `nvim-lspconfig` with rust-analyzer:

```lua
require('lspconfig').rust_analyzer.setup({
  settings = {
    ['rust-analyzer'] = {
      checkOnSave = {
        command = 'clippy',
        extraArgs = { '--', '-D', 'warnings' },
      },
      cargo = {
        features = 'all',
      },
    },
  },
})
```

---

## Development Workflow

### Daily Development

```bash
# Start watch mode (auto-rebuild on changes)
cargo watch -x check -x test

# Or with clippy
cargo watch -x clippy -x test
```

### Before Committing

```bash
# Format code
cargo fmt --all

# Run clippy
cargo clippy --workspace -- -D warnings

# Run tests
cargo test --workspace

# Security audit
cargo audit
```

### Feature Branch Workflow

```bash
# Create feature branch
git checkout -b feat/my-feature

# Make changes and commit
git add -p
git commit -m "feat(cli): add new command"

# Push and create PR
git push -u origin feat/my-feature
gh pr create
```

---

## Troubleshooting

### Build Errors

#### Missing libpcap

```
error: could not find native static library `pcap`
```

Solution: Install libpcap development package (see platform-specific instructions above).

#### Linker Errors on Windows

```
LINK : fatal error LNK1181: cannot open input file 'Packet.lib'
```

Solution: Ensure Npcap SDK is installed and `LIB` environment variable is set.

#### Old Rust Version

```
error: package `spectre-cli v0.1.0` cannot be built because it requires rustc 1.88
```

Solution: Update Rust with `rustup update stable`.

### Runtime Errors

#### Permission Denied (Linux)

```
PermissionError: Operation not permitted
```

Solution: Run with `sudo` or set capabilities:
```bash
sudo setcap cap_net_raw,cap_net_admin+ep target/debug/spectre
```

#### pcap Not Found (macOS)

```
dyld: Library not loaded: /usr/local/opt/libpcap/lib/libpcap.dylib
```

Solution: Install or reinstall libpcap via Homebrew.

---

## Next Steps

- Read [ARCHITECTURE.md](ARCHITECTURE.md) for codebase overview
- Read [TESTING.md](TESTING.md) for testing guidelines
- Read [DEBUGGING.md](DEBUGGING.md) for debugging techniques
- Check [../CONTRIBUTING.md](../../CONTRIBUTING.md) for contribution guidelines
