//! Scan scheduling with cron-like expressions
//!
//! Provides time-based scan execution scheduling.

use chrono::{DateTime, Datelike, Timelike, Utc};
use serde::{Deserialize, Serialize};
use tracing::debug;

/// A cron-like schedule expression
///
/// Format: `minute hour day_of_month month day_of_week`
/// Supports: `*` (any), specific values, and ranges.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleExpression {
    /// Minutes (0-59)
    pub minute: ScheduleField,
    /// Hours (0-23)
    pub hour: ScheduleField,
    /// Day of month (1-31)
    pub day_of_month: ScheduleField,
    /// Month (1-12)
    pub month: ScheduleField,
    /// Day of week (0-6, 0 = Sunday)
    pub day_of_week: ScheduleField,
}

/// A single field in a schedule expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScheduleField {
    /// Any value matches
    Any,
    /// Specific value
    Value(u32),
    /// Range of values (inclusive)
    Range(u32, u32),
    /// List of specific values
    List(Vec<u32>),
    /// Every N-th value (step)
    Step(u32),
}

impl ScheduleField {
    /// Check if a value matches this field
    pub fn matches(&self, value: u32) -> bool {
        match self {
            Self::Any => true,
            Self::Value(v) => *v == value,
            Self::Range(start, end) => value >= *start && value <= *end,
            Self::List(values) => values.contains(&value),
            Self::Step(step) => *step > 0 && value.is_multiple_of(*step),
        }
    }
}

impl ScheduleExpression {
    /// Create a schedule that runs every hour
    pub fn hourly() -> Self {
        Self {
            minute: ScheduleField::Value(0),
            hour: ScheduleField::Any,
            day_of_month: ScheduleField::Any,
            month: ScheduleField::Any,
            day_of_week: ScheduleField::Any,
        }
    }

    /// Create a schedule that runs daily at a specific hour
    pub fn daily(hour: u32) -> Self {
        Self {
            minute: ScheduleField::Value(0),
            hour: ScheduleField::Value(hour),
            day_of_month: ScheduleField::Any,
            month: ScheduleField::Any,
            day_of_week: ScheduleField::Any,
        }
    }

    /// Create a schedule that runs weekly on a specific day and hour
    pub fn weekly(day_of_week: u32, hour: u32) -> Self {
        Self {
            minute: ScheduleField::Value(0),
            hour: ScheduleField::Value(hour),
            day_of_month: ScheduleField::Any,
            month: ScheduleField::Any,
            day_of_week: ScheduleField::Value(day_of_week),
        }
    }

    /// Parse a simple cron expression string
    ///
    /// Format: "minute hour day_of_month month day_of_week"
    /// Example: "0 2 * * 1" (every Monday at 2:00 AM)
    pub fn parse(expr: &str) -> crate::Result<Self> {
        let parts: Vec<&str> = expr.split_whitespace().collect();
        if parts.len() != 5 {
            return Err(crate::SpectreError::Orchestration(
                crate::error::OrchestrationError::InvalidSchedule(format!(
                    "Expected 5 fields, got {}",
                    parts.len()
                )),
            ));
        }

        Ok(Self {
            minute: Self::parse_field(parts[0])?,
            hour: Self::parse_field(parts[1])?,
            day_of_month: Self::parse_field(parts[2])?,
            month: Self::parse_field(parts[3])?,
            day_of_week: Self::parse_field(parts[4])?,
        })
    }

    /// Parse a single schedule field
    fn parse_field(field: &str) -> crate::Result<ScheduleField> {
        if field == "*" {
            return Ok(ScheduleField::Any);
        }

        // Check for step (*/N)
        if let Some(step) = field.strip_prefix("*/") {
            let n: u32 = step.parse().map_err(|_| {
                crate::SpectreError::Orchestration(
                    crate::error::OrchestrationError::InvalidSchedule(format!(
                        "Invalid step value: {}",
                        step
                    )),
                )
            })?;
            return Ok(ScheduleField::Step(n));
        }

        // Check for range (N-M)
        if field.contains('-') {
            let parts: Vec<&str> = field.split('-').collect();
            if parts.len() == 2 {
                let start: u32 = parts[0].parse().map_err(|_| {
                    crate::SpectreError::Orchestration(
                        crate::error::OrchestrationError::InvalidSchedule(format!(
                            "Invalid range start: {}",
                            parts[0]
                        )),
                    )
                })?;
                let end: u32 = parts[1].parse().map_err(|_| {
                    crate::SpectreError::Orchestration(
                        crate::error::OrchestrationError::InvalidSchedule(format!(
                            "Invalid range end: {}",
                            parts[1]
                        )),
                    )
                })?;
                return Ok(ScheduleField::Range(start, end));
            }
        }

        // Check for list (N,M,...)
        if field.contains(',') {
            let values: Result<Vec<u32>, _> = field.split(',').map(|v| v.parse()).collect();
            let values = values.map_err(|_| {
                crate::SpectreError::Orchestration(
                    crate::error::OrchestrationError::InvalidSchedule(format!(
                        "Invalid list value in: {}",
                        field
                    )),
                )
            })?;
            return Ok(ScheduleField::List(values));
        }

        // Single value
        let value: u32 = field.parse().map_err(|_| {
            crate::SpectreError::Orchestration(crate::error::OrchestrationError::InvalidSchedule(
                format!("Invalid value: {}", field),
            ))
        })?;
        Ok(ScheduleField::Value(value))
    }

    /// Check if a given datetime matches this schedule
    pub fn matches(&self, dt: &DateTime<Utc>) -> bool {
        self.minute.matches(dt.minute())
            && self.hour.matches(dt.hour())
            && self.day_of_month.matches(dt.day())
            && self.month.matches(dt.month())
            && self
                .day_of_week
                .matches(dt.weekday().num_days_from_sunday())
    }
}

/// A scheduled scan entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleEntry {
    /// Entry name
    pub name: String,
    /// Schedule expression
    pub schedule: ScheduleExpression,
    /// Scan template name to use
    pub template_name: String,
    /// Targets to scan
    pub targets: Vec<String>,
    /// Whether this entry is enabled
    pub enabled: bool,
    /// Last execution time
    pub last_run: Option<DateTime<Utc>>,
    /// Next scheduled execution time
    pub next_run: Option<DateTime<Utc>>,
}

impl ScheduleEntry {
    /// Create a new schedule entry
    pub fn new(name: &str, schedule: ScheduleExpression, template_name: &str) -> Self {
        Self {
            name: name.to_string(),
            schedule,
            template_name: template_name.to_string(),
            targets: Vec::new(),
            enabled: true,
            last_run: None,
            next_run: None,
        }
    }

    /// Set targets
    pub fn with_targets(mut self, targets: Vec<String>) -> Self {
        self.targets = targets;
        self
    }

    /// Check if this entry should run at the given time
    pub fn should_run(&self, now: &DateTime<Utc>) -> bool {
        if !self.enabled {
            return false;
        }
        self.schedule.matches(now)
    }
}

/// A scan scheduler that manages scheduled scan entries
#[derive(Debug, Clone)]
pub struct ScanScheduler {
    entries: Vec<ScheduleEntry>,
}

impl ScanScheduler {
    /// Create a new scan scheduler
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Add a schedule entry
    pub fn add(&mut self, entry: ScheduleEntry) {
        debug!(name = %entry.name, "Adding schedule entry");
        self.entries.push(entry);
    }

    /// Remove a schedule entry by name
    pub fn remove(&mut self, name: &str) -> bool {
        let len_before = self.entries.len();
        self.entries.retain(|e| e.name != name);
        self.entries.len() < len_before
    }

    /// Get all entries that should run at the given time
    pub fn due_entries(&self, now: &DateTime<Utc>) -> Vec<&ScheduleEntry> {
        self.entries.iter().filter(|e| e.should_run(now)).collect()
    }

    /// List all entries
    pub fn list(&self) -> &[ScheduleEntry] {
        &self.entries
    }

    /// Get entry count
    pub fn count(&self) -> usize {
        self.entries.len()
    }

    /// Enable or disable an entry
    pub fn set_enabled(&mut self, name: &str, enabled: bool) -> bool {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.name == name) {
            entry.enabled = enabled;
            true
        } else {
            false
        }
    }
}

impl Default for ScanScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_schedule_field_any() {
        assert!(ScheduleField::Any.matches(0));
        assert!(ScheduleField::Any.matches(59));
    }

    #[test]
    fn test_schedule_field_value() {
        assert!(ScheduleField::Value(5).matches(5));
        assert!(!ScheduleField::Value(5).matches(6));
    }

    #[test]
    fn test_schedule_field_range() {
        assert!(ScheduleField::Range(1, 5).matches(3));
        assert!(ScheduleField::Range(1, 5).matches(1));
        assert!(ScheduleField::Range(1, 5).matches(5));
        assert!(!ScheduleField::Range(1, 5).matches(6));
    }

    #[test]
    fn test_schedule_field_list() {
        assert!(ScheduleField::List(vec![1, 3, 5]).matches(3));
        assert!(!ScheduleField::List(vec![1, 3, 5]).matches(2));
    }

    #[test]
    fn test_schedule_field_step() {
        assert!(ScheduleField::Step(5).matches(0));
        assert!(ScheduleField::Step(5).matches(10));
        assert!(!ScheduleField::Step(5).matches(3));
    }

    #[test]
    fn test_schedule_expression_hourly() {
        let sched = ScheduleExpression::hourly();
        let dt = Utc.with_ymd_and_hms(2025, 6, 15, 14, 0, 0).unwrap();
        assert!(sched.matches(&dt));
        let dt2 = Utc.with_ymd_and_hms(2025, 6, 15, 14, 30, 0).unwrap();
        assert!(!sched.matches(&dt2));
    }

    #[test]
    fn test_schedule_expression_daily() {
        let sched = ScheduleExpression::daily(2);
        let dt = Utc.with_ymd_and_hms(2025, 6, 15, 2, 0, 0).unwrap();
        assert!(sched.matches(&dt));
        let dt2 = Utc.with_ymd_and_hms(2025, 6, 15, 3, 0, 0).unwrap();
        assert!(!sched.matches(&dt2));
    }

    #[test]
    fn test_schedule_expression_parse() {
        let sched = ScheduleExpression::parse("0 2 * * 1").unwrap();
        // Monday at 2:00 AM
        let dt = Utc.with_ymd_and_hms(2025, 6, 16, 2, 0, 0).unwrap(); // June 16, 2025 is a Monday
        assert!(sched.matches(&dt));
    }

    #[test]
    fn test_schedule_expression_parse_step() {
        let sched = ScheduleExpression::parse("*/15 * * * *").unwrap();
        let dt = Utc.with_ymd_and_hms(2025, 6, 15, 14, 0, 0).unwrap();
        assert!(sched.matches(&dt));
        let dt2 = Utc.with_ymd_and_hms(2025, 6, 15, 14, 15, 0).unwrap();
        assert!(sched.matches(&dt2));
    }

    #[test]
    fn test_schedule_expression_parse_invalid() {
        assert!(ScheduleExpression::parse("0 2 *").is_err());
        assert!(ScheduleExpression::parse("abc def ghi jkl mno").is_err());
    }

    #[test]
    fn test_schedule_entry() {
        let entry = ScheduleEntry::new("nightly", ScheduleExpression::daily(2), "full-audit")
            .with_targets(vec!["10.0.0.0/24".to_string()]);

        assert_eq!(entry.name, "nightly");
        assert_eq!(entry.template_name, "full-audit");
        assert!(entry.enabled);
    }

    #[test]
    fn test_schedule_entry_should_run() {
        let entry = ScheduleEntry::new("test", ScheduleExpression::hourly(), "quick-recon");
        let dt = Utc.with_ymd_and_hms(2025, 6, 15, 14, 0, 0).unwrap();
        assert!(entry.should_run(&dt));
    }

    #[test]
    fn test_schedule_entry_disabled() {
        let mut entry = ScheduleEntry::new("test", ScheduleExpression::hourly(), "quick-recon");
        entry.enabled = false;
        let dt = Utc.with_ymd_and_hms(2025, 6, 15, 14, 0, 0).unwrap();
        assert!(!entry.should_run(&dt));
    }

    #[test]
    fn test_scan_scheduler() {
        let mut scheduler = ScanScheduler::new();
        scheduler.add(ScheduleEntry::new(
            "hourly-recon",
            ScheduleExpression::hourly(),
            "quick-recon",
        ));
        scheduler.add(ScheduleEntry::new(
            "nightly-audit",
            ScheduleExpression::daily(2),
            "full-audit",
        ));

        assert_eq!(scheduler.count(), 2);
    }

    #[test]
    fn test_scan_scheduler_remove() {
        let mut scheduler = ScanScheduler::new();
        scheduler.add(ScheduleEntry::new(
            "test",
            ScheduleExpression::hourly(),
            "quick-recon",
        ));
        assert!(scheduler.remove("test"));
        assert_eq!(scheduler.count(), 0);
        assert!(!scheduler.remove("nonexistent"));
    }

    #[test]
    fn test_scan_scheduler_due_entries() {
        let mut scheduler = ScanScheduler::new();
        scheduler.add(ScheduleEntry::new(
            "hourly",
            ScheduleExpression::hourly(),
            "quick-recon",
        ));

        let dt = Utc.with_ymd_and_hms(2025, 6, 15, 14, 0, 0).unwrap();
        let due = scheduler.due_entries(&dt);
        assert_eq!(due.len(), 1);
    }

    #[test]
    fn test_scan_scheduler_set_enabled() {
        let mut scheduler = ScanScheduler::new();
        scheduler.add(ScheduleEntry::new(
            "test",
            ScheduleExpression::hourly(),
            "quick-recon",
        ));
        assert!(scheduler.set_enabled("test", false));
        assert!(!scheduler.set_enabled("nonexistent", false));
    }
}
