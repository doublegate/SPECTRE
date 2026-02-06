# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take security seriously. If you discover a security vulnerability in SPECTRE, please report it responsibly.

### Do NOT

- Open a public GitHub issue
- Discuss the vulnerability publicly before it's fixed
- Exploit the vulnerability beyond what's necessary to demonstrate it

### Do

- Report the vulnerability privately
- Provide sufficient detail to reproduce the issue
- Allow reasonable time for us to address the issue

### How to Report

**Email:** security@doublegate.dev (placeholder - update with real contact)

**Encrypted Reports:** For sensitive vulnerabilities, use our PGP key:
- Key ID: `(To be published)`
- Fingerprint: `(To be published)`

### What to Include

1. **Description:** Clear description of the vulnerability
2. **Impact:** Potential security impact
3. **Reproduction:** Steps to reproduce the issue
4. **Environment:** Affected versions, OS, configuration
5. **Mitigation:** Any known workarounds
6. **Proof of Concept:** If applicable (minimal, non-destructive)

### Response Timeline

| Phase | Timeline |
|-------|----------|
| Acknowledgment | Within 48 hours |
| Initial Assessment | Within 7 days |
| Status Update | Every 7 days |
| Fix Development | Depends on severity |
| Public Disclosure | After fix is released |

### Severity Classification

| Severity | Description | Target Fix Time |
|----------|-------------|-----------------|
| Critical | Remote code execution, data breach | 24-72 hours |
| High | Privilege escalation, authentication bypass | 7 days |
| Medium | Information disclosure, denial of service | 30 days |
| Low | Minor issues, hardening recommendations | Next release |

### Safe Harbor

We consider security research conducted in accordance with this policy to be:
- Authorized concerning any applicable anti-hacking laws
- Exempt from restrictions in our Terms of Service that would interfere with conducting security research

We will not pursue legal action against researchers who:
- Act in good faith to avoid privacy violations, destruction of data, and interruption of services
- Only interact with accounts you own or with explicit permission
- Do not exploit vulnerabilities beyond what's necessary to confirm them
- Report vulnerabilities promptly

### Recognition

Security researchers who responsibly disclose vulnerabilities may be:
- Credited in release notes (if desired)
- Listed in our security acknowledgments
- Eligible for our bug bounty program (when established)

## Security Best Practices

When using SPECTRE:

### Deployment Security

- Run with minimum required privileges
- Use network isolation where possible
- Enable logging for audit trails
- Regularly update to latest versions

### Configuration Security

- Protect configuration files (`chmod 600`)
- Use environment variables for sensitive values
- Never commit secrets to version control
- Rotate credentials regularly

### Operational Security

- Review scan targets before execution
- Sanitize logs before sharing
- Use secure channels for data transfer
- Follow your organization's security policies

## Known Limitations

### GHSA-wrw7-89jp-8q8g: glib Unsoundness in GUI Application (Linux Only)

**Status:** Accepted Risk (Upstream Dependency Limitation)
**Severity:** Medium
**Affected Component:** spectre-gui (Tauri 2.10.2 on Linux)
**Platforms:** Linux only (macOS and Windows use different rendering engines)

#### Background

The SPECTRE GUI application (via Tauri 2.10.2) depends on GTK3 bindings which transitively require `glib` v0.18.5. This version contains an unsoundness issue ([GHSA-wrw7-89jp-8q8g](https://github.com/advisories/GHSA-wrw7-89jp-8q8g) / [RUSTSEC-2024-0429](https://rustsec.org/advisories/RUSTSEC-2024-0429)) in the `VariantStrIter` iterator implementation that can cause NULL pointer dereferences and crashes.

#### Impact Assessment

- **Exploitability:** LOW - Primarily causes application crashes rather than remote code execution
- **Attack Surface:** The vulnerable code path (`glib::VariantStrIter`) is not directly used by SPECTRE
- **Real-World Risk:** Minimal - requires specific conditions to trigger undefined behavior
- **Platforms:** Only affects Linux builds (macOS uses Cocoa, Windows uses WebView2)

#### Fix Status

The vulnerability is fixed in `glib` v0.20.0+, but upgrading requires:
- Migration from GTK3 to GTK4 ecosystem
- Tauri v3 release (currently in development)
- Full rebuild of webkit2gtk-rs bindings

**Timeline:** Monitoring Tauri v3 release for GTK4 migration (see [Issue #7335](https://github.com/tauri-apps/tauri/issues/7335))

#### Mitigation

Until upstream dependencies migrate to GTK4:
1. **Use CLI/TUI interfaces** for mission-critical operations (not affected)
2. **Test GUI thoroughly** on your target Linux distribution
3. **Report crashes** if you encounter NULL pointer dereferences
4. **Monitor updates** - subscribe to SPECTRE releases for Tauri v3 upgrade

#### References

- [GitHub Advisory GHSA-wrw7-89jp-8q8g](https://github.com/advisories/GHSA-wrw7-89jp-8q8g)
- [RustSec Advisory RUSTSEC-2024-0429](https://rustsec.org/advisories/RUSTSEC-2024-0429)
- [Tauri Issue #12048](https://github.com/tauri-apps/tauri/issues/12048)
- [gtk-rs Fix PR #1343](https://github.com/gtk-rs/gtk-rs-core/pull/1343)

## Scope

This security policy covers:

- SPECTRE CLI and core library
- SPECTRE TUI interface
- SPECTRE GUI application
- SPECTRE MCP server
- Official documentation

### Out of Scope

- Third-party integrations (report to respective maintainers)
- WRAITH-Protocol (see its own security policy)
- ProRT-IP (see its own security policy)
- CyberChef-MCP (see its own security policy)

## Security Updates

Security advisories will be published via:
- GitHub Security Advisories
- Release notes
- Project mailing list (when established)

Subscribe to releases to receive security notifications:
```
gh repo subscribe doublegate/SPECTRE --watch
```

---

Thank you for helping keep SPECTRE and its users safe.
