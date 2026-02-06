/** Mirrors Rust: crates/spectre-gui/src/commands/results.rs */

export interface FindingSummary {
  id: string;
  host: string;
  port?: number;
  service?: string;
  protocol?: string;
  state: string;
  severity: Severity | string;
  title: string;
  description: string;
  version?: string;
  cves?: string[];
  remediation?: string;
  references: string[];
  tags: string[];
  timestamp: string;
}

// Alias for components that use this name
export type Finding = FindingSummary;

export interface DashboardStats {
  total_hosts: number;
  total_ports: number;
  open_ports: number;
  services_found: number;
  severity_counts: SeverityCounts;
  top_services: ServiceCount[];
  recent_activity: ScanActivity[];
}

export interface SeverityCounts {
  critical: number;
  high: number;
  medium: number;
  low: number;
  info: number;
}

export interface ServiceCount {
  name: string;
  count: number;
}

export interface ScanActivity {
  scan_id: string;
  status: string;
  target_count: number;
  duration_ms?: number;
  timestamp: string;
}

export interface FindingFilters {
  severity?: Severity[];
  severities?: string[];  // Alternative naming for some components
  service?: string[];
  host?: string;
  port_range?: [number, number];
}

export interface SortOptions {
  field: string;
  direction: "asc" | "desc";
}

export interface Pagination {
  page: number;
  per_page: number;
}

export interface FindingsResponse {
  findings: FindingSummary[];
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
}

/** Mirrors Rust: crates/spectre-gui/src/commands/report.rs */

export interface ReportRequest {
  format: ReportFormat;
  campaign?: string;
  template?: string;
  include_executive_summary: boolean;
  filters?: ReportFilters;
}

export interface ReportFilters {
  severity?: Severity[];
  date_range?: [string, string];
  services?: string[];
}

export interface ReportResult {
  format: string;
  path?: string;
  content: string;
  preview: string;
  size_bytes: number;
}

export interface ExportRequest {
  format: ExportFormat;
  data_type: ExportDataType;
  filters?: ReportFilters;
  path?: string;
}

export interface ExportResult {
  format: string;
  path?: string;
  content: string;
  record_count: number;
}

/** Enums */

export enum Severity {
  Critical = "critical",
  High = "high",
  Medium = "medium",
  Low = "low",
  Info = "info",
}

export enum ReportFormat {
  Html = "html",
  Markdown = "markdown",
}

export type ReportFormatString = 'html' | 'markdown' | 'json' | 'xml' | 'csv';

export enum ExportFormat {
  Csv = "csv",
  Json = "json",
  Xml = "xml",
  Html = "html",
  Markdown = "markdown",
}

export type ExportFormatString = 'csv' | 'json' | 'xml' | 'html' | 'markdown';
export type ReportTemplate = 'executive' | 'technical' | 'full';

export enum ExportDataType {
  Findings = "findings",
  Hosts = "hosts",
  Ports = "ports",
}

/** Color palette for severity visualization */
export const SEVERITY_COLORS = {
  [Severity.Critical]: "#dc2626", // red-600
  [Severity.High]: "#ea580c", // orange-600
  [Severity.Medium]: "#f59e0b", // amber-500
  [Severity.Low]: "#3b82f6", // blue-500
  [Severity.Info]: "#64748b", // slate-500
} as const;

/** Helper functions */

export function getSeverityColor(severity: Severity): string {
  return SEVERITY_COLORS[severity];
}

export function getSeverityBadgeClass(severity: Severity): string {
  const baseClasses = "px-2 py-1 rounded-full text-xs font-semibold";

  switch (severity) {
    case Severity.Critical:
      return `${baseClasses} bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200`;
    case Severity.High:
      return `${baseClasses} bg-orange-100 text-orange-800 dark:bg-orange-900 dark:text-orange-200`;
    case Severity.Medium:
      return `${baseClasses} bg-amber-100 text-amber-800 dark:bg-amber-900 dark:text-amber-200`;
    case Severity.Low:
      return `${baseClasses} bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200`;
    case Severity.Info:
      return `${baseClasses} bg-slate-100 text-slate-800 dark:bg-slate-900 dark:text-slate-200`;
  }
}

export function formatTimestamp(timestamp: string): string {
  const date = new Date(timestamp);
  return new Intl.DateTimeFormat("en-US", {
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  }).format(date);
}

export function formatDuration(ms: number): string {
  if (ms < 1000) return `${ms}ms`;
  if (ms < 60000) return `${(ms / 1000).toFixed(1)}s`;
  if (ms < 3600000) return `${(ms / 60000).toFixed(1)}m`;
  return `${(ms / 3600000).toFixed(1)}h`;
}
