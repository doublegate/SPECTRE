/** Mirrors Rust: crates/spectre-gui/src/commands/campaign.rs */

export interface CampaignSummary {
  name: string;
  status: string;
  phase: string;
  created: string;
  target_count: number;
}

export interface CreateCampaignRequest {
  name: string;
  description?: string;
  targets?: string[];
}

export type CampaignPhase =
  | "planning"
  | "recon"
  | "analysis"
  | "exploitation"
  | "post_exploitation"
  | "reporting"
  | "complete";
