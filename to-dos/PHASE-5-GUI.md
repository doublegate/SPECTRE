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

**Status:** Planned | **Duration:** 2 weeks

### Tasks

- [ ] Test on Linux
- [ ] Test on macOS
- [ ] Test on Windows
- [ ] Fix platform-specific issues
- [ ] Optimize for each platform
- [ ] Create platform installers
- [ ] Add auto-update testing
- [ ] Write cross-platform tests

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
