//! Integration tests -- export format generation
#![allow(clippy::disallowed_methods)]

use spectre_core::export::{
    CsvExporter, ExportState, ExportTemplate, ExportTemplateEngine, IncrementalExporter,
};
use spectre_core::results::{Finding, FindingSeverity, Host, Service};
use spectre_core::scan::{OsInfo, PortResult, PortState, Protocol, ScanResult, Target};
use std::time::Duration;

fn make_scan_results() -> Vec<ScanResult> {
    vec![
        ScanResult {
            host: "10.0.0.1".to_string(),
            ip: Target::Ip("10.0.0.1".parse().unwrap()),
            hostname: Some("web.example.com".to_string()),
            ports: vec![
                PortResult {
                    port: 80,
                    protocol: Protocol::Tcp,
                    state: PortState::Open,
                    service: Some("http".to_string()),
                    version: Some("nginx 1.24".to_string()),
                    banner: None,
                },
                PortResult {
                    port: 443,
                    protocol: Protocol::Tcp,
                    state: PortState::Open,
                    service: Some("https".to_string()),
                    version: None,
                    banner: None,
                },
            ],
            os: Some(OsInfo {
                name: "Linux".to_string(),
                version: Some("5.15".to_string()),
                confidence: 90,
            }),
            scan_time: Duration::from_millis(150),
        },
        ScanResult {
            host: "10.0.0.2".to_string(),
            ip: Target::Ip("10.0.0.2".parse().unwrap()),
            hostname: None,
            ports: vec![PortResult {
                port: 22,
                protocol: Protocol::Tcp,
                state: PortState::Open,
                service: Some("ssh".to_string()),
                version: Some("OpenSSH 9.0".to_string()),
                banner: None,
            }],
            os: None,
            scan_time: Duration::from_millis(100),
        },
    ]
}

fn make_hosts() -> Vec<Host> {
    vec![Host {
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
    }]
}

fn make_findings() -> Vec<Finding> {
    vec![
        Finding::new("Open HTTP", "10.0.0.1", FindingSeverity::Info).with_port(80),
        Finding::new("Weak TLS", "10.0.0.1", FindingSeverity::High).with_port(443),
    ]
}

#[test]
fn test_csv_export_scan_results() {
    let results = make_scan_results();
    let csv = CsvExporter::export_scan_results(&results);

    assert!(csv.starts_with("host,"));
    assert!(csv.contains("10.0.0.1"));
    assert!(csv.contains("10.0.0.2"));
    assert!(csv.contains("80"));
    assert!(csv.contains("22"));
}

#[test]
fn test_csv_export_hosts() {
    let hosts = make_hosts();
    let csv = CsvExporter::export_hosts(&hosts);

    assert!(csv.starts_with("ip,"));
    assert!(csv.contains("10.0.0.1"));
    assert!(csv.contains("web.example.com"));
}

#[test]
fn test_csv_export_findings() {
    let findings = make_findings();
    let csv = CsvExporter::export_findings(&findings);

    assert!(csv.starts_with("severity,title,"));
    assert!(csv.contains("Open HTTP"));
    assert!(csv.contains("Weak TLS"));
}

#[test]
fn test_custom_template_export() {
    let template = ExportTemplate::new(
        "host-port",
        "Host: {{host}} Port: {{port}} Service: {{service}}",
    );

    let items = vec![
        vec![
            ("host".to_string(), "10.0.0.1".to_string()),
            ("port".to_string(), "80".to_string()),
            ("service".to_string(), "http".to_string()),
        ]
        .into_iter()
        .collect::<std::collections::HashMap<_, _>>(),
        vec![
            ("host".to_string(), "10.0.0.2".to_string()),
            ("port".to_string(), "22".to_string()),
            ("service".to_string(), "ssh".to_string()),
        ]
        .into_iter()
        .collect::<std::collections::HashMap<_, _>>(),
    ];

    let output = ExportTemplateEngine::render_items(&template, &items);
    assert!(output.contains("Host: 10.0.0.1 Port: 80 Service: http"));
    assert!(output.contains("Host: 10.0.0.2 Port: 22 Service: ssh"));
}

#[test]
fn test_incremental_export_flow() {
    let results = make_scan_results();
    let mut exporter = IncrementalExporter::new();

    let new = exporter.filter_new(&results);
    assert_eq!(new.len(), 2);

    exporter.record_export(&results[..1]);
    assert_eq!(exporter.exported_count(), 1);

    let new = exporter.filter_new(&results);
    assert_eq!(new.len(), 1);
    assert_eq!(new[0].host, "10.0.0.2");

    exporter.record_export(&results[1..]);
    assert_eq!(exporter.exported_count(), 2);

    let new = exporter.filter_new(&results);
    assert!(new.is_empty());
}

#[test]
fn test_incremental_export_state_persistence() {
    let mut state = ExportState::new();
    state.mark_exported(&["10.0.0.1".to_string(), "10.0.0.2".to_string()]);

    let json = serde_json::to_string(&state).unwrap();
    let loaded: ExportState = serde_json::from_str(&json).unwrap();

    assert!(loaded.is_exported("10.0.0.1"));
    assert!(loaded.is_exported("10.0.0.2"));
    assert!(!loaded.is_exported("10.0.0.3"));
    assert_eq!(loaded.sequence, 1);
}

#[test]
fn test_incremental_reset_and_re_export() {
    let results = make_scan_results();
    let mut exporter = IncrementalExporter::new();

    exporter.record_export(&results);
    assert_eq!(exporter.filter_new(&results).len(), 0);

    exporter.reset();
    assert_eq!(exporter.exported_count(), 0);
    assert_eq!(exporter.filter_new(&results).len(), 2);
}

#[test]
fn test_template_variable_substitution() {
    let template = ExportTemplate::new("simple", "{{name}}: {{value}}");

    let mut vars = std::collections::HashMap::new();
    vars.insert("name".to_string(), "Test".to_string());
    vars.insert("value".to_string(), "42".to_string());

    let output = ExportTemplateEngine::render(&template, &vars);
    assert_eq!(output, "Test: 42");
}
