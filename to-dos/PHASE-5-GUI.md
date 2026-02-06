# Phase 5: GUI Application (Operation SHADOW)

**Version:** v0.5.x | **Status:** Planned | **Timeline:** Q3 2026

---

## Phase Objective

Develop a cross-platform desktop GUI application using Tauri 2.0 and React for visual campaign planning, scan management, and results analysis.

---

## Sprint 5.1: Tauri 2.0 Setup

**Status:** Planned | **Duration:** 2 weeks

### Tasks

- [ ] Create spectre-gui crate
- [ ] Initialize Tauri 2.0 project
- [ ] Configure Tauri plugins
- [ ] Set up IPC commands
- [ ] Implement state management
- [ ] Create window configuration
- [ ] Add system tray support
- [ ] Implement auto-updater
- [ ] Write Tauri tests

---

## Sprint 5.2: React Frontend

**Status:** Planned | **Duration:** 2 weeks

### Tasks

- [ ] Initialize React project
- [ ] Configure Vite bundler
- [ ] Set up TypeScript
- [ ] Add Tailwind CSS
- [ ] Create component library
- [ ] Implement routing
- [ ] Add state management (Zustand)
- [ ] Create IPC hooks
- [ ] Write component tests

---

## Sprint 5.3: Campaign Planning UI

**Status:** ✅ Complete (2026-02-05) | **Duration:** 2 weeks

### Tasks

- [x] Design campaign creation flow (4-step wizard: Name → Objectives → Targets → Review)
- [x] Implement target input UI (TargetInput.tsx - CIDR parsing with IPC validation)
- [x] Add phase planning interface (PhaseTimeline.tsx - 6-phase visual timeline)
- [x] Create timeline view (PhaseTimeline component with advance button + prerequisites)
- [x] Campaign CRUD operations (create, list, get, advance, export, import, archive)
- [x] Backend IPC wiring (7 campaign commands + target parsing)
- [x] Write campaign UI tests (3 frontend + 7 Rust tests = 10 new tests, all passing)
- [ ] Create scope visualization (deferred)
- [ ] Implement workflow builder (deferred)
- [ ] Add collaboration features (deferred to future sprint)

---

## Sprint 5.4: Scan Visualization

**Status:** Planned | **Duration:** 2 weeks

### Tasks

- [ ] Create network topology view
- [ ] Implement real-time scan progress
- [ ] Add host detail cards
- [ ] Create port visualization
- [ ] Implement service icons
- [ ] Add scan control buttons
- [ ] Create scan queue display
- [ ] Write visualization tests

---

## Sprint 5.5: Results Dashboard

**Status:** ✅ Complete (2026-02-06) | **Duration:** 1 day

### Tasks

- [x] Design dashboard layout
- [x] Create statistics widgets (StatCard - hosts, ports, services, findings)
- [x] Implement findings table (FindingsTable - sort, filter, pagination)
- [x] Add filtering and search (severity, service, port, search input)
- [x] Create detail views (FindingDetail modal with CVE links)
- [x] Implement export buttons (ExportPanel - 5 formats: CSV, JSON, XML, HTML, Markdown)
- [x] Add report preview (ReportPreview with DOMPurify sanitization)
- [x] Write dashboard tests (15+ component tests, 31 backend tests)
- [x] Implement SeverityChart (Recharts PieChart)
- [x] Implement ServicesChart (Recharts BarChart)
- [x] Create ActivityTimeline component
- [x] Wire backend IPC (results.rs: 10 tests, report.rs: 21 tests)

### Deliverables

- Dashboard page with real-time statistics (auto-refresh 30s)
- Reports page with export functionality (5 formats)
- 31 backend tests (results.rs: 10, report.rs: 21)
- 15+ frontend component tests (117/121 passing - 96.7%)
- 12 shadcn/ui components integrated
- DOMPurify security for HTML previews
- Recharts 2 integration for charts

---

## Sprint 5.6: Settings, Analysis & Comms UI

**Status:** ✅ Complete (2026-02-06) | **Duration:** 6 hours

### Tasks

- [x] Create settings page with 8 tabs (General, Scan, Analysis, Comms, Output, Theme, Shortcuts, About)
- [x] Implement theme selection (5 themes with live preview)
- [x] Add configuration editor (fully wired to spectre-core Config)
- [x] Create scan profiles UI (timing templates T0-T5, port presets, detection toggles)
- [x] Implement keyboard shortcuts reference (11 shortcuts documented)
- [x] Create about/help pages (version info, component versions, tech stack, license)
- [x] Wire Analysis page to CyberChef operations (15 operations across 4 categories)
- [x] Wire Comms page to WRAITH protocol stubs (identity, peers, send)
- [x] Write settings tests (8 backend config tests + 15 frontend tests = 23 new tests)
- [x] Backend IPC wiring: config.rs (5 tests), chef.rs (7 tests), comms.rs (3 tests)

### Deliverables

- 8 settings components (68 frontend files total, +8 from Sprint 5.5)
- Backend: 74 tests (up from 66, +8 new tests)
- Frontend: 121 tests (updated mocks)
- Total GUI: 195 tests passing
- Added dependencies: base64, hex, urlencoding, rand

---

## Sprint 5.7: Cross-Platform Testing

**Status:** ✅ Complete (2026-02-06) | **Duration:** 1 day

### Tasks

- [x] Create dedicated GUI CI workflow (gui.yml - 135 lines)
- [x] Build matrix for all platforms (Linux x64, macOS x64/ARM64, Windows x64)
- [x] Update release workflow with tauri-action (installer generation)
- [x] Platform test scripts (Linux, macOS, Windows)
- [x] Platform requirements documentation
- [x] Tauri bundle configuration (6 installer formats)
- [x] Version bump to v0.5.0-beta.1
- [x] Update all documentation

### Deliverables

**GitHub Actions Workflows:**
- `.github/workflows/gui.yml` (135 lines) - Dedicated GUI CI with build matrix
  - frontend-check job: TypeScript type check + frontend tests
  - gui-build-matrix job: Multi-platform builds (4 platforms)
  - Platform-specific dependencies (webkit2gtk-4.1, libpcap, Npcap SDK)
  - Rust and pnpm caching for faster builds
- `.github/workflows/release.yml` (updated, +94 lines) - Installer generation
  - build-gui job: tauri-apps/tauri-action@v0 integration
  - Automated builds for AppImage, deb, rpm, dmg, msi, nsis
  - Artifact upload and release attachment

**Platform Test Scripts:**
- `scripts/test-linux.sh` (45 lines) - webkit2gtk-4.1 + GTK 3 verification
- `scripts/test-macos.sh` (50 lines) - Xcode CLT + architecture detection
- `scripts/test-windows.ps1` (60 lines) - VS Build Tools + MSI build

**Documentation:**
- `PLATFORM-REQUIREMENTS.md` (280 lines) - Comprehensive platform guide
  - Linux: Debian/Ubuntu, Fedora/RHEL, Arch install commands
  - macOS: Xcode Command Line Tools, Intel/Apple Silicon support
  - Windows: WebView2, VS Build Tools, optional Npcap
  - Troubleshooting guide

**Configuration:**
- `tauri.conf.json` updated:
  - Version: 0.5.0-beta.1
  - Bundle identifier: com.doublegate.spectre
  - 6 bundle targets: deb, appimage, rpm, dmg, msi, nsis
  - Platform-specific settings (Linux deps, macOS minimum 10.15, Windows WIX)

**Version Updates:**
- Workspace: 0.5.0-beta.1 (Cargo.toml)
- GUI frontend: 0.5.0-beta.1 (package.json)
- GUI config: 0.5.0-beta.1 (tauri.conf.json)
- README badge updated
- CHANGELOG.md comprehensive entry (150+ lines)

**CI/CD Fixes (5 commits):**
1. **0cc6d3e** - Initial Tauri config fixes and explicit runner versions
2. **e132776** - Cache invalidation v1 + macOS runner revert
3. **e8edb70** - macOS runner deprecation fix (macos-13 → macos-15-intel)
4. **1743c51** - Cache invalidation v2 with prefix-key: "v2"
5. **2098779** - Tauri identifier location fix (ROOT level per Tauri 2.x) - CRITICAL

**Issues Resolved:**
- Tauri 2.x identifier field location (moved from bundle section to root level)
- macOS 13 runner deprecated December 4, 2025 (migrated to macos-15-intel)
- Stale Rust cache causing sqlx errors (invalidated with prefix-key versioning)
- CI reproducibility with explicit runner versions (22.04, 15-intel/14, 2022)

**Technical Debt:**
- Code signing/notarization deferred (requires Apple Developer account)
- Icon design deferred to Sprint 5.8 (placeholders in place)

---

## Sprint 5.8: GUI Polish and Release

**Status:** Planned | **Duration:** 2 weeks

### Tasks

- [ ] UI/UX review
- [ ] Accessibility audit
- [ ] Performance optimization
- [ ] Fix discovered issues
- [ ] Create user documentation
- [ ] Build release packages
- [ ] Update CHANGELOG
- [ ] Create v0.5.0 release

---

## Phase 5 Summary

### Deliverables

1. **spectre-gui** application:
   - Cross-platform (Linux, macOS, Windows)
   - Campaign planning interface
   - Scan visualization
   - Results dashboard
   - Settings management
   - Auto-updates

### Success Metrics

- [ ] Works on all platforms
- [ ] < 200MB memory usage
- [ ] Responsive UI (< 100ms)
- [ ] v0.5.0 released

---

## Next Phase

[Phase 6: MCP Server (Operation WRAITH)](PHASE-6-MCP.md)
