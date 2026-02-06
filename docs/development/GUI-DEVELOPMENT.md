# SPECTRE GUI Development Guide

## Table of Contents

1. [Architecture](#architecture)
2. [Development Setup](#development-setup)
3. [Project Structure](#project-structure)
4. [Building](#building)
5. [Testing](#testing)
6. [Component Development](#component-development)
7. [IPC Communication](#ipc-communication)
8. [State Management](#state-management)
9. [Adding New Pages](#adding-new-pages)
10. [Theming](#theming)
11. [CI/CD](#cicd)
12. [Release Process](#release-process)

---

## Architecture

SPECTRE GUI uses Tauri 2.10 + React 19 for a modern, performant desktop application.

### Technology Stack

```
┌────────────────────────────────────────┐
│   React 19 Frontend (TypeScript)       │
│   - UI Components (shadcn/ui + Radix)  │
│   - State Management (Zustand)         │
│   - Routing (React Router 7)           │
│   - Visualization (D3.js 7, Recharts)  │
│   - Styling (Tailwind CSS 4)           │
└──────────────┬─────────────────────────┘
               │ Tauri IPC (invoke/listen)
┌──────────────▼─────────────────────────┐
│   Rust Backend (spectre-gui)           │
│   - Tauri 2.10 Application             │
│   - IPC Handlers (commands/)           │
│   - State Management (AppState)        │
│   - Event Emission (scan progress)     │
└──────────────┬─────────────────────────┘
               │ Direct function calls
┌──────────────▼─────────────────────────┐
│   spectre-core (Rust Library)          │
│   - ProRT-IP Integration               │
│   - CyberChef-MCP Integration          │
│   - WRAITH Integration                 │
│   - Campaign, Results, Reports         │
└────────────────────────────────────────┘
```

### Data Flow

1. **User Action** → Frontend component event handler
2. **IPC Invoke** → Tauri `invoke()` call to Rust backend
3. **Command Handler** → Rust function processes request
4. **Core Logic** → `spectre-core` library functions
5. **Response** → Serialized data returned to frontend
6. **State Update** → Zustand store updates
7. **React Re-render** → UI updates with new data

### Event System

Real-time updates using Tauri events:

```
Rust Backend (emit)  →  Tauri Event Bridge  →  React Frontend (listen)

Examples:
- scan:progress  → Progress updates during scan
- scan:result    → Individual host discovered
- scan:complete  → Scan finished
- scan:error     → Scan failed
```

---

## Development Setup

### Prerequisites

**Required**:
- **Rust**: 1.92+ (`rustup toolchain install 1.92`)
- **Node.js**: 22+ (`nvm install 22`)
- **pnpm**: 9+ (`npm install -g pnpm`)

**Platform-Specific Dependencies**:

**Linux**:
```bash
# Debian/Ubuntu
sudo apt install libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libpcap-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel \
    openssl-devel \
    gtk3-devel \
    libappindicator-gtk3-devel \
    librsvg2-devel \
    libpcap-devel

# Arch
sudo pacman -S webkit2gtk \
    gtk3 \
    libappindicator-gtk3 \
    librsvg \
    libpcap
```

**macOS**:
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Homebrew dependencies
brew install libpcap
```

**Windows**:
- Visual Studio 2019+ with C++ build tools
- WebView2 SDK (bundled with Tauri)
- [Npcap](https://npcap.com/) for packet capture

### Clone Repository

```bash
git clone https://github.com/doublegate/SPECTRE.git
cd SPECTRE

# Initialize submodules (ProRT-IP, WRAITH, CyberChef-MCP)
git submodule update --init --recursive
```

### Install Dependencies

**Rust (workspace)**:
```bash
cargo build -p spectre-gui
```

**Frontend**:
```bash
cd crates/spectre-gui/frontend
pnpm install
```

### Development Server

**Option 1: Tauri Dev Script (Recommended)**:
```bash
cd crates/spectre-gui
./dev.sh  # Linux/macOS
# or
dev.bat   # Windows
```

**Option 2: Manual Commands**:
```bash
# Terminal 1: Frontend dev server
cd crates/spectre-gui/frontend
pnpm dev

# Terminal 2: Tauri dev (watches Rust code)
cd crates/spectre-gui
cargo tauri dev
```

**What Happens**:
- Vite dev server starts on `http://localhost:1420`
- Tauri application window opens with frontend loaded
- Hot module replacement (HMR) for React components
- Rust changes trigger automatic rebuild (slower than HMR)

**Access**:
- Application window opens automatically
- Console: `Ctrl+Shift+I` (Linux/Windows) or `Cmd+Option+I` (macOS)
- Backend logs: Terminal running `cargo tauri dev`

---

## Project Structure

```
crates/spectre-gui/
├── Cargo.toml              # Rust dependencies + Tauri config
├── build.rs                # Tauri build script
├── tauri.conf.json         # Tauri configuration
├── capabilities/           # IPC security permissions
│   └── default.json        # Allowed IPC commands
├── icons/                  # Application icons (various formats)
│   ├── 32x32.png
│   ├── 128x128.png
│   ├── 128x128@2x.png
│   ├── icon.icns (macOS)
│   └── icon.ico (Windows)
├── src/                    # Rust backend
│   ├── main.rs             # Desktop entry point
│   ├── lib.rs              # Tauri Builder + plugins + IPC handlers
│   ├── state.rs            # AppState (RwLock<Config> + active_scans)
│   ├── events.rs           # Event payload types (ScanProgressEvent, etc.)
│   └── commands/           # IPC command handlers (10 modules)
│       ├── status.rs       # get_version, get_status
│       ├── scan.rs         # start_scan, stop_scan, get_scan_results
│       ├── campaign.rs     # create/list/get/advance/export/import campaign
│       ├── results.rs      # get_dashboard_stats, get_findings
│       ├── report.rs       # generate_report, export_data
│       ├── target.rs       # parse_targets
│       ├── chef.rs         # execute_chef, list_chef_operations (stubs)
│       ├── comms.rs        # get_identity, list_peers, send_data (stubs)
│       └── config.rs       # get_config, set_config (stubs)
└── frontend/               # React application
    ├── package.json        # Frontend dependencies
    ├── vite.config.ts      # Vite bundler config
    ├── tsconfig.json       # TypeScript compiler config
    ├── tailwind.config.ts  # Tailwind CSS config
    ├── index.html          # HTML entry point
    └── src/
        ├── main.tsx        # React entry point
        ├── App.tsx         # Root component with ErrorBoundary
        ├── router.tsx      # React Router 7 config (lazy-loaded routes)
        ├── layouts/        # MainLayout, Sidebar, Header, StatusBar
        ├── pages/          # Page components (Dashboard, Recon, Campaigns, etc.)
        │   ├── Dashboard.tsx
        │   ├── Recon.tsx
        │   ├── Campaigns.tsx
        │   ├── CampaignDetail.tsx
        │   ├── Reports.tsx
        │   ├── Settings.tsx
        │   ├── Analysis.tsx (stub)
        │   ├── Comms.tsx (stub)
        │   └── Targets.tsx (stub)
        ├── components/     # UI components
        │   ├── ui/         # shadcn/ui components (12 components)
        │   │   ├── badge.tsx
        │   │   ├── button.tsx
        │   │   ├── card.tsx
        │   │   ├── checkbox.tsx
        │   │   ├── dialog.tsx
        │   │   ├── input.tsx
        │   │   ├── label.tsx
        │   │   ├── radio-group.tsx
        │   │   ├── select.tsx
        │   │   ├── separator.tsx
        │   │   ├── table.tsx
        │   │   └── tabs.tsx
        │   ├── scan/       # Scan components
        │   │   ├── NetworkTopology.tsx (D3.js force-directed graph)
        │   │   ├── ScanConfigForm.tsx
        │   │   ├── ScanProgress.tsx
        │   │   ├── HostCard.tsx
        │   │   └── ResultsTable.tsx
        │   ├── campaign/   # Campaign components
        │   │   ├── CampaignCard.tsx
        │   │   ├── CreateWizard.tsx
        │   │   ├── ObjectiveList.tsx
        │   │   ├── PhaseTimeline.tsx
        │   │   └── TargetInput.tsx
        │   ├── dashboard/  # Dashboard components
        │   │   ├── StatCard.tsx
        │   │   ├── SeverityChart.tsx (Recharts pie chart)
        │   │   ├── ServicesChart.tsx (Recharts bar chart)
        │   │   └── ActivityTimeline.tsx
        │   ├── reports/    # Reports components
        │   │   ├── FindingsTable.tsx
        │   │   ├── FindingDetail.tsx (modal)
        │   │   ├── ExportPanel.tsx
        │   │   └── ReportPreview.tsx (DOMPurify sanitized)
        │   ├── settings/   # Settings components (8 tabs)
        │   │   ├── GeneralTab.tsx
        │   │   ├── ScanTab.tsx
        │   │   ├── AnalysisTab.tsx
        │   │   ├── CommsTab.tsx
        │   │   ├── OutputTab.tsx
        │   │   ├── ThemesTab.tsx
        │   │   ├── ShortcutsTab.tsx
        │   │   └── AboutTab.tsx
        │   ├── analysis/   # Analysis components (stubs)
        │   └── comms/      # Comms components (stubs)
        ├── stores/         # Zustand state stores
        │   ├── scanStore.ts (multi-scan tracking)
        │   ├── campaignStore.ts
        │   ├── dashboardStore.ts
        │   └── uiStore.ts (sidebar, theme)
        ├── hooks/          # Custom React hooks
        │   ├── useScan.ts (event listeners)
        │   ├── useCampaign.ts
        │   ├── useDashboard.ts
        │   └── useIpc.ts
        ├── types/          # TypeScript type definitions
        │   ├── scan.ts (ScanRequest, ScanResult, Host, Port, etc.)
        │   ├── campaign.ts (Campaign, Phase, Artifact)
        │   ├── dashboard.ts (DashboardStats, Finding, Severity)
        │   └── config.ts (Config, ScanConfig, etc.)
        ├── config/         # Configuration files
        │   ├── shortcuts.ts (keyboard shortcuts)
        │   └── themes.ts (5 theme definitions)
        ├── styles/         # Global styles
        │   └── globals.css (Tailwind + theme CSS vars)
        ├── lib/            # Utility functions
        │   └── utils.ts (cn() class merger)
        └── __tests__/      # Frontend tests (117 tests)
            ├── components/
            ├── pages/
            ├── stores/
            └── hooks/
```

---

## Building

### Development Build

```bash
# Build Rust backend only
cargo build -p spectre-gui

# Build frontend only
cd crates/spectre-gui/frontend
pnpm build
# Output: dist/ directory
```

### Production Build

**Full Application**:
```bash
cd crates/spectre-gui
cargo tauri build
```

**Output Locations**:

**Linux**:
- `target/release/bundle/appimage/SPECTRE_0.5.0_amd64.AppImage`
- `target/release/bundle/deb/spectre_0.5.0_amd64.deb`
- `target/release/bundle/rpm/spectre-0.5.0-1.x86_64.rpm`

**macOS**:
- `target/release/bundle/dmg/SPECTRE_0.5.0_x64.dmg` (Intel)
- `target/release/bundle/dmg/SPECTRE_0.5.0_aarch64.dmg` (Apple Silicon)
- `target/release/bundle/macos/SPECTRE.app`

**Windows**:
- `target/release/bundle/msi/SPECTRE_0.5.0_x64_en-US.msi`
- `target/release/bundle/nsis/SPECTRE_0.5.0_x64-setup.exe`

### Build Configuration

**`tauri.conf.json`**:

Key sections:
```json
{
  "productName": "SPECTRE",
  "version": "0.5.0",
  "identifier": "com.spectre.gui",
  "build": {
    "beforeDevCommand": "cd frontend && pnpm dev",
    "beforeBuildCommand": "cd frontend && pnpm build",
    "devUrl": "http://localhost:1420",
    "frontendDist": "../frontend/dist"
  },
  "app": {
    "windows": [
      {
        "title": "SPECTRE - Security Platform",
        "width": 1280,
        "height": 800,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "security": {
      "csp": "default-src 'self' tauri:"
    }
  }
}
```

---

## Testing

### Rust Tests

**All GUI tests**:
```bash
cargo test -p spectre-gui
```

**Specific test file**:
```bash
cargo test -p spectre-gui --test commands_test
```

**With output**:
```bash
cargo test -p spectre-gui -- --nocapture
```

**Watch mode** (with cargo-watch):
```bash
cargo watch -x "test -p spectre-gui"
```

**Test Structure**:
```
crates/spectre-gui/src/
└── commands/
    └── scan.rs (contains #[cfg(test)] mod tests)
tests/
├── commands_test.rs
└── integration_test.rs
```

### Frontend Tests

**Run all tests**:
```bash
cd crates/spectre-gui/frontend
pnpm test
```

**Watch mode**:
```bash
pnpm test:watch
```

**Coverage**:
```bash
pnpm test:coverage
# Output: coverage/ directory with HTML report
```

**Type Checking**:
```bash
pnpm typecheck
# Runs: tsc --noEmit
```

**Test Example**:
```typescript
import { render, screen } from '@testing-library/react';
import { Dashboard } from '@/pages/Dashboard';
import { useDashboard } from '@/hooks/useDashboard';

// Mock the hook
vi.mock('@/hooks/useDashboard');

describe('Dashboard', () => {
  it('renders stats cards', () => {
    (useDashboard as any).mockReturnValue({
      stats: {
        total_hosts: 42,
        open_ports: 128,
        services_found: 15,
        severity_counts: { critical: 3, high: 5, medium: 10, low: 2, info: 1 },
      },
      isLoading: false,
      error: null,
    });

    render(<Dashboard />);

    expect(screen.getByText('42')).toBeInTheDocument(); // Total Hosts
    expect(screen.getByText('128')).toBeInTheDocument(); // Open Ports
  });

  it('shows error state', () => {
    (useDashboard as any).mockReturnValue({
      stats: null,
      isLoading: false,
      error: 'Failed to fetch stats',
    });

    render(<Dashboard />);

    expect(screen.getByText('Failed to load dashboard')).toBeInTheDocument();
  });
});
```

---

## Component Development

### Creating a New Component

**Example: HostDetails.tsx**

```typescript
// File: src/components/scan/HostDetails.tsx
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Host } from '@/types/scan';

interface HostDetailsProps {
  host: Host;
  onClose: () => void;
}

export function HostDetails({ host, onClose }: HostDetailsProps) {
  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center justify-between">
          <span>{host.ip}</span>
          <Badge variant={host.status === 'up' ? 'default' : 'secondary'}>
            {host.status}
          </Badge>
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-2">
        {host.hostname && (
          <div>
            <span className="text-sm font-medium">Hostname:</span>
            <span className="ml-2 text-sm text-muted-foreground">{host.hostname}</span>
          </div>
        )}
        <div>
          <span className="text-sm font-medium">Open Ports:</span>
          <span className="ml-2 text-sm text-muted-foreground">
            {host.ports?.length ?? 0}
          </span>
        </div>
        {host.os && (
          <div>
            <span className="text-sm font-medium">OS:</span>
            <span className="ml-2 text-sm text-muted-foreground">{host.os}</span>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
```

**Component Testing**:

```typescript
// File: __tests__/components/scan/HostDetails.test.tsx
import { render, screen } from '@testing-library/react';
import { HostDetails } from '@/components/scan/HostDetails';

describe('HostDetails', () => {
  it('renders host information', () => {
    const host = {
      ip: '192.168.1.1',
      hostname: 'gateway.local',
      status: 'up',
      ports: [{ port: 80, state: 'open' }],
      os: 'Linux 5.15',
    };

    render(<HostDetails host={host} onClose={() => {}} />);

    expect(screen.getByText('192.168.1.1')).toBeInTheDocument();
    expect(screen.getByText('gateway.local')).toBeInTheDocument();
    expect(screen.getByText('up')).toBeInTheDocument();
    expect(screen.getByText('1')).toBeInTheDocument(); // ports length
    expect(screen.getByText('Linux 5.15')).toBeInTheDocument();
  });
});
```

### Component Best Practices

1. **Type Safety**: Always define TypeScript interfaces for props
2. **Accessibility**: Add ARIA labels, semantic HTML, keyboard support
3. **Error Handling**: Handle loading/error states gracefully
4. **Memoization**: Use `React.memo()` for expensive components
5. **Composition**: Break large components into smaller, reusable pieces
6. **Styling**: Use Tailwind utilities + shadcn/ui components
7. **Testing**: Write tests for all interactive behavior

---

## IPC Communication

### Backend: Creating IPC Commands

**1. Define Command Handler** (`src/commands/example.rs`):

```rust
use tauri::State;
use crate::state::AppState;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ExampleRequest {
    pub param: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExampleResponse {
    pub value: String,
    pub count: u32,
}

#[tauri::command]
pub async fn get_example_data(
    state: State<'_, AppState>,
    request: ExampleRequest,
) -> Result<ExampleResponse, String> {
    // Access shared config
    let config = state.config.read()
        .map_err(|e| format!("Lock error: {}", e))?;

    // Business logic here
    let result = process_data(&request.param, &config)?;

    Ok(ExampleResponse {
        value: result.value,
        count: result.count,
    })
}

fn process_data(param: &str, config: &spectre_core::Config) -> Result<ProcessResult, String> {
    // Implementation
    Ok(ProcessResult { value: param.to_string(), count: 42 })
}

struct ProcessResult {
    value: String,
    count: u32,
}
```

**2. Register Command** (`src/lib.rs`):

```rust
mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        // ... other plugins
        .invoke_handler(tauri::generate_handler![
            commands::status::get_version,
            commands::status::get_status,
            commands::scan::start_scan,
            // Add new command here
            commands::example::get_example_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**3. Add to Capabilities** (`capabilities/default.json`):

```json
{
  "permissions": [
    "core:default",
    "shell:allow-open",
    {
      "identifier": "example:default",
      "allow": [
        { "cmd": "get_example_data" }
      ]
    }
  ]
}
```

### Frontend: Calling IPC Commands

**1. Type Definition** (`src/types/example.ts`):

```typescript
export interface ExampleRequest {
  param: string;
}

export interface ExampleResponse {
  value: string;
  count: number;
}
```

**2. Hook for IPC Call** (`src/hooks/useExample.ts`):

```typescript
import { useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { ExampleRequest, ExampleResponse } from '@/types/example';

export function useExample() {
  const [data, setData] = useState<ExampleResponse | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchData = useCallback(async (param: string) => {
    setLoading(true);
    setError(null);

    try {
      const request: ExampleRequest = { param };
      const response = await invoke<ExampleResponse>('get_example_data', request);
      setData(response);
    } catch (err) {
      setError(String(err));
      console.error('Failed to fetch example data:', err);
    } finally {
      setLoading(false);
    }
  }, []);

  return { data, loading, error, fetchData };
}
```

**3. Component Usage**:

```typescript
import { useExample } from '@/hooks/useExample';

export function ExampleComponent() {
  const { data, loading, error, fetchData } = useExample();

  useEffect(() => {
    fetchData('test-param');
  }, []);

  if (loading) return <LoadingSpinner />;
  if (error) return <ErrorMessage error={error} />;
  if (!data) return null;

  return (
    <div>
      <h2>Value: {data.value}</h2>
      <p>Count: {data.count}</p>
    </div>
  );
}
```

### Event Listening (Real-time Updates)

**Backend: Emit Events**:

```rust
use tauri::Manager;
use serde::Serialize;

#[derive(Clone, Serialize)]
struct ScanProgressEvent {
    scan_id: String,
    progress: f64,
    message: String,
}

#[tauri::command]
pub async fn start_scan(
    app_handle: tauri::AppHandle,
    request: ScanRequest,
) -> Result<String, String> {
    let scan_id = uuid::Uuid::new_v4().to_string();

    // Spawn background task
    tokio::spawn(async move {
        for i in 0..=100 {
            let event = ScanProgressEvent {
                scan_id: scan_id.clone(),
                progress: i as f64 / 100.0,
                message: format!("Scanning... {}%", i),
            };

            app_handle.emit("scan:progress", event)
                .expect("Failed to emit event");

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    });

    Ok(scan_id)
}
```

**Frontend: Listen for Events**:

```typescript
import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';

interface ScanProgressEvent {
  scan_id: string;
  progress: number;
  message: string;
}

export function useScanProgress(scanId: string) {
  const [progress, setProgress] = useState(0);
  const [message, setMessage] = useState('');

  useEffect(() => {
    const unlisten = listen<ScanProgressEvent>('scan:progress', (event) => {
      if (event.payload.scan_id === scanId) {
        setProgress(event.payload.progress);
        setMessage(event.payload.message);
      }
    });

    return () => {
      unlisten.then(fn => fn());
    };
  }, [scanId]);

  return { progress, message };
}
```

---

## State Management

SPECTRE GUI uses Zustand for client-side state management.

### Creating a Store

**Example: `stores/exampleStore.ts`**:

```typescript
import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import type { ExampleData } from '@/types/example';

interface ExampleState {
  // State
  data: ExampleData | null;
  loading: boolean;
  error: string | null;

  // Actions
  fetchData: (param: string) => Promise<void>;
  clearData: () => void;
  reset: () => void;
}

export const useExampleStore = create<ExampleState>((set, get) => ({
  // Initial state
  data: null,
  loading: false,
  error: null,

  // Actions
  fetchData: async (param: string) => {
    set({ loading: true, error: null });

    try {
      const data = await invoke<ExampleData>('get_example_data', { param });
      set({ data, loading: false });
    } catch (error) {
      set({ error: String(error), loading: false });
    }
  },

  clearData: () => set({ data: null }),

  reset: () => set({
    data: null,
    loading: false,
    error: null,
  }),
}));
```

### Using a Store in Components

```typescript
import { useExampleStore } from '@/stores/exampleStore';

export function ExampleComponent() {
  // Select specific state slices (prevents unnecessary re-renders)
  const data = useExampleStore((s) => s.data);
  const loading = useExampleStore((s) => s.loading);
  const fetchData = useExampleStore((s) => s.fetchData);

  useEffect(() => {
    fetchData('example-param');
  }, []);

  if (loading) return <LoadingSpinner />;

  return <div>{data?.value}</div>;
}
```

### Store Best Practices

1. **Selective Subscriptions**: Use `(s) => s.field` to subscribe to specific state slices
2. **Async Actions**: Define async functions directly in the store
3. **Error Handling**: Always include error state and handle failures
4. **Reset Functions**: Provide a way to reset state to initial values
5. **Devtools**: Use `devtools` middleware for debugging (development only)

---

## Adding New Pages

### 1. Create Page Component

**`src/pages/NewPage.tsx`**:

```typescript
import { useState } from 'react';
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card';

export function NewPage() {
  return (
    <div className="space-y-6 p-6">
      <h1 className="text-3xl font-bold">New Page</h1>
      <Card>
        <CardHeader>
          <CardTitle>Section Title</CardTitle>
        </CardHeader>
        <CardContent>
          {/* Page content */}
        </CardContent>
      </Card>
    </div>
  );
}

// Default export for lazy loading
export default NewPage;
```

### 2. Add Route

**`src/router.tsx`**:

```typescript
import { lazy } from 'react';

const NewPage = lazy(() => import('@/pages/NewPage'));

export const router = createBrowserRouter([
  {
    element: <MainLayout />,
    children: [
      // ... existing routes
      {
        path: 'new-page',
        element: (
          <Suspense fallback={<LoadingFallback />}>
            <NewPage />
          </Suspense>
        ),
      },
    ],
  },
]);
```

### 3. Add Navigation

**`src/layouts/Sidebar.tsx`**:

```typescript
import { NewPageIcon } from 'lucide-react';

<NavLink to="/new-page">
  <NewPageIcon className="h-5 w-5" />
  <span>New Page</span>
</NavLink>
```

### 4. Add Keyboard Shortcut (Optional)

**`src/config/shortcuts.ts`**:

```typescript
export const KEYBOARD_SHORTCUTS = {
  navigation: {
    // ... existing shortcuts
    newPage: {
      key: 'Alt+6',
      description: 'Go to New Page',
      action: '/new-page',
    },
  },
};
```

**`src/layouts/MainLayout.tsx`** (already wired for shortcuts):

No additional code needed—shortcuts are handled automatically.

---

## Theming

SPECTRE uses Tailwind CSS 4 with CSS custom properties for theming.

### Theme Structure

**`src/styles/globals.css`**:

```css
/* Dark Theme (default) */
:root,
[data-theme="dark"] {
  --background: oklch(0.16 0.02 280);      /* #12121c */
  --foreground: oklch(0.84 0.01 270);      /* #c8c8dc */
  --primary: oklch(0.65 0.15 255);         /* #6495ed */
  /* ... other colors */
}

/* Light Theme */
[data-theme="light"] {
  --background: oklch(0.97 0.005 270);     /* #f5f5fa */
  --foreground: oklch(0.22 0.02 270);      /* #1e1e32 */
  --primary: oklch(0.28 0.07 270);         /* #191970 */
  /* ... other colors */
}

/* Tactical, Matrix, Hacker themes... */
```

### Adding a New Theme

**1. Define Theme Colors** (`src/styles/globals.css`):

```css
[data-theme="custom"] {
  --background: oklch(0.15 0.03 200);
  --foreground: oklch(0.85 0.02 200);
  --primary: oklch(0.60 0.20 200);
  --secondary: oklch(0.50 0.18 210);
  --accent: oklch(0.70 0.15 190);
  --destructive: oklch(0.55 0.22 15);
  --success: oklch(0.70 0.18 140);
  --warning: oklch(0.75 0.16 65);
  /* ... other colors */
}
```

**2. Register Theme** (`src/config/themes.ts`):

```typescript
export const themes = {
  // ... existing themes
  custom: {
    name: 'Custom Theme',
    description: 'My custom color scheme',
  },
};
```

**3. Apply Theme** (handled by Settings page):

```typescript
// Settings → ThemesTab.tsx
const handleThemeChange = (theme: string) => {
  document.documentElement.setAttribute('data-theme', theme);
  localStorage.setItem('theme', theme);
};
```

---

## CI/CD

SPECTRE GUI uses GitHub Actions for automated testing and building.

### Workflows

**`.github/workflows/gui.yml`**:

```yaml
name: GUI CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  frontend-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: pnpm/action-setup@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: pnpm
      - run: cd crates/spectre-gui/frontend && pnpm install
      - run: cd crates/spectre-gui/frontend && pnpm typecheck
      - run: cd crates/spectre-gui/frontend && pnpm test

  gui-build-matrix:
    strategy:
      fail-fast: false
      matrix:
        include:
          - platform: ubuntu-24.04
          - platform: macos-latest
          - platform: macos-14  # Apple Silicon
          - platform: windows-latest

    runs-on: ${{ matrix.platform }}
    steps:
      - uses: actions/checkout@v4
        with:
          submodules: recursive

      - name: Install dependencies (Linux)
        if: matrix.platform == 'ubuntu-24.04'
        run: |
          sudo apt update
          sudo apt install -y libwebkit2gtk-4.1-dev libssl-dev libpcap-dev

      - name: Install dependencies (macOS)
        if: startsWith(matrix.platform, 'macos')
        run: brew install libpcap

      - uses: dtolnay/rust-toolchain@stable
      - uses: pnpm/action-setup@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: pnpm

      - name: Install frontend dependencies
        run: cd crates/spectre-gui/frontend && pnpm install

      - name: Build Tauri application
        run: cd crates/spectre-gui && cargo tauri build

      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: spectre-gui-${{ matrix.platform }}
          path: |
            crates/spectre-gui/target/release/bundle/**/*
```

### Running CI Locally

**Using `act` (GitHub Actions local runner)**:

```bash
# Install act
brew install act  # macOS
sudo pacman -S act  # Arch Linux

# Run specific job
act -j frontend-check

# Run specific platform
act -j gui-build-matrix --matrix platform:ubuntu-24.04
```

**Manual Testing**:

```bash
# Typecheck
cd crates/spectre-gui/frontend
pnpm typecheck

# Tests
pnpm test

# Build
cd ..
cargo tauri build
```

---

## Release Process

### 1. Version Bump

Update version in multiple files:

**`Cargo.toml` (workspace)**:
```toml
[workspace.package]
version = "0.5.0"
```

**`crates/spectre-gui/Cargo.toml`**:
```toml
[package]
version = "0.5.0"
```

**`crates/spectre-gui/frontend/package.json`**:
```json
{
  "version": "0.5.0"
}
```

**`crates/spectre-gui/tauri.conf.json`**:
```json
{
  "version": "0.5.0"
}
```

**Script to update all versions**:
```bash
#!/bin/bash
VERSION="0.5.0"

# Update workspace Cargo.toml
sed -i "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml

# Update GUI Cargo.toml
sed -i "s/^version = \".*\"/version = \"$VERSION\"/" crates/spectre-gui/Cargo.toml

# Update frontend package.json
cd crates/spectre-gui/frontend
pnpm version $VERSION --no-git-tag-version
cd ../../..

# Update tauri.conf.json
sed -i "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" crates/spectre-gui/tauri.conf.json

echo "Version updated to $VERSION"
```

### 2. Update CHANGELOG

Add entry to `CHANGELOG.md`:

```markdown
## [0.5.0] - 2026-02-06

### Added
- Feature 1
- Feature 2

### Changed
- Change 1

### Fixed
- Fix 1

### Performance
- Optimization 1
```

### 3. Commit and Tag

```bash
git add .
git commit -m "chore: release v0.5.0"
git tag -a v0.5.0 -m "Release v0.5.0: Description"
git push origin main
git push origin v0.5.0
```

### 4. GitHub Actions Builds Installers

CI automatically:
1. Detects new tag
2. Builds for all platforms (Linux, macOS Intel, macOS ARM, Windows)
3. Creates GitHub release
4. Attaches installers to release

### 5. Publish Release

1. Go to GitHub Releases
2. Edit draft release (auto-created by CI)
3. Add release notes from CHANGELOG
4. Mark as pre-release or stable
5. Publish

---

**For User Documentation**: See [GUI-GUIDE.md](../user-guide/GUI-GUIDE.md)

**For Architecture Details**: See [SYSTEM-DESIGN.md](../architecture/SYSTEM-DESIGN.md)

**For Platform Requirements**: See [PLATFORM-REQUIREMENTS.md](../../crates/spectre-gui/PLATFORM-REQUIREMENTS.md)
