/** Mirrors Rust: crates/spectre-gui/src/commands/campaign.rs */

export interface CampaignSummary {
  id: string;
  name: string;
  description: string;
  status: string;
  phase: CampaignPhase;
  created: string;
  target_count: number;
  objectives: string[];
}

export interface CampaignDetail extends CampaignSummary {
  targets: string[];
  artifacts: CampaignArtifact[];
  notes: string;
  started_at?: string;
  completed_at?: string;
}

export interface CreateCampaignRequest {
  name: string;
  description?: string;
  objectives?: string[];
  targets?: string[];
}

export interface ExportRequest {
  id: string;
  path: string;
}

export interface ImportRequest {
  path: string;
}

export type CampaignPhase =
  | "planning"
  | "recon"
  | "analysis"
  | "exploitation"
  | "reporting"
  | "complete"
  | "archived";

export interface Objective {
  id: string;
  description: string;
  completed: boolean;
  created: string;
}

export interface CampaignArtifact {
  id: string;
  type: ArtifactType;
  name: string;
  description?: string;
  size: number;
  created: string;
}

export type ArtifactType =
  | "scan_result"
  | "analysis_output"
  | "finding_report"
  | "screenshot"
  | "log_file"
  | "config_snapshot"
  | "custom";
