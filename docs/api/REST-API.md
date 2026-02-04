# REST API Reference

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## Overview

SPECTRE exposes an optional REST API for remote control and integration. The API is disabled by default and must be explicitly enabled.

**Base URL:** `http://localhost:8080/api/v1`

---

## Authentication

### API Key Authentication

```bash
# Header authentication
curl -H "X-API-Key: your-api-key" http://localhost:8080/api/v1/status

# Query parameter (less secure)
curl "http://localhost:8080/api/v1/status?api_key=your-api-key"
```

### Generate API Key

```bash
spectre api keygen --name "CI Integration"
# Output: sk_live_abc123...
```

### Configuration

```toml
# spectre.toml
[api]
enabled = true
bind = "127.0.0.1:8080"
api_key = "sk_live_abc123..."
```

---

## Endpoints

### Status

#### GET /status

Get SPECTRE status and version information.

**Request:**
```bash
curl http://localhost:8080/api/v1/status
```

**Response:**
```json
{
  "status": "running",
  "version": "0.1.0",
  "uptime_seconds": 3600,
  "components": {
    "prtip": "connected",
    "cyberchef": "connected",
    "wraith": "disconnected"
  },
  "active_scans": 2,
  "active_channels": 0
}
```

---

### Scanning

#### POST /scan

Start a new scan.

**Request:**
```bash
curl -X POST http://localhost:8080/api/v1/scan \
  -H "Content-Type: application/json" \
  -H "X-API-Key: sk_live_..." \
  -d '{
    "targets": ["192.168.1.0/24"],
    "ports": "1-1000",
    "scan_type": "syn",
    "options": {
      "service_detection": true,
      "os_detection": false,
      "rate": 1000
    }
  }'
```

**Response:**
```json
{
  "scan_id": "scan_abc123",
  "status": "started",
  "estimated_duration": "5m30s",
  "targets_count": 254
}
```

#### GET /scan/{scan_id}

Get scan status and results.

**Request:**
```bash
curl http://localhost:8080/api/v1/scan/scan_abc123
```

**Response:**
```json
{
  "scan_id": "scan_abc123",
  "status": "running",
  "progress": 0.45,
  "started_at": "2026-02-04T10:00:00Z",
  "hosts_scanned": 115,
  "hosts_up": 12,
  "open_ports": 47,
  "results": {
    "hosts": [
      {
        "ip": "192.168.1.10",
        "status": "up",
        "open_ports": [
          {"port": 22, "service": "ssh", "version": "OpenSSH 8.9"},
          {"port": 80, "service": "http", "version": "nginx 1.18"}
        ]
      }
    ]
  }
}
```

#### GET /scan/{scan_id}/stream

Stream scan results (Server-Sent Events).

**Request:**
```bash
curl -N http://localhost:8080/api/v1/scan/scan_abc123/stream
```

**Response:**
```
event: host_discovered
data: {"ip": "192.168.1.10", "status": "up"}

event: port_open
data: {"ip": "192.168.1.10", "port": 22, "service": "ssh"}

event: progress
data: {"percent": 50}

event: complete
data: {"total_hosts": 254, "hosts_up": 15}
```

#### DELETE /scan/{scan_id}

Cancel a running scan.

**Request:**
```bash
curl -X DELETE http://localhost:8080/api/v1/scan/scan_abc123
```

**Response:**
```json
{
  "scan_id": "scan_abc123",
  "status": "cancelled"
}
```

---

### CyberChef Operations

#### POST /chef/bake

Execute CyberChef recipe.

**Request:**
```bash
curl -X POST http://localhost:8080/api/v1/chef/bake \
  -H "Content-Type: application/json" \
  -d '{
    "input": "SGVsbG8gV29ybGQh",
    "recipe": ["From_Base64", "To_Hex"]
  }'
```

**Response:**
```json
{
  "output": "48 65 6c 6c 6f 20 57 6f 72 6c 64 21",
  "duration_ms": 5
}
```

#### POST /chef/magic

Auto-detect input format.

**Request:**
```bash
curl -X POST http://localhost:8080/api/v1/chef/magic \
  -H "Content-Type: application/json" \
  -d '{
    "input": "UEsDBBQAAAA..."
  }'
```

**Response:**
```json
{
  "detections": [
    {"format": "ZIP", "confidence": 95},
    {"format": "Base64", "confidence": 80}
  ]
}
```

#### GET /chef/operations

List available operations.

**Response:**
```json
{
  "operations": [
    {"name": "From_Base64", "category": "encoding"},
    {"name": "AES_Decrypt", "category": "encryption"},
    {"name": "Extract_URLs", "category": "extraction"}
  ],
  "total": 463
}
```

---

### Campaigns

#### POST /campaign

Create a new campaign.

**Request:**
```bash
curl -X POST http://localhost:8080/api/v1/campaign \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Network Assessment",
    "targets": ["192.168.1.0/24", "10.0.0.0/24"],
    "workflow": "recon-full"
  }'
```

**Response:**
```json
{
  "campaign_id": "camp_xyz789",
  "name": "Network Assessment",
  "status": "created",
  "created_at": "2026-02-04T10:00:00Z"
}
```

#### GET /campaign/{campaign_id}

Get campaign details.

#### PUT /campaign/{campaign_id}

Update campaign.

#### DELETE /campaign/{campaign_id}

Delete campaign.

---

### Artifacts

#### GET /artifacts

List artifacts.

**Request:**
```bash
curl "http://localhost:8080/api/v1/artifacts?campaign_id=camp_xyz789"
```

**Response:**
```json
{
  "artifacts": [
    {
      "id": "art_123",
      "type": "scan_results",
      "campaign_id": "camp_xyz789",
      "created_at": "2026-02-04T10:30:00Z",
      "size_bytes": 15420
    }
  ]
}
```

#### GET /artifacts/{artifact_id}

Download artifact.

#### DELETE /artifacts/{artifact_id}

Delete artifact.

---

## Error Responses

All errors follow this format:

```json
{
  "error": {
    "code": "INVALID_REQUEST",
    "message": "Invalid port range specified",
    "details": {
      "field": "ports",
      "value": "1-99999",
      "constraint": "Port must be between 1 and 65535"
    }
  }
}
```

### Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `UNAUTHORIZED` | 401 | Missing or invalid API key |
| `FORBIDDEN` | 403 | API key lacks permission |
| `NOT_FOUND` | 404 | Resource not found |
| `INVALID_REQUEST` | 400 | Invalid request parameters |
| `RATE_LIMITED` | 429 | Too many requests |
| `INTERNAL_ERROR` | 500 | Server error |

---

## Rate Limiting

Default limits:
- 100 requests per minute per API key
- 10 concurrent scans per API key

Headers:
```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1707048000
```

---

## Webhooks

Configure webhooks for async notifications:

```toml
[api.webhooks]
scan_complete = "https://your-server.com/webhook"
campaign_update = "https://your-server.com/webhook"
```

Webhook payload:
```json
{
  "event": "scan_complete",
  "timestamp": "2026-02-04T10:35:00Z",
  "data": {
    "scan_id": "scan_abc123",
    "status": "complete",
    "summary": {
      "hosts_scanned": 254,
      "hosts_up": 15,
      "open_ports": 47
    }
  }
}
```
