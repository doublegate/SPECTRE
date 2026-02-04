# Encryption and Cryptography

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## Overview

SPECTRE leverages WRAITH-Protocol's cryptographic stack for secure communications and implements additional encryption for local data protection.

---

## Cryptographic Primitives

### Symmetric Encryption

| Algorithm | Use Case | Key Size |
|-----------|----------|----------|
| XChaCha20-Poly1305 | Message encryption | 256-bit |
| AES-256-GCM | Alternative (hardware accel) | 256-bit |

**Selection Rationale:**
- XChaCha20: Extended nonce (192-bit) prevents nonce reuse
- Poly1305: Fast, constant-time authentication
- ChaCha20: Better on systems without AES-NI

### Asymmetric Encryption

| Algorithm | Use Case | Key Size |
|-----------|----------|----------|
| X25519 | Key exchange | 256-bit |
| Ed25519 | Signatures | 256-bit |
| ML-KEM-768 | Post-quantum KEM | 2400-bit |

**Selection Rationale:**
- Curve25519: Widely vetted, efficient
- Post-quantum: Future-proof hybrid mode

### Hash Functions

| Algorithm | Use Case |
|-----------|----------|
| BLAKE3 | Fast hashing, file integrity |
| SHA-256 | Compatibility, checksums |
| SHA-512 | Extended security margin |
| Argon2id | Password/key derivation |

---

## Key Management

### Key Hierarchy

```
Master Key (User Password + Argon2id)
    │
    ├── Identity Key (Ed25519)
    │   └── Signing operations
    │
    ├── Exchange Key (X25519)
    │   └── Key agreement
    │
    ├── Storage Key (XChaCha20)
    │   └── Local data encryption
    │
    └── Session Keys (Derived per-connection)
        └── Message encryption
```

### Key Derivation

```rust
// Master key from password
let master = Argon2id::hash(
    password,
    salt,
    Params {
        m_cost: 65536,  // 64 MB
        t_cost: 3,      // 3 iterations
        p_cost: 4,      // 4 parallel lanes
    }
)?;

// Derive subkeys
let identity_key = HKDF::expand(master, b"spectre-identity");
let exchange_key = HKDF::expand(master, b"spectre-exchange");
let storage_key = HKDF::expand(master, b"spectre-storage");
```

### Key Storage

| Platform | Storage |
|----------|---------|
| Linux | libsecret (GNOME Keyring) |
| macOS | Keychain |
| Windows | Credential Manager |
| Fallback | Encrypted file (~/.spectre/keys.enc) |

---

## Communication Security

### WRAITH Protocol Stack

```
Application Data
       │
       ▼
┌─────────────────┐
│  Double Ratchet │ (Forward secrecy)
└────────┬────────┘
         │
┌────────▼────────┐
│   Noise_XX      │ (Handshake)
└────────┬────────┘
         │
┌────────▼────────┐
│ XChaCha20-Poly  │ (Encryption)
└────────┬────────┘
         │
┌────────▼────────┐
│  Transport      │ (TCP/UDP)
└─────────────────┘
```

### Session Establishment

1. **X3DH Key Agreement:**
   - Identity keys exchanged
   - Ephemeral keys for forward secrecy
   - Prekeys for async initiation

2. **Noise_XX Handshake:**
   - Mutual authentication
   - Key confirmation
   - Transcript binding

3. **Double Ratchet:**
   - Symmetric ratchet (every message)
   - DH ratchet (periodically)
   - Message keys: send/receive chains

### Forward Secrecy

- Ephemeral keys generated per session
- DH ratchet updates on message direction change
- Compromised key reveals only future messages
- Past messages remain protected

### Post-Compromise Security

- Ratchet automatically heals
- New DH keys restore security
- Attacker must maintain active presence

---

## Data at Rest

### Configuration Encryption

```toml
# spectre.toml (sensitive fields encrypted)
[api]
key = "ENC:v1:nonce:ciphertext"

[wraith]
identity = "ENC:v1:nonce:ciphertext"
```

### Database Encryption

- SQLCipher for local database
- 256-bit AES encryption
- Key derived from master key
- Per-page encryption

### Scan Results

- Optional encryption for stored results
- Campaign-level encryption keys
- Secure deletion on cleanup

---

## Implementation Notes

### Constant-Time Operations

All security-critical operations use constant-time implementations:

```rust
use subtle::{ConstantTimeEq, Choice};

// Constant-time comparison
fn verify_mac(expected: &[u8], actual: &[u8]) -> bool {
    expected.ct_eq(actual).into()
}
```

### Secure Memory

- Sensitive data cleared on drop
- Memory locking for keys (mlock)
- No swapping of key material

```rust
use zeroize::Zeroize;

struct SecretKey {
    bytes: [u8; 32],
}

impl Drop for SecretKey {
    fn drop(&mut self) {
        self.bytes.zeroize();
    }
}
```

### Random Number Generation

- System CSPRNG (getrandom)
- Fallback to /dev/urandom
- Entropy health checks

---

## Compliance

### Standards

| Standard | Coverage |
|----------|----------|
| NIST SP 800-38D | AES-GCM |
| NIST SP 800-56A | Key Agreement |
| NIST SP 800-108 | KDF |
| RFC 8439 | ChaCha20-Poly1305 |
| RFC 7748 | X25519 |
| RFC 8032 | Ed25519 |

### Certifications

SPECTRE's cryptographic implementations use certified libraries:
- `ring`: BoringSSL base
- `dalek`: Formally verified components
- `blake3`: Official implementation

---

## Security Considerations

### Do NOT

- Reuse nonces (XChaCha20 mitigates but don't)
- Store keys in plaintext
- Log key material
- Use ECB mode
- Implement custom crypto

### Do

- Use authenticated encryption
- Rotate keys periodically
- Validate all crypto inputs
- Use established libraries
- Enable hardware acceleration
