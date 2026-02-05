//! Recon panel implementation for network scanning display.
//!
//! Shows scan progress, rate metrics, host/port result tables,
//! and provides scrollable result browsing with search and sort.

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Gauge, Paragraph, Row, Table};
use ratatui::Frame;

use crate::keybindings::AppAction;
use crate::panels::Panel;
use crate::scan_state::ScanState;
use crate::theme::Theme;

/// Recon panel state (references external scan state from App).
#[derive(Debug, Clone, Default)]
pub struct ReconPanel {
    /// Title override
    title: String,
}

impl ReconPanel {
    /// Create a new recon panel.
    pub fn new() -> Self {
        Self {
            title: "Recon [ProRT-IP]".to_string(),
        }
    }

    /// Render the recon panel with scan state data.
    pub fn render_with_state(
        &self,
        frame: &mut Frame,
        area: Rect,
        focused: bool,
        theme: &Theme,
        scan: &ScanState,
    ) {
        let border_style = if focused {
            theme.border_focused_style()
        } else {
            theme.border_style()
        };

        let block = Block::default()
            .title(format!(" {} ", self.title))
            .borders(Borders::ALL)
            .border_style(border_style)
            .style(theme.default_style());

        let inner = block.inner(area);
        frame.render_widget(block, area);

        if inner.height < 3 || inner.width < 10 {
            return;
        }

        if scan.active || !scan.recent_results.is_empty() {
            self.render_active_scan(frame, inner, theme, scan);
        } else {
            self.render_idle(frame, inner, theme);
        }
    }

    /// Render idle state (no active scan).
    fn render_idle(&self, frame: &mut Frame, area: Rect, theme: &Theme) {
        let lines = vec![
            Line::from(Span::styled("No active scan", theme.muted_style())),
            Line::from(""),
            Line::from(Span::styled(
                "Use :scan <target> to start",
                theme.muted_style(),
            )),
            Line::from(Span::styled(
                "  e.g. :scan 192.168.1.0/24 -p 1-1000",
                theme.muted_style(),
            )),
        ];

        let paragraph = Paragraph::new(lines).style(theme.default_style());
        frame.render_widget(paragraph, area);
    }

    /// Render active scan with progress and results.
    fn render_active_scan(&self, frame: &mut Frame, area: Rect, theme: &Theme, scan: &ScanState) {
        // Split into: metrics (3 lines), progress bar (1 line), results table (rest)
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Min(3),
            ])
            .split(area);

        self.render_metrics(frame, chunks[0], theme, scan);
        self.render_progress_bar(frame, chunks[1], theme, scan);
        self.render_results_table(frame, chunks[2], theme, scan);
    }

    /// Render scan metric summary.
    fn render_metrics(&self, frame: &mut Frame, area: Rect, theme: &Theme, scan: &ScanState) {
        let scan_type_str = format!("{:?}", scan.scan_type);
        let lines = vec![
            Line::from(vec![
                Span::styled("Target: ", theme.muted_style()),
                Span::styled(&scan.target, Style::default().fg(theme.accent)),
                Span::raw("  "),
                Span::styled("Type: ", theme.muted_style()),
                Span::styled(scan_type_str, Style::default().fg(theme.primary)),
            ]),
            Line::from(vec![
                Span::styled("Hosts: ", theme.muted_style()),
                Span::styled(
                    format!("{}/{}", scan.hosts_scanned, scan.hosts_total),
                    Style::default().fg(theme.fg),
                ),
                Span::raw("  "),
                Span::styled("Open: ", theme.muted_style()),
                Span::styled(
                    scan.open_ports.to_string(),
                    Style::default().fg(theme.success),
                ),
                Span::raw("  "),
                Span::styled("Svcs: ", theme.muted_style()),
                Span::styled(
                    scan.services_found.to_string(),
                    Style::default().fg(theme.secondary),
                ),
            ]),
            Line::from(vec![
                Span::styled("Rate: ", theme.muted_style()),
                Span::styled(
                    format!("{} pps", scan.rate_pps),
                    Style::default().fg(theme.fg),
                ),
                Span::raw("  "),
                Span::styled("Elapsed: ", theme.muted_style()),
                Span::styled(scan.elapsed_string(), Style::default().fg(theme.fg)),
                Span::raw("  "),
                Span::styled("ETA: ", theme.muted_style()),
                Span::styled(scan.eta_string(), Style::default().fg(theme.warning)),
            ]),
        ];

        let paragraph = Paragraph::new(lines).style(theme.default_style());
        frame.render_widget(paragraph, area);
    }

    /// Render the progress bar.
    fn render_progress_bar(&self, frame: &mut Frame, area: Rect, theme: &Theme, scan: &ScanState) {
        let label = format!("{}%", scan.progress_percent());
        let gauge = Gauge::default()
            .gauge_style(
                Style::default()
                    .fg(theme.primary)
                    .bg(theme.bg)
                    .add_modifier(Modifier::BOLD),
            )
            .label(label)
            .ratio(scan.progress.min(1.0));

        frame.render_widget(gauge, area);
    }

    /// Render the results table.
    fn render_results_table(&self, frame: &mut Frame, area: Rect, theme: &Theme, scan: &ScanState) {
        let header_cells = ["Host", "Port", "State", "Proto", "Service", "Version"]
            .iter()
            .map(|h| {
                Cell::from(*h).style(
                    Style::default()
                        .fg(theme.primary)
                        .add_modifier(Modifier::BOLD),
                )
            });
        let header = Row::new(header_cells).height(1);

        let filtered = scan.filtered_results();
        let rows: Vec<Row> = filtered
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let style = if i == scan.selected_index {
                    theme.highlight_style()
                } else {
                    theme.default_style()
                };

                let state_style = match entry.state.as_str() {
                    "Open" => Style::default().fg(theme.success),
                    "Filtered" | "OpenFiltered" => Style::default().fg(theme.warning),
                    "Closed" => Style::default().fg(theme.error),
                    _ => Style::default().fg(theme.muted),
                };

                Row::new(vec![
                    Cell::from(entry.host.as_str()),
                    Cell::from(entry.port.to_string()),
                    Cell::from(Span::styled(entry.state.as_str(), state_style)),
                    Cell::from(entry.protocol.as_str()),
                    Cell::from(entry.service.as_deref().unwrap_or("-")),
                    Cell::from(entry.version.as_deref().unwrap_or("-")),
                ])
                .style(style)
            })
            .collect();

        let table = Table::new(
            rows,
            [
                Constraint::Min(12),
                Constraint::Length(6),
                Constraint::Length(10),
                Constraint::Length(5),
                Constraint::Min(10),
                Constraint::Min(12),
            ],
        )
        .header(header)
        .style(theme.default_style());

        frame.render_widget(table, area);
    }
}

impl Panel for ReconPanel {
    fn render(&self, frame: &mut Frame, area: Rect, focused: bool, theme: &Theme) {
        // Render with empty scan state when called through the trait
        let empty_scan = ScanState::new();
        self.render_with_state(frame, area, focused, theme, &empty_scan);
    }

    fn on_key(&mut self, key: KeyEvent) -> Option<AppAction> {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => Some(AppAction::NavigateDown),
            KeyCode::Char('k') | KeyCode::Up => Some(AppAction::NavigateUp),
            KeyCode::Char('g') => Some(AppAction::JumpToTop),
            KeyCode::Char('G') => Some(AppAction::JumpToBottom),
            _ => None,
        }
    }

    fn title(&self) -> &str {
        &self.title
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    fn test_terminal() -> Terminal<TestBackend> {
        let backend = TestBackend::new(100, 40);
        Terminal::new(backend).unwrap()
    }

    #[test]
    fn test_recon_panel_new() {
        let panel = ReconPanel::new();
        assert_eq!(panel.title(), "Recon [ProRT-IP]");
    }

    #[test]
    fn test_recon_panel_render_idle() {
        let mut terminal = test_terminal();
        let panel = ReconPanel::new();
        let theme = Theme::dark();

        terminal
            .draw(|frame| {
                let area = frame.area();
                panel.render(frame, area, true, &theme);
            })
            .unwrap();
    }

    #[test]
    fn test_recon_panel_render_with_scan() {
        let mut terminal = test_terminal();
        let panel = ReconPanel::new();
        let theme = Theme::dark();

        let mut scan = ScanState::new();
        scan.start(
            "192.168.1.0/24",
            spectre_core::scan::ScanType::Syn,
            256,
            "job-1",
        );
        scan.update_progress(128, 256);

        terminal
            .draw(|frame| {
                let area = frame.area();
                panel.render_with_state(frame, area, true, &theme, &scan);
            })
            .unwrap();
    }

    #[test]
    fn test_recon_panel_render_with_results() {
        let mut terminal = test_terminal();
        let panel = ReconPanel::new();
        let theme = Theme::dark();

        let mut scan = ScanState::new();
        scan.start("10.0.0.1", spectre_core::scan::ScanType::Connect, 1, "j1");
        scan.add_host_results(
            "10.0.0.1",
            &[spectre_core::scan::PortResult {
                port: 22,
                protocol: spectre_core::scan::Protocol::Tcp,
                state: spectre_core::scan::PortState::Open,
                service: Some("ssh".to_string()),
                version: Some("OpenSSH 8.9".to_string()),
                banner: None,
            }],
        );

        terminal
            .draw(|frame| {
                let area = frame.area();
                panel.render_with_state(frame, area, false, &theme, &scan);
            })
            .unwrap();
    }

    #[test]
    fn test_recon_panel_on_key_nav_down() {
        let mut panel = ReconPanel::new();
        let key = KeyEvent::new(KeyCode::Char('j'), crossterm::event::KeyModifiers::NONE);
        let action = panel.on_key(key);
        assert_eq!(action, Some(AppAction::NavigateDown));
    }

    #[test]
    fn test_recon_panel_on_key_nav_up() {
        let mut panel = ReconPanel::new();
        let key = KeyEvent::new(KeyCode::Char('k'), crossterm::event::KeyModifiers::NONE);
        let action = panel.on_key(key);
        assert_eq!(action, Some(AppAction::NavigateUp));
    }

    #[test]
    fn test_recon_panel_on_key_jump_top() {
        let mut panel = ReconPanel::new();
        let key = KeyEvent::new(KeyCode::Char('g'), crossterm::event::KeyModifiers::NONE);
        let action = panel.on_key(key);
        assert_eq!(action, Some(AppAction::JumpToTop));
    }

    #[test]
    fn test_recon_panel_on_key_jump_bottom() {
        let mut panel = ReconPanel::new();
        let key = KeyEvent::new(KeyCode::Char('G'), crossterm::event::KeyModifiers::NONE);
        let action = panel.on_key(key);
        assert_eq!(action, Some(AppAction::JumpToBottom));
    }

    #[test]
    fn test_recon_panel_on_key_unbound() {
        let mut panel = ReconPanel::new();
        let key = KeyEvent::new(KeyCode::Char('x'), crossterm::event::KeyModifiers::NONE);
        let action = panel.on_key(key);
        assert!(action.is_none());
    }

    #[test]
    fn test_recon_panel_small_area() {
        let backend = TestBackend::new(8, 2);
        let mut terminal = Terminal::new(backend).unwrap();
        let panel = ReconPanel::new();
        let theme = Theme::dark();

        // Should not panic with very small area
        terminal
            .draw(|frame| {
                let area = frame.area();
                panel.render(frame, area, true, &theme);
            })
            .unwrap();
    }

    #[test]
    fn test_recon_panel_render_focused_vs_unfocused() {
        let mut terminal = test_terminal();
        let panel = ReconPanel::new();
        let theme = Theme::dark();

        // Focused
        terminal
            .draw(|frame| {
                let area = frame.area();
                panel.render(frame, area, true, &theme);
            })
            .unwrap();

        // Unfocused
        terminal
            .draw(|frame| {
                let area = frame.area();
                panel.render(frame, area, false, &theme);
            })
            .unwrap();
    }
}
