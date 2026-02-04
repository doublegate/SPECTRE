# Operational Security Guide

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## Overview

This document provides operational security (OPSEC) guidelines for using SPECTRE in security assessments and red team operations.

---

## Pre-Operation

### Authorization

Before any operation:
- [ ] Written authorization from asset owner
- [ ] Defined scope (IP ranges, domains, times)
- [ ] Rules of engagement documented
- [ ] Emergency contacts identified
- [ ] Legal review completed

### Environment Preparation

```bash
# Dedicated assessment VM
# - Fresh OS installation
# - No personal data
# - Encrypted disk

# Network isolation
# - Separate physical interface
# - VPN/proxy chain configured
# - DNS leak protection

# Time synchronization
sudo timedatectl set-timezone UTC
```

### Identity Separation

- Use dedicated accounts
- Separate SSH keys for operations
- Unique WRAITH identities per engagement
- No cross-contamination with personal

---

## Network OPSEC

### Traffic Anonymization

```bash
# Configure proxy chain
export SPECTRE_PROXY="socks5://127.0.0.1:9050"

# Or in spectre.toml
[network]
proxy = "socks5://127.0.0.1:9050"
proxy_dns = true
```

### DNS Considerations

```bash
# Use DNS over HTTPS
[network]
dns_servers = ["https://cloudflare-dns.com/dns-query"]

# Or DNS over WRAITH
[network]
dns_over_wraith = true
```

### Source IP Management

- Use jump hosts
- Rotate source IPs
- Avoid scanning from attributable infrastructure
- Consider cloud-based scanning nodes

### Traffic Patterns

```bash
# Randomize timing
spectre scan -sS --randomize-hosts --delay 100-500ms 192.168.1.0/24

# Use polite timing for stealth
spectre scan -sS -T2 target

# Fragment packets
spectre scan -sS -f --mtu 8 target
```

---

## Host OPSEC

### Filesystem Security

```bash
# Encrypted working directory
mkdir -p /tmp/spectre-work
sudo mount -t tmpfs -o size=1G,mode=700 tmpfs /tmp/spectre-work
export SPECTRE_DATA_DIR=/tmp/spectre-work

# Or use encrypted container
cryptsetup luksOpen /dev/sdb1 spectre-data
mount /dev/mapper/spectre-data /mnt/spectre
```

### Memory Protection

```bash
# Disable core dumps
ulimit -c 0

# Lock memory
[security]
lock_memory = true
```

### Log Management

```bash
# Minimize logging during ops
export RUST_LOG=error

# Or configure in spectre.toml
[logging]
level = "error"
sanitize = true
file = "/tmp/spectre-work/spectre.log"
```

---

## Data Handling

### Classification

| Data Type | Handling |
|-----------|----------|
| Scan results | Encrypt, time-limit |
| Credentials | Never store plaintext |
| Artifacts | Chain of custody |
| Reports | Classified per engagement |

### Secure Storage

```bash
# Encrypt all operation data
spectre campaign export --encrypt campaign_data.enc

# Secure deletion
shred -vfz -n 3 sensitive_file
```

### Data Transfer

```bash
# Use WRAITH for data exfiltration
spectre send --channel secure-exfil --file data.tar.gz

# Or encrypted archive
gpg --symmetric --cipher-algo AES256 data.tar.gz
```

---

## Communication OPSEC

### Secure Channels

```bash
# Establish WRAITH channel
spectre channel create --name ops-channel --auth mutual

# All C2 over WRAITH
[wraith]
c2_channel = "ops-channel"
```

### Protocol Mimicry

```bash
# Mimic HTTPS traffic
[wraith.protocol]
mimicry = "tls13"
sni = "www.microsoft.com"

# Or DNS over HTTPS
[wraith.protocol]
mimicry = "doh"
```

### Traffic Scheduling

```bash
# Operate during business hours
[schedule]
active_hours = "09:00-17:00"
timezone = "America/New_York"
jitter = "15m"
```

---

## Incident Response

### Detection Indicators

If you suspect detection:

1. **Immediate:**
   - Stop active scans
   - Close WRAITH channels
   - Clear local artifacts

2. **Assessment:**
   - Review target behavior
   - Check for honeypots
   - Analyze timing correlation

3. **Response:**
   - Rotate infrastructure
   - Change techniques
   - Document for debrief

### Emergency Procedures

```bash
# Emergency shutdown
spectre emergency-stop --wipe

# This will:
# - Stop all scans
# - Close all channels
# - Clear memory
# - Secure-delete temp files
```

### Evidence Preservation

For legitimate operations, preserve evidence:

```bash
# Create forensic archive
spectre campaign archive --signed --timestamp campaign_id

# Includes:
# - All logs
# - All artifacts
# - Chain of custody
# - Timestamps
```

---

## Post-Operation

### Cleanup Checklist

- [ ] All scans stopped
- [ ] All channels closed
- [ ] Temporary files deleted
- [ ] Logs sanitized
- [ ] Keys rotated
- [ ] VM reverted/destroyed
- [ ] Report delivered securely

### Secure Reporting

```bash
# Encrypt report for client
gpg --encrypt --recipient client@example.com report.pdf

# Or use secure delivery
spectre send --channel client-delivery --file report.pdf.gpg
```

### Debriefing

Document:
- Techniques used
- Detection events
- Effectiveness
- Improvements for future

---

## Tools Configuration

### Timing Profiles

| Profile | Use Case |
|---------|----------|
| T0 (Paranoid) | IDS evasion required |
| T1 (Sneaky) | Minimal detection risk |
| T2 (Polite) | Normal stealth ops |
| T3 (Normal) | Authorized testing |
| T4 (Aggressive) | Time-critical |

### Evasion Techniques

```toml
# spectre.toml
[scan.evasion]
# Packet fragmentation
fragment = true
mtu = 8

# Decoy scanning
decoys = ["RND:5"]

# Source port spoofing
source_port = 53

# TTL manipulation
ttl = 64

# Bad checksum (firewall bypass)
bad_checksum = false
```

### Protocol Configuration

```toml
# WRAITH OPSEC settings
[wraith]
# Change protocol fingerprint
protocol_version = "randomize"

# Padding to normalize packet sizes
padding = true
min_padding = 64
max_padding = 256

# Timing jitter
jitter_ms = 100
```

---

## Red Team Considerations

### Infrastructure

- Separate command and control
- Redundant communication paths
- Geographically distributed
- Disposable attack infrastructure

### Attribution Prevention

- No identifiable patterns
- Vary techniques between targets
- Use common tools/techniques
- Avoid signature behaviors

### Persistence Considerations

- Minimal footprint
- Encrypted communications only
- Time-limited access
- Dead man switches
