/** Mirrors Rust: crates/spectre-gui/src/commands/chef.rs */

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
