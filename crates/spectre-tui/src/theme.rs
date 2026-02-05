//! Theme definitions and color schemes for the TUI dashboard.
//!
//! Provides built-in themes (dark, light, tactical, matrix, hacker)
//! and runtime theme switching via the `:theme` command.

use ratatui::style::{Color, Modifier, Style};

/// A complete color theme for the TUI dashboard.
#[derive(Debug, Clone)]
pub struct Theme {
    /// Theme name
    pub name: String,
    /// Background color
    pub bg: Color,
    /// Default foreground color
    pub fg: Color,
    /// Primary accent color
    pub primary: Color,
    /// Secondary accent color
    pub secondary: Color,
    /// Tertiary/highlight accent color
    pub accent: Color,
    /// Success indicator color
    pub success: Color,
    /// Warning indicator color
    pub warning: Color,
    /// Error indicator color
    pub error: Color,
    /// Muted/dimmed text color
    pub muted: Color,
    /// Default border color
    pub border: Color,
    /// Focused panel border color
    pub border_focused: Color,
    /// Character used for filled progress
    pub progress_filled: &'static str,
    /// Character used for empty progress
    pub progress_empty: &'static str,
}

impl Theme {
    /// Create the default dark theme.
    pub fn dark() -> Self {
        Self {
            name: "dark".to_string(),
            bg: Color::Rgb(18, 18, 28),
            fg: Color::Rgb(200, 200, 220),
            primary: Color::Rgb(100, 149, 237),
            secondary: Color::Rgb(138, 43, 226),
            accent: Color::Rgb(0, 255, 255),
            success: Color::Rgb(50, 205, 50),
            warning: Color::Rgb(255, 165, 0),
            error: Color::Rgb(220, 20, 60),
            muted: Color::Rgb(100, 100, 120),
            border: Color::Rgb(60, 60, 80),
            border_focused: Color::Rgb(100, 149, 237),
            progress_filled: "█",
            progress_empty: "░",
        }
    }

    /// Create the light theme.
    pub fn light() -> Self {
        Self {
            name: "light".to_string(),
            bg: Color::Rgb(245, 245, 250),
            fg: Color::Rgb(30, 30, 50),
            primary: Color::Rgb(25, 25, 112),
            secondary: Color::Rgb(72, 61, 139),
            accent: Color::Rgb(0, 128, 128),
            success: Color::Rgb(34, 139, 34),
            warning: Color::Rgb(204, 102, 0),
            error: Color::Rgb(178, 34, 34),
            muted: Color::Rgb(150, 150, 170),
            border: Color::Rgb(180, 180, 200),
            border_focused: Color::Rgb(25, 25, 112),
            progress_filled: "█",
            progress_empty: "░",
        }
    }

    /// Create the tactical (military) theme.
    pub fn tactical() -> Self {
        Self {
            name: "tactical".to_string(),
            bg: Color::Rgb(10, 15, 10),
            fg: Color::Rgb(180, 200, 180),
            primary: Color::Rgb(34, 139, 34),
            secondary: Color::Rgb(85, 107, 47),
            accent: Color::Rgb(107, 142, 35),
            success: Color::Rgb(0, 200, 0),
            warning: Color::Rgb(218, 165, 32),
            error: Color::Rgb(200, 50, 50),
            muted: Color::Rgb(80, 100, 80),
            border: Color::Rgb(34, 60, 34),
            border_focused: Color::Rgb(34, 139, 34),
            progress_filled: "=",
            progress_empty: "-",
        }
    }

    /// Create the matrix theme.
    pub fn matrix() -> Self {
        Self {
            name: "matrix".to_string(),
            bg: Color::Rgb(0, 0, 0),
            fg: Color::Rgb(0, 255, 0),
            primary: Color::Rgb(0, 200, 0),
            secondary: Color::Rgb(0, 150, 0),
            accent: Color::Rgb(0, 255, 128),
            success: Color::Rgb(0, 255, 0),
            warning: Color::Rgb(200, 200, 0),
            error: Color::Rgb(255, 0, 0),
            muted: Color::Rgb(0, 80, 0),
            border: Color::Rgb(0, 60, 0),
            border_focused: Color::Rgb(0, 255, 0),
            progress_filled: "#",
            progress_empty: ".",
        }
    }

    /// Create the hacker theme.
    pub fn hacker() -> Self {
        Self {
            name: "hacker".to_string(),
            bg: Color::Rgb(10, 10, 20),
            fg: Color::Rgb(0, 230, 230),
            primary: Color::Rgb(255, 0, 128),
            secondary: Color::Rgb(128, 0, 255),
            accent: Color::Rgb(0, 255, 255),
            success: Color::Rgb(0, 255, 128),
            warning: Color::Rgb(255, 128, 0),
            error: Color::Rgb(255, 0, 64),
            muted: Color::Rgb(60, 80, 80),
            border: Color::Rgb(40, 40, 60),
            border_focused: Color::Rgb(255, 0, 128),
            progress_filled: ">",
            progress_empty: " ",
        }
    }

    /// Get a theme by name. Returns the dark theme for unrecognized names.
    pub fn by_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "light" => Self::light(),
            "tactical" => Self::tactical(),
            "matrix" => Self::matrix(),
            "hacker" => Self::hacker(),
            _ => Self::dark(),
        }
    }

    /// List all available theme names.
    pub fn available_themes() -> Vec<&'static str> {
        vec!["dark", "light", "tactical", "matrix", "hacker"]
    }

    /// Build a default text style using the theme foreground/background.
    pub fn default_style(&self) -> Style {
        Style::default().fg(self.fg).bg(self.bg)
    }

    /// Build a style for panel borders (unfocused).
    pub fn border_style(&self) -> Style {
        Style::default().fg(self.border).bg(self.bg)
    }

    /// Build a style for focused panel borders.
    pub fn border_focused_style(&self) -> Style {
        Style::default()
            .fg(self.border_focused)
            .bg(self.bg)
            .add_modifier(Modifier::BOLD)
    }

    /// Build a style for the header/title bar.
    pub fn header_style(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .bg(self.bg)
            .add_modifier(Modifier::BOLD)
    }

    /// Build a style for highlighted/selected items.
    pub fn highlight_style(&self) -> Style {
        Style::default()
            .fg(self.bg)
            .bg(self.primary)
            .add_modifier(Modifier::BOLD)
    }

    /// Build a style for muted/secondary text.
    pub fn muted_style(&self) -> Style {
        Style::default().fg(self.muted).bg(self.bg)
    }

    /// Build a style for success messages.
    pub fn success_style(&self) -> Style {
        Style::default().fg(self.success).bg(self.bg)
    }

    /// Build a style for warning messages.
    pub fn warning_style(&self) -> Style {
        Style::default().fg(self.warning).bg(self.bg)
    }

    /// Build a style for error messages.
    pub fn error_style(&self) -> Style {
        Style::default().fg(self.error).bg(self.bg)
    }

    /// Build a style for the accent color.
    pub fn accent_style(&self) -> Style {
        Style::default().fg(self.accent).bg(self.bg)
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dark_theme() {
        let theme = Theme::dark();
        assert_eq!(theme.name, "dark");
        assert_eq!(theme.progress_filled, "\u{2588}");
        assert_eq!(theme.progress_empty, "\u{2591}");
    }

    #[test]
    fn test_light_theme() {
        let theme = Theme::light();
        assert_eq!(theme.name, "light");
    }

    #[test]
    fn test_tactical_theme() {
        let theme = Theme::tactical();
        assert_eq!(theme.name, "tactical");
        assert_eq!(theme.progress_filled, "=");
    }

    #[test]
    fn test_matrix_theme() {
        let theme = Theme::matrix();
        assert_eq!(theme.name, "matrix");
        assert_eq!(theme.progress_filled, "#");
    }

    #[test]
    fn test_hacker_theme() {
        let theme = Theme::hacker();
        assert_eq!(theme.name, "hacker");
        assert_eq!(theme.progress_filled, ">");
    }

    #[test]
    fn test_theme_by_name() {
        assert_eq!(Theme::by_name("dark").name, "dark");
        assert_eq!(Theme::by_name("light").name, "light");
        assert_eq!(Theme::by_name("tactical").name, "tactical");
        assert_eq!(Theme::by_name("matrix").name, "matrix");
        assert_eq!(Theme::by_name("hacker").name, "hacker");
        assert_eq!(Theme::by_name("DARK").name, "dark");
        assert_eq!(Theme::by_name("unknown").name, "dark");
    }

    #[test]
    fn test_available_themes() {
        let themes = Theme::available_themes();
        assert_eq!(themes.len(), 5);
        assert!(themes.contains(&"dark"));
        assert!(themes.contains(&"light"));
        assert!(themes.contains(&"tactical"));
        assert!(themes.contains(&"matrix"));
        assert!(themes.contains(&"hacker"));
    }

    #[test]
    fn test_default_theme_is_dark() {
        let theme = Theme::default();
        assert_eq!(theme.name, "dark");
    }

    #[test]
    fn test_style_builders() {
        let theme = Theme::dark();
        // Verify each style builder returns a valid Style
        let _ = theme.default_style();
        let _ = theme.border_style();
        let _ = theme.border_focused_style();
        let _ = theme.header_style();
        let _ = theme.highlight_style();
        let _ = theme.muted_style();
        let _ = theme.success_style();
        let _ = theme.warning_style();
        let _ = theme.error_style();
        let _ = theme.accent_style();
    }

    #[test]
    fn test_theme_clone() {
        let theme = Theme::tactical();
        let cloned = theme.clone();
        assert_eq!(theme.name, cloned.name);
        assert_eq!(theme.progress_filled, cloned.progress_filled);
    }

    #[test]
    fn test_theme_debug() {
        let theme = Theme::dark();
        let debug_str = format!("{:?}", theme);
        assert!(debug_str.contains("dark"));
    }

    #[test]
    fn test_all_themes_have_distinct_names() {
        let themes: Vec<Theme> = Theme::available_themes()
            .iter()
            .map(|name| Theme::by_name(name))
            .collect();
        for (i, t1) in themes.iter().enumerate() {
            for t2 in themes.iter().skip(i + 1) {
                assert_ne!(t1.name, t2.name, "Duplicate theme name found");
            }
        }
    }
}
