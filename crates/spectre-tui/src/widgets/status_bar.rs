//! Bottom status bar widget.
//!
//! Displays keyboard shortcut hints, active panel indicator,
//! command input, and system metrics.

use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::command::CommandInput;
use crate::layout::PanelId;
use crate::theme::Theme;

/// Render the header bar at the top of the screen.
pub fn render_header(
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    campaign_name: Option<&str>,
    frame_count: u64,
) {
    let mut spans = vec![Span::styled(
        " SPECTRE ",
        Style::default()
            .fg(theme.accent)
            .add_modifier(Modifier::BOLD),
    )];

    if let Some(name) = campaign_name {
        spans.push(Span::styled(" | ", Style::default().fg(theme.muted)));
        spans.push(Span::styled(name, Style::default().fg(theme.primary)));
    }

    // Right-aligned frame counter (for debug/performance)
    let right_text = format!("F:{} ", frame_count);
    let used_width: usize = spans.iter().map(ratatui::prelude::Span::width).sum();
    let remaining = (area.width as usize).saturating_sub(used_width + right_text.len());

    if remaining > 0 {
        spans.push(Span::raw(" ".repeat(remaining)));
    }
    spans.push(Span::styled(right_text, Style::default().fg(theme.muted)));

    let line = Line::from(spans);
    let paragraph = Paragraph::new(vec![line]).style(
        Style::default()
            .fg(theme.fg)
            .bg(theme.bg)
            .add_modifier(Modifier::BOLD),
    );
    frame.render_widget(paragraph, area);
}

/// Render the status bar at the bottom of the screen.
///
/// Shows either the command input (when in command mode) or shortcut hints.
pub fn render_status_bar(
    frame: &mut Frame,
    area: Rect,
    theme: &Theme,
    focused_panel: PanelId,
    command_input: &CommandInput,
) {
    if command_input.active {
        render_command_line(frame, area, theme, command_input);
    } else if let Some(ref err) = command_input.error {
        render_error_line(frame, area, theme, err);
    } else if let Some(ref msg) = command_input.message {
        render_message_line(frame, area, theme, msg);
    } else {
        render_shortcut_hints(frame, area, theme, focused_panel);
    }
}

/// Render the command input line.
fn render_command_line(frame: &mut Frame, area: Rect, theme: &Theme, input: &CommandInput) {
    let display = input.display_text();
    let line = Line::from(vec![Span::styled(
        display,
        Style::default()
            .fg(theme.accent)
            .add_modifier(Modifier::BOLD),
    )]);

    let paragraph = Paragraph::new(vec![line]).style(Style::default().fg(theme.fg).bg(theme.bg));
    frame.render_widget(paragraph, area);

    // Position cursor
    let cursor_x = area.x + input.display_cursor() as u16;
    let cursor_y = area.y;
    if cursor_x < area.x + area.width {
        frame.set_cursor_position((cursor_x, cursor_y));
    }
}

/// Render an error message in the status bar.
fn render_error_line(frame: &mut Frame, area: Rect, theme: &Theme, error: &str) {
    let line = Line::from(vec![
        Span::styled(" ERROR: ", theme.error_style().add_modifier(Modifier::BOLD)),
        Span::styled(error, theme.error_style()),
    ]);

    let paragraph = Paragraph::new(vec![line]).style(Style::default().fg(theme.fg).bg(theme.bg));
    frame.render_widget(paragraph, area);
}

/// Render an info message in the status bar.
fn render_message_line(frame: &mut Frame, area: Rect, theme: &Theme, message: &str) {
    let line = Line::from(Span::styled(
        format!(" {}", message),
        Style::default().fg(theme.success),
    ));

    let paragraph = Paragraph::new(vec![line]).style(Style::default().fg(theme.fg).bg(theme.bg));
    frame.render_widget(paragraph, area);
}

/// Render the default shortcut hints.
fn render_shortcut_hints(frame: &mut Frame, area: Rect, theme: &Theme, focused: PanelId) {
    let spans = vec![
        Span::styled(
            format!(" [{}] ", focused.title()),
            Style::default().fg(theme.primary),
        ),
        Span::styled("| ", Style::default().fg(theme.muted)),
        Span::styled("?: ", Style::default().fg(theme.accent)),
        Span::styled("Help ", Style::default().fg(theme.fg)),
        Span::styled(":: ", Style::default().fg(theme.accent)),
        Span::styled("Cmd ", Style::default().fg(theme.fg)),
        Span::styled("Tab: ", Style::default().fg(theme.accent)),
        Span::styled("Switch ", Style::default().fg(theme.fg)),
        Span::styled("q: ", Style::default().fg(theme.accent)),
        Span::styled("Quit", Style::default().fg(theme.fg)),
    ];

    let line = Line::from(spans);
    let paragraph = Paragraph::new(vec![line]).style(Style::default().fg(theme.fg).bg(theme.bg));
    frame.render_widget(paragraph, area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    fn test_terminal() -> Terminal<TestBackend> {
        let backend = TestBackend::new(120, 40);
        Terminal::new(backend).unwrap()
    }

    #[test]
    fn test_render_header_no_campaign() {
        let mut terminal = test_terminal();
        let theme = Theme::dark();

        terminal
            .draw(|frame| {
                let area = Rect::new(0, 0, 120, 1);
                render_header(frame, area, &theme, None, 42);
            })
            .unwrap();
    }

    #[test]
    fn test_render_header_with_campaign() {
        let mut terminal = test_terminal();
        let theme = Theme::dark();

        terminal
            .draw(|frame| {
                let area = Rect::new(0, 0, 120, 1);
                render_header(frame, area, &theme, Some("Operation NIGHTFALL"), 100);
            })
            .unwrap();
    }

    #[test]
    fn test_render_status_bar_shortcut_hints() {
        let mut terminal = test_terminal();
        let theme = Theme::dark();
        let input = CommandInput::new();

        terminal
            .draw(|frame| {
                let area = Rect::new(0, 39, 120, 1);
                render_status_bar(frame, area, &theme, PanelId::Recon, &input);
            })
            .unwrap();
    }

    #[test]
    fn test_render_status_bar_command_mode() {
        let mut terminal = test_terminal();
        let theme = Theme::dark();
        let mut input = CommandInput::new();
        input.activate();
        input.buffer = "scan 10.0.0.1".to_string();
        input.cursor = 13;

        terminal
            .draw(|frame| {
                let area = Rect::new(0, 39, 120, 1);
                render_status_bar(frame, area, &theme, PanelId::Recon, &input);
            })
            .unwrap();
    }

    #[test]
    fn test_render_status_bar_error() {
        let mut terminal = test_terminal();
        let theme = Theme::dark();
        let mut input = CommandInput::new();
        input.error = Some("Unknown command: foo".to_string());

        terminal
            .draw(|frame| {
                let area = Rect::new(0, 39, 120, 1);
                render_status_bar(frame, area, &theme, PanelId::Recon, &input);
            })
            .unwrap();
    }

    #[test]
    fn test_render_status_bar_message() {
        let mut terminal = test_terminal();
        let theme = Theme::dark();
        let mut input = CommandInput::new();
        input.message = Some("Theme changed to tactical".to_string());

        terminal
            .draw(|frame| {
                let area = Rect::new(0, 39, 120, 1);
                render_status_bar(frame, area, &theme, PanelId::Recon, &input);
            })
            .unwrap();
    }

    #[test]
    fn test_render_header_narrow_terminal() {
        let backend = TestBackend::new(30, 5);
        let mut terminal = Terminal::new(backend).unwrap();
        let theme = Theme::dark();

        terminal
            .draw(|frame| {
                let area = Rect::new(0, 0, 30, 1);
                render_header(frame, area, &theme, Some("Op"), 1);
            })
            .unwrap();
    }

    #[test]
    fn test_render_all_panels_in_status() {
        let mut terminal = test_terminal();
        let theme = Theme::dark();
        let input = CommandInput::new();

        for panel in PanelId::all() {
            terminal
                .draw(|frame| {
                    let area = Rect::new(0, 39, 120, 1);
                    render_status_bar(frame, area, &theme, *panel, &input);
                })
                .unwrap();
        }
    }
}
