//! Analysis panel implementation for CyberChef-MCP display.
//!
//! Shows recipe execution status, operation output preview,
//! and available operations listing.

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::keybindings::AppAction;
use crate::panels::Panel;
use crate::theme::Theme;

/// State for the Analysis panel.
#[derive(Debug, Clone)]
pub struct AnalysisPanel {
    /// Panel title
    title: String,
    /// Current recipe name (if running)
    pub current_recipe: Option<String>,
    /// Recipe execution status
    pub status: AnalysisStatus,
    /// Output preview text
    pub output_preview: Vec<String>,
    /// Scroll position in the output
    pub scroll_offset: usize,
}

/// Status of the analysis operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnalysisStatus {
    /// Idle, no operation running
    Idle,
    /// Operation is running
    Running,
    /// Operation completed successfully
    Complete,
    /// Operation failed
    Error(String),
}

impl AnalysisPanel {
    /// Create a new analysis panel.
    pub fn new() -> Self {
        Self {
            title: "Analysis [CyberChef]".to_string(),
            current_recipe: None,
            status: AnalysisStatus::Idle,
            output_preview: Vec::new(),
            scroll_offset: 0,
        }
    }

    /// Set the current recipe being executed.
    pub fn set_recipe(&mut self, recipe: &str) {
        self.current_recipe = Some(recipe.to_string());
        self.status = AnalysisStatus::Running;
        self.output_preview.clear();
        self.scroll_offset = 0;
    }

    /// Mark the current operation as complete with output.
    pub fn set_complete(&mut self, output: Vec<String>) {
        self.status = AnalysisStatus::Complete;
        self.output_preview = output;
        self.scroll_offset = 0;
    }

    /// Mark the current operation as failed.
    pub fn set_error(&mut self, error: &str) {
        self.status = AnalysisStatus::Error(error.to_string());
    }

    /// Clear the panel state.
    pub fn clear(&mut self) {
        self.current_recipe = None;
        self.status = AnalysisStatus::Idle;
        self.output_preview.clear();
        self.scroll_offset = 0;
    }

    /// Scroll down in the output preview.
    pub const fn scroll_down(&mut self) {
        if self.scroll_offset < self.output_preview.len().saturating_sub(1) {
            self.scroll_offset += 1;
        }
    }

    /// Scroll up in the output preview.
    pub const fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }
}

impl Default for AnalysisPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl Panel for AnalysisPanel {
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

        if inner.height < 2 || inner.width < 10 {
            return;
        }

        let mut lines: Vec<Line> = Vec::new();

        match &self.status {
            AnalysisStatus::Idle => {
                lines.push(Line::from(Span::styled(
                    "No active analysis",
                    theme.muted_style(),
                )));
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    "Use :chef <recipe> to start",
                    theme.muted_style(),
                )));
                lines.push(Line::from(Span::styled(
                    "  e.g. :chef base64_decode",
                    theme.muted_style(),
                )));
            },
            AnalysisStatus::Running => {
                let recipe_name = self.current_recipe.as_deref().unwrap_or("unknown");
                lines.push(Line::from(vec![
                    Span::styled("Recipe: ", theme.muted_style()),
                    Span::styled(recipe_name, Style::default().fg(theme.accent)),
                ]));
                lines.push(Line::from(Span::styled(
                    "Status: running...",
                    theme.warning_style(),
                )));
            },
            AnalysisStatus::Complete => {
                let recipe_name = self.current_recipe.as_deref().unwrap_or("unknown");
                lines.push(Line::from(vec![
                    Span::styled("Recipe: ", theme.muted_style()),
                    Span::styled(recipe_name, Style::default().fg(theme.accent)),
                    Span::raw("  "),
                    Span::styled("COMPLETE", theme.success_style()),
                ]));
                lines.push(Line::from(""));

                // Output preview with scroll
                for line in self.output_preview.iter().skip(self.scroll_offset) {
                    lines.push(Line::from(Span::styled(
                        line.as_str(),
                        Style::default().fg(theme.fg),
                    )));
                }
            },
            AnalysisStatus::Error(err) => {
                let recipe_name = self.current_recipe.as_deref().unwrap_or("unknown");
                lines.push(Line::from(vec![
                    Span::styled("Recipe: ", theme.muted_style()),
                    Span::styled(recipe_name, Style::default().fg(theme.accent)),
                    Span::raw("  "),
                    Span::styled("FAILED", theme.error_style()),
                ]));
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(err.as_str(), theme.error_style())));
            },
        }

        let paragraph = Paragraph::new(lines).style(theme.default_style());
        frame.render_widget(paragraph, inner);
    }

    fn on_key(&mut self, key: KeyEvent) -> Option<AppAction> {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                self.scroll_down();
                None
            },
            KeyCode::Char('k') | KeyCode::Up => {
                self.scroll_up();
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
        let backend = TestBackend::new(80, 30);
        Terminal::new(backend).unwrap()
    }

    #[test]
    fn test_analysis_panel_new() {
        let panel = AnalysisPanel::new();
        assert_eq!(panel.title(), "Analysis [CyberChef]");
        assert_eq!(panel.status, AnalysisStatus::Idle);
        assert!(panel.current_recipe.is_none());
    }

    #[test]
    fn test_analysis_panel_set_recipe() {
        let mut panel = AnalysisPanel::new();
        panel.set_recipe("base64_decode");
        assert_eq!(panel.current_recipe.as_deref(), Some("base64_decode"));
        assert_eq!(panel.status, AnalysisStatus::Running);
    }

    #[test]
    fn test_analysis_panel_set_complete() {
        let mut panel = AnalysisPanel::new();
        panel.set_recipe("base64_decode");
        panel.set_complete(vec![
            "decoded line 1".to_string(),
            "decoded line 2".to_string(),
        ]);
        assert_eq!(panel.status, AnalysisStatus::Complete);
        assert_eq!(panel.output_preview.len(), 2);
    }

    #[test]
    fn test_analysis_panel_set_error() {
        let mut panel = AnalysisPanel::new();
        panel.set_recipe("bad_recipe");
        panel.set_error("Recipe not found");
        assert!(matches!(panel.status, AnalysisStatus::Error(ref e) if e == "Recipe not found"));
    }

    #[test]
    fn test_analysis_panel_clear() {
        let mut panel = AnalysisPanel::new();
        panel.set_recipe("test");
        panel.clear();
        assert_eq!(panel.status, AnalysisStatus::Idle);
        assert!(panel.current_recipe.is_none());
    }

    #[test]
    fn test_analysis_panel_scroll() {
        let mut panel = AnalysisPanel::new();
        panel.set_recipe("test");
        panel.set_complete(vec!["a".to_string(), "b".to_string(), "c".to_string()]);

        assert_eq!(panel.scroll_offset, 0);
        panel.scroll_down();
        assert_eq!(panel.scroll_offset, 1);
        panel.scroll_down();
        assert_eq!(panel.scroll_offset, 2);
        panel.scroll_down(); // at end, should not go beyond
        assert_eq!(panel.scroll_offset, 2);
        panel.scroll_up();
        assert_eq!(panel.scroll_offset, 1);
        panel.scroll_up();
        assert_eq!(panel.scroll_offset, 0);
        panel.scroll_up(); // at top, should not underflow
        assert_eq!(panel.scroll_offset, 0);
    }

    #[test]
    fn test_analysis_panel_render_idle() {
        let mut terminal = test_terminal();
        let panel = AnalysisPanel::new();
        let theme = Theme::dark();

        terminal
            .draw(|frame| {
                let area = frame.area();
                panel.render(frame, area, true, &theme);
            })
            .unwrap();
    }

    #[test]
    fn test_analysis_panel_render_running() {
        let mut terminal = test_terminal();
        let mut panel = AnalysisPanel::new();
        panel.set_recipe("base64_decode");
        let theme = Theme::dark();

        terminal
            .draw(|frame| {
                let area = frame.area();
                panel.render(frame, area, false, &theme);
            })
            .unwrap();
    }

    #[test]
    fn test_analysis_panel_render_complete() {
        let mut terminal = test_terminal();
        let mut panel = AnalysisPanel::new();
        panel.set_recipe("test");
        panel.set_complete(vec!["output line".to_string()]);
        let theme = Theme::dark();

        terminal
            .draw(|frame| {
                let area = frame.area();
                panel.render(frame, area, true, &theme);
            })
            .unwrap();
    }

    #[test]
    fn test_analysis_panel_render_error() {
        let mut terminal = test_terminal();
        let mut panel = AnalysisPanel::new();
        panel.set_recipe("bad");
        panel.set_error("Something went wrong");
        let theme = Theme::dark();

        terminal
            .draw(|frame| {
                let area = frame.area();
                panel.render(frame, area, true, &theme);
            })
            .unwrap();
    }

    #[test]
    fn test_analysis_panel_on_key_nav() {
        let mut panel = AnalysisPanel::new();
        panel.set_recipe("test");
        panel.set_complete(vec!["a".to_string(), "b".to_string()]);

        let down = KeyEvent::new(KeyCode::Char('j'), crossterm::event::KeyModifiers::NONE);
        assert!(panel.on_key(down).is_none());
        assert_eq!(panel.scroll_offset, 1);

        let up = KeyEvent::new(KeyCode::Char('k'), crossterm::event::KeyModifiers::NONE);
        assert!(panel.on_key(up).is_none());
        assert_eq!(panel.scroll_offset, 0);
    }

    #[test]
    fn test_analysis_panel_on_key_unbound() {
        let mut panel = AnalysisPanel::new();
        let key = KeyEvent::new(KeyCode::Char('x'), crossterm::event::KeyModifiers::NONE);
        assert!(panel.on_key(key).is_none());
    }

    #[test]
    fn test_analysis_panel_default() {
        let panel = AnalysisPanel::default();
        assert_eq!(panel.status, AnalysisStatus::Idle);
    }

    #[test]
    fn test_analysis_panel_small_area() {
        let backend = TestBackend::new(8, 2);
        let mut terminal = Terminal::new(backend).unwrap();
        let panel = AnalysisPanel::new();
        let theme = Theme::dark();

        terminal
            .draw(|frame| {
                let area = frame.area();
                panel.render(frame, area, true, &theme);
            })
            .unwrap();
    }
}
