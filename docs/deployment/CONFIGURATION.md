# Configuration Reference

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## Overview

SPECTRE uses TOML configuration files with the following precedence (lowest to highest):

1. Default values (built-in)
2. System config (`/etc/spectre/spectre.toml`)
3. User config (`~/.config/spectre/spectre.toml`)
4. Project config (`./spectre.toml`)
5. Environment variables (`SPECTRE_*`)
6. Command-line arguments

---

## Configuration File

### Generate Default Config

```bash
spectre config init
spectre config show
```

### Full Configuration Example

```toml
# ~/.config/spectre/spectre.toml

# General settings
[general]
# Data directory for artifacts and cache
data_dir = "~/.spectre"

# Temporary file directory
temp_dir = "/tmp/spectre"

# Log level: trace, debug, info, warn, error
log_level = "info"

# Log file (optional, defaults to stderr)
log_file = "~/.spectre/logs/spectre.log"

# Enable color output
color = true

#─────────────────────────────────────────────────────────────────────────────
# Scanning Configuration
#─────────────────────────────────────────────────────────────────────────────
[scan]
# Default scan type: syn, connect, fin, null, xmas, ack, udp
default_type = "syn"

# Default ports to scan
default_ports = "21-25,80,110,143,443,445,3306,3389,8080,8443"

# Scan rate (packets per second)
rate = 1000

# Probe timeout (milliseconds)
timeout = 3000

# Number of retries
retries = 2

# Timing template (0-5): paranoid, sneaky, polite, normal, aggressive, insane
timing = 3

# Network interface (auto-detect if not specified)
# interface = "eth0"

# Enable AF_XDP kernel bypass (Linux 5.4+)
af_xdp = false

[scan.detection]
# Service detection intensity (0-9)
service_intensity = 5

# OS detection
os_detection = false

# Banner grab timeout (milliseconds)
banner_timeout = 5000

[scan.evasion]
# Packet fragmentation
fragment = false
mtu = 0

# Decoy IPs (RND for random)
# decoys = ["192.168.1.50", "RND", "RND"]

# Source port (0 = random)
source_port = 0

# TTL (0 = system default)
ttl = 0

# Bad checksum (bypass some firewalls)
bad_checksum = false

[scan.scope]
# Only allow scanning these ranges (empty = all allowed)
# allowed = ["192.168.0.0/16", "10.0.0.0/8"]

# Never scan these (always blocked)
blocked = ["127.0.0.0/8", "224.0.0.0/4"]

#─────────────────────────────────────────────────────────────────────────────
# CyberChef Configuration
#─────────────────────────────────────────────────────────────────────────────
[chef]
# MCP endpoint
mcp_endpoint = "stdio://cyberchef-mcp"

# Alternative HTTP endpoint
# mcp_endpoint = "http://localhost:3000"

# Operation timeout (seconds)
timeout = 30

# Maximum input size (MB)
max_input_size = 100

# Result caching
cache_enabled = true
cache_size = 1000

[chef.docker]
# Auto-start CyberChef container
auto_start = true

# Container image
image = "ghcr.io/doublegate/cyberchef-mcp:latest"

# Container name
name = "spectre-cyberchef"

#─────────────────────────────────────────────────────────────────────────────
# WRAITH Configuration
#─────────────────────────────────────────────────────────────────────────────
[wraith]
# WRAITH identity file
identity_file = "~/.spectre/identity.key"

# Default channel settings
[wraith.channel]
# Encryption algorithm
cipher = "xchacha20poly1305"

# Key exchange
kex = "x25519"

# Enable post-quantum hybrid mode
post_quantum = false

[wraith.protocol]
# Protocol mimicry: none, tls13, http2, dns, doh
mimicry = "none"

# SNI for TLS mimicry
# sni = "www.example.com"

# Traffic padding
padding = true
min_padding = 0
max_padding = 256

# Timing jitter (milliseconds)
jitter = 0

[wraith.transport]
# Default transport: tcp, udp, quic
transport = "tcp"

# Connection timeout (seconds)
connect_timeout = 10

# Keepalive interval (seconds)
keepalive = 30

#─────────────────────────────────────────────────────────────────────────────
# Campaign Configuration
#─────────────────────────────────────────────────────────────────────────────
[campaign]
# Campaign data directory
data_dir = "~/.spectre/campaigns"

# Auto-save interval (seconds)
auto_save = 60

# Maximum artifacts per campaign
max_artifacts = 10000

#─────────────────────────────────────────────────────────────────────────────
# API Configuration
#─────────────────────────────────────────────────────────────────────────────
[api]
# Enable REST API
enabled = false

# Bind address
bind = "127.0.0.1:8080"

# API key (set via environment for security)
# key = "sk_live_..."

# Rate limit (requests per minute)
rate_limit = 100

# Enable CORS
cors = false
# cors_origins = ["http://localhost:3000"]

#─────────────────────────────────────────────────────────────────────────────
# MCP Server Configuration
#─────────────────────────────────────────────────────────────────────────────
[mcp]
# MCP transport: stdio, http
transport = "stdio"

# HTTP port (if transport = http)
port = 3001

# Authentication token
# auth_token = "..."

#─────────────────────────────────────────────────────────────────────────────
# Plugin Configuration
#─────────────────────────────────────────────────────────────────────────────
[plugins]
# Plugin directory
directory = "~/.spectre/plugins"

# Enable plugin system
enabled = true

# Loaded plugins (or ["*"] for all)
load = ["*"]

# Plugin resource limits
[plugins.limits]
memory_mb = 64
cpu_seconds = 10
network = false

#─────────────────────────────────────────────────────────────────────────────
# Output Configuration
#─────────────────────────────────────────────────────────────────────────────
[output]
# Default format: json, table, greppable, xml
format = "table"

# Pretty print JSON
json_pretty = true

# Include timestamps
timestamps = true

# Color output
color = true
```

---

## Environment Variables

All configuration options can be set via environment variables with the `SPECTRE_` prefix:

| Variable | Config Path | Example |
|----------|-------------|---------|
| `SPECTRE_LOG_LEVEL` | `general.log_level` | `debug` |
| `SPECTRE_SCAN_RATE` | `scan.rate` | `10000` |
| `SPECTRE_SCAN_TIMEOUT` | `scan.timeout` | `5000` |
| `SPECTRE_CHEF_ENDPOINT` | `chef.mcp_endpoint` | `http://localhost:3000` |
| `SPECTRE_API_KEY` | `api.key` | `sk_live_...` |
| `SPECTRE_API_ENABLED` | `api.enabled` | `true` |

Nested keys use underscores:
```bash
export SPECTRE_SCAN_DETECTION_SERVICE_INTENSITY=9
export SPECTRE_WRAITH_PROTOCOL_MIMICRY=tls13
```

---

## Command-Line Overrides

Most options can be overridden on the command line:

```bash
# Override scan rate
spectre scan -sS --rate 5000 target

# Override timing
spectre scan -sS -T4 target

# Override output format
spectre scan -sS --output json target

# Override log level
spectre --log-level debug scan -sS target
```

---

## Configuration Validation

```bash
# Validate configuration
spectre config check

# Show effective configuration
spectre config show

# Show specific section
spectre config show scan

# Show config file locations
spectre config paths
```
