//! Job state machine implementation

use std::fmt;

use serde::{Deserialize, Serialize};

/// Job lifecycle states
///
/// State transitions:
/// ```text
/// Created  -> Queued
/// Queued   -> Running, Cancelled
/// Running  -> Paused, Complete, Failed, Cancelled
/// Paused   -> Running, Cancelled
/// Complete -> (terminal)
/// Failed   -> (terminal)
/// Cancelled -> (terminal)
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JobState {
    /// Job has been created but not yet queued
    Created,
    /// Job is queued for execution
    Queued,
    /// Job is currently running
    Running,
    /// Job is paused (can be resumed)
    Paused,
    /// Job completed successfully
    Complete,
    /// Job failed with an error
    Failed,
    /// Job was cancelled by the user
    Cancelled,
}

impl JobState {
    /// Check if a transition to the given state is valid
    pub const fn can_transition_to(&self, target: &Self) -> bool {
        matches!(
            (self, target),
            (Self::Created, Self::Queued)
                | (Self::Queued | Self::Paused, Self::Running)
                | (Self::Queued | Self::Running | Self::Paused, Self::Cancelled)
                | (Self::Running, Self::Paused | Self::Complete | Self::Failed)
        )
    }

    /// Check if this is a terminal state
    pub const fn is_terminal(&self) -> bool {
        matches!(self, Self::Complete | Self::Failed | Self::Cancelled)
    }

    /// Get all valid next states from the current state
    pub fn valid_transitions(&self) -> Vec<Self> {
        match self {
            Self::Created => vec![Self::Queued],
            Self::Queued | Self::Paused => vec![Self::Running, Self::Cancelled],
            Self::Running => vec![Self::Paused, Self::Complete, Self::Failed, Self::Cancelled],
            Self::Complete | Self::Failed | Self::Cancelled => vec![],
        }
    }
}

impl fmt::Display for JobState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Created => write!(f, "created"),
            Self::Queued => write!(f, "queued"),
            Self::Running => write!(f, "running"),
            Self::Paused => write!(f, "paused"),
            Self::Complete => write!(f, "complete"),
            Self::Failed => write!(f, "failed"),
            Self::Cancelled => write!(f, "cancelled"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transitions() {
        assert!(JobState::Created.can_transition_to(&JobState::Queued));
        assert!(JobState::Queued.can_transition_to(&JobState::Running));
        assert!(JobState::Running.can_transition_to(&JobState::Complete));
        assert!(JobState::Running.can_transition_to(&JobState::Failed));
        assert!(JobState::Running.can_transition_to(&JobState::Cancelled));
        assert!(JobState::Running.can_transition_to(&JobState::Paused));
        assert!(JobState::Paused.can_transition_to(&JobState::Running));
        assert!(JobState::Paused.can_transition_to(&JobState::Cancelled));
    }

    #[test]
    fn test_invalid_transitions() {
        assert!(!JobState::Created.can_transition_to(&JobState::Running));
        assert!(!JobState::Created.can_transition_to(&JobState::Complete));
        assert!(!JobState::Complete.can_transition_to(&JobState::Running));
        assert!(!JobState::Failed.can_transition_to(&JobState::Running));
        assert!(!JobState::Cancelled.can_transition_to(&JobState::Running));
    }

    #[test]
    fn test_is_terminal() {
        assert!(!JobState::Created.is_terminal());
        assert!(!JobState::Queued.is_terminal());
        assert!(!JobState::Running.is_terminal());
        assert!(!JobState::Paused.is_terminal());
        assert!(JobState::Complete.is_terminal());
        assert!(JobState::Failed.is_terminal());
        assert!(JobState::Cancelled.is_terminal());
    }

    #[test]
    fn test_valid_transitions_list() {
        let transitions = JobState::Running.valid_transitions();
        assert_eq!(transitions.len(), 4);
        assert!(transitions.contains(&JobState::Paused));
        assert!(transitions.contains(&JobState::Complete));
        assert!(transitions.contains(&JobState::Failed));
        assert!(transitions.contains(&JobState::Cancelled));
    }

    #[test]
    fn test_terminal_no_transitions() {
        assert!(JobState::Complete.valid_transitions().is_empty());
        assert!(JobState::Failed.valid_transitions().is_empty());
        assert!(JobState::Cancelled.valid_transitions().is_empty());
    }

    #[test]
    fn test_state_display() {
        assert_eq!(JobState::Created.to_string(), "created");
        assert_eq!(JobState::Queued.to_string(), "queued");
        assert_eq!(JobState::Running.to_string(), "running");
        assert_eq!(JobState::Paused.to_string(), "paused");
        assert_eq!(JobState::Complete.to_string(), "complete");
        assert_eq!(JobState::Failed.to_string(), "failed");
        assert_eq!(JobState::Cancelled.to_string(), "cancelled");
    }
}
