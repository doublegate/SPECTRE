//! Performance metrics collection using tracing-based hooks

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::Mutex;
use tracing::debug;

/// Performance metrics collector
#[derive(Debug, Clone)]
pub struct PerfMetrics {
    timings: Arc<Mutex<HashMap<String, Vec<Duration>>>>,
    counters: Arc<Mutex<HashMap<String, u64>>>,
}

impl PerfMetrics {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            timings: Arc::new(Mutex::new(HashMap::new())),
            counters: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Record a timing measurement
    pub async fn record_timing(&self, name: &str, duration: Duration) {
        let mut timings = self.timings.lock().await;
        timings.entry(name.to_string()).or_default().push(duration);
    }

    /// Increment a counter
    pub async fn increment(&self, name: &str) {
        let mut counters = self.counters.lock().await;
        *counters.entry(name.to_string()).or_insert(0) += 1;
    }

    /// Increment a counter by a specific amount
    pub async fn increment_by(&self, name: &str, amount: u64) {
        let mut counters = self.counters.lock().await;
        *counters.entry(name.to_string()).or_insert(0) += amount;
    }

    /// Get average timing for a metric
    pub async fn avg_timing(&self, name: &str) -> Option<Duration> {
        let timings = self.timings.lock().await;
        timings.get(name).and_then(|durations| {
            if durations.is_empty() {
                None
            } else {
                let total: Duration = durations.iter().sum();
                Some(total / durations.len() as u32)
            }
        })
    }

    /// Get the count for a counter
    pub async fn counter(&self, name: &str) -> u64 {
        let counters = self.counters.lock().await;
        counters.get(name).copied().unwrap_or(0)
    }

    /// Get min/max/avg timing for a metric
    pub async fn timing_stats(&self, name: &str) -> Option<TimingStats> {
        let timings = self.timings.lock().await;
        timings.get(name).and_then(|durations| {
            if durations.is_empty() {
                None
            } else {
                let min = *durations.iter().min().expect("non-empty vec");
                let max = *durations.iter().max().expect("non-empty vec");
                let total: Duration = durations.iter().sum();
                let avg = total / durations.len() as u32;

                Some(TimingStats {
                    min,
                    max,
                    avg,
                    count: durations.len(),
                    total,
                })
            }
        })
    }

    /// Get all timing metric names
    pub async fn timing_names(&self) -> Vec<String> {
        let timings = self.timings.lock().await;
        timings.keys().cloned().collect()
    }

    /// Get all counter names
    pub async fn counter_names(&self) -> Vec<String> {
        let counters = self.counters.lock().await;
        counters.keys().cloned().collect()
    }

    /// Reset all metrics
    pub async fn reset(&self) {
        self.timings.lock().await.clear();
        self.counters.lock().await.clear();
    }
}

impl Default for PerfMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Timing statistics for a metric
#[derive(Debug, Clone)]
pub struct TimingStats {
    /// Minimum duration
    pub min: Duration,
    /// Maximum duration
    pub max: Duration,
    /// Average duration
    pub avg: Duration,
    /// Number of samples
    pub count: usize,
    /// Total duration
    pub total: Duration,
}

/// RAII timer for automatic duration measurement
pub struct PerfTimer {
    name: String,
    start: Instant,
    metrics: PerfMetrics,
}

impl PerfTimer {
    /// Create a new timer that records to the given metrics
    pub fn new(name: &str, metrics: &PerfMetrics) -> Self {
        debug!(name = name, "Starting perf timer");
        Self {
            name: name.to_string(),
            start: Instant::now(),
            metrics: metrics.clone(),
        }
    }

    /// Get elapsed time without stopping the timer
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    /// Stop the timer and record the measurement
    pub async fn stop(self) {
        let duration = self.start.elapsed();
        self.metrics.record_timing(&self.name, duration).await;
        debug!(
            name = self.name,
            duration_ms = duration.as_millis(),
            "Perf timer stopped"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_metrics_new() {
        let metrics = PerfMetrics::new();
        assert!(metrics.timing_names().await.is_empty());
        assert!(metrics.counter_names().await.is_empty());
    }

    #[tokio::test]
    async fn test_record_timing() {
        let metrics = PerfMetrics::new();
        metrics
            .record_timing("scan", Duration::from_millis(100))
            .await;
        metrics
            .record_timing("scan", Duration::from_millis(200))
            .await;

        let avg = metrics.avg_timing("scan").await.unwrap();
        assert_eq!(avg, Duration::from_millis(150));
    }

    #[tokio::test]
    async fn test_increment_counter() {
        let metrics = PerfMetrics::new();
        metrics.increment("packets_sent").await;
        metrics.increment("packets_sent").await;
        metrics.increment("packets_sent").await;

        assert_eq!(metrics.counter("packets_sent").await, 3);
    }

    #[tokio::test]
    async fn test_increment_by() {
        let metrics = PerfMetrics::new();
        metrics.increment_by("bytes", 1024).await;
        metrics.increment_by("bytes", 2048).await;

        assert_eq!(metrics.counter("bytes").await, 3072);
    }

    #[tokio::test]
    async fn test_timing_stats() {
        let metrics = PerfMetrics::new();
        metrics
            .record_timing("op", Duration::from_millis(100))
            .await;
        metrics
            .record_timing("op", Duration::from_millis(300))
            .await;
        metrics
            .record_timing("op", Duration::from_millis(200))
            .await;

        let stats = metrics.timing_stats("op").await.unwrap();
        assert_eq!(stats.min, Duration::from_millis(100));
        assert_eq!(stats.max, Duration::from_millis(300));
        assert_eq!(stats.avg, Duration::from_millis(200));
        assert_eq!(stats.count, 3);
    }

    #[tokio::test]
    async fn test_timing_stats_nonexistent() {
        let metrics = PerfMetrics::new();
        assert!(metrics.timing_stats("nonexistent").await.is_none());
    }

    #[tokio::test]
    async fn test_avg_timing_nonexistent() {
        let metrics = PerfMetrics::new();
        assert!(metrics.avg_timing("nonexistent").await.is_none());
    }

    #[tokio::test]
    async fn test_counter_nonexistent() {
        let metrics = PerfMetrics::new();
        assert_eq!(metrics.counter("nonexistent").await, 0);
    }

    #[tokio::test]
    async fn test_reset() {
        let metrics = PerfMetrics::new();
        metrics
            .record_timing("op", Duration::from_millis(100))
            .await;
        metrics.increment("count").await;

        metrics.reset().await;
        assert!(metrics.timing_names().await.is_empty());
        assert!(metrics.counter_names().await.is_empty());
    }

    #[tokio::test]
    async fn test_perf_timer() {
        let metrics = PerfMetrics::new();
        let timer = PerfTimer::new("test_op", &metrics);

        // Simulate work
        tokio::time::sleep(Duration::from_millis(10)).await;

        assert!(timer.elapsed() >= Duration::from_millis(10));
        timer.stop().await;

        let stats = metrics.timing_stats("test_op").await.unwrap();
        assert!(stats.min >= Duration::from_millis(10));
    }

    #[tokio::test]
    async fn test_metric_names() {
        let metrics = PerfMetrics::new();
        metrics
            .record_timing("alpha", Duration::from_millis(1))
            .await;
        metrics
            .record_timing("beta", Duration::from_millis(1))
            .await;
        metrics.increment("gamma").await;

        let timing_names = metrics.timing_names().await;
        assert_eq!(timing_names.len(), 2);

        let counter_names = metrics.counter_names().await;
        assert_eq!(counter_names.len(), 1);
    }
}
