//! ProRT-IP scanner adapter for SPECTRE
//!
//! This module bridges SPECTRE's scanner interface with the ProRT-IP scanning
//! library. It converts SPECTRE types to ProRT-IP types, delegates scanning
//! to the appropriate ProRT-IP scanner engine, and aggregates per-port results
//! into SPECTRE's per-host result format.
//!
//! # Scanner Engine Selection
//!
//! The adapter selects the appropriate ProRT-IP scanner based on the scan type:
//!
//! | SPECTRE ScanType | ProRT-IP Scanner | Notes |
//! |------------------|-----------------|-------|
//! | Connect | `TcpConnectScanner` | No root required |
//! | Syn | `SynScanner` | Requires root |
//! | Fin | `StealthScanner` (Fin) | Requires root |
//! | Null | `StealthScanner` (Null) | Requires root |
//! | Xmas | `StealthScanner` (Xmas) | Requires root |
//! | Ack | `StealthScanner` (Ack) | Requires root |
//! | Window | `StealthScanner` (Ack) | Mapped to ACK (closest) |
//! | Udp | `UdpScanner` | Requires root |
//!
//! # Result Aggregation
//!
//! ProRT-IP returns per-port `ScanResult` values. SPECTRE expects per-host
//! `ScanResult` values containing a `Vec<PortResult>`. This adapter groups
//! ProRT-IP results by target IP and aggregates them into SPECTRE's format.
//!
//! # Thread Safety
//!
//! Some ProRT-IP scanner engines (SYN, Stealth, UDP) use thread-local random
//! number generators that are `!Send`. These scanners are executed inside
//! `tokio::task::spawn_blocking` with a dedicated runtime to avoid violating
//! the `Send` requirement of the `Scanner` trait's async methods.

use std::net::IpAddr;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use tracing::{debug, info};

use super::traits::{ProgressCallback, Scanner};
use super::types::{
    PortResult, PortState, Protocol, ScanConfig, ScanResult, ScanType, Target, TimingTemplate,
};
use crate::config::Config;

/// ProRT-IP scanner adapter implementing SPECTRE's [`Scanner`] trait.
///
/// Wraps the ProRT-IP scanning engines (TCP Connect, SYN, Stealth, UDP)
/// and presents them through SPECTRE's unified scanner interface.
///
/// # Example
///
/// ```no_run
/// use spectre_core::config::Config;
/// use spectre_core::scan::prtip_adapter::PrtipScanner;
/// use spectre_core::scan::{Scanner, ScanConfig, ScanType, Target};
///
/// # async fn example() -> spectre_core::Result<()> {
/// let config = Config::default();
/// let scanner = PrtipScanner::new(&config)?;
///
/// let scan_config = ScanConfig {
///     scan_type: ScanType::Connect,
///     targets: vec![Target::Ip("127.0.0.1".parse().unwrap())],
///     ports: vec![80, 443],
///     ..Default::default()
/// };
///
/// let results = scanner.scan(scan_config, None).await?;
/// # Ok(())
/// # }
/// ```
pub struct PrtipScanner {
    /// Network interface to use for scanning
    interface: Option<String>,
    /// ProRT-IP configuration for scanner engines that need it
    prtip_config: prtip_core::Config,
}

impl PrtipScanner {
    /// Create a new ProRT-IP scanner adapter from SPECTRE configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - SPECTRE configuration providing scan defaults
    ///
    /// # Returns
    ///
    /// A configured `PrtipScanner` ready for use, or an error if
    /// configuration is invalid.
    pub fn new(config: &Config) -> crate::Result<Self> {
        let mut prtip_config = prtip_core::Config::default();

        // Map SPECTRE config fields to ProRT-IP config
        if let Some(ref iface) = config.scan.interface {
            debug!(interface = %iface, "Configuring scanner interface");
        }

        // Map timing from SPECTRE's numeric default_timing to ProRT-IP's template
        prtip_config.scan.timing_template =
            timing_to_prtip(numeric_to_timing(config.scan.default_timing));

        // Map rate limit (SPECTRE u64 -> ProRT-IP u32, clamp to u32::MAX)
        if config.scan.max_rate > 0 {
            let clamped = u32::try_from(config.scan.max_rate).unwrap_or(u32::MAX);
            prtip_config.performance.max_rate = Some(clamped);
        }

        // Map service detection defaults
        prtip_config.scan.service_detection.enabled = config.scan.service_detection;

        Ok(Self {
            interface: config.scan.interface.clone(),
            prtip_config,
        })
    }
}

#[async_trait]
impl Scanner for PrtipScanner {
    async fn scan(
        &self,
        config: ScanConfig,
        progress_callback: Option<ProgressCallback>,
    ) -> crate::Result<Vec<ScanResult>> {
        let total_targets = config.targets.len();
        info!(
            targets = total_targets,
            ports = config.ports.len(),
            scan_type = ?config.scan_type,
            "Starting ProRT-IP scan"
        );

        let mut all_results = Vec::new();

        for (idx, target) in config.targets.iter().enumerate() {
            let host_start = Instant::now();

            // Resolve target to IP addresses for ProRT-IP
            let ips = resolve_target(target)?;

            for ip in &ips {
                // Execute the scan using the appropriate ProRT-IP engine
                let prtip_results = execute_scan(&self.prtip_config, *ip, &config).await?;

                // Determine protocol from scan type for result aggregation
                let proto = protocol_for_scan_type(&config.scan_type);

                // Aggregate per-port results into per-host SPECTRE results
                let host_result = aggregate_results(
                    target,
                    *ip,
                    prtip_results,
                    proto,
                    config.resolve_dns,
                    host_start.elapsed(),
                );

                all_results.push(host_result);
            }

            // Invoke progress callback
            if let Some(ref callback) = progress_callback {
                callback(idx + 1, total_targets);
            }
        }

        info!(results = all_results.len(), "ProRT-IP scan complete");

        Ok(all_results)
    }

    fn interface(&self) -> Option<&str> {
        self.interface.as_deref()
    }

    fn requires_root(&self) -> bool {
        // Only TCP Connect scans work without root
        // Since we can't know the scan type at trait level, conservatively return true
        true
    }
}

/// Execute a scan against a single IP address using the appropriate ProRT-IP engine.
///
/// Selects the scanner based on [`ScanType`] and delegates to ProRT-IP.
/// For scanner engines that produce `!Send` futures (SYN, Stealth, UDP),
/// the work is dispatched to a blocking thread with a local Tokio runtime.
async fn execute_scan(
    prtip_config: &prtip_core::Config,
    ip: IpAddr,
    config: &ScanConfig,
) -> crate::Result<Vec<prtip_core::ScanResult>> {
    let timing = timing_to_prtip(config.timing);
    let timeout_duration = Duration::from_millis(timing.timeout_ms());
    let max_concurrent = timing.max_parallelism();
    let retries = timing.max_retries().unwrap_or(0) as u32;

    match config.scan_type {
        ScanType::Connect => {
            // TcpConnectScanner is Send-safe, can run directly
            scan_tcp_connect(ip, &config.ports, timeout_duration, retries, max_concurrent).await
        },
        ScanType::Syn => {
            // SynScanner is !Send due to internal ThreadRng; run on blocking thread
            let cfg = prtip_config.clone();
            let ports = config.ports.clone();
            run_on_blocking_thread(move || async move {
                let scanner = prtip_scanner::SynScanner::new(cfg)?;
                scanner.scan_ports(ip, ports).await
            })
            .await
        },
        ScanType::Fin => {
            let cfg = prtip_config.clone();
            let ports = config.ports.clone();
            run_on_blocking_thread(move || async move {
                let scanner = prtip_scanner::StealthScanner::new(cfg)?;
                scanner
                    .scan_ports(ip, ports, prtip_scanner::StealthScanType::Fin)
                    .await
            })
            .await
        },
        ScanType::Null => {
            let cfg = prtip_config.clone();
            let ports = config.ports.clone();
            run_on_blocking_thread(move || async move {
                let scanner = prtip_scanner::StealthScanner::new(cfg)?;
                scanner
                    .scan_ports(ip, ports, prtip_scanner::StealthScanType::Null)
                    .await
            })
            .await
        },
        ScanType::Xmas => {
            let cfg = prtip_config.clone();
            let ports = config.ports.clone();
            run_on_blocking_thread(move || async move {
                let scanner = prtip_scanner::StealthScanner::new(cfg)?;
                scanner
                    .scan_ports(ip, ports, prtip_scanner::StealthScanType::Xmas)
                    .await
            })
            .await
        },
        ScanType::Ack | ScanType::Window => {
            // Window scan maps to ACK scan (closest equivalent in ProRT-IP)
            if config.scan_type == ScanType::Window {
                debug!("Window scan mapped to ACK scan (closest ProRT-IP equivalent)");
            }
            let cfg = prtip_config.clone();
            let ports = config.ports.clone();
            run_on_blocking_thread(move || async move {
                let scanner = prtip_scanner::StealthScanner::new(cfg)?;
                scanner
                    .scan_ports(ip, ports, prtip_scanner::StealthScanType::Ack)
                    .await
            })
            .await
        },
        ScanType::Udp => {
            let cfg = prtip_config.clone();
            let ports = config.ports.clone();
            run_on_blocking_thread(move || async move {
                let scanner = prtip_scanner::UdpScanner::new(cfg)?;
                scanner.scan_ports(ip, ports).await
            })
            .await
        },
    }
}

/// Execute a TCP Connect scan using ProRT-IP's `TcpConnectScanner`.
///
/// This scan type does not require root privileges and produces `Send` futures.
async fn scan_tcp_connect(
    ip: IpAddr,
    ports: &[u16],
    timeout: Duration,
    retries: u32,
    max_concurrent: usize,
) -> crate::Result<Vec<prtip_core::ScanResult>> {
    let scanner = prtip_scanner::TcpConnectScanner::new(timeout, retries);
    scanner
        .scan_ports(ip, ports.to_vec(), max_concurrent)
        .await
        .map_err(|e| {
            crate::SpectreError::Scan(crate::error::ScanError::NetworkError(e.to_string()))
        })
}

/// Run a `!Send` async scanner operation on a dedicated blocking thread.
///
/// Creates a new single-threaded Tokio runtime inside `spawn_blocking` to
/// execute the scanner future. This is necessary because ProRT-IP's SYN,
/// Stealth, and UDP scanners use `ThreadRng` internally which is `!Send`.
///
/// # Type Parameters
///
/// * `F` - Factory closure that creates the async scanner future
/// * `Fut` - The async future returned by the factory
async fn run_on_blocking_thread<F, Fut>(f: F) -> crate::Result<Vec<prtip_core::ScanResult>>
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: std::future::Future<Output = prtip_core::Result<Vec<prtip_core::ScanResult>>>,
{
    tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| {
                prtip_core::Error::Network(format!("Failed to create scanner runtime: {}", e))
            })?;
        rt.block_on(f())
    })
    .await
    .map_err(|e| {
        crate::SpectreError::Scan(crate::error::ScanError::NetworkError(format!(
            "Scanner task failed: {}",
            e
        )))
    })?
    .map_err(|e| crate::SpectreError::Scan(crate::error::ScanError::NetworkError(e.to_string())))
}

// ---------------------------------------------------------------------------
// Type conversion functions
// ---------------------------------------------------------------------------

/// Convert a SPECTRE numeric timing value (0-5) to a [`TimingTemplate`].
fn numeric_to_timing(value: u8) -> TimingTemplate {
    match value {
        0 => TimingTemplate::Paranoid,
        1 => TimingTemplate::Sneaky,
        2 => TimingTemplate::Polite,
        3 => TimingTemplate::Normal,
        4 => TimingTemplate::Aggressive,
        5 => TimingTemplate::Insane,
        _ => TimingTemplate::Normal,
    }
}

/// Convert a SPECTRE [`TimingTemplate`] to a ProRT-IP `TimingTemplate`.
fn timing_to_prtip(timing: TimingTemplate) -> prtip_core::TimingTemplate {
    match timing {
        TimingTemplate::Paranoid => prtip_core::TimingTemplate::Paranoid,
        TimingTemplate::Sneaky => prtip_core::TimingTemplate::Sneaky,
        TimingTemplate::Polite => prtip_core::TimingTemplate::Polite,
        TimingTemplate::Normal => prtip_core::TimingTemplate::Normal,
        TimingTemplate::Aggressive => prtip_core::TimingTemplate::Aggressive,
        TimingTemplate::Insane => prtip_core::TimingTemplate::Insane,
    }
}

/// Convert a ProRT-IP [`prtip_core::PortState`] to a SPECTRE [`PortState`].
fn port_state_from_prtip(state: prtip_core::PortState) -> PortState {
    match state {
        prtip_core::PortState::Open => PortState::Open,
        prtip_core::PortState::Closed => PortState::Closed,
        prtip_core::PortState::Filtered => PortState::Filtered,
        prtip_core::PortState::Unknown => PortState::Unknown,
    }
}

/// Convert a ProRT-IP [`prtip_core::Protocol`] to a SPECTRE [`Protocol`].
///
/// ProRT-IP has an ICMP variant that SPECTRE does not. ICMP is mapped to TCP
/// as a fallback since ICMP results typically accompany TCP/UDP scanning.
fn _protocol_from_prtip(protocol: prtip_core::Protocol) -> Protocol {
    match protocol {
        prtip_core::Protocol::Tcp => Protocol::Tcp,
        prtip_core::Protocol::Udp => Protocol::Udp,
        prtip_core::Protocol::Icmp => Protocol::Tcp, // Fallback: ICMP has no SPECTRE equivalent
    }
}

/// Determine the protocol for a scan result based on the scan type.
///
/// ProRT-IP's `ScanResult` does not carry a protocol field, so we infer
/// the protocol from the scanner that produced the result.
fn protocol_for_scan_type(scan_type: &ScanType) -> Protocol {
    match scan_type {
        ScanType::Udp => Protocol::Udp,
        _ => Protocol::Tcp,
    }
}

/// Resolve a SPECTRE [`Target`] into one or more IP addresses.
///
/// For `Hostname` targets, this performs a blocking DNS lookup in the
/// current thread (acceptable since scanning itself is async).
fn resolve_target(target: &Target) -> crate::Result<Vec<IpAddr>> {
    match target {
        Target::Ip(ip) => Ok(vec![*ip]),
        Target::Cidr(network) => Ok(network.iter().collect()),
        Target::Hostname(hostname) => {
            use std::net::ToSocketAddrs;
            let addr_str = format!("{}:0", hostname);
            let addrs: Vec<IpAddr> = addr_str
                .to_socket_addrs()
                .map_err(|e| {
                    crate::SpectreError::Scan(crate::error::ScanError::InvalidTarget(format!(
                        "DNS resolution failed for '{}': {}",
                        hostname, e
                    )))
                })?
                .map(|sa| sa.ip())
                .collect();

            if addrs.is_empty() {
                return Err(crate::SpectreError::Scan(
                    crate::error::ScanError::InvalidTarget(format!(
                        "No addresses found for hostname: {}",
                        hostname
                    )),
                ));
            }

            // Use only the first resolved address
            Ok(vec![addrs[0]])
        },
    }
}

// ---------------------------------------------------------------------------
// Result aggregation
// ---------------------------------------------------------------------------

/// Aggregate ProRT-IP per-port results into a SPECTRE per-host [`ScanResult`].
///
/// Groups scan results by target IP, converts port states and service info,
/// and assembles the final host-level result.
///
/// # Arguments
///
/// * `target` - Original SPECTRE target specification
/// * `ip` - Resolved IP address that was scanned
/// * `prtip_results` - Per-port results from ProRT-IP
/// * `protocol` - Protocol to assign to port results (inferred from scan type)
/// * `resolve_dns` - Whether DNS resolution was requested
/// * `scan_time` - Total time spent scanning this host
fn aggregate_results(
    target: &Target,
    ip: IpAddr,
    prtip_results: Vec<prtip_core::ScanResult>,
    protocol: Protocol,
    resolve_dns: bool,
    scan_time: Duration,
) -> ScanResult {
    let mut ports = Vec::new();

    for pr in &prtip_results {
        // Only include non-closed ports (matching StubScanner behavior)
        let state = port_state_from_prtip(pr.state);
        if state == PortState::Closed {
            continue;
        }

        let port_result = PortResult {
            port: pr.port,
            protocol,
            state,
            service: pr.service.clone(),
            version: pr.version.clone(),
            banner: pr.banner.clone(),
        };

        ports.push(port_result);
    }

    let hostname = if resolve_dns { reverse_dns(ip) } else { None };

    ScanResult {
        host: target.to_string(),
        ip: Target::Ip(ip),
        hostname,
        ports,
        os: None, // OS detection via ProRT-IP will be added in a future integration
        scan_time,
    }
}

/// Attempt a reverse DNS lookup for the given IP address.
///
/// Returns `None` since the standard library does not support PTR record
/// lookups directly. A future integration could use the `dns-lookup` crate.
fn reverse_dns(_ip: IpAddr) -> Option<String> {
    // PTR record lookup not supported by std; would require dns-lookup crate
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numeric_to_timing() {
        assert_eq!(numeric_to_timing(0), TimingTemplate::Paranoid);
        assert_eq!(numeric_to_timing(1), TimingTemplate::Sneaky);
        assert_eq!(numeric_to_timing(2), TimingTemplate::Polite);
        assert_eq!(numeric_to_timing(3), TimingTemplate::Normal);
        assert_eq!(numeric_to_timing(4), TimingTemplate::Aggressive);
        assert_eq!(numeric_to_timing(5), TimingTemplate::Insane);
        assert_eq!(numeric_to_timing(255), TimingTemplate::Normal);
    }

    #[test]
    fn test_timing_to_prtip() {
        assert_eq!(
            timing_to_prtip(TimingTemplate::Paranoid),
            prtip_core::TimingTemplate::Paranoid
        );
        assert_eq!(
            timing_to_prtip(TimingTemplate::Normal),
            prtip_core::TimingTemplate::Normal
        );
        assert_eq!(
            timing_to_prtip(TimingTemplate::Insane),
            prtip_core::TimingTemplate::Insane
        );
    }

    #[test]
    fn test_port_state_from_prtip() {
        assert_eq!(
            port_state_from_prtip(prtip_core::PortState::Open),
            PortState::Open
        );
        assert_eq!(
            port_state_from_prtip(prtip_core::PortState::Closed),
            PortState::Closed
        );
        assert_eq!(
            port_state_from_prtip(prtip_core::PortState::Filtered),
            PortState::Filtered
        );
        assert_eq!(
            port_state_from_prtip(prtip_core::PortState::Unknown),
            PortState::Unknown
        );
    }

    #[test]
    fn test_protocol_from_prtip() {
        assert_eq!(
            _protocol_from_prtip(prtip_core::Protocol::Tcp),
            Protocol::Tcp
        );
        assert_eq!(
            _protocol_from_prtip(prtip_core::Protocol::Udp),
            Protocol::Udp
        );
        assert_eq!(
            _protocol_from_prtip(prtip_core::Protocol::Icmp),
            Protocol::Tcp
        );
    }

    #[test]
    fn test_protocol_for_scan_type() {
        assert_eq!(protocol_for_scan_type(&ScanType::Udp), Protocol::Udp);
        assert_eq!(protocol_for_scan_type(&ScanType::Syn), Protocol::Tcp);
        assert_eq!(protocol_for_scan_type(&ScanType::Connect), Protocol::Tcp);
        assert_eq!(protocol_for_scan_type(&ScanType::Fin), Protocol::Tcp);
    }

    #[test]
    fn test_resolve_target_ip() {
        let target = Target::Ip("127.0.0.1".parse().unwrap());
        let ips = resolve_target(&target).unwrap();
        assert_eq!(ips.len(), 1);
        assert_eq!(ips[0].to_string(), "127.0.0.1");
    }

    #[test]
    fn test_resolve_target_cidr() {
        let target = Target::Cidr("192.168.1.0/30".parse().unwrap());
        let ips = resolve_target(&target).unwrap();
        assert_eq!(ips.len(), 4);
    }

    #[test]
    fn test_aggregate_results_empty() {
        let target = Target::Ip("192.168.1.1".parse().unwrap());
        let ip: IpAddr = "192.168.1.1".parse().unwrap();
        let result = aggregate_results(
            &target,
            ip,
            vec![],
            Protocol::Tcp,
            false,
            Duration::from_millis(100),
        );
        assert_eq!(result.host, "192.168.1.1");
        assert!(result.ports.is_empty());
        assert_eq!(result.scan_time, Duration::from_millis(100));
    }

    #[test]
    fn test_aggregate_results_filters_closed() {
        let target = Target::Ip("192.168.1.1".parse().unwrap());
        let ip: IpAddr = "192.168.1.1".parse().unwrap();

        let prtip_results = vec![
            prtip_core::ScanResult::new(ip, 80, prtip_core::PortState::Open),
            prtip_core::ScanResult::new(ip, 81, prtip_core::PortState::Closed),
            prtip_core::ScanResult::new(ip, 443, prtip_core::PortState::Filtered),
        ];

        let result = aggregate_results(
            &target,
            ip,
            prtip_results,
            Protocol::Tcp,
            false,
            Duration::from_millis(200),
        );

        // Closed ports should be filtered out
        assert_eq!(result.ports.len(), 2);
        assert_eq!(result.ports[0].port, 80);
        assert_eq!(result.ports[0].state, PortState::Open);
        assert_eq!(result.ports[1].port, 443);
        assert_eq!(result.ports[1].state, PortState::Filtered);
    }

    #[test]
    fn test_aggregate_results_preserves_service_info() {
        let target = Target::Ip("192.168.1.1".parse().unwrap());
        let ip: IpAddr = "192.168.1.1".parse().unwrap();

        let mut prtip_result = prtip_core::ScanResult::new(ip, 80, prtip_core::PortState::Open);
        prtip_result.service = Some("http".to_string());
        prtip_result.version = Some("nginx/1.24".to_string());
        prtip_result.banner = Some("Welcome to nginx".to_string());

        let result = aggregate_results(
            &target,
            ip,
            vec![prtip_result],
            Protocol::Tcp,
            false,
            Duration::from_millis(100),
        );

        assert_eq!(result.ports.len(), 1);
        assert_eq!(result.ports[0].service, Some("http".to_string()));
        assert_eq!(result.ports[0].version, Some("nginx/1.24".to_string()));
        assert_eq!(result.ports[0].banner, Some("Welcome to nginx".to_string()));
    }

    #[test]
    fn test_aggregate_results_udp_protocol() {
        let target = Target::Ip("192.168.1.1".parse().unwrap());
        let ip: IpAddr = "192.168.1.1".parse().unwrap();

        let prtip_results = vec![prtip_core::ScanResult::new(
            ip,
            53,
            prtip_core::PortState::Open,
        )];

        let result = aggregate_results(
            &target,
            ip,
            prtip_results,
            Protocol::Udp,
            false,
            Duration::from_millis(100),
        );

        assert_eq!(result.ports.len(), 1);
        assert_eq!(result.ports[0].protocol, Protocol::Udp);
    }

    #[test]
    fn test_prtip_scanner_new() {
        let config = Config::default();
        let scanner = PrtipScanner::new(&config);
        assert!(scanner.is_ok());
    }

    #[test]
    fn test_prtip_scanner_requires_root() {
        let config = Config::default();
        let scanner = PrtipScanner::new(&config).unwrap();
        assert!(scanner.requires_root());
    }

    #[test]
    fn test_prtip_scanner_interface_none() {
        let config = Config::default();
        let scanner = PrtipScanner::new(&config).unwrap();
        assert!(scanner.interface().is_none());
    }

    #[test]
    fn test_prtip_scanner_interface_configured() {
        let mut config = Config::default();
        config.scan.interface = Some("eth0".to_string());
        let scanner = PrtipScanner::new(&config).unwrap();
        assert_eq!(scanner.interface(), Some("eth0"));
    }

    #[tokio::test]
    async fn test_prtip_scanner_tcp_connect_localhost() {
        let config = Config::default();
        let scanner = PrtipScanner::new(&config).unwrap();

        let scan_config = ScanConfig {
            scan_type: ScanType::Connect,
            targets: vec![Target::Ip("127.0.0.1".parse().unwrap())],
            ports: vec![9999], // Likely closed port
            ..Default::default()
        };

        let results = scanner.scan(scan_config, None).await.unwrap();

        // Should have one result for the single target
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].host, "127.0.0.1");
    }
}
