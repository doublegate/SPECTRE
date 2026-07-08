//! Executive summary generation

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::results::{Finding, FindingSeverity, Host};

/// Executive summary — high-level overview of findings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutiveSummary {
    /// Total hosts scanned
    pub total_hosts: usize,
    /// Hosts with open ports
    pub hosts_up: usize,
    /// Total open ports
    pub total_open_ports: usize,
    /// Total findings
    pub total_findings: usize,
    /// Findings by severity
    pub severity_breakdown: HashMap<String, usize>,
    /// Critical finding count
    pub critical_count: usize,
    /// High finding count
    pub high_count: usize,
    /// Medium finding count
    pub medium_count: usize,
    /// Low finding count
    pub low_count: usize,
    /// Info finding count
    pub info_count: usize,
    /// Top affected hosts (by finding count)
    pub top_affected_hosts: Vec<(String, usize)>,
    /// Risk rating (0-100)
    pub risk_score: u32,
}

impl ExecutiveSummary {
    /// Generate executive summary from hosts and findings
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    pub fn from_hosts_and_findings(hosts: &[Host], findings: &[Finding]) -> Self {
        let total_hosts = hosts.len();
        let hosts_up = hosts.iter().filter(|h| h.open_services() > 0).count();
        let total_open_ports: usize = hosts.iter().map(Host::open_services).sum();

        let mut severity_breakdown = HashMap::new();
        let mut host_finding_count: HashMap<String, usize> = HashMap::new();
        let mut critical_count = 0;
        let mut high_count = 0;
        let mut medium_count = 0;
        let mut low_count = 0;
        let mut info_count = 0;

        for finding in findings {
            match finding.severity {
                FindingSeverity::Critical => critical_count += 1,
                FindingSeverity::High => high_count += 1,
                FindingSeverity::Medium => medium_count += 1,
                FindingSeverity::Low => low_count += 1,
                FindingSeverity::Info => info_count += 1,
            }

            *severity_breakdown
                .entry(finding.severity.to_string())
                .or_insert(0) += 1;

            *host_finding_count.entry(finding.host.clone()).or_insert(0) += 1;
        }

        // Top affected hosts
        let mut top_affected: Vec<(String, usize)> = host_finding_count.into_iter().collect();
        top_affected.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
        top_affected.truncate(10);

        // Risk score calculation
        let risk_score = Self::calculate_risk_score(
            critical_count,
            high_count,
            medium_count,
            low_count,
            total_hosts,
        );

        Self {
            total_hosts,
            hosts_up,
            total_open_ports,
            total_findings: findings.len(),
            severity_breakdown,
            critical_count,
            high_count,
            medium_count,
            low_count,
            info_count,
            top_affected_hosts: top_affected,
            risk_score,
        }
    }

    /// Calculate risk score (0-100)
    fn calculate_risk_score(
        critical: usize,
        high: usize,
        medium: usize,
        low: usize,
        total_hosts: usize,
    ) -> u32 {
        if total_hosts == 0 {
            return 0;
        }

        let weighted_score = (critical * 40 + high * 20 + medium * 10 + low * 5) as f64;
        let normalized = (weighted_score / total_hosts as f64).min(100.0);
        normalized as u32
    }

    /// Get a text-based risk rating
    pub const fn risk_rating(&self) -> &str {
        match self.risk_score {
            0..=20 => "Low",
            21..=40 => "Moderate",
            41..=60 => "Elevated",
            61..=80 => "High",
            _ => "Critical",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::results::{Finding, FindingSeverity, Host, Service};

    fn make_host(ip: &str, open_ports: usize) -> Host {
        Host {
            ip: ip.to_string(),
            hostname: None,
            os: None,
            os_version: None,
            services: (0..open_ports)
                .map(|i| Service {
                    port: 80 + i as u16,
                    protocol: "tcp".to_string(),
                    state: "Open".to_string(),
                    service_name: Some("http".to_string()),
                    version: None,
                    banner: None,
                })
                .collect(),
            scan_time_ms: 100,
        }
    }

    #[test]
    fn test_empty_summary() {
        let summary = ExecutiveSummary::from_hosts_and_findings(&[], &[]);
        assert_eq!(summary.total_hosts, 0);
        assert_eq!(summary.total_findings, 0);
        assert_eq!(summary.risk_score, 0);
    }

    #[test]
    fn test_summary_with_hosts() {
        let hosts = vec![make_host("10.0.0.1", 2), make_host("10.0.0.2", 0)];
        let summary = ExecutiveSummary::from_hosts_and_findings(&hosts, &[]);
        assert_eq!(summary.total_hosts, 2);
        assert_eq!(summary.hosts_up, 1);
        assert_eq!(summary.total_open_ports, 2);
    }

    #[test]
    fn test_summary_with_findings() {
        let hosts = vec![make_host("10.0.0.1", 2)];
        let findings = vec![
            Finding::new("Critical vuln", "10.0.0.1", FindingSeverity::Critical),
            Finding::new("High vuln", "10.0.0.1", FindingSeverity::High),
            Finding::new("Info", "10.0.0.1", FindingSeverity::Info),
        ];

        let summary = ExecutiveSummary::from_hosts_and_findings(&hosts, &findings);
        assert_eq!(summary.total_findings, 3);
        assert_eq!(summary.critical_count, 1);
        assert_eq!(summary.high_count, 1);
        assert_eq!(summary.info_count, 1);
    }

    #[test]
    fn test_risk_rating() {
        let mut summary = ExecutiveSummary::from_hosts_and_findings(&[], &[]);
        summary.risk_score = 10;
        assert_eq!(summary.risk_rating(), "Low");

        summary.risk_score = 50;
        assert_eq!(summary.risk_rating(), "Elevated");

        summary.risk_score = 90;
        assert_eq!(summary.risk_rating(), "Critical");
    }

    #[test]
    fn test_top_affected_hosts() {
        let hosts = vec![make_host("10.0.0.1", 1)];
        let findings = vec![
            Finding::new("vuln1", "10.0.0.1", FindingSeverity::High),
            Finding::new("vuln2", "10.0.0.1", FindingSeverity::Medium),
            Finding::new("vuln3", "10.0.0.2", FindingSeverity::Low),
        ];

        let summary = ExecutiveSummary::from_hosts_and_findings(&hosts, &findings);
        assert!(!summary.top_affected_hosts.is_empty());
        assert_eq!(summary.top_affected_hosts[0].0, "10.0.0.1");
        assert_eq!(summary.top_affected_hosts[0].1, 2);
    }

    #[test]
    fn test_severity_breakdown() {
        let findings = vec![
            Finding::new("v1", "10.0.0.1", FindingSeverity::High),
            Finding::new("v2", "10.0.0.1", FindingSeverity::High),
            Finding::new("v3", "10.0.0.1", FindingSeverity::Medium),
        ];

        let summary = ExecutiveSummary::from_hosts_and_findings(&[], &findings);
        assert_eq!(summary.severity_breakdown.get("high"), Some(&2));
        assert_eq!(summary.severity_breakdown.get("medium"), Some(&1));
    }
}
