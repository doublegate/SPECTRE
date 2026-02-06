use serde::Serialize;
use tauri::command;

/// Finding summary for the results dashboard.
#[derive(Debug, Serialize)]
pub struct FindingSummary {
    pub host: String,
    pub port: u16,
    pub service: String,
    pub severity: String,
    pub description: String,
}

/// Dashboard statistics.
#[derive(Debug, Serialize)]
pub struct DashboardStats {
    pub total_hosts: usize,
    pub total_ports: usize,
    pub open_ports: usize,
    pub services_found: usize,
    pub severity_counts: SeverityCounts,
}

/// Severity breakdown.
#[derive(Debug, Serialize)]
pub struct SeverityCounts {
    pub critical: usize,
    pub high: usize,
    pub medium: usize,
    pub low: usize,
    pub info: usize,
}

/// Get dashboard statistics. Stub - will be wired in Sprint 5.5.
#[command]
pub async fn get_dashboard_stats() -> Result<DashboardStats, String> {
    Ok(DashboardStats {
        total_hosts: 0,
        total_ports: 0,
        open_ports: 0,
        services_found: 0,
        severity_counts: SeverityCounts {
            critical: 0,
            high: 0,
            medium: 0,
            low: 0,
            info: 0,
        },
    })
}

/// Get all findings. Stub.
#[command]
pub async fn get_findings() -> Result<Vec<FindingSummary>, String> {
    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_dashboard_stats_stub() {
        let result = get_dashboard_stats().await;
        assert!(result.is_ok());
        let stats = result.expect("Test assertion failed");
        assert_eq!(stats.total_hosts, 0);
    }

    #[tokio::test]
    async fn test_get_findings_stub() {
        let result = get_findings().await;
        assert!(result.is_ok());
        assert!(result.expect("Test assertion failed").is_empty());
    }
}
