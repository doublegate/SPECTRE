---
name: Bug Report
about: Report a bug to help us improve SPECTRE
title: '[BUG] '
labels: bug, triage
assignees: ''
---

## Description

A clear and concise description of the bug.

## Steps to Reproduce

1. Run command '...'
2. With configuration '...'
3. Against target '...'
4. See error

## Expected Behavior

What you expected to happen.

## Actual Behavior

What actually happened.

## Environment

- **SPECTRE Version:** `spectre --version`
- **OS:** (e.g., Ubuntu 24.04, Fedora 41, macOS 15)
- **Kernel:** `uname -r`
- **Rust Version:** `rustc --version`
- **Installation Method:** (binary, cargo install, source)

## Logs

<details>
<summary>Error Output</summary>

```
Paste error messages here
```

</details>

<details>
<summary>Debug Logs</summary>

```
Run with RUST_LOG=debug and paste output here
```

</details>

## Configuration

<details>
<summary>spectre.toml (sanitized)</summary>

```toml
# Paste relevant config (remove sensitive values)
```

</details>

## Additional Context

- Were you scanning a specific target type?
- Did this work in a previous version?
- Any recent system changes?

## Checklist

- [ ] I have searched existing issues for duplicates
- [ ] I have included all relevant information
- [ ] I have removed any sensitive data from logs/config
- [ ] I can reproduce this issue consistently
