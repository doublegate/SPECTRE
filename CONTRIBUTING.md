# Contributing to SPECTRE

Thank you for your interest in contributing to SPECTRE. This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Code Style Guidelines](#code-style-guidelines)
- [Commit Message Conventions](#commit-message-conventions)
- [Pull Request Process](#pull-request-process)
- [Testing Requirements](#testing-requirements)
- [Documentation Requirements](#documentation-requirements)
- [Security Considerations](#security-considerations)

---

## Code of Conduct

This project adheres to the Contributor Covenant Code of Conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to the maintainers.

See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for details.

---

## Getting Started

### Understanding the Project

SPECTRE is a unified offensive security platform integrating three components:

| Component | Repository | Purpose |
|-----------|------------|---------|
| ProRT-IP | [github.com/doublegate/ProRT-IP](https://github.com/doublegate/ProRT-IP) | Network reconnaissance |
| CyberChef-MCP | [github.com/doublegate/CyberChef-MCP](https://github.com/doublegate/CyberChef-MCP) | Data analysis |
| WRAITH-Protocol | [github.com/doublegate/WRAITH-Protocol](https://github.com/doublegate/WRAITH-Protocol) | Secure communications |

Each component has its own repository and contribution guidelines. SPECTRE provides the orchestration layer.

### Ways to Contribute

- **Bug Reports:** Submit detailed bug reports with reproduction steps
- **Feature Requests:** Propose new features or improvements
- **Code Contributions:** Submit pull requests for bug fixes or features
- **Documentation:** Improve or add documentation
- **Testing:** Add test cases or improve test coverage
- **Security:** Report security vulnerabilities responsibly

---

## Development Setup

### Prerequisites

- **Rust 1.88+** (stable toolchain)
- **Node.js 22+** (for CyberChef-MCP)
- **Docker** (recommended for CyberChef-MCP)
- **Git** (for version control)
- **libpcap** (Linux/macOS) or **Npcap** (Windows)
- **Linux kernel 6.2+** (recommended for optimal performance)

### Clone and Setup

```bash
# Clone the repository with submodules
git clone --recursive https://github.com/doublegate/SPECTRE.git
cd SPECTRE

# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable

# Install required components
rustup component add clippy rustfmt

# Install development tools
cargo install cargo-tarpaulin cargo-audit cargo-deny

# Build the project
cargo build --workspace

# Run tests
cargo test --workspace

# Verify setup
./target/debug/spectre --version
```

### IDE Setup

**Visual Studio Code:**

```json
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.check.command": "clippy",
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

**JetBrains RustRover/CLion:**
- Install Rust plugin
- Configure Cargo toolchain
- Enable clippy on save

---

## How to Contribute

### Reporting Bugs

1. **Search existing issues** to avoid duplicates
2. **Use the bug report template** when creating a new issue
3. **Include:**
   - Clear description of the bug
   - Steps to reproduce
   - Expected behavior
   - Actual behavior
   - Environment details (OS, Rust version, etc.)
   - Relevant logs or screenshots

### Requesting Features

1. **Search existing issues** for similar requests
2. **Use the feature request template**
3. **Include:**
   - Problem description (what are you trying to solve?)
   - Proposed solution
   - Alternative solutions considered
   - Use cases and examples

### Contributing Code

1. **Fork the repository** on GitHub
2. **Create a feature branch** from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Make your changes** following the code style guidelines
4. **Write tests** for your changes
5. **Update documentation** as needed
6. **Commit your changes** using conventional commits
7. **Push to your fork** and submit a pull request

---

## Code Style Guidelines

### Rust

Follow the standard Rust style guidelines enforced by `rustfmt` and `clippy`:

```bash
# Format code
cargo fmt --all

# Run clippy with strict warnings
cargo clippy --workspace -- -D warnings
```

**Key conventions:**
- Use `snake_case` for functions, variables, and modules
- Use `PascalCase` for types, traits, and enums
- Use `SCREAMING_SNAKE_CASE` for constants
- Prefer `&str` over `String` for function parameters when possible
- Use `Result<T, E>` for fallible operations
- Document public APIs with doc comments (`///`)
- Keep functions focused and under 50 lines when possible

**Example:**

```rust
/// Performs a SYN scan on the specified target.
///
/// # Arguments
///
/// * `target` - The target IP address or CIDR range
/// * `ports` - The ports to scan
///
/// # Returns
///
/// Returns scan results or an error if the scan fails.
///
/// # Errors
///
/// Returns `ScanError` if:
/// - The target is unreachable
/// - Insufficient permissions for raw sockets
pub fn syn_scan(target: &str, ports: &[u16]) -> Result<ScanResults, ScanError> {
    // Implementation
}
```

### TypeScript/JavaScript

For any TypeScript/JavaScript code (primarily CyberChef integration):

```bash
# ESLint for linting
npx eslint .

# Prettier for formatting
npx prettier --write .
```

**Key conventions:**
- Use `camelCase` for variables and functions
- Use `PascalCase` for classes and types
- Use `SCREAMING_SNAKE_CASE` for constants
- Prefer `const` over `let`
- Use TypeScript strict mode
- Document functions with JSDoc/TSDoc

### General Guidelines

- **Keep files focused:** One module/class per file
- **Limit line length:** 100 characters maximum
- **Use meaningful names:** Self-documenting code
- **Comment why, not what:** Explain rationale, not mechanics
- **No commented-out code:** Use version control instead

---

## Commit Message Conventions

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

### Types

| Type | Description |
|------|-------------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation changes |
| `style` | Formatting, no code change |
| `refactor` | Code change without feature/fix |
| `perf` | Performance improvement |
| `test` | Adding/updating tests |
| `build` | Build system changes |
| `ci` | CI configuration changes |
| `chore` | Maintenance tasks |

### Scopes

| Scope | Description |
|-------|-------------|
| `cli` | spectre-cli crate |
| `core` | spectre-core crate |
| `tui` | spectre-tui crate |
| `gui` | spectre-gui crate |
| `mcp` | spectre-mcp crate |
| `docs` | Documentation |
| `deps` | Dependencies |

### Examples

```
feat(cli): add campaign orchestration subcommand

Implements the `spectre campaign run` subcommand for executing
campaign definitions from YAML files.

Closes #42
```

```
fix(core): resolve race condition in scan scheduler

The scan scheduler could deadlock when multiple scans completed
simultaneously. This fix uses a bounded channel to prevent the issue.

Fixes #123
```

```
docs: update CLI reference with new flags

Added documentation for --timing-template and --evasion flags
introduced in v0.2.0.
```

---

## Pull Request Process

### Before Submitting

1. **Ensure all tests pass:**
   ```bash
   cargo test --workspace
   ```

2. **Run linting and formatting:**
   ```bash
   cargo fmt --all --check
   cargo clippy --workspace -- -D warnings
   ```

3. **Run security audit:**
   ```bash
   cargo audit
   ```

4. **Update documentation** if your changes affect user-facing behavior

5. **Add entries to CHANGELOG.md** under the `[Unreleased]` section

### PR Requirements

- **Fill out the PR template** completely
- **Link related issues** using keywords (Closes #123)
- **Ensure CI passes** before requesting review
- **Keep PRs focused:** One feature/fix per PR
- **Include tests** for new functionality
- **Update documentation** for user-facing changes

### Review Process

1. **Automated checks** must pass (CI, linting, tests)
2. **Code review** by at least one maintainer
3. **Address feedback** promptly
4. **Squash commits** if requested
5. **Maintainer merges** when approved

### After Merge

- Delete your feature branch
- Close related issues if not auto-closed
- Monitor for any CI failures in main

---

## Testing Requirements

### Test Coverage

- All new features must have tests
- Bug fixes should include regression tests
- Aim for 80%+ code coverage on new code

### Types of Tests

**Unit Tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_target_cidr() {
        let target = parse_target("192.168.1.0/24").unwrap();
        assert_eq!(target.hosts().count(), 254);
    }
}
```

**Integration Tests:**
```rust
// tests/integration/scan_workflow.rs
#[test]
fn test_scan_to_analysis_pipeline() {
    // Test component integration
}
```

**End-to-End Tests:**
```rust
// tests/e2e/campaign_execution.rs
#[test]
fn test_full_campaign_workflow() {
    // Test complete workflow
}
```

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run with output
cargo test --workspace -- --nocapture

# Run specific test
cargo test test_name

# Run with coverage
cargo tarpaulin --workspace --out Html
```

---

## Documentation Requirements

### Code Documentation

- All public APIs must have doc comments
- Include examples in doc comments
- Document error conditions
- Document panics (if any)

### User Documentation

- Update relevant docs in `docs/` for user-facing changes
- Keep README.md in sync with major changes
- Add tutorials for complex features

### Changelog

Add entries to CHANGELOG.md for:
- New features (`### Added`)
- Changes to existing features (`### Changed`)
- Deprecated features (`### Deprecated`)
- Removed features (`### Removed`)
- Bug fixes (`### Fixed`)
- Security fixes (`### Security`)

---

## Security Considerations

### Reporting Vulnerabilities

**Do not open public issues for security vulnerabilities.**

See [SECURITY.md](SECURITY.md) for responsible disclosure instructions.

### Security in Code

- Never commit secrets, keys, or credentials
- Validate all user input
- Use safe defaults for security-sensitive operations
- Follow the principle of least privilege
- Log security-relevant events (without sensitive data)

### Dependencies

- Run `cargo audit` before submitting PRs
- Keep dependencies updated
- Prefer well-maintained crates with security track records

---

## Questions?

- **GitHub Discussions:** [SPECTRE Discussions](https://github.com/doublegate/SPECTRE/discussions)
- **Issues:** [GitHub Issues](https://github.com/doublegate/SPECTRE/issues)

Thank you for contributing to SPECTRE.
