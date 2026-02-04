# ProRT-IP WarScan Integration

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## Overview

ProRT-IP WarScan provides SPECTRE's network reconnaissance capabilities, enabling high-performance scanning, service detection, and OS fingerprinting for red team operations.

**Component Version:** v1.0.0
**Repository:** [github.com/doublegate/ProRT-IP](https://github.com/doublegate/ProRT-IP)

---

## Capabilities

| Feature | Description |
|---------|-------------|
| **Wire-Speed Scanning** | 10M+ packets/second with AF_XDP kernel bypass |
| **Scan Types** | SYN, Connect, FIN, NULL, Xmas, ACK, UDP, Idle/Zombie |
| **Service Detection** | 1000+ service signatures with version detection |
| **OS Fingerprinting** | TCP/IP stack analysis for OS identification |
| **60 FPS TUI** | Real-time visualization during scans |
| **Lua Plugins** | Extensible probe and analysis system |

---

## Integration Architecture

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                       SPECTRE ↔ ProRT-IP Integration                        │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌────────────────────────────────────────────────────────────────────────┐ │
│  │                        SPECTRE Core                                     │ │
│  │                                                                         │ │
│  │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────────┐ │ │
│  │  │   Scan Manager  │  │   Target Queue  │  │   Results Aggregator    │ │ │
│  │  │                 │  │                 │  │                         │ │ │
│  │  │  • Job creation │  │  • CIDR parsing │  │  • Stream processing    │ │ │
│  │  │  • Scheduling   │  │  • Prioritization│  │  • Deduplication       │ │ │
│  │  │  • Monitoring   │  │  • Rate limiting │  │  • Export formats      │ │ │
│  │  └────────┬────────┘  └────────┬────────┘  └────────────┬────────────┘ │ │
│  │           └───────────────────┬┴────────────────────────┘              │ │
│  └───────────────────────────────┼────────────────────────────────────────┘ │
│                                  │                                          │
│  ┌───────────────────────────────▼────────────────────────────────────────┐ │
│  │                    ProRT-IP Integration Layer                           │ │
│  │                                                                         │ │
│  │  ┌────────────────────────────────────────────────────────────────────┐ │ │
│  │  │                    SpectreScanner API                               │ │ │
│  │  │                                                                    │ │ │
│  │  │   fn scan_syn(targets, ports, opts) -> ScanResults                 │ │ │
│  │  │   fn scan_connect(targets, ports, opts) -> ScanResults             │ │ │
│  │  │   fn detect_services(hosts) -> Vec<ServiceInfo>                    │ │ │
│  │  │   fn detect_os(hosts) -> Vec<OsInfo>                               │ │ │
│  │  │   fn scan_stream(targets, opts) -> impl Stream<ScanResult>         │ │ │
│  │  └────────────────────────────────────────────────────────────────────┘ │ │
│  └───────────────────────────────┬────────────────────────────────────────┘ │
│                                  │                                          │
│  ┌───────────────────────────────▼────────────────────────────────────────┐ │
│  │                      ProRT-IP Engine Stack                              │ │
│  │                                                                         │ │
│  │  ┌───────────────┐  ┌───────────────┐  ┌───────────────────────────┐   │ │
│  │  │  prtip-core   │  │  prtip-scan   │  │      prtip-detect         │   │ │
│  │  │               │  │               │  │                           │   │ │
│  │  │  • Engine     │  │  • SYN/ACK    │  │  • Service probes         │   │ │
│  │  │  • Scheduler  │  │  • UDP        │  │  • OS fingerprints        │   │ │
│  │  │  • Results    │  │  • Stealth    │  │  • Banner grabbing        │   │ │
│  │  └───────────────┘  └───────────────┘  └───────────────────────────┘   │ │
│  │                                                                         │ │
│  │  ┌───────────────┐  ┌───────────────┐  ┌───────────────────────────┐   │ │
│  │  │ prtip-packet  │  │  prtip-net    │  │       prtip-lua           │   │ │
│  │  │               │  │               │  │                           │   │ │
│  │  │  • Crafting   │  │  • AF_XDP     │  │  • Custom probes          │   │ │
│  │  │  • Parsing    │  │  • io_uring   │  │  • Analysis scripts       │   │ │
│  │  │  • Checksums  │  │  • Raw sockets│  │  • Result processing      │   │ │
│  │  └───────────────┘  └───────────────┘  └───────────────────────────┘   │ │
│  └─────────────────────────────────────────────────────────────────────────┘ │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## API Reference

### SpectreScanner

Main interface for ProRT-IP scanning within SPECTRE.

```rust
use spectre_scanner::{SpectreScanner, ScanConfig};

// Initialize scanner
let scanner = SpectreScanner::new(ScanConfig {
    interface: Some("eth0".into()),
    rate_limit: Some(10000),  // packets per second
    ..Default::default()
})?;
```

### SYN Scan

```rust
use spectre_scanner::{SynScanOptions, PortSpec};

let results = scanner.scan_syn(
    &["192.168.1.0/24"],
    PortSpec::Range(1, 1000),
    SynScanOptions {
        service_detection: true,
        os_detection: false,
        timeout: Duration::from_secs(3),
        retries: 2,
        ..Default::default()
    },
).await?;

for host in results.hosts {
    println!("{}: {} open ports", host.ip, host.open_ports.len());
    for port in host.open_ports {
        println!("  {} - {}", port.number, port.service.unwrap_or_default());
    }
}
```

### Connect Scan (Unprivileged)

```rust
use spectre_scanner::ConnectScanOptions;

// Full TCP connect - no root required
let results = scanner.scan_connect(
    &["192.168.1.1"],
    PortSpec::List(vec![22, 80, 443, 8080]),
    ConnectScanOptions {
        timeout: Duration::from_secs(5),
        concurrent_connections: 100,
        ..Default::default()
    },
).await?;
```

### Stealth Scans

```rust
// FIN scan
let results = scanner.scan_fin(&targets, ports, FinScanOptions::default()).await?;

// NULL scan
let results = scanner.scan_null(&targets, ports, NullScanOptions::default()).await?;

// Xmas scan
let results = scanner.scan_xmas(&targets, ports, XmasScanOptions::default()).await?;
```

### ACK Scan (Firewall Mapping)

```rust
use spectre_scanner::AckScanOptions;

let results = scanner.scan_ack(
    &["firewall.example.com"],
    PortSpec::Range(1, 1024),
    AckScanOptions::default(),
).await?;

// Identify filtered vs unfiltered ports
for port in results.ports {
    match port.state {
        PortState::Unfiltered => println!("{}: unfiltered", port.number),
        PortState::Filtered => println!("{}: filtered", port.number),
    }
}
```

### UDP Scan

```rust
use spectre_scanner::UdpScanOptions;

let results = scanner.scan_udp(
    &["192.168.1.1"],
    PortSpec::List(vec![53, 161, 123, 500]),
    UdpScanOptions {
        protocol_probes: true,  // Use protocol-specific probes
        ..Default::default()
    },
).await?;
```

### Idle/Zombie Scan

```rust
use spectre_scanner::IdleScanOptions;

let results = scanner.scan_idle(
    &["192.168.1.1"],
    "192.168.1.50",  // Zombie host
    PortSpec::List(vec![80, 443]),
    IdleScanOptions::default(),
).await?;
```

### Service Detection

```rust
use spectre_scanner::{ServiceDetection, DetectionIntensity};

let services = scanner.detect_services(
    &hosts_with_open_ports,
    ServiceDetection {
        intensity: DetectionIntensity::Normal,  // 0-9 scale
        version_all: true,
        ..Default::default()
    },
).await?;

for svc in services {
    println!("{}:{} - {} {} ({})",
        svc.host, svc.port,
        svc.name, svc.version.unwrap_or_default(),
        svc.confidence
    );
}
```

### OS Detection

```rust
use spectre_scanner::OsDetection;

let os_results = scanner.detect_os(
    &["192.168.1.1", "192.168.1.2"],
    OsDetection {
        aggressive: false,
        ..Default::default()
    },
).await?;

for os in os_results {
    println!("{}: {} ({}% confidence)",
        os.host,
        os.best_match.name,
        os.best_match.accuracy
    );
}
```

### Streaming Results

```rust
use futures::StreamExt;

let mut stream = scanner.scan_stream(
    &["192.168.1.0/24"],
    PortSpec::Range(1, 1000),
    StreamScanOptions::default(),
);

while let Some(result) = stream.next().await {
    match result {
        ScanEvent::HostDiscovered(host) => {
            println!("Found host: {}", host.ip);
        }
        ScanEvent::PortOpen(host, port) => {
            println!("{}:{} open", host, port);
        }
        ScanEvent::ServiceDetected(host, port, service) => {
            println!("{}:{} - {}", host, port, service);
        }
        ScanEvent::Progress(pct) => {
            // Update progress indicator
        }
        ScanEvent::Complete => break,
    }
}
```

---

## Configuration

### SPECTRE Config (spectre.toml)

```toml
[scan]
# Default scan rate (packets per second)
default_rate = 1000

# Default timeout for probes (milliseconds)
default_timeout = 3000

# Number of retries for failed probes
default_retries = 2

# Preferred interface
interface = "eth0"

# Enable AF_XDP kernel bypass (requires Linux 5.4+)
af_xdp = false

# Default timing template (0-5)
timing_template = 3

[scan.detection]
# Service detection intensity (0-9)
service_intensity = 5

# Enable OS detection by default
os_detection = false

# Banner grab timeout (milliseconds)
banner_timeout = 5000

[scan.evasion]
# Default fragmentation
fragment = false
mtu = 0

# Default TTL (0 = system default)
ttl = 0

# Random data padding (bytes)
data_length = 0

[scan.output]
# Default output format
format = "json"

# Include raw packets in output
include_packets = false

# PCAP capture during scans
capture_pcap = false
```

### Environment Variables

| Variable | Description |
|----------|-------------|
| `SPECTRE_SCAN_RATE` | Override default scan rate |
| `SPECTRE_SCAN_TIMEOUT` | Override default timeout |
| `SPECTRE_SCAN_INTERFACE` | Override interface |
| `PRTIP_AF_XDP` | Enable AF_XDP bypass |
| `PRTIP_VERBOSE` | ProRT-IP verbosity level |

---

## Timing Templates

SPECTRE supports nmap-compatible timing templates:

| Template | Name | Delay | Use Case |
|----------|------|-------|----------|
| T0 | Paranoid | 5 min | IDS evasion, extremely slow |
| T1 | Sneaky | 15 sec | IDS evasion |
| T2 | Polite | 400 ms | Reduced bandwidth usage |
| T3 | Normal | Default | Standard scanning |
| T4 | Aggressive | 10 ms | Fast, reliable networks |
| T5 | Insane | 5 ms | Very fast networks |

```rust
use spectre_scanner::TimingTemplate;

let opts = SynScanOptions {
    timing: TimingTemplate::Aggressive,
    ..Default::default()
};
```

---

## Evasion Techniques

### Packet Fragmentation

```rust
let opts = SynScanOptions {
    fragment: true,
    mtu: Some(8),  // 8-byte fragments
    ..Default::default()
};
```

### Decoy Scanning

```rust
use spectre_scanner::Decoy;

let opts = SynScanOptions {
    decoys: vec![
        Decoy::Random,
        Decoy::Random,
        Decoy::Me,  // Your actual IP
        Decoy::Random,
        Decoy::Random,
    ],
    ..Default::default()
};
```

### Source Spoofing

```rust
let opts = SynScanOptions {
    source_ip: Some("10.0.0.1".parse()?),
    source_port: Some(53),  // Appear as DNS
    ..Default::default()
};
```

### TTL Manipulation

```rust
let opts = SynScanOptions {
    ttl: Some(64),  // Common Linux TTL
    ..Default::default()
};
```

### Bad Checksums

```rust
let opts = SynScanOptions {
    bad_checksum: true,  // Bypass some firewalls
    ..Default::default()
};
```

---

## Performance Tuning

### High-Speed Scanning

```toml
[scan]
# Enable AF_XDP for kernel bypass
af_xdp = true

# Maximum rate
default_rate = 1000000  # 1M pps

# Reduce timeouts
default_timeout = 1000

# Single retry
default_retries = 1

[scan.detection]
# Disable service detection for speed
service_intensity = 0
os_detection = false
```

### Low-Impact Scanning

```toml
[scan]
# Conservative rate
default_rate = 100

# Longer timeouts
default_timeout = 10000

# More retries
default_retries = 3

# Polite timing
timing_template = 2
```

### Balanced Scanning

```toml
[scan]
default_rate = 5000
default_timeout = 3000
default_retries = 2
timing_template = 3

[scan.detection]
service_intensity = 5
os_detection = true
```

---

## Lua Plugins

ProRT-IP supports Lua plugins for custom probes and analysis.

### Custom Service Probe

```lua
-- ~/.spectre/plugins/probes/custom-service.lua
local probe = {}

probe.name = "custom-webapp"
probe.ports = {8000, 8001, 8002}
probe.protocol = "tcp"

function probe.send()
    return "GET /api/version HTTP/1.0\r\n\r\n"
end

function probe.match(response)
    local version = response:match("X-App-Version: ([%d.]+)")
    if version then
        return {
            name = "CustomWebApp",
            version = version,
            confidence = 90
        }
    end
    return nil
end

return probe
```

### Custom Analysis Script

```lua
-- ~/.spectre/plugins/analysis/find-vulns.lua
local analysis = {}

analysis.name = "vulnerability-checker"

function analysis.process(results)
    local findings = {}

    for _, host in ipairs(results.hosts) do
        for _, port in ipairs(host.open_ports) do
            -- Check for known vulnerable versions
            if port.service == "ssh" and port.version then
                if version_lt(port.version, "8.0") then
                    table.insert(findings, {
                        host = host.ip,
                        port = port.number,
                        severity = "medium",
                        description = "Outdated SSH version: " .. port.version
                    })
                end
            end
        end
    end

    return findings
end

return analysis
```

### Loading Plugins

```toml
[scan.plugins]
# Plugin directories
paths = [
    "~/.spectre/plugins",
    "/usr/share/spectre/plugins"
]

# Enabled plugins
enabled = [
    "custom-service",
    "find-vulns"
]
```

---

## Output Formats

### JSON Output

```rust
let results = scanner.scan_syn(&targets, ports, opts).await?;
let json = results.to_json()?;
```

```json
{
  "scan_info": {
    "type": "syn",
    "start_time": "2026-02-04T10:30:00Z",
    "end_time": "2026-02-04T10:35:00Z",
    "targets": ["192.168.1.0/24"],
    "ports": "1-1000"
  },
  "hosts": [
    {
      "ip": "192.168.1.10",
      "hostname": "webserver.local",
      "status": "up",
      "open_ports": [
        {
          "number": 22,
          "protocol": "tcp",
          "state": "open",
          "service": {
            "name": "ssh",
            "version": "OpenSSH 8.9p1",
            "product": "OpenSSH",
            "extra_info": "Ubuntu Linux"
          }
        },
        {
          "number": 80,
          "protocol": "tcp",
          "state": "open",
          "service": {
            "name": "http",
            "version": "1.18.0",
            "product": "nginx"
          }
        }
      ],
      "os": {
        "name": "Linux 5.x",
        "accuracy": 95,
        "family": "Linux",
        "vendor": "Linux"
      }
    }
  ],
  "statistics": {
    "hosts_scanned": 254,
    "hosts_up": 15,
    "ports_scanned": 254000,
    "ports_open": 47,
    "duration_seconds": 300
  }
}
```

### Nmap XML Compatibility

```rust
let xml = results.to_nmap_xml()?;
```

ProRT-IP can export results in nmap-compatible XML format for use with existing tools.

### Greppable Output

```rust
let grep = results.to_greppable()?;
```

```
Host: 192.168.1.10 ()    Status: Up
Host: 192.168.1.10 ()    Ports: 22/open/tcp//ssh//OpenSSH 8.9p1/, 80/open/tcp//http//nginx 1.18.0/
```

---

## Troubleshooting

### Permission Denied

```bash
# SYN scan requires CAP_NET_RAW
sudo setcap cap_net_raw+ep $(which spectre)

# Or run as root
sudo spectre scan -sS 192.168.1.0/24

# Or use unprivileged connect scan
spectre scan -sT 192.168.1.0/24
```

### AF_XDP Not Available

```bash
# Check kernel version (need 5.4+)
uname -r

# Check for XDP support
ip link show eth0 | grep xdp

# Fall back to raw sockets
PRTIP_AF_XDP=0 spectre scan -sS ...
```

### Slow Scanning

```bash
# Check current rate
spectre scan --stats ...

# Increase rate
spectre scan --rate 10000 ...

# Use aggressive timing
spectre scan -T4 ...

# Reduce timeout
spectre scan --timeout 1000 ...
```

### Missing Services

```bash
# Increase detection intensity
spectre scan -sV --version-intensity 9 ...

# Increase timeout
spectre scan -sV --timeout 10000 ...

# Check probe coverage
spectre scan --list-probes | grep <service>
```

### Firewall Blocking

```bash
# Try ACK scan to map firewall
spectre scan -sA 192.168.1.1

# Use fragmentation
spectre scan -sS -f ...

# Try different source port
spectre scan -sS -g 53 ...

# Use decoys
spectre scan -sS -D RND:5 ...
```

---

## Security Considerations

### Privilege Requirements

| Scan Type | Privileges Required |
|-----------|---------------------|
| SYN | root or CAP_NET_RAW |
| Connect | None |
| FIN/NULL/Xmas | root or CAP_NET_RAW |
| ACK | root or CAP_NET_RAW |
| UDP | root or CAP_NET_RAW |
| Idle | root or CAP_NET_RAW |

### Rate Limiting

Always use appropriate rate limiting to avoid:
- Network congestion
- IDS/IPS alerts
- Target system impact
- Legal issues

```rust
let opts = SynScanOptions {
    rate_limit: Some(1000),  // Conservative rate
    ..Default::default()
};
```

### Scope Validation

SPECTRE validates targets against authorized scope:

```toml
[scan.scope]
# Only allow scanning these ranges
allowed = [
    "192.168.1.0/24",
    "10.0.0.0/8"
]

# Never scan these
blocked = [
    "192.168.1.1",  # Gateway
    "192.168.1.254" # Critical infrastructure
]
```

---

## References

- [ProRT-IP README](https://github.com/doublegate/ProRT-IP/blob/main/README.md)
- [ProRT-IP Architecture](https://github.com/doublegate/ProRT-IP/blob/main/docs/architecture/)
- [ProRT-IP Scanning Guide](https://github.com/doublegate/ProRT-IP/blob/main/docs/scanning.md)
- [Nmap Reference](https://nmap.org/book/man.html)
