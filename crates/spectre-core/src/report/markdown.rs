//! Markdown report generator

use std::fmt::Write;

use super::ReportData;

/// Markdown report generator
pub struct MarkdownReportGenerator;

impl MarkdownReportGenerator {
    /// Generate a Markdown report
    pub fn generate(data: &ReportData) -> crate::Result<String> {
        let mut md = String::with_capacity(4096);

        // Title
        let _ = write!(md, "# {}\n\n", data.title);
        let _ = write!(md, "**Date:** {}\n\n", data.date);
        if let Some(ref campaign) = data.campaign {
            let _ = write!(md, "**Campaign:** {}\n\n", campaign);
        }
        md.push_str("---\n\n");

        // Executive Summary
        md.push_str("## Executive Summary\n\n");
        md.push_str("| Metric | Value |\n|--------|-------|\n");
        let _ = writeln!(md, "| Total Hosts | {} |", data.summary.total_hosts);
        let _ = writeln!(md, "| Hosts Up | {} |", data.summary.hosts_up);
        let _ = writeln!(md, "| Open Ports | {} |", data.summary.total_open_ports);
        let _ = writeln!(md, "| Total Findings | {} |", data.summary.total_findings);
        let _ = writeln!(
            md,
            "| Risk Score | {} ({}) |",
            data.summary.risk_score,
            data.summary.risk_rating()
        );
        md.push('\n');

        // Severity Breakdown
        if data.summary.total_findings > 0 {
            md.push_str("### Severity Breakdown\n\n");
            md.push_str("| Severity | Count |\n|----------|-------|\n");
            let _ = writeln!(md, "| Critical | {} |", data.summary.critical_count);
            let _ = writeln!(md, "| High | {} |", data.summary.high_count);
            let _ = writeln!(md, "| Medium | {} |", data.summary.medium_count);
            let _ = writeln!(md, "| Low | {} |", data.summary.low_count);
            let _ = writeln!(md, "| Info | {} |", data.summary.info_count);
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
                    .map_or_else(|| "-".to_string(), |p| p.to_string());
                let service = finding.service.as_deref().unwrap_or("-");

                let _ = writeln!(
                    md,
                    "| {} | {} | {} | {} | {} |",
                    finding.severity, finding.title, finding.host, port, service
                );
            }
            md.push('\n');

            // Detailed findings
            md.push_str("### Finding Details\n\n");
            for finding in &data.findings {
                let _ = write!(md, "#### {} ({})\n\n", finding.title, finding.severity);
                let _ = write!(md, "**Host:** {}\n\n", finding.host);

                if !finding.description.is_empty() {
                    let _ = write!(md, "{}\n\n", finding.description);
                }

                if let Some(ref remediation) = finding.remediation {
                    let _ = write!(md, "**Remediation:** {}\n\n", remediation);
                }

                if !finding.references.is_empty() {
                    md.push_str("**References:**\n");
                    for reference in &finding.references {
                        let _ = writeln!(md, "- {}", reference);
                    }
                    md.push('\n');
                }
            }
        }

        // Host Details
        if !data.hosts.is_empty() {
            md.push_str("## Host Details\n\n");

            for host in &data.hosts {
                let _ = write!(md, "### {}", host.ip);
                if let Some(ref hostname) = host.hostname {
                    let _ = write!(md, " ({})", hostname);
                }
                md.push_str("\n\n");

                if let Some(ref os) = host.os {
                    let _ = write!(md, "**OS:** {}", os);
                    if let Some(ref ver) = host.os_version {
                        let _ = write!(md, " {}", ver);
                    }
                    md.push_str("\n\n");
                }

                if !host.services.is_empty() {
                    md.push_str("| Port | Protocol | State | Service | Version |\n");
                    md.push_str("|------|----------|-------|---------|--------|\n");

                    for svc in &host.services {
                        let name = svc.service_name.as_deref().unwrap_or("-");
                        let ver = svc.version.as_deref().unwrap_or("-");
                        let _ = writeln!(
                            md,
                            "| {} | {} | {} | {} | {} |",
                            svc.port, svc.protocol, svc.state, name, ver
                        );
                    }
                    md.push('\n');
                }
            }
        }

        // Footer
        md.push_str("---\n\n");
        let _ = writeln!(md, "*Generated by SPECTRE v{}*", env!("CARGO_PKG_VERSION"));

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
