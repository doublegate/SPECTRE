//! Real-time scan progress tracking.
//!
//! Maintains state about active scans, including progress percentage,
//! packet rate, hosts scanned, and open ports discovered. Integrates
//! with the [`spectre_core::job::JobEvent`] system for live updates.

use std::collections::VecDeque;
use std::time::{Duration, Instant};

use spectre_core::job::JobEvent;
use spectre_core::scan::{PortResult, ScanType};

/// Maximum number of recent results to keep in the display buffer.
const MAX_RECENT_RESULTS: usize = 100;

/// Tracks the state of the currently active scan.
#[derive(Debug, Clone)]
pub struct ScanState {
    /// Whether a scan is currently active
    pub active: bool,
    /// Target description (e.g., "192.168.1.0/24")
    pub target: String,
    /// Type of scan being performed
    pub scan_type: ScanType,
    /// Progress as a fraction (0.0 to 1.0)
    pub progress: f64,
    /// Current packet rate (packets per second)
    pub rate_pps: u64,
    /// Number of hosts scanned so far
    pub hosts_scanned: usize,
    /// Total hosts to scan
    pub hosts_total: usize,
    /// Total open ports discovered
    pub open_ports: usize,
    /// Total services identified
    pub services_found: usize,
    /// When the scan started
    pub started_at: Option<Instant>,
    /// Estimated time remaining
    pub eta: Option<Duration>,
    /// Recent port results for the results table
    pub recent_results: VecDeque<PortResultEntry>,
    /// Currently selected result index (for scrolling)
    pub selected_index: usize,
    /// Scroll offset for the results list
    pub scroll_offset: usize,
    /// Search/filter query
    pub filter: String,
    /// Sort criterion for results
    pub sort_by: ScanSortCriterion,
    /// Job ID of the active scan
    pub job_id: Option<String>,
    /// Error message from the last scan, if any
    pub last_error: Option<String>,
}

/// A single port result entry for the display table.
#[derive(Debug, Clone)]
pub struct PortResultEntry {
    /// Host IP or name
    pub host: String,
    /// Port number
    pub port: u16,
    /// Port state string
    pub state: String,
    /// Protocol (tcp/udp)
    pub protocol: String,
    /// Service name
    pub service: Option<String>,
    /// Service version
    pub version: Option<String>,
}

/// Sort criterion for scan results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanSortCriterion {
    /// Sort by host address
    Host,
    /// Sort by port number
    Port,
    /// Sort by service name
    Service,
    /// Sort by port state
    State,
}

impl ScanState {
    /// Create a new idle scan state.
    pub const fn new() -> Self {
        Self {
            active: false,
            target: String::new(),
            scan_type: ScanType::Connect,
            progress: 0.0,
            rate_pps: 0,
            hosts_scanned: 0,
            hosts_total: 0,
            open_ports: 0,
            services_found: 0,
            started_at: None,
            eta: None,
            recent_results: VecDeque::new(),
            selected_index: 0,
            scroll_offset: 0,
            filter: String::new(),
            sort_by: ScanSortCriterion::Port,
            job_id: None,
            last_error: None,
        }
    }

    /// Start tracking a new scan.
    pub fn start(&mut self, target: &str, scan_type: ScanType, total_hosts: usize, job_id: &str) {
        self.active = true;
        self.target = target.to_string();
        self.scan_type = scan_type;
        self.progress = 0.0;
        self.rate_pps = 0;
        self.hosts_scanned = 0;
        self.hosts_total = total_hosts;
        self.open_ports = 0;
        self.services_found = 0;
        self.started_at = Some(Instant::now());
        self.eta = None;
        self.recent_results.clear();
        self.selected_index = 0;
        self.scroll_offset = 0;
        self.job_id = Some(job_id.to_string());
        self.last_error = None;
    }

    /// Mark the scan as complete.
    pub const fn complete(&mut self) {
        self.active = false;
        self.progress = 1.0;
        self.eta = None;
    }

    /// Mark the scan as failed.
    pub fn fail(&mut self, error: &str) {
        self.active = false;
        self.last_error = Some(error.to_string());
    }

    /// Update progress from a job event.
    pub fn update_progress(&mut self, completed: usize, total: usize) {
        self.hosts_scanned = completed;
        self.hosts_total = total;
        self.progress = if total > 0 {
            completed as f64 / total as f64
        } else {
            0.0
        };
        self.update_eta();
    }

    /// Add port results from a completed host scan.
    pub fn add_host_results(&mut self, host: &str, ports: &[PortResult]) {
        for port in ports {
            let state_str = format!("{:?}", port.state);

            if matches!(
                port.state,
                spectre_core::scan::PortState::Open | spectre_core::scan::PortState::OpenFiltered
            ) {
                self.open_ports += 1;
            }
            if port.service.is_some() {
                self.services_found += 1;
            }

            let entry = PortResultEntry {
                host: host.to_string(),
                port: port.port,
                state: state_str,
                protocol: port.protocol.to_string(),
                service: port.service.clone(),
                version: port.version.clone(),
            };

            self.recent_results.push_back(entry);
            if self.recent_results.len() > MAX_RECENT_RESULTS {
                self.recent_results.pop_front();
            }
        }
    }

    /// Process a job event and update scan state accordingly.
    pub fn handle_event(&mut self, event: &JobEvent) {
        match event {
            JobEvent::Progress {
                completed, total, ..
            } => {
                self.update_progress(*completed, *total);
            },
            JobEvent::Completed { .. } => {
                self.complete();
            },
            JobEvent::Failed { error, .. } => {
                self.fail(error);
            },
            JobEvent::Cancelled { .. } => {
                self.active = false;
                self.last_error = Some("Scan cancelled".to_string());
            },
            _ => {},
        }
    }

    /// Update the estimated time remaining based on current progress.
    fn update_eta(&mut self) {
        if let Some(started) = self.started_at {
            if self.progress > 0.0 && self.progress < 1.0 {
                let elapsed = started.elapsed();
                let total_estimated =
                    Duration::from_secs_f64(elapsed.as_secs_f64() / self.progress);
                let remaining = total_estimated.saturating_sub(elapsed);
                self.eta = Some(remaining);
            } else if self.progress >= 1.0 {
                self.eta = None;
            }
        }
    }

    /// Get the elapsed time since the scan started.
    pub fn elapsed(&self) -> Option<Duration> {
        self.started_at.map(|start| start.elapsed())
    }

    /// Get a progress percentage (0-100).
    pub fn progress_percent(&self) -> u16 {
        (self.progress * 100.0).min(100.0) as u16
    }

    /// Get the formatted ETA string.
    pub fn eta_string(&self) -> String {
        match self.eta {
            Some(remaining) => {
                let secs = remaining.as_secs();
                if secs >= 3600 {
                    format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
                } else if secs >= 60 {
                    format!("{}m {}s", secs / 60, secs % 60)
                } else {
                    format!("{}s", secs)
                }
            },
            None => {
                if self.active {
                    "calculating...".to_string()
                } else {
                    "--".to_string()
                }
            },
        }
    }

    /// Get the formatted elapsed time string.
    pub fn elapsed_string(&self) -> String {
        match self.elapsed() {
            Some(elapsed) => {
                let secs = elapsed.as_secs();
                if secs >= 3600 {
                    format!("{}h {}m {}s", secs / 3600, (secs % 3600) / 60, secs % 60)
                } else if secs >= 60 {
                    format!("{}m {}s", secs / 60, secs % 60)
                } else {
                    format!("{}s", secs)
                }
            },
            None => "--".to_string(),
        }
    }

    /// Get filtered results based on the current filter string.
    pub fn filtered_results(&self) -> Vec<&PortResultEntry> {
        if self.filter.is_empty() {
            return self.recent_results.iter().collect();
        }
        let filter_lower = self.filter.to_lowercase();
        self.recent_results
            .iter()
            .filter(|r| {
                r.host.to_lowercase().contains(&filter_lower)
                    || r.port.to_string().contains(&filter_lower)
                    || r.service
                        .as_deref()
                        .unwrap_or_default()
                        .to_lowercase()
                        .contains(&filter_lower)
                    || r.state.to_lowercase().contains(&filter_lower)
            })
            .collect()
    }

    /// Move selection up in the results list.
    pub const fn select_prev(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// Move selection down in the results list.
    pub fn select_next(&mut self) {
        let max = self.filtered_results().len().saturating_sub(1);
        if self.selected_index < max {
            self.selected_index += 1;
        }
    }

    /// Jump to the top of the results list.
    pub const fn select_first(&mut self) {
        self.selected_index = 0;
        self.scroll_offset = 0;
    }

    /// Jump to the bottom of the results list.
    pub fn select_last(&mut self) {
        let len = self.filtered_results().len();
        self.selected_index = len.saturating_sub(1);
    }

    /// Set the sort criterion.
    pub const fn set_sort(&mut self, criterion: ScanSortCriterion) {
        self.sort_by = criterion;
    }

    /// Clear all scan state and results.
    pub fn clear(&mut self) {
        *self = Self::new();
    }
}

impl Default for ScanState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    use chrono::Utc;
    use spectre_core::scan::{PortState, Protocol};

    fn make_port_result(port: u16, state: PortState, service: Option<&str>) -> PortResult {
        PortResult {
            port,
            protocol: Protocol::Tcp,
            state,
            service: service.map(String::from),
            version: None,
            banner: None,
        }
    }

    #[test]
    fn test_scan_state_new() {
        let state = ScanState::new();
        assert!(!state.active);
        assert!(state.target.is_empty());
        assert_eq!(state.progress, 0.0);
        assert_eq!(state.hosts_scanned, 0);
    }

    #[test]
    fn test_scan_state_start() {
        let mut state = ScanState::new();
        state.start("192.168.1.0/24", ScanType::Syn, 256, "job-1");
        assert!(state.active);
        assert_eq!(state.target, "192.168.1.0/24");
        assert_eq!(state.hosts_total, 256);
        assert!(state.started_at.is_some());
        assert_eq!(state.job_id.as_deref(), Some("job-1"));
    }

    #[test]
    fn test_scan_state_complete() {
        let mut state = ScanState::new();
        state.start("10.0.0.1", ScanType::Connect, 1, "job-1");
        state.complete();
        assert!(!state.active);
        assert_eq!(state.progress, 1.0);
    }

    #[test]
    fn test_scan_state_fail() {
        let mut state = ScanState::new();
        state.start("10.0.0.1", ScanType::Connect, 1, "job-1");
        state.fail("timeout");
        assert!(!state.active);
        assert_eq!(state.last_error.as_deref(), Some("timeout"));
    }

    #[test]
    fn test_scan_state_progress() {
        let mut state = ScanState::new();
        state.start("10.0.0.0/24", ScanType::Connect, 256, "job-1");
        state.update_progress(128, 256);
        assert!((state.progress - 0.5).abs() < f64::EPSILON);
        assert_eq!(state.hosts_scanned, 128);
        assert_eq!(state.progress_percent(), 50);
    }

    #[test]
    fn test_scan_state_add_host_results() {
        let mut state = ScanState::new();
        state.start("10.0.0.1", ScanType::Connect, 1, "job-1");

        let ports = vec![
            make_port_result(22, PortState::Open, Some("ssh")),
            make_port_result(80, PortState::Open, Some("http")),
            make_port_result(443, PortState::Closed, None),
        ];

        state.add_host_results("10.0.0.1", &ports);
        assert_eq!(state.recent_results.len(), 3);
        assert_eq!(state.open_ports, 2);
        assert_eq!(state.services_found, 2);
    }

    #[test]
    fn test_scan_state_max_results() {
        let mut state = ScanState::new();
        state.start("10.0.0.0/8", ScanType::Connect, 1000, "job-1");

        for i in 0..150 {
            let ports = vec![make_port_result(
                (i % 65535) as u16 + 1,
                PortState::Open,
                None,
            )];
            state.add_host_results(&format!("10.0.0.{}", i % 256), &ports);
        }

        assert_eq!(state.recent_results.len(), MAX_RECENT_RESULTS);
    }

    #[test]
    fn test_scan_state_handle_progress_event() {
        let mut state = ScanState::new();
        state.start("10.0.0.0/24", ScanType::Connect, 256, "job-1");

        let event = JobEvent::Progress {
            job_id: "job-1".to_string(),
            progress: 0.5,
            completed: 128,
            total: 256,
            timestamp: Utc::now(),
        };

        state.handle_event(&event);
        assert_eq!(state.hosts_scanned, 128);
    }

    #[test]
    fn test_scan_state_handle_completed_event() {
        let mut state = ScanState::new();
        state.start("10.0.0.1", ScanType::Connect, 1, "job-1");

        let event = JobEvent::Completed {
            job_id: "job-1".to_string(),
            elapsed_ms: 1000,
            timestamp: Utc::now(),
        };

        state.handle_event(&event);
        assert!(!state.active);
        assert_eq!(state.progress, 1.0);
    }

    #[test]
    fn test_scan_state_handle_failed_event() {
        let mut state = ScanState::new();
        state.start("10.0.0.1", ScanType::Connect, 1, "job-1");

        let event = JobEvent::Failed {
            job_id: "job-1".to_string(),
            error: "network error".to_string(),
            timestamp: Utc::now(),
        };

        state.handle_event(&event);
        assert!(!state.active);
        assert_eq!(state.last_error.as_deref(), Some("network error"));
    }

    #[test]
    fn test_scan_state_handle_cancelled_event() {
        let mut state = ScanState::new();
        state.start("10.0.0.1", ScanType::Connect, 1, "job-1");

        let event = JobEvent::Cancelled {
            job_id: "job-1".to_string(),
            timestamp: Utc::now(),
        };

        state.handle_event(&event);
        assert!(!state.active);
    }

    #[test]
    fn test_eta_string_inactive() {
        let state = ScanState::new();
        assert_eq!(state.eta_string(), "--");
    }

    #[test]
    fn test_eta_string_active_calculating() {
        let mut state = ScanState::new();
        state.active = true;
        assert_eq!(state.eta_string(), "calculating...");
    }

    #[test]
    fn test_eta_string_with_value() {
        let mut state = ScanState::new();
        state.eta = Some(Duration::from_secs(125));
        assert_eq!(state.eta_string(), "2m 5s");
    }

    #[test]
    fn test_eta_string_hours() {
        let mut state = ScanState::new();
        state.eta = Some(Duration::from_secs(7200));
        assert_eq!(state.eta_string(), "2h 0m");
    }

    #[test]
    fn test_elapsed_string_none() {
        let state = ScanState::new();
        assert_eq!(state.elapsed_string(), "--");
    }

    #[test]
    fn test_filtered_results_empty_filter() {
        let mut state = ScanState::new();
        state.recent_results.push_back(PortResultEntry {
            host: "10.0.0.1".to_string(),
            port: 22,
            state: "Open".to_string(),
            protocol: "tcp".to_string(),
            service: Some("ssh".to_string()),
            version: None,
        });

        let results = state.filtered_results();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_filtered_results_with_filter() {
        let mut state = ScanState::new();
        state.recent_results.push_back(PortResultEntry {
            host: "10.0.0.1".to_string(),
            port: 22,
            state: "Open".to_string(),
            protocol: "tcp".to_string(),
            service: Some("ssh".to_string()),
            version: None,
        });
        state.recent_results.push_back(PortResultEntry {
            host: "10.0.0.2".to_string(),
            port: 80,
            state: "Open".to_string(),
            protocol: "tcp".to_string(),
            service: Some("http".to_string()),
            version: None,
        });

        state.filter = "ssh".to_string();
        let results = state.filtered_results();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].host, "10.0.0.1");
    }

    #[test]
    fn test_select_navigation() {
        let mut state = ScanState::new();
        for i in 0..5 {
            state.recent_results.push_back(PortResultEntry {
                host: format!("10.0.0.{}", i),
                port: 22,
                state: "Open".to_string(),
                protocol: "tcp".to_string(),
                service: None,
                version: None,
            });
        }

        assert_eq!(state.selected_index, 0);
        state.select_next();
        assert_eq!(state.selected_index, 1);
        state.select_next();
        assert_eq!(state.selected_index, 2);
        state.select_prev();
        assert_eq!(state.selected_index, 1);

        state.select_last();
        assert_eq!(state.selected_index, 4);
        state.select_first();
        assert_eq!(state.selected_index, 0);
    }

    #[test]
    fn test_select_boundary() {
        let mut state = ScanState::new();
        state.select_prev(); // should not underflow
        assert_eq!(state.selected_index, 0);

        state.select_next(); // no results, should stay at 0
        assert_eq!(state.selected_index, 0);
    }

    #[test]
    fn test_clear() {
        let mut state = ScanState::new();
        state.start("10.0.0.1", ScanType::Syn, 1, "job-1");
        state.open_ports = 5;
        state.clear();
        assert!(!state.active);
        assert_eq!(state.open_ports, 0);
        assert!(state.target.is_empty());
    }

    #[test]
    fn test_set_sort() {
        let mut state = ScanState::new();
        assert_eq!(state.sort_by, ScanSortCriterion::Port);
        state.set_sort(ScanSortCriterion::Host);
        assert_eq!(state.sort_by, ScanSortCriterion::Host);
    }

    #[test]
    fn test_progress_zero_total() {
        let mut state = ScanState::new();
        state.update_progress(0, 0);
        assert_eq!(state.progress, 0.0);
    }

    #[test]
    fn test_progress_percent_cap() {
        let mut state = ScanState::new();
        state.progress = 1.5; // over 100%
        assert_eq!(state.progress_percent(), 100);
    }

    #[test]
    fn test_default() {
        let state = ScanState::default();
        assert!(!state.active);
    }

    #[test]
    fn test_handle_created_event() {
        let mut state = ScanState::new();
        let event = JobEvent::Created {
            job_id: "job-1".to_string(),
            name: "test".to_string(),
            timestamp: Utc::now(),
        };
        // Should not panic, just a no-op for scan state
        state.handle_event(&event);
    }

    #[test]
    fn test_handle_state_changed_event() {
        let mut state = ScanState::new();
        let event = JobEvent::StateChanged {
            job_id: "job-1".to_string(),
            from: spectre_core::job::JobState::Queued,
            to: spectre_core::job::JobState::Running,
            timestamp: Utc::now(),
        };
        // Should not panic, just a no-op
        state.handle_event(&event);
    }
}
