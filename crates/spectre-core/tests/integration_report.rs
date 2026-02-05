//! Integration tests -- report generation across modules
#![allow(clippy::disallowed_methods)]

use spectre_core::report::{
    generate_report, ExecutiveSummary, HtmlReportGenerator, MarkdownReportGenerator, ReportData,
    ReportFormat, ReportSection, ReportTemplate,
};
use spectre_core::results::{Finding, FindingSeverity, Host, Service};

fn make_hosts() -> Vec<Host> {
    vec![
        Host {
            ip: "10.0.0.1".to_string(),
            hostname: Some("web-server.example.com".to_string()),
            os: Some("Linux".to_string()),
            os_version: Some("5.15".to_string()),
            services: vec![
                Service {
                    port: 80,
                    protocol: "tcp".to_string(),
                    state: "Open".to_string(),
                    service_name: Some("http".to_string()),
                    version: Some("nginx 1.24".to_string()),
                    banner: Some("nginx".to_string()),
                },
                Service {
                    port: 443,
                    protocol: "tcp".to_string(),
                    state: "Open".to_string(),
                    service_name: Some("https".to_string()),
                    version: Some("nginx 1.24".to_string()),
                    banner: None,
                },
            ],
            scan_time_ms: 150,
        },
        Host {
            ip: "10.0.0.2".to_string(),
            hostname: Some("db-server.example.com".to_string()),
            os: Some("Linux".to_string()),
            os_version: Some("6.1".to_string()),
            services: vec![Service {
                port: 5432,
                protocol: "tcp".to_string(),
                state: "Open".to_string(),
                service_name: Some("postgresql".to_string()),
                version: Some("16.2".to_string()),
                banner: None,
            }],
            scan_time_ms: 100,
        },
        Host {
            ip: "10.0.0.3".to_string(),
            hostname: None,
            os: None,
            os_version: None,
            services: vec![],
            scan_time_ms: 50,
        },
    ]
}

fn make_findings() -> Vec<Finding> {
    vec![
        Finding::new(
            "Outdated TLS Configuration",
            "10.0.0.1",
            FindingSeverity::High,
        )
        .with_port(443)
        .with_service("https")
        .with_description("TLS 1.0/1.1 is enabled and should be disabled")
        .with_remediation("Disable TLS 1.0 and 1.1 in nginx configuration"),
        Finding::new("Default Web Server Page", "10.0.0.1", FindingSeverity::Info)
            .with_port(80)
            .with_service("http"),
        Finding::new(
            "Database Accessible Externally",
            "10.0.0.2",
            FindingSeverity::Critical,
        )
        .with_port(5432)
        .with_service("postgresql")
        .with_description("PostgreSQL is accessible from external networks")
        .with_remediation("Restrict access to internal network only"),
        Finding::new(
            "Missing HTTP Security Headers",
            "10.0.0.1",
            FindingSeverity::Medium,
        )
        .with_port(80)
        .with_service("http"),
    ]
}

#[test]
fn test_executive_summary_integration() {
    let hosts = make_hosts();
    let findings = make_findings();
    let summary = ExecutiveSummary::from_hosts_and_findings(&hosts, &findings);

    assert_eq!(summary.total_hosts, 3);
    assert_eq!(summary.hosts_up, 2);
    assert_eq!(summary.total_open_ports, 3);
    assert_eq!(summary.total_findings, 4);
    assert_eq!(summary.critical_count, 1);
    assert_eq!(summary.high_count, 1);
    assert_eq!(summary.medium_count, 1);
    assert_eq!(summary.info_count, 1);
    assert!(summary.risk_score > 0);
}

#[test]
fn test_html_report_full_pipeline() {
    let hosts = make_hosts();
    let findings = make_findings();
    let summary = ExecutiveSummary::from_hosts_and_findings(&hosts, &findings);

    let data = ReportData {
        title: "Security Assessment Report".to_string(),
        date: "2025-06-15".to_string(),
        campaign: Some("Op TEST".to_string()),
        hosts,
        findings,
        summary,
    };

    let html = HtmlReportGenerator::generate(&data).unwrap();

    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("Security Assessment Report"));
    assert!(html.contains("Executive Summary"));
    assert!(html.contains("10.0.0.1"));
    assert!(html.contains("web-server.example.com"));
    assert!(html.contains("Outdated TLS Configuration"));
    assert!(html.contains("Database Accessible Externally"));
    assert!(html.contains("<style>"));
}

#[test]
fn test_markdown_report_full_pipeline() {
    let hosts = make_hosts();
    let findings = make_findings();
    let summary = ExecutiveSummary::from_hosts_and_findings(&hosts, &findings);

    let data = ReportData {
        title: "Security Assessment Report".to_string(),
        date: "2025-06-15".to_string(),
        campaign: None,
        hosts,
        findings,
        summary,
    };

    let md = MarkdownReportGenerator::generate(&data).unwrap();

    assert!(md.contains("# Security Assessment Report"));
    assert!(md.contains("## Executive Summary"));
    assert!(md.contains("## Findings"));
    assert!(md.contains("## Host Details"));
    assert!(md.contains("10.0.0.1"));
    assert!(md.contains("Outdated TLS Configuration"));
}

#[test]
fn test_report_with_empty_data() {
    let data = ReportData::from_results("Empty Report", &[], &[]);

    let html = generate_report(&data, ReportFormat::Html).unwrap();
    assert!(html.contains("Empty Report"));

    let md = generate_report(&data, ReportFormat::Markdown).unwrap();
    assert!(md.contains("Empty Report"));
}

#[test]
fn test_report_template_sections() {
    let full = ReportTemplate::full();
    assert!(full.config.sections.contains(&ReportSection::Summary));
    assert!(full.config.sections.contains(&ReportSection::Findings));
    assert!(full.config.sections.contains(&ReportSection::HostDetails));
    assert!(full.config.sections.contains(&ReportSection::Statistics));

    let summary_only = ReportTemplate::summary_only();
    assert!(summary_only
        .config
        .sections
        .contains(&ReportSection::Summary));
    assert!(!summary_only
        .config
        .sections
        .contains(&ReportSection::HostDetails));
}

#[test]
fn test_risk_score_and_rating() {
    let hosts = make_hosts();
    let findings = make_findings();
    let summary = ExecutiveSummary::from_hosts_and_findings(&hosts, &findings);

    let rating = summary.risk_rating();
    assert!(
        ["Low", "Moderate", "Elevated", "High", "Critical"].contains(&rating),
        "Invalid risk rating: {}",
        rating
    );
}

#[test]
fn test_top_affected_hosts() {
    let hosts = make_hosts();
    let findings = make_findings();
    let summary = ExecutiveSummary::from_hosts_and_findings(&hosts, &findings);

    assert!(!summary.top_affected_hosts.is_empty());
    assert_eq!(summary.top_affected_hosts[0].0, "10.0.0.1");
    assert_eq!(summary.top_affected_hosts[0].1, 3);
}
