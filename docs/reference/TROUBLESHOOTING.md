# Troubleshooting Guide

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## Installation Issues

### Rust Version Too Old

**Error:**
```
error: package `spectre v0.1.0` cannot be built because it requires rustc 1.88
```

**Solution:**
```bash
rustup update stable
rustc --version  # Should be 1.88+
```

### libpcap Not Found

**Error:**
```
error: could not find native static library `pcap`
```

**Solution:**

Linux (Debian/Ubuntu):
```bash
sudo apt-get install libpcap-dev
```

Linux (Fedora):
```bash
sudo dnf install libpcap-devel
```

macOS:
```bash
brew install libpcap
```

### Windows Npcap Issues

**Error:**
```
LINK : fatal error LNK1181: cannot open input file 'Packet.lib'
```

**Solution:**
1. Install Npcap from https://npcap.com/
2. Install with "Install Npcap SDK" option
3. Set environment:
```powershell
$env:LIB = "C:\npcap-sdk\Lib\x64"
```

---

## Permission Issues

### Permission Denied (Raw Sockets)

**Error:**
```
Error: Permission denied (os error 13)
```

**Cause:** SYN scans require raw socket access.

**Solutions:**

Option 1 - Run as root:
```bash
sudo spectre scan -sS target
```

Option 2 - Set capabilities (persistent, Linux only):
```bash
sudo setcap cap_net_raw,cap_net_admin+ep $(which spectre)
```

Option 3 - Use connect scan:
```bash
spectre scan -sT target  # No special privileges needed
```

### Cannot Bind to Port

**Error:**
```
Error: Address already in use (os error 98)
```

**Solution:**
```bash
# Find process using port
lsof -i :8080

# Kill or wait
kill <PID>
# Or wait for TIME_WAIT to expire (~60s)
```

---

## Scanning Issues

### No Hosts Found

**Error:**
```
Scan complete: 0 hosts up
```

**Causes and Solutions:**

1. **Firewall blocking ICMP:**
   ```bash
   spectre scan -Pn target  # Skip host discovery
   ```

2. **Wrong network:**
   ```bash
   ip addr  # Verify your IP range
   ```

3. **Host truly down:**
   ```bash
   ping target  # Basic connectivity test
   ```

### Scan Timeout

**Error:**
```
Error: operation timed out
```

**Solutions:**
```bash
# Increase timeout
spectre scan -sS --timeout 10000 target

# Reduce rate
spectre scan -sS --rate 100 target

# Use slower timing
spectre scan -sS -T2 target
```

### All Ports Filtered

**Output:**
```
All 1000 ports are filtered
```

**Cause:** Firewall dropping packets.

**Solutions:**
```bash
# Try different scan type
spectre scan -sA target  # ACK scan for firewall mapping

# Try fragmentation
spectre scan -sS -f target

# Try different source port
spectre scan -sS -g 53 target  # DNS port
```

### Service Detection Failing

**Issue:** Services showing as "unknown"

**Solutions:**
```bash
# Increase intensity
spectre scan -sV --version-intensity 9 target

# Increase timeout
spectre scan -sV --banner-timeout 10000 target

# Try specific ports
spectre scan -sV -p 22,80,443 target
```

---

## CyberChef Issues

### CyberChef Connection Failed

**Error:**
```
Error: Failed to connect to CyberChef-MCP
```

**Solutions:**

1. Check if container is running:
   ```bash
   docker ps | grep cyberchef
   ```

2. Start container:
   ```bash
   docker start spectre-cyberchef
   # Or
   spectre chef setup
   ```

3. Check logs:
   ```bash
   docker logs spectre-cyberchef
   ```

### Operation Not Found

**Error:**
```
Error: Unknown operation 'My_Operation'
```

**Solution:**
```bash
# List available operations
spectre chef --list | grep -i "operation"

# Check exact name
spectre chef --describe "From_Base64"  # Note underscores
```

### Operation Timeout

**Error:**
```
Error: Operation timed out after 30s
```

**Solutions:**
```bash
# Increase timeout
spectre chef --timeout 120 "Heavy_Operation" --input ...

# For large files, use streaming
spectre chef --stream "Gunzip" --file large.gz
```

---

## WRAITH Issues

### Identity Generation Failed

**Error:**
```
Error: Failed to generate identity
```

**Solutions:**
```bash
# Check directory permissions
ls -la ~/.spectre/

# Create directory
mkdir -p ~/.spectre
chmod 700 ~/.spectre

# Retry
spectre identity init
```

### Peer Connection Failed

**Error:**
```
Error: Failed to connect to peer
```

**Causes and Solutions:**

1. **Peer offline:**
   ```bash
   spectre peer status teammate
   ```

2. **Network blocked:**
   ```bash
   # Try different port
   spectre channel create --peer teammate --port 443
   ```

3. **Wrong identity:**
   ```bash
   spectre peer verify teammate
   ```

### Channel Handshake Failed

**Error:**
```
Error: Handshake failed: invalid signature
```

**Solution:**
```bash
# Remove and re-add peer
spectre peer remove teammate
spectre peer add --name teammate "spectre:correct_id..."
```

---

## Performance Issues

### High Memory Usage

**Cause:** Buffering large scan results

**Solutions:**
```bash
# Stream results
spectre scan -sS --stream target

# Limit scan size
spectre scan -sS --max-hosts 1000 target

# Export incrementally
spectre scan -sS --output-incremental results/ target
```

### Slow Scanning

**Cause:** Conservative defaults

**Solutions:**
```bash
# Enable AF_XDP (Linux)
spectre scan -sS --af-xdp target

# Increase rate
spectre scan -sS --rate 10000 target

# Aggressive timing
spectre scan -sS -T4 target
```

### CPU Usage High

**Cause:** Service detection intensive

**Solution:**
```bash
# Reduce detection intensity
spectre scan -sV --version-intensity 3 target

# Skip detection
spectre scan -sS target  # No -sV
```

---

## Configuration Issues

### Config Not Loading

**Check config paths:**
```bash
spectre config paths
```

**Validate config:**
```bash
spectre config check
```

**Show effective config:**
```bash
spectre config show
```

### Environment Variables Ignored

**Cause:** Config file takes precedence

**Solution:** Remove conflicting setting from config file or use CLI override:
```bash
spectre --log-level debug scan ...
```

---

## Docker Issues

### Container Can't Scan

**Error:**
```
Error: Operation not permitted
```

**Solution:**
```bash
docker run --net=host --cap-add=NET_RAW ghcr.io/doublegate/spectre:latest scan -sS target
```

### Container Can't See Network

**Cause:** Bridge networking isolates container

**Solution:**
```bash
# Use host networking
docker run --net=host ...

# Or use macvlan for dedicated IP
```

---

## Getting Help

### Collect Debug Information

```bash
# Version info
spectre --version

# System info
uname -a
rustc --version

# Configuration
spectre config show

# Run with debug logging
RUST_LOG=debug spectre scan -sS target 2>&1 | tee debug.log
```

### Report Issues

Include in bug reports:
1. SPECTRE version
2. Operating system and version
3. Steps to reproduce
4. Expected vs actual behavior
5. Debug logs (sanitized of sensitive data)
6. Configuration (sanitized)

File at: https://github.com/doublegate/SPECTRE/issues
