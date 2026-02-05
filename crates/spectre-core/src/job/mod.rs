//! Scan job orchestration module
//!
//! Provides lifecycle management for scan jobs including:
//! - Job state machine (Created -> Queued -> Running -> Paused -> Complete/Failed/Cancelled)
//! - Concurrent job execution via JobManager
//! - Job prioritization and rate limiting
//! - Pause/resume/cancel support
//! - Event system using tokio broadcast channels

mod events;
mod manager;
mod state;

use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use events::{JobEvent, JobEventReceiver, JobEventSender};
pub use manager::JobManager;
pub use state::JobState;

/// A scan job with lifecycle management
///
/// Jobs progress through a state machine:
/// ```text
/// Created -> Queued -> Running -> Complete
///                   |         |-> Failed
///                   |         |-> Cancelled
///                   |-> Paused -> Running (resume)
///                              |-> Cancelled
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanJob {
    /// Unique job identifier
    pub id: String,
    /// Human-readable job name
    pub name: String,
    /// Current state
    pub state: JobState,
    /// Job priority (higher = processed first)
    pub priority: i32,
    /// Scan configuration for this job
    pub config: ScanJobConfig,
    /// When the job was created
    pub created_at: DateTime<Utc>,
    /// When the job started running
    pub started_at: Option<DateTime<Utc>>,
    /// When the job completed (or failed/cancelled)
    pub completed_at: Option<DateTime<Utc>>,
    /// Error message if job failed
    pub error: Option<String>,
    /// Progress (0.0 - 1.0)
    pub progress: f64,
    /// Number of targets completed
    pub targets_completed: usize,
    /// Total number of targets
    pub targets_total: usize,
}

/// Serializable scan job configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanJobConfig {
    /// Targets to scan
    pub targets: Vec<String>,
    /// Ports to scan
    pub ports: Vec<u16>,
    /// Scan type name
    pub scan_type: String,
    /// Enable service detection
    pub service_detection: bool,
    /// Enable OS detection
    pub os_detection: bool,
    /// Rate limit (packets per second)
    pub rate_limit: Option<u64>,
    /// Timeout per target
    pub timeout: Option<Duration>,
}

impl ScanJob {
    /// Create a new scan job
    pub fn new(name: &str, config: ScanJobConfig) -> Self {
        let targets_total = config.targets.len();
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            state: JobState::Created,
            priority: 0,
            config,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            error: None,
            progress: 0.0,
            targets_completed: 0,
            targets_total,
        }
    }

    /// Set the job priority
    pub const fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Transition to a new state
    pub fn transition(&mut self, new_state: JobState) -> crate::Result<()> {
        if !self.state.can_transition_to(&new_state) {
            return Err(crate::SpectreError::Job(
                crate::error::JobError::InvalidTransition {
                    from: self.state.to_string(),
                    to: new_state.to_string(),
                },
            ));
        }

        match new_state {
            JobState::Running => {
                if self.started_at.is_none() {
                    self.started_at = Some(Utc::now());
                }
            },
            JobState::Complete | JobState::Failed | JobState::Cancelled => {
                self.completed_at = Some(Utc::now());
            },
            _ => {},
        }

        self.state = new_state;
        Ok(())
    }

    /// Update progress
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    pub fn update_progress(&mut self, completed: usize, total: usize) {
        self.targets_completed = completed;
        self.targets_total = total;
        self.progress = if total > 0 {
            completed as f64 / total as f64
        } else {
            0.0
        };
    }

    /// Mark the job as failed with an error message
    pub fn fail(&mut self, error: &str) -> crate::Result<()> {
        self.error = Some(error.to_string());
        self.transition(JobState::Failed)
    }

    /// Get the elapsed time since the job started
    pub fn elapsed(&self) -> Option<Duration> {
        self.started_at.map(|start| {
            let end = self.completed_at.unwrap_or_else(Utc::now);
            let diff = end - start;
            Duration::from_millis(diff.num_milliseconds().max(0) as u64)
        })
    }

    /// Check if the job is in a terminal state
    pub const fn is_terminal(&self) -> bool {
        matches!(
            self.state,
            JobState::Complete | JobState::Failed | JobState::Cancelled
        )
    }

    /// Check if the job is actively running
    pub fn is_running(&self) -> bool {
        self.state == JobState::Running
    }
}

impl Default for ScanJobConfig {
    fn default() -> Self {
        Self {
            targets: Vec::new(),
            ports: Vec::new(),
            scan_type: "Connect".to_string(),
            service_detection: false,
            os_detection: false,
            rate_limit: None,
            timeout: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_job() -> ScanJob {
        ScanJob::new(
            "test-job",
            ScanJobConfig {
                targets: vec!["192.168.1.1".to_string()],
                ports: vec![80, 443],
                ..Default::default()
            },
        )
    }

    #[test]
    fn test_job_creation() {
        let job = make_job();
        assert_eq!(job.name, "test-job");
        assert_eq!(job.state, JobState::Created);
        assert_eq!(job.targets_total, 1);
        assert!(!job.id.is_empty());
    }

    #[test]
    fn test_job_with_priority() {
        let job = make_job().with_priority(5);
        assert_eq!(job.priority, 5);
    }

    #[test]
    fn test_job_state_transitions() {
        let mut job = make_job();

        assert!(job.transition(JobState::Queued).is_ok());
        assert_eq!(job.state, JobState::Queued);

        assert!(job.transition(JobState::Running).is_ok());
        assert_eq!(job.state, JobState::Running);
        assert!(job.started_at.is_some());

        assert!(job.transition(JobState::Complete).is_ok());
        assert_eq!(job.state, JobState::Complete);
        assert!(job.completed_at.is_some());
    }

    #[test]
    fn test_job_invalid_transition() {
        let mut job = make_job();
        // Cannot go from Created directly to Complete
        let result = job.transition(JobState::Complete);
        assert!(result.is_err());
    }

    #[test]
    fn test_job_pause_resume() {
        let mut job = make_job();
        job.transition(JobState::Queued).unwrap();
        job.transition(JobState::Running).unwrap();
        job.transition(JobState::Paused).unwrap();
        assert_eq!(job.state, JobState::Paused);

        job.transition(JobState::Running).unwrap();
        assert_eq!(job.state, JobState::Running);
    }

    #[test]
    fn test_job_progress() {
        let mut job = make_job();
        job.update_progress(5, 10);
        assert!((job.progress - 0.5).abs() < f64::EPSILON);
        assert_eq!(job.targets_completed, 5);
    }

    #[test]
    fn test_job_fail() {
        let mut job = make_job();
        job.transition(JobState::Queued).unwrap();
        job.transition(JobState::Running).unwrap();

        job.fail("network error").unwrap();
        assert_eq!(job.state, JobState::Failed);
        assert_eq!(job.error.as_deref(), Some("network error"));
    }

    #[test]
    fn test_job_is_terminal() {
        let mut job = make_job();
        assert!(!job.is_terminal());

        job.transition(JobState::Queued).unwrap();
        job.transition(JobState::Running).unwrap();
        job.transition(JobState::Complete).unwrap();
        assert!(job.is_terminal());
    }

    #[test]
    fn test_job_is_running() {
        let mut job = make_job();
        assert!(!job.is_running());

        job.transition(JobState::Queued).unwrap();
        job.transition(JobState::Running).unwrap();
        assert!(job.is_running());
    }

    #[test]
    fn test_job_elapsed() {
        let job = make_job();
        assert!(job.elapsed().is_none());
    }
}
