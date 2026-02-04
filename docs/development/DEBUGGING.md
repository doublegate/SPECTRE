# Debugging Guide

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## Logging

### Enabling Logs

SPECTRE uses `tracing` for structured logging.

```bash
# Basic logging
RUST_LOG=info spectre scan -sS 192.168.1.1

# Debug level
RUST_LOG=debug spectre scan -sS 192.168.1.1

# Trace level (very verbose)
RUST_LOG=trace spectre scan -sS 192.168.1.1

# Module-specific
RUST_LOG=spectre_core::scan=debug spectre scan -sS 192.168.1.1

# Multiple modules
RUST_LOG=spectre_core=debug,spectre_cli=info spectre scan -sS 192.168.1.1
```

### Log Format

```bash
# JSON output (for log aggregation)
RUST_LOG=debug RUST_LOG_FORMAT=json spectre scan ...

# Pretty output (default)
RUST_LOG=debug RUST_LOG_FORMAT=pretty spectre scan ...

# Compact output
RUST_LOG=debug RUST_LOG_FORMAT=compact spectre scan ...
```

### Adding Logs in Code

```rust
use tracing::{debug, info, warn, error, trace, instrument};

#[instrument(skip(config))]
pub fn scan_target(target: &Target, config: &Config) -> Result<Finding> {
    info!(target = %target.ip, "Starting scan");

    debug!(ports = ?config.ports, "Scanning ports");

    if let Err(e) = connect(target) {
        warn!(error = %e, "Connection failed, retrying");
    }

    trace!(raw_response = ?response, "Received response");

    Ok(finding)
}
```

---

## Debug Builds

### Build with Debug Symbols

```bash
# Debug build (default)
cargo build

# Debug symbols in release
cargo build --release
# Or in Cargo.toml:
# [profile.release]
# debug = true
```

### Running in Debug Mode

```bash
# More assertions enabled
cargo run -- scan -sS 192.168.1.1

# With backtrace
RUST_BACKTRACE=1 cargo run -- scan -sS 192.168.1.1

# Full backtrace
RUST_BACKTRACE=full cargo run -- scan -sS 192.168.1.1
```

---

## Debugger Integration

### LLDB (macOS/Linux)

```bash
# Build debug binary
cargo build

# Run with lldb
lldb target/debug/spectre -- scan -sS 192.168.1.1
```

LLDB commands:
```lldb
# Set breakpoint
(lldb) b scan_target
(lldb) b spectre_core::scan::manager::process

# Run
(lldb) r

# Step over
(lldb) n

# Step into
(lldb) s

# Print variable
(lldb) p target
(lldb) p *config

# Continue
(lldb) c

# Backtrace
(lldb) bt
```

### GDB (Linux)

```bash
# Run with gdb
rust-gdb target/debug/spectre -- scan -sS 192.168.1.1
```

GDB commands:
```gdb
# Set breakpoint
(gdb) break scan_target
(gdb) b src/scan/manager.rs:42

# Run
(gdb) run

# Print
(gdb) print target
(gdb) p/x value

# Continue
(gdb) continue

# Backtrace
(gdb) backtrace
```

### VS Code Integration

`.vscode/launch.json`:
```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug spectre",
      "cargo": {
        "args": ["build", "--bin=spectre", "--package=spectre-cli"],
        "filter": {
          "name": "spectre",
          "kind": "bin"
        }
      },
      "args": ["scan", "-sS", "192.168.1.1"],
      "cwd": "${workspaceFolder}",
      "env": {
        "RUST_LOG": "debug",
        "RUST_BACKTRACE": "1"
      }
    }
  ]
}
```

---

## Common Issues

### Permission Denied

**Symptom:**
```
Error: Permission denied (os error 13)
```

**Cause:** SYN scans require raw socket access.

**Solutions:**
```bash
# Run as root
sudo spectre scan -sS 192.168.1.1

# Set capabilities (permanent)
sudo setcap cap_net_raw,cap_net_admin+ep target/debug/spectre

# Use unprivileged scan type
spectre scan -sT 192.168.1.1  # Connect scan
```

### Address Already in Use

**Symptom:**
```
Error: Address already in use (os error 98)
```

**Cause:** Port conflict or previous instance not cleaned up.

**Solutions:**
```bash
# Find process using port
lsof -i :8080
sudo ss -tulpn | grep 8080

# Kill previous instance
pkill spectre

# Wait for TIME_WAIT
sleep 30
```

### Timeout Errors

**Symptom:**
```
Error: operation timed out
```

**Cause:** Network issues or aggressive timing.

**Solutions:**
```bash
# Increase timeout
spectre scan -sS --timeout 10000 192.168.1.1

# Use slower timing
spectre scan -sS -T2 192.168.1.1

# Check connectivity
ping 192.168.1.1
traceroute 192.168.1.1
```

### Memory Issues

**Symptom:**
```
Error: memory allocation failed
```

**Cause:** Large target list or results.

**Solutions:**
```bash
# Limit concurrent scans
spectre scan -sS --max-concurrent 10 192.168.1.0/24

# Stream results instead of buffering
spectre scan -sS --stream 192.168.1.0/24

# Increase memory limit
ulimit -v unlimited
```

---

## Profiling

### CPU Profiling with perf

```bash
# Record profile
perf record --call-graph dwarf target/release/spectre scan -sS 192.168.1.0/24

# View flame graph
perf script | inferno-collapse-perf | inferno-flamegraph > flame.svg
```

### Memory Profiling with heaptrack

```bash
# Record allocations
heaptrack target/debug/spectre scan -sS 192.168.1.0/24

# Analyze
heaptrack_gui heaptrack.spectre.*.gz
```

### Valgrind

```bash
# Memory leaks
valgrind --leak-check=full target/debug/spectre scan -sS 192.168.1.1

# Cache analysis
valgrind --tool=cachegrind target/release/spectre scan -sS 192.168.1.0/24
```

---

## Network Debugging

### Packet Capture

```bash
# Capture SPECTRE traffic
sudo tcpdump -i eth0 -w spectre.pcap host 192.168.1.1

# View in Wireshark
wireshark spectre.pcap
```

### Network Tracing

```bash
# Trace system calls
strace -f -e trace=network target/debug/spectre scan -sS 192.168.1.1

# Trace with timing
strace -f -T -e trace=network target/debug/spectre scan -sS 192.168.1.1
```

---

## Async Debugging

### Tokio Console

Add to `Cargo.toml`:
```toml
[dependencies]
console-subscriber = "0.2"
```

In code:
```rust
fn main() {
    console_subscriber::init();
    // ...
}
```

Run:
```bash
RUSTFLAGS="--cfg tokio_unstable" cargo build
tokio-console
```

### Task Tracing

```rust
use tracing::{instrument, Instrument};

#[instrument]
async fn process_scan(target: &Target) -> Result<Finding> {
    let span = tracing::info_span!("process_scan", target = %target.ip);

    async {
        // Work here
    }
    .instrument(span)
    .await
}
```

---

## Panic Debugging

### Catching Panics

```rust
use std::panic;

fn main() {
    panic::set_hook(Box::new(|info| {
        eprintln!("Panic occurred: {}", info);
        if let Some(location) = info.location() {
            eprintln!("  at {}:{}:{}",
                location.file(),
                location.line(),
                location.column()
            );
        }
    }));

    // Application code
}
```

### Panic Backtrace

```bash
# Enable backtrace
RUST_BACKTRACE=1 spectre scan -sS 192.168.1.1

# Full backtrace
RUST_BACKTRACE=full spectre scan -sS 192.168.1.1
```

---

## Test Debugging

### Running Single Test with Output

```bash
# Show println! output
cargo test test_name -- --nocapture

# Show only failed
cargo test -- --nocapture 2>&1 | grep -A 20 "FAILED"
```

### Debugging Tests in VS Code

`.vscode/launch.json`:
```json
{
  "type": "lldb",
  "request": "launch",
  "name": "Debug Tests",
  "cargo": {
    "args": ["test", "--no-run", "--lib", "-p", "spectre-core"],
    "filter": {
      "kind": "lib"
    }
  },
  "args": ["test_scan_target", "--nocapture"],
  "cwd": "${workspaceFolder}"
}
```

---

## Tips and Tricks

### Print Debugging

```rust
// Pretty print complex structures
dbg!(&complex_struct);

// With format
eprintln!("Value: {:#?}", value);

// Quick type inspection
fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
println!("Type: {}", type_of(&variable));
```

### Conditional Compilation

```rust
#[cfg(debug_assertions)]
fn debug_check(value: &Value) {
    assert!(value.is_valid(), "Invalid value in debug build");
}

#[cfg(not(debug_assertions))]
fn debug_check(_: &Value) {}
```

### Debug Trait Implementation

```rust
use std::fmt;

impl fmt::Debug for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Target")
            .field("ip", &self.ip)
            .field("ports", &self.ports.len())  // Just count, not all ports
            .finish()
    }
}
```
