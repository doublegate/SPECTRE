//! Main application state and logic for the TUI dashboard.
//!
//! The [`App`] struct holds all TUI state, including panel states,
//! scan progress, theme, and command input. It processes events and
//! orchestrates rendering.

use std::collections::HashMap;
use std::time::Duration;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use tokio::sync::mpsc;

use spectre_core::config::Config;

use crate::command::{Command, CommandInput};
use crate::event::{AppEvent, ComponentEvent};
use crate::keybindings::{map_global_key, map_panel_key, AppAction};
use crate::layout::{LayoutMode, PanelId, PanelManager};
use crate::panels::analysis::AnalysisPanel;
use crate::panels::campaign::CampaignPanel;
use crate::panels::comms::{self, CommsPanel};
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

    /// Config for creating component clients
    config: Config,
    /// Sender for component events back to the event loop
    event_tx: mpsc::UnboundedSender<AppEvent>,
}

impl App {
    /// Create a new application with default settings.
    ///
    /// Uses a default config and a no-op event sender. Suitable for tests
    /// that do not exercise async component commands.
    pub fn new() -> Self {
        let (tx, _rx) = mpsc::unbounded_channel();
        Self::with_config(Config::default(), tx)
    }

    /// Create a new application with the given config and event sender.
    ///
    /// The `event_tx` sender is used to deliver results from async component
    /// operations (scans, chef, comms) back to the main event loop.
    pub fn with_config(config: Config, event_tx: mpsc::UnboundedSender<AppEvent>) -> Self {
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
            config,
            event_tx,
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
                let parts: Vec<&str> = args.split_whitespace().collect();
                if parts.is_empty() {
                    self.command_input.error = Some("Usage: :scan <target> [-p ports]".to_string());
                    return;
                }
                let target = parts[0].to_string();
                // Parse -p flag if present
                let ports_spec = parts
                    .iter()
                    .position(|&p| p == "-p")
                    .and_then(|i| parts.get(i + 1))
                    .copied()
                    .map(String::from);

                self.scan_state.start(
                    &target,
                    spectre_core::scan::ScanType::Connect,
                    0,
                    "tui-scan",
                );
                self.panels.focus_panel(PanelId::Recon);
                self.command_input.message = Some(format!("Starting scan: {}", target));

                let config = self.config.clone();
                let tx = self.event_tx.clone();
                let target_clone = target.clone();
                tokio::spawn(async move {
                    match spectre_core::scan::create_stub_scanner(&config) {
                        Ok(scanner) => {
                            let targets = match spectre_core::scan::parse_targets(&[target_clone]) {
                                Ok(t) => t,
                                Err(e) => {
                                    let _ =
                                        tx.send(AppEvent::Component(ComponentEvent::ScanFailed {
                                            error: format!("Invalid target: {}", e),
                                        }));
                                    return;
                                },
                            };
                            let port_list = spectre_core::scan::parse_ports(ports_spec.as_deref())
                                .unwrap_or_else(|_| vec![22, 80, 443, 8080, 8443]);

                            let scan_config = spectre_core::scan::ScanConfig {
                                targets,
                                ports: port_list,
                                scan_type: spectre_core::scan::ScanType::Connect,
                                timing: spectre_core::scan::TimingTemplate::Normal,
                                ..Default::default()
                            };

                            let tx2 = tx.clone();
                            let callback: spectre_core::scan::ProgressCallback =
                                Box::new(move |completed, total| {
                                    let _ = tx2.send(AppEvent::Component(
                                        ComponentEvent::ScanProgress { completed, total },
                                    ));
                                });

                            match scanner.scan(scan_config, Some(callback)).await {
                                Ok(results) => {
                                    for result in &results {
                                        let _ = tx.send(AppEvent::Component(
                                            ComponentEvent::ScanResult {
                                                host: result.host.clone(),
                                                ports: result.ports.clone(),
                                            },
                                        ));
                                    }
                                    let _ =
                                        tx.send(AppEvent::Component(ComponentEvent::ScanComplete));
                                },
                                Err(e) => {
                                    let _ =
                                        tx.send(AppEvent::Component(ComponentEvent::ScanFailed {
                                            error: e.to_string(),
                                        }));
                                },
                            }
                        },
                        Err(e) => {
                            let _ = tx.send(AppEvent::Component(ComponentEvent::ScanFailed {
                                error: format!("Scanner unavailable: {}", e),
                            }));
                        },
                    }
                });
            },
            Command::Chef { args } => {
                let parts: Vec<&str> = args.splitn(2, char::is_whitespace).collect();
                if parts.is_empty() || parts[0].is_empty() {
                    self.command_input.error = Some("Usage: :chef <operation> [input]".to_string());
                    return;
                }
                let operation = parts[0].to_string();
                let input = parts.get(1).unwrap_or(&"").to_string();

                self.analysis_panel.set_recipe(&operation);
                self.panels.focus_panel(PanelId::Analysis);
                self.command_input.message = Some(format!("Running: {}", operation));

                let config = self.config.clone();
                let tx = self.event_tx.clone();
                let op = operation.clone();
                tokio::spawn(async move {
                    match spectre_core::chef::create_stub_chef_client(&config).await {
                        Ok(client) => {
                            let args_map: HashMap<String, String> = HashMap::new();
                            match client.execute(&op, input.as_bytes(), &args_map).await {
                                Ok(output) => {
                                    let output_str = String::from_utf8_lossy(&output).to_string();
                                    let lines: Vec<String> =
                                        output_str.lines().map(String::from).collect();
                                    let _ = tx.send(AppEvent::Component(
                                        ComponentEvent::ChefComplete {
                                            recipe: op,
                                            output: lines,
                                        },
                                    ));
                                },
                                Err(e) => {
                                    let _ =
                                        tx.send(AppEvent::Component(ComponentEvent::ChefFailed {
                                            recipe: op,
                                            error: e.to_string(),
                                        }));
                                },
                            }
                        },
                        Err(e) => {
                            let _ = tx.send(AppEvent::Component(ComponentEvent::ChefFailed {
                                recipe: op,
                                error: format!("Chef unavailable: {}", e),
                            }));
                        },
                    }
                });
            },
            Command::Send { args } => {
                let parts: Vec<&str> = args.splitn(2, char::is_whitespace).collect();
                if parts.len() < 2 {
                    self.command_input.error = Some("Usage: :send <peer> <message>".to_string());
                    return;
                }
                let peer_name = parts[0].to_string();
                let data = parts[1].to_string();
                let data_size = data.len();

                // Add a transfer entry to the UI
                self.comms_panel.transfers.push(comms::TransferEntry {
                    name: format!("msg-to-{}", peer_name),
                    direction: comms::TransferDirection::Send,
                    progress: 0.0,
                    size: data_size as u64,
                    status: comms::TransferStatus::Active,
                });
                self.panels.focus_panel(PanelId::Comms);
                self.command_input.message = Some(format!("Sending to {}...", peer_name));

                let tx = self.event_tx.clone();
                let peer = peer_name.clone();
                tokio::spawn(async move {
                    // Simulate the send completing after a brief delay.
                    // Real WRAITH integration requires identity/peer setup first.
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    let _ = tx.send(AppEvent::Component(ComponentEvent::CommsSent {
                        peer,
                        size: data_size,
                    }));
                });
            },
            Command::Campaign { args } => {
                let parts: Vec<&str> = args.splitn(2, char::is_whitespace).collect();
                let subcommand = parts.first().copied().unwrap_or_default();
                let rest = parts.get(1).copied().unwrap_or_default();

                match subcommand {
                    "new" | "create" if !rest.is_empty() => {
                        self.campaign_name = Some(rest.to_string());
                        self.command_input.message = Some(format!("Campaign '{}' created", rest));
                    },
                    "status" => {
                        let name = self.campaign_name.as_deref().unwrap_or("none");
                        self.command_input.message = Some(format!("Active campaign: {}", name));
                    },
                    "clear" | "end" => {
                        self.campaign_name = None;
                        self.campaign_panel.clear();
                        self.command_input.message = Some("Campaign ended".to_string());
                    },
                    _ => {
                        self.command_input.error =
                            Some("Usage: :campaign new|status|clear [name]".to_string());
                    },
                }
                self.panels.focus_panel(PanelId::Campaign);
            },
            Command::Export { args } => {
                self.command_input.message = Some(format!("Export requested: {}", args));
            },
        }
    }

    /// Handle an event from an async component operation.
    pub fn handle_component_event(&mut self, event: ComponentEvent) {
        match event {
            ComponentEvent::ScanProgress { completed, total } => {
                self.scan_state.update_progress(completed, total);
            },
            ComponentEvent::ScanResult { host, ports } => {
                self.scan_state.add_host_results(&host, &ports);
            },
            ComponentEvent::ScanComplete => {
                self.scan_state.complete();
                self.command_input.message = Some("Scan complete".to_string());
            },
            ComponentEvent::ScanFailed { error } => {
                self.scan_state.fail(&error);
                self.command_input.error = Some(format!("Scan failed: {}", error));
            },
            ComponentEvent::ChefComplete { recipe, output } => {
                self.analysis_panel.set_complete(output);
                self.command_input.message = Some(format!("Chef '{}' complete", recipe));
            },
            ComponentEvent::ChefFailed { recipe, error } => {
                self.analysis_panel.set_error(&error);
                self.command_input.error = Some(format!("Chef '{}' failed: {}", recipe, error));
            },
            ComponentEvent::CommsSent { peer, size } => {
                // Mark the latest transfer as complete
                if let Some(xfer) = self.comms_panel.transfers.last_mut() {
                    xfer.progress = 1.0;
                    xfer.status = comms::TransferStatus::Complete;
                }
                self.command_input.message = Some(format!("Sent {} bytes to {}", size, peer));
            },
            ComponentEvent::CommsFailed { peer, error } => {
                if let Some(xfer) = self.comms_panel.transfers.last_mut() {
                    xfer.status = comms::TransferStatus::Failed;
                }
                self.command_input.error = Some(format!("Send to {} failed: {}", peer, error));
            },
            ComponentEvent::StatusMessage(msg) => {
                self.command_input.message = Some(msg);
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

    #[tokio::test]
    async fn test_app_command_scan() {
        let mut app = App::new();
        app.command_input.activate();
        app.command_input.buffer = "scan 10.0.0.0/24".to_string();
        app.command_input.cursor = app.command_input.buffer.len();

        app.on_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(app.panels.focused, PanelId::Recon);
        assert!(app.command_input.message.is_some());
    }

    #[tokio::test]
    async fn test_app_command_chef() {
        let mut app = App::new();
        app.command_input.activate();
        app.command_input.buffer = "chef base64_decode".to_string();
        app.command_input.cursor = app.command_input.buffer.len();

        app.on_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(app.panels.focused, PanelId::Analysis);
    }

    #[tokio::test]
    async fn test_app_command_send() {
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

    #[test]
    fn test_app_with_config() {
        let config = Config::default();
        let (tx, _rx) = mpsc::unbounded_channel();
        let app = App::with_config(config, tx);
        assert!(app.running);
    }

    // -- Component event handler tests --

    #[test]
    fn test_handle_scan_progress() {
        let mut app = App::new();
        app.scan_state.start(
            "10.0.0.1",
            spectre_core::scan::ScanType::Connect,
            10,
            "test",
        );
        app.handle_component_event(ComponentEvent::ScanProgress {
            completed: 5,
            total: 10,
        });
        assert_eq!(app.scan_state.hosts_scanned, 5);
        assert_eq!(app.scan_state.hosts_total, 10);
    }

    #[test]
    fn test_handle_scan_result() {
        let mut app = App::new();
        app.scan_state
            .start("10.0.0.1", spectre_core::scan::ScanType::Connect, 1, "test");

        let ports = vec![spectre_core::scan::PortResult {
            port: 80,
            protocol: spectre_core::scan::Protocol::Tcp,
            state: spectre_core::scan::PortState::Open,
            service: Some("http".to_string()),
            version: None,
            banner: None,
        }];

        app.handle_component_event(ComponentEvent::ScanResult {
            host: "10.0.0.1".to_string(),
            ports,
        });
        assert_eq!(app.scan_state.recent_results.len(), 1);
        assert_eq!(app.scan_state.open_ports, 1);
    }

    #[test]
    fn test_handle_scan_complete() {
        let mut app = App::new();
        app.scan_state
            .start("10.0.0.1", spectre_core::scan::ScanType::Connect, 1, "test");
        app.handle_component_event(ComponentEvent::ScanComplete);
        assert!(!app.scan_state.active);
        assert_eq!(app.command_input.message.as_deref(), Some("Scan complete"));
    }

    #[test]
    fn test_handle_scan_failed() {
        let mut app = App::new();
        app.scan_state
            .start("10.0.0.1", spectre_core::scan::ScanType::Connect, 1, "test");
        app.handle_component_event(ComponentEvent::ScanFailed {
            error: "timeout".to_string(),
        });
        assert!(!app.scan_state.active);
        assert!(app
            .command_input
            .error
            .as_ref()
            .unwrap()
            .contains("timeout"));
    }

    #[test]
    fn test_handle_chef_complete() {
        let mut app = App::new();
        app.analysis_panel.set_recipe("base64_decode");
        app.handle_component_event(ComponentEvent::ChefComplete {
            recipe: "base64_decode".to_string(),
            output: vec!["decoded output".to_string()],
        });
        assert_eq!(
            app.analysis_panel.status,
            crate::panels::analysis::AnalysisStatus::Complete
        );
        assert_eq!(app.analysis_panel.output_preview.len(), 1);
        assert!(app
            .command_input
            .message
            .as_ref()
            .unwrap()
            .contains("base64_decode"));
    }

    #[test]
    fn test_handle_chef_failed() {
        let mut app = App::new();
        app.analysis_panel.set_recipe("bad_recipe");
        app.handle_component_event(ComponentEvent::ChefFailed {
            recipe: "bad_recipe".to_string(),
            error: "not found".to_string(),
        });
        assert!(matches!(
            app.analysis_panel.status,
            crate::panels::analysis::AnalysisStatus::Error(_)
        ));
        assert!(app
            .command_input
            .error
            .as_ref()
            .unwrap()
            .contains("bad_recipe"));
    }

    #[test]
    fn test_handle_comms_sent() {
        let mut app = App::new();
        app.comms_panel.transfers.push(comms::TransferEntry {
            name: "msg-to-alpha".to_string(),
            direction: comms::TransferDirection::Send,
            progress: 0.0,
            size: 100,
            status: comms::TransferStatus::Active,
        });
        app.handle_component_event(ComponentEvent::CommsSent {
            peer: "alpha".to_string(),
            size: 100,
        });
        let xfer = app.comms_panel.transfers.last().unwrap();
        assert_eq!(xfer.status, comms::TransferStatus::Complete);
        assert!((xfer.progress - 1.0).abs() < f64::EPSILON);
        assert!(app.command_input.message.as_ref().unwrap().contains("100"));
    }

    #[test]
    fn test_handle_comms_failed() {
        let mut app = App::new();
        app.comms_panel.transfers.push(comms::TransferEntry {
            name: "msg-to-bravo".to_string(),
            direction: comms::TransferDirection::Send,
            progress: 0.0,
            size: 50,
            status: comms::TransferStatus::Active,
        });
        app.handle_component_event(ComponentEvent::CommsFailed {
            peer: "bravo".to_string(),
            error: "refused".to_string(),
        });
        let xfer = app.comms_panel.transfers.last().unwrap();
        assert_eq!(xfer.status, comms::TransferStatus::Failed);
        assert!(app
            .command_input
            .error
            .as_ref()
            .unwrap()
            .contains("refused"));
    }

    #[test]
    fn test_handle_comms_sent_no_transfers() {
        // Should not panic when transfers list is empty
        let mut app = App::new();
        app.handle_component_event(ComponentEvent::CommsSent {
            peer: "ghost".to_string(),
            size: 0,
        });
        assert!(app.command_input.message.is_some());
    }

    #[test]
    fn test_handle_status_message() {
        let mut app = App::new();
        app.handle_component_event(ComponentEvent::StatusMessage("all systems go".to_string()));
        assert_eq!(app.command_input.message.as_deref(), Some("all systems go"));
    }

    #[test]
    fn test_command_scan_empty_args() {
        let mut app = App::new();
        app.handle_command(Command::Scan {
            args: String::new(),
        });
        assert!(app.command_input.error.is_some());
        assert!(app.command_input.error.as_ref().unwrap().contains("Usage"));
    }

    #[test]
    fn test_command_chef_empty_args() {
        let mut app = App::new();
        app.handle_command(Command::Chef {
            args: String::new(),
        });
        assert!(app.command_input.error.is_some());
        assert!(app.command_input.error.as_ref().unwrap().contains("Usage"));
    }

    #[test]
    fn test_command_send_missing_message() {
        let mut app = App::new();
        app.handle_command(Command::Send {
            args: "peer1".to_string(),
        });
        assert!(app.command_input.error.is_some());
        assert!(app.command_input.error.as_ref().unwrap().contains("Usage"));
    }

    #[test]
    fn test_command_campaign_new() {
        let mut app = App::new();
        app.handle_command(Command::Campaign {
            args: "new TestOp".to_string(),
        });
        assert_eq!(app.campaign_name.as_deref(), Some("TestOp"));
        assert!(app.command_input.message.is_some());
    }

    #[test]
    fn test_command_campaign_status() {
        let mut app = App::new();
        app.campaign_name = Some("Alpha".to_string());
        app.handle_command(Command::Campaign {
            args: "status".to_string(),
        });
        assert!(app
            .command_input
            .message
            .as_ref()
            .unwrap()
            .contains("Alpha"));
    }

    #[test]
    fn test_command_campaign_clear() {
        let mut app = App::new();
        app.campaign_name = Some("Beta".to_string());
        app.handle_command(Command::Campaign {
            args: "clear".to_string(),
        });
        assert!(app.campaign_name.is_none());
        assert!(app
            .command_input
            .message
            .as_ref()
            .unwrap()
            .contains("ended"));
    }

    #[test]
    fn test_command_campaign_end() {
        let mut app = App::new();
        app.campaign_name = Some("Gamma".to_string());
        app.handle_command(Command::Campaign {
            args: "end".to_string(),
        });
        assert!(app.campaign_name.is_none());
    }

    #[test]
    fn test_command_campaign_invalid() {
        let mut app = App::new();
        app.handle_command(Command::Campaign {
            args: "invalid".to_string(),
        });
        assert!(app.command_input.error.is_some());
    }

    #[test]
    fn test_command_campaign_new_no_name() {
        let mut app = App::new();
        app.handle_command(Command::Campaign {
            args: "new".to_string(),
        });
        // "new" without a name should fall through to the default error case
        assert!(app.command_input.error.is_some());
    }

    #[tokio::test]
    async fn test_command_scan_with_ports() {
        let mut app = App::new();
        app.command_input.activate();
        app.command_input.buffer = "scan 10.0.0.1 -p 22,80".to_string();
        app.command_input.cursor = app.command_input.buffer.len();

        app.on_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(app.panels.focused, PanelId::Recon);
        assert!(app.scan_state.active);
        assert!(app.command_input.message.is_some());
    }

    #[tokio::test]
    async fn test_command_send_with_data() {
        let mut app = App::new();
        app.command_input.activate();
        app.command_input.buffer = "send alpha hello world".to_string();
        app.command_input.cursor = app.command_input.buffer.len();

        app.on_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(app.panels.focused, PanelId::Comms);
        assert_eq!(app.comms_panel.transfers.len(), 1);
        assert_eq!(app.comms_panel.transfers[0].name, "msg-to-alpha");
    }

    #[tokio::test]
    async fn test_command_chef_with_input() {
        let mut app = App::new();
        app.command_input.activate();
        app.command_input.buffer = "chef base64_decode SGVsbG8=".to_string();
        app.command_input.cursor = app.command_input.buffer.len();

        app.on_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(app.panels.focused, PanelId::Analysis);
        assert_eq!(
            app.analysis_panel.current_recipe.as_deref(),
            Some("base64_decode")
        );
        assert_eq!(
            app.analysis_panel.status,
            crate::panels::analysis::AnalysisStatus::Running
        );
    }
}
