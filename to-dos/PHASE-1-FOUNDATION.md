# Phase 1: Foundation (Operation BLACKOUT)

**Version:** v0.1.x | **Status:** Current | **Timeline:** Q1 2026

---

## Phase Objective

Establish the project foundation with workspace configuration, CLI skeleton, initial component integration, and comprehensive documentation.

---

## Sprint 1.1: Project Setup

**Status:** Complete | **Duration:** 2 weeks

### Objectives
- Initialize Cargo workspace
- Configure development tooling
- Establish CI/CD pipeline
- Create documentation structure

### Tasks

- [x] Create GitHub repository
- [x] Initialize Cargo workspace
- [x] Configure workspace members (cli, core, tui, gui, mcp)
- [x] Set up rustfmt.toml configuration
- [x] Set up clippy.toml configuration
- [x] Create .editorconfig
- [x] Configure .gitignore
- [x] Create CI workflow (GitHub Actions)
- [x] Create release workflow
- [x] Set up dependabot
- [x] Create CONTRIBUTING.md
- [x] Create SECURITY.md
- [x] Create issue templates
- [x] Create PR template
- [x] Create initial README.md
- [x] Create CHANGELOG.md
- [x] Create CLAUDE.md
- [x] Create LICENSE file

### Acceptance Criteria
- [x] `cargo build` succeeds (empty workspace)
- [x] CI pipeline runs on push
- [x] Documentation renders correctly
- [x] All templates in place

---

## Sprint 1.2: CLI Argument Parsing

**Status:** Planned | **Duration:** 2 weeks

### Objectives
- Implement CLI structure with clap
- Define command hierarchy
- Add help and version display

### Tasks

- [ ] Create spectre-cli crate
- [ ] Add clap dependency with derive feature
- [ ] Define top-level CLI struct
- [ ] Implement `scan` command structure
  - [ ] `-sS` SYN scan flag
  - [ ] `-sT` Connect scan flag
  - [ ] `-sV` Service detection flag
  - [ ] `-p` Port specification
  - [ ] `-o` Output format
  - [ ] Target arguments
- [ ] Implement `chef` command structure
  - [ ] Operation arguments
  - [ ] `--input` flag
  - [ ] `--file` flag
  - [ ] `--recipe` flag
- [ ] Implement `send` command structure
- [ ] Implement `receive` command structure
- [ ] Implement `status` command
- [ ] Implement `config` command
- [ ] Add shell completion generation
- [ ] Add colored help output
- [ ] Write CLI unit tests

### Acceptance Criteria
- [ ] `spectre --help` displays all commands
- [ ] `spectre scan --help` shows scan options
- [ ] Tab completion works in bash/zsh
- [ ] Invalid commands show helpful errors

---

## Sprint 1.3: Configuration Management

**Status:** Planned | **Duration:** 2 weeks

### Objectives
- Implement configuration loading
- Support multiple config sources
- Add environment variable support

### Tasks

- [ ] Create configuration module in spectre-core
- [ ] Define Config struct with serde
- [ ] Implement TOML parsing
- [ ] Implement config file discovery
  - [ ] System: /etc/spectre/spectre.toml
  - [ ] User: ~/.config/spectre/spectre.toml
  - [ ] Project: ./spectre.toml
- [ ] Implement config merging (precedence)
- [ ] Add environment variable support (SPECTRE_*)
- [ ] Implement `config init` command
- [ ] Implement `config show` command
- [ ] Implement `config check` command
- [ ] Create default configuration file
- [ ] Document all configuration options
- [ ] Write configuration tests

### Acceptance Criteria
- [ ] `spectre config init` creates default config
- [ ] `spectre config show` displays effective config
- [ ] Environment variables override config file
- [ ] CLI arguments override everything

---

## Sprint 1.4: Logging and Error Handling

**Status:** Planned | **Duration:** 2 weeks

### Objectives
- Implement structured logging
- Create error type hierarchy
- Add debug and trace output

### Tasks

- [ ] Add tracing dependencies
- [ ] Configure tracing-subscriber
- [ ] Implement log level configuration
- [ ] Add RUST_LOG support
- [ ] Implement log file output (optional)
- [ ] Create SpectreError enum
- [ ] Implement error display and sources
- [ ] Add anyhow for CLI errors
- [ ] Implement #[instrument] on key functions
- [ ] Add progress indicators
- [ ] Create error code reference
- [ ] Write logging tests

### Acceptance Criteria
- [ ] `RUST_LOG=debug` shows debug output
- [ ] Errors display user-friendly messages
- [ ] Log file created when configured
- [ ] Progress shown during long operations

---

## Sprint 1.5: Basic ProRT-IP Integration

**Status:** Planned | **Duration:** 2 weeks

### Objectives
- Integrate ProRT-IP scanning library
- Implement basic scan functionality
- Support key scan types

### Tasks

- [ ] Add ProRT-IP as workspace dependency
- [ ] Create scan module in spectre-core
- [ ] Implement Scanner trait
- [ ] Create PrtipScanner implementation
- [ ] Implement SYN scan wrapper
- [ ] Implement Connect scan wrapper
- [ ] Implement port parsing
- [ ] Implement target parsing (IP, CIDR, hostname)
- [ ] Implement basic result formatting
- [ ] Add JSON output support
- [ ] Add table output support
- [ ] Handle scan errors gracefully
- [ ] Write integration tests

### Acceptance Criteria
- [ ] `spectre scan -sS 127.0.0.1` works
- [ ] `spectre scan -sT example.com` works
- [ ] Results display in table format
- [ ] `-o json` outputs valid JSON

---

## Sprint 1.6: Basic CyberChef-MCP Integration

**Status:** Planned | **Duration:** 2 weeks

### Objectives
- Integrate CyberChef via MCP
- Implement basic operations
- Support recipe execution

### Tasks

- [ ] Add MCP client dependency
- [ ] Create chef module in spectre-core
- [ ] Implement MCP connection management
- [ ] Implement Docker container management
- [ ] Create Chef trait
- [ ] Implement CyberChefMcp adapter
- [ ] Implement single operation execution
- [ ] Implement recipe execution
- [ ] Add input from stdin support
- [ ] Add input from file support
- [ ] Implement `chef setup` command
- [ ] Implement `chef --health` check
- [ ] Write integration tests

### Acceptance Criteria
- [ ] `spectre chef From_Base64 --input "SGVsbG8="` works
- [ ] `spectre chef setup` starts container
- [ ] `spectre chef --health` shows status
- [ ] Operations chain together

---

## Sprint 1.7: Basic WRAITH Integration

**Status:** Planned | **Duration:** 2 weeks

### Objectives
- Integrate WRAITH-Protocol library
- Implement identity management
- Support basic secure messaging

### Tasks

- [ ] Add WRAITH-Protocol as workspace dependency
- [ ] Create comms module in spectre-core
- [ ] Implement identity generation
- [ ] Implement identity storage
- [ ] Implement `identity init` command
- [ ] Implement `identity show` command
- [ ] Implement `peer add` command
- [ ] Implement basic channel creation
- [ ] Implement `send` command (text)
- [ ] Implement `receive` command
- [ ] Handle connection errors
- [ ] Write integration tests

### Acceptance Criteria
- [ ] `spectre identity init` generates keys
- [ ] `spectre peer add` adds trusted peer
- [ ] Basic send/receive works between peers
- [ ] Keys stored securely

---

## Sprint 1.8: End-to-End Testing

**Status:** Planned | **Duration:** 2 weeks

### Objectives
- Complete integration tests
- Documentation review
- Bug fixes and polish

### Tasks

- [ ] Write CLI integration tests
- [ ] Write scan integration tests
- [ ] Write chef integration tests
- [ ] Write comms integration tests
- [ ] Create test fixtures
- [ ] Run full test suite
- [ ] Fix discovered bugs
- [ ] Review and update all documentation
- [ ] Update CHANGELOG for v0.1.0
- [ ] Create GitHub release
- [ ] Build release binaries
- [ ] Update README with usage examples

### Acceptance Criteria
- [ ] All tests passing
- [ ] 70%+ code coverage
- [ ] Documentation complete
- [ ] v0.1.0 released on GitHub

---

## Phase 1 Summary

### Deliverables

1. **spectre-cli** binary with commands:
   - `scan` - Basic network scanning
   - `chef` - CyberChef operations
   - `send` / `receive` - WRAITH messaging
   - `identity` - Key management
   - `peer` - Peer management
   - `config` - Configuration
   - `status` - Component status

2. **spectre-core** library with:
   - Configuration management
   - ProRT-IP integration
   - CyberChef-MCP integration
   - WRAITH integration
   - Error handling

3. **Documentation**:
   - README, CHANGELOG, CONTRIBUTING
   - User guides
   - API documentation
   - Architecture docs

### Success Metrics

- [ ] All 8 sprints complete
- [ ] 70%+ test coverage
- [ ] 0 critical bugs
- [ ] v0.1.0 released

### Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Component API incompatibility | Use adapter pattern |
| Build complexity | Document requirements |
| Performance issues | Defer optimization |

---

## Next Phase

[Phase 2: Core Orchestration (Operation NIGHTFALL)](PHASE-2-CORE.md)
