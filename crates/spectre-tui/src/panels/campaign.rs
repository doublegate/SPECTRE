//! Campaign panel implementation for campaign lifecycle display.
//!
//! Shows campaign phase progress, timeline, objectives, and artifacts.

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use spectre_core::campaign::CampaignPhase;

use crate::keybindings::AppAction;
use crate::panels::Panel;
use crate::theme::Theme;

/// State for the Campaign panel.
#[derive(Debug, Clone)]
pub struct CampaignPanel {
    /// Panel title
    title: String,
    /// Active campaign info
    pub campaign: Option<CampaignInfo>,
    /// Scroll position for notes/details
    pub scroll_offset: usize,
}

/// Summary info about the active campaign.
#[derive(Debug, Clone)]
pub struct CampaignInfo {
    /// Campaign name/codename
    pub name: String,
    /// Campaign description
    pub description: String,
    /// Current phase
    pub phase: CampaignPhase,
    /// Number of objectives
    pub objective_count: usize,
    /// Number of completed objectives
    pub objectives_complete: usize,
    /// Number of targets
    pub target_count: usize,
    /// Number of artifacts collected
    pub artifact_count: usize,
    /// Campaign tags
    pub tags: Vec<String>,
    /// Started timestamp string
    pub started: Option<String>,
    /// Elapsed time string
    pub elapsed: Option<String>,
}

impl CampaignPanel {
    /// Create a new campaign panel.
    pub fn new() -> Self {
        Self {
            title: "Campaign".to_string(),
            campaign: None,
            scroll_offset: 0,
        }
    }

    /// Set the active campaign info.
    pub fn set_campaign(&mut self, info: CampaignInfo) {
        self.campaign = Some(info);
        self.scroll_offset = 0;
    }

    /// Clear campaign state.
    pub fn clear(&mut self) {
        self.campaign = None;
        self.scroll_offset = 0;
    }

    /// Render the phase progress timeline.
    fn render_phase_timeline(
        &self,
        frame: &mut Frame,
        area: Rect,
        theme: &Theme,
        current_phase: CampaignPhase,
    ) {
        let phases = [
            ("PLAN", CampaignPhase::Planning),
            ("RECON", CampaignPhase::Recon),
            ("ANALYSIS", CampaignPhase::Analysis),
            ("EXPLOIT", CampaignPhase::Exploitation),
            ("REPORT", CampaignPhase::Reporting),
            ("DONE", CampaignPhase::Complete),
        ];

        let mut spans = Vec::new();
        let mut past_current = false;

        for (i, (label, phase)) in phases.iter().enumerate() {
            if i > 0 {
                let connector_style = if past_current {
                    Style::default().fg(theme.muted)
                } else {
                    Style::default().fg(theme.success)
                };
                spans.push(Span::styled(" -> ", connector_style));
            }

            let is_current = *phase == current_phase;
            let style = if is_current {
                past_current = true;
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD)
            } else if past_current {
                Style::default().fg(theme.muted)
            } else {
                Style::default().fg(theme.success)
            };

            let display = if is_current {
                format!("[{}]", label)
            } else {
                label.to_string()
            };

            spans.push(Span::styled(display, style));
        }

        let line = Line::from(spans);
        let paragraph = Paragraph::new(vec![line]).style(theme.default_style());
        frame.render_widget(paragraph, area);
    }
}

impl Default for CampaignPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl Panel for CampaignPanel {
    fn render(&self, frame: &mut Frame, area: Rect, focused: bool, theme: &Theme) {
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

        match &self.campaign {
            None => {
                let lines = vec![
                    Line::from(Span::styled("No active campaign", theme.muted_style())),
                    Line::from(""),
                    Line::from(Span::styled(
                        "Use :campaign new <name> to create",
                        theme.muted_style(),
                    )),
                ];
                let paragraph = Paragraph::new(lines).style(theme.default_style());
                frame.render_widget(paragraph, inner);
            },
            Some(info) => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(1), // Phase timeline
                        Constraint::Length(1), // Blank
                        Constraint::Min(3),    // Details
                    ])
                    .split(inner);

                self.render_phase_timeline(frame, chunks[0], theme, info.phase);

                let mut lines = Vec::new();

                lines.push(Line::from(vec![
                    Span::styled("Campaign: ", theme.muted_style()),
                    Span::styled(
                        info.name.as_str(),
                        Style::default()
                            .fg(theme.accent)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]));

                if !info.description.is_empty() {
                    lines.push(Line::from(Span::styled(
                        info.description.as_str(),
                        theme.muted_style(),
                    )));
                }

                lines.push(Line::from(""));

                lines.push(Line::from(vec![
                    Span::styled("Objectives: ", theme.muted_style()),
                    Span::styled(
                        format!("{}/{}", info.objectives_complete, info.objective_count),
                        Style::default().fg(theme.fg),
                    ),
                    Span::raw("  "),
                    Span::styled("Targets: ", theme.muted_style()),
                    Span::styled(info.target_count.to_string(), Style::default().fg(theme.fg)),
                    Span::raw("  "),
                    Span::styled("Artifacts: ", theme.muted_style()),
                    Span::styled(
                        info.artifact_count.to_string(),
                        Style::default().fg(theme.fg),
                    ),
                ]));

                if let Some(ref started) = info.started {
                    lines.push(Line::from(vec![
                        Span::styled("Started: ", theme.muted_style()),
                        Span::styled(started.as_str(), Style::default().fg(theme.fg)),
                        if let Some(ref elapsed) = info.elapsed {
                            Span::styled(
                                format!("  ({})", elapsed),
                                Style::default().fg(theme.muted),
                            )
                        } else {
                            Span::raw("")
                        },
                    ]));
                }

                if !info.tags.is_empty() {
                    lines.push(Line::from(vec![
                        Span::styled("Tags: ", theme.muted_style()),
                        Span::styled(info.tags.join(", "), Style::default().fg(theme.secondary)),
                    ]));
                }

                // Apply scroll offset
                let display_lines: Vec<Line> = lines.into_iter().skip(self.scroll_offset).collect();

                let paragraph = Paragraph::new(display_lines).style(theme.default_style());
                frame.render_widget(paragraph, chunks[2]);
            },
        }
    }

    fn on_key(&mut self, key: KeyEvent) -> Option<AppAction> {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                self.scroll_offset += 1;
                None
            },
            KeyCode::Char('k') | KeyCode::Up => {
                self.scroll_offset = self.scroll_offset.saturating_sub(1);
                None
            },
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

    fn sample_campaign() -> CampaignInfo {
        CampaignInfo {
            name: "Operation NIGHTFALL".to_string(),
            description: "Internal network assessment".to_string(),
            phase: CampaignPhase::Recon,
            objective_count: 5,
            objectives_complete: 2,
            target_count: 3,
            artifact_count: 7,
            tags: vec!["internal".to_string(), "red-team".to_string()],
            started: Some("2025-01-15 09:00".to_string()),
            elapsed: Some("2h 30m".to_string()),
        }
    }

    #[test]
    fn test_campaign_panel_new() {
        let panel = CampaignPanel::new();
        assert_eq!(panel.title(), "Campaign");
        assert!(panel.campaign.is_none());
    }

    #[test]
    fn test_campaign_panel_set_campaign() {
        let mut panel = CampaignPanel::new();
        panel.set_campaign(sample_campaign());
        assert!(panel.campaign.is_some());
        assert_eq!(panel.campaign.as_ref().unwrap().name, "Operation NIGHTFALL");
    }

    #[test]
    fn test_campaign_panel_clear() {
        let mut panel = CampaignPanel::new();
        panel.set_campaign(sample_campaign());
        panel.clear();
        assert!(panel.campaign.is_none());
    }

    #[test]
    fn test_campaign_panel_render_empty() {
        let mut terminal = test_terminal();
        let panel = CampaignPanel::new();
        let theme = Theme::dark();

        terminal
            .draw(|frame| {
                let area = frame.area();
                panel.render(frame, area, true, &theme);
            })
            .unwrap();
    }

    #[test]
    fn test_campaign_panel_render_with_campaign() {
        let mut terminal = test_terminal();
        let mut panel = CampaignPanel::new();
        panel.set_campaign(sample_campaign());
        let theme = Theme::dark();

        terminal
            .draw(|frame| {
                let area = frame.area();
                panel.render(frame, area, true, &theme);
            })
            .unwrap();
    }

    #[test]
    fn test_campaign_panel_render_different_phases() {
        let mut terminal = test_terminal();
        let theme = Theme::dark();

        for phase in CampaignPhase::all() {
            let mut panel = CampaignPanel::new();
            let mut info = sample_campaign();
            info.phase = phase;
            panel.set_campaign(info);

            terminal
                .draw(|frame| {
                    let area = frame.area();
                    panel.render(frame, area, true, &theme);
                })
                .unwrap();
        }
    }

    #[test]
    fn test_campaign_panel_scroll() {
        let mut panel = CampaignPanel::new();
        panel.set_campaign(sample_campaign());

        let down = KeyEvent::new(KeyCode::Char('j'), crossterm::event::KeyModifiers::NONE);
        panel.on_key(down);
        assert_eq!(panel.scroll_offset, 1);

        let up = KeyEvent::new(KeyCode::Char('k'), crossterm::event::KeyModifiers::NONE);
        panel.on_key(up);
        assert_eq!(panel.scroll_offset, 0);

        panel.on_key(up); // should not underflow
        assert_eq!(panel.scroll_offset, 0);
    }

    #[test]
    fn test_campaign_panel_on_key_unbound() {
        let mut panel = CampaignPanel::new();
        let key = KeyEvent::new(KeyCode::Char('x'), crossterm::event::KeyModifiers::NONE);
        assert!(panel.on_key(key).is_none());
    }

    #[test]
    fn test_campaign_panel_default() {
        let panel = CampaignPanel::default();
        assert!(panel.campaign.is_none());
    }

    #[test]
    fn test_campaign_panel_small_area() {
        let backend = TestBackend::new(8, 2);
        let mut terminal = Terminal::new(backend).unwrap();
        let panel = CampaignPanel::new();
        let theme = Theme::dark();

        terminal
            .draw(|frame| {
                let area = frame.area();
                panel.render(frame, area, true, &theme);
            })
            .unwrap();
    }

    #[test]
    fn test_campaign_info_no_started() {
        let mut terminal = test_terminal();
        let mut panel = CampaignPanel::new();
        let mut info = sample_campaign();
        info.started = None;
        info.elapsed = None;
        info.tags.clear();
        info.description.clear();
        panel.set_campaign(info);
        let theme = Theme::dark();

        terminal
            .draw(|frame| {
                let area = frame.area();
                panel.render(frame, area, true, &theme);
            })
            .unwrap();
    }
}
