//! Event handling for the TUI dashboard.
//!
//! Provides an async event handler that polls crossterm for keyboard, mouse,
//! and resize events, and also emits periodic tick events for rendering.

use std::time::Duration;

use crossterm::event::{self, Event, KeyEvent, MouseEvent};
use tokio::sync::mpsc;

/// Application-level events.
#[derive(Debug, Clone)]
pub enum AppEvent {
    /// Keyboard event
    Key(KeyEvent),
    /// Mouse event
    Mouse(MouseEvent),
    /// Terminal resize event
    Resize(u16, u16),
    /// Periodic tick for rendering updates
    Tick,
}

/// Async event handler that polls crossterm events and emits tick events.
pub struct EventHandler {
    /// Receiving end of the event channel
    rx: mpsc::UnboundedReceiver<AppEvent>,
    /// Handle to the background polling task
    _task: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    /// Create a new event handler with the given tick rate.
    ///
    /// Spawns a background tokio task that polls for crossterm events
    /// and emits tick events at the specified interval.
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        let task = tokio::spawn(async move {
            let mut tick_interval = tokio::time::interval(tick_rate);
            loop {
                let event = tokio::select! {
                    _ = tick_interval.tick() => {
                        AppEvent::Tick
                    }
                    maybe_event = Self::poll_crossterm() => {
                        match maybe_event {
                            Some(evt) => evt,
                            None => continue,
                        }
                    }
                };

                if tx.send(event).is_err() {
                    break;
                }
            }
        });

        Self { rx, _task: task }
    }

    /// Poll crossterm for the next event (non-blocking via tokio).
    async fn poll_crossterm() -> Option<AppEvent> {
        let available = tokio::task::spawn_blocking(|| event::poll(Duration::from_millis(10)))
            .await
            .ok()?
            .ok()?;

        if !available {
            // Yield briefly to avoid busy-spinning
            tokio::time::sleep(Duration::from_millis(1)).await;
            return None;
        }

        let raw_event = tokio::task::spawn_blocking(event::read).await.ok()?.ok()?;

        match raw_event {
            Event::Key(key) => Some(AppEvent::Key(key)),
            Event::Mouse(mouse) => Some(AppEvent::Mouse(mouse)),
            Event::Resize(w, h) => Some(AppEvent::Resize(w, h)),
            _ => None,
        }
    }

    /// Receive the next event, blocking until one is available.
    pub async fn next(&mut self) -> anyhow::Result<AppEvent> {
        self.rx
            .recv()
            .await
            .ok_or_else(|| anyhow::anyhow!("Event channel closed"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_event_debug() {
        let tick = AppEvent::Tick;
        let debug_str = format!("{:?}", tick);
        assert!(debug_str.contains("Tick"));
    }

    #[test]
    fn test_app_event_clone() {
        let tick = AppEvent::Tick;
        let cloned = tick.clone();
        assert!(matches!(cloned, AppEvent::Tick));
    }

    #[test]
    fn test_app_event_resize() {
        let resize = AppEvent::Resize(120, 40);
        if let AppEvent::Resize(w, h) = resize {
            assert_eq!(w, 120);
            assert_eq!(h, 40);
        } else {
            panic!("Expected Resize event");
        }
    }

    #[tokio::test]
    async fn test_event_handler_creation() {
        // Simply verify the event handler can be created without panicking
        let _handler = EventHandler::new(Duration::from_millis(100));
        // Give it a moment, then drop
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
