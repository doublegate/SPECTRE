//! Campaign state management module
//!
//! Provides campaign lifecycle management with:
//! - Campaign creation and configuration
//! - Phase management (planning, recon, analysis, exploitation, reporting)
//! - SQLite persistence
//! - Artifact storage
//! - Export/import (JSON/YAML)
//! - Campaign status reporting and metrics

mod artifact;
mod state;
mod storage;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use artifact::{Artifact, ArtifactType};
pub use state::CampaignPhase;
pub use storage::CampaignStore;

/// A campaign representing a complete offensive security operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Campaign {
    /// Unique campaign identifier
    pub id: String,
    /// Campaign name/codename
    pub name: String,
    /// Campaign description
    pub description: String,
    /// Current phase
    pub phase: CampaignPhase,
    /// Campaign objectives
    pub objectives: Vec<String>,
    /// Target networks/systems
    pub targets: Vec<String>,
    /// When the campaign was created
    pub created_at: DateTime<Utc>,
    /// When the campaign was last updated
    pub updated_at: DateTime<Utc>,
    /// When the campaign started
    pub started_at: Option<DateTime<Utc>>,
    /// When the campaign completed
    pub completed_at: Option<DateTime<Utc>>,
    /// Campaign tags for organization
    pub tags: Vec<String>,
    /// Campaign notes
    pub notes: String,
}

impl Campaign {
    /// Create a new campaign
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            description: description.to_string(),
            phase: CampaignPhase::Planning,
            objectives: Vec::new(),
            targets: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            started_at: None,
            completed_at: None,
            tags: Vec::new(),
            notes: String::new(),
        }
    }

    /// Add an objective
    pub fn add_objective(&mut self, objective: &str) {
        self.objectives.push(objective.to_string());
        self.updated_at = Utc::now();
    }

    /// Add a target
    pub fn add_target(&mut self, target: &str) {
        self.targets.push(target.to_string());
        self.updated_at = Utc::now();
    }

    /// Add a tag
    pub fn add_tag(&mut self, tag: &str) {
        if !self.tags.contains(&tag.to_string()) {
            self.tags.push(tag.to_string());
            self.updated_at = Utc::now();
        }
    }

    /// Advance to the next phase
    pub fn advance_phase(&mut self) -> crate::Result<()> {
        let next = self.phase.next().ok_or_else(|| {
            crate::SpectreError::Campaign(crate::error::CampaignError::InvalidState(format!(
                "No phase after {}",
                self.phase
            )))
        })?;

        if self.started_at.is_none() && next == CampaignPhase::Recon {
            self.started_at = Some(Utc::now());
        }

        if next == CampaignPhase::Complete {
            self.completed_at = Some(Utc::now());
        }

        self.phase = next;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Set a specific phase
    pub fn set_phase(&mut self, phase: CampaignPhase) {
        self.phase = phase;
        self.updated_at = Utc::now();
    }

    /// Check if the campaign is active (not planning and not complete)
    pub fn is_active(&self) -> bool {
        !matches!(
            self.phase,
            CampaignPhase::Planning | CampaignPhase::Complete | CampaignPhase::Archived
        )
    }

    /// Check if the campaign is complete
    pub fn is_complete(&self) -> bool {
        matches!(
            self.phase,
            CampaignPhase::Complete | CampaignPhase::Archived
        )
    }

    /// Export campaign as JSON
    pub fn to_json(&self) -> crate::Result<String> {
        serde_json::to_string_pretty(self).map_err(|e| {
            crate::SpectreError::Campaign(crate::error::CampaignError::ExportError(e.to_string()))
        })
    }

    /// Import campaign from JSON
    pub fn from_json(json: &str) -> crate::Result<Self> {
        serde_json::from_str(json).map_err(|e| {
            crate::SpectreError::Campaign(crate::error::CampaignError::ExportError(e.to_string()))
        })
    }

    /// Export campaign as YAML
    pub fn to_yaml(&self) -> crate::Result<String> {
        serde_yaml::to_string(self).map_err(|e| {
            crate::SpectreError::Campaign(crate::error::CampaignError::ExportError(e.to_string()))
        })
    }

    /// Import campaign from YAML
    pub fn from_yaml(yaml: &str) -> crate::Result<Self> {
        serde_yaml::from_str(yaml).map_err(|e| {
            crate::SpectreError::Campaign(crate::error::CampaignError::ExportError(e.to_string()))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_campaign_creation() {
        let campaign = Campaign::new("Operation NIGHTFALL", "Phase 2 testing campaign");
        assert!(!campaign.id.is_empty());
        assert_eq!(campaign.name, "Operation NIGHTFALL");
        assert_eq!(campaign.phase, CampaignPhase::Planning);
        assert!(!campaign.is_active());
        assert!(!campaign.is_complete());
    }

    #[test]
    fn test_campaign_objectives() {
        let mut campaign = Campaign::new("test", "test");
        campaign.add_objective("Enumerate all services");
        campaign.add_objective("Identify vulnerabilities");
        assert_eq!(campaign.objectives.len(), 2);
    }

    #[test]
    fn test_campaign_targets() {
        let mut campaign = Campaign::new("test", "test");
        campaign.add_target("192.168.1.0/24");
        campaign.add_target("10.0.0.0/8");
        assert_eq!(campaign.targets.len(), 2);
    }

    #[test]
    fn test_campaign_tags() {
        let mut campaign = Campaign::new("test", "test");
        campaign.add_tag("internal");
        campaign.add_tag("red-team");
        campaign.add_tag("internal"); // duplicate
        assert_eq!(campaign.tags.len(), 2);
    }

    #[test]
    fn test_campaign_phase_advancement() {
        let mut campaign = Campaign::new("test", "test");
        assert_eq!(campaign.phase, CampaignPhase::Planning);

        campaign.advance_phase().unwrap(); // -> Recon
        assert_eq!(campaign.phase, CampaignPhase::Recon);
        assert!(campaign.started_at.is_some());
        assert!(campaign.is_active());

        campaign.advance_phase().unwrap(); // -> Analysis
        assert_eq!(campaign.phase, CampaignPhase::Analysis);

        campaign.advance_phase().unwrap(); // -> Exploitation
        assert_eq!(campaign.phase, CampaignPhase::Exploitation);

        campaign.advance_phase().unwrap(); // -> Reporting
        assert_eq!(campaign.phase, CampaignPhase::Reporting);

        campaign.advance_phase().unwrap(); // -> Complete
        assert_eq!(campaign.phase, CampaignPhase::Complete);
        assert!(campaign.completed_at.is_some());
        assert!(campaign.is_complete());
    }

    #[test]
    fn test_campaign_json_roundtrip() {
        let campaign = Campaign::new("test", "test campaign");
        let json = campaign.to_json().unwrap();
        let loaded = Campaign::from_json(&json).unwrap();
        assert_eq!(campaign.id, loaded.id);
        assert_eq!(campaign.name, loaded.name);
    }

    #[test]
    fn test_campaign_yaml_roundtrip() {
        let campaign = Campaign::new("test", "test campaign");
        let yaml = campaign.to_yaml().unwrap();
        let loaded = Campaign::from_yaml(&yaml).unwrap();
        assert_eq!(campaign.id, loaded.id);
        assert_eq!(campaign.name, loaded.name);
    }

    #[test]
    fn test_campaign_complete_no_advance() {
        let mut campaign = Campaign::new("test", "test");
        campaign.set_phase(CampaignPhase::Complete);
        let result = campaign.advance_phase();
        assert!(result.is_err());
    }
}
