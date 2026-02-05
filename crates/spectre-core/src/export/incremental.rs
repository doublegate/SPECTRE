//! Incremental export — export only new/changed results since last export

use std::collections::HashSet;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::scan::ScanResult;

/// State tracking for incremental exports
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExportState {
    /// Last export timestamp
    pub last_export: Option<DateTime<Utc>>,
    /// Set of previously exported host identifiers
    pub exported_hosts: HashSet<String>,
    /// Sequence number for ordering
    pub sequence: u64,
}

impl ExportState {
    /// Create a new export state
    pub fn new() -> Self {
        Self::default()
    }

    /// Mark a set of hosts as exported
    pub fn mark_exported(&mut self, hosts: &[String]) {
        for host in hosts {
            self.exported_hosts.insert(host.clone());
        }
        self.last_export = Some(Utc::now());
        self.sequence += 1;
    }

    /// Check if a host has been exported
    pub fn is_exported(&self, host: &str) -> bool {
        self.exported_hosts.contains(host)
    }

    /// Reset the export state
    pub fn reset(&mut self) {
        self.last_export = None;
        self.exported_hosts.clear();
        self.sequence = 0;
    }
}

/// Incremental exporter that tracks state
pub struct IncrementalExporter {
    state: ExportState,
}

impl IncrementalExporter {
    /// Create a new incremental exporter
    pub fn new() -> Self {
        Self {
            state: ExportState::new(),
        }
    }

    /// Create from existing state
    pub const fn from_state(state: ExportState) -> Self {
        Self { state }
    }

    /// Filter scan results to only include new/updated hosts
    pub fn filter_new<'a>(&self, results: &'a [ScanResult]) -> Vec<&'a ScanResult> {
        results
            .iter()
            .filter(|r| !self.state.is_exported(&r.host))
            .collect()
    }

    /// Record that results have been exported
    pub fn record_export(&mut self, results: &[ScanResult]) {
        let hosts: Vec<String> = results.iter().map(|r| r.host.clone()).collect();
        self.state.mark_exported(&hosts);
        debug!(
            new_hosts = hosts.len(),
            total_exported = self.state.exported_hosts.len(),
            sequence = self.state.sequence,
            "Incremental export recorded"
        );
    }

    /// Get the current state
    pub const fn state(&self) -> &ExportState {
        &self.state
    }

    /// Get the number of exported hosts
    pub fn exported_count(&self) -> usize {
        self.state.exported_hosts.len()
    }

    /// Reset the exporter state
    pub fn reset(&mut self) {
        self.state.reset();
    }
}

impl Default for IncrementalExporter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scan::{PortResult, PortState, Protocol, ScanResult, Target};
    use std::time::Duration;

    fn make_result(host: &str) -> ScanResult {
        ScanResult {
            host: host.to_string(),
            ip: Target::Ip(host.parse().unwrap()),
            hostname: None,
            ports: vec![PortResult {
                port: 80,
                protocol: Protocol::Tcp,
                state: PortState::Open,
                service: None,
                version: None,
                banner: None,
            }],
            os: None,
            scan_time: Duration::from_millis(100),
        }
    }

    #[test]
    fn test_export_state_new() {
        let state = ExportState::new();
        assert!(state.last_export.is_none());
        assert!(state.exported_hosts.is_empty());
        assert_eq!(state.sequence, 0);
    }

    #[test]
    fn test_mark_exported() {
        let mut state = ExportState::new();
        state.mark_exported(&["10.0.0.1".to_string(), "10.0.0.2".to_string()]);
        assert!(state.is_exported("10.0.0.1"));
        assert!(state.is_exported("10.0.0.2"));
        assert!(!state.is_exported("10.0.0.3"));
        assert_eq!(state.sequence, 1);
    }

    #[test]
    fn test_reset() {
        let mut state = ExportState::new();
        state.mark_exported(&["10.0.0.1".to_string()]);
        state.reset();
        assert!(!state.is_exported("10.0.0.1"));
        assert_eq!(state.sequence, 0);
    }

    #[test]
    fn test_incremental_filter_new() {
        let mut exporter = IncrementalExporter::new();

        let results = vec![
            make_result("10.0.0.1"),
            make_result("10.0.0.2"),
            make_result("10.0.0.3"),
        ];

        // First export: all are new
        let new_results = exporter.filter_new(&results);
        assert_eq!(new_results.len(), 3);

        // Record the export
        exporter.record_export(&results[..2]); // Export first 2

        // Second export: only 10.0.0.3 is new
        let new_results = exporter.filter_new(&results);
        assert_eq!(new_results.len(), 1);
        assert_eq!(new_results[0].host, "10.0.0.3");
    }

    #[test]
    fn test_incremental_exported_count() {
        let mut exporter = IncrementalExporter::new();
        assert_eq!(exporter.exported_count(), 0);

        exporter.record_export(&[make_result("10.0.0.1")]);
        assert_eq!(exporter.exported_count(), 1);
    }

    #[test]
    fn test_incremental_reset() {
        let mut exporter = IncrementalExporter::new();
        exporter.record_export(&[make_result("10.0.0.1")]);
        exporter.reset();
        assert_eq!(exporter.exported_count(), 0);
    }

    #[test]
    fn test_export_state_serialization() {
        let mut state = ExportState::new();
        state.mark_exported(&["10.0.0.1".to_string()]);

        let json = serde_json::to_string(&state).unwrap();
        let deserialized: ExportState = serde_json::from_str(&json).unwrap();
        assert!(deserialized.is_exported("10.0.0.1"));
    }

    #[test]
    fn test_from_state() {
        let mut state = ExportState::new();
        state.mark_exported(&["10.0.0.1".to_string()]);

        let exporter = IncrementalExporter::from_state(state);
        assert_eq!(exporter.exported_count(), 1);
    }
}
