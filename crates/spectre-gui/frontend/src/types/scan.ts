/** Mirrors Rust: crates/spectre-gui/src/commands/scan.rs */

export interface ScanRequest {
  targets: string[];
  ports?: string;
  scan_type?: string;
  timing?: number;
  service_detection?: boolean;
  os_detection?: boolean;
}

export interface ScanSummary {
  scan_id: string;
  hosts_scanned: number;
  hosts_up: number;
  open_ports: number;
  duration_ms: number;
}

/** Mirrors Rust: crates/spectre-gui/src/events.rs */

export interface ScanProgressEvent {
  scan_id: string;
  completed: number;
  total: number;
  percent: number;
  current_target?: string;
  rate_pps?: number;
}

export interface PortInfo {
  port: number;
  protocol: string;
  state: string;
  service?: string;
  version?: string;
  banner?: string;
}

export interface OsInfo {
  name: string;
  version?: string;
  confidence: number;
}

export interface ScanResultEvent {
  scan_id: string;
  host: string;
  hostname?: string;
  ports: PortInfo[];
  os?: OsInfo;
}

export interface ScanCompleteEvent {
  scan_id: string;
  hosts_scanned: number;
  open_ports: number;
  services_found: number;
  duration_ms: number;
}

export interface ScanErrorEvent {
  scan_id: string;
  error: string;
  target?: string;
}

/** Extended types for frontend use */

export interface Host {
  ip: string;
  hostname?: string;
  os?: OsInfo;
  ports: PortInfo[];
  severity: Severity;
  scan_id: string;
}

export enum ScanType {
  Syn = "syn",
  Connect = "connect",
  Udp = "udp",
  Ack = "ack",
  Fin = "fin",
  Xmas = "xmas",
  Null = "null",
  Window = "window",
}

export enum Severity {
  Critical = "critical",
  High = "high",
  Medium = "medium",
  Low = "low",
  Info = "info",
}

export enum PortState {
  Open = "open",
  Closed = "closed",
  Filtered = "filtered",
  Unknown = "unknown",
  OpenFiltered = "openfiltered",
  ClosedFiltered = "closedfiltered",
}

/** Helper to determine severity from host data */
export function calculateSeverity(ports: PortInfo[]): Severity {
  const openPorts = ports.filter((p) => p.state === "open");
  const criticalServices = ["telnet", "ftp", "rexec", "rlogin"];
  const highRiskServices = ["ssh", "rdp", "vnc", "mysql", "postgres", "mongodb"];

  // Critical if any critical service is open
  if (openPorts.some((p) => criticalServices.includes(p.service || ""))) {
    return Severity.Critical;
  }

  // High if any high-risk service is open
  if (openPorts.some((p) => highRiskServices.includes(p.service || ""))) {
    return Severity.High;
  }

  // Medium if more than 5 open ports
  if (openPorts.length > 5) {
    return Severity.Medium;
  }

  // Low if any open ports
  if (openPorts.length > 0) {
    return Severity.Low;
  }

  return Severity.Info;
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
