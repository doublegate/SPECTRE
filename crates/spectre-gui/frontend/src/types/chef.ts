/** Mirrors Rust: crates/spectre-gui/src/commands/chef.rs */

export interface ChefOperation {
  name: string;
  category: string;
  description: string;
}

export interface ChefRequest {
  operation: string;
  input: string;
  args?: Record<string, string>;
}

export interface ChefResult {
  operation: string;
  output: string;
  success: boolean;
}

export interface ChefStatus {
  healthy: boolean;
  container_status: string;
  operation_count: number;
}

export const CHEF_CATEGORIES = [
  'All',
  'Encoding',
  'Hashing',
  'Encryption',
  'Compression',
];
