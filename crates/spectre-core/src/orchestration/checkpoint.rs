//! Checkpoint/resume for interrupted scans
//!
//! Provides serializable checkpoint state and a store for persisting checkpoints.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

/// A scan checkpoint capturing state for resume
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    /// Unique checkpoint ID
    pub id: String,
    /// Associated scan chain or job name
    pub scan_name: String,
    /// Targets that have been completed
    pub completed_targets: Vec<String>,
    /// Targets remaining to be scanned
    pub remaining_targets: Vec<String>,
    /// Current step index (for scan chains)
    pub current_step: usize,
    /// When the checkpoint was created
    pub created_at: DateTime<Utc>,
    /// Partial results collected so far (serialized JSON)
    pub partial_results: String,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl Checkpoint {
    /// Create a new checkpoint
    pub fn new(id: &str, scan_name: &str) -> Self {
        Self {
            id: id.to_string(),
            scan_name: scan_name.to_string(),
            completed_targets: Vec::new(),
            remaining_targets: Vec::new(),
            current_step: 0,
            created_at: Utc::now(),
            partial_results: "[]".to_string(),
            metadata: HashMap::new(),
        }
    }

    /// Add a completed target
    pub fn mark_target_complete(&mut self, target: &str) {
        self.remaining_targets.retain(|t| t != target);
        if !self.completed_targets.contains(&target.to_string()) {
            self.completed_targets.push(target.to_string());
        }
    }

    /// Get progress as a fraction (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        let total = self.completed_targets.len() + self.remaining_targets.len();
        if total == 0 {
            return 0.0;
        }
        self.completed_targets.len() as f64 / total as f64
    }

    /// Set metadata
    pub fn set_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
    }
}

/// Store for persisting checkpoints to disk
#[derive(Debug, Clone)]
pub struct CheckpointStore {
    /// Directory where checkpoints are stored
    store_dir: PathBuf,
}

impl CheckpointStore {
    /// Create a new checkpoint store
    pub fn new(store_dir: &Path) -> Self {
        Self {
            store_dir: store_dir.to_path_buf(),
        }
    }

    /// Save a checkpoint to disk
    pub fn save(&self, checkpoint: &Checkpoint) -> crate::Result<()> {
        std::fs::create_dir_all(&self.store_dir)?;

        let path = self.checkpoint_path(&checkpoint.id);
        let json = serde_json::to_string_pretty(checkpoint).map_err(|e| {
            crate::SpectreError::Orchestration(crate::error::OrchestrationError::CheckpointError(
                format!("Serialization error: {}", e),
            ))
        })?;

        std::fs::write(&path, json)?;
        debug!(id = %checkpoint.id, path = %path.display(), "Checkpoint saved");
        Ok(())
    }

    /// Load a checkpoint from disk
    pub fn load(&self, id: &str) -> crate::Result<Checkpoint> {
        let path = self.checkpoint_path(id);
        if !path.exists() {
            return Err(crate::SpectreError::Orchestration(
                crate::error::OrchestrationError::CheckpointError(format!(
                    "Checkpoint not found: {}",
                    id
                )),
            ));
        }

        let content = std::fs::read_to_string(&path)?;
        let checkpoint: Checkpoint = serde_json::from_str(&content).map_err(|e| {
            crate::SpectreError::Orchestration(crate::error::OrchestrationError::CheckpointError(
                format!("Deserialization error: {}", e),
            ))
        })?;

        info!(id = %id, "Checkpoint loaded");
        Ok(checkpoint)
    }

    /// Delete a checkpoint
    pub fn delete(&self, id: &str) -> crate::Result<bool> {
        let path = self.checkpoint_path(id);
        if path.exists() {
            std::fs::remove_file(&path)?;
            debug!(id = %id, "Checkpoint deleted");
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// List all checkpoints
    pub fn list(&self) -> crate::Result<Vec<Checkpoint>> {
        if !self.store_dir.exists() {
            return Ok(Vec::new());
        }

        let mut checkpoints = Vec::new();
        for entry in std::fs::read_dir(&self.store_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                match std::fs::read_to_string(&path) {
                    Ok(content) => match serde_json::from_str::<Checkpoint>(&content) {
                        Ok(cp) => checkpoints.push(cp),
                        Err(e) => {
                            warn!(path = %path.display(), error = %e, "Failed to parse checkpoint")
                        },
                    },
                    Err(e) => {
                        warn!(path = %path.display(), error = %e, "Failed to read checkpoint")
                    },
                }
            }
        }

        checkpoints.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(checkpoints)
    }

    /// Get the file path for a checkpoint
    fn checkpoint_path(&self, id: &str) -> PathBuf {
        self.store_dir.join(format!("{}.json", id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkpoint_new() {
        let cp = Checkpoint::new("cp-1", "my-scan");
        assert_eq!(cp.id, "cp-1");
        assert_eq!(cp.scan_name, "my-scan");
        assert_eq!(cp.current_step, 0);
    }

    #[test]
    fn test_checkpoint_mark_target_complete() {
        let mut cp = Checkpoint::new("cp-1", "scan");
        cp.remaining_targets = vec![
            "10.0.0.1".to_string(),
            "10.0.0.2".to_string(),
            "10.0.0.3".to_string(),
        ];

        cp.mark_target_complete("10.0.0.1");
        assert_eq!(cp.completed_targets.len(), 1);
        assert_eq!(cp.remaining_targets.len(), 2);
    }

    #[test]
    fn test_checkpoint_progress() {
        let mut cp = Checkpoint::new("cp-1", "scan");
        cp.remaining_targets = vec!["10.0.0.1".to_string(), "10.0.0.2".to_string()];
        assert!((cp.progress() - 0.0).abs() < f64::EPSILON);

        cp.mark_target_complete("10.0.0.1");
        assert!((cp.progress() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_checkpoint_progress_empty() {
        let cp = Checkpoint::new("cp-1", "scan");
        assert!((cp.progress() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_checkpoint_metadata() {
        let mut cp = Checkpoint::new("cp-1", "scan");
        cp.set_metadata("key", "value");
        assert_eq!(cp.metadata.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_checkpoint_store_save_load() {
        let dir = tempfile::tempdir().unwrap();
        let store = CheckpointStore::new(dir.path());

        let mut cp = Checkpoint::new("cp-1", "test-scan");
        cp.remaining_targets = vec!["10.0.0.1".to_string()];
        cp.set_metadata("step", "1");

        store.save(&cp).unwrap();

        let loaded = store.load("cp-1").unwrap();
        assert_eq!(loaded.id, "cp-1");
        assert_eq!(loaded.scan_name, "test-scan");
        assert_eq!(loaded.remaining_targets.len(), 1);
        assert_eq!(loaded.metadata.get("step"), Some(&"1".to_string()));
    }

    #[test]
    fn test_checkpoint_store_load_not_found() {
        let dir = tempfile::tempdir().unwrap();
        let store = CheckpointStore::new(dir.path());
        assert!(store.load("nonexistent").is_err());
    }

    #[test]
    fn test_checkpoint_store_delete() {
        let dir = tempfile::tempdir().unwrap();
        let store = CheckpointStore::new(dir.path());

        let cp = Checkpoint::new("cp-1", "scan");
        store.save(&cp).unwrap();

        assert!(store.delete("cp-1").unwrap());
        assert!(!store.delete("cp-1").unwrap());
    }

    #[test]
    fn test_checkpoint_store_list() {
        let dir = tempfile::tempdir().unwrap();
        let store = CheckpointStore::new(dir.path());

        store.save(&Checkpoint::new("cp-1", "scan-1")).unwrap();
        store.save(&Checkpoint::new("cp-2", "scan-2")).unwrap();

        let list = store.list().unwrap();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_checkpoint_store_list_empty() {
        let dir = tempfile::tempdir().unwrap();
        let store = CheckpointStore::new(dir.path());
        let list = store.list().unwrap();
        assert!(list.is_empty());
    }

    #[test]
    fn test_checkpoint_serialization() {
        let cp = Checkpoint::new("cp-1", "scan");
        let json = serde_json::to_string(&cp).unwrap();
        let deserialized: Checkpoint = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, "cp-1");
    }
}
