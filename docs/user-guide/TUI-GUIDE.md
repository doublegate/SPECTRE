# SPECTRE TUI Guide

**Version:** 0.1.0 | **Last Updated:** 2026-02-04

---

## Overview

The SPECTRE Terminal User Interface (TUI) provides a real-time operational dashboard for monitoring and controlling security operations directly from the terminal. Built on the ProRT-IP 60 FPS TUI framework, it offers responsive visualization and keyboard-driven operation.

---

## Launching the TUI

```bash
# Launch full dashboard
spectre --tui

# Launch with specific campaign
spectre --tui --campaign "Operation BLACKOUT"

# Launch directly into scan mode
spectre scan --tui 192.168.1.0/24
```

---

## Dashboard Layout

```text
┌─────────────────────────────────────────────────────────────────────────────────┐
│ SPECTRE v0.1.0               Campaign: Operation BLACKOUT        [F1] Help     │
├───────────────────────────────────────┬─────────────────────────────────────────┤
│           RECON PANEL [F2]            │           ANALYSIS PANEL [F3]           │
│                                       │                                         │
│                                       │                                         │
│                                       │                                         │
│                                       │                                         │
│                                       │                                         │
│                                       │                                         │
│                                       │                                         │
├───────────────────────────────────────┼─────────────────────────────────────────┤
│           COMMS PANEL [F4]            │         CAMPAIGN PANEL [F5]             │
│                                       │                                         │
│                                       │                                         │
│                                       │                                         │
│                                       │                                         │
│                                       │                                         │
├─────────────────────────────────────────────────────────────────────────────────┤
│ [s]can [a]nalyze [t]ransfer [r]eport | Status: GREEN | CPU: 15% | MEM: 512MB   │
└─────────────────────────────────────────────────────────────────────────────────┘
```

---

## Panels

### Recon Panel (F2)

Displays network scanning status and results.

```text
┌─ RECON PANEL ─────────────────────────────────────────────────┐
│ Target: 192.168.1.0/24                                        │
│ Scan:   SYN + Service Detection                               │
│                                                               │
│ Progress: ████████████████░░░░░░░░ 67%                        │
│ Rate:     45,231 pps                                          │
│ Hosts:    171 / 254 scanned                                   │
│ Ports:    1,247 open                                          │
│ Services: 89 identified                                       │
│                                                               │
│ ┌─PORT───STATE──SERVICE──────VERSION───────────────────────┐  │
│ │ 22    open   ssh        OpenSSH 8.9p1 Ubuntu             │  │
│ │ 80    open   http       nginx 1.18.0                     │  │
│ │ 443   open   https      nginx 1.18.0                     │  │
│ │ 3306  open   mysql      MySQL 8.0.28                     │  │
│ │ 8080  open   http-proxy Apache Tomcat 9.0.56             │  │
│ │ 9200  open   http       Elasticsearch 7.17.0             │  │
│ └────────────────────────────────────────────────────────────┘  │
│                                                               │
│ [s] New scan  [p] Pause  [x] Stop  [f] Filter  [e] Export   │
└───────────────────────────────────────────────────────────────┘
```

**Controls:**
| Key | Action |
|-----|--------|
| `s` | Start new scan |
| `p` | Pause/resume scan |
| `x` | Stop scan |
| `f` | Filter results |
| `e` | Export results |
| `j`/`↓` | Navigate down |
| `k`/`↑` | Navigate up |
| `Enter` | View details |
| `/` | Search |

### Analysis Panel (F3)

CyberChef analysis operations and results.

```text
┌─ ANALYSIS PANEL ──────────────────────────────────────────────┐
│ Recipe: @decode-credentials                                   │
│ Input:  banners.txt (2.4 MB)                                  │
│ Status: Processing...                                         │
│                                                               │
│ Progress: ████████████████████░░░░ 78%                        │
│ Speed:    1.2 MB/s                                            │
│ ETA:      00:00:32                                            │
│                                                               │
│ ┌─ Output Preview ───────────────────────────────────────────┐  │
│ │ admin:password123                                          │  │
│ │ root:toor                                                  │  │
│ │ user:hunter2                                               │  │
│ │ service:P@ssw0rd!                                          │  │
│ │ backup:backup2024                                          │  │
│ │ mysql:mysql_admin                                          │  │
│ │ ...                                                        │  │
│ └────────────────────────────────────────────────────────────┘  │
│                                                               │
│ [r] Recipe  [i] Input  [Enter] Run  [c] Clear  [s] Save      │
└───────────────────────────────────────────────────────────────┘
```

**Controls:**
| Key | Action |
|-----|--------|
| `r` | Select recipe |
| `i` | Select input file |
| `Enter` | Execute analysis |
| `c` | Clear output |
| `s` | Save output |
| `l` | Load from scan |

### Comms Panel (F4)

WRAITH secure communications status.

```text
┌─ COMMS PANEL ─────────────────────────────────────────────────┐
│ Identity: f3a9c2b1...4d8e7f                                   │
│ Status:   Online                                              │
│ Peers:    3 connected                                         │
│                                                               │
│ ┌─ Active Channels ──────────────────────────────────────────┐  │
│ │ PEER          PROTOCOL    STATUS    TX/RX                 │  │
│ │ c2-server     TLS 1.3     ✓ Active  1.2 GB / 45 MB        │  │
│ │ analyst-1     WebSocket   ✓ Active  200 MB / 12 MB        │  │
│ │ backup        DoH         ✓ Active  50 MB / 5 MB          │  │
│ └────────────────────────────────────────────────────────────┘  │
│                                                               │
│ ┌─ Transfer Queue ───────────────────────────────────────────┐  │
│ │ 1. report.pdf → c2-server    [████████░░] 78%             │  │
│ │ 2. findings.json → analyst-1 [Pending]                    │  │
│ └────────────────────────────────────────────────────────────┘  │
│                                                               │
│ [c] Connect  [d] Disconnect  [t] Transfer  [l] List peers    │
└───────────────────────────────────────────────────────────────┘
```

**Controls:**
| Key | Action |
|-----|--------|
| `c` | Connect to peer |
| `d` | Disconnect peer |
| `t` | Start transfer |
| `l` | List all peers |
| `a` | Add new peer |

### Campaign Panel (F5)

Campaign status and timeline.

```text
┌─ CAMPAIGN PANEL ──────────────────────────────────────────────┐
│ Campaign: Operation BLACKOUT                                  │
│ Phase:    RECON ▸ ANALYSIS                                    │
│ Status:   ACTIVE                                              │
│ Duration: 2h 35m                                              │
│                                                               │
│ ┌─ Phase Progress ───────────────────────────────────────────┐  │
│ │ [✓] PLANNING    [▶] RECON       [ ] ANALYSIS              │  │
│ │ [ ] EXPLOIT     [ ] EXFIL       [ ] REPORTING             │  │
│ └────────────────────────────────────────────────────────────┘  │
│                                                               │
│ ┌─ Timeline ─────────────────────────────────────────────────┐  │
│ │ 14:00  Campaign started                                    │  │
│ │ 14:05  RECON phase initiated                               │  │
│ │ 14:15  171 hosts discovered                                │  │
│ │ 14:22  89 services identified                              │  │
│ │ 14:30  Analysis started                                    │  │
│ │ 14:35  ◀ Current                                           │  │
│ └────────────────────────────────────────────────────────────┘  │
│                                                               │
│ [n] New  [p] Pause  [r] Resume  [a] Abort  [e] Export        │
└───────────────────────────────────────────────────────────────┘
```

**Controls:**
| Key | Action |
|-----|--------|
| `n` | New campaign |
| `p` | Pause campaign |
| `r` | Resume campaign |
| `a` | Abort campaign |
| `e` | Export campaign |

---

## Global Keyboard Shortcuts

### Navigation

| Key | Action |
|-----|--------|
| `F1` | Show help overlay |
| `F2` | Focus Recon panel |
| `F3` | Focus Analysis panel |
| `F4` | Focus Comms panel |
| `F5` | Focus Campaign panel |
| `Tab` | Cycle panel focus |
| `Shift+Tab` | Reverse cycle |
| `F10` / `q` | Quit application |

### Quick Actions

| Key | Action |
|-----|--------|
| `s` | Start scan |
| `a` | Run analysis |
| `t` | Transfer file |
| `r` | Generate report |
| `/` | Command palette |
| `:` | Command mode |
| `?` | Show shortcuts |

### Within Panels

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `h` / `←` | Move left |
| `l` / `→` | Move right |
| `Enter` | Select/activate |
| `Esc` | Cancel/back |
| `g` | Go to top |
| `G` | Go to bottom |
| `Ctrl+d` | Page down |
| `Ctrl+u` | Page up |

---

## Command Mode

Press `:` to enter command mode (vim-style):

```
:scan 192.168.1.0/24 -sS -p 1-1000
:chef @decode-credentials input.txt
:send report.pdf @c2-server
:campaign new "Operation NIGHTFALL"
:set theme dark
:export results.json
:help
:quit
```

### Available Commands

| Command | Description |
|---------|-------------|
| `:scan <target> [opts]` | Start scan |
| `:chef <recipe> [file]` | Run analysis |
| `:send <file> <peer>` | Send file |
| `:receive` | Start receiving |
| `:campaign <cmd>` | Campaign operations |
| `:set <key> <val>` | Set option |
| `:theme <name>` | Change theme |
| `:export <file>` | Export current view |
| `:clear` | Clear current panel |
| `:help [topic]` | Show help |
| `:quit` / `:q` | Quit application |

---

## Command Palette

Press `/` to open the command palette for fuzzy search:

```text
┌─ Command Palette ─────────────────────────────────────────────┐
│ > scan                                                        │
│                                                               │
│   [s] Start SYN scan                                          │
│   [S] Start scan wizard                                       │
│   [p] Pause current scan                                      │
│   [x] Stop current scan                                       │
│   [e] Export scan results                                     │
│   [i] Import targets                                          │
│                                                               │
│ ↑↓ Navigate  Enter Select  Esc Cancel                         │
└───────────────────────────────────────────────────────────────┘
```

---

## Themes

SPECTRE TUI supports multiple themes.

### Built-in Themes

| Theme | Description |
|-------|-------------|
| `dark` | Dark background, green accents (default) |
| `light` | Light background, dark text |
| `tactical` | Military-style green on black |
| `matrix` | Green text on black |
| `hacker` | Amber text on dark |

### Changing Themes

```
:set theme tactical
```

Or in config file:
```toml
[tui]
theme = "tactical"
```

### Custom Theme

Create `~/.config/spectre/themes/custom.toml`:

```toml
[colors]
background = "#1a1a2e"
foreground = "#eaeaea"
primary = "#00ff88"
secondary = "#ff6b6b"
accent = "#4ecdc4"
success = "#00ff00"
warning = "#ffff00"
error = "#ff0000"
muted = "#666666"

[borders]
style = "rounded"  # rounded, double, single, thick
color = "#333333"

[widgets]
progress_filled = "█"
progress_empty = "░"
checkbox_checked = "✓"
checkbox_unchecked = "○"
```

---

## Layout Customization

### Resize Panels

Use `Ctrl+Arrow` to resize the focused panel:

| Keys | Action |
|------|--------|
| `Ctrl+↑` | Increase height |
| `Ctrl+↓` | Decrease height |
| `Ctrl+←` | Decrease width |
| `Ctrl+→` | Increase width |

### Preset Layouts

```
:layout default     # 4-panel grid
:layout wide        # 2 columns
:layout tall        # 2 rows
:layout focus       # Single panel maximized
```

### Save Custom Layout

```
:layout save my-layout
:layout load my-layout
```

---

## Workflow Examples

### Quick Network Scan

1. Press `F2` to focus Recon panel
2. Press `s` to start new scan
3. Enter target: `192.168.1.0/24`
4. Select scan type: `SYN`
5. Watch progress in real-time
6. Press `e` to export when complete

### Analyze Scan Results

1. Complete a scan (above)
2. Press `F3` to focus Analysis panel
3. Press `l` to load scan results
4. Press `r` to select recipe
5. Choose `@extract-iocs`
6. Press `Enter` to run
7. Press `s` to save output

### Secure Exfiltration

1. Press `F4` to focus Comms panel
2. Press `c` to connect to peer
3. Enter peer ID or alias
4. Press `t` to transfer
5. Select file to send
6. Monitor progress

### Campaign Execution

1. Press `F5` to focus Campaign panel
2. Press `n` for new campaign
3. Enter campaign details
4. Define workflow phases
5. Press `Enter` to start
6. Monitor progress across panels

---

## Troubleshooting

### TUI not displaying correctly

```bash
# Check terminal capabilities
echo $TERM

# Try forcing 256 colors
TERM=xterm-256color spectre --tui

# Check encoding
locale
```

### Keys not responding

```bash
# Check for key conflicts
stty -a

# Reset terminal
reset
```

### Performance issues

```toml
# In ~/.config/spectre/spectre.toml
[tui]
fps_limit = 30        # Reduce from 60
unicode = false       # Use ASCII
animations = false    # Disable animations
```

---

## Configuration

### TUI Settings

```toml
# ~/.config/spectre/spectre.toml

[tui]
theme = "dark"
fps_limit = 60
unicode = true
animations = true
mouse_support = true

[tui.layout]
default = "grid"
save_on_exit = true

[tui.panels]
recon = { visible = true, position = "top-left" }
analysis = { visible = true, position = "top-right" }
comms = { visible = true, position = "bottom-left" }
campaign = { visible = true, position = "bottom-right" }
```

---

## Tips

1. **Use command mode** (`:`) for complex operations
2. **Use the palette** (`/`) for quick fuzzy search
3. **Tab between panels** for rapid navigation
4. **Maximize a panel** with `:layout focus` when needed
5. **Export often** to preserve findings
6. **Set up aliases** for common peers: `:peers add <id> --alias c2`
