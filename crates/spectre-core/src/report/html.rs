//! HTML report generator — standalone HTML with embedded CSS

use super::ReportData;

/// HTML report generator
pub struct HtmlReportGenerator;

impl HtmlReportGenerator {
    /// Generate a standalone HTML report
    pub fn generate(data: &ReportData) -> crate::Result<String> {
        let mut html = String::with_capacity(8192);

        // HTML header
        html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str(
            "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str(&format!("<title>{}</title>\n", escape_html(&data.title)));
        html.push_str("<style>\n");
        html.push_str(Self::embedded_css());
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");

        // Header
        html.push_str("<div class=\"header\">\n");
        html.push_str(&format!("<h1>{}</h1>\n", escape_html(&data.title)));
        html.push_str(&format!(
            "<p class=\"date\">Generated: {}</p>\n",
            escape_html(&data.date)
        ));
        if let Some(ref campaign) = data.campaign {
            html.push_str(&format!(
                "<p class=\"campaign\">Campaign: {}</p>\n",
                escape_html(campaign)
            ));
        }
        html.push_str("</div>\n");

        // Executive Summary
        html.push_str("<div class=\"section\">\n");
        html.push_str("<h2>Executive Summary</h2>\n");
        html.push_str("<div class=\"summary-grid\">\n");
        html.push_str(&format!(
            "<div class=\"stat\"><span class=\"stat-value\">{}</span><span class=\"stat-label\">Total Hosts</span></div>\n",
            data.summary.total_hosts
        ));
        html.push_str(&format!(
            "<div class=\"stat\"><span class=\"stat-value\">{}</span><span class=\"stat-label\">Hosts Up</span></div>\n",
            data.summary.hosts_up
        ));
        html.push_str(&format!(
            "<div class=\"stat\"><span class=\"stat-value\">{}</span><span class=\"stat-label\">Open Ports</span></div>\n",
            data.summary.total_open_ports
        ));
        html.push_str(&format!(
            "<div class=\"stat\"><span class=\"stat-value\">{}</span><span class=\"stat-label\">Findings</span></div>\n",
            data.summary.total_findings
        ));
        html.push_str(&format!(
            "<div class=\"stat risk-{}\"><span class=\"stat-value\">{}</span><span class=\"stat-label\">Risk: {}</span></div>\n",
            data.summary.risk_rating().to_lowercase(),
            data.summary.risk_score,
            data.summary.risk_rating()
        ));
        html.push_str("</div>\n");
        html.push_str("</div>\n");

        // Severity Breakdown
        if data.summary.total_findings > 0 {
            html.push_str("<div class=\"section\">\n");
            html.push_str("<h2>Severity Breakdown</h2>\n");
            html.push_str("<table>\n<tr><th>Severity</th><th>Count</th></tr>\n");
            html.push_str(&format!(
                "<tr class=\"critical\"><td>Critical</td><td>{}</td></tr>\n",
                data.summary.critical_count
            ));
            html.push_str(&format!(
                "<tr class=\"high\"><td>High</td><td>{}</td></tr>\n",
                data.summary.high_count
            ));
            html.push_str(&format!(
                "<tr class=\"medium\"><td>Medium</td><td>{}</td></tr>\n",
                data.summary.medium_count
            ));
            html.push_str(&format!(
                "<tr class=\"low\"><td>Low</td><td>{}</td></tr>\n",
                data.summary.low_count
            ));
            html.push_str(&format!(
                "<tr class=\"info\"><td>Info</td><td>{}</td></tr>\n",
                data.summary.info_count
            ));
            html.push_str("</table>\n");
            html.push_str("</div>\n");
        }

        // Findings Table
        if !data.findings.is_empty() {
            html.push_str("<div class=\"section\">\n");
            html.push_str("<h2>Findings</h2>\n");
            html.push_str("<table>\n<tr><th>Severity</th><th>Title</th><th>Host</th><th>Port</th><th>Service</th></tr>\n");

            for finding in &data.findings {
                let port_str = finding
                    .port
                    .map(|p| p.to_string())
                    .unwrap_or_else(|| "-".to_string());
                let service_str = finding.service.as_deref().unwrap_or("-");

                let severity_str = finding.severity.to_string();
                html.push_str(&format!(
                    "<tr class=\"{}\"><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
                    severity_str,
                    escape_html(&severity_str),
                    escape_html(&finding.title),
                    escape_html(&finding.host),
                    escape_html(&port_str),
                    escape_html(service_str),
                ));
            }

            html.push_str("</table>\n");
            html.push_str("</div>\n");
        }

        // Host Details
        if !data.hosts.is_empty() {
            html.push_str("<div class=\"section\">\n");
            html.push_str("<h2>Host Details</h2>\n");

            for host in &data.hosts {
                html.push_str(&format!("<h3>{}", escape_html(&host.ip)));
                if let Some(ref hostname) = host.hostname {
                    html.push_str(&format!(" ({})", escape_html(hostname)));
                }
                html.push_str("</h3>\n");

                if let Some(ref os) = host.os {
                    html.push_str(&format!("<p>OS: {}", escape_html(os)));
                    if let Some(ref ver) = host.os_version {
                        html.push_str(&format!(" {}", escape_html(ver)));
                    }
                    html.push_str("</p>\n");
                }

                if !host.services.is_empty() {
                    html.push_str("<table>\n<tr><th>Port</th><th>Protocol</th><th>State</th><th>Service</th><th>Version</th></tr>\n");
                    for svc in &host.services {
                        let name = svc.service_name.as_deref().unwrap_or("-");
                        let ver = svc.version.as_deref().unwrap_or("-");
                        html.push_str(&format!(
                            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
                            svc.port,
                            escape_html(&svc.protocol),
                            escape_html(&svc.state),
                            escape_html(name),
                            escape_html(ver),
                        ));
                    }
                    html.push_str("</table>\n");
                }
            }

            html.push_str("</div>\n");
        }

        // Footer
        html.push_str("<div class=\"footer\">\n");
        html.push_str(&format!(
            "<p>Generated by SPECTRE v{}</p>\n",
            env!("CARGO_PKG_VERSION")
        ));
        html.push_str("</div>\n");
        html.push_str("</body>\n</html>\n");

        Ok(html)
    }

    /// Embedded CSS for standalone reports
    fn embedded_css() -> &'static str {
        r#"
body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 0; padding: 20px; background: #f5f5f5; color: #333; }
.header { background: #1a1a2e; color: #fff; padding: 30px; margin: -20px -20px 20px -20px; }
.header h1 { margin: 0 0 10px 0; }
.header .date, .header .campaign { margin: 5px 0; opacity: 0.8; }
.section { background: #fff; padding: 20px; margin-bottom: 20px; border-radius: 4px; box-shadow: 0 1px 3px rgba(0,0,0,0.1); }
h2 { color: #1a1a2e; border-bottom: 2px solid #e0e0e0; padding-bottom: 10px; }
table { width: 100%; border-collapse: collapse; margin: 10px 0; }
th, td { padding: 8px 12px; text-align: left; border-bottom: 1px solid #e0e0e0; }
th { background: #f8f8f8; font-weight: 600; }
.summary-grid { display: flex; gap: 15px; flex-wrap: wrap; }
.stat { background: #f8f8f8; padding: 15px; border-radius: 4px; text-align: center; min-width: 120px; }
.stat-value { display: block; font-size: 24px; font-weight: bold; color: #1a1a2e; }
.stat-label { display: block; font-size: 12px; color: #666; margin-top: 4px; }
.critical td:first-child, tr.critical td:first-child { color: #d32f2f; font-weight: bold; }
.high td:first-child, tr.high td:first-child { color: #f57c00; font-weight: bold; }
.medium td:first-child, tr.medium td:first-child { color: #fbc02d; font-weight: bold; }
.low td:first-child, tr.low td:first-child { color: #388e3c; }
.info td:first-child, tr.info td:first-child { color: #1976d2; }
.risk-critical { border-left: 4px solid #d32f2f; }
.risk-high { border-left: 4px solid #f57c00; }
.risk-elevated { border-left: 4px solid #fbc02d; }
.risk-moderate { border-left: 4px solid #388e3c; }
.risk-low { border-left: 4px solid #1976d2; }
.footer { text-align: center; padding: 20px; color: #999; font-size: 12px; }
"#
    }
}

/// Escape HTML special characters
fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
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
                .with_service("http"),
        ];

        let summary = crate::report::ExecutiveSummary::from_hosts_and_findings(&hosts, &findings);

        ReportData {
            title: "Test Report".to_string(),
            date: "2025-06-15".to_string(),
            campaign: Some("Test Campaign".to_string()),
            hosts,
            findings,
            summary,
        }
    }

    #[test]
    fn test_html_generation() {
        let data = make_report_data();
        let html = HtmlReportGenerator::generate(&data).unwrap();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Test Report"));
        assert!(html.contains("10.0.0.1"));
        assert!(html.contains("web.example.com"));
    }

    #[test]
    fn test_html_contains_findings() {
        let data = make_report_data();
        let html = HtmlReportGenerator::generate(&data).unwrap();
        assert!(html.contains("Open HTTP Port"));
    }

    #[test]
    fn test_html_contains_summary() {
        let data = make_report_data();
        let html = HtmlReportGenerator::generate(&data).unwrap();
        assert!(html.contains("Executive Summary"));
    }

    #[test]
    fn test_html_contains_css() {
        let data = make_report_data();
        let html = HtmlReportGenerator::generate(&data).unwrap();
        assert!(html.contains("<style>"));
    }

    #[test]
    fn test_html_empty_data() {
        let data = ReportData::from_results("Empty Report", &[], &[]);
        let html = HtmlReportGenerator::generate(&data).unwrap();
        assert!(html.contains("Empty Report"));
    }

    #[test]
    fn test_escape_html() {
        assert_eq!(escape_html("<script>"), "&lt;script&gt;");
        assert_eq!(escape_html("a & b"), "a &amp; b");
        assert_eq!(escape_html("\"hello\""), "&quot;hello&quot;");
    }
}
