//! Markdown report generator

use super::ReportData;

/// Markdown report generator
pub struct MarkdownReportGenerator;

impl MarkdownReportGenerator {
    /// Generate a Markdown report
    pub fn generate(data: &ReportData) -> crate::Result<String> {
        let mut md = String::with_capacity(4096);

        // Title
        md.push_str(&format!("# {}\n\n", data.title));
        md.push_str(&format!("**Date:** {}\n\n", data.date));
        if let Some(ref campaign) = data.campaign {
            md.push_str(&format!("**Campaign:** {}\n\n", campaign));
        }
        md.push_str("---\n\n");

        // Executive Summary
        md.push_str("## Executive Summary\n\n");
        md.push_str("| Metric | Value |\n|--------|-------|\n");
        md.push_str(&format!("| Total Hosts | {} |\n", data.summary.total_hosts));
        md.push_str(&format!("| Hosts Up | {} |\n", data.summary.hosts_up));
        md.push_str(&format!(
            "| Open Ports | {} |\n",
            data.summary.total_open_ports
        ));
        md.push_str(&format!(
            "| Total Findings | {} |\n",
            data.summary.total_findings
        ));
        md.push_str(&format!(
            "| Risk Score | {} ({}) |\n",
            data.summary.risk_score,
            data.summary.risk_rating()
        ));
        md.push('\n');

        // Severity Breakdown
        if data.summary.total_findings > 0 {
            md.push_str("### Severity Breakdown\n\n");
            md.push_str("| Severity | Count |\n|----------|-------|\n");
            md.push_str(&format!("| Critical | {} |\n", data.summary.critical_count));
            md.push_str(&format!("| High | {} |\n", data.summary.high_count));
            md.push_str(&format!("| Medium | {} |\n", data.summary.medium_count));
            md.push_str(&format!("| Low | {} |\n", data.summary.low_count));
            md.push_str(&format!("| Info | {} |\n", data.summary.info_count));
            md.push('\n');
        }

        // Findings
        if !data.findings.is_empty() {
            md.push_str("## Findings\n\n");
            md.push_str("| Severity | Title | Host | Port | Service |\n");
            md.push_str("|----------|-------|------|------|--------|\n");

            for finding in &data.findings {
                let port = finding
                    .port
                    .map(|p| p.to_string())
                    .unwrap_or_else(|| "-".to_string());
                let service = finding.service.as_deref().unwrap_or("-");

                md.push_str(&format!(
                    "| {} | {} | {} | {} | {} |\n",
                    finding.severity, finding.title, finding.host, port, service
                ));
            }
            md.push('\n');

            // Detailed findings
            md.push_str("### Finding Details\n\n");
            for finding in &data.findings {
                md.push_str(&format!(
                    "#### {} ({})\n\n",
                    finding.title, finding.severity
                ));
                md.push_str(&format!("**Host:** {}\n\n", finding.host));

                if !finding.description.is_empty() {
                    md.push_str(&format!("{}\n\n", finding.description));
                }

                if let Some(ref remediation) = finding.remediation {
                    md.push_str(&format!("**Remediation:** {}\n\n", remediation));
                }

                if !finding.references.is_empty() {
                    md.push_str("**References:**\n");
                    for reference in &finding.references {
                        md.push_str(&format!("- {}\n", reference));
                    }
                    md.push('\n');
                }
            }
        }

        // Host Details
        if !data.hosts.is_empty() {
            md.push_str("## Host Details\n\n");

            for host in &data.hosts {
                md.push_str(&format!("### {}", host.ip));
                if let Some(ref hostname) = host.hostname {
                    md.push_str(&format!(" ({})", hostname));
                }
                md.push_str("\n\n");

                if let Some(ref os) = host.os {
                    md.push_str(&format!("**OS:** {}", os));
                    if let Some(ref ver) = host.os_version {
                        md.push_str(&format!(" {}", ver));
                    }
                    md.push_str("\n\n");
                }

                if !host.services.is_empty() {
                    md.push_str("| Port | Protocol | State | Service | Version |\n");
                    md.push_str("|------|----------|-------|---------|--------|\n");

                    for svc in &host.services {
                        let name = svc.service_name.as_deref().unwrap_or("-");
                        let ver = svc.version.as_deref().unwrap_or("-");
                        md.push_str(&format!(
                            "| {} | {} | {} | {} | {} |\n",
                            svc.port, svc.protocol, svc.state, name, ver
                        ));
                    }
                    md.push('\n');
                }
            }
        }

        // Footer
        md.push_str("---\n\n");
        md.push_str(&format!(
            "*Generated by SPECTRE v{}*\n",
            env!("CARGO_PKG_VERSION")
        ));

        Ok(md)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::report::ReportData;
    use crate::results::{Finding, FindingSeverity, Host, Service};

    fn make_report_data() -> ReportData {
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
                version: Some("nginx 1.24".to_string()),
                banner: None,
            }],
            scan_time_ms: 100,
        }];

        let findings = vec![
            Finding::new("Open HTTP Port", "10.0.0.1", FindingSeverity::Info)
                .with_port(80)
                .with_service("http")
                .with_description("HTTP port is publicly accessible")
                .with_remediation("Consider restricting access"),
        ];

        let summary = crate::report::ExecutiveSummary::from_hosts_and_findings(&hosts, &findings);

        ReportData {
            title: "Test Markdown Report".to_string(),
            date: "2025-06-15".to_string(),
            campaign: Some("Op Test".to_string()),
            hosts,
            findings,
            summary,
        }
    }

    #[test]
    fn test_markdown_generation() {
        let data = make_report_data();
        let md = MarkdownReportGenerator::generate(&data).unwrap();
        assert!(md.contains("# Test Markdown Report"));
        assert!(md.contains("10.0.0.1"));
    }

    #[test]
    fn test_markdown_contains_summary() {
        let data = make_report_data();
        let md = MarkdownReportGenerator::generate(&data).unwrap();
        assert!(md.contains("## Executive Summary"));
        assert!(md.contains("| Total Hosts |"));
    }

    #[test]
    fn test_markdown_contains_findings() {
        let data = make_report_data();
        let md = MarkdownReportGenerator::generate(&data).unwrap();
        assert!(md.contains("## Findings"));
        assert!(md.contains("Open HTTP Port"));
    }

    #[test]
    fn test_markdown_contains_host_details() {
        let data = make_report_data();
        let md = MarkdownReportGenerator::generate(&data).unwrap();
        assert!(md.contains("## Host Details"));
        assert!(md.contains("web.example.com"));
    }

    #[test]
    fn test_markdown_empty_data() {
        let data = ReportData::from_results("Empty", &[], &[]);
        let md = MarkdownReportGenerator::generate(&data).unwrap();
        assert!(md.contains("# Empty"));
    }

    #[test]
    fn test_markdown_finding_details() {
        let data = make_report_data();
        let md = MarkdownReportGenerator::generate(&data).unwrap();
        assert!(md.contains("HTTP port is publicly accessible"));
        assert!(md.contains("Consider restricting access"));
    }
}
