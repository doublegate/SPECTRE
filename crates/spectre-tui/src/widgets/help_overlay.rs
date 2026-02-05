//! Help overlay popup showing keyboard shortcuts.
//!
//! Renders a centered modal with all available keybindings grouped by category.

use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};
use ratatui::Frame;

use crate::keybindings::{all_keybindings, KeyCategory};
use crate::theme::Theme;

/// Render the help overlay centered in the given area.
pub fn render_help_overlay(frame: &mut Frame, area: Rect, theme: &Theme) {
    let popup_area = centered_rect(60, 80, area);

    // Clear the area behind the popup
    frame.render_widget(Clear, popup_area);

    let block = Block::default()
        .title(" Keyboard Shortcuts (? to close) ")
        .borders(Borders::ALL)
        .border_style(
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().fg(theme.fg).bg(theme.bg));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    let bindings = all_keybindings();
    let mut lines: Vec<Line> = Vec::new();

    let categories = [
        KeyCategory::Global,
        KeyCategory::PanelFocus,
        KeyCategory::Navigation,
        KeyCategory::CommandMode,
    ];

    for (i, category) in categories.iter().enumerate() {
        if i > 0 {
            lines.push(Line::from(""));
        }

        lines.push(Line::from(Span::styled(
            format!("-- {} --", category),
            Style::default()
                .fg(theme.primary)
                .add_modifier(Modifier::BOLD),
        )));

        for binding in &bindings {
            if binding.category == *category {
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("{:20}", binding.keys),
                        Style::default()
                            .fg(theme.accent)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(binding.description, Style::default().fg(theme.fg)),
                ]));
            }
        }
    }

    let paragraph = Paragraph::new(lines)
        .style(Style::default().fg(theme.fg).bg(theme.bg))
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, inner);
}

/// Compute a centered rectangle within the given area.
///
/// `percent_x` and `percent_y` specify the size as a percentage of the area.
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_width = area.width * percent_x / 100;
    let popup_height = area.height * percent_y / 100;

    let x = area.x + (area.width.saturating_sub(popup_width)) / 2;
    let y = area.y + (area.height.saturating_sub(popup_height)) / 2;

    // Ensure minimum size
    let width = popup_width.max(20).min(area.width);
    let height = popup_height.max(10).min(area.height);

    Rect::new(x, y, width, height)
}

/// Get the number of lines in the help overlay content.
///
/// Used for layout calculations.
pub fn help_content_lines() -> usize {
    let bindings = all_keybindings();
    let categories = [
        KeyCategory::Global,
        KeyCategory::PanelFocus,
        KeyCategory::Navigation,
        KeyCategory::CommandMode,
    ];

    let mut count = 0;
    for (i, category) in categories.iter().enumerate() {
        if i > 0 {
            count += 1; // blank line
        }
        count += 1; // category header
        count += bindings.iter().filter(|b| b.category == *category).count();
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    #[test]
    fn test_render_help_overlay() {
        let backend = TestBackend::new(120, 50);
        let mut terminal = Terminal::new(backend).unwrap();
        let theme = Theme::dark();

        terminal
            .draw(|frame| {
                let area = frame.area();
                render_help_overlay(frame, area, &theme);
            })
            .unwrap();
    }

    #[test]
    fn test_centered_rect() {
        let area = Rect::new(0, 0, 100, 50);
        let rect = centered_rect(50, 50, area);

        // Should be centered
        assert!(rect.x > 0);
        assert!(rect.y > 0);
        assert!(rect.width <= 100);
        assert!(rect.height <= 50);
    }

    #[test]
    fn test_centered_rect_small_area() {
        let area = Rect::new(0, 0, 15, 8);
        let rect = centered_rect(60, 80, area);

        // Should enforce minimum sizes but not exceed area
        assert!(rect.width <= area.width);
        assert!(rect.height <= area.height);
    }

    #[test]
    fn test_help_content_lines() {
        let count = help_content_lines();
        assert!(count > 20);
    }

    #[test]
    fn test_render_help_small_terminal() {
        let backend = TestBackend::new(30, 15);
        let mut terminal = Terminal::new(backend).unwrap();
        let theme = Theme::dark();

        terminal
            .draw(|frame| {
                let area = frame.area();
                render_help_overlay(frame, area, &theme);
            })
            .unwrap();
    }

    #[test]
    fn test_render_help_all_themes() {
        let backend = TestBackend::new(100, 40);
        let mut terminal = Terminal::new(backend).unwrap();

        for name in Theme::available_themes() {
            let theme = Theme::by_name(name);
            terminal
                .draw(|frame| {
                    let area = frame.area();
                    render_help_overlay(frame, area, &theme);
                })
                .unwrap();
        }
    }
}
