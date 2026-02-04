# Tutorial: Your First Scan

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## Overview

This tutorial walks you through performing your first network scan with SPECTRE.

**Time Required:** 10 minutes

**Prerequisites:**
- SPECTRE installed (`spectre --version`)
- Network capabilities set (or run as root)
- A target network you're authorized to scan

---

## Step 1: Verify Installation

```bash
# Check SPECTRE is installed
spectre --version

# Check component status
spectre status
```

You should see:
```
SPECTRE v0.1.0
Components:
  ProRT-IP:     connected
  CyberChef:    connected
  WRAITH:       disconnected (optional)
```

---

## Step 2: Simple Port Scan

Let's start with a basic TCP connect scan against a single host.

```bash
# Connect scan (no special privileges needed)
spectre scan -sT 192.168.1.1
```

Output:
```
Starting SPECTRE scan of 192.168.1.1
Scan type: TCP Connect

PORT      STATE    SERVICE
22/tcp    open     ssh
80/tcp    open     http
443/tcp   open     https

Scan completed: 1 host up, 3 open ports (0.5s)
```

---

## Step 3: SYN Scan (Faster)

SYN scans are faster and stealthier, but require elevated privileges.

```bash
# SYN scan (requires root or CAP_NET_RAW)
sudo spectre scan -sS 192.168.1.1

# Or if capabilities are set:
spectre scan -sS 192.168.1.1
```

---

## Step 4: Specify Ports

Scan specific ports instead of defaults:

```bash
# Common web ports
spectre scan -sS -p 80,443,8080,8443 192.168.1.1

# Port range
spectre scan -sS -p 1-1000 192.168.1.1

# All ports
spectre scan -sS -p- 192.168.1.1

# Top 100 ports
spectre scan -sS --top-ports 100 192.168.1.1
```

---

## Step 5: Service Detection

Identify what services are running on open ports:

```bash
spectre scan -sS -sV 192.168.1.1
```

Output:
```
PORT      STATE    SERVICE         VERSION
22/tcp    open     ssh             OpenSSH 8.9p1 Ubuntu
80/tcp    open     http            nginx 1.24.0
443/tcp   open     https           nginx 1.24.0
3306/tcp  open     mysql           MySQL 8.0.35
```

---

## Step 6: Scan a Network Range

Scan an entire subnet:

```bash
# Scan /24 network
spectre scan -sS 192.168.1.0/24

# With service detection
spectre scan -sS -sV 192.168.1.0/24

# Limit rate to be gentle
spectre scan -sS --rate 100 192.168.1.0/24
```

---

## Step 7: Output Formats

Save results in different formats:

```bash
# JSON output
spectre scan -sS -o json 192.168.1.0/24 > results.json

# Table output (default)
spectre scan -sS -o table 192.168.1.0/24

# Greppable output
spectre scan -sS -o greppable 192.168.1.0/24 > results.gnmap

# All formats
spectre scan -sS -oA scan_results 192.168.1.0/24
```

---

## Step 8: Review Results

View the JSON results:

```bash
# Pretty print JSON
cat results.json | jq '.'

# List all hosts with open ports
cat results.json | jq '.hosts[] | select(.open_ports | length > 0) | .ip'

# Find web servers
cat results.json | jq '.hosts[].open_ports[] | select(.port == 80 or .port == 443)'
```

---

## Common Scan Types

| Flag | Type | Privileges | Use Case |
|------|------|------------|----------|
| `-sS` | SYN | Root/CAP_NET_RAW | Fast, stealthy |
| `-sT` | Connect | None | Unprivileged |
| `-sU` | UDP | Root/CAP_NET_RAW | UDP services |
| `-sV` | Version | Depends | Service detection |
| `-sA` | ACK | Root/CAP_NET_RAW | Firewall mapping |

---

## Timing Options

| Flag | Name | Speed | Stealth |
|------|------|-------|---------|
| `-T0` | Paranoid | Slowest | Highest |
| `-T1` | Sneaky | Very slow | High |
| `-T2` | Polite | Slow | Medium |
| `-T3` | Normal | Default | Default |
| `-T4` | Aggressive | Fast | Low |
| `-T5` | Insane | Fastest | None |

```bash
# Aggressive timing for local network
spectre scan -sS -T4 192.168.1.0/24

# Sneaky timing for external targets
spectre scan -sS -T1 target.example.com
```

---

## Next Steps

- [CLI Reference](../user-guide/CLI-REFERENCE.md) - Complete command documentation
- [TUI Guide](../user-guide/TUI-GUIDE.md) - Use the interactive dashboard
- [Secure Channel Tutorial](SECURE-CHANNEL.md) - Set up encrypted communications
- [Data Analysis Tutorial](DATA-ANALYSIS.md) - Analyze scan results
