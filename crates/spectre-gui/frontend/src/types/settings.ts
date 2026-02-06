/** Settings types for SPECTRE GUI - mirrors Rust backend */

/** Mirrors crates/spectre-gui/src/commands/config.rs::GeneralConfig */
export interface GeneralConfig {
  verbosity: number;
  color: boolean;
}

/** Mirrors crates/spectre-gui/src/commands/config.rs::ScanConfig */
export interface ScanConfig {
  default_ports: string;
  default_timing: number; // 0-5 (T0-T5)
  service_detection: boolean;
  os_detection: boolean;
  resolve_dns: boolean;
}

/** Mirrors crates/spectre-gui/src/commands/config.rs::ChefConfig */
export interface ChefConfig {
  docker_image: string;
  container_name: string;
  auto_start: boolean;
  timeout: number;
}

/** Mirrors crates/spectre-gui/src/commands/config.rs::CommsConfig */
export interface CommsConfig {
  default_expiration: number;
  compress: boolean;
  timeout: number;
}

/** Mirrors crates/spectre-gui/src/commands/config.rs::OutputConfig */
export interface OutputConfig {
  format: string;
  timestamps: boolean;
  pretty_json: boolean;
}

/** Mirrors crates/spectre-gui/src/commands/config.rs::AppConfig */
export interface AppConfig {
  general: GeneralConfig;
  scan: ScanConfig;
  chef: ChefConfig;
  comms: CommsConfig;
  output: OutputConfig;
}

/** Mirrors crates/spectre-gui/src/commands/config.rs::ConfigUpdate */
export interface ConfigUpdate {
  general?: GeneralConfig;
  scan?: ScanConfig;
  chef?: ChefConfig;
  comms?: CommsConfig;
  output?: OutputConfig;
}

/** Default configuration (matches Rust backend defaults) */
export const DEFAULT_CONFIG: AppConfig = {
  general: {
    verbosity: 0,
    color: true,
  },
  scan: {
    default_ports: 'common',
    default_timing: 3,
    service_detection: false,
    os_detection: false,
    resolve_dns: true,
  },
  chef: {
    docker_image: 'doublegate/cyberchef-mcp:latest',
    container_name: 'spectre-cyberchef',
    auto_start: true,
    timeout: 30,
  },
  comms: {
    default_expiration: 0,
    compress: false,
    timeout: 30,
  },
  output: {
    format: 'table',
    timestamps: true,
    pretty_json: true,
  },
};

/** Timing templates (T0-T5) */
export const TIMING_TEMPLATES = [
  { value: 0, label: 'T0 - Paranoid', description: 'Slowest, most evasive' },
  { value: 1, label: 'T1 - Sneaky', description: 'Slow, stealthy' },
  { value: 2, label: 'T2 - Polite', description: 'Moderate speed' },
  { value: 3, label: 'T3 - Normal', description: 'Default timing' },
  { value: 4, label: 'T4 - Aggressive', description: 'Fast scanning' },
  { value: 5, label: 'T5 - Insane', description: 'Fastest, no delays' },
];

/** Available themes */
export const THEMES = [
  { value: 'dark', label: 'Dark', description: 'Default dark theme' },
  { value: 'light', label: 'Light', description: 'Light mode' },
  { value: 'tactical', label: 'Tactical', description: 'Tactical green theme' },
  { value: 'matrix', label: 'Matrix', description: 'Matrix-style green' },
  { value: 'hacker', label: 'Hacker', description: 'Retro terminal theme' },
];

/** Log levels (verbosity settings) */
export const VERBOSITY_LEVELS = [
  { value: 0, label: 'Normal', description: 'Standard output' },
  { value: 1, label: 'Verbose', description: 'More detailed output' },
  { value: 2, label: 'Very Verbose', description: 'Very detailed output' },
  { value: 3, label: 'Debug', description: 'Debug information' },
];

/** Port range presets */
export const PORT_PRESETS = [
  { value: 'common', label: 'Common Ports', description: 'Top 1000 most common' },
  { value: '1-1024', label: 'Well-Known', description: 'Ports 1-1024' },
  { value: '1-65535', label: 'All Ports', description: 'Full range' },
];

/** Output formats */
export const OUTPUT_FORMATS = [
  { value: 'table', label: 'Table', description: 'Human-readable table' },
  { value: 'json', label: 'JSON', description: 'JSON format' },
  { value: 'yaml', label: 'YAML', description: 'YAML format' },
  { value: 'xml', label: 'XML', description: 'XML format' },
  { value: 'csv', label: 'CSV', description: 'CSV spreadsheet' },
];
