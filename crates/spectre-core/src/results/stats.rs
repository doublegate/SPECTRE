//! Result statistics computation

use std::collections::HashMap;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::scan::{PortState, ScanResult};

/// Statistics computed from scan results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultStats {
    /// Total number of hosts scanned
    pub total_hosts: usize,
    /// Number of hosts with at least one open port
    pub hosts_up: usize,
    /// Total number of open ports found
    pub open_ports: usize,
    /// Total number of closed ports found
    pub closed_ports: usize,
    /// Total number of filtered ports found
    pub filtered_ports: usize,
    /// Unique services found
    pub unique_services: usize,
    /// Service distribution (service_name -> count)
    pub service_distribution: HashMap<String, usize>,
    /// Port distribution (port -> count of hosts with port open)
    pub port_distribution: HashMap<u16, usize>,
    /// Total scan time
    pub total_scan_time: Duration,
    /// Average scan time per host
    pub avg_scan_time: Duration,
}

impl ResultStats {
    /// Compute statistics from scan results
    pub fn from_results(results: &[ScanResult]) -> Self {
        let total_hosts = results.len();
        let mut hosts_up = 0;
        let mut open_ports = 0;
        let mut closed_ports = 0;
        let mut filtered_ports = 0;
        let mut service_distribution: HashMap<String, usize> = HashMap::new();
        let mut port_distribution: HashMap<u16, usize> = HashMap::new();
        let mut total_scan_time = Duration::ZERO;

        for result in results {
            let has_open = result
                .ports
                .iter()
                .any(|p| matches!(p.state, PortState::Open));

            if has_open {
                hosts_up += 1;
            }

            for port in &result.ports {
                match port.state {
                    PortState::Open => {
                        open_ports += 1;
                        *port_distribution.entry(port.port).or_insert(0) += 1;
                    },
                    PortState::Closed => closed_ports += 1,
                    PortState::Filtered | PortState::OpenFiltered | PortState::ClosedFiltered => {
                        filtered_ports += 1;
                    },
                    PortState::Unknown => {},
                }

                if let Some(ref service) = port.service {
                    *service_distribution.entry(service.clone()).or_insert(0) += 1;
                }
            }

            total_scan_time += result.scan_time;
        }

        let unique_services = service_distribution.len();
        let avg_scan_time = if total_hosts > 0 {
            total_scan_time / total_hosts as u32
        } else {
            Duration::ZERO
        };

        Self {
            total_hosts,
            hosts_up,
            open_ports,
            closed_ports,
            filtered_ports,
            unique_services,
            service_distribution,
            port_distribution,
            total_scan_time,
            avg_scan_time,
        }
    }

    /// Get the most common open ports
    pub fn top_ports(&self, limit: usize) -> Vec<(u16, usize)> {
        let mut ports: Vec<(u16, usize)> = self.port_distribution.clone().into_iter().collect();
        ports.sort_by(|a, b| b.1.cmp(&a.1));
        ports.truncate(limit);
        ports
    }

    /// Get the most common services
    pub fn top_services(&self, limit: usize) -> Vec<(String, usize)> {
        let mut services: Vec<(String, usize)> =
            self.service_distribution.clone().into_iter().collect();
        services.sort_by(|a, b| b.1.cmp(&a.1));
        services.truncate(limit);
        services
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scan::{PortResult, Protocol, ScanResult, Target};

    fn make_result(host: &str, ports: Vec<(u16, PortState, Option<&str>)>) -> ScanResult {
        ScanResult {
            host: host.to_string(),
            ip: Target::Ip(host.parse().unwrap()),
            hostname: None,
            ports: ports
                .into_iter()
                .map(|(port, state, service)| PortResult {
                    port,
                    protocol: Protocol::Tcp,
                    state,
                    service: service.map(String::from),
                    version: None,
                    banner: None,
                })
                .collect(),
            os: None,
            scan_time: Duration::from_millis(100),
        }
    }

    #[test]
    fn test_stats_from_results() {
        let results = vec![
            make_result(
                "10.0.0.1",
                vec![
                    (22, PortState::Open, Some("ssh")),
                    (80, PortState::Open, Some("http")),
                    (443, PortState::Filtered, None),
                ],
            ),
            make_result(
                "10.0.0.2",
                vec![
                    (22, PortState::Open, Some("ssh")),
                    (80, PortState::Closed, None),
                ],
            ),
            make_result("10.0.0.3", vec![(80, PortState::Closed, None)]),
        ];

        let stats = ResultStats::from_results(&results);

        assert_eq!(stats.total_hosts, 3);
        assert_eq!(stats.hosts_up, 2);
        assert_eq!(stats.open_ports, 3);
        assert_eq!(stats.closed_ports, 2);
        assert_eq!(stats.filtered_ports, 1);
        assert_eq!(stats.unique_services, 2); // ssh, http
    }

    #[test]
    fn test_top_ports() {
        let results = vec![
            make_result("10.0.0.1", vec![(22, PortState::Open, Some("ssh"))]),
            make_result("10.0.0.2", vec![(22, PortState::Open, Some("ssh"))]),
            make_result("10.0.0.3", vec![(80, PortState::Open, Some("http"))]),
        ];

        let stats = ResultStats::from_results(&results);
        let top = stats.top_ports(5);

        assert_eq!(top[0], (22, 2)); // Port 22 most common
    }

    #[test]
    fn test_top_services() {
        let results = vec![
            make_result("10.0.0.1", vec![(22, PortState::Open, Some("ssh"))]),
            make_result("10.0.0.2", vec![(22, PortState::Open, Some("ssh"))]),
            make_result("10.0.0.3", vec![(80, PortState::Open, Some("http"))]),
        ];

        let stats = ResultStats::from_results(&results);
        let top = stats.top_services(5);

        assert_eq!(top[0].0, "ssh");
        assert_eq!(top[0].1, 2);
    }

    #[test]
    fn test_empty_results_stats() {
        let stats = ResultStats::from_results(&[]);
        assert_eq!(stats.total_hosts, 0);
        assert_eq!(stats.hosts_up, 0);
        assert_eq!(stats.open_ports, 0);
    }
}
