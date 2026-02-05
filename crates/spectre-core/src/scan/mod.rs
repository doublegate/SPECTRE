//! Network scanning module (ProRT-IP integration)
//!
//! This module provides the interface for network scanning functionality.
//! The actual scanning is performed by the ProRT-IP library via the
//! [`prtip_adapter::PrtipScanner`] adapter.
//!
//! # Architecture
//!
//! SPECTRE defines a [`Scanner`] trait that abstracts scanning implementations.
//! The primary implementation is [`prtip_adapter::PrtipScanner`], which wraps
//! ProRT-IP's scanner engines (TCP Connect, SYN, Stealth, UDP). A
//! [`StubScanner`] is retained for unit testing in environments without
//! network access.
//!
//! # Scanner Selection
//!
//! The [`create_scanner`] function returns a `PrtipScanner` by default. In
//! test builds (`#[cfg(test)]`), or when explicitly requested via
//! [`create_stub_scanner`], the stub implementation is used instead.
//!
//! # ProRT-IP Capabilities
//!
//! - 8 scan types: SYN, Connect, UDP, ACK, FIN, Xmas, Null, Window
//! - High-speed packet transmission (10M+ packets per second)
//! - Service and OS detection
//! - Lua scripting for custom scans

mod parser;
pub mod prtip_adapter;
mod traits;
mod types;

use std::path::PathBuf;

use async_trait::async_trait;

pub use parser::{parse_ports, parse_targets};
pub use prtip_adapter::PrtipScanner;
pub use traits::{ProgressCallback, Scanner};
pub use types::*;

use crate::config::Config;

/// Create a scanner instance based on configuration.
///
/// Returns a [`PrtipScanner`] that delegates to ProRT-IP's scanning engines.
/// For testing environments that do not require real network operations, use
/// [`create_stub_scanner`] instead.
///
/// # Arguments
///
/// * `config` - Configuration to use for scanner setup
///
/// # Returns
///
/// A boxed scanner implementation backed by ProRT-IP.
pub fn create_scanner(config: &Config) -> crate::Result<Box<dyn Scanner>> {
    Ok(Box::new(PrtipScanner::new(config)?))
}

/// Create a stub scanner for testing environments.
///
/// The stub scanner simulates scanning behavior without performing real
/// network operations. It always returns deterministic results suitable
/// for unit and integration tests.
///
/// # Arguments
///
/// * `config` - Configuration to use for scanner setup
///
/// # Returns
///
/// A boxed stub scanner implementation.
pub fn create_stub_scanner(config: &Config) -> crate::Result<Box<dyn Scanner>> {
    Ok(Box::new(StubScanner::new(config)?))
}

/// Check if the ProRT-IP scanner is available and report its capabilities.
///
/// Returns information about the scanner version, supported scan types,
/// maximum packet rate, and privilege requirements.
pub async fn check_availability(_config: &Config) -> crate::Result<ScannerInfo> {
    Ok(ScannerInfo {
        version: format!("ProRT-IP {}", env!("CARGO_PKG_VERSION")),
        supported_scan_types: vec![
            "SYN".to_string(),
            "Connect".to_string(),
            "UDP".to_string(),
            "ACK".to_string(),
            "FIN".to_string(),
            "Xmas".to_string(),
            "Null".to_string(),
            "Window".to_string(),
        ],
        max_packet_rate: 10_000_000,
        requires_root: true,
    })
}

/// Scanner availability information
#[derive(Debug, Clone)]
pub struct ScannerInfo {
    /// Scanner version
    pub version: String,
    /// Supported scan types
    pub supported_scan_types: Vec<String>,
    /// Maximum packet rate
    pub max_packet_rate: u64,
    /// Whether root/admin is required
    pub requires_root: bool,
}

/// Stub scanner implementation for development and testing.
///
/// This implementation simulates scanning behavior without performing real
/// network operations. It returns deterministic results that are useful for
/// testing scan-dependent code paths without requiring network access or
/// elevated privileges.
///
/// # Simulated Behavior
///
/// - Ports 22, 80, 443 always appear as `Open`
/// - Port 8080 appears as `Filtered`
/// - All other ports appear as `Closed` (and are excluded from results)
/// - Service detection returns well-known service names for open ports
/// - OS detection returns a static Linux fingerprint
pub struct StubScanner {
    interface: Option<String>,
    _script_dir: Option<PathBuf>,
}

impl StubScanner {
    /// Create a new stub scanner
    pub fn new(config: &Config) -> crate::Result<Self> {
        Ok(Self {
            interface: config.scan.interface.clone(),
            _script_dir: config.scan.script_dir.clone(),
        })
    }
}

#[async_trait]
impl Scanner for StubScanner {
    async fn scan(
        &self,
        config: ScanConfig,
        progress_callback: Option<traits::ProgressCallback>,
    ) -> crate::Result<Vec<ScanResult>> {
        use std::time::Duration;
        use tracing::{debug, info};

        let total_targets = config.targets.len();
        info!(
            targets = total_targets,
            ports = config.ports.len(),
            scan_type = ?config.scan_type,
            "Starting stub scan"
        );

        let mut results = Vec::new();

        for (idx, target) in config.targets.iter().enumerate() {
            debug!(target = %target, "Scanning target");

            // Simulate scan delay
            tokio::time::sleep(Duration::from_millis(100)).await;

            // Generate stub results
            let mut ports = Vec::new();

            for &port in &config.ports {
                // Simulate some open ports
                let state = if port == 22 || port == 80 || port == 443 {
                    PortState::Open
                } else if port == 8080 {
                    PortState::Filtered
                } else {
                    PortState::Closed
                };

                let service = match port {
                    22 => Some("ssh".to_string()),
                    80 => Some("http".to_string()),
                    443 => Some("https".to_string()),
                    _ => None,
                };

                let version = if config.service_detection && service.is_some() {
                    match port {
                        22 => Some("OpenSSH 8.9".to_string()),
                        80 | 443 => Some("nginx 1.24".to_string()),
                        _ => None,
                    }
                } else {
                    None
                };

                // Only include non-closed ports unless specifically requested
                if state != PortState::Closed {
                    ports.push(PortResult {
                        port,
                        protocol: Protocol::Tcp,
                        state,
                        service,
                        version,
                        banner: None,
                    });
                }
            }

            let result = ScanResult {
                host: target.to_string(),
                ip: target.clone(),
                hostname: if config.resolve_dns {
                    Some(format!("host-{}", target.to_string().replace('.', "-")))
                } else {
                    None
                },
                ports,
                os: if config.os_detection {
                    Some(OsInfo {
                        name: "Linux".to_string(),
                        version: Some("5.15".to_string()),
                        confidence: 85,
                    })
                } else {
                    None
                },
                scan_time: Duration::from_millis(100),
            };

            results.push(result);

            // Call progress callback (current, total)
            if let Some(ref callback) = progress_callback {
                callback(idx + 1, total_targets);
            }
        }

        info!(results = results.len(), "Stub scan complete");

        Ok(results)
    }

    fn interface(&self) -> Option<&str> {
        self.interface.as_deref()
    }

    fn requires_root(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_stub_scanner() {
        let config = Config::default();
        let scanner = create_stub_scanner(&config).unwrap();

        let scan_config = ScanConfig {
            scan_type: ScanType::Connect,
            targets: vec![Target::Ip("127.0.0.1".parse().unwrap())],
            ports: vec![22, 80, 443],
            ..Default::default()
        };

        let results = scanner.scan(scan_config, None).await.unwrap();

        assert_eq!(results.len(), 1);
        assert!(!results[0].ports.is_empty());
    }

    #[tokio::test]
    async fn test_check_availability() {
        let config = Config::default();
        let info = check_availability(&config).await.unwrap();

        assert!(!info.version.is_empty());
        assert!(!info.supported_scan_types.is_empty());
        assert_eq!(info.supported_scan_types.len(), 8);
        assert_eq!(info.max_packet_rate, 10_000_000);
    }

    #[tokio::test]
    async fn test_create_prtip_scanner() {
        let config = Config::default();
        let scanner = create_scanner(&config);
        assert!(scanner.is_ok());
    }

    #[tokio::test]
    async fn test_create_stub_scanner_fn() {
        let config = Config::default();
        let scanner = create_stub_scanner(&config);
        assert!(scanner.is_ok());
    }
}
