# Tutorial: Setting Up Secure Channels

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## Overview

This tutorial shows how to establish encrypted communication channels using SPECTRE's WRAITH integration.

**Time Required:** 15 minutes

**Prerequisites:**
- SPECTRE installed
- Two systems (for peer communication)

---

## Step 1: Generate Identity

Create your WRAITH identity (one-time setup):

```bash
spectre identity init
```

Output:
```
Generating WRAITH identity...
Identity created: ~/.spectre/identity.key

Your public identity:
  ID:          spectre:abc123def456...
  Fingerprint: A1:B2:C3:D4:E5:F6:...

Share your ID with peers to establish secure channels.
```

---

## Step 2: Exchange Identities

Share your identity with your communication partner:

```bash
# Show your identity
spectre identity show

# Export to file
spectre identity export > my_identity.pub
```

Import a peer's identity:

```bash
# Add peer from ID string
spectre peer add "spectre:xyz789..."

# Or from file
spectre peer add --file peer_identity.pub

# Name the peer
spectre peer add --name "teammate" "spectre:xyz789..."
```

---

## Step 3: Create a Channel

Establish a secure channel with a peer:

```bash
# Create channel
spectre channel create --peer teammate --name ops-channel
```

Output:
```
Creating secure channel with teammate...
Performing key exchange...
Channel established!

Channel: ops-channel
  Peer:   teammate
  Cipher: XChaCha20-Poly1305
  Status: Connected
```

---

## Step 4: Send Messages

Send an encrypted message:

```bash
# Interactive mode
spectre send --channel ops-channel
> This is a secure message
> ^D
Message sent.

# One-liner
echo "Hello teammate" | spectre send --channel ops-channel

# Send a file
spectre send --channel ops-channel --file report.pdf
```

---

## Step 5: Receive Messages

Listen for incoming messages:

```bash
# Receive mode
spectre receive --channel ops-channel

# Save to file
spectre receive --channel ops-channel --output received_files/

# With timeout
spectre receive --channel ops-channel --timeout 60
```

---

## Step 6: Configure Transport

Choose transport protocol:

```bash
# TCP (default, reliable)
spectre channel create --transport tcp --peer teammate

# UDP (faster, for real-time)
spectre channel create --transport udp --peer teammate

# QUIC (modern, multiplexed)
spectre channel create --transport quic --peer teammate
```

---

## Step 7: Protocol Mimicry

Make traffic look like normal protocols:

```bash
# Mimic HTTPS
spectre channel create --peer teammate --mimicry tls13 --sni www.microsoft.com

# Mimic DNS over HTTPS
spectre channel create --peer teammate --mimicry doh

# Mimic HTTP/2
spectre channel create --peer teammate --mimicry http2
```

---

## Channel Configuration

Full channel configuration in `spectre.toml`:

```toml
[wraith]
identity_file = "~/.spectre/identity.key"

[wraith.channel]
cipher = "xchacha20poly1305"
kex = "x25519"
post_quantum = false

[wraith.protocol]
mimicry = "tls13"
sni = "www.example.com"
padding = true

[wraith.transport]
transport = "tcp"
connect_timeout = 10
keepalive = 30
```

---

## Managing Channels

```bash
# List channels
spectre channel list

# Channel details
spectre channel info ops-channel

# Close channel
spectre channel close ops-channel

# Delete channel
spectre channel delete ops-channel
```

---

## Security Features

### Perfect Forward Secrecy

Each message uses unique keys:
```bash
# View ratchet state
spectre channel info ops-channel --verbose
```

### Post-Quantum Security

Enable hybrid mode for quantum resistance:
```bash
spectre channel create --peer teammate --post-quantum
```

### Key Rotation

Manually rotate channel keys:
```bash
spectre channel rotate ops-channel
```

---

## Troubleshooting

### Connection Failed

```bash
# Check peer is online
spectre peer status teammate

# Check network connectivity
spectre peer ping teammate

# Use relay server
spectre channel create --peer teammate --relay relay.example.com
```

### Identity Mismatch

```bash
# Verify peer fingerprint
spectre peer verify teammate

# Remove and re-add peer
spectre peer remove teammate
spectre peer add --name teammate "spectre:xyz789..."
```

---

## Next Steps

- [Operational Security](../security/OPERATIONAL-SECURITY.md) - OPSEC guidelines
- [Campaign Planning](CAMPAIGN-PLANNING.md) - Coordinate multi-phase operations
