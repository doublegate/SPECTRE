//! Job manager for concurrent scan job execution
//!
//! Manages multiple scan jobs with configurable concurrency,
//! rate limiting, and graceful shutdown support.

use std::collections::HashMap;
use std::sync::Arc;

use chrono::Utc;
use tokio::sync::{Mutex, RwLock};
use tokio_util::sync::CancellationToken;
use tracing::{debug, info, instrument, warn};

use super::events::{self, JobEvent, JobEventSender};
use super::state::JobState;
use super::ScanJob;

/// Job manager for orchestrating concurrent scan jobs
///
/// The manager handles:
/// - Job submission and queuing
/// - Concurrent execution with configurable limits
/// - Job lifecycle tracking
/// - Event emission for monitoring
/// - Graceful shutdown via CancellationToken
pub struct JobManager {
    /// All known jobs
    jobs: Arc<RwLock<HashMap<String, ScanJob>>>,
    /// Event sender for job notifications
    event_tx: JobEventSender,
    /// Maximum concurrent jobs
    max_concurrent: usize,
    /// Currently running job count
    running_count: Arc<Mutex<usize>>,
    /// Cancellation token for graceful shutdown
    cancel_token: CancellationToken,
}

impl JobManager {
    /// Create a new job manager
    ///
    /// # Arguments
    ///
    /// * `max_concurrent` - Maximum number of jobs to run concurrently
    /// * `event_capacity` - Channel capacity for event broadcasting
    pub fn new(max_concurrent: usize, event_capacity: usize) -> Self {
        let (event_tx, _rx) = events::create_event_channel(event_capacity);

        Self {
            jobs: Arc::new(RwLock::new(HashMap::new())),
            event_tx,
            max_concurrent: max_concurrent.max(1),
            running_count: Arc::new(Mutex::new(0)),
            cancel_token: CancellationToken::new(),
        }
    }

    /// Subscribe to job events
    pub fn subscribe(&self) -> events::JobEventReceiver {
        self.event_tx.subscribe()
    }

    /// Get the cancellation token for shutdown coordination
    pub fn cancel_token(&self) -> CancellationToken {
        self.cancel_token.clone()
    }

    /// Submit a new job
    #[instrument(skip(self, job), fields(job_id = %job.id, job_name = %job.name))]
    pub async fn submit(&self, mut job: ScanJob) -> crate::Result<String> {
        if self.cancel_token.is_cancelled() {
            return Err(crate::SpectreError::Job(
                crate::error::JobError::ManagerShutdown,
            ));
        }

        let job_id = job.id.clone();
        job.transition(JobState::Queued)?;

        let _ = self.event_tx.send(JobEvent::Created {
            job_id: job_id.clone(),
            name: job.name.clone(),
            timestamp: Utc::now(),
        });

        self.jobs.write().await.insert(job_id.clone(), job);
        info!(job_id = %job_id, "Job submitted");

        Ok(job_id)
    }

    /// Get a job by ID
    pub async fn get_job(&self, job_id: &str) -> crate::Result<ScanJob> {
        self.jobs.read().await.get(job_id).cloned().ok_or_else(|| {
            crate::SpectreError::Job(crate::error::JobError::NotFound(job_id.to_string()))
        })
    }

    /// List all jobs
    pub async fn list_jobs(&self) -> Vec<ScanJob> {
        self.jobs.read().await.values().cloned().collect()
    }

    /// List jobs matching a specific state
    pub async fn list_jobs_by_state(&self, state: JobState) -> Vec<ScanJob> {
        self.jobs
            .read()
            .await
            .values()
            .filter(|j| j.state == state)
            .cloned()
            .collect()
    }

    /// Pause a running job
    #[instrument(skip(self), fields(job_id = %job_id))]
    pub async fn pause_job(&self, job_id: &str) -> crate::Result<()> {
        let mut jobs = self.jobs.write().await;
        let job = jobs.get_mut(job_id).ok_or_else(|| {
            crate::SpectreError::Job(crate::error::JobError::NotFound(job_id.to_string()))
        })?;

        let old_state = job.state;
        job.transition(JobState::Paused)?;

        let _ = self.event_tx.send(JobEvent::StateChanged {
            job_id: job_id.to_string(),
            from: old_state,
            to: JobState::Paused,
            timestamp: Utc::now(),
        });

        info!(job_id = %job_id, "Job paused");
        Ok(())
    }

    /// Resume a paused job
    #[instrument(skip(self), fields(job_id = %job_id))]
    pub async fn resume_job(&self, job_id: &str) -> crate::Result<()> {
        let mut jobs = self.jobs.write().await;
        let job = jobs.get_mut(job_id).ok_or_else(|| {
            crate::SpectreError::Job(crate::error::JobError::NotFound(job_id.to_string()))
        })?;

        let old_state = job.state;
        job.transition(JobState::Running)?;

        let _ = self.event_tx.send(JobEvent::StateChanged {
            job_id: job_id.to_string(),
            from: old_state,
            to: JobState::Running,
            timestamp: Utc::now(),
        });

        info!(job_id = %job_id, "Job resumed");
        Ok(())
    }

    /// Cancel a job
    #[instrument(skip(self), fields(job_id = %job_id))]
    pub async fn cancel_job(&self, job_id: &str) -> crate::Result<()> {
        let mut jobs = self.jobs.write().await;
        let job = jobs.get_mut(job_id).ok_or_else(|| {
            crate::SpectreError::Job(crate::error::JobError::NotFound(job_id.to_string()))
        })?;

        let old_state = job.state;
        job.transition(JobState::Cancelled)?;

        let _ = self.event_tx.send(JobEvent::Cancelled {
            job_id: job_id.to_string(),
            timestamp: Utc::now(),
        });

        info!(job_id = %job_id, from = %old_state, "Job cancelled");
        Ok(())
    }

    /// Update job progress
    pub async fn update_progress(
        &self,
        job_id: &str,
        completed: usize,
        total: usize,
    ) -> crate::Result<()> {
        let mut jobs = self.jobs.write().await;
        let job = jobs.get_mut(job_id).ok_or_else(|| {
            crate::SpectreError::Job(crate::error::JobError::NotFound(job_id.to_string()))
        })?;

        job.update_progress(completed, total);

        let _ = self.event_tx.send(JobEvent::Progress {
            job_id: job_id.to_string(),
            progress: job.progress,
            completed,
            total,
            timestamp: Utc::now(),
        });

        debug!(job_id = %job_id, progress = job.progress, "Progress updated");
        Ok(())
    }

    /// Mark a job as complete
    pub async fn complete_job(&self, job_id: &str) -> crate::Result<()> {
        let mut jobs = self.jobs.write().await;
        let job = jobs.get_mut(job_id).ok_or_else(|| {
            crate::SpectreError::Job(crate::error::JobError::NotFound(job_id.to_string()))
        })?;

        job.transition(JobState::Complete)?;

        let elapsed_ms = job.elapsed().map(|d| d.as_millis() as u64).unwrap_or(0);

        let _ = self.event_tx.send(JobEvent::Completed {
            job_id: job_id.to_string(),
            elapsed_ms,
            timestamp: Utc::now(),
        });

        let mut count = self.running_count.lock().await;
        *count = count.saturating_sub(1);

        info!(job_id = %job_id, elapsed_ms = elapsed_ms, "Job completed");
        Ok(())
    }

    /// Mark a job as failed
    pub async fn fail_job(&self, job_id: &str, error: &str) -> crate::Result<()> {
        let mut jobs = self.jobs.write().await;
        let job = jobs.get_mut(job_id).ok_or_else(|| {
            crate::SpectreError::Job(crate::error::JobError::NotFound(job_id.to_string()))
        })?;

        job.fail(error)?;

        let _ = self.event_tx.send(JobEvent::Failed {
            job_id: job_id.to_string(),
            error: error.to_string(),
            timestamp: Utc::now(),
        });

        let mut count = self.running_count.lock().await;
        *count = count.saturating_sub(1);

        warn!(job_id = %job_id, error = %error, "Job failed");
        Ok(())
    }

    /// Try to start a queued job (if concurrency limit allows)
    pub async fn try_start_job(&self, job_id: &str) -> crate::Result<bool> {
        let mut count = self.running_count.lock().await;
        if *count >= self.max_concurrent {
            return Ok(false);
        }

        let mut jobs = self.jobs.write().await;
        let job = jobs.get_mut(job_id).ok_or_else(|| {
            crate::SpectreError::Job(crate::error::JobError::NotFound(job_id.to_string()))
        })?;

        if job.state != JobState::Queued {
            return Ok(false);
        }

        job.transition(JobState::Running)?;
        *count += 1;

        let _ = self.event_tx.send(JobEvent::StateChanged {
            job_id: job_id.to_string(),
            from: JobState::Queued,
            to: JobState::Running,
            timestamp: Utc::now(),
        });

        info!(job_id = %job_id, "Job started");
        Ok(true)
    }

    /// Get the number of currently running jobs
    pub async fn running_count(&self) -> usize {
        *self.running_count.lock().await
    }

    /// Get the maximum concurrent job limit
    pub fn max_concurrent(&self) -> usize {
        self.max_concurrent
    }

    /// Initiate graceful shutdown
    pub fn shutdown(&self) {
        info!("Job manager shutting down");
        self.cancel_token.cancel();
    }

    /// Check if the manager is shutting down
    pub fn is_shutting_down(&self) -> bool {
        self.cancel_token.is_cancelled()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::job::ScanJobConfig;

    fn make_job(name: &str) -> ScanJob {
        ScanJob::new(
            name,
            ScanJobConfig {
                targets: vec!["10.0.0.1".to_string()],
                ports: vec![80],
                ..Default::default()
            },
        )
    }

    #[tokio::test]
    async fn test_manager_creation() {
        let manager = JobManager::new(4, 64);
        assert_eq!(manager.max_concurrent(), 4);
        assert_eq!(manager.running_count().await, 0);
        assert!(!manager.is_shutting_down());
    }

    #[tokio::test]
    async fn test_submit_job() {
        let manager = JobManager::new(4, 64);
        let job = make_job("test-submit");

        let job_id = manager.submit(job).await.unwrap();
        assert!(!job_id.is_empty());

        let retrieved = manager.get_job(&job_id).await.unwrap();
        assert_eq!(retrieved.name, "test-submit");
        assert_eq!(retrieved.state, JobState::Queued);
    }

    #[tokio::test]
    async fn test_list_jobs() {
        let manager = JobManager::new(4, 64);
        manager.submit(make_job("job-1")).await.unwrap();
        manager.submit(make_job("job-2")).await.unwrap();

        let jobs = manager.list_jobs().await;
        assert_eq!(jobs.len(), 2);
    }

    #[tokio::test]
    async fn test_list_jobs_by_state() {
        let manager = JobManager::new(4, 64);
        let id1 = manager.submit(make_job("job-1")).await.unwrap();
        manager.submit(make_job("job-2")).await.unwrap();

        manager.try_start_job(&id1).await.unwrap();

        let queued = manager.list_jobs_by_state(JobState::Queued).await;
        assert_eq!(queued.len(), 1);

        let running = manager.list_jobs_by_state(JobState::Running).await;
        assert_eq!(running.len(), 1);
    }

    #[tokio::test]
    async fn test_start_job() {
        let manager = JobManager::new(4, 64);
        let job_id = manager.submit(make_job("test")).await.unwrap();

        let started = manager.try_start_job(&job_id).await.unwrap();
        assert!(started);
        assert_eq!(manager.running_count().await, 1);

        let job = manager.get_job(&job_id).await.unwrap();
        assert_eq!(job.state, JobState::Running);
    }

    #[tokio::test]
    async fn test_concurrency_limit() {
        let manager = JobManager::new(1, 64);
        let id1 = manager.submit(make_job("job-1")).await.unwrap();
        let id2 = manager.submit(make_job("job-2")).await.unwrap();

        assert!(manager.try_start_job(&id1).await.unwrap());
        assert!(!manager.try_start_job(&id2).await.unwrap()); // Should be rejected
    }

    #[tokio::test]
    async fn test_pause_resume() {
        let manager = JobManager::new(4, 64);
        let job_id = manager.submit(make_job("test")).await.unwrap();
        manager.try_start_job(&job_id).await.unwrap();

        manager.pause_job(&job_id).await.unwrap();
        let job = manager.get_job(&job_id).await.unwrap();
        assert_eq!(job.state, JobState::Paused);

        manager.resume_job(&job_id).await.unwrap();
        let job = manager.get_job(&job_id).await.unwrap();
        assert_eq!(job.state, JobState::Running);
    }

    #[tokio::test]
    async fn test_cancel_job() {
        let manager = JobManager::new(4, 64);
        let job_id = manager.submit(make_job("test")).await.unwrap();
        manager.try_start_job(&job_id).await.unwrap();

        manager.cancel_job(&job_id).await.unwrap();
        let job = manager.get_job(&job_id).await.unwrap();
        assert_eq!(job.state, JobState::Cancelled);
    }

    #[tokio::test]
    async fn test_complete_job() {
        let manager = JobManager::new(4, 64);
        let job_id = manager.submit(make_job("test")).await.unwrap();
        manager.try_start_job(&job_id).await.unwrap();
        assert_eq!(manager.running_count().await, 1);

        manager.complete_job(&job_id).await.unwrap();
        let job = manager.get_job(&job_id).await.unwrap();
        assert_eq!(job.state, JobState::Complete);
        assert_eq!(manager.running_count().await, 0);
    }

    #[tokio::test]
    async fn test_fail_job() {
        let manager = JobManager::new(4, 64);
        let job_id = manager.submit(make_job("test")).await.unwrap();
        manager.try_start_job(&job_id).await.unwrap();

        manager.fail_job(&job_id, "test error").await.unwrap();
        let job = manager.get_job(&job_id).await.unwrap();
        assert_eq!(job.state, JobState::Failed);
        assert_eq!(job.error.as_deref(), Some("test error"));
    }

    #[tokio::test]
    async fn test_progress_update() {
        let manager = JobManager::new(4, 64);
        let job_id = manager.submit(make_job("test")).await.unwrap();
        manager.try_start_job(&job_id).await.unwrap();

        manager.update_progress(&job_id, 5, 10).await.unwrap();
        let job = manager.get_job(&job_id).await.unwrap();
        assert!((job.progress - 0.5).abs() < f64::EPSILON);
    }

    #[tokio::test]
    async fn test_shutdown() {
        let manager = JobManager::new(4, 64);
        assert!(!manager.is_shutting_down());

        manager.shutdown();
        assert!(manager.is_shutting_down());

        // Submitting after shutdown should fail
        let result = manager.submit(make_job("test")).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_nonexistent_job() {
        let manager = JobManager::new(4, 64);
        let result = manager.get_job("nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_event_subscription() {
        let manager = JobManager::new(4, 64);
        let mut rx = manager.subscribe();

        let job_id = manager.submit(make_job("test")).await.unwrap();

        let event = rx.recv().await.unwrap();
        assert_eq!(event.job_id(), &job_id);
    }
}
