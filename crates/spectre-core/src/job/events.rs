//! Job event system using tokio broadcast channels
//!
//! Provides real-time notifications of job state changes,
//! progress updates, and completion events.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

use super::JobState;

/// Type alias for event sender
pub type JobEventSender = broadcast::Sender<JobEvent>;

/// Type alias for event receiver
pub type JobEventReceiver = broadcast::Receiver<JobEvent>;

/// Job events emitted during job lifecycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobEvent {
    /// Job was created
    Created {
        /// Job ID
        job_id: String,
        /// Job name
        name: String,
        /// When the event occurred
        timestamp: DateTime<Utc>,
    },

    /// Job state changed
    StateChanged {
        /// Job ID
        job_id: String,
        /// Previous state
        from: JobState,
        /// New state
        to: JobState,
        /// When the event occurred
        timestamp: DateTime<Utc>,
    },

    /// Job progress updated
    Progress {
        /// Job ID
        job_id: String,
        /// Progress fraction (0.0 - 1.0)
        progress: f64,
        /// Targets completed
        completed: usize,
        /// Total targets
        total: usize,
        /// When the event occurred
        timestamp: DateTime<Utc>,
    },

    /// Job completed successfully
    Completed {
        /// Job ID
        job_id: String,
        /// Elapsed time in milliseconds
        elapsed_ms: u64,
        /// When the event occurred
        timestamp: DateTime<Utc>,
    },

    /// Job failed
    Failed {
        /// Job ID
        job_id: String,
        /// Error message
        error: String,
        /// When the event occurred
        timestamp: DateTime<Utc>,
    },

    /// Job was cancelled
    Cancelled {
        /// Job ID
        job_id: String,
        /// When the event occurred
        timestamp: DateTime<Utc>,
    },
}

impl JobEvent {
    /// Get the job ID associated with this event
    pub fn job_id(&self) -> &str {
        match self {
            Self::Created { job_id, .. }
            | Self::StateChanged { job_id, .. }
            | Self::Progress { job_id, .. }
            | Self::Completed { job_id, .. }
            | Self::Failed { job_id, .. }
            | Self::Cancelled { job_id, .. } => job_id,
        }
    }

    /// Get the timestamp of this event
    pub fn timestamp(&self) -> DateTime<Utc> {
        match self {
            Self::Created { timestamp, .. }
            | Self::StateChanged { timestamp, .. }
            | Self::Progress { timestamp, .. }
            | Self::Completed { timestamp, .. }
            | Self::Failed { timestamp, .. }
            | Self::Cancelled { timestamp, .. } => *timestamp,
        }
    }
}

/// Create a new job event channel pair
///
/// Returns a sender and receiver. Additional receivers can be
/// created by calling `sender.subscribe()`.
pub fn create_event_channel(capacity: usize) -> (JobEventSender, JobEventReceiver) {
    let (tx, rx) = broadcast::channel(capacity);
    (tx, rx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_job_id() {
        let event = JobEvent::Created {
            job_id: "test-123".to_string(),
            name: "test job".to_string(),
            timestamp: Utc::now(),
        };
        assert_eq!(event.job_id(), "test-123");
    }

    #[test]
    fn test_event_timestamp() {
        let now = Utc::now();
        let event = JobEvent::Completed {
            job_id: "test-123".to_string(),
            elapsed_ms: 1000,
            timestamp: now,
        };
        assert_eq!(event.timestamp(), now);
    }

    #[tokio::test]
    async fn test_event_channel() {
        let (tx, mut rx) = create_event_channel(16);

        let event = JobEvent::Created {
            job_id: "job-1".to_string(),
            name: "test".to_string(),
            timestamp: Utc::now(),
        };

        tx.send(event).unwrap();

        let received = rx.recv().await.unwrap();
        assert_eq!(received.job_id(), "job-1");
    }

    #[tokio::test]
    async fn test_event_channel_multiple_receivers() {
        let (tx, mut rx1) = create_event_channel(16);
        let mut rx2 = tx.subscribe();

        let event = JobEvent::Progress {
            job_id: "job-1".to_string(),
            progress: 0.5,
            completed: 5,
            total: 10,
            timestamp: Utc::now(),
        };

        tx.send(event).unwrap();

        let r1 = rx1.recv().await.unwrap();
        let r2 = rx2.recv().await.unwrap();

        assert_eq!(r1.job_id(), "job-1");
        assert_eq!(r2.job_id(), "job-1");
    }

    #[test]
    fn test_all_event_variants_job_id() {
        let now = Utc::now();
        let events = vec![
            JobEvent::Created {
                job_id: "j1".to_string(),
                name: "test".to_string(),
                timestamp: now,
            },
            JobEvent::StateChanged {
                job_id: "j1".to_string(),
                from: JobState::Created,
                to: JobState::Queued,
                timestamp: now,
            },
            JobEvent::Progress {
                job_id: "j1".to_string(),
                progress: 0.5,
                completed: 1,
                total: 2,
                timestamp: now,
            },
            JobEvent::Completed {
                job_id: "j1".to_string(),
                elapsed_ms: 100,
                timestamp: now,
            },
            JobEvent::Failed {
                job_id: "j1".to_string(),
                error: "err".to_string(),
                timestamp: now,
            },
            JobEvent::Cancelled {
                job_id: "j1".to_string(),
                timestamp: now,
            },
        ];

        for event in &events {
            assert_eq!(event.job_id(), "j1");
            assert_eq!(event.timestamp(), now);
        }
    }
}
