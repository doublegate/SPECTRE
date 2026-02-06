/** Mirrors Rust: crates/spectre-gui/src/commands/scan.rs */

export interface ScanRequest {
  targets: string[];
  ports?: string;
  scan_type?: string;
  timing?: number;
}

export interface ScanSummary {
  hosts_scanned: number;
  hosts_up: number;
  open_ports: number;
  duration_ms: number;
}

/** Mirrors Rust: crates/spectre-gui/src/events.rs */

export interface ScanProgressEvent {
  completed: number;
  total: number;
  percent: number;
  current_target?: string;
  rate_pps?: number;
}

export interface ScanResultEvent {
  host: string;
  port: number;
  state: string;
  protocol: string;
  service?: string;
  version?: string;
}

export interface ScanCompleteEvent {
  hosts_scanned: number;
  open_ports: number;
  services_found?: number;
  duration_ms: number;
}

export interface ScanErrorEvent {
  error: string;
  target?: string;
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
