//! Campaign phase state machine

use std::fmt;

use serde::{Deserialize, Serialize};

/// Campaign operational phases
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CampaignPhase {
    /// Planning phase - defining objectives and scope
    Planning,
    /// Reconnaissance phase - gathering intelligence
    Recon,
    /// Analysis phase - processing gathered data
    Analysis,
    /// Exploitation phase - active testing
    Exploitation,
    /// Reporting phase - documenting findings
    Reporting,
    /// Campaign complete
    Complete,
    /// Campaign archived
    Archived,
}

impl CampaignPhase {
    /// Get the next phase in the campaign lifecycle
    pub const fn next(&self) -> Option<Self> {
        match self {
            Self::Planning => Some(Self::Recon),
            Self::Recon => Some(Self::Analysis),
            Self::Analysis => Some(Self::Exploitation),
            Self::Exploitation => Some(Self::Reporting),
            Self::Reporting => Some(Self::Complete),
            Self::Complete | Self::Archived => None,
        }
    }

    /// Get the previous phase
    pub const fn previous(&self) -> Option<Self> {
        match self {
            Self::Planning => None,
            Self::Recon => Some(Self::Planning),
            Self::Analysis => Some(Self::Recon),
            Self::Exploitation => Some(Self::Analysis),
            Self::Reporting => Some(Self::Exploitation),
            Self::Complete => Some(Self::Reporting),
            Self::Archived => Some(Self::Complete),
        }
    }

    /// Get all phases in order
    pub fn all() -> Vec<Self> {
        vec![
            Self::Planning,
            Self::Recon,
            Self::Analysis,
            Self::Exploitation,
            Self::Reporting,
            Self::Complete,
            Self::Archived,
        ]
    }

    /// Check if this is an active (non-terminal) phase
    pub const fn is_active(&self) -> bool {
        !matches!(self, Self::Planning | Self::Complete | Self::Archived)
    }
}

impl fmt::Display for CampaignPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Planning => write!(f, "planning"),
            Self::Recon => write!(f, "recon"),
            Self::Analysis => write!(f, "analysis"),
            Self::Exploitation => write!(f, "exploitation"),
            Self::Reporting => write!(f, "reporting"),
            Self::Complete => write!(f, "complete"),
            Self::Archived => write!(f, "archived"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_progression() {
        let mut phase = CampaignPhase::Planning;
        let expected = vec![
            CampaignPhase::Recon,
            CampaignPhase::Analysis,
            CampaignPhase::Exploitation,
            CampaignPhase::Reporting,
            CampaignPhase::Complete,
        ];

        for expected_next in expected {
            let next = phase.next().unwrap();
            assert_eq!(next, expected_next);
            phase = next;
        }

        assert!(phase.next().is_none());
    }

    #[test]
    fn test_phase_previous() {
        assert!(CampaignPhase::Planning.previous().is_none());
        assert_eq!(
            CampaignPhase::Recon.previous(),
            Some(CampaignPhase::Planning)
        );
        assert_eq!(
            CampaignPhase::Complete.previous(),
            Some(CampaignPhase::Reporting)
        );
    }

    #[test]
    fn test_all_phases() {
        let phases = CampaignPhase::all();
        assert_eq!(phases.len(), 7);
        assert_eq!(phases[0], CampaignPhase::Planning);
        assert_eq!(phases[6], CampaignPhase::Archived);
    }

    #[test]
    fn test_phase_is_active() {
        assert!(!CampaignPhase::Planning.is_active());
        assert!(CampaignPhase::Recon.is_active());
        assert!(CampaignPhase::Analysis.is_active());
        assert!(CampaignPhase::Exploitation.is_active());
        assert!(CampaignPhase::Reporting.is_active());
        assert!(!CampaignPhase::Complete.is_active());
        assert!(!CampaignPhase::Archived.is_active());
    }

    #[test]
    fn test_phase_display() {
        assert_eq!(CampaignPhase::Planning.to_string(), "planning");
        assert_eq!(CampaignPhase::Recon.to_string(), "recon");
        assert_eq!(CampaignPhase::Archived.to_string(), "archived");
    }
}
