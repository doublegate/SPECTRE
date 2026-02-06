use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{command, State};

use crate::state::AppState;
use spectre_core::results::Finding;

/// Finding summary for the results dashboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindingSummary {
    pub id: String,
    pub host: String,
    pub port: Option<u16>,
    pub service: Option<String>,
    pub protocol: Option<String>,
    pub state: String,
    pub severity: String,
    pub title: String,
    pub description: String,
    pub remediation: Option<String>,
    pub references: Vec<String>,
    pub tags: Vec<String>,
    pub timestamp: String,
}

/// Dashboard statistics.
#[derive(Debug, Serialize)]
pub struct DashboardStats {
    pub total_hosts: usize,
    pub total_ports: usize,
    pub open_ports: usize,
    pub services_found: usize,
    pub severity_counts: SeverityCounts,
    pub top_services: Vec<ServiceCount>,
    pub recent_activity: Vec<ScanActivity>,
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

/// Service count for charts.
#[derive(Debug, Serialize)]
pub struct ServiceCount {
    pub name: String,
    pub count: usize,
}

/// Scan activity for timeline.
#[derive(Debug, Serialize)]
pub struct ScanActivity {
    pub scan_id: String,
    pub status: String,
    pub target_count: usize,
    pub duration_ms: Option<u64>,
    pub timestamp: String,
}

/// Filters for findings query.
#[derive(Debug, Clone, Deserialize)]
pub struct FindingFilters {
    pub severity: Option<Vec<String>>,
    pub service: Option<Vec<String>>,
    pub host: Option<String>,
    pub port_range: Option<(u16, u16)>,
}

/// Sort options for findings.
#[derive(Debug, Clone, Deserialize)]
pub struct SortOptions {
    pub field: String,
    pub direction: String, // "asc" or "desc"
}

/// Pagination options.
#[derive(Debug, Clone, Deserialize)]
pub struct Pagination {
    pub page: usize,
    pub per_page: usize,
}

/// Findings response with pagination.
#[derive(Debug, Serialize)]
pub struct FindingsResponse {
    pub findings: Vec<FindingSummary>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
    pub total_pages: usize,
}

impl From<Finding> for FindingSummary {
    fn from(finding: Finding) -> Self {
        Self {
            id: format!(
                "{}-{}-{}",
                finding.host,
                finding.port.unwrap_or(0),
                finding.title.replace(' ', "-")
            ),
            host: finding.host,
            port: finding.port,
            service: finding.service,
            protocol: Some("tcp".to_string()), // Default, could be enhanced
            state: "open".to_string(),         // Default, could be enhanced
            severity: finding.severity.to_string(),
            title: finding.title,
            description: finding.description,
            remediation: finding.remediation,
            references: finding.references,
            tags: finding.tags,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// Get dashboard statistics from stored scan results.
///
/// This aggregates data from completed scans to provide overview metrics.
/// In a production system, this would query a persistent store (SQLite/PostgreSQL).
/// For Sprint 5.5, we're demonstrating the structure with mock data.
#[command]
pub async fn get_dashboard_stats(
    _state: State<'_, Arc<AppState>>,
) -> Result<DashboardStats, String> {
    // NOTE: In production, this would query:
    // 1. Campaign store for scan history
    // 2. Results cache for recent findings
    // 3. Statistics aggregation tables

    // For now, return mock data structure to wire frontend
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
        top_services: vec![],
        recent_activity: vec![],
    })
}

/// Get findings with filtering, sorting, and pagination.
///
/// # Arguments
/// * `filters` - Optional severity, service, host, port_range filters
/// * `sort` - Optional sort field and direction
/// * `pagination` - Optional page and per_page (defaults to page 1, 25 items)
#[command]
pub async fn get_findings(
    _state: State<'_, Arc<AppState>>,
    filters: Option<FindingFilters>,
    sort: Option<SortOptions>,
    pagination: Option<Pagination>,
) -> Result<FindingsResponse, String> {
    // NOTE: In production, this would:
    // 1. Query findings from persistent store
    // 2. Apply filters (severity, service, host, port_range)
    // 3. Apply sorting (by severity, host, port, timestamp)
    // 4. Apply pagination

    let pagination = pagination.unwrap_or(Pagination {
        page: 1,
        per_page: 25,
    });

    // Mock response structure
    let findings = vec![];
    let total = findings.len();
    let total_pages = total.div_ceil(pagination.per_page);

    tracing::debug!(
        filters = ?filters,
        sort = ?sort,
        page = pagination.page,
        per_page = pagination.per_page,
        "Fetching findings"
    );

    Ok(FindingsResponse {
        findings,
        total,
        page: pagination.page,
        per_page: pagination.per_page,
        total_pages,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectre_core::results::FindingSeverity;

    // NOTE: IPC command tests require Tauri runtime mock, which is complex.
    // These tests focus on data structures and conversion logic.
    // Integration tests with full Tauri context would go in tests/ directory.

    #[test]
    fn test_finding_summary_from_finding() {
        let finding = Finding::new("Test Finding", "10.0.0.1", FindingSeverity::High)
            .with_port(80)
            .with_service("http")
            .with_description("Test description")
            .with_remediation("Test remediation");

        let summary = FindingSummary::from(finding);
        assert_eq!(summary.host, "10.0.0.1");
        assert_eq!(summary.port, Some(80));
        assert_eq!(summary.service, Some("http".to_string()));
        assert_eq!(summary.severity, "high");
        assert_eq!(summary.title, "Test Finding");
        assert_eq!(summary.description, "Test description");
        assert!(summary.remediation.is_some());
    }

    #[test]
    fn test_finding_summary_id_generation() {
        let finding = Finding::new("Open Port", "192.168.1.1", FindingSeverity::Info)
            .with_port(22)
            .with_service("ssh");

        let summary = FindingSummary::from(finding);
        // ID format: host-port-title-slug
        assert!(summary.id.contains("192.168.1.1"));
        assert!(summary.id.contains("22"));
    }

    #[test]
    fn test_finding_filters_deserialize() {
        let json = r#"{"severity":["critical","high"],"service":["http","ssh"],"host":"10.0.0.1","port_range":[80,443]}"#;
        let filters: FindingFilters =
            serde_json::from_str(json).expect("Failed to deserialize FindingFilters");
        assert_eq!(
            filters
                .severity
                .as_ref()
                .expect("severity should exist")
                .len(),
            2
        );
        assert_eq!(
            filters
                .service
                .as_ref()
                .expect("service should exist")
                .len(),
            2
        );
        assert_eq!(
            filters.host.as_ref().expect("host should exist"),
            "10.0.0.1"
        );
        assert_eq!(filters.port_range, Some((80, 443)));
    }

    #[test]
    fn test_finding_filters_partial() {
        let json = r#"{"severity":null,"service":null,"host":"10.0.0.1","port_range":null}"#;
        let filters: FindingFilters =
            serde_json::from_str(json).expect("Failed to deserialize partial FindingFilters");
        assert!(filters.severity.is_none());
        assert!(filters.service.is_none());
        assert_eq!(
            filters.host.as_ref().expect("host should exist"),
            "10.0.0.1"
        );
        assert!(filters.port_range.is_none());
    }

    #[test]
    fn test_sort_options_deserialize() {
        let json = r#"{"field":"severity","direction":"desc"}"#;
        let sort: SortOptions =
            serde_json::from_str(json).expect("Failed to deserialize SortOptions");
        assert_eq!(sort.field, "severity");
        assert_eq!(sort.direction, "desc");
    }

    #[test]
    fn test_sort_options_asc() {
        let json = r#"{"field":"host","direction":"asc"}"#;
        let sort: SortOptions =
            serde_json::from_str(json).expect("Failed to deserialize SortOptions (asc)");
        assert_eq!(sort.field, "host");
        assert_eq!(sort.direction, "asc");
    }

    #[test]
    fn test_pagination_deserialize() {
        let json = r#"{"page":2,"per_page":50}"#;
        let pagination: Pagination =
            serde_json::from_str(json).expect("Failed to deserialize Pagination");
        assert_eq!(pagination.page, 2);
        assert_eq!(pagination.per_page, 50);
    }

    #[test]
    fn test_pagination_first_page() {
        let json = r#"{"page":1,"per_page":25}"#;
        let pagination: Pagination =
            serde_json::from_str(json).expect("Failed to deserialize Pagination (first page)");
        assert_eq!(pagination.page, 1);
        assert_eq!(pagination.per_page, 25);
    }

    #[test]
    fn test_dashboard_stats_structure() {
        let stats = DashboardStats {
            total_hosts: 100,
            total_ports: 500,
            open_ports: 42,
            services_found: 15,
            severity_counts: SeverityCounts {
                critical: 2,
                high: 5,
                medium: 10,
                low: 15,
                info: 10,
            },
            top_services: vec![
                ServiceCount {
                    name: "http".to_string(),
                    count: 20,
                },
                ServiceCount {
                    name: "ssh".to_string(),
                    count: 15,
                },
            ],
            recent_activity: vec![],
        };

        assert_eq!(stats.total_hosts, 100);
        assert_eq!(stats.severity_counts.critical, 2);
        assert_eq!(stats.top_services.len(), 2);
    }

    #[test]
    fn test_findings_response_pagination_calc() {
        let response = FindingsResponse {
            findings: vec![],
            total: 100,
            page: 2,
            per_page: 25,
            total_pages: 4,
        };

        assert_eq!(response.total_pages, 4);
        assert_eq!(response.page, 2);
    }
}
