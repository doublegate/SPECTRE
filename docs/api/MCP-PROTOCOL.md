# MCP Protocol Specification

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## Overview

SPECTRE implements the Model Context Protocol (MCP) for AI assistant integration, enabling natural language control of security operations.

---

## Server Configuration

### Starting the MCP Server

```bash
# Stdio transport (for Claude Desktop, Cursor)
spectre mcp serve

# HTTP transport (for custom clients)
spectre mcp serve --transport http --port 3000
```

### Claude Desktop Configuration

```json
{
  "mcpServers": {
    "spectre": {
      "command": "spectre",
      "args": ["mcp", "serve"]
    }
  }
}
```

---

## Tools

### scan_network

Execute network reconnaissance.

**Schema:**
```json
{
  "name": "scan_network",
  "description": "Scan network targets for open ports and services",
  "inputSchema": {
    "type": "object",
    "properties": {
      "targets": {
        "type": "array",
        "items": {"type": "string"},
        "description": "IP addresses, CIDR ranges, or hostnames"
      },
      "ports": {
        "type": "string",
        "description": "Port specification"
      },
      "scan_type": {
        "type": "string",
        "enum": ["syn", "connect", "fin", "null", "xmas", "ack", "udp"],
        "default": "syn"
      }
    },
    "required": ["targets"]
  }
}
```

### analyze_data

Process data with CyberChef operations.

**Schema:**
```json
{
  "name": "analyze_data",
  "description": "Analyze and transform data using CyberChef operations",
  "inputSchema": {
    "type": "object",
    "properties": {
      "input": {"type": "string"},
      "operations": {"type": "array", "items": {"type": "string"}}
    },
    "required": ["input"]
  }
}
```

### extract_iocs

Extract indicators of compromise from data.

### hash_data

Generate cryptographic hashes.

### secure_send

Send data over secure WRAITH channel.

### campaign_status

Get campaign status and metrics.

---

## Resources

- `spectre://scan/{scan_id}` - Scan results
- `spectre://campaign/{id}/artifacts` - Campaign data
- `spectre://recipes/{name}` - CyberChef recipes

---

## Prompts

- `recon_network` - Guided network reconnaissance
- `analyze_sample` - Analyze suspicious data

---

## Protocol Messages

Standard MCP JSON-RPC 2.0 messages for initialization, tool calls, and responses.
