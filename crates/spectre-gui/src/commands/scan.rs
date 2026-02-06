use serde::{Deserialize, Serialize};
use tauri::command;

/// Scan configuration received from the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanRequest {
    pub targets: Vec<String>,
    pub ports: Option<String>,
    pub scan_type: Option<String>,
    pub timing: Option<u8>,
}

/// Scan summary returned after completion.
#[derive(Debug, Serialize)]
pub struct ScanSummary {
    pub hosts_scanned: usize,
    pub hosts_up: usize,
    pub open_ports: usize,
    pub duration_ms: u64,
}

/// Start a new scan. Stub - will be wired to spectre-core in Sprint 5.4.
#[command]
pub async fn start_scan(request: ScanRequest) -> Result<String, String> {
    tracing::info!(targets = ?request.targets, "Starting scan (stub)");
    Ok(format!(
        "Scan queued for {} target(s)",
        request.targets.len()
    ))
}

/// Stop a running scan. Stub.
#[command]
pub async fn stop_scan() -> Result<String, String> {
    Ok("Scan stopped".to_string())
}

/// Get scan results. Stub.
#[command]
pub async fn get_scan_results() -> Result<Vec<serde_json::Value>, String> {
    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_start_scan_stub() {
        let request = ScanRequest {
            targets: vec!["10.0.0.1".to_string()],
            ports: Some("22,80,443".to_string()),
            scan_type: Some("connect".to_string()),
            timing: Some(3),
        };
        let result = start_scan(request).await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("1 target"));
    }

    #[tokio::test]
    async fn test_stop_scan_stub() {
        let result = stop_scan().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_scan_results_stub() {
        let result = get_scan_results().await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }
}
