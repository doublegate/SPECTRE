# Frequently Asked Questions

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## General

### What is SPECTRE?

SPECTRE (Security Platform for Encrypted Comms, Testing, Enumeration, Recon) is a unified offensive security toolkit that integrates network scanning (ProRT-IP), data analysis (CyberChef-MCP), and secure communications (WRAITH-Protocol).

### Is SPECTRE free?

SPECTRE CLI is released under the MIT license. Component licenses vary:
- SPECTRE CLI: MIT
- WRAITH-Protocol: MIT
- ProRT-IP: GPLv3
- CyberChef-MCP: Apache 2.0

### What platforms are supported?

- Linux (primary, best performance)
- macOS
- Windows (with Npcap)

### Do I need root access?

- **SYN scans:** Yes (or CAP_NET_RAW capability)
- **Connect scans:** No
- **CyberChef operations:** No
- **WRAITH communications:** No

---

## Installation

### How do I install SPECTRE?

See the [Installation Guide](../deployment/INSTALLATION.md) for detailed instructions. Quick start:

```bash
# Linux
curl -LO https://github.com/doublegate/SPECTRE/releases/latest/download/spectre-linux-x86_64.tar.gz
tar xzf spectre-linux-x86_64.tar.gz
sudo mv spectre /usr/local/bin/
```

### How do I update SPECTRE?

```bash
# Download and replace binary
curl -LO https://github.com/doublegate/SPECTRE/releases/latest/download/spectre-linux-x86_64.tar.gz
tar xzf spectre-linux-x86_64.tar.gz
sudo mv spectre /usr/local/bin/
```

### What are the system requirements?

- **OS:** Linux (kernel 5.4+ for best performance), macOS 12+, Windows 10+
- **RAM:** 512 MB minimum, 2 GB recommended
- **Disk:** 100 MB for binaries, more for campaign data
- **Network:** libpcap (Linux/macOS) or Npcap (Windows)

---

## Scanning

### Why do I get "Permission denied"?

SYN scans require raw socket access. Solutions:

```bash
# Option 1: Run as root
sudo spectre scan -sS target

# Option 2: Set capabilities (Linux)
sudo setcap cap_net_raw+ep $(which spectre)

# Option 3: Use connect scan (no privileges needed)
spectre scan -sT target
```

### How fast can SPECTRE scan?

With AF_XDP enabled on Linux:
- 10+ million packets per second
- Scan a /16 network in under 1 minute

Default configuration is more conservative for reliability.

### Is scanning legal?

Only scan systems you own or have explicit written authorization to test. Unauthorized scanning may violate computer crime laws.

### How do I scan quietly?

```bash
# Slow timing
spectre scan -sS -T1 target

# Low rate
spectre scan -sS --rate 10 target

# Fragmentation
spectre scan -sS -f target
```

---

## CyberChef

### How do I start CyberChef?

SPECTRE auto-starts CyberChef-MCP when needed. Manual setup:

```bash
# Docker (recommended)
docker pull ghcr.io/doublegate/cyberchef-mcp:latest
spectre chef setup

# Verify
spectre chef --health
```

### How do I find operations?

```bash
spectre chef --list
spectre chef --list | grep base64
spectre chef --describe From_Base64
```

### Can I use CyberChef recipes from the web?

Yes, export recipes from the CyberChef web app and import:

```bash
spectre chef recipe import --url "https://gchq.github.io/CyberChef/#recipe=..."
```

---

## WRAITH Communications

### Do I need WRAITH for basic scanning?

No. WRAITH is optional and used for:
- Secure data exfiltration
- Team communication
- C2 channels

### How secure is WRAITH?

WRAITH uses:
- XChaCha20-Poly1305 encryption
- X25519 key exchange
- Double Ratchet for forward secrecy
- Optional post-quantum hybrid mode

### Can WRAITH be detected?

WRAITH supports protocol mimicry to appear as:
- HTTPS/TLS 1.3
- DNS over HTTPS
- HTTP/2
- WebSocket

---

## Performance

### How do I improve scan speed?

```bash
# Enable AF_XDP (Linux 5.4+)
spectre scan -sS --af-xdp target

# Increase rate
spectre scan -sS --rate 10000 target

# Aggressive timing
spectre scan -sS -T4 target

# Disable service detection for speed
spectre scan -sS target  # No -sV
```

### How do I reduce memory usage?

```bash
# Stream results instead of buffering
spectre scan -sS --stream target

# Limit concurrent connections
spectre scan -sS --max-concurrent 10 target
```

---

## Troubleshooting

### "Connection refused" on all ports

The host may be:
- Behind a firewall
- Down/offline
- Blocking your source IP

Try:
```bash
spectre scan -Pn target  # Skip host discovery
```

### CyberChef operations failing

```bash
# Check health
spectre chef --health

# Restart container
docker restart spectre-cyberchef

# Check logs
docker logs spectre-cyberchef
```

### Scans timing out

```bash
# Increase timeout
spectre scan -sS --timeout 10000 target

# Reduce rate
spectre scan -sS --rate 100 target
```

---

## Contributing

### How can I contribute?

See [CONTRIBUTING.md](../../CONTRIBUTING.md):
1. Fork the repository
2. Create a feature branch
3. Make changes with tests
4. Submit a pull request

### How do I report bugs?

Open an issue at https://github.com/doublegate/SPECTRE/issues with:
- SPECTRE version
- Operating system
- Steps to reproduce
- Expected vs actual behavior
- Relevant logs

### How do I request features?

Open a feature request issue with:
- Problem description
- Proposed solution
- Use cases

---

## Contact

### Where can I get help?

- **Documentation:** This docs folder
- **Issues:** GitHub Issues
- **Discussions:** GitHub Discussions

### Is there commercial support?

Contact the maintainers for enterprise support options.
