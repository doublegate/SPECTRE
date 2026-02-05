//! Layout computation and panel management.
//!
//! Implements a 4-panel layout system with multiple layout modes:
//! Grid (2x2), Wide (horizontal), Tall (vertical), and Focus (single panel).

use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Identifies which panel is focused or referenced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PanelId {
    /// Network reconnaissance panel (ProRT-IP)
    Recon,
    /// Data analysis panel (CyberChef-MCP)
    Analysis,
    /// Secure communications panel (WRAITH)
    Comms,
    /// Campaign management panel
    Campaign,
}

impl PanelId {
    /// Get all panel IDs in order.
    pub const fn all() -> &'static [Self] {
        &[Self::Recon, Self::Analysis, Self::Comms, Self::Campaign]
    }

    /// Get the next panel in cycle order.
    pub const fn next(self) -> Self {
        match self {
            Self::Recon => Self::Analysis,
            Self::Analysis => Self::Comms,
            Self::Comms => Self::Campaign,
            Self::Campaign => Self::Recon,
        }
    }

    /// Get the previous panel in cycle order.
    pub const fn prev(self) -> Self {
        match self {
            Self::Recon => Self::Campaign,
            Self::Analysis => Self::Recon,
            Self::Comms => Self::Analysis,
            Self::Campaign => Self::Comms,
        }
    }

    /// Get the display title for this panel.
    pub const fn title(self) -> &'static str {
        match self {
            Self::Recon => "Recon [ProRT-IP]",
            Self::Analysis => "Analysis [CyberChef]",
            Self::Comms => "Comms [WRAITH]",
            Self::Campaign => "Campaign",
        }
    }

    /// Get a numeric index (0-based) for this panel.
    pub const fn index(self) -> usize {
        match self {
            Self::Recon => 0,
            Self::Analysis => 1,
            Self::Comms => 2,
            Self::Campaign => 3,
        }
    }

    /// Create a panel ID from a numeric index.
    pub const fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Recon),
            1 => Some(Self::Analysis),
            2 => Some(Self::Comms),
            3 => Some(Self::Campaign),
            _ => None,
        }
    }
}

impl std::fmt::Display for PanelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title())
    }
}

/// Layout mode for the dashboard panels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutMode {
    /// 2x2 grid layout (default)
    Grid,
    /// Horizontal split (4 columns)
    Wide,
    /// Vertical split (4 rows)
    Tall,
    /// Focus on a single panel (fullscreen)
    Focus(PanelId),
}

impl LayoutMode {
    /// Parse a layout mode from a string.
    #[allow(clippy::match_same_arms)] // "focus" arm documented separately for clarity
    pub fn from_str_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "grid" | "g" => Some(Self::Grid),
            "wide" | "w" | "horizontal" => Some(Self::Wide),
            "tall" | "t" | "vertical" => Some(Self::Tall),
            "focus" | "f" => None, // Needs a panel ID
            _ => None,
        }
    }

    /// Get available layout mode names.
    pub const fn available_modes() -> &'static [&'static str] {
        &["grid", "wide", "tall", "focus"]
    }
}

/// Manages panel layout computation.
#[derive(Debug, Clone)]
pub struct PanelManager {
    /// Current layout mode
    pub mode: LayoutMode,
    /// Currently focused panel
    pub focused: PanelId,
}

impl PanelManager {
    /// Create a new panel manager with default grid layout.
    pub const fn new() -> Self {
        Self {
            mode: LayoutMode::Grid,
            focused: PanelId::Recon,
        }
    }

    /// Cycle focus to the next panel.
    pub const fn focus_next(&mut self) {
        self.focused = self.focused.next();
    }

    /// Cycle focus to the previous panel.
    pub const fn focus_prev(&mut self) {
        self.focused = self.focused.prev();
    }

    /// Set focus to a specific panel.
    pub const fn focus_panel(&mut self, panel: PanelId) {
        self.focused = panel;
    }

    /// Set the layout mode.
    pub const fn set_mode(&mut self, mode: LayoutMode) {
        self.mode = mode;
    }

    /// Toggle between grid layout and focus mode for the current panel.
    pub const fn toggle_focus(&mut self) {
        self.mode = match self.mode {
            LayoutMode::Focus(_) => LayoutMode::Grid,
            _ => LayoutMode::Focus(self.focused),
        };
    }

    /// Compute the main content area by splitting off header and status bar.
    ///
    /// Returns `(header_area, content_area, status_area)`.
    pub fn compute_main_areas(&self, area: Rect) -> (Rect, Rect, Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Header
                Constraint::Min(10),   // Content
                Constraint::Length(1), // Status bar
            ])
            .split(area);
        (chunks[0], chunks[1], chunks[2])
    }

    /// Compute panel areas within the content area based on the current layout mode.
    ///
    /// Returns a vec of `(PanelId, Rect)` tuples for each visible panel.
    pub fn compute_panel_areas(&self, content: Rect) -> Vec<(PanelId, Rect)> {
        match self.mode {
            LayoutMode::Grid => self.compute_grid(content),
            LayoutMode::Wide => self.compute_wide(content),
            LayoutMode::Tall => self.compute_tall(content),
            LayoutMode::Focus(panel) => {
                vec![(panel, content)]
            },
        }
    }

    /// Compute 2x2 grid layout.
    fn compute_grid(&self, area: Rect) -> Vec<(PanelId, Rect)> {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let top_cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(rows[0]);

        let bottom_cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(rows[1]);

        vec![
            (PanelId::Recon, top_cols[0]),
            (PanelId::Analysis, top_cols[1]),
            (PanelId::Comms, bottom_cols[0]),
            (PanelId::Campaign, bottom_cols[1]),
        ]
    }

    /// Compute horizontal (wide) layout: 4 columns.
    fn compute_wide(&self, area: Rect) -> Vec<(PanelId, Rect)> {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(area);

        vec![
            (PanelId::Recon, cols[0]),
            (PanelId::Analysis, cols[1]),
            (PanelId::Comms, cols[2]),
            (PanelId::Campaign, cols[3]),
        ]
    }

    /// Compute vertical (tall) layout: 4 rows.
    fn compute_tall(&self, area: Rect) -> Vec<(PanelId, Rect)> {
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(area);

        vec![
            (PanelId::Recon, rows[0]),
            (PanelId::Analysis, rows[1]),
            (PanelId::Comms, rows[2]),
            (PanelId::Campaign, rows[3]),
        ]
    }
}

impl Default for PanelManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_panel_id_all() {
        let all = PanelId::all();
        assert_eq!(all.len(), 4);
        assert_eq!(all[0], PanelId::Recon);
        assert_eq!(all[3], PanelId::Campaign);
    }

    #[test]
    fn test_panel_id_next() {
        assert_eq!(PanelId::Recon.next(), PanelId::Analysis);
        assert_eq!(PanelId::Analysis.next(), PanelId::Comms);
        assert_eq!(PanelId::Comms.next(), PanelId::Campaign);
        assert_eq!(PanelId::Campaign.next(), PanelId::Recon);
    }

    #[test]
    fn test_panel_id_prev() {
        assert_eq!(PanelId::Recon.prev(), PanelId::Campaign);
        assert_eq!(PanelId::Analysis.prev(), PanelId::Recon);
        assert_eq!(PanelId::Comms.prev(), PanelId::Analysis);
        assert_eq!(PanelId::Campaign.prev(), PanelId::Comms);
    }

    #[test]
    fn test_panel_id_title() {
        assert!(PanelId::Recon.title().contains("Recon"));
        assert!(PanelId::Analysis.title().contains("Analysis"));
        assert!(PanelId::Comms.title().contains("Comms"));
        assert!(PanelId::Campaign.title().contains("Campaign"));
    }

    #[test]
    fn test_panel_id_index_roundtrip() {
        for panel in PanelId::all() {
            let idx = panel.index();
            assert_eq!(PanelId::from_index(idx), Some(*panel));
        }
    }

    #[test]
    fn test_panel_id_from_invalid_index() {
        assert!(PanelId::from_index(4).is_none());
        assert!(PanelId::from_index(100).is_none());
    }

    #[test]
    fn test_panel_id_display() {
        let s = format!("{}", PanelId::Recon);
        assert!(s.contains("Recon"));
    }

    #[test]
    fn test_layout_mode_from_str() {
        assert_eq!(LayoutMode::from_str_name("grid"), Some(LayoutMode::Grid));
        assert_eq!(LayoutMode::from_str_name("wide"), Some(LayoutMode::Wide));
        assert_eq!(LayoutMode::from_str_name("tall"), Some(LayoutMode::Tall));
        assert_eq!(LayoutMode::from_str_name("GRID"), Some(LayoutMode::Grid));
        assert!(LayoutMode::from_str_name("focus").is_none());
        assert!(LayoutMode::from_str_name("invalid").is_none());
    }

    #[test]
    fn test_layout_mode_available() {
        let modes = LayoutMode::available_modes();
        assert!(modes.contains(&"grid"));
        assert!(modes.contains(&"wide"));
        assert!(modes.contains(&"tall"));
        assert!(modes.contains(&"focus"));
    }

    #[test]
    fn test_panel_manager_new() {
        let pm = PanelManager::new();
        assert_eq!(pm.mode, LayoutMode::Grid);
        assert_eq!(pm.focused, PanelId::Recon);
    }

    #[test]
    fn test_panel_manager_focus_cycle() {
        let mut pm = PanelManager::new();
        pm.focus_next();
        assert_eq!(pm.focused, PanelId::Analysis);
        pm.focus_next();
        assert_eq!(pm.focused, PanelId::Comms);
        pm.focus_prev();
        assert_eq!(pm.focused, PanelId::Analysis);
    }

    #[test]
    fn test_panel_manager_focus_panel() {
        let mut pm = PanelManager::new();
        pm.focus_panel(PanelId::Campaign);
        assert_eq!(pm.focused, PanelId::Campaign);
    }

    #[test]
    fn test_panel_manager_set_mode() {
        let mut pm = PanelManager::new();
        pm.set_mode(LayoutMode::Wide);
        assert_eq!(pm.mode, LayoutMode::Wide);
    }

    #[test]
    fn test_panel_manager_toggle_focus() {
        let mut pm = PanelManager::new();
        pm.focus_panel(PanelId::Comms);
        pm.toggle_focus();
        assert_eq!(pm.mode, LayoutMode::Focus(PanelId::Comms));
        pm.toggle_focus();
        assert_eq!(pm.mode, LayoutMode::Grid);
    }

    #[test]
    fn test_compute_main_areas() {
        let pm = PanelManager::new();
        let area = Rect::new(0, 0, 120, 40);
        let (header, content, status) = pm.compute_main_areas(area);

        assert_eq!(header.height, 1);
        assert_eq!(status.height, 1);
        assert!(content.height >= 10);
        assert_eq!(header.y, 0);
        assert_eq!(status.y, header.height + content.height);
    }

    #[test]
    fn test_compute_grid_layout() {
        let pm = PanelManager::new();
        let area = Rect::new(0, 0, 100, 40);
        let panels = pm.compute_panel_areas(area);

        assert_eq!(panels.len(), 4);
        assert_eq!(panels[0].0, PanelId::Recon);
        assert_eq!(panels[1].0, PanelId::Analysis);
        assert_eq!(panels[2].0, PanelId::Comms);
        assert_eq!(panels[3].0, PanelId::Campaign);
    }

    #[test]
    fn test_compute_wide_layout() {
        let mut pm = PanelManager::new();
        pm.set_mode(LayoutMode::Wide);
        let area = Rect::new(0, 0, 200, 40);
        let panels = pm.compute_panel_areas(area);

        assert_eq!(panels.len(), 4);
        // All panels should be on the same row (same y)
        let y_vals: Vec<u16> = panels.iter().map(|(_, r)| r.y).collect();
        assert!(y_vals.iter().all(|&y| y == y_vals[0]));
    }

    #[test]
    fn test_compute_tall_layout() {
        let mut pm = PanelManager::new();
        pm.set_mode(LayoutMode::Tall);
        let area = Rect::new(0, 0, 80, 80);
        let panels = pm.compute_panel_areas(area);

        assert_eq!(panels.len(), 4);
        // All panels should be in the same column (same x)
        let x_vals: Vec<u16> = panels.iter().map(|(_, r)| r.x).collect();
        assert!(x_vals.iter().all(|&x| x == x_vals[0]));
    }

    #[test]
    fn test_compute_focus_layout() {
        let mut pm = PanelManager::new();
        pm.set_mode(LayoutMode::Focus(PanelId::Analysis));
        let area = Rect::new(0, 0, 100, 40);
        let panels = pm.compute_panel_areas(area);

        assert_eq!(panels.len(), 1);
        assert_eq!(panels[0].0, PanelId::Analysis);
        assert_eq!(panels[0].1, area);
    }

    #[test]
    fn test_panel_manager_default() {
        let pm = PanelManager::default();
        assert_eq!(pm.mode, LayoutMode::Grid);
    }

    #[test]
    fn test_full_cycle() {
        let mut pm = PanelManager::new();
        for _ in 0..4 {
            pm.focus_next();
        }
        assert_eq!(pm.focused, PanelId::Recon);

        for _ in 0..4 {
            pm.focus_prev();
        }
        assert_eq!(pm.focused, PanelId::Recon);
    }
}
