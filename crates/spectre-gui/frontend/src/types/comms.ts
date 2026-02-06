/** Mirrors Rust: crates/spectre-gui/src/commands/comms.rs */

export interface IdentityInfo {
  name: string;
  fingerprint: string;
  created: string;
}

export interface PeerInfo {
  name: string;
  fingerprint: string;
  status: string;
  last_seen?: string;
}

export interface SendRequest {
  peer: string;
  data: string;
}
