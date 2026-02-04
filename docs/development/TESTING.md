# Testing Guide

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## Overview

SPECTRE uses a comprehensive testing strategy including unit tests, integration tests, property-based tests, and documentation tests.

---

## Running Tests

### All Tests

```bash
# Run all workspace tests
cargo test --workspace

# With output visible
cargo test --workspace -- --nocapture

# With parallel test threads limited
cargo test --workspace -- --test-threads=4
```

### Specific Tests

```bash
# Single crate
cargo test -p spectre-core

# Single test
cargo test test_scan_syn

# Tests matching pattern
cargo test scan_

# Specific module
cargo test scan::tests
```

### Test Categories

```bash
# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test '*'

# Documentation tests
cargo test --doc

# Ignored tests (long-running)
cargo test -- --ignored

# All including ignored
cargo test -- --include-ignored
```

---

## Test Organization

### Unit Tests

Located in each module with `#[cfg(test)]`:

```rust
// src/scan/manager.rs

pub fn process_target(target: &Target) -> Result<Finding> {
    // Implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_target_valid() {
        let target = Target::new("192.168.1.1");
        let result = process_target(&target);
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_target_invalid() {
        let target = Target::new("invalid");
        let result = process_target(&target);
        assert!(result.is_err());
    }
}
```

### Integration Tests

Located in `/tests/` directory:

```
tests/
├── cli_integration.rs      # CLI tests
├── scan_integration.rs     # Scanning tests
├── chef_integration.rs     # CyberChef tests
├── common/                 # Shared test utilities
│   ├── mod.rs
│   └── fixtures.rs
└── fixtures/               # Test data
    ├── targets.txt
    └── expected_output.json
```

```rust
// tests/cli_integration.rs

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    Command::cargo_bin("spectre")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("SPECTRE"));
}

#[test]
fn test_cli_version() {
    Command::cargo_bin("spectre")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}
```

### Property-Based Tests

Using `proptest` for randomized testing:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_parse_port_range(start in 1u16..65000, end in 1u16..65535) {
        prop_assume!(start <= end);

        let range = format!("{}-{}", start, end);
        let result = parse_port_range(&range);

        prop_assert!(result.is_ok());
        let ports = result.unwrap();
        prop_assert_eq!(ports.start, start);
        prop_assert_eq!(ports.end, end);
    }

    #[test]
    fn test_ip_roundtrip(a in 0u8..=255, b in 0u8..=255, c in 0u8..=255, d in 0u8..=255) {
        let ip = format!("{}.{}.{}.{}", a, b, c, d);
        let parsed = parse_ip(&ip).unwrap();
        let formatted = format_ip(&parsed);
        prop_assert_eq!(ip, formatted);
    }
}
```

### Documentation Tests

In doc comments:

```rust
/// Parse a CIDR network specification.
///
/// # Examples
///
/// ```
/// use spectre_core::parse_cidr;
///
/// let network = parse_cidr("192.168.1.0/24").unwrap();
/// assert_eq!(network.prefix_len(), 24);
/// assert_eq!(network.hosts().count(), 254);
/// ```
///
/// # Errors
///
/// Returns an error if the CIDR notation is invalid:
///
/// ```
/// use spectre_core::parse_cidr;
///
/// assert!(parse_cidr("invalid").is_err());
/// assert!(parse_cidr("192.168.1.0/33").is_err());
/// ```
pub fn parse_cidr(input: &str) -> Result<Network> {
    // Implementation
}
```

---

## Async Tests

For async functions:

```rust
#[tokio::test]
async fn test_scan_target() {
    let scanner = Scanner::new().await.unwrap();
    let result = scanner.scan(&target).await;
    assert!(result.is_ok());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn test_concurrent_scans() {
    let scanner = Scanner::new().await.unwrap();

    let handles: Vec<_> = targets.iter()
        .map(|t| tokio::spawn(scanner.scan(t)))
        .collect();

    for handle in handles {
        assert!(handle.await.unwrap().is_ok());
    }
}
```

---

## Test Fixtures

### Data Files

```rust
use std::path::PathBuf;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

#[test]
fn test_parse_targets_file() {
    let path = fixture_path("targets.txt");
    let targets = parse_targets_file(&path).unwrap();
    assert_eq!(targets.len(), 10);
}
```

### Test Builders

```rust
// tests/common/builders.rs

pub struct TargetBuilder {
    ip: String,
    ports: Vec<u16>,
    hostname: Option<String>,
}

impl TargetBuilder {
    pub fn new(ip: &str) -> Self {
        Self {
            ip: ip.to_string(),
            ports: vec![],
            hostname: None,
        }
    }

    pub fn with_ports(mut self, ports: Vec<u16>) -> Self {
        self.ports = ports;
        self
    }

    pub fn with_hostname(mut self, hostname: &str) -> Self {
        self.hostname = Some(hostname.to_string());
        self
    }

    pub fn build(self) -> Target {
        Target {
            ip: self.ip,
            ports: self.ports,
            hostname: self.hostname,
        }
    }
}

// Usage
#[test]
fn test_with_builder() {
    let target = TargetBuilder::new("192.168.1.1")
        .with_ports(vec![22, 80, 443])
        .with_hostname("test.local")
        .build();

    assert_eq!(target.ports.len(), 3);
}
```

---

## Mocking

### Using mockall

```rust
use mockall::{automock, predicate::*};

#[automock]
pub trait Scanner {
    async fn scan(&self, target: &Target) -> Result<Finding>;
}

#[tokio::test]
async fn test_orchestrator_with_mock() {
    let mut mock_scanner = MockScanner::new();

    mock_scanner
        .expect_scan()
        .with(eq(target))
        .times(1)
        .returning(|_| Ok(Finding::default()));

    let orchestrator = Orchestrator::new(mock_scanner);
    let result = orchestrator.run(&target).await;

    assert!(result.is_ok());
}
```

### Test Doubles

```rust
pub struct FakeScanner {
    responses: HashMap<Target, Result<Finding>>,
}

impl FakeScanner {
    pub fn new() -> Self {
        Self { responses: HashMap::new() }
    }

    pub fn when(&mut self, target: Target, response: Result<Finding>) {
        self.responses.insert(target, response);
    }
}

impl Scanner for FakeScanner {
    async fn scan(&self, target: &Target) -> Result<Finding> {
        self.responses.get(target)
            .cloned()
            .unwrap_or(Err(ScanError::NotConfigured.into()))
    }
}
```

---

## Test Coverage

### Running Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --workspace --out Html

# View report
open tarpaulin-report.html
```

### Coverage Targets

| Component | Target | Critical |
|-----------|--------|----------|
| spectre-core | 80% | 90% |
| spectre-cli | 70% | 80% |
| spectre-tui | 60% | 70% |
| spectre-mcp | 80% | 90% |

### Excluding from Coverage

```rust
#[cfg(not(tarpaulin_include))]
fn debug_only_function() {
    // Not included in coverage
}
```

---

## Benchmarks

Using `criterion`:

```rust
// benches/scan_benchmark.rs

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use spectre_core::parse_targets;

fn benchmark_parse_targets(c: &mut Criterion) {
    let inputs = vec![
        ("small", include_str!("fixtures/targets_10.txt")),
        ("medium", include_str!("fixtures/targets_100.txt")),
        ("large", include_str!("fixtures/targets_1000.txt")),
    ];

    let mut group = c.benchmark_group("parse_targets");

    for (name, input) in inputs {
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            input,
            |b, input| {
                b.iter(|| parse_targets(input))
            },
        );
    }

    group.finish();
}

criterion_group!(benches, benchmark_parse_targets);
criterion_main!(benches);
```

Running benchmarks:

```bash
cargo bench

# With comparison to baseline
cargo bench -- --save-baseline main
# Make changes...
cargo bench -- --baseline main
```

---

## CI Integration

Tests run automatically on:
- Push to main/develop
- Pull requests

See `.github/workflows/ci.yml` for configuration.

### Required Checks

- [ ] `cargo fmt --check`
- [ ] `cargo clippy -- -D warnings`
- [ ] `cargo test --workspace`
- [ ] `cargo audit`

---

## Best Practices

### Naming

```rust
// Pattern: test_<function>_<scenario>_<expected>
#[test]
fn test_parse_port_valid_range_returns_ports() { }

#[test]
fn test_parse_port_invalid_format_returns_error() { }

#[test]
fn test_scan_timeout_returns_timeout_error() { }
```

### Assertions

```rust
// Prefer specific assertions
assert_eq!(result.len(), 5);
assert!(result.contains(&expected));

// With context
assert_eq!(
    result.len(),
    5,
    "Expected 5 ports but got {}: {:?}",
    result.len(),
    result
);
```

### Test Independence

```rust
// BAD: Tests depend on order
static mut COUNTER: u32 = 0;

#[test]
fn test_a() {
    unsafe { COUNTER += 1; }
}

#[test]
fn test_b() {
    unsafe { assert_eq!(COUNTER, 1); } // Flaky!
}

// GOOD: Independent tests
#[test]
fn test_a() {
    let counter = AtomicU32::new(0);
    counter.fetch_add(1, Ordering::SeqCst);
    assert_eq!(counter.load(Ordering::SeqCst), 1);
}
```

### Cleanup

```rust
use tempfile::TempDir;

#[test]
fn test_with_temp_dir() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("test.txt");

    // Test code using temp_dir

    // Automatically cleaned up when temp_dir drops
}
```
