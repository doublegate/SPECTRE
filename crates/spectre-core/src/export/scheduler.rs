//! Export scheduling — automated periodic exports

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::debug;

use super::ExportFormat;

/// An export schedule entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportScheduleEntry {
    /// Entry name
    pub name: String,
    /// Export format
    pub format: ExportFormat,
    /// Output path pattern (supports `{date}` substitution)
    pub output_path: String,
    /// Interval in seconds
    pub interval_seconds: u64,
    /// Whether this entry is enabled
    pub enabled: bool,
    /// Last export time
    pub last_export: Option<DateTime<Utc>>,
}

impl ExportScheduleEntry {
    /// Create a new export schedule entry
    pub fn new(name: &str, format: ExportFormat, output_path: &str, interval_seconds: u64) -> Self {
        Self {
            name: name.to_string(),
            format,
            output_path: output_path.to_string(),
            interval_seconds,
            enabled: true,
            last_export: None,
        }
    }

    /// Check if this entry is due for export
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    pub fn is_due(&self, now: &DateTime<Utc>) -> bool {
        if !self.enabled {
            return false;
        }
        match self.last_export {
            None => true, // Never exported, due immediately
            Some(last) => {
                let elapsed = (*now - last).num_seconds();
                elapsed >= self.interval_seconds as i64
            },
        }
    }

    /// Generate the output path with date substitution
    pub fn resolved_path(&self, now: &DateTime<Utc>) -> String {
        self.output_path
            .replace("{date}", &now.format("%Y%m%d").to_string())
            .replace("{time}", &now.format("%H%M%S").to_string())
            .replace("{datetime}", &now.format("%Y%m%d_%H%M%S").to_string())
    }
}

/// Export scheduler managing periodic exports
#[derive(Debug, Clone, Default)]
pub struct ExportScheduler {
    entries: Vec<ExportScheduleEntry>,
}

impl ExportScheduler {
    /// Create a new export scheduler
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an export schedule entry
    pub fn add(&mut self, entry: ExportScheduleEntry) {
        debug!(name = %entry.name, "Adding export schedule entry");
        self.entries.push(entry);
    }

    /// Get entries that are due for export
    pub fn due_entries(&self, now: &DateTime<Utc>) -> Vec<&ExportScheduleEntry> {
        self.entries.iter().filter(|e| e.is_due(now)).collect()
    }

    /// Mark an entry as exported
    pub fn mark_exported(&mut self, name: &str, at: DateTime<Utc>) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.name == name) {
            entry.last_export = Some(at);
        }
    }

    /// Get all entries
    pub fn entries(&self) -> &[ExportScheduleEntry] {
        &self.entries
    }

    /// Get entry count
    pub const fn count(&self) -> usize {
        self.entries.len()
    }

    /// Remove an entry
    pub fn remove(&mut self, name: &str) -> bool {
        let len_before = self.entries.len();
        self.entries.retain(|e| e.name != name);
        self.entries.len() < len_before
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_schedule_entry_new() {
        let entry =
            ExportScheduleEntry::new("hourly-csv", ExportFormat::Csv, "/exports/{date}.csv", 3600);
        assert_eq!(entry.name, "hourly-csv");
        assert!(entry.enabled);
        assert!(entry.last_export.is_none());
    }

    #[test]
    fn test_is_due_never_exported() {
        let entry = ExportScheduleEntry::new("test", ExportFormat::Csv, "/out.csv", 3600);
        let now = Utc::now();
        assert!(entry.is_due(&now));
    }

    #[test]
    fn test_is_due_recently_exported() {
        let mut entry = ExportScheduleEntry::new("test", ExportFormat::Csv, "/out.csv", 3600);
        entry.last_export = Some(Utc::now());
        let now = Utc::now();
        assert!(!entry.is_due(&now)); // Not enough time passed
    }

    #[test]
    fn test_is_due_enough_time_passed() {
        let mut entry = ExportScheduleEntry::new("test", ExportFormat::Csv, "/out.csv", 60);
        // Set last export to 2 minutes ago
        entry.last_export = Some(Utc::now() - chrono::Duration::seconds(120));
        let now = Utc::now();
        assert!(entry.is_due(&now));
    }

    #[test]
    fn test_is_due_disabled() {
        let mut entry = ExportScheduleEntry::new("test", ExportFormat::Csv, "/out.csv", 0);
        entry.enabled = false;
        assert!(!entry.is_due(&Utc::now()));
    }

    #[test]
    fn test_resolved_path() {
        let entry =
            ExportScheduleEntry::new("test", ExportFormat::Csv, "/exports/{date}_scan.csv", 3600);
        let dt = Utc.with_ymd_and_hms(2025, 6, 15, 14, 30, 0).unwrap();
        let path = entry.resolved_path(&dt);
        assert_eq!(path, "/exports/20250615_scan.csv");
    }

    #[test]
    fn test_scheduler_add_and_count() {
        let mut scheduler = ExportScheduler::new();
        scheduler.add(ExportScheduleEntry::new(
            "test",
            ExportFormat::Csv,
            "/out.csv",
            3600,
        ));
        assert_eq!(scheduler.count(), 1);
    }

    #[test]
    fn test_scheduler_due_entries() {
        let mut scheduler = ExportScheduler::new();
        scheduler.add(ExportScheduleEntry::new(
            "test",
            ExportFormat::Csv,
            "/out.csv",
            3600,
        ));
        let now = Utc::now();
        let due = scheduler.due_entries(&now);
        assert_eq!(due.len(), 1);
    }

    #[test]
    fn test_scheduler_mark_exported() {
        let mut scheduler = ExportScheduler::new();
        scheduler.add(ExportScheduleEntry::new(
            "test",
            ExportFormat::Csv,
            "/out.csv",
            3600,
        ));

        let now = Utc::now();
        scheduler.mark_exported("test", now);

        // Should no longer be due
        let due = scheduler.due_entries(&now);
        assert_eq!(due.len(), 0);
    }

    #[test]
    fn test_scheduler_remove() {
        let mut scheduler = ExportScheduler::new();
        scheduler.add(ExportScheduleEntry::new(
            "test",
            ExportFormat::Csv,
            "/out.csv",
            3600,
        ));
        assert!(scheduler.remove("test"));
        assert_eq!(scheduler.count(), 0);
    }
}
