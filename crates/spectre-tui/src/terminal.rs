//! Terminal initialization and restoration utilities.
//!
//! Handles setting up raw mode, alternate screen, mouse capture, and
//! installs a panic hook that restores the terminal on unexpected exit.

use std::io::{self, stdout, Stdout};

use crossterm::{
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

/// Type alias for the configured terminal backend.
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal for TUI rendering.
///
/// Sets up:
/// - Raw mode (disable line buffering, echo)
/// - Alternate screen buffer
/// - Mouse capture
/// - Panic hook to restore terminal on crash
///
/// Returns a configured [`ratatui::Terminal`].
pub fn init_terminal() -> anyhow::Result<Tui> {
    install_panic_hook();
    terminal::enable_raw_mode()?;
    execute!(
        stdout(),
        EnterAlternateScreen,
        crossterm::event::EnableMouseCapture,
    )?;

    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}

/// Restore the terminal to its original state.
///
/// Disables raw mode, leaves the alternate screen, and disables mouse capture.
/// Safe to call multiple times.
pub fn restore_terminal() -> anyhow::Result<()> {
    terminal::disable_raw_mode()?;
    execute!(
        stdout(),
        crossterm::event::DisableMouseCapture,
        LeaveAlternateScreen,
    )?;
    Ok(())
}

/// Install a panic hook that restores the terminal before printing the panic.
///
/// Without this hook, a panic would leave the terminal in raw mode with the
/// alternate screen active, making the panic message invisible and the
/// terminal unusable.
fn install_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // Best-effort terminal restoration - ignore errors
        let _ = terminal::disable_raw_mode();
        let _ = execute!(
            io::stdout(),
            crossterm::event::DisableMouseCapture,
            LeaveAlternateScreen,
        );
        original_hook(panic_info);
    }));
}

#[cfg(test)]
mod tests {
    // NOTE: Terminal init/restore tests cannot run in a headless CI environment
    // because they require an actual terminal. These are verified through
    // integration testing instead.

    #[test]
    fn test_type_alias_exists() {
        // Verify the Tui type alias resolves correctly
        fn _accepts_tui(_t: &super::Tui) {}
    }
}
