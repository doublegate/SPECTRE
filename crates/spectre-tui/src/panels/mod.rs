//! Panel implementations for the TUI dashboard.
//!
//! Each panel renders a specific domain of the SPECTRE toolkit:
//! - [`recon`] - Network reconnaissance (ProRT-IP)
//! - [`analysis`] - Data analysis (CyberChef-MCP)
//! - [`comms`] - Secure communications (WRAITH)
//! - [`campaign`] - Campaign management

pub mod analysis;
pub mod campaign;
pub mod comms;
pub mod recon;

use crossterm::event::KeyEvent;
use ratatui::layout::Rect;
use ratatui::Frame;

use crate::keybindings::AppAction;
use crate::theme::Theme;

/// Trait for panel rendering and event handling.
pub trait Panel {
    /// Render the panel contents into the given area.
    fn render(&self, frame: &mut Frame, area: Rect, focused: bool, theme: &Theme);

    /// Handle a key event within this panel.
    ///
    /// Returns an `AppAction` if the panel wants to trigger a global action,
    /// or `None` if the event was consumed locally.
    fn on_key(&mut self, key: KeyEvent) -> Option<AppAction>;

    /// Get the display title for this panel.
    fn title(&self) -> &str;
}
