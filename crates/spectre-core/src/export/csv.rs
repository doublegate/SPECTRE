//! CSV export for scan results, findings, and host summaries

use crate::results::{Finding, Host};
use crate::scan::ScanResult;

/// CSV exporter
pub struct CsvExporter;

impl CsvExporter {
    /// Export scan results to CSV
    pub fn export_scan_results(results: &[ScanResult]) -> String {
        let mut csv = String::from("host,port,protocol,state,service,version\n");

        for result in results {
            for port in &result.ports {
                let service = port.service.as_deref().unwrap_or("");
                let version = port.version.as_deref().unwrap_or("");

                csv.push_str(&format!(
                    "{},{},{},{:?},{},{}\n",
                    Self::escape_csv(&result.host),
                    port.port,
                    port.protocol,
                    port.state,
                    Self::escape_csv(service),
                    Self::escape_csv(version),
                ));
            }
        }

        csv
    }

    /// Export findings to CSV
    pub fn export_findings(findings: &[Finding]) -> String {
        let mut csv = String::from("severity,title,host,port,service,description,remediation\n");

        for finding in findings {
            let port = finding.port.map(|p| p.to_string()).unwrap_or_default();
            let service = finding.service.as_deref().unwrap_or("");
            let remediation = finding.remediation.as_deref().unwrap_or("");

            csv.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                finding.severity,
                Self::escape_csv(&finding.title),
                Self::escape_csv(&finding.host),
                port,
                Self::escape_csv(service),
                Self::escape_csv(&finding.description),
                Self::escape_csv(remediation),
            ));
        }

        csv
    }

    /// Export host summaries to CSV
    pub fn export_hosts(hosts: &[Host]) -> String {
        let mut csv = String::from("ip,hostname,os,os_version,open_services,scan_time_ms\n");

        for host in hosts {
            let hostname = host.hostname.as_deref().unwrap_or("");
            let os = host.os.as_deref().unwrap_or("");
            let os_ver = host.os_version.as_deref().unwrap_or("");

            csv.push_str(&format!(
                "{},{},{},{},{},{}\n",
                Self::escape_csv(&host.ip),
                Self::escape_csv(hostname),
                Self::escape_csv(os),
                Self::escape_csv(os_ver),
                host.open_services(),
                host.scan_time_ms,
            ));
        }

        csv
    }

    /// Escape a value for CSV (double-quote if contains comma, quote, or newline)
    fn escape_csv(value: &str) -> String {
        if value.contains(',') || value.contains('"') || value.contains('\n') {
            format!("\"{}\"", value.replace('"', "\"\""))
        } else {
            value.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::results::{Finding, FindingSeverity, Host, Service};
    use crate::scan::{PortResult, PortState, Protocol, ScanResult, Target};
    use std::time::Duration;

    fn make_result() -> ScanResult {
        ScanResult {
            host: "10.0.0.1".to_string(),
            ip: Target::Ip("10.0.0.1".parse().unwrap()),
            hostname: None,
            ports: vec![PortResult {
                port: 80,
                protocol: Protocol::Tcp,
                state: PortState::Open,
                service: Some("http".to_string()),
                version: Some("nginx 1.24".to_string()),
                banner: None,
            }],
            os: None,
            scan_time: Duration::from_millis(100),
        }
    }

    #[test]
    fn test_export_scan_results() {
        let results = vec![make_result()];
        let csv = CsvExporter::export_scan_results(&results);
        assert!(csv.contains("host,port,protocol"));
        assert!(csv.contains("10.0.0.1"));
        assert!(csv.contains("80"));
        assert!(csv.contains("http"));
    }

    #[test]
    fn test_export_findings() {
        let findings = vec![
            Finding::new("Test Finding", "10.0.0.1", FindingSeverity::High)
                .with_port(80)
                .with_service("http"),
        ];

        let csv = CsvExporter::export_findings(&findings);
        assert!(csv.contains("severity,title"));
        assert!(csv.contains("high"));
        assert!(csv.contains("Test Finding"));
    }

    #[test]
    fn test_export_hosts() {
        let hosts = vec![Host {
            ip: "10.0.0.1".to_string(),
            hostname: Some("web.example.com".to_string()),
            os: Some("Linux".to_string()),
            os_version: Some("5.15".to_string()),
            services: vec![Service {
                port: 80,
                protocol: "tcp".to_string(),
                state: "Open".to_string(),
                service_name: Some("http".to_string()),
                version: None,
                banner: None,
            }],
            scan_time_ms: 100,
        }];

        let csv = CsvExporter::export_hosts(&hosts);
        assert!(csv.contains("ip,hostname"));
        assert!(csv.contains("10.0.0.1"));
        assert!(csv.contains("web.example.com"));
    }

    #[test]
    fn test_export_empty() {
        let csv = CsvExporter::export_scan_results(&[]);
        assert!(csv.contains("host,port,protocol"));
        // Only header line
        assert_eq!(csv.lines().count(), 1);
    }

    #[test]
    fn test_escape_csv() {
        assert_eq!(CsvExporter::escape_csv("simple"), "simple");
        assert_eq!(CsvExporter::escape_csv("has,comma"), "\"has,comma\"");
        assert_eq!(CsvExporter::escape_csv("has\"quote"), "\"has\"\"quote\"");
    }

    #[test]
    fn test_csv_newline_escape() {
        assert_eq!(CsvExporter::escape_csv("line1\nline2"), "\"line1\nline2\"");
    }
}
