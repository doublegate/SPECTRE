use serde::{Deserialize, Serialize};
use tauri::command;

/// Report generation request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportRequest {
    pub format: String,
    pub campaign: Option<String>,
    pub include_executive_summary: bool,
}

/// Report generation result.
#[derive(Debug, Serialize)]
pub struct ReportResult {
    pub format: String,
    pub path: Option<String>,
    pub content_preview: String,
}

/// Generate a report. Stub - will be wired in Sprint 5.5.
#[command]
pub async fn generate_report(request: ReportRequest) -> Result<ReportResult, String> {
    tracing::info!(format = %request.format, "Generate report (stub)");
    Ok(ReportResult {
        format: request.format,
        path: None,
        content_preview: String::new(),
    })
}

/// Export data in a given format. Stub.
#[command]
pub async fn export_data(format: String) -> Result<String, String> {
    tracing::info!(format = %format, "Export data (stub)");
    Ok(format!("Export in {} format queued", format))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_report_stub() {
        let request = ReportRequest {
            format: "html".to_string(),
            campaign: Some("TestOp".to_string()),
            include_executive_summary: true,
        };
        let result = generate_report(request).await;
        assert!(result.is_ok());
        assert_eq!(result.expect("Test assertion failed").format, "html");
    }

    #[tokio::test]
    async fn test_export_data_stub() {
        let result = export_data("csv".to_string()).await;
        assert!(result.is_ok());
        assert!(result.expect("Test assertion failed").contains("csv"));
    }
}
