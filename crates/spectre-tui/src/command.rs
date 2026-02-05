//! Command input widget with parsing, history, and tab completion.
//!
//! Supports commands like `:scan`, `:chef`, `:send`, `:campaign`, `:set`,
//! `:theme`, `:quit`, `:help`, `:export`, `:clear`, and `:layout`.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// All recognized TUI commands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// Start a scan: `:scan <targets> [options]`
    Scan {
        /// Raw arguments string
        args: String,
    },
    /// Run a CyberChef recipe: `:chef <recipe> [input]`
    Chef {
        /// Raw arguments string
        args: String,
    },
    /// Send data via WRAITH: `:send <peer> <data>`
    Send {
        /// Raw arguments string
        args: String,
    },
    /// Campaign management: `:campaign <action>`
    Campaign {
        /// Raw arguments string
        args: String,
    },
    /// Set a configuration value: `:set <key> <value>`
    Set {
        /// The configuration key
        key: String,
        /// The configuration value
        value: String,
    },
    /// Switch theme: `:theme <name>`
    Theme {
        /// Theme name
        name: String,
    },
    /// Quit the application
    Quit,
    /// Show help
    Help,
    /// Export results: `:export <format> [path]`
    Export {
        /// Raw arguments string
        args: String,
    },
    /// Clear the current panel
    Clear,
    /// Switch layout mode: `:layout <mode>`
    Layout {
        /// Layout mode name
        mode: String,
    },
}

/// Available command names for tab completion.
const COMMAND_NAMES: &[&str] = &[
    "scan", "chef", "send", "campaign", "set", "theme", "quit", "help", "export", "clear", "layout",
];

/// Parse a command string into a [`Command`].
///
/// Returns `None` if the input is empty or unrecognized.
pub fn parse_command(input: &str) -> Option<Command> {
    let trimmed = input.trim().trim_start_matches(':');
    if trimmed.is_empty() {
        return None;
    }

    let mut parts = trimmed.splitn(2, char::is_whitespace);
    let cmd = parts.next().unwrap_or_default().to_lowercase();
    let rest = parts.next().unwrap_or_default().trim().to_string();

    match cmd.as_str() {
        "scan" | "s" => Some(Command::Scan { args: rest }),
        "chef" | "c" => Some(Command::Chef { args: rest }),
        "send" => Some(Command::Send { args: rest }),
        "campaign" | "cam" => Some(Command::Campaign { args: rest }),
        "set" => {
            let mut kv = rest.splitn(2, char::is_whitespace);
            let key = kv.next().unwrap_or_default().to_string();
            let value = kv.next().unwrap_or_default().trim().to_string();
            if key == "theme" {
                Some(Command::Theme { name: value })
            } else {
                Some(Command::Set { key, value })
            }
        },
        "theme" | "t" => Some(Command::Theme { name: rest }),
        "quit" | "q" => Some(Command::Quit),
        "help" | "h" | "?" => Some(Command::Help),
        "export" | "e" => Some(Command::Export { args: rest }),
        "clear" => Some(Command::Clear),
        "layout" | "l" => Some(Command::Layout { mode: rest }),
        _ => None,
    }
}

/// State for the command input widget.
#[derive(Debug, Clone)]
pub struct CommandInput {
    /// Whether command mode is active
    pub active: bool,
    /// Current input buffer
    pub buffer: String,
    /// Cursor position within the buffer
    pub cursor: usize,
    /// Command history
    pub history: Vec<String>,
    /// Current history browsing index (None = new input)
    pub history_index: Option<usize>,
    /// Saved buffer when browsing history
    saved_buffer: String,
    /// Last error message to display
    pub error: Option<String>,
    /// Last info/result message to display
    pub message: Option<String>,
}

impl CommandInput {
    /// Create a new command input widget.
    pub fn new() -> Self {
        Self {
            active: false,
            buffer: String::new(),
            cursor: 0,
            history: Vec::new(),
            history_index: None,
            saved_buffer: String::new(),
            error: None,
            message: None,
        }
    }

    /// Activate command mode.
    pub fn activate(&mut self) {
        self.active = true;
        self.buffer.clear();
        self.cursor = 0;
        self.history_index = None;
        self.error = None;
        self.message = None;
    }

    /// Deactivate command mode.
    pub fn deactivate(&mut self) {
        self.active = false;
        self.buffer.clear();
        self.cursor = 0;
        self.history_index = None;
    }

    /// Submit the current buffer and return the parsed command.
    pub fn submit(&mut self) -> Option<Command> {
        let input = self.buffer.trim().to_string();
        if input.is_empty() {
            self.deactivate();
            return None;
        }

        // Add to history if different from last entry
        if self.history.last().is_none_or(|last| *last != input) {
            self.history.push(input.clone());
        }

        let cmd = parse_command(&input);
        self.deactivate();

        if cmd.is_none() {
            self.error = Some(format!("Unknown command: {}", input));
        }

        cmd
    }

    /// Handle a key event while command mode is active.
    ///
    /// Returns `true` if the event was consumed.
    pub fn on_key(&mut self, key: KeyEvent) -> bool {
        if !self.active {
            return false;
        }

        match key.code {
            KeyCode::Char(c) => {
                self.buffer.insert(self.cursor, c);
                self.cursor += 1;
                self.error = None;
            },
            KeyCode::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.buffer.remove(self.cursor);
                }
            },
            KeyCode::Delete => {
                if self.cursor < self.buffer.len() {
                    self.buffer.remove(self.cursor);
                }
            },
            KeyCode::Left => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    // Move to previous word boundary
                    self.cursor = self.prev_word_boundary();
                } else if self.cursor > 0 {
                    self.cursor -= 1;
                }
            },
            KeyCode::Right => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    // Move to next word boundary
                    self.cursor = self.next_word_boundary();
                } else if self.cursor < self.buffer.len() {
                    self.cursor += 1;
                }
            },
            KeyCode::Home => {
                self.cursor = 0;
            },
            KeyCode::End => {
                self.cursor = self.buffer.len();
            },
            KeyCode::Up => {
                self.history_prev();
            },
            KeyCode::Down => {
                self.history_next();
            },
            KeyCode::Tab => {
                self.tab_complete();
            },
            KeyCode::Esc => {
                self.deactivate();
            },
            _ => {},
        }

        true
    }

    /// Navigate to the previous history entry.
    fn history_prev(&mut self) {
        if self.history.is_empty() {
            return;
        }

        match self.history_index {
            None => {
                self.saved_buffer = self.buffer.clone();
                let idx = self.history.len() - 1;
                self.history_index = Some(idx);
                self.buffer = self.history[idx].clone();
                self.cursor = self.buffer.len();
            },
            Some(idx) if idx > 0 => {
                let new_idx = idx - 1;
                self.history_index = Some(new_idx);
                self.buffer = self.history[new_idx].clone();
                self.cursor = self.buffer.len();
            },
            _ => {},
        }
    }

    /// Navigate to the next history entry.
    fn history_next(&mut self) {
        if let Some(idx) = self.history_index {
            if idx + 1 < self.history.len() {
                let new_idx = idx + 1;
                self.history_index = Some(new_idx);
                self.buffer = self.history[new_idx].clone();
                self.cursor = self.buffer.len();
            } else {
                self.history_index = None;
                self.buffer = self.saved_buffer.clone();
                self.cursor = self.buffer.len();
            }
        }
    }

    /// Perform tab completion on the current buffer.
    fn tab_complete(&mut self) {
        let trimmed = self.buffer.trim_start_matches(':');
        if trimmed.is_empty() {
            return;
        }

        // Only complete the command name (first word)
        let first_word = trimmed.split_whitespace().next().unwrap_or_default();
        let has_space = trimmed.contains(' ');

        if has_space {
            return; // Do not complete arguments
        }

        let matches: Vec<&&str> = COMMAND_NAMES
            .iter()
            .filter(|name| name.starts_with(first_word))
            .collect();

        if matches.len() == 1 {
            let prefix = if self.buffer.starts_with(':') {
                ":"
            } else {
                ""
            };
            self.buffer = format!("{}{} ", prefix, matches[0]);
            self.cursor = self.buffer.len();
        }
    }

    /// Find the previous word boundary from cursor position.
    fn prev_word_boundary(&self) -> usize {
        if self.cursor == 0 {
            return 0;
        }
        let bytes = self.buffer.as_bytes();
        let mut pos = self.cursor - 1;
        // Skip whitespace
        while pos > 0 && bytes[pos] == b' ' {
            pos -= 1;
        }
        // Skip word characters
        while pos > 0 && bytes[pos - 1] != b' ' {
            pos -= 1;
        }
        pos
    }

    /// Find the next word boundary from cursor position.
    fn next_word_boundary(&self) -> usize {
        let len = self.buffer.len();
        if self.cursor >= len {
            return len;
        }
        let bytes = self.buffer.as_bytes();
        let mut pos = self.cursor;
        // Skip current word characters
        while pos < len && bytes[pos] != b' ' {
            pos += 1;
        }
        // Skip whitespace
        while pos < len && bytes[pos] == b' ' {
            pos += 1;
        }
        pos
    }

    /// Get the current buffer content.
    pub fn text(&self) -> &str {
        &self.buffer
    }

    /// Get the display text including the command prefix.
    pub fn display_text(&self) -> String {
        format!(":{}", self.buffer)
    }

    /// Get the cursor position for display (accounting for the ':' prefix).
    pub fn display_cursor(&self) -> usize {
        self.cursor + 1
    }
}

impl Default for CommandInput {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_scan_command() {
        let cmd = parse_command(":scan 192.168.1.0/24 -p 80,443");
        assert!(matches!(cmd, Some(Command::Scan { args }) if args == "192.168.1.0/24 -p 80,443"));
    }

    #[test]
    fn test_parse_scan_shorthand() {
        let cmd = parse_command(":s 10.0.0.1");
        assert!(matches!(cmd, Some(Command::Scan { args }) if args == "10.0.0.1"));
    }

    #[test]
    fn test_parse_chef_command() {
        let cmd = parse_command(":chef base64_decode input");
        assert!(matches!(cmd, Some(Command::Chef { args }) if args == "base64_decode input"));
    }

    #[test]
    fn test_parse_send_command() {
        let cmd = parse_command(":send peer1 hello");
        assert!(matches!(cmd, Some(Command::Send { args }) if args == "peer1 hello"));
    }

    #[test]
    fn test_parse_campaign_command() {
        let cmd = parse_command(":campaign new TestOp");
        assert!(matches!(cmd, Some(Command::Campaign { args }) if args == "new TestOp"));
    }

    #[test]
    fn test_parse_set_command() {
        let cmd = parse_command(":set scan.timing 4");
        assert!(
            matches!(cmd, Some(Command::Set { key, value }) if key == "scan.timing" && value == "4")
        );
    }

    #[test]
    fn test_parse_set_theme_alias() {
        let cmd = parse_command(":set theme matrix");
        assert!(matches!(cmd, Some(Command::Theme { name }) if name == "matrix"));
    }

    #[test]
    fn test_parse_theme_command() {
        let cmd = parse_command(":theme tactical");
        assert!(matches!(cmd, Some(Command::Theme { name }) if name == "tactical"));
    }

    #[test]
    fn test_parse_quit() {
        assert!(matches!(parse_command(":quit"), Some(Command::Quit)));
        assert!(matches!(parse_command(":q"), Some(Command::Quit)));
    }

    #[test]
    fn test_parse_help() {
        assert!(matches!(parse_command(":help"), Some(Command::Help)));
        assert!(matches!(parse_command(":h"), Some(Command::Help)));
        assert!(matches!(parse_command(":?"), Some(Command::Help)));
    }

    #[test]
    fn test_parse_export() {
        let cmd = parse_command(":export json /tmp/out.json");
        assert!(matches!(cmd, Some(Command::Export { args }) if args == "json /tmp/out.json"));
    }

    #[test]
    fn test_parse_clear() {
        assert!(matches!(parse_command(":clear"), Some(Command::Clear)));
    }

    #[test]
    fn test_parse_layout() {
        let cmd = parse_command(":layout wide");
        assert!(matches!(cmd, Some(Command::Layout { mode }) if mode == "wide"));
    }

    #[test]
    fn test_parse_empty() {
        assert!(parse_command("").is_none());
        assert!(parse_command(":").is_none());
        assert!(parse_command("  ").is_none());
    }

    #[test]
    fn test_parse_unknown() {
        assert!(parse_command(":foobar").is_none());
    }

    #[test]
    fn test_command_input_new() {
        let input = CommandInput::new();
        assert!(!input.active);
        assert!(input.buffer.is_empty());
        assert_eq!(input.cursor, 0);
        assert!(input.history.is_empty());
    }

    #[test]
    fn test_command_input_activate_deactivate() {
        let mut input = CommandInput::new();
        input.activate();
        assert!(input.active);

        input.deactivate();
        assert!(!input.active);
        assert!(input.buffer.is_empty());
    }

    #[test]
    fn test_command_input_char_insertion() {
        let mut input = CommandInput::new();
        input.activate();

        input.on_key(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE));
        input.on_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));

        assert_eq!(input.buffer, "hi");
        assert_eq!(input.cursor, 2);
    }

    #[test]
    fn test_command_input_backspace() {
        let mut input = CommandInput::new();
        input.activate();
        input.buffer = "abc".to_string();
        input.cursor = 3;

        input.on_key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
        assert_eq!(input.buffer, "ab");
        assert_eq!(input.cursor, 2);
    }

    #[test]
    fn test_command_input_delete() {
        let mut input = CommandInput::new();
        input.activate();
        input.buffer = "abc".to_string();
        input.cursor = 1;

        input.on_key(KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE));
        assert_eq!(input.buffer, "ac");
    }

    #[test]
    fn test_command_input_left_right() {
        let mut input = CommandInput::new();
        input.activate();
        input.buffer = "abc".to_string();
        input.cursor = 2;

        input.on_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
        assert_eq!(input.cursor, 1);

        input.on_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
        assert_eq!(input.cursor, 2);
    }

    #[test]
    fn test_command_input_home_end() {
        let mut input = CommandInput::new();
        input.activate();
        input.buffer = "abcdef".to_string();
        input.cursor = 3;

        input.on_key(KeyEvent::new(KeyCode::Home, KeyModifiers::NONE));
        assert_eq!(input.cursor, 0);

        input.on_key(KeyEvent::new(KeyCode::End, KeyModifiers::NONE));
        assert_eq!(input.cursor, 6);
    }

    #[test]
    fn test_command_input_submit() {
        let mut input = CommandInput::new();
        input.activate();
        input.buffer = "quit".to_string();
        input.cursor = 4;

        let cmd = input.submit();
        assert!(matches!(cmd, Some(Command::Quit)));
        assert!(!input.active);
        assert_eq!(input.history.len(), 1);
    }

    #[test]
    fn test_command_input_submit_empty() {
        let mut input = CommandInput::new();
        input.activate();
        let cmd = input.submit();
        assert!(cmd.is_none());
        assert!(!input.active);
    }

    #[test]
    fn test_command_input_submit_unknown() {
        let mut input = CommandInput::new();
        input.activate();
        input.buffer = "foobar".to_string();
        input.cursor = 6;
        let cmd = input.submit();
        assert!(cmd.is_none());
        assert!(input.error.is_some());
    }

    #[test]
    fn test_command_input_history_navigation() {
        let mut input = CommandInput::new();
        input.history = vec!["scan 10.0.0.1".to_string(), "theme dark".to_string()];

        input.activate();
        input.buffer = "new".to_string();

        // Press Up - should go to last history entry
        input.on_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
        assert_eq!(input.buffer, "theme dark");
        assert_eq!(input.history_index, Some(1));

        // Press Up again - should go to first
        input.on_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
        assert_eq!(input.buffer, "scan 10.0.0.1");
        assert_eq!(input.history_index, Some(0));

        // Press Down - should go back to last
        input.on_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        assert_eq!(input.buffer, "theme dark");

        // Press Down again - should restore saved buffer
        input.on_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        assert_eq!(input.buffer, "new");
        assert!(input.history_index.is_none());
    }

    #[test]
    fn test_command_input_esc() {
        let mut input = CommandInput::new();
        input.activate();
        input.buffer = "partial".to_string();

        input.on_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        assert!(!input.active);
    }

    #[test]
    fn test_command_input_tab_complete() {
        let mut input = CommandInput::new();
        input.activate();
        input.buffer = ":sc".to_string();
        input.cursor = 3;

        input.on_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
        assert_eq!(input.buffer, ":scan ");
    }

    #[test]
    fn test_command_input_tab_complete_ambiguous() {
        let mut input = CommandInput::new();
        input.activate();
        input.buffer = "s".to_string();
        input.cursor = 1;

        // "s" matches scan, send, set - should not complete
        input.on_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
        assert_eq!(input.buffer, "s");
    }

    #[test]
    fn test_command_input_display() {
        let mut input = CommandInput::new();
        input.activate();
        input.buffer = "help".to_string();
        input.cursor = 4;

        assert_eq!(input.display_text(), ":help");
        assert_eq!(input.display_cursor(), 5);
        assert_eq!(input.text(), "help");
    }

    #[test]
    fn test_command_input_inactive_ignores_keys() {
        let mut input = CommandInput::new();
        let consumed = input.on_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
        assert!(!consumed);
        assert!(input.buffer.is_empty());
    }

    #[test]
    fn test_command_input_no_duplicate_history() {
        let mut input = CommandInput::new();
        input.activate();
        input.buffer = "quit".to_string();
        input.submit();

        input.activate();
        input.buffer = "quit".to_string();
        input.submit();

        assert_eq!(input.history.len(), 1);
    }

    #[test]
    fn test_word_boundaries() {
        let mut input = CommandInput::new();
        input.activate();
        input.buffer = "scan 192.168.1.1".to_string();
        input.cursor = 16;

        // Ctrl+Left to move back one word
        input.on_key(KeyEvent::new(KeyCode::Left, KeyModifiers::CONTROL));
        assert_eq!(input.cursor, 5);

        // Ctrl+Left again
        input.on_key(KeyEvent::new(KeyCode::Left, KeyModifiers::CONTROL));
        assert_eq!(input.cursor, 0);

        // Ctrl+Right to move forward one word
        input.on_key(KeyEvent::new(KeyCode::Right, KeyModifiers::CONTROL));
        assert_eq!(input.cursor, 5);
    }

    #[test]
    fn test_backspace_at_start() {
        let mut input = CommandInput::new();
        input.activate();
        input.cursor = 0;
        input.on_key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
        assert_eq!(input.cursor, 0);
    }

    #[test]
    fn test_left_at_start() {
        let mut input = CommandInput::new();
        input.activate();
        input.cursor = 0;
        input.on_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
        assert_eq!(input.cursor, 0);
    }

    #[test]
    fn test_right_at_end() {
        let mut input = CommandInput::new();
        input.activate();
        input.buffer = "ab".to_string();
        input.cursor = 2;
        input.on_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
        assert_eq!(input.cursor, 2);
    }

    #[test]
    fn test_history_up_empty() {
        let mut input = CommandInput::new();
        input.activate();
        // No history entries
        input.on_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
        assert!(input.history_index.is_none());
    }

    #[test]
    fn test_default_command_input() {
        let input = CommandInput::default();
        assert!(!input.active);
    }
}
