//! Main application state and logic for the TUI dashboard.
//!
//! The [`App`] struct holds all TUI state, including panel states,
//! scan progress, theme, and command input. It processes events and
//! orchestrates rendering.

use std::time::Duration;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;

use crate::command::{Command, CommandInput};
use crate::keybindings::{map_global_key, map_panel_key, AppAction};
use crate::layout::{LayoutMode, PanelId, PanelManager};
use crate::panels::analysis::AnalysisPanel;
use crate::panels::campaign::CampaignPanel;
use crate::panels::comms::CommsPanel;
use crate::panels::recon::ReconPanel;
use crate::panels::Panel;
use crate::scan_state::ScanState;
use crate::theme::Theme;
use crate::widgets::help_overlay::render_help_overlay;
use crate::widgets::status_bar::{render_header, render_status_bar};

/// Main application state for the TUI dashboard.
pub struct App {
    /// Whether the application is still running
    pub running: bool,
    /// Panel layout manager
    pub panels: PanelManager,
    /// Current color theme
    pub theme: Theme,
    /// Command input widget state
    pub command_input: CommandInput,
    /// Whether the help overlay is visible
    pub show_help: bool,
    /// Tick rate for the render loop
    pub tick_rate: Duration,
    /// Total frames rendered
    pub frame_count: u64,
    /// Active campaign name for the header bar
    pub campaign_name: Option<String>,

    // Panel states
    /// Recon panel state
    pub recon_panel: ReconPanel,
    /// Analysis panel state
    pub analysis_panel: AnalysisPanel,
    /// Comms panel state
    pub comms_panel: CommsPanel,
    /// Campaign panel state
    pub campaign_panel: CampaignPanel,
    /// Active scan state
    pub scan_state: ScanState,
}

impl App {
    /// Create a new application with default settings.
    pub fn new() -> Self {
        Self {
            running: true,
            panels: PanelManager::new(),
            theme: Theme::dark(),
            command_input: CommandInput::new(),
            show_help: false,
            tick_rate: Duration::from_millis(16), // ~60 FPS
            frame_count: 0,
            campaign_name: None,
            recon_panel: ReconPanel::new(),
            analysis_panel: AnalysisPanel::new(),
            comms_panel: CommsPanel::new(),
            campaign_panel: CampaignPanel::new(),
            scan_state: ScanState::new(),
        }
    }

    /// Create a new application with a custom tick rate.
    pub const fn with_tick_rate(mut self, tick_rate: Duration) -> Self {
        self.tick_rate = tick_rate;
        self
    }

    /// Process a tick event (called on each render cycle).
    pub const fn tick(&mut self) {
        self.frame_count += 1;
    }

    /// Handle a terminal resize event.
    pub const fn on_resize(&mut self, _width: u16, _height: u16) {
        // Layout recomputation happens automatically during rendering
    }

    /// Handle a key event.
    pub fn on_key(&mut self, key: KeyEvent) {
        // Command mode takes priority
        if self.command_input.active {
            if key.code == KeyCode::Enter {
                if let Some(cmd) = self.command_input.submit() {
                    self.handle_command(cmd);
                }
                return;
            }
            self.command_input.on_key(key);
            return;
        }

        // Help overlay intercepts Esc and ?
        if self.show_help {
            match key.code {
                KeyCode::Esc | KeyCode::Char('?' | 'q') => {
                    self.show_help = false;
                },
                _ => {},
            }
            return;
        }

        // Try global keybindings first
        let global_action = map_global_key(key);
        if global_action != AppAction::None {
            self.handle_action(global_action);
            return;
        }

        // Try panel-specific keybindings
        let panel_action = map_panel_key(key);
        if panel_action != AppAction::None {
            self.handle_action(panel_action);
            return;
        }

        // Delegate to the focused panel
        let action = match self.panels.focused {
            PanelId::Recon => self.recon_panel.on_key(key),
            PanelId::Analysis => self.analysis_panel.on_key(key),
            PanelId::Comms => self.comms_panel.on_key(key),
            PanelId::Campaign => self.campaign_panel.on_key(key),
        };

        if let Some(action) = action {
            self.handle_action(action);
        }
    }

    /// Handle an application action.
    fn handle_action(&mut self, action: AppAction) {
        match action {
            AppAction::Quit => {
                self.running = false;
            },
            AppAction::ToggleHelp => {
                self.show_help = !self.show_help;
            },
            AppAction::FocusPanel(panel) => {
                self.panels.focus_panel(panel);
            },
            AppAction::FocusNext => {
                self.panels.focus_next();
            },
            AppAction::FocusPrev => {
                self.panels.focus_prev();
            },
            AppAction::EnterCommandMode | AppAction::OpenCommandPalette => {
                self.command_input.activate();
            },
            AppAction::ToggleFocus => {
                self.panels.toggle_focus();
            },
            AppAction::NavigateDown => {
                self.navigate_panel_down();
            },
            AppAction::NavigateUp => {
                self.navigate_panel_up();
            },
            AppAction::JumpToTop => {
                self.navigate_panel_top();
            },
            AppAction::JumpToBottom => {
                self.navigate_panel_bottom();
            },
            AppAction::HalfPageDown => {
                for _ in 0..10 {
                    self.navigate_panel_down();
                }
            },
            AppAction::HalfPageUp => {
                for _ in 0..10 {
                    self.navigate_panel_up();
                }
            },
            AppAction::Back => {
                if self.show_help {
                    self.show_help = false;
                }
            },
            AppAction::NavigateLeft
            | AppAction::NavigateRight
            | AppAction::Select
            | AppAction::None => {},
        }
    }

    /// Navigate down in the focused panel.
    fn navigate_panel_down(&mut self) {
        match self.panels.focused {
            PanelId::Recon => {
                self.scan_state.select_next();
            },
            PanelId::Analysis => {
                self.analysis_panel.scroll_down();
            },
            PanelId::Comms => {
                let key = KeyEvent::new(KeyCode::Char('j'), crossterm::event::KeyModifiers::NONE);
                self.comms_panel.on_key(key);
            },
            PanelId::Campaign => {
                self.campaign_panel.scroll_offset += 1;
            },
        }
    }

    /// Navigate up in the focused panel.
    fn navigate_panel_up(&mut self) {
        match self.panels.focused {
            PanelId::Recon => {
                self.scan_state.select_prev();
            },
            PanelId::Analysis => {
                self.analysis_panel.scroll_up();
            },
            PanelId::Comms => {
                let key = KeyEvent::new(KeyCode::Char('k'), crossterm::event::KeyModifiers::NONE);
                self.comms_panel.on_key(key);
            },
            PanelId::Campaign => {
                self.campaign_panel.scroll_offset =
                    self.campaign_panel.scroll_offset.saturating_sub(1);
            },
        }
    }

    /// Jump to top in the focused panel.
    const fn navigate_panel_top(&mut self) {
        match self.panels.focused {
            PanelId::Recon => {
                self.scan_state.select_first();
            },
            PanelId::Analysis => {
                self.analysis_panel.scroll_offset = 0;
            },
            PanelId::Comms => {},
            PanelId::Campaign => {
                self.campaign_panel.scroll_offset = 0;
            },
        }
    }

    /// Jump to bottom in the focused panel.
    fn navigate_panel_bottom(&mut self) {
        match self.panels.focused {
            PanelId::Recon => {
                self.scan_state.select_last();
            },
            PanelId::Analysis => {
                self.analysis_panel.scroll_offset =
                    self.analysis_panel.output_preview.len().saturating_sub(1);
            },
            PanelId::Comms | PanelId::Campaign => {},
        }
    }

    /// Handle a parsed command.
    fn handle_command(&mut self, command: Command) {
        match command {
            Command::Quit => {
                self.running = false;
            },
            Command::Help => {
                self.show_help = true;
            },
            Command::Theme { name } => {
                if Theme::available_themes().contains(&name.as_str()) {
                    self.theme = Theme::by_name(&name);
                    self.command_input.message = Some(format!("Theme changed to {}", name));
                } else {
                    self.command_input.error = Some(format!(
                        "Unknown theme '{}'. Available: {}",
                        name,
                        Theme::available_themes().join(", ")
                    ));
                }
            },
            Command::Layout { mode } => {
                if let Some(layout_mode) = LayoutMode::from_str_name(&mode) {
                    self.panels.set_mode(layout_mode);
                    self.command_input.message = Some(format!("Layout changed to {}", mode));
                } else {
                    self.command_input.error = Some(format!(
                        "Unknown layout '{}'. Available: {}",
                        mode,
                        LayoutMode::available_modes().join(", ")
                    ));
                }
            },
            Command::Clear => {
                match self.panels.focused {
                    PanelId::Recon => self.scan_state.clear(),
                    PanelId::Analysis => self.analysis_panel.clear(),
                    PanelId::Comms => self.comms_panel.clear(),
                    PanelId::Campaign => self.campaign_panel.clear(),
                }
                self.command_input.message = Some("Panel cleared".to_string());
            },
            Command::Set { key, value } => {
                self.command_input.message = Some(format!("Set {} = {}", key, value));
            },
            Command::Scan { args } => {
                self.command_input.message = Some(format!("Scan requested: {}", args));
                self.panels.focus_panel(PanelId::Recon);
            },
            Command::Chef { args } => {
                self.command_input.message = Some(format!("Chef recipe requested: {}", args));
                self.panels.focus_panel(PanelId::Analysis);
            },
            Command::Send { args } => {
                self.command_input.message = Some(format!("Send requested: {}", args));
                self.panels.focus_panel(PanelId::Comms);
            },
            Command::Campaign { args } => {
                self.command_input.message = Some(format!("Campaign action: {}", args));
                self.panels.focus_panel(PanelId::Campaign);
            },
            Command::Export { args } => {
                self.command_input.message = Some(format!("Export requested: {}", args));
            },
        }
    }

    /// Render the complete application UI.
    pub fn render(&self, frame: &mut Frame) {
        let area = frame.area();

        let (header_area, content_area, status_area) = self.panels.compute_main_areas(area);

        // Render header
        render_header(
            frame,
            header_area,
            &self.theme,
            self.campaign_name.as_deref(),
            self.frame_count,
        );

        // Render panels
        let panel_areas = self.panels.compute_panel_areas(content_area);
        for (panel_id, panel_area) in &panel_areas {
            let focused = *panel_id == self.panels.focused;
            match panel_id {
                PanelId::Recon => {
                    self.recon_panel.render_with_state(
                        frame,
                        *panel_area,
                        focused,
                        &self.theme,
                        &self.scan_state,
                    );
                },
                PanelId::Analysis => {
                    self.analysis_panel
                        .render(frame, *panel_area, focused, &self.theme);
                },
                PanelId::Comms => {
                    self.comms_panel
                        .render(frame, *panel_area, focused, &self.theme);
                },
                PanelId::Campaign => {
                    self.campaign_panel
                        .render(frame, *panel_area, focused, &self.theme);
                },
            }
        }

        // Render status bar
        render_status_bar(
            frame,
            status_area,
            &self.theme,
            self.panels.focused,
            &self.command_input,
        );

        // Render help overlay on top if visible
        if self.show_help {
            render_help_overlay(frame, area, &self.theme);
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyModifiers;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    fn test_terminal() -> Terminal<TestBackend> {
        let backend = TestBackend::new(120, 40);
        Terminal::new(backend).unwrap()
    }

    #[test]
    fn test_app_new() {
        let app = App::new();
        assert!(app.running);
        assert!(!app.show_help);
        assert_eq!(app.frame_count, 0);
        assert_eq!(app.panels.focused, PanelId::Recon);
    }

    #[test]
    fn test_app_default() {
        let app = App::default();
        assert!(app.running);
    }

    #[test]
    fn test_app_with_tick_rate() {
        let app = App::new().with_tick_rate(Duration::from_millis(33));
        assert_eq!(app.tick_rate, Duration::from_millis(33));
    }

    #[test]
    fn test_app_tick() {
        let mut app = App::new();
        app.tick();
        assert_eq!(app.frame_count, 1);
        app.tick();
        assert_eq!(app.frame_count, 2);
    }

    #[test]
    fn test_app_on_resize() {
        let mut app = App::new();
        app.on_resize(200, 60); // should not panic
    }

    #[test]
    fn test_app_quit_on_q() {
        let mut app = App::new();
        app.on_key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));
        assert!(!app.running);
    }

    #[test]
    fn test_app_quit_on_ctrl_c() {
        let mut app = App::new();
        app.on_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL));
        assert!(!app.running);
    }

    #[test]
    fn test_app_toggle_help() {
        let mut app = App::new();
        assert!(!app.show_help);

        app.on_key(KeyEvent::new(KeyCode::Char('?'), KeyModifiers::NONE));
        assert!(app.show_help);

        // Press ? again to close
        app.on_key(KeyEvent::new(KeyCode::Char('?'), KeyModifiers::NONE));
        assert!(!app.show_help);
    }

    #[test]
    fn test_app_help_close_on_esc() {
        let mut app = App::new();
        app.show_help = true;

        app.on_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        assert!(!app.show_help);
    }

    #[test]
    fn test_app_focus_cycle() {
        let mut app = App::new();
        assert_eq!(app.panels.focused, PanelId::Recon);

        app.on_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
        assert_eq!(app.panels.focused, PanelId::Analysis);

        app.on_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
        assert_eq!(app.panels.focused, PanelId::Comms);
    }

    #[test]
    fn test_app_focus_function_keys() {
        let mut app = App::new();

        app.on_key(KeyEvent::new(KeyCode::F(2), KeyModifiers::NONE));
        assert_eq!(app.panels.focused, PanelId::Analysis);

        app.on_key(KeyEvent::new(KeyCode::F(3), KeyModifiers::NONE));
        assert_eq!(app.panels.focused, PanelId::Comms);

        app.on_key(KeyEvent::new(KeyCode::F(4), KeyModifiers::NONE));
        assert_eq!(app.panels.focused, PanelId::Campaign);

        app.on_key(KeyEvent::new(KeyCode::F(1), KeyModifiers::NONE));
        assert_eq!(app.panels.focused, PanelId::Recon);
    }

    #[test]
    fn test_app_enter_command_mode() {
        let mut app = App::new();
        app.on_key(KeyEvent::new(KeyCode::Char(':'), KeyModifiers::NONE));
        assert!(app.command_input.active);
    }

    #[test]
    fn test_app_command_mode_quit() {
        let mut app = App::new();
        app.on_key(KeyEvent::new(KeyCode::Char(':'), KeyModifiers::NONE));
        assert!(app.command_input.active);

        // Type "quit"
        for c in "quit".chars() {
            app.on_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
        }

        app.on_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert!(!app.running);
    }

    #[test]
    fn test_app_command_mode_theme() {
        let mut app = App::new();
        app.command_input.activate();
        app.command_input.buffer = "theme tactical".to_string();
        app.command_input.cursor = app.command_input.buffer.len();

        app.on_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(app.theme.name, "tactical");
    }

    #[test]
    fn test_app_command_mode_layout() {
        let mut app = App::new();
        app.command_input.activate();
        app.command_input.buffer = "layout wide".to_string();
        app.command_input.cursor = app.command_input.buffer.len();

        app.on_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(app.panels.mode, LayoutMode::Wide);
    }

    #[test]
    fn test_app_command_mode_clear() {
        let mut app = App::new();
        app.scan_state.open_ports = 5;
        app.command_input.activate();
        app.command_input.buffer = "clear".to_string();
        app.command_input.cursor = 5;

        app.on_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(app.scan_state.open_ports, 0);
    }

    #[test]
    fn test_app_command_mode_esc() {
        let mut app = App::new();
        app.on_key(KeyEvent::new(KeyCode::Char(':'), KeyModifiers::NONE));
        assert!(app.command_input.active);

        app.on_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        assert!(!app.command_input.active);
    }

    #[test]
    fn test_app_command_mode_help() {
        let mut app = App::new();
        app.command_input.activate();
        app.command_input.buffer = "help".to_string();
        app.command_input.cursor = 4;

        app.on_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert!(app.show_help);
    }

    #[test]
    fn test_app_command_unknown() {
        let mut app = App::new();
        app.command_input.activate();
        app.command_input.buffer = "foobar".to_string();
        app.command_input.cursor = 6;

        app.on_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert!(app.command_input.error.is_some());
    }

    #[test]
    fn test_app_command_theme_invalid() {
        let mut app = App::new();
        app.command_input.activate();
        app.command_input.buffer = "theme nope".to_string();
        app.command_input.cursor = 10;

        app.on_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert!(app.command_input.error.is_some());
        assert_eq!(app.theme.name, "dark"); // unchanged
    }

    #[test]
    fn test_app_navigate_recon() {
        let mut app = App::new();
        // Add some scan results
        for i in 0..5 {
            app.scan_state
                .recent_results
                .push_back(crate::scan_state::PortResultEntry {
                    host: format!("10.0.0.{}", i),
                    port: 22,
                    state: "Open".to_string(),
                    protocol: "tcp".to_string(),
                    service: None,
                    version: None,
                });
        }

        app.on_key(KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE));
        assert_eq!(app.scan_state.selected_index, 1);

        app.on_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
        assert_eq!(app.scan_state.selected_index, 0);

        app.on_key(KeyEvent::new(KeyCode::Char('G'), KeyModifiers::NONE));
        assert_eq!(app.scan_state.selected_index, 4);

        app.on_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
        assert_eq!(app.scan_state.selected_index, 0);
    }

    #[test]
    fn test_app_toggle_focus_mode() {
        let mut app = App::new();
        assert_eq!(app.panels.mode, LayoutMode::Grid);

        app.on_key(KeyEvent::new(KeyCode::F(5), KeyModifiers::NONE));
        assert!(matches!(app.panels.mode, LayoutMode::Focus(_)));

        app.on_key(KeyEvent::new(KeyCode::F(5), KeyModifiers::NONE));
        assert_eq!(app.panels.mode, LayoutMode::Grid);
    }

    #[test]
    fn test_app_half_page() {
        let mut app = App::new();
        for i in 0..50 {
            app.scan_state
                .recent_results
                .push_back(crate::scan_state::PortResultEntry {
                    host: format!("10.0.0.{}", i),
                    port: 22,
                    state: "Open".to_string(),
                    protocol: "tcp".to_string(),
                    service: None,
                    version: None,
                });
        }

        app.on_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL));
        assert_eq!(app.scan_state.selected_index, 10);
    }

    #[test]
    fn test_app_render() {
        let mut terminal = test_terminal();
        let app = App::new();

        terminal
            .draw(|frame| {
                app.render(frame);
            })
            .unwrap();
    }

    #[test]
    fn test_app_render_with_help() {
        let mut terminal = test_terminal();
        let mut app = App::new();
        app.show_help = true;

        terminal
            .draw(|frame| {
                app.render(frame);
            })
            .unwrap();
    }

    #[test]
    fn test_app_render_command_mode() {
        let mut terminal = test_terminal();
        let mut app = App::new();
        app.command_input.activate();
        app.command_input.buffer = "scan test".to_string();
        app.command_input.cursor = 9;

        terminal
            .draw(|frame| {
                app.render(frame);
            })
            .unwrap();
    }

    #[test]
    fn test_app_render_all_layouts() {
        let mut terminal = test_terminal();
        let mut app = App::new();

        for mode in [
            LayoutMode::Grid,
            LayoutMode::Wide,
            LayoutMode::Tall,
            LayoutMode::Focus(PanelId::Recon),
        ] {
            app.panels.set_mode(mode);
            terminal
                .draw(|frame| {
                    app.render(frame);
                })
                .unwrap();
        }
    }

    #[test]
    fn test_app_render_all_themes() {
        let mut terminal = test_terminal();
        let mut app = App::new();

        for name in Theme::available_themes() {
            app.theme = Theme::by_name(name);
            terminal
                .draw(|frame| {
                    app.render(frame);
                })
                .unwrap();
        }
    }

    #[test]
    fn test_app_command_scan() {
        let mut app = App::new();
        app.command_input.activate();
        app.command_input.buffer = "scan 10.0.0.0/24".to_string();
        app.command_input.cursor = app.command_input.buffer.len();

        app.on_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(app.panels.focused, PanelId::Recon);
        assert!(app.command_input.message.is_some());
    }

    #[test]
    fn test_app_command_chef() {
        let mut app = App::new();
        app.command_input.activate();
        app.command_input.buffer = "chef base64_decode".to_string();
        app.command_input.cursor = app.command_input.buffer.len();

        app.on_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(app.panels.focused, PanelId::Analysis);
    }

    #[test]
    fn test_app_command_send() {
        let mut app = App::new();
        app.command_input.activate();
        app.command_input.buffer = "send peer1 data".to_string();
        app.command_input.cursor = app.command_input.buffer.len();

        app.on_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(app.panels.focused, PanelId::Comms);
    }

    #[test]
    fn test_app_command_campaign() {
        let mut app = App::new();
        app.command_input.activate();
        app.command_input.buffer = "campaign new Test".to_string();
        app.command_input.cursor = app.command_input.buffer.len();

        app.on_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(app.panels.focused, PanelId::Campaign);
    }

    #[test]
    fn test_app_command_export() {
        let mut app = App::new();
        app.command_input.activate();
        app.command_input.buffer = "export json".to_string();
        app.command_input.cursor = app.command_input.buffer.len();

        app.on_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert!(app.command_input.message.is_some());
    }

    #[test]
    fn test_app_command_set() {
        let mut app = App::new();
        app.command_input.activate();
        app.command_input.buffer = "set scan.timing 4".to_string();
        app.command_input.cursor = app.command_input.buffer.len();

        app.on_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert!(app.command_input.message.is_some());
    }

    #[test]
    fn test_app_command_layout_invalid() {
        let mut app = App::new();
        app.command_input.activate();
        app.command_input.buffer = "layout invalid".to_string();
        app.command_input.cursor = app.command_input.buffer.len();

        app.on_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert!(app.command_input.error.is_some());
    }

    #[test]
    fn test_app_navigate_analysis() {
        let mut app = App::new();
        app.panels.focus_panel(PanelId::Analysis);
        app.analysis_panel
            .set_complete(vec!["a".to_string(), "b".to_string(), "c".to_string()]);

        app.on_key(KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE));
        assert_eq!(app.analysis_panel.scroll_offset, 1);
    }

    #[test]
    fn test_app_navigate_campaign() {
        let mut app = App::new();
        app.panels.focus_panel(PanelId::Campaign);

        app.on_key(KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE));
        assert_eq!(app.campaign_panel.scroll_offset, 1);

        app.on_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
        assert_eq!(app.campaign_panel.scroll_offset, 0);
    }
}
