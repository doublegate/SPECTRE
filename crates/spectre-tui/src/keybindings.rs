//! Keyboard shortcut definitions and action routing.
//!
//! Defines all keybindings for the TUI dashboard, including global
//! shortcuts, panel-specific bindings, and vim-style navigation.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::layout::PanelId;

/// Actions that can be triggered by keyboard shortcuts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppAction {
    /// Quit the application
    Quit,
    /// Toggle help overlay
    ToggleHelp,
    /// Focus a specific panel by ID
    FocusPanel(PanelId),
    /// Cycle focus to the next panel
    FocusNext,
    /// Cycle focus to the previous panel
    FocusPrev,
    /// Enter command mode
    EnterCommandMode,
    /// Open command palette (same as command mode)
    OpenCommandPalette,
    /// Toggle focus/fullscreen mode for current panel
    ToggleFocus,
    /// Navigate up in the current panel
    NavigateUp,
    /// Navigate down in the current panel
    NavigateDown,
    /// Navigate left in the current panel
    NavigateLeft,
    /// Navigate right in the current panel
    NavigateRight,
    /// Jump to top of list
    JumpToTop,
    /// Jump to bottom of list
    JumpToBottom,
    /// Scroll half page up
    HalfPageUp,
    /// Scroll half page down
    HalfPageDown,
    /// Select/confirm the current item
    Select,
    /// Go back/cancel
    Back,
    /// No action (key not bound)
    None,
}

/// Map a key event to a global action (applies regardless of focused panel).
///
/// Returns `AppAction::None` if the key is not globally bound.
pub fn map_global_key(key: KeyEvent) -> AppAction {
    // Function keys for panel focus
    match key.code {
        KeyCode::F(1) => return AppAction::FocusPanel(PanelId::Recon),
        KeyCode::F(2) => return AppAction::FocusPanel(PanelId::Analysis),
        KeyCode::F(3) => return AppAction::FocusPanel(PanelId::Comms),
        KeyCode::F(4) => return AppAction::FocusPanel(PanelId::Campaign),
        KeyCode::F(5) => return AppAction::ToggleFocus,
        KeyCode::F(10) => return AppAction::Quit,
        _ => {},
    }

    // Tab cycling
    if key.code == KeyCode::Tab && !key.modifiers.contains(KeyModifiers::SHIFT) {
        return AppAction::FocusNext;
    }
    if key.code == KeyCode::BackTab
        || (key.code == KeyCode::Tab && key.modifiers.contains(KeyModifiers::SHIFT))
    {
        return AppAction::FocusPrev;
    }

    // Ctrl combinations
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        match key.code {
            KeyCode::Char('c' | 'q') => return AppAction::Quit,
            KeyCode::Char('d') => return AppAction::HalfPageDown,
            KeyCode::Char('u') => return AppAction::HalfPageUp,
            _ => {},
        }
    }

    // Single character shortcuts (only when not in command mode)
    match key.code {
        KeyCode::Char('q') => AppAction::Quit,
        KeyCode::Char('?') => AppAction::ToggleHelp,
        KeyCode::Char(':') => AppAction::EnterCommandMode,
        KeyCode::Char('/') => AppAction::OpenCommandPalette,
        KeyCode::Esc => AppAction::Back,
        _ => AppAction::None,
    }
}

/// Map a key event to a panel-specific navigation action.
///
/// Returns `AppAction::None` if the key is not bound for panel navigation.
pub const fn map_panel_key(key: KeyEvent) -> AppAction {
    match key.code {
        // Vim-style navigation
        KeyCode::Char('j') | KeyCode::Down => AppAction::NavigateDown,
        KeyCode::Char('k') | KeyCode::Up => AppAction::NavigateUp,
        KeyCode::Char('h') | KeyCode::Left => AppAction::NavigateLeft,
        KeyCode::Char('l') | KeyCode::Right => AppAction::NavigateRight,
        KeyCode::Char('g') => AppAction::JumpToTop,
        KeyCode::Char('G') => AppAction::JumpToBottom,
        KeyCode::Enter => AppAction::Select,
        _ => AppAction::None,
    }
}

/// A single keybinding entry for help display.
#[derive(Debug, Clone)]
pub struct KeybindingEntry {
    /// Key combination string
    pub keys: &'static str,
    /// Description of what it does
    pub description: &'static str,
    /// Category for grouping
    pub category: KeyCategory,
}

/// Category of keybinding for help display grouping.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCategory {
    /// Global navigation shortcuts
    Global,
    /// Panel focus shortcuts
    PanelFocus,
    /// In-panel navigation
    Navigation,
    /// Command mode shortcuts
    CommandMode,
}

impl std::fmt::Display for KeyCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Global => write!(f, "Global"),
            Self::PanelFocus => write!(f, "Panel Focus"),
            Self::Navigation => write!(f, "Navigation"),
            Self::CommandMode => write!(f, "Command Mode"),
        }
    }
}

/// Get all keybinding entries for the help overlay.
#[allow(clippy::too_many_lines)]
pub fn all_keybindings() -> Vec<KeybindingEntry> {
    vec![
        // Global
        KeybindingEntry {
            keys: "q / F10 / Ctrl+c",
            description: "Quit application",
            category: KeyCategory::Global,
        },
        KeybindingEntry {
            keys: "?",
            description: "Toggle help overlay",
            category: KeyCategory::Global,
        },
        KeybindingEntry {
            keys: ":",
            description: "Enter command mode",
            category: KeyCategory::Global,
        },
        KeybindingEntry {
            keys: "/",
            description: "Open command palette",
            category: KeyCategory::Global,
        },
        KeybindingEntry {
            keys: "Esc",
            description: "Close overlay / cancel",
            category: KeyCategory::Global,
        },
        // Panel Focus
        KeybindingEntry {
            keys: "F1",
            description: "Focus Recon panel",
            category: KeyCategory::PanelFocus,
        },
        KeybindingEntry {
            keys: "F2",
            description: "Focus Analysis panel",
            category: KeyCategory::PanelFocus,
        },
        KeybindingEntry {
            keys: "F3",
            description: "Focus Comms panel",
            category: KeyCategory::PanelFocus,
        },
        KeybindingEntry {
            keys: "F4",
            description: "Focus Campaign panel",
            category: KeyCategory::PanelFocus,
        },
        KeybindingEntry {
            keys: "F5",
            description: "Toggle fullscreen focus",
            category: KeyCategory::PanelFocus,
        },
        KeybindingEntry {
            keys: "Tab",
            description: "Next panel",
            category: KeyCategory::PanelFocus,
        },
        KeybindingEntry {
            keys: "Shift+Tab",
            description: "Previous panel",
            category: KeyCategory::PanelFocus,
        },
        // Navigation
        KeybindingEntry {
            keys: "j / Down",
            description: "Move down",
            category: KeyCategory::Navigation,
        },
        KeybindingEntry {
            keys: "k / Up",
            description: "Move up",
            category: KeyCategory::Navigation,
        },
        KeybindingEntry {
            keys: "h / Left",
            description: "Move left",
            category: KeyCategory::Navigation,
        },
        KeybindingEntry {
            keys: "l / Right",
            description: "Move right",
            category: KeyCategory::Navigation,
        },
        KeybindingEntry {
            keys: "g",
            description: "Jump to top",
            category: KeyCategory::Navigation,
        },
        KeybindingEntry {
            keys: "G",
            description: "Jump to bottom",
            category: KeyCategory::Navigation,
        },
        KeybindingEntry {
            keys: "Ctrl+d",
            description: "Half page down",
            category: KeyCategory::Navigation,
        },
        KeybindingEntry {
            keys: "Ctrl+u",
            description: "Half page up",
            category: KeyCategory::Navigation,
        },
        KeybindingEntry {
            keys: "Enter",
            description: "Select / confirm",
            category: KeyCategory::Navigation,
        },
        // Command Mode
        KeybindingEntry {
            keys: "Esc",
            description: "Exit command mode",
            category: KeyCategory::CommandMode,
        },
        KeybindingEntry {
            keys: "Enter",
            description: "Execute command",
            category: KeyCategory::CommandMode,
        },
        KeybindingEntry {
            keys: "Tab",
            description: "Auto-complete command",
            category: KeyCategory::CommandMode,
        },
        KeybindingEntry {
            keys: "Up / Down",
            description: "Browse command history",
            category: KeyCategory::CommandMode,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_global_f1() {
        let key = KeyEvent::new(KeyCode::F(1), KeyModifiers::NONE);
        assert_eq!(map_global_key(key), AppAction::FocusPanel(PanelId::Recon));
    }

    #[test]
    fn test_map_global_f2() {
        let key = KeyEvent::new(KeyCode::F(2), KeyModifiers::NONE);
        assert_eq!(
            map_global_key(key),
            AppAction::FocusPanel(PanelId::Analysis)
        );
    }

    #[test]
    fn test_map_global_f3() {
        let key = KeyEvent::new(KeyCode::F(3), KeyModifiers::NONE);
        assert_eq!(map_global_key(key), AppAction::FocusPanel(PanelId::Comms));
    }

    #[test]
    fn test_map_global_f4() {
        let key = KeyEvent::new(KeyCode::F(4), KeyModifiers::NONE);
        assert_eq!(
            map_global_key(key),
            AppAction::FocusPanel(PanelId::Campaign)
        );
    }

    #[test]
    fn test_map_global_f5() {
        let key = KeyEvent::new(KeyCode::F(5), KeyModifiers::NONE);
        assert_eq!(map_global_key(key), AppAction::ToggleFocus);
    }

    #[test]
    fn test_map_global_f10() {
        let key = KeyEvent::new(KeyCode::F(10), KeyModifiers::NONE);
        assert_eq!(map_global_key(key), AppAction::Quit);
    }

    #[test]
    fn test_map_global_tab() {
        let key = KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE);
        assert_eq!(map_global_key(key), AppAction::FocusNext);
    }

    #[test]
    fn test_map_global_backtab() {
        let key = KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT);
        assert_eq!(map_global_key(key), AppAction::FocusPrev);
    }

    #[test]
    fn test_map_global_ctrl_c() {
        let key = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        assert_eq!(map_global_key(key), AppAction::Quit);
    }

    #[test]
    fn test_map_global_ctrl_q() {
        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL);
        assert_eq!(map_global_key(key), AppAction::Quit);
    }

    #[test]
    fn test_map_global_ctrl_d() {
        let key = KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL);
        assert_eq!(map_global_key(key), AppAction::HalfPageDown);
    }

    #[test]
    fn test_map_global_ctrl_u() {
        let key = KeyEvent::new(KeyCode::Char('u'), KeyModifiers::CONTROL);
        assert_eq!(map_global_key(key), AppAction::HalfPageUp);
    }

    #[test]
    fn test_map_global_q() {
        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
        assert_eq!(map_global_key(key), AppAction::Quit);
    }

    #[test]
    fn test_map_global_question_mark() {
        let key = KeyEvent::new(KeyCode::Char('?'), KeyModifiers::NONE);
        assert_eq!(map_global_key(key), AppAction::ToggleHelp);
    }

    #[test]
    fn test_map_global_colon() {
        let key = KeyEvent::new(KeyCode::Char(':'), KeyModifiers::NONE);
        assert_eq!(map_global_key(key), AppAction::EnterCommandMode);
    }

    #[test]
    fn test_map_global_slash() {
        let key = KeyEvent::new(KeyCode::Char('/'), KeyModifiers::NONE);
        assert_eq!(map_global_key(key), AppAction::OpenCommandPalette);
    }

    #[test]
    fn test_map_global_esc() {
        let key = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
        assert_eq!(map_global_key(key), AppAction::Back);
    }

    #[test]
    fn test_map_global_unbound() {
        let key = KeyEvent::new(KeyCode::Char('z'), KeyModifiers::NONE);
        assert_eq!(map_global_key(key), AppAction::None);
    }

    #[test]
    fn test_map_panel_vim_j() {
        let key = KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE);
        assert_eq!(map_panel_key(key), AppAction::NavigateDown);
    }

    #[test]
    fn test_map_panel_vim_k() {
        let key = KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE);
        assert_eq!(map_panel_key(key), AppAction::NavigateUp);
    }

    #[test]
    fn test_map_panel_vim_h() {
        let key = KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE);
        assert_eq!(map_panel_key(key), AppAction::NavigateLeft);
    }

    #[test]
    fn test_map_panel_vim_l() {
        let key = KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE);
        assert_eq!(map_panel_key(key), AppAction::NavigateRight);
    }

    #[test]
    fn test_map_panel_g() {
        let key = KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE);
        assert_eq!(map_panel_key(key), AppAction::JumpToTop);
    }

    #[test]
    fn test_map_panel_g_upper() {
        let key = KeyEvent::new(KeyCode::Char('G'), KeyModifiers::NONE);
        assert_eq!(map_panel_key(key), AppAction::JumpToBottom);
    }

    #[test]
    fn test_map_panel_enter() {
        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
        assert_eq!(map_panel_key(key), AppAction::Select);
    }

    #[test]
    fn test_map_panel_arrow_down() {
        let key = KeyEvent::new(KeyCode::Down, KeyModifiers::NONE);
        assert_eq!(map_panel_key(key), AppAction::NavigateDown);
    }

    #[test]
    fn test_map_panel_arrow_up() {
        let key = KeyEvent::new(KeyCode::Up, KeyModifiers::NONE);
        assert_eq!(map_panel_key(key), AppAction::NavigateUp);
    }

    #[test]
    fn test_map_panel_unbound() {
        let key = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE);
        assert_eq!(map_panel_key(key), AppAction::None);
    }

    #[test]
    fn test_all_keybindings_not_empty() {
        let bindings = all_keybindings();
        assert!(!bindings.is_empty());
        assert!(bindings.len() > 20);
    }

    #[test]
    fn test_keybindings_have_all_categories() {
        let bindings = all_keybindings();
        let has_global = bindings.iter().any(|b| b.category == KeyCategory::Global);
        let has_panel = bindings
            .iter()
            .any(|b| b.category == KeyCategory::PanelFocus);
        let has_nav = bindings
            .iter()
            .any(|b| b.category == KeyCategory::Navigation);
        let has_cmd = bindings
            .iter()
            .any(|b| b.category == KeyCategory::CommandMode);

        assert!(has_global);
        assert!(has_panel);
        assert!(has_nav);
        assert!(has_cmd);
    }

    #[test]
    fn test_key_category_display() {
        assert_eq!(KeyCategory::Global.to_string(), "Global");
        assert_eq!(KeyCategory::PanelFocus.to_string(), "Panel Focus");
        assert_eq!(KeyCategory::Navigation.to_string(), "Navigation");
        assert_eq!(KeyCategory::CommandMode.to_string(), "Command Mode");
    }
}
