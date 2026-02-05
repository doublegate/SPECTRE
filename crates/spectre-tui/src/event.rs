//! Event handling for the TUI dashboard.
//!
//! Provides an async event handler that polls crossterm for keyboard, mouse,
//! and resize events, and also emits periodic tick events for rendering.

use std::time::Duration;

use crossterm::event::{self, Event, KeyEvent, MouseEvent};
use tokio::sync::mpsc;

use spectre_core::scan::PortResult;

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
    /// Result from an async component operation
    Component(ComponentEvent),
}

/// Events from async component operations (scan results, chef output, comms state).
#[derive(Debug, Clone)]
pub enum ComponentEvent {
    /// Scan progress update
    ScanProgress { completed: usize, total: usize },
    /// Scan host results
    ScanResult {
        host: String,
        ports: Vec<PortResult>,
    },
    /// Scan completed
    ScanComplete,
    /// Scan failed
    ScanFailed { error: String },
    /// Chef operation completed with output
    ChefComplete { recipe: String, output: Vec<String> },
    /// Chef operation failed
    ChefFailed { recipe: String, error: String },
    /// Comms message sent
    CommsSent { peer: String, size: usize },
    /// Comms send failed
    CommsFailed { peer: String, error: String },
    /// Status message (generic info)
    StatusMessage(String),
}

/// Async event handler that polls crossterm events and emits tick events.
pub struct EventHandler {
    /// Receiving end of the event channel
    rx: mpsc::UnboundedReceiver<AppEvent>,
    /// Sender end of the event channel (kept for cloning)
    tx: mpsc::UnboundedSender<AppEvent>,
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
        let tx_clone = tx.clone();

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

                if tx_clone.send(event).is_err() {
                    break;
                }
            }
        });

        Self {
            rx,
            tx,
            _task: task,
        }
    }

    /// Get a clone of the event sender for component tasks.
    pub fn sender(&self) -> mpsc::UnboundedSender<AppEvent> {
        self.tx.clone()
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
        let cloned = Clone::clone(&tick);
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

    #[test]
    fn test_app_event_component() {
        let event = AppEvent::Component(ComponentEvent::ScanComplete);
        assert!(matches!(
            event,
            AppEvent::Component(ComponentEvent::ScanComplete)
        ));
    }

    #[test]
    fn test_component_event_debug() {
        let event = ComponentEvent::ScanProgress {
            completed: 5,
            total: 10,
        };
        let debug_str = format!("{:?}", event);
        assert!(debug_str.contains("ScanProgress"));
    }

    #[test]
    fn test_component_event_clone() {
        let event = ComponentEvent::StatusMessage("test".to_string());
        let cloned = event.clone();
        assert!(matches!(cloned, ComponentEvent::StatusMessage(ref msg) if msg == "test"));
    }

    #[test]
    fn test_component_event_scan_failed() {
        let event = ComponentEvent::ScanFailed {
            error: "timeout".to_string(),
        };
        assert!(matches!(event, ComponentEvent::ScanFailed { ref error } if error == "timeout"));
    }

    #[test]
    fn test_component_event_chef_complete() {
        let event = ComponentEvent::ChefComplete {
            recipe: "base64".to_string(),
            output: vec!["decoded".to_string()],
        };
        assert!(
            matches!(event, ComponentEvent::ChefComplete { ref recipe, .. } if recipe == "base64")
        );
    }

    #[test]
    fn test_component_event_chef_failed() {
        let event = ComponentEvent::ChefFailed {
            recipe: "bad".to_string(),
            error: "not found".to_string(),
        };
        assert!(matches!(event, ComponentEvent::ChefFailed { ref recipe, .. } if recipe == "bad"));
    }

    #[test]
    fn test_component_event_comms_sent() {
        let event = ComponentEvent::CommsSent {
            peer: "alpha".to_string(),
            size: 1024,
        };
        assert!(matches!(
            event,
            ComponentEvent::CommsSent { size: 1024, .. }
        ));
    }

    #[test]
    fn test_component_event_comms_failed() {
        let event = ComponentEvent::CommsFailed {
            peer: "alpha".to_string(),
            error: "refused".to_string(),
        };
        assert!(matches!(event, ComponentEvent::CommsFailed { ref peer, .. } if peer == "alpha"));
    }

    #[test]
    fn test_component_event_scan_result() {
        let event = ComponentEvent::ScanResult {
            host: "10.0.0.1".to_string(),
            ports: vec![],
        };
        assert!(matches!(event, ComponentEvent::ScanResult { ref host, .. } if host == "10.0.0.1"));
    }

    #[tokio::test]
    async fn test_event_handler_creation() {
        // Simply verify the event handler can be created without panicking
        let _handler = EventHandler::new(Duration::from_millis(100));
        // Give it a moment, then drop
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    #[tokio::test]
    async fn test_event_handler_sender() {
        let handler = EventHandler::new(Duration::from_millis(100));
        let sender = handler.sender();
        // Sender should be usable for sending component events
        let result = sender.send(AppEvent::Component(ComponentEvent::ScanComplete));
        assert!(result.is_ok());
    }
}
