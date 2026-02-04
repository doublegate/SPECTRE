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
