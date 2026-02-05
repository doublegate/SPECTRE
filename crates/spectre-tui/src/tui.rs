//! Main TUI run loop entry point.
//!
//! Provides the [`run`] function that initializes the terminal,
//! creates the application state, and enters the main event loop.

use anyhow::Result;
use tracing::info;

use crate::app::App;
use crate::event::{AppEvent, EventHandler};
use crate::terminal::{init_terminal, restore_terminal};

/// Main entry point for the TUI dashboard.
///
/// Initializes the terminal, creates the application state, and enters
/// the main render loop. Returns when the user quits.
///
/// # Arguments
///
/// * `config` - The SPECTRE configuration (passed to App for component clients)
///
/// # Errors
///
/// Returns an error if terminal initialization fails or if an unrecoverable
/// error occurs during the event loop.
pub async fn run(config: &spectre_core::config::Config) -> Result<()> {
    info!("Starting SPECTRE TUI dashboard");

    let mut events = EventHandler::new(std::time::Duration::from_millis(16));
    let event_tx = events.sender();
    let mut app = App::with_config(config.clone(), event_tx);
    let mut terminal = init_terminal()?;

    // Main event loop
    while app.running {
        // Render
        terminal.draw(|frame| {
            app.render(frame);
        })?;

        // Handle events
        match events.next().await? {
            AppEvent::Key(key) => {
                app.on_key(key);
            },
            AppEvent::Mouse(_mouse) => {
                // Mouse events reserved for future use
            },
            AppEvent::Resize(w, h) => {
                app.on_resize(w, h);
            },
            AppEvent::Tick => {
                app.tick();
            },
            AppEvent::Component(component_event) => {
                app.handle_component_event(component_event);
            },
        }
    }

    // Cleanup
    restore_terminal()?;
    info!("SPECTRE TUI dashboard shut down cleanly");

    Ok(())
}

#[cfg(test)]
mod tests {
    // The run() function requires a real terminal and is tested via
    // integration tests. Here we verify module compilation and imports.

    #[test]
    #[allow(clippy::no_effect_underscore_binding)]
    fn test_run_function_exists() {
        // Verify the run function is accessible from the module
        // by confirming we can reference it as a value
        let _run_ref = super::run;
    }
}
