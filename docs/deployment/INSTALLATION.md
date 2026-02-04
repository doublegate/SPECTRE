# Installation Guide

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## Quick Install

### Binary Download (Recommended)

Download pre-built binaries from the [releases page](https://github.com/doublegate/SPECTRE/releases).

```bash
# Linux x86_64
curl -LO https://github.com/doublegate/SPECTRE/releases/latest/download/spectre-linux-x86_64.tar.gz
tar xzf spectre-linux-x86_64.tar.gz
sudo mv spectre /usr/local/bin/

# macOS (Apple Silicon)
curl -LO https://github.com/doublegate/SPECTRE/releases/latest/download/spectre-macos-aarch64.tar.gz
tar xzf spectre-macos-aarch64.tar.gz
sudo mv spectre /usr/local/bin/

# macOS (Intel)
curl -LO https://github.com/doublegate/SPECTRE/releases/latest/download/spectre-macos-x86_64.tar.gz
tar xzf spectre-macos-x86_64.tar.gz
sudo mv spectre /usr/local/bin/
```

### Verify Installation

```bash
spectre --version
spectre status
```

---

## Building from Source

### Prerequisites

| Requirement | Version | Purpose |
|-------------|---------|---------|
| Rust | 1.88+ | Compiler |
| Git | 2.40+ | Source control |
| libpcap | 1.10+ | Packet capture |
| Docker | 24+ | CyberChef-MCP |

### Linux (Debian/Ubuntu)

```bash
# Install dependencies
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libpcap-dev git

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone and build
git clone https://github.com/doublegate/SPECTRE.git
cd SPECTRE
cargo build --release

# Install
sudo cp target/release/spectre /usr/local/bin/
```

### Linux (Fedora/RHEL)

```bash
# Install dependencies
sudo dnf install -y gcc make pkg-config libpcap-devel git

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone and build
git clone https://github.com/doublegate/SPECTRE.git
cd SPECTRE
cargo build --release

# Install
sudo cp target/release/spectre /usr/local/bin/
```

### macOS

```bash
# Install Xcode CLI tools
xcode-select --install

# Install dependencies
brew install libpcap

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Clone and build
git clone https://github.com/doublegate/SPECTRE.git
cd SPECTRE
cargo build --release

# Install
sudo cp target/release/spectre /usr/local/bin/
```

### Windows

1. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
2. Install [Npcap](https://npcap.com/) (with SDK)
3. Install [Rust](https://www.rust-lang.org/tools/install)

```powershell
# Set Npcap SDK path
$env:LIB = "C:\npcap-sdk\Lib\x64"

# Clone and build
git clone https://github.com/doublegate/SPECTRE.git
cd SPECTRE
cargo build --release

# Install
Copy-Item target\release\spectre.exe C:\Windows\System32\
```

---

## Docker Installation

### Pull Pre-built Image

```bash
docker pull ghcr.io/doublegate/spectre:latest
```

### Run with Docker

```bash
# Basic usage
docker run --rm -it ghcr.io/doublegate/spectre:latest --help

# With network access (for scanning)
docker run --rm -it --net=host --cap-add=NET_RAW \
    ghcr.io/doublegate/spectre:latest scan -sS 192.168.1.0/24

# With persistent config
docker run --rm -it \
    -v ~/.spectre:/root/.spectre \
    ghcr.io/doublegate/spectre:latest
```

### Build Docker Image

```bash
cd SPECTRE
docker build -t spectre:local .
```

---

## Post-Installation

### Network Capabilities

For SYN scans without root:

```bash
# Linux - Set capabilities
sudo setcap cap_net_raw,cap_net_admin+ep $(which spectre)

# Verify
getcap $(which spectre)
```

### Initial Configuration

```bash
# Create config directory
mkdir -p ~/.config/spectre

# Generate default config
spectre config init

# Edit configuration
$EDITOR ~/.config/spectre/spectre.toml
```

### CyberChef-MCP Setup

```bash
# Pull container
docker pull ghcr.io/doublegate/cyberchef-mcp:latest

# Or let SPECTRE manage it
spectre chef setup
```

### Shell Completion

```bash
# Bash
spectre completions bash > /etc/bash_completion.d/spectre

# Zsh
spectre completions zsh > ~/.zfunc/_spectre

# Fish
spectre completions fish > ~/.config/fish/completions/spectre.fish
```

---

## Upgrading

### Binary Upgrade

```bash
# Download latest
curl -LO https://github.com/doublegate/SPECTRE/releases/latest/download/spectre-linux-x86_64.tar.gz

# Backup current
sudo mv /usr/local/bin/spectre /usr/local/bin/spectre.bak

# Install new
tar xzf spectre-linux-x86_64.tar.gz
sudo mv spectre /usr/local/bin/

# Verify
spectre --version
```

### Source Upgrade

```bash
cd SPECTRE
git pull
cargo build --release
sudo cp target/release/spectre /usr/local/bin/
```

---

## Uninstallation

### Remove Binary

```bash
sudo rm /usr/local/bin/spectre
```

### Remove Configuration

```bash
rm -rf ~/.config/spectre
rm -rf ~/.spectre
```

### Remove Docker Images

```bash
docker rmi ghcr.io/doublegate/spectre:latest
docker rmi ghcr.io/doublegate/cyberchef-mcp:latest
```

---

## Troubleshooting

### Permission Denied

```
Error: Permission denied (os error 13)
```

**Solution:** Set capabilities or run as root.

### libpcap Not Found

```
error: could not find native static library `pcap`
```

**Solution:** Install libpcap development package.

### Docker Network Issues

```
Error: Cannot scan - no network access
```

**Solution:** Use `--net=host` and `--cap-add=NET_RAW`.
