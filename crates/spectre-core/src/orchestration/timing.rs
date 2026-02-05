//! Adaptive timing — auto-adjust scan timing based on network response
//!
//! Tracks success rates and latency to dynamically adjust scan speed.

use std::collections::VecDeque;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use crate::scan::TimingTemplate;

/// Metrics collected for timing adaptation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingMetrics {
    /// Average response time
    pub avg_response_time: Duration,
    /// Packet loss rate (0.0 - 1.0)
    pub loss_rate: f64,
    /// Total packets sent
    pub packets_sent: u64,
    /// Total packets received (responses)
    pub packets_received: u64,
    /// Number of timeouts
    pub timeouts: u64,
    /// Number of errors
    pub errors: u64,
}

impl Default for TimingMetrics {
    fn default() -> Self {
        Self {
            avg_response_time: Duration::ZERO,
            loss_rate: 0.0,
            packets_sent: 0,
            packets_received: 0,
            timeouts: 0,
            errors: 0,
        }
    }
}

/// Adaptive timing controller that adjusts scan speed based on network feedback
#[derive(Debug, Clone)]
pub struct AdaptiveTiming {
    /// Current timing template
    current_timing: TimingTemplate,
    /// Window of recent response times
    response_window: VecDeque<Duration>,
    /// Window size for averaging
    window_size: usize,
    /// Maximum acceptable loss rate before slowing down
    max_loss_rate: f64,
    /// Current metrics
    metrics: TimingMetrics,
    /// Whether adaptation is enabled
    enabled: bool,
    /// Minimum delay between probes
    min_delay: Duration,
    /// Maximum delay between probes
    max_delay: Duration,
    /// Current calculated delay
    current_delay: Duration,
}

impl AdaptiveTiming {
    /// Create a new adaptive timing controller
    pub fn new(initial_timing: TimingTemplate) -> Self {
        let (min_delay, max_delay, current_delay) = Self::timing_bounds(initial_timing);
        Self {
            current_timing: initial_timing,
            response_window: VecDeque::with_capacity(100),
            window_size: 100,
            max_loss_rate: 0.10, // 10% loss threshold
            metrics: TimingMetrics::default(),
            enabled: true,
            min_delay,
            max_delay,
            current_delay,
        }
    }

    /// Get delay bounds for a timing template
    const fn timing_bounds(timing: TimingTemplate) -> (Duration, Duration, Duration) {
        match timing {
            TimingTemplate::Paranoid => (
                Duration::from_secs(5),
                Duration::from_secs(300),
                Duration::from_secs(60),
            ),
            TimingTemplate::Sneaky => (
                Duration::from_secs(1),
                Duration::from_secs(30),
                Duration::from_secs(15),
            ),
            TimingTemplate::Polite => (
                Duration::from_millis(400),
                Duration::from_secs(10),
                Duration::from_secs(1),
            ),
            TimingTemplate::Normal => (
                Duration::from_millis(100),
                Duration::from_secs(5),
                Duration::from_millis(500),
            ),
            TimingTemplate::Aggressive => (
                Duration::from_millis(10),
                Duration::from_secs(1),
                Duration::from_millis(100),
            ),
            TimingTemplate::Insane => (
                Duration::from_millis(1),
                Duration::from_millis(500),
                Duration::from_millis(10),
            ),
        }
    }

    /// Enable or disable adaptive timing
    pub const fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Record a successful probe response
    pub fn record_response(&mut self, response_time: Duration) {
        self.metrics.packets_sent += 1;
        self.metrics.packets_received += 1;

        if self.response_window.len() >= self.window_size {
            self.response_window.pop_front();
        }
        self.response_window.push_back(response_time);

        self.recalculate();
    }

    /// Record a timeout (no response)
    pub fn record_timeout(&mut self) {
        self.metrics.packets_sent += 1;
        self.metrics.timeouts += 1;

        self.recalculate();
    }

    /// Record an error
    pub fn record_error(&mut self) {
        self.metrics.packets_sent += 1;
        self.metrics.errors += 1;

        self.recalculate();
    }

    /// Recalculate timing based on current metrics
    fn recalculate(&mut self) {
        if !self.enabled || self.metrics.packets_sent == 0 {
            return;
        }

        // Calculate loss rate
        self.metrics.loss_rate = if self.metrics.packets_sent > 0 {
            1.0 - (self.metrics.packets_received as f64 / self.metrics.packets_sent as f64)
        } else {
            0.0
        };

        // Calculate average response time
        if !self.response_window.is_empty() {
            let total: Duration = self.response_window.iter().sum();
            self.metrics.avg_response_time = total / self.response_window.len() as u32;
        }

        // Adjust delay based on loss rate
        if self.metrics.loss_rate > self.max_loss_rate {
            // Too much packet loss — slow down
            self.current_delay = Duration::from_millis(
                (self.current_delay.as_millis() as f64 * 1.5).min(self.max_delay.as_millis() as f64)
                    as u64,
            );
            warn!(
                loss_rate = self.metrics.loss_rate,
                new_delay_ms = self.current_delay.as_millis(),
                "High loss rate, slowing down"
            );
        } else if self.metrics.loss_rate < self.max_loss_rate * 0.5 {
            // Low loss rate — speed up cautiously
            self.current_delay = Duration::from_millis(
                (self.current_delay.as_millis() as f64 * 0.9).max(self.min_delay.as_millis() as f64)
                    as u64,
            );
            debug!(
                loss_rate = self.metrics.loss_rate,
                new_delay_ms = self.current_delay.as_millis(),
                "Low loss rate, speeding up"
            );
        }
    }

    /// Get the current recommended delay between probes
    pub const fn current_delay(&self) -> Duration {
        self.current_delay
    }

    /// Get the current timing template
    pub const fn current_timing(&self) -> TimingTemplate {
        self.current_timing
    }

    /// Get the current metrics
    pub const fn metrics(&self) -> &TimingMetrics {
        &self.metrics
    }

    /// Reset the timing controller
    pub fn reset(&mut self) {
        self.response_window.clear();
        self.metrics = TimingMetrics::default();
        let (_, _, default_delay) = Self::timing_bounds(self.current_timing);
        self.current_delay = default_delay;
        info!("Adaptive timing reset");
    }

    /// Get the recommended timing template based on current conditions
    pub fn recommended_timing(&self) -> TimingTemplate {
        if self.metrics.loss_rate > 0.30 {
            TimingTemplate::Polite
        } else if self.metrics.loss_rate > 0.15 {
            TimingTemplate::Normal
        } else if self.metrics.loss_rate < 0.02 {
            TimingTemplate::Aggressive
        } else {
            self.current_timing
        }
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_timing_new() {
        let timing = AdaptiveTiming::new(TimingTemplate::Normal);
        assert!(matches!(timing.current_timing(), TimingTemplate::Normal));
        assert!(timing.current_delay() > Duration::ZERO);
    }

    #[test]
    fn test_record_response() {
        let mut timing = AdaptiveTiming::new(TimingTemplate::Normal);
        timing.record_response(Duration::from_millis(50));
        assert_eq!(timing.metrics().packets_sent, 1);
        assert_eq!(timing.metrics().packets_received, 1);
        assert!(timing.metrics().avg_response_time > Duration::ZERO);
    }

    #[test]
    fn test_record_timeout() {
        let mut timing = AdaptiveTiming::new(TimingTemplate::Normal);
        timing.record_timeout();
        assert_eq!(timing.metrics().packets_sent, 1);
        assert_eq!(timing.metrics().timeouts, 1);
    }

    #[test]
    fn test_record_error() {
        let mut timing = AdaptiveTiming::new(TimingTemplate::Normal);
        timing.record_error();
        assert_eq!(timing.metrics().errors, 1);
    }

    #[test]
    fn test_adaptive_slowdown_on_loss() {
        let mut timing = AdaptiveTiming::new(TimingTemplate::Normal);
        let initial_delay = timing.current_delay();

        // Simulate high packet loss
        for _ in 0..20 {
            timing.record_timeout();
        }

        // Delay should have increased
        assert!(timing.current_delay() >= initial_delay);
    }

    #[test]
    fn test_adaptive_speedup_on_success() {
        let mut timing = AdaptiveTiming::new(TimingTemplate::Normal);

        // Record many successes
        for _ in 0..50 {
            timing.record_response(Duration::from_millis(10));
        }

        assert!(timing.metrics().loss_rate < 0.01);
    }

    #[test]
    fn test_reset() {
        let mut timing = AdaptiveTiming::new(TimingTemplate::Normal);
        timing.record_response(Duration::from_millis(50));
        timing.record_timeout();
        timing.reset();
        assert_eq!(timing.metrics().packets_sent, 0);
    }

    #[test]
    fn test_recommended_timing_high_loss() {
        let mut timing = AdaptiveTiming::new(TimingTemplate::Aggressive);
        // Simulate 50% loss
        for _ in 0..50 {
            timing.record_timeout();
        }
        for _ in 0..50 {
            timing.record_response(Duration::from_millis(100));
        }
        // With high loss, should recommend slower timing
        let recommended = timing.recommended_timing();
        assert!(matches!(
            recommended,
            TimingTemplate::Normal | TimingTemplate::Polite
        ));
    }

    #[test]
    fn test_disabled_adaptation() {
        let mut timing = AdaptiveTiming::new(TimingTemplate::Normal);
        timing.set_enabled(false);
        let initial_delay = timing.current_delay();

        for _ in 0..100 {
            timing.record_timeout();
        }

        // Delay should not change when disabled
        assert_eq!(timing.current_delay(), initial_delay);
    }

    #[test]
    fn test_timing_metrics_default() {
        let metrics = TimingMetrics::default();
        assert_eq!(metrics.packets_sent, 0);
        assert_eq!(metrics.loss_rate, 0.0);
    }
}
