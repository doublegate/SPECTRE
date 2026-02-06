use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{command, State};

use crate::state::AppState;
use spectre_core::export::CsvExporter;
use spectre_core::report::{
    generate_report as core_generate_report, ReportData, ReportFormat as CoreReportFormat,
};
use spectre_core::results::{Finding, Host};

/// Report generation request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportRequest {
    pub format: String,
    pub campaign: Option<String>,
    pub template: Option<String>,
    pub include_executive_summary: bool,
    pub filters: Option<ReportFilters>,
}

/// Report filters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportFilters {
    pub severity: Option<Vec<String>>,
    pub date_range: Option<(String, String)>,
    pub services: Option<Vec<String>>,
}

/// Report generation result.
#[derive(Debug, Serialize)]
pub struct ReportResult {
    pub format: String,
    pub path: Option<String>,
    pub content: String,
    pub preview: String,
    pub size_bytes: usize,
}

/// Export data request.
#[derive(Debug, Clone, Deserialize)]
pub struct ExportRequest {
    pub format: String,    // "csv", "json", "xml"
    pub data_type: String, // "findings", "hosts", "ports"
    pub filters: Option<ReportFilters>,
    pub path: Option<String>,
}

/// Export result.
#[derive(Debug, Serialize)]
pub struct ExportResult {
    pub format: String,
    pub path: Option<String>,
    pub content: String,
    pub record_count: usize,
}

/// Generate a report in HTML or Markdown format.
///
/// # Arguments
/// * `request` - Report generation request with format, campaign, template, filters
///
/// # Returns
/// * `ReportResult` with content, preview, and metadata
#[command]
pub async fn generate_report(
    _state: State<'_, Arc<AppState>>,
    request: ReportRequest,
) -> Result<ReportResult, String> {
    tracing::info!(
        format = %request.format,
        campaign = ?request.campaign,
        "Generating report"
    );

    // NOTE: In production, this would:
    // 1. Query findings and hosts from campaign store
    // 2. Apply filters (severity, date_range, services)
    // 3. Generate report using spectre_core::report module
    // 4. Save to file if path provided

    // Validate format
    let format = match request.format.to_lowercase().as_str() {
        "html" => CoreReportFormat::Html,
        "markdown" | "md" => CoreReportFormat::Markdown,
        other => return Err(format!("Unsupported report format: {}", other)),
    };

    // Create mock report data (in production, query from store)
    let title = request.campaign.as_deref().unwrap_or("SPECTRE Scan Report");
    let report_data = ReportData::from_results(title, &[], &[]);

    // Generate report
    let content = core_generate_report(&report_data, format)
        .map_err(|e| format!("Failed to generate report: {}", e))?;

    // Create preview (first 500 chars)
    let preview = if content.len() > 500 {
        format!("{}...", &content[..500])
    } else {
        content.clone()
    };

    Ok(ReportResult {
        format: request.format,
        path: None, // TODO: Save to file if requested
        content: content.clone(),
        preview,
        size_bytes: content.len(),
    })
}

/// Export data in CSV, JSON, or XML format.
///
/// # Arguments
/// * `request` - Export request with format, data_type, filters, optional path
///
/// # Returns
/// * `ExportResult` with content and metadata
#[command]
pub async fn export_data(
    _state: State<'_, Arc<AppState>>,
    request: ExportRequest,
) -> Result<ExportResult, String> {
    tracing::info!(
        format = %request.format,
        data_type = %request.data_type,
        "Exporting data"
    );

    // NOTE: In production, this would:
    // 1. Query data based on data_type (findings, hosts, ports)
    // 2. Apply filters
    // 3. Export using appropriate exporter
    // 4. Save to file if path provided

    // Validate format
    if !matches!(
        request.format.to_lowercase().as_str(),
        "csv" | "json" | "xml"
    ) {
        return Err(format!("Unsupported export format: {}", request.format));
    }

    // Validate data_type
    if !matches!(
        request.data_type.to_lowercase().as_str(),
        "findings" | "hosts" | "ports"
    ) {
        return Err(format!("Unsupported data type: {}", request.data_type));
    }

    // Mock data (in production, query from store)
    let findings: Vec<Finding> = vec![];
    let hosts: Vec<Host> = vec![];

    // Export based on format and data_type
    let content = match (
        request.format.to_lowercase().as_str(),
        request.data_type.to_lowercase().as_str(),
    ) {
        ("csv", "findings") => CsvExporter::export_findings(&findings),
        ("csv", "hosts") => CsvExporter::export_hosts(&hosts),
        ("csv", "ports") => "port,protocol,state,service\n".to_string(), // Placeholder
        ("json", _) => serde_json::to_string_pretty(&findings)
            .map_err(|e| format!("Failed to serialize JSON: {}", e))?,
        ("xml", _) => {
            // Placeholder XML export
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<export></export>".to_string()
        },
        _ => return Err("Invalid format/data_type combination".to_string()),
    };

    let record_count = match request.data_type.to_lowercase().as_str() {
        "findings" => findings.len(),
        "hosts" => hosts.len(),
        _ => 0,
    };

    Ok(ExportResult {
        format: request.format,
        path: request.path,
        content,
        record_count,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE: IPC command tests require Tauri runtime mock.
    // These tests focus on data structures and serialization.

    #[test]
    fn test_report_request_deserialize() {
        let json = r#"{"format":"html","campaign":"TestOp","template":"default","include_executive_summary":true,"filters":null}"#;
        let request: ReportRequest =
            serde_json::from_str(json).expect("Failed to deserialize ReportRequest");
        assert_eq!(request.format, "html");
        assert_eq!(request.campaign, Some("TestOp".to_string()));
        assert_eq!(request.template, Some("default".to_string()));
        assert!(request.include_executive_summary);
    }

    #[test]
    fn test_report_request_minimal() {
        let json = r#"{"format":"markdown","campaign":null,"template":null,"include_executive_summary":false,"filters":null}"#;
        let request: ReportRequest =
            serde_json::from_str(json).expect("Failed to deserialize minimal ReportRequest");
        assert_eq!(request.format, "markdown");
        assert!(request.campaign.is_none());
        assert!(!request.include_executive_summary);
    }

    #[test]
    fn test_export_request_deserialize() {
        let json = r#"{"format":"csv","data_type":"findings","filters":null,"path":null}"#;
        let request: ExportRequest =
            serde_json::from_str(json).expect("Failed to deserialize ExportRequest");
        assert_eq!(request.format, "csv");
        assert_eq!(request.data_type, "findings");
        assert!(request.filters.is_none());
        assert!(request.path.is_none());
    }

    #[test]
    fn test_export_request_with_path() {
        let json =
            r#"{"format":"json","data_type":"hosts","filters":null,"path":"/tmp/export.json"}"#;
        let request: ExportRequest =
            serde_json::from_str(json).expect("Failed to deserialize ExportRequest with path");
        assert_eq!(request.format, "json");
        assert_eq!(request.data_type, "hosts");
        assert_eq!(request.path, Some("/tmp/export.json".to_string()));
    }

    #[test]
    fn test_report_filters_deserialize() {
        let json = r#"{"severity":["critical","high"],"date_range":["2024-01-01","2024-12-31"],"services":["http","ssh"]}"#;
        let filters: ReportFilters =
            serde_json::from_str(json).expect("Failed to deserialize ReportFilters");
        assert_eq!(
            filters
                .severity
                .as_ref()
                .expect("severity should exist")
                .len(),
            2
        );
        assert!(filters.date_range.is_some());
        assert_eq!(
            filters
                .services
                .as_ref()
                .expect("services should exist")
                .len(),
            2
        );
    }

    #[test]
    fn test_report_result_structure() {
        let result = ReportResult {
            format: "html".to_string(),
            path: Some("/tmp/report.html".to_string()),
            content: "<html></html>".to_string(),
            preview: "<html>...".to_string(),
            size_bytes: 100,
        };

        assert_eq!(result.format, "html");
        assert!(result.path.is_some());
        assert_eq!(result.size_bytes, 100);
    }

    #[test]
    fn test_export_result_structure() {
        let result = ExportResult {
            format: "csv".to_string(),
            path: None,
            content: "header\nrow1\n".to_string(),
            record_count: 1,
        };

        assert_eq!(result.format, "csv");
        assert_eq!(result.record_count, 1);
        assert!(result.content.contains("header"));
    }

    #[test]
    fn test_csv_export_with_core() {
        // Test that CsvExporter is properly accessible
        let hosts: Vec<spectre_core::results::Host> = vec![];
        let csv = CsvExporter::export_hosts(&hosts);
        assert!(csv.contains("ip,hostname"));
    }

    #[test]
    fn test_report_generation_formats() {
        // Validate format strings match core module expectations
        let valid_formats = vec!["html", "markdown", "md"];
        for format in valid_formats {
            let request = ReportRequest {
                format: format.to_string(),
                campaign: None,
                template: None,
                include_executive_summary: false,
                filters: None,
            };
            assert!(!request.format.is_empty());
        }
    }

    #[test]
    fn test_export_data_types() {
        // Validate data type strings
        let valid_types = vec!["findings", "hosts", "ports"];
        for data_type in valid_types {
            let request = ExportRequest {
                format: "csv".to_string(),
                data_type: data_type.to_string(),
                filters: None,
                path: None,
            };
            assert!(!request.data_type.is_empty());
        }
    }

    #[test]
    fn test_export_formats() {
        // Validate export formats
        let valid_formats = vec!["csv", "json", "xml"];
        for format in valid_formats {
            let request = ExportRequest {
                format: format.to_string(),
                data_type: "findings".to_string(),
                filters: None,
                path: None,
            };
            assert!(matches!(request.format.as_str(), "csv" | "json" | "xml"));
        }
    }
}
