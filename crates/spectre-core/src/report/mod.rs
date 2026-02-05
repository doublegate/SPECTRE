//! Report generation module
//!
//! Generates HTML and Markdown reports from scan results
//! with executive summaries and technical detail sections.

mod html;
mod markdown;
mod summary;
mod template;

pub use html::HtmlReportGenerator;
pub use markdown::MarkdownReportGenerator;
pub use summary::ExecutiveSummary;
pub use template::{ReportConfig, ReportSection, ReportTemplate};

use serde::{Deserialize, Serialize};

use crate::results::{Finding, Host};
use crate::scan::ScanResult;

/// Report data container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportData {
    /// Report title
    pub title: String,
    /// Report date
    pub date: String,
    /// Campaign name (if applicable)
    pub campaign: Option<String>,
    /// Hosts found
    pub hosts: Vec<Host>,
    /// Findings
    pub findings: Vec<Finding>,
    /// Executive summary
    pub summary: ExecutiveSummary,
}

impl ReportData {
    /// Create report data from scan results
    pub fn from_results(title: &str, results: &[ScanResult], findings: &[Finding]) -> Self {
        let hosts = crate::results::aggregate_results(results);
        let summary = ExecutiveSummary::from_hosts_and_findings(&hosts, findings);

        Self {
            title: title.to_string(),
            date: chrono::Utc::now()
                .format("%Y-%m-%d %H:%M:%S UTC")
                .to_string(),
            campaign: None,
            hosts,
            findings: findings.to_vec(),
            summary,
        }
    }

    /// Set campaign name
    pub fn with_campaign(mut self, campaign: &str) -> Self {
        self.campaign = Some(campaign.to_string());
        self
    }
}

/// Generate report format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportFormat {
    /// HTML report
    Html,
    /// Markdown report
    Markdown,
}

/// Generate a report in the specified format
pub fn generate_report(data: &ReportData, format: ReportFormat) -> crate::Result<String> {
    match format {
        ReportFormat::Html => HtmlReportGenerator::generate(data),
        ReportFormat::Markdown => MarkdownReportGenerator::generate(data),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_data_from_results() {
        let data = ReportData::from_results("Test Report", &[], &[]);
        assert_eq!(data.title, "Test Report");
        assert!(data.hosts.is_empty());
    }

    #[test]
    fn test_report_data_with_campaign() {
        let data = ReportData::from_results("Test", &[], &[]).with_campaign("Op BLACKOUT");
        assert_eq!(data.campaign, Some("Op BLACKOUT".to_string()));
    }

    #[test]
    fn test_generate_html_report() {
        let data = ReportData::from_results("Test Report", &[], &[]);
        let html = generate_report(&data, ReportFormat::Html).unwrap();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Test Report"));
    }

    #[test]
    fn test_generate_markdown_report() {
        let data = ReportData::from_results("Test Report", &[], &[]);
        let md = generate_report(&data, ReportFormat::Markdown).unwrap();
        assert!(md.contains("# Test Report"));
    }
}
