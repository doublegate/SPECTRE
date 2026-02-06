# SPECTRE GUI User Guide

## Table of Contents

1. [Getting Started](#getting-started)
2. [Interface Overview](#interface-overview)
3. [Dashboard](#dashboard)
4. [Reconnaissance](#reconnaissance)
5. [Campaign Management](#campaign-management)
6. [Reports & Analysis](#reports--analysis)
7. [Settings](#settings)
8. [Keyboard Shortcuts](#keyboard-shortcuts)
9. [Troubleshooting](#troubleshooting)

---

## Getting Started

### Installation

SPECTRE GUI is available for Linux, macOS, and Windows. Download the appropriate installer for your platform:

#### Linux

**AppImage (recommended)**:
```bash
# Download and make executable
chmod +x SPECTRE-0.5.0.AppImage
./SPECTRE-0.5.0.AppImage
```

**Debian/Ubuntu (.deb)**:
```bash
sudo dpkg -i SPECTRE_0.5.0_amd64.deb
# Or double-click the .deb file in your file manager
```

**Fedora/RHEL (.rpm)**:
```bash
sudo rpm -i SPECTRE-0.5.0.x86_64.rpm
# Or use dnf: sudo dnf install SPECTRE-0.5.0.x86_64.rpm
```

**Dependencies**: webkit2gtk-4.1, libssl, libpcap
```bash
# Debian/Ubuntu
sudo apt install libwebkit2gtk-4.1-dev libssl-dev libpcap-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel openssl-devel libpcap-devel

# Arch
sudo pacman -S webkit2gtk openssl libpcap
```

#### macOS

**DMG Installer**:
1. Download `SPECTRE-0.5.0.dmg`
2. Double-click to mount
3. Drag SPECTRE.app to Applications folder
4. Right-click → Open (first launch only, to bypass Gatekeeper)

**Requirements**: macOS 10.15+ (Catalina or later)

Available for both Intel and Apple Silicon (M1/M2/M3).

#### Windows

**MSI Installer**:
1. Download `SPECTRE-0.5.0.msi`
2. Double-click to run installer
3. Follow installation wizard
4. Launch from Start Menu

**NSIS Installer** (alternative):
1. Download `SPECTRE-0.5.0-setup.exe`
2. Run installer
3. Accept UAC prompt if requested

**Dependencies**: WebView2 Runtime (bundled), Visual C++ Redistributable

**For raw packet scanning**: Install [Npcap](https://npcap.com/) for Windows packet capture support.

### First Launch

When you first launch SPECTRE GUI:

1. The application window opens with the **Dashboard** view
2. The sidebar shows navigation options (Dashboard, Recon, Campaigns, Reports, Settings)
3. Status bar at the bottom displays system information
4. No scans are running initially

### Platform Requirements

See [PLATFORM-REQUIREMENTS.md](../../crates/spectre-gui/PLATFORM-REQUIREMENTS.md) for detailed system requirements and dependency installation instructions.

---

## Interface Overview

### Main Layout

The SPECTRE GUI uses a consistent layout across all pages:

```
┌─────────────────────────────────────────────────┐
│  SPECTRE                    [Status Indicators]  │  Header
├──────────┬──────────────────────────────────────┤
│          │                                       │
│ Sidebar  │         Main Content Area            │
│          │                                       │
│ - Dash   │                                       │
│ - Recon  │                                       │
│ - Camp   │                                       │
│ - Report │                                       │
│ - Set    │                                       │
│          │                                       │
├──────────┴──────────────────────────────────────┤
│  Status Bar: Active Scans | Memory | Time       │  Status Bar
└─────────────────────────────────────────────────┘
```

**Components**:

- **Header**: Application title, current page, and status indicators
- **Sidebar**: Navigation menu with 5 main sections
- **Content Area**: Main application content (changes based on current page)
- **Status Bar**: Real-time system information and notifications

### Navigation

**Sidebar Menu**:
- **Dashboard** (Alt+1): Overview metrics and recent activity
- **Reconnaissance** (Alt+2): Network scanning interface
- **Campaigns** (Alt+3): Multi-phase security assessments
- **Reports** (Alt+4): Findings, analysis, and exports
- **Settings** (Alt+5): Application configuration

Click any menu item to navigate, or use the keyboard shortcuts (Alt+1 through Alt+5).

### Themes

SPECTRE includes 5 built-in themes matching the TUI interface:

1. **Dark** (default): High-contrast dark theme for extended use
2. **Light**: Bright interface optimized for daylight conditions
3. **Tactical**: Military-inspired green-on-black
4. **Matrix**: Classic green terminal aesthetic
5. **Hacker**: Neon cyan/magenta cyberpunk theme

**To change themes**:
1. Navigate to Settings (Alt+5)
2. Select "Themes" tab
3. Choose your preferred theme
4. Changes apply immediately

---

## Dashboard

The Dashboard provides at-a-glance metrics for your reconnaissance activities.

### Overview Cards

Four metric cards display key statistics:

1. **Total Hosts**: Number of discovered network hosts across all scans
2. **Open Ports**: Count of accessible ports found
3. **Services**: Number of identified network services
4. **Findings**: Total security issues detected (color-coded by severity)

**Severity Color Coding**:
- **Critical**: Red (immediate action required)
- **High**: Orange (important issues)
- **Medium**: Yellow (moderate concern)
- **Low**: Green (minor issues)
- **Info**: Gray (informational only)

### Charts

**Severity Distribution (Pie Chart)**:
- Visual breakdown of findings by severity level
- Hover over sections for exact counts and percentages
- Click legend items to toggle visibility

**Top Services (Bar Chart)**:
- Most common services detected across all scans
- Sorted by frequency (highest first)
- Maximum 10 services displayed
- Hover for exact counts

### Recent Activity Timeline

Chronological list of scan history:
- **Scan Started**: Blue indicator
- **Scan Completed**: Green indicator with duration
- **Scan Failed**: Red indicator with error message

**Features**:
- Click any activity to view full scan results
- Shows target count and duration for completed scans
- Automatically updates when new scans run

---

## Reconnaissance

Network scanning interface powered by ProRT-IP (10M+ packets/sec scanning engine).

### Creating a Scan

1. Navigate to **Reconnaissance** (Alt+2 or sidebar)
2. Click **"New Scan"** button (or press Ctrl+N)
3. Configure scan parameters

### Scan Configuration

**Targets** (required):
- Enter IP addresses, CIDR ranges, or hostnames
- Supports multiple formats:
  - Single IP: `192.168.1.1`
  - CIDR notation: `192.168.1.0/24`
  - Hostname: `example.com`
  - Multiple targets: Comma or newline separated

Example:
```
10.0.0.1
192.168.1.0/24
example.com
scanme.nmap.org
```

**Ports**:
- Common ports (default): `22,80,443,8080,8443`
- Custom ranges: `1-1000` or `80,443,8000-9000`
- All ports: Leave empty and enable in advanced options

**Scan Types**:

| Type | Description | Speed | Stealth | Use Case |
|------|-------------|-------|---------|----------|
| **SYN** | Half-open scan (SYN only) | Fast | High | Stealth scanning, firewall evasion |
| **Connect** | Full TCP handshake | Medium | Low | Reliable, works without raw socket access |
| **UDP** | UDP port scanning | Slow | Medium | Discover UDP services (DNS, SNMP) |
| **ACK** | ACK probe for firewall detection | Fast | High | Firewall rule mapping |
| **FIN** | FIN flag scan | Medium | High | Stealthy alternative to SYN |
| **Xmas** | FIN+PSH+URG flags | Medium | High | IDS evasion |
| **Null** | No flags set | Medium | High | Ultra-stealthy scanning |
| **Window** | TCP window size analysis | Medium | Medium | Advanced OS fingerprinting |

**Timing Templates**:

| Template | Speed | Detection Risk | Use Case |
|----------|-------|----------------|----------|
| **T0 (Paranoid)** | Very Slow | Minimal | IDS evasion, ultra-stealthy |
| **T1 (Sneaky)** | Slow | Low | Avoid detection |
| **T2 (Polite)** | Moderate | Low | Reduce bandwidth usage |
| **T3 (Normal)** | Fast | Medium | Default balanced scan |
| **T4 (Aggressive)** | Very Fast | High | Quick results, less careful |
| **T5 (Insane)** | Extremely Fast | Very High | Speed over accuracy |

**Detection Options**:
- ☑ **Service Detection**: Identify service names and versions (e.g., Apache 2.4.41)
- ☐ **OS Detection**: Fingerprint target operating system (experimental)

### Starting a Scan

1. Configure parameters as described above
2. Click **"Start Scan"** button
3. Scan progress appears in the main content area

**Permissions**:
- Linux/macOS: Raw socket scanning (SYN, FIN, etc.) requires `sudo` or `CAP_NET_RAW` capability
- Windows: Requires Npcap for raw packet access

### Viewing Results

Results are displayed in three views:

#### 1. Network Topology (Visual Map)

Interactive D3.js force-directed graph of discovered hosts:

**Features**:
- Nodes represent discovered hosts (colored by severity)
- Links show subnet relationships (same /24)
- Zoom: Scroll wheel
- Pan: Click and drag background
- Move nodes: Click and drag individual nodes
- Click host: Open detailed view

**Legend**:
- Critical: Red nodes
- High: Orange nodes
- Medium: Yellow nodes
- Low: Green nodes
- Info: Gray nodes

**Controls**:
- Mouse wheel: Zoom in/out
- Click+drag background: Pan the view
- Click+drag node: Reposition host
- Click node: View host details

#### 2. Host Cards (Grid View)

Card-based display of discovered hosts:

Each card shows:
- IP address and hostname (if resolved)
- Status indicator (up/down)
- Open ports count
- Operating system (if detected)
- Services summary
- Severity badge

**Interactions**:
- Click card to expand details
- Filter by severity, status, or service
- Sort by IP, hostname, ports, or severity

#### 3. Results Table (Detailed List)

Sortable, filterable table of all discovered ports:

**Columns**:
- Host: IP address
- Port: Port number
- Protocol: TCP/UDP
- State: Open/Closed/Filtered
- Service: Service name
- Version: Software version
- Severity: Risk level

**Features**:
- **Sort**: Click column headers
- **Filter**: Text search box (searches all columns)
- **Export**: CSV, JSON, XML buttons above table
- **Pagination**: 50 results per page (configurable)

### Active Scans

The status bar shows active scan progress:
- Scan ID and target count
- Current progress percentage
- Estimated time remaining (ETA)
- Stop button (abort scan)

**Multiple Scans**:
- Run up to 5 concurrent scans (configurable in Settings)
- Each scan tracks progress independently
- Results appear in real-time as hosts are discovered

---

## Campaign Management

Organize multi-phase security assessments with the Campaign Manager.

### What is a Campaign?

A **campaign** is a structured security assessment consisting of:
- **Objectives**: Goals and scope
- **Targets**: IP ranges, domains, or hosts
- **Phases**: Reconnaissance → Scanning → Analysis → Exfiltration
- **Timeline**: Start and end dates
- **Artifacts**: Scan results, reports, exported data

### Creating a Campaign

1. Navigate to **Campaigns** (Alt+3)
2. Click **"Create Campaign"** button
3. Fill out the campaign wizard:

#### Step 1: Basic Information

- **Name**: Descriptive campaign title (e.g., "Q1 2026 Internal Network Audit")
- **Description**: Purpose and scope notes
- **Start Date**: Campaign start date
- **End Date**: Expected completion date

#### Step 2: Objectives

Define what you want to achieve:
- Enumerate all hosts in target subnet
- Identify critical vulnerabilities
- Map network topology
- Detect unauthorized services

**Format**: Bulleted list or numbered list. Example:
```
1. Discover all active hosts in 10.0.0.0/16
2. Identify services on critical infrastructure
3. Detect outdated software versions
4. Map firewall rules via ACK scanning
```

#### Step 3: Targets

Enter target IP ranges or hostnames:
- **Import from file**: Upload CSV/TXT with one target per line
- **Paste targets**: Enter manually (comma or newline separated)
- **CIDR ranges**: `10.0.0.0/16`, `192.168.0.0/24`
- **Individual IPs**: `10.0.0.1`, `10.0.0.254`
- **Hostnames**: `router.local`, `gateway.example.com`

**Target Validation**: Automatically checks format and resolves hostnames.

#### Step 4: Review and Create

- Review all settings
- Click **"Create Campaign"** to finalize
- Campaign appears in the list

### Managing Campaigns

**Campaign List View**:

Each campaign card displays:
- Name and description
- Current phase indicator
- Progress bar (% complete)
- Target count
- Created date
- Status badge (Active/Paused/Complete/Archived)

**Actions**:
- Click card: Open campaign detail view
- Edit button: Modify campaign settings
- Archive button: Archive completed campaigns

### Campaign Detail View

**Tabs**:

1. **Overview**: Summary, objectives, targets, timeline
2. **Phases**: Phase progression with checkmarks
3. **Scans**: Associated scan results
4. **Artifacts**: Exported reports and data files
5. **Notes**: Campaign notes and observations

### Campaign Phases

Campaigns progress through 4 phases:

#### 1. Reconnaissance (Planning)

- Define scope and objectives
- Identify targets
- Plan scan strategy
- Status: **Planning**

#### 2. Scanning (Execution)

- Run network scans
- Collect data
- Identify services and vulnerabilities
- Status: **In Progress**

**Actions**:
- Start new scan within campaign context
- View scan results
- Track progress

#### 3. Analysis (Processing)

- Analyze scan results
- Transform data with CyberChef operations
- Generate reports
- Status: **Analyzing**

**CyberChef Integration**:
- Select scan results
- Apply data transformations (decode, decrypt, parse)
- Save processed artifacts

#### 4. Exfiltration (Secure Transfer)

- Export campaign data
- Securely transfer via WRAITH protocol
- Archive final deliverables
- Status: **Complete**

**Export Options**:
- Full campaign archive (ZIP)
- Individual scan results (CSV, JSON, XML)
- Executive summary (HTML, Markdown, PDF)

### Advancing Phases

1. Complete all tasks in current phase
2. Click **"Advance to Next Phase"** button
3. Confirm transition
4. New phase becomes active

**Phase Gates**: Cannot skip phases. Must complete in order:
Reconnaissance → Scanning → Analysis → Exfiltration

### Campaign Export

Export complete campaign data:

1. Open campaign detail view
2. Click **"Export Campaign"** button
3. Select format:
   - **ZIP Archive**: All artifacts, scans, reports
   - **JSON**: Structured campaign data
   - **HTML Report**: Formatted summary with charts

**Export Contents**:
- Campaign metadata (objectives, timeline)
- All scan results (full data)
- Generated reports (HTML, Markdown)
- Notes and observations
- Target list with resolution status

### Campaign Import

Import previously exported campaigns:

1. Navigate to Campaigns page
2. Click **"Import Campaign"** button
3. Select campaign archive (ZIP or JSON)
4. Review import preview
5. Click **"Import"** to restore

**Use Cases**:
- Transfer campaigns between systems
- Backup campaign data
- Share with team members
- Archive historical assessments

### Archiving Campaigns

Archive completed campaigns to reduce clutter:

1. Open campaign or select from list
2. Click **"Archive Campaign"**
3. Confirm archiving

**Archived Campaigns**:
- Moved to "Archived" view (toggle filter)
- Read-only (cannot modify)
- Can be restored or permanently deleted
- Excluded from active campaign count

---

## Reports & Analysis

### Findings Overview

The Reports page displays all discovered security issues across all scans and campaigns.

**Findings Table**:

| Column | Description |
|--------|-------------|
| Severity | Critical/High/Medium/Low/Info (color-coded) |
| Host | Target IP address |
| Port | Port number |
| Service | Service name (e.g., http, ssh, mysql) |
| Version | Software version if detected |
| CVEs | Related CVE identifiers (if applicable) |
| Timestamp | Discovery date/time |

**Sorting**:
- Click column headers to sort (ascending/descending)
- Default: Severity (Critical first), then Host (alphabetical)

**Filtering**:
- **Severity**: Select one or more severity levels
- **Service**: Filter by service type
- **Host**: Enter IP or hostname
- **Date Range**: Limit to specific time period

**Search**:
- Global search box (Ctrl+F)
- Searches across all columns
- Real-time filtering as you type

### Finding Details

Click any finding to open the detail modal:

**Information Displayed**:

1. **Host Information**
   - IP Address: Full IPv4/IPv6 address
   - Port: Port number and protocol (TCP/UDP)
   - Protocol: Application protocol (HTTP, SSH, etc.)
   - Discovered: Relative time (e.g., "2 hours ago")

2. **Service Details**
   - Service Name: Identified service
   - Version: Software version and build
   - Banner: Raw service banner (if captured)

3. **Description**
   - Detailed finding description
   - Why this matters (risk explanation)
   - Potential impact

4. **Related CVEs**
   - CVE identifiers (e.g., CVE-2024-1234)
   - Clickable links to NVD (National Vulnerability Database)
   - CVSS scores (if available)

5. **Remediation**
   - Recommended fixes
   - Patching instructions
   - Mitigation steps

**Actions**:
- **Export JSON**: Download finding as JSON
- **Export PDF**: Generate PDF report (single finding)
- **Close** (Esc): Close detail modal

**Accessibility**:
- Keyboard navigation: Tab through elements
- Esc key: Close modal
- Focus trap: Keeps focus within modal
- Screen reader support: Full ARIA labels

### Report Generation

Generate comprehensive reports for findings:

1. Select findings (checkboxes or "Select All")
2. Click **"Generate Report"** button
3. Choose report format:
   - **HTML**: Interactive web report
   - **Markdown**: Plain text with formatting
   - **CSV**: Spreadsheet-compatible
   - **JSON**: Structured data
   - **XML**: Machine-readable format

**Report Sections**:
- **Executive Summary**: High-level overview, risk summary
- **Methodology**: Scan types, targets, timeframe
- **Findings by Severity**: Grouped and sorted
- **Detailed Results**: Full finding descriptions
- **Recommendations**: Prioritized remediation steps
- **Appendix**: Technical details, raw data

**Preview**:
- HTML and Markdown reports support live preview
- Click **"Preview"** to see report before export
- Sanitized with DOMPurify for security

### Export Options

**CSV Export**:
```csv
Severity,Host,Port,Service,Version,Timestamp
critical,192.168.1.1,22,ssh,OpenSSH 5.3,2026-02-06T10:30:00Z
high,192.168.1.10,80,http,Apache 2.2.15,2026-02-06T10:31:00Z
```

**JSON Export**:
```json
{
  "findings": [
    {
      "id": "f1234",
      "severity": "critical",
      "host": "192.168.1.1",
      "port": 22,
      "service": "ssh",
      "version": "OpenSSH 5.3",
      "cves": ["CVE-2023-12345"],
      "description": "Outdated SSH server with known vulnerabilities",
      "remediation": "Upgrade to OpenSSH 9.0 or later",
      "timestamp": "2026-02-06T10:30:00Z"
    }
  ],
  "metadata": {
    "generated": "2026-02-06T11:00:00Z",
    "total_findings": 42,
    "critical": 3,
    "high": 12,
    "medium": 18,
    "low": 7,
    "info": 2
  }
}
```

**XML Export**:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<report>
  <metadata>
    <generated>2026-02-06T11:00:00Z</generated>
    <total_findings>42</total_findings>
  </metadata>
  <findings>
    <finding>
      <severity>critical</severity>
      <host>192.168.1.1</host>
      <port>22</port>
      <service>ssh</service>
      <version>OpenSSH 5.3</version>
    </finding>
  </findings>
</report>
```

### Incremental Export Tracking

SPECTRE tracks what has been exported to avoid duplicates:

- Exported findings are marked with timestamp
- Filter: "Only show new findings" (hide previously exported)
- Reset tracking: "Clear export history"

---

## Settings

Configure SPECTRE GUI preferences and component integrations.

Navigate to **Settings** (Alt+5) to access 8 configuration tabs.

### 1. General Settings

**Application Behavior**:
- **Verbosity Level**: None, Error, Warn, Info, Debug, Trace
- **Color Output**: Enable/disable ANSI color codes
- **Startup Behavior**: Restore last session, open dashboard, custom

**Logging**:
- **Log to File**: Enable file logging
- **Log Path**: Location for log files (default: `~/.spectre/logs/`)
- **Max Log Size**: Rotate logs at size limit (MB)

### 2. Scan Settings

**Default Scan Configuration**:
- **Default Timing**: T0-T5 (default: T3 Normal)
- **Port Range Presets**: Common, All, Custom
- **Service Detection**: Enable by default (checkbox)
- **OS Detection**: Enable by default (checkbox)

**Performance**:
- **Concurrent Scans**: Maximum simultaneous scans (1-10, default: 5)
- **Connection Timeout**: Socket timeout in seconds (1-60, default: 5)
- **Packet Rate Limit**: Max packets/sec (1000-10000000, default: 10000000)

**ProRT-IP Integration**:
- **Path to ProRT-IP**: Binary location (auto-detected)
- **Enable AF_XDP**: Use XDP for kernel bypass (Linux only)
- **Enable io_uring**: Use io_uring for async I/O (Linux 5.1+)

### 3. Analysis Settings

**CyberChef Integration**:
- **Docker Container**: Use Docker image (recommended)
- **Container Name**: `cyberchef-mcp` (default)
- **Auto-start**: Launch container on app start
- **MCP Server**: Connect via MCP protocol (stdio or socket)

**Favorite Operations**:
Select frequently used CyberChef operations for quick access:
- From Base64
- To Hex
- AES Decrypt
- Decode Text
- Parse JSON
- ... (15 total available)

### 4. Comms Settings

**WRAITH Protocol Configuration**:
- **Identity Path**: Cryptographic identity file location
- **Peer Database**: Trusted peers database path
- **Default Port**: WRAITH listener port (default: 9001)

**Encryption**:
- **Cipher Suite**: ChaCha20-Poly1305, AES-256-GCM
- **Perfect Forward Secrecy**: Enable Diffie-Hellman key exchange
- **Post-Quantum**: Enable Kyber1024 hybrid encryption (experimental)

**Network**:
- **Max Throughput**: 10+ Gbps E2EE (auto-detected)
- **Buffer Size**: Send/receive buffer (KB)

### 5. Output Settings

**Default Export Format**:
- JSON (structured)
- CSV (spreadsheet)
- XML (machine-readable)
- HTML (formatted report)
- Markdown (documentation)

**Formatting**:
- **Pretty Print**: Enable JSON/XML indentation
- **Include Metadata**: Add generation timestamp, version info
- **Timestamps**: ISO 8601 or Unix epoch

**File Naming**:
- **Template**: `{scan_id}_{timestamp}.{format}` (customizable)
- **Save Location**: Default export directory

### 6. Themes

**Theme Selector**:

Select from 5 built-in themes:

1. **Dark** (default)
   - Background: Near-black (`#12121c`)
   - Foreground: Light gray (`#c8c8dc`)
   - Primary: Cornflower blue (`#6495ed`)
   - Use Case: Extended use, reduces eye strain

2. **Light**
   - Background: Off-white (`#f5f5fa`)
   - Foreground: Dark blue (`#1e1e32`)
   - Primary: Midnight blue (`#191970`)
   - Use Case: Daylight conditions, presentations

3. **Tactical**
   - Background: Dark green (`#0a0f0a`)
   - Foreground: Light green (`#b4c8b4`)
   - Primary: Forest green (`#228b22`)
   - Use Case: Military aesthetic, terminal feel

4. **Matrix**
   - Background: Pure black (`#000000`)
   - Foreground: Bright green (`#00ff00`)
   - Primary: Lime green (`#00c800`)
   - Use Case: Classic hacker aesthetic

5. **Hacker**
   - Background: Dark blue-black (`#0a0a14`)
   - Foreground: Cyan (`#00e6e6`)
   - Primary: Deep pink (`#ff0080`)
   - Use Case: Cyberpunk aesthetic, neon theme

**Preview**:
- Live preview updates as you select themes
- Changes apply immediately (no restart required)

**Custom Themes** (planned):
- Import/export custom color schemes
- Theme editor with color picker
- Share themes with community

### 7. Keyboard Shortcuts

**Complete Reference Table**:

| Action | Shortcut | Description |
|--------|----------|-------------|
| **Navigation** | | |
| Dashboard | Alt+1 | Go to Dashboard page |
| Reconnaissance | Alt+2 | Go to Recon page |
| Campaigns | Alt+3 | Go to Campaigns page |
| Reports | Alt+4 | Go to Reports page |
| Settings | Alt+5 | Go to Settings page |
| **Actions** | | |
| New Scan | Ctrl+N | Create new scan |
| Save | Ctrl+S | Save current work |
| Search | Ctrl+F | Search current page |
| **Help** | | |
| Open Help | F1 | Open help documentation |
| **Modal Controls** | | |
| Close Modal | Esc | Close open dialog/modal |
| Confirm | Enter | Confirm action |
| Cancel | Esc | Cancel action |

**Custom Keybindings** (planned):
- Remap shortcuts to custom keys
- Import/export keybinding profiles
- Vim-style navigation mode

### 8. About

**Version Information**:
- **SPECTRE GUI**: v0.5.0
- **Build Date**: 2026-02-06
- **Git Commit**: SHA hash

**Component Versions**:
- **ProRT-IP**: v1.0.0 (network scanner)
- **CyberChef-MCP**: v1.9.0 (data analysis)
- **WRAITH Protocol**: v2.3.7 (secure comms)

**System Information**:
- **Operating System**: Detected platform
- **Architecture**: x86_64, aarch64, etc.
- **Tauri**: v2.10
- **WebView**: Version and renderer

**License**:
- SPECTRE CLI: MIT License
- ProRT-IP: GPLv3
- CyberChef-MCP: Apache 2.0
- WRAITH Protocol: MIT License

**Credits**:
- Author and contributors
- Third-party dependencies
- Special thanks

**Links**:
- [GitHub Repository](https://github.com/doublegate/SPECTRE)
- [Report Issues](https://github.com/doublegate/SPECTRE/issues)
- [Documentation](https://github.com/doublegate/SPECTRE/tree/main/docs)

---

## Keyboard Shortcuts

Complete reference for keyboard navigation.

### Global Navigation

| Shortcut | Action |
|----------|--------|
| Alt+1 | Navigate to Dashboard |
| Alt+2 | Navigate to Reconnaissance |
| Alt+3 | Navigate to Campaigns |
| Alt+4 | Navigate to Reports |
| Alt+5 | Navigate to Settings |

### Actions

| Shortcut | Action |
|----------|--------|
| Ctrl+N | New Scan (when on Recon page) |
| Ctrl+S | Save current work |
| Ctrl+F | Search current page |
| Enter | Submit form / Confirm action |
| Esc | Close modal / Cancel action |

### Help

| Shortcut | Action |
|----------|--------|
| F1 | Open help documentation |

### Modal Navigation

When a modal dialog is open:

| Shortcut | Action |
|----------|--------|
| Tab | Move to next interactive element |
| Shift+Tab | Move to previous interactive element |
| Enter | Confirm / Submit |
| Esc | Close modal |
| Space | Toggle checkboxes |

### Accessibility

| Shortcut | Action |
|----------|--------|
| Tab | Navigate forward through page elements |
| Shift+Tab | Navigate backward through page elements |
| Enter | Activate buttons and links |
| Space | Toggle checkboxes and radio buttons |
| Arrow Keys | Navigate within dropdown menus |

---

## Troubleshooting

### Application Won't Start

#### Linux

**Missing Dependencies**:
```bash
# Debian/Ubuntu - check webkit2gtk-4.1
pkg-config --exists webkit2gtk-4.1 && echo "OK" || echo "MISSING"

# Install missing dependencies
sudo apt install libwebkit2gtk-4.1-dev libssl-dev libpcap-dev
```

**AppImage Permissions**:
```bash
# Make executable
chmod +x SPECTRE-0.5.0.AppImage

# If FUSE is not available
./SPECTRE-0.5.0.AppImage --appimage-extract
./squashfs-root/AppRun
```

**Wayland Issues**:
```bash
# Force X11 backend if Wayland causes problems
GDK_BACKEND=x11 ./SPECTRE-0.5.0.AppImage
```

#### macOS

**Gatekeeper Blocking App**:
1. Right-click SPECTRE.app
2. Select "Open"
3. Click "Open" in security dialog
4. (First launch only)

**Or disable Gatekeeper for this app**:
```bash
sudo xattr -rd com.apple.quarantine /Applications/SPECTRE.app
```

**Missing Xcode Command Line Tools**:
```bash
xcode-select --install
```

#### Windows

**WebView2 Runtime Missing**:
- Download from: https://developer.microsoft.com/microsoft-edge/webview2/
- Or use MSI installer (bundles WebView2)

**Visual C++ Redistributable**:
- Download: https://aka.ms/vs/17/release/vc_redist.x64.exe
- Install and restart

### Scans Fail to Start

**Linux: Permission Denied**:

Raw socket scanning requires elevated privileges:

```bash
# Option 1: Run with sudo
sudo SPECTRE-0.5.0.AppImage

# Option 2: Add CAP_NET_RAW capability (AppImage extract first)
./SPECTRE-0.5.0.AppImage --appimage-extract
sudo setcap cap_net_raw+ep ./squashfs-root/AppRun
./squashfs-root/AppRun
```

**Windows: Npcap Not Installed**:

1. Download Npcap: https://npcap.com/
2. Install with "WinPcap API-compatible mode" enabled
3. Restart SPECTRE

**ProRT-IP Not Found**:

1. Go to Settings → Scan Settings
2. Check "Path to ProRT-IP"
3. If empty or invalid, specify correct path:
   - Linux: `/usr/bin/prtip` or `./components/prtip/target/release/prtip`
   - macOS: `/usr/local/bin/prtip` or similar
   - Windows: `C:\Program Files\SPECTRE\prtip.exe`

**Target Validation Errors**:

Ensure targets are in correct format:
- Valid IP: `192.168.1.1`
- Valid CIDR: `192.168.1.0/24` (not `/32` for single host)
- Valid hostname: `example.com` (resolves to IP)

### Performance Issues

**Slow Scans**:

1. Reduce concurrency:
   - Settings → Scan Settings → Concurrent Scans (lower to 1-2)

2. Use slower timing template:
   - T0 (Paranoid) or T1 (Sneaky) instead of T4/T5

3. Scan fewer hosts:
   - Break large CIDR ranges into smaller subnets
   - `/16` → multiple `/24` scans

**High Memory Usage**:

1. Reduce concurrent scans (Settings → Scan Settings)
2. Close unused browser tabs (if using web view)
3. Restart application to clear caches

**UI Freezing/Laggy**:

1. Disable animations:
   - Settings → General → Reduce Animations (planned feature)

2. Use lighter theme:
   - Settings → Themes → Light (fewer transparency effects)

3. Reduce network topology nodes:
   - Only scan specific targets instead of entire subnets

### Network Issues

**Can't Connect to Docker (CyberChef)**:

```bash
# Check if Docker is running
docker ps

# Check if CyberChef container exists
docker ps -a | grep cyberchef-mcp

# Restart container
docker restart cyberchef-mcp

# Pull latest image
docker pull doublegate/cyberchef-mcp:latest
```

**WRAITH Protocol Connection Failed**:

1. Check identity file exists:
   - Settings → Comms Settings → Identity Path
   - Generate new identity if missing

2. Verify peer is reachable:
   - Ping target host
   - Check firewall rules (port 9001 default)

3. Check WRAITH logs:
   - `~/.spectre/logs/wraith.log`

### Finding Results

**No Findings Displayed**:

1. **Check Filters**:
   - Reports → Clear all filters
   - Ensure severity checkboxes are enabled

2. **Verify Scan Completed**:
   - Dashboard → Recent Activity
   - Look for "Scan Completed" (green indicator)

3. **Check Export History**:
   - Disable "Hide Exported Findings" filter

**Scan Shows "0 Hosts Discovered"**:

1. **Network Connectivity**:
   - Verify you can ping target hosts
   - Check network interface is correct

2. **Firewall Rules**:
   - Ensure outbound traffic is allowed
   - Host firewall may be blocking ICMP/TCP

3. **Target Online**:
   - Targets may be offline or blocking ICMP echo
   - Try Connect scan (T) instead of SYN (S)

### Export/Report Issues

**Export Fails**:

1. **Check Disk Space**:
   - Ensure sufficient space in export directory
   - Default: `~/Downloads/`

2. **Permissions**:
   - Verify write permissions to export directory
   - Try changing export location (Settings → Output)

3. **File Path Too Long** (Windows):
   - Shorten file name template
   - Use shorter campaign/scan names

**Report Preview Not Loading**:

1. **DOMPurify Error**:
   - Report may contain invalid HTML
   - Export as JSON/CSV instead

2. **Large Report**:
   - Preview limited to 1000 findings
   - Export full report instead

### Getting Help

If issues persist:

1. **Check Logs**:
   ```bash
   # Linux/macOS
   tail -f ~/.spectre/logs/gui.log

   # Windows
   type %APPDATA%\SPECTRE\logs\gui.log
   ```

2. **Report Issue on GitHub**:
   - https://github.com/doublegate/SPECTRE/issues
   - Include: OS, version, error logs, steps to reproduce

3. **Platform Requirements**:
   - Review [PLATFORM-REQUIREMENTS.md](../../crates/spectre-gui/PLATFORM-REQUIREMENTS.md)
   - Ensure all dependencies are installed

4. **Reset Configuration**:
   ```bash
   # Backup first
   cp ~/.spectre/config.toml ~/.spectre/config.toml.backup

   # Remove config (regenerates defaults)
   rm ~/.spectre/config.toml
   ```

---

**For Developer Documentation**: See [GUI-DEVELOPMENT.md](../development/GUI-DEVELOPMENT.md)

**For Component Integration**: See docs in `docs/integration/` directory

**For Architecture Details**: See [SYSTEM-DESIGN.md](../architecture/SYSTEM-DESIGN.md)
