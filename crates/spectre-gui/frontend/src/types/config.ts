/** Mirrors Rust: crates/spectre-gui/src/commands/config.rs */

export interface ConfigSummary {
  scan_timing: number;
  scan_default_ports: string;
  chef_docker_image: string;
  comms_identity_dir: string;
  output_format: string;
  verbose: number;
}

/** Mirrors Rust: crates/spectre-gui/src/commands/status.rs */

export interface VersionInfo {
  version: string;
  name: string;
  description: string;
}

export interface ComponentStatus {
  name: string;
  status: string;
  version?: string;
  details?: string;
}

export interface SystemStatus {
  components: ComponentStatus[];
  config_loaded: boolean;
}

/** Mirrors Rust: crates/spectre-gui/src/commands/results.rs */

export interface FindingSummary {
  host: string;
  port: number;
  service: string;
  severity: string;
  description: string;
}

export interface SeverityCounts {
  critical: number;
  high: number;
  medium: number;
  low: number;
  info: number;
}

export interface DashboardStats {
  total_hosts: number;
  total_ports: number;
  open_ports: number;
  services_found: number;
  severity_counts: SeverityCounts;
}

/** Mirrors Rust: crates/spectre-gui/src/commands/report.rs */

export interface ReportRequest {
  format: string;
  campaign?: string;
  include_executive_summary: boolean;
}

export interface ReportResult {
  format: string;
  path?: string;
  content_preview: string;
}

/** Mirrors Rust: crates/spectre-gui/src/commands/target.rs */

export interface ParsedTarget {
  original: string;
  expanded: string[];
  count: number;
}

export interface TargetInput {
  targets: string[];
}

/** Mirrors Rust: crates/spectre-gui/src/events.rs */

export interface StatusEvent {
  message: string;
  level: string;
}
