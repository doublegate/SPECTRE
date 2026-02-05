//! Comms panel implementation for WRAITH secure communications display.
//!
//! Shows peer connections, message/file transfer queue, and channel status.

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table};
use ratatui::Frame;

use crate::keybindings::AppAction;
use crate::panels::Panel;
use crate::theme::Theme;

/// State for the Communications panel.
#[derive(Debug, Clone)]
pub struct CommsPanel {
    /// Panel title
    title: String,
    /// Connected peers
    pub peers: Vec<PeerInfo>,
    /// Transfer queue entries
    pub transfers: Vec<TransferEntry>,
    /// Selected peer index
    pub selected_peer: usize,
    /// Selected transfer index
    pub selected_transfer: usize,
    /// Whether peer list or transfer list is focused
    pub focus_transfers: bool,
}

/// Information about a connected peer.
#[derive(Debug, Clone)]
pub struct PeerInfo {
    /// Peer display name or identifier
    pub name: String,
    /// Connection status
    pub status: PeerStatus,
    /// Latency in milliseconds
    pub latency_ms: Option<u32>,
    /// Encryption cipher in use
    pub cipher: String,
}

/// Peer connection status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeerStatus {
    /// Connected and active
    Connected,
    /// Connection attempt in progress
    Connecting,
    /// Disconnected
    Disconnected,
    /// Authentication in progress
    Authenticating,
}

impl std::fmt::Display for PeerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Connected => write!(f, "CONNECTED"),
            Self::Connecting => write!(f, "CONNECTING"),
            Self::Disconnected => write!(f, "OFFLINE"),
            Self::Authenticating => write!(f, "AUTHING"),
        }
    }
}

/// A file/message transfer entry.
#[derive(Debug, Clone)]
pub struct TransferEntry {
    /// Transfer name or filename
    pub name: String,
    /// Transfer direction
    pub direction: TransferDirection,
    /// Progress as a fraction (0.0 to 1.0)
    pub progress: f64,
    /// Size in bytes
    pub size: u64,
    /// Transfer status
    pub status: TransferStatus,
}

/// Transfer direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferDirection {
    /// Sending to peer
    Send,
    /// Receiving from peer
    Receive,
}

impl std::fmt::Display for TransferDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Send => write!(f, "TX"),
            Self::Receive => write!(f, "RX"),
        }
    }
}

/// Transfer status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferStatus {
    /// Transfer is queued
    Queued,
    /// Transfer is in progress
    Active,
    /// Transfer completed
    Complete,
    /// Transfer failed
    Failed,
}

impl std::fmt::Display for TransferStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Queued => write!(f, "QUEUED"),
            Self::Active => write!(f, "ACTIVE"),
            Self::Complete => write!(f, "DONE"),
            Self::Failed => write!(f, "FAILED"),
        }
    }
}

impl CommsPanel {
    /// Create a new comms panel.
    pub fn new() -> Self {
        Self {
            title: "Comms [WRAITH]".to_string(),
            peers: Vec::new(),
            transfers: Vec::new(),
            selected_peer: 0,
            selected_transfer: 0,
            focus_transfers: false,
        }
    }

    /// Clear all comms state.
    pub fn clear(&mut self) {
        self.peers.clear();
        self.transfers.clear();
        self.selected_peer = 0;
        self.selected_transfer = 0;
    }

    /// Format a byte size into a human-readable string.
    fn format_size(bytes: u64) -> String {
        if bytes >= 1_073_741_824 {
            format!("{:.1} GB", bytes as f64 / 1_073_741_824.0)
        } else if bytes >= 1_048_576 {
            format!("{:.1} MB", bytes as f64 / 1_048_576.0)
        } else if bytes >= 1024 {
            format!("{:.1} KB", bytes as f64 / 1024.0)
        } else {
            format!("{} B", bytes)
        }
    }
}

impl Default for CommsPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl Panel for CommsPanel {
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

        if self.peers.is_empty() && self.transfers.is_empty() {
            let lines = vec![
                Line::from(Span::styled("No active connections", theme.muted_style())),
                Line::from(""),
                Line::from(Span::styled(
                    "Use :send <peer> <data> to connect",
                    theme.muted_style(),
                )),
            ];
            let paragraph = Paragraph::new(lines).style(theme.default_style());
            frame.render_widget(paragraph, inner);
            return;
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(inner);

        // Peer list
        self.render_peers(frame, chunks[0], theme);

        // Transfer queue
        self.render_transfers(frame, chunks[1], theme);
    }

    fn on_key(&mut self, key: KeyEvent) -> Option<AppAction> {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if self.focus_transfers {
                    let max = self.transfers.len().saturating_sub(1);
                    if self.selected_transfer < max {
                        self.selected_transfer += 1;
                    }
                } else {
                    let max = self.peers.len().saturating_sub(1);
                    if self.selected_peer < max {
                        self.selected_peer += 1;
                    }
                }
                None
            },
            KeyCode::Char('k') | KeyCode::Up => {
                if self.focus_transfers {
                    self.selected_transfer = self.selected_transfer.saturating_sub(1);
                } else {
                    self.selected_peer = self.selected_peer.saturating_sub(1);
                }
                None
            },
            KeyCode::Char('\t') | KeyCode::Tab => {
                self.focus_transfers = !self.focus_transfers;
                None
            },
            _ => None,
        }
    }

    fn title(&self) -> &str {
        &self.title
    }
}

impl CommsPanel {
    /// Render the peer list section.
    fn render_peers(&self, frame: &mut Frame, area: Rect, theme: &Theme) {
        let header_cells = ["Peer", "Status", "Latency", "Cipher"].iter().map(|h| {
            Cell::from(*h).style(
                Style::default()
                    .fg(theme.primary)
                    .add_modifier(Modifier::BOLD),
            )
        });
        let header = Row::new(header_cells).height(1);

        let rows: Vec<Row> = self
            .peers
            .iter()
            .enumerate()
            .map(|(i, peer)| {
                let style = if i == self.selected_peer && !self.focus_transfers {
                    theme.highlight_style()
                } else {
                    theme.default_style()
                };

                let status_style = match peer.status {
                    PeerStatus::Connected => Style::default().fg(theme.success),
                    PeerStatus::Connecting | PeerStatus::Authenticating => {
                        Style::default().fg(theme.warning)
                    },
                    PeerStatus::Disconnected => Style::default().fg(theme.error),
                };

                let latency = peer
                    .latency_ms
                    .map_or_else(|| "--".to_string(), |ms| format!("{}ms", ms));

                Row::new(vec![
                    Cell::from(peer.name.as_str()),
                    Cell::from(Span::styled(peer.status.to_string(), status_style)),
                    Cell::from(latency),
                    Cell::from(peer.cipher.as_str()),
                ])
                .style(style)
            })
            .collect();

        let table = Table::new(
            rows,
            [
                Constraint::Min(15),
                Constraint::Length(12),
                Constraint::Length(8),
                Constraint::Min(15),
            ],
        )
        .header(header)
        .style(theme.default_style());

        frame.render_widget(table, area);
    }

    /// Render the transfer queue section.
    fn render_transfers(&self, frame: &mut Frame, area: Rect, theme: &Theme) {
        let header_cells = ["Name", "Dir", "Progress", "Size", "Status"]
            .iter()
            .map(|h| {
                Cell::from(*h).style(
                    Style::default()
                        .fg(theme.primary)
                        .add_modifier(Modifier::BOLD),
                )
            });
        let header = Row::new(header_cells).height(1);

        let rows: Vec<Row> = self
            .transfers
            .iter()
            .enumerate()
            .map(|(i, xfer)| {
                let style = if i == self.selected_transfer && self.focus_transfers {
                    theme.highlight_style()
                } else {
                    theme.default_style()
                };

                let status_style = match xfer.status {
                    TransferStatus::Complete => Style::default().fg(theme.success),
                    TransferStatus::Active => Style::default().fg(theme.warning),
                    TransferStatus::Queued => Style::default().fg(theme.muted),
                    TransferStatus::Failed => Style::default().fg(theme.error),
                };

                let progress_str = format!("{}%", (xfer.progress * 100.0) as u32);
                let size_str = Self::format_size(xfer.size);

                Row::new(vec![
                    Cell::from(xfer.name.as_str()),
                    Cell::from(xfer.direction.to_string()),
                    Cell::from(progress_str),
                    Cell::from(size_str),
                    Cell::from(Span::styled(xfer.status.to_string(), status_style)),
                ])
                .style(style)
            })
            .collect();

        let table = Table::new(
            rows,
            [
                Constraint::Min(15),
                Constraint::Length(4),
                Constraint::Length(8),
                Constraint::Length(10),
                Constraint::Length(8),
            ],
        )
        .header(header)
        .style(theme.default_style());

        frame.render_widget(table, area);
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
    fn test_comms_panel_new() {
        let panel = CommsPanel::new();
        assert_eq!(panel.title(), "Comms [WRAITH]");
        assert!(panel.peers.is_empty());
        assert!(panel.transfers.is_empty());
    }

    #[test]
    fn test_comms_panel_clear() {
        let mut panel = CommsPanel::new();
        panel.peers.push(PeerInfo {
            name: "peer1".to_string(),
            status: PeerStatus::Connected,
            latency_ms: Some(50),
            cipher: "AES-256-GCM".to_string(),
        });
        panel.clear();
        assert!(panel.peers.is_empty());
    }

    #[test]
    fn test_comms_panel_render_empty() {
        let mut terminal = test_terminal();
        let panel = CommsPanel::new();
        let theme = Theme::dark();

        terminal
            .draw(|frame| {
                let area = frame.area();
                panel.render(frame, area, true, &theme);
            })
            .unwrap();
    }

    #[test]
    fn test_comms_panel_render_with_peers() {
        let mut terminal = test_terminal();
        let mut panel = CommsPanel::new();
        panel.peers.push(PeerInfo {
            name: "alpha".to_string(),
            status: PeerStatus::Connected,
            latency_ms: Some(12),
            cipher: "ChaCha20-Poly1305".to_string(),
        });
        panel.peers.push(PeerInfo {
            name: "bravo".to_string(),
            status: PeerStatus::Disconnected,
            latency_ms: None,
            cipher: "AES-256-GCM".to_string(),
        });
        let theme = Theme::dark();

        terminal
            .draw(|frame| {
                let area = frame.area();
                panel.render(frame, area, true, &theme);
            })
            .unwrap();
    }

    #[test]
    fn test_comms_panel_render_with_transfers() {
        let mut terminal = test_terminal();
        let mut panel = CommsPanel::new();
        panel.peers.push(PeerInfo {
            name: "peer1".to_string(),
            status: PeerStatus::Connected,
            latency_ms: Some(5),
            cipher: "AES-256".to_string(),
        });
        panel.transfers.push(TransferEntry {
            name: "scan_results.json".to_string(),
            direction: TransferDirection::Send,
            progress: 0.75,
            size: 2_048_000,
            status: TransferStatus::Active,
        });
        let theme = Theme::dark();

        terminal
            .draw(|frame| {
                let area = frame.area();
                panel.render(frame, area, false, &theme);
            })
            .unwrap();
    }

    #[test]
    fn test_comms_panel_on_key_nav() {
        let mut panel = CommsPanel::new();
        panel.peers.push(PeerInfo {
            name: "p1".to_string(),
            status: PeerStatus::Connected,
            latency_ms: None,
            cipher: "aes".to_string(),
        });
        panel.peers.push(PeerInfo {
            name: "p2".to_string(),
            status: PeerStatus::Connected,
            latency_ms: None,
            cipher: "aes".to_string(),
        });

        let down = KeyEvent::new(KeyCode::Char('j'), crossterm::event::KeyModifiers::NONE);
        panel.on_key(down);
        assert_eq!(panel.selected_peer, 1);

        let up = KeyEvent::new(KeyCode::Char('k'), crossterm::event::KeyModifiers::NONE);
        panel.on_key(up);
        assert_eq!(panel.selected_peer, 0);
    }

    #[test]
    fn test_comms_panel_tab_toggle() {
        let mut panel = CommsPanel::new();
        assert!(!panel.focus_transfers);

        let tab = KeyEvent::new(KeyCode::Tab, crossterm::event::KeyModifiers::NONE);
        panel.on_key(tab);
        assert!(panel.focus_transfers);

        let tab = KeyEvent::new(KeyCode::Tab, crossterm::event::KeyModifiers::NONE);
        panel.on_key(tab);
        assert!(!panel.focus_transfers);
    }

    #[test]
    fn test_comms_panel_default() {
        let panel = CommsPanel::default();
        assert!(panel.peers.is_empty());
    }

    #[test]
    fn test_peer_status_display() {
        assert_eq!(PeerStatus::Connected.to_string(), "CONNECTED");
        assert_eq!(PeerStatus::Connecting.to_string(), "CONNECTING");
        assert_eq!(PeerStatus::Disconnected.to_string(), "OFFLINE");
        assert_eq!(PeerStatus::Authenticating.to_string(), "AUTHING");
    }

    #[test]
    fn test_transfer_direction_display() {
        assert_eq!(TransferDirection::Send.to_string(), "TX");
        assert_eq!(TransferDirection::Receive.to_string(), "RX");
    }

    #[test]
    fn test_transfer_status_display() {
        assert_eq!(TransferStatus::Queued.to_string(), "QUEUED");
        assert_eq!(TransferStatus::Active.to_string(), "ACTIVE");
        assert_eq!(TransferStatus::Complete.to_string(), "DONE");
        assert_eq!(TransferStatus::Failed.to_string(), "FAILED");
    }

    #[test]
    fn test_format_size() {
        assert_eq!(CommsPanel::format_size(500), "500 B");
        assert_eq!(CommsPanel::format_size(1500), "1.5 KB");
        assert_eq!(CommsPanel::format_size(1_572_864), "1.5 MB");
        assert_eq!(CommsPanel::format_size(1_610_612_736), "1.5 GB");
    }

    #[test]
    fn test_comms_panel_small_area() {
        let backend = TestBackend::new(8, 2);
        let mut terminal = Terminal::new(backend).unwrap();
        let panel = CommsPanel::new();
        let theme = Theme::dark();

        terminal
            .draw(|frame| {
                let area = frame.area();
                panel.render(frame, area, true, &theme);
            })
            .unwrap();
    }
}
