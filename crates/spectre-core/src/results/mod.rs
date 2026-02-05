//! Results aggregation and formatting module
//!
//! Provides enhanced result handling including:
//! - Finding struct with vulnerability-style classification
//! - Host struct with services collection
//! - Result deduplication and filtering
//! - Result sorting
//! - Multiple output formats (JSON, XML, greppable)
//! - Statistics computation

mod finding;
mod output;
mod stats;

pub use finding::{Finding, FindingSeverity, Host, Service};
pub use output::{format_greppable, format_json, format_xml, OutputFormat};
pub use stats::ResultStats;

use crate::scan::ScanResult;

/// Aggregate scan results into host-centric findings
///
/// Converts raw scan results into a more structured format
/// with host-level aggregation and service grouping.
pub fn aggregate_results(results: &[ScanResult]) -> Vec<Host> {
    let mut hosts = Vec::new();

    for result in results {
        let mut services = Vec::new();

        for port_result in &result.ports {
            services.push(Service {
                port: port_result.port,
                protocol: port_result.protocol.to_string(),
                state: format!("{:?}", port_result.state),
                service_name: port_result.service.clone(),
                version: port_result.version.clone(),
                banner: port_result.banner.clone(),
            });
        }

        hosts.push(Host {
            ip: result.host.clone(),
            hostname: result.hostname.clone(),
            os: result.os.as_ref().map(|os| os.name.clone()),
            os_version: result.os.as_ref().and_then(|os| os.version.clone()),
            services,
            scan_time_ms: result.scan_time.as_millis() as u64,
        });
    }

    hosts
}

/// Deduplicate results by IP address, merging port information
pub fn deduplicate_results(results: &mut Vec<ScanResult>) {
    use std::collections::HashMap;

    let mut seen: HashMap<String, usize> = HashMap::new();
    let mut deduped = Vec::new();

    for result in results.drain(..) {
        if let Some(&idx) = seen.get(&result.host) {
            // Merge ports into existing result
            let existing: &mut ScanResult = &mut deduped[idx];
            for port in result.ports {
                if !existing.ports.iter().any(|p| p.port == port.port) {
                    existing.ports.push(port);
                }
            }
        } else {
            seen.insert(result.host.clone(), deduped.len());
            deduped.push(result);
        }
    }

    *results = deduped;
}

/// Sort results by the specified criterion
pub fn sort_results(results: &mut [ScanResult], criterion: SortCriterion) {
    match criterion {
        SortCriterion::Ip => {
            results.sort_by(|a, b| a.host.cmp(&b.host));
        },
        SortCriterion::OpenPorts => {
            results.sort_by(|a, b| {
                let a_open = a
                    .ports
                    .iter()
                    .filter(|p| matches!(p.state, crate::scan::PortState::Open))
                    .count();
                let b_open = b
                    .ports
                    .iter()
                    .filter(|p| matches!(p.state, crate::scan::PortState::Open))
                    .count();
                b_open.cmp(&a_open)
            });
        },
        SortCriterion::ScanTime => {
            results.sort_by(|a, b| a.scan_time.cmp(&b.scan_time));
        },
    }
}

/// Sort criterion for results
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortCriterion {
    /// Sort by IP address
    Ip,
    /// Sort by number of open ports (descending)
    OpenPorts,
    /// Sort by scan time
    ScanTime,
}

/// Filter results to only include hosts with open ports
pub fn filter_open_ports(results: &[ScanResult]) -> Vec<ScanResult> {
    results
        .iter()
        .filter(|r| {
            r.ports
                .iter()
                .any(|p| matches!(p.state, crate::scan::PortState::Open))
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scan::{PortResult, PortState, Protocol, ScanResult, Target};
    use std::time::Duration;

    fn make_result(host: &str, ports: Vec<(u16, PortState)>) -> ScanResult {
        ScanResult {
            host: host.to_string(),
            ip: Target::Ip(host.parse().unwrap()),
            hostname: None,
            ports: ports
                .into_iter()
                .map(|(port, state)| PortResult {
                    port,
                    protocol: Protocol::Tcp,
                    state,
                    service: None,
                    version: None,
                    banner: None,
                })
                .collect(),
            os: None,
            scan_time: Duration::from_millis(100),
        }
    }

    #[test]
    fn test_aggregate_results() {
        let results = vec![make_result("10.0.0.1", vec![(80, PortState::Open)])];
        let hosts = aggregate_results(&results);
        assert_eq!(hosts.len(), 1);
        assert_eq!(hosts[0].ip, "10.0.0.1");
        assert_eq!(hosts[0].services.len(), 1);
    }

    #[test]
    fn test_deduplicate_results() {
        let mut results = vec![
            make_result("10.0.0.1", vec![(80, PortState::Open)]),
            make_result("10.0.0.1", vec![(443, PortState::Open)]),
            make_result("10.0.0.2", vec![(22, PortState::Open)]),
        ];

        deduplicate_results(&mut results);
        assert_eq!(results.len(), 2);
        // First host should have merged ports
        assert_eq!(results[0].ports.len(), 2);
    }

    #[test]
    fn test_sort_by_ip() {
        let mut results = vec![
            make_result("10.0.0.3", vec![]),
            make_result("10.0.0.1", vec![]),
            make_result("10.0.0.2", vec![]),
        ];

        sort_results(&mut results, SortCriterion::Ip);
        assert_eq!(results[0].host, "10.0.0.1");
        assert_eq!(results[1].host, "10.0.0.2");
        assert_eq!(results[2].host, "10.0.0.3");
    }

    #[test]
    fn test_sort_by_open_ports() {
        let mut results = vec![
            make_result("10.0.0.1", vec![(80, PortState::Open)]),
            make_result(
                "10.0.0.2",
                vec![(80, PortState::Open), (443, PortState::Open)],
            ),
            make_result("10.0.0.3", vec![]),
        ];

        sort_results(&mut results, SortCriterion::OpenPorts);
        assert_eq!(results[0].host, "10.0.0.2"); // Most open ports first
    }

    #[test]
    fn test_filter_open_ports() {
        let results = vec![
            make_result("10.0.0.1", vec![(80, PortState::Open)]),
            make_result("10.0.0.2", vec![(80, PortState::Closed)]),
            make_result("10.0.0.3", vec![(22, PortState::Open)]),
        ];

        let filtered = filter_open_ports(&results);
        assert_eq!(filtered.len(), 2);
    }
}
