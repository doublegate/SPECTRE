# Threat Model

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## Overview

This document describes the threat model for SPECTRE, identifying potential attack vectors, trust boundaries, and mitigations.

---

## Assets

### Primary Assets

| Asset | Description | Sensitivity |
|-------|-------------|-------------|
| Scan Results | Network reconnaissance data | High |
| Campaign Data | Target lists, findings, reports | High |
| Configuration | API keys, credentials | Critical |
| WRAITH Keys | Encryption keys, identities | Critical |
| Plugin Code | Custom extensions | Medium |

### Secondary Assets

| Asset | Description | Sensitivity |
|-------|-------------|-------------|
| Logs | Operation logs | Medium |
| Cached Data | Temporary scan data | Low |
| Metrics | Performance data | Low |

---

## Trust Boundaries

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           UNTRUSTED                                      │
│                                                                          │
│   ┌──────────────┐    ┌──────────────┐    ┌──────────────┐             │
│   │   Network    │    │   External   │    │   Plugins    │             │
│   │   Targets    │    │   APIs       │    │   (User)     │             │
│   └──────┬───────┘    └──────┬───────┘    └──────┬───────┘             │
│          │                   │                   │                      │
└──────────┼───────────────────┼───────────────────┼──────────────────────┘
           │                   │                   │
      ═════╪═══════════════════╪═══════════════════╪═════  TRUST BOUNDARY
           │                   │                   │
┌──────────┼───────────────────┼───────────────────┼──────────────────────┐
│          ▼                   ▼                   ▼                      │
│   ┌─────────────────────────────────────────────────────────────────┐  │
│   │                      SPECTRE Core                                │  │
│   │  ┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐      │  │
│   │  │ Scanner │    │  Chef   │    │  Comms  │    │ Plugins │      │  │
│   │  │  (prtip)│    │  (mcp)  │    │(wraith) │    │ (lua)   │      │  │
│   │  └─────────┘    └─────────┘    └─────────┘    └─────────┘      │  │
│   └─────────────────────────────────────────────────────────────────┘  │
│                                │                                        │
│          ══════════════════════╪═════════════════  TRUST BOUNDARY      │
│                                │                                        │
│   ┌────────────────────────────▼────────────────────────────────────┐  │
│   │                     Persistent Storage                           │  │
│   │    ┌──────────┐    ┌──────────┐    ┌──────────┐                │  │
│   │    │  Config  │    │  Keys    │    │   Data   │                │  │
│   │    └──────────┘    └──────────┘    └──────────┘                │  │
│   └─────────────────────────────────────────────────────────────────┘  │
│                           TRUSTED                                       │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Threat Categories

### T1: Input Injection

**Threats:**
- Command injection via target specifications
- Path traversal in file operations
- Lua injection in plugin system
- Recipe injection in CyberChef operations

**Mitigations:**
- Input validation for all user input
- Allowlist-based target validation
- Sandboxed Lua execution
- Recipe schema validation

### T2: Unauthorized Access

**Threats:**
- API key disclosure
- WRAITH key compromise
- Configuration file exposure
- Unauthorized MCP access

**Mitigations:**
- Secure key storage (OS keyring)
- File permission enforcement (600)
- API authentication required
- MCP authorization checks

### T3: Data Exposure

**Threats:**
- Scan results in logs
- Credentials in configuration
- Sensitive data in error messages
- Cache leakage

**Mitigations:**
- Log sanitization
- Secret redaction
- Generic error messages
- Secure cache clearing

### T4: Network Attacks

**Threats:**
- Man-in-the-middle on WRAITH channels
- DNS spoofing
- Target impersonation
- Traffic analysis

**Mitigations:**
- End-to-end encryption
- Certificate pinning
- Target verification
- Traffic obfuscation

### T5: Plugin Security

**Threats:**
- Malicious plugin code
- Resource exhaustion
- Sandbox escape
- Data exfiltration

**Mitigations:**
- Permission model
- Resource limits
- Sandbox hardening
- Network restrictions

### T6: Denial of Service

**Threats:**
- Scan flooding
- Memory exhaustion
- CPU exhaustion
- Disk filling

**Mitigations:**
- Rate limiting
- Memory limits
- Timeout enforcement
- Disk quotas

---

## Attack Scenarios

### Scenario 1: Malicious Target

**Attack:** Attacker hosts a honeypot that sends crafted responses to exploit parsing vulnerabilities.

**Risk:** Medium

**Mitigations:**
1. Fuzz testing of all parsers
2. Memory-safe parsing (Rust)
3. Input length limits
4. Timeout on responses

### Scenario 2: Compromised Plugin

**Attack:** User installs malicious plugin that exfiltrates scan data.

**Risk:** High

**Mitigations:**
1. Plugin signature verification
2. Network permission required
3. Data access auditing
4. Plugin source review

### Scenario 3: API Key Theft

**Attack:** Attacker gains access to API key through log exposure.

**Risk:** High

**Mitigations:**
1. Key rotation capability
2. Key scoping (limited permissions)
3. Usage monitoring
4. Log sanitization

### Scenario 4: WRAITH Channel Compromise

**Attack:** Attacker compromises WRAITH key material.

**Risk:** Critical

**Mitigations:**
1. Key derivation with ratcheting
2. Forward secrecy
3. Key compromise detection
4. Emergency key revocation

---

## Security Controls

### Authentication

| Control | Implementation |
|---------|----------------|
| CLI Auth | Local user context |
| API Auth | API key (header or env) |
| MCP Auth | Token-based |
| WRAITH Auth | Public key + identity |

### Authorization

| Resource | Access Control |
|----------|----------------|
| Scans | User ownership |
| Campaigns | User ownership |
| Plugins | Permission manifest |
| WRAITH Channels | Peer authorization |

### Encryption

| Data | Encryption |
|------|------------|
| At Rest | OS-level (keyring) |
| In Transit | TLS 1.3+ |
| WRAITH | XChaCha20-Poly1305 |
| Keys | Argon2id derivation |

### Auditing

| Event | Logged |
|-------|--------|
| Scan start/stop | Yes |
| API access | Yes |
| Config changes | Yes |
| Plugin load | Yes |
| WRAITH connections | Yes |

---

## Residual Risks

| Risk | Likelihood | Impact | Acceptance |
|------|------------|--------|------------|
| Zero-day in dependency | Low | High | Accepted with monitoring |
| Side-channel attacks | Low | Medium | Accepted |
| Social engineering | Medium | High | User education |
| Physical access | Low | Critical | Out of scope |

---

## Security Updates

- Subscribe to security advisories
- Monitor dependency vulnerabilities
- Regular security audits
- Penetration testing (annual)
