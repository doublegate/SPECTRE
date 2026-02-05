//! Workflow persistence — save/load/resume workflows

use std::path::{Path, PathBuf};

use tracing::{debug, info};

use super::dsl::WorkflowDef;
use super::executor::WorkflowExecutor;

/// Store for persisting workflows and their execution state
#[derive(Debug, Clone)]
pub struct WorkflowStore {
    store_dir: PathBuf,
}

impl WorkflowStore {
    /// Create a new workflow store
    pub fn new(store_dir: &Path) -> Self {
        Self {
            store_dir: store_dir.to_path_buf(),
        }
    }

    /// Save a workflow definition
    pub fn save_definition(&self, workflow: &WorkflowDef) -> crate::Result<()> {
        let dir = self.store_dir.join("definitions");
        std::fs::create_dir_all(&dir)?;

        let path = dir.join(format!("{}.yaml", workflow.name));
        let yaml = serde_yaml::to_string(workflow).map_err(|e| {
            crate::SpectreError::Workflow(crate::error::WorkflowError::PersistenceError(format!(
                "Serialization error: {}",
                e
            )))
        })?;

        std::fs::write(&path, yaml)?;
        debug!(name = %workflow.name, "Workflow definition saved");
        Ok(())
    }

    /// Load a workflow definition
    pub fn load_definition(&self, name: &str) -> crate::Result<WorkflowDef> {
        let path = self
            .store_dir
            .join("definitions")
            .join(format!("{}.yaml", name));
        if !path.exists() {
            return Err(crate::SpectreError::Workflow(
                crate::error::WorkflowError::TemplateNotFound(name.to_string()),
            ));
        }

        let content = std::fs::read_to_string(&path)?;
        serde_yaml::from_str(&content).map_err(|e| {
            crate::SpectreError::Workflow(crate::error::WorkflowError::ParseError(format!(
                "YAML parse error: {}",
                e
            )))
        })
    }

    /// Save workflow execution state
    pub fn save_state(&self, executor: &WorkflowExecutor) -> crate::Result<()> {
        let dir = self.store_dir.join("state");
        std::fs::create_dir_all(&dir)?;

        let path = dir.join(format!("{}.json", executor.workflow.name));
        let json = serde_json::to_string_pretty(executor).map_err(|e| {
            crate::SpectreError::Workflow(crate::error::WorkflowError::PersistenceError(format!(
                "Serialization error: {}",
                e
            )))
        })?;

        std::fs::write(&path, json)?;
        info!(name = %executor.workflow.name, "Workflow state saved");
        Ok(())
    }

    /// Load workflow execution state
    pub fn load_state(&self, name: &str) -> crate::Result<WorkflowExecutor> {
        let path = self.store_dir.join("state").join(format!("{}.json", name));
        if !path.exists() {
            return Err(crate::SpectreError::Workflow(
                crate::error::WorkflowError::PersistenceError(format!(
                    "No saved state for workflow: {}",
                    name
                )),
            ));
        }

        let content = std::fs::read_to_string(&path)?;
        serde_json::from_str(&content).map_err(|e| {
            crate::SpectreError::Workflow(crate::error::WorkflowError::PersistenceError(format!(
                "Deserialization error: {}",
                e
            )))
        })
    }

    /// List all saved workflow definitions
    pub fn list_definitions(&self) -> crate::Result<Vec<String>> {
        let dir = self.store_dir.join("definitions");
        if !dir.exists() {
            return Ok(Vec::new());
        }

        let mut names = Vec::new();
        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("yaml") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    names.push(stem.to_string());
                }
            }
        }

        names.sort();
        Ok(names)
    }

    /// Delete a workflow definition
    pub fn delete_definition(&self, name: &str) -> crate::Result<bool> {
        let path = self
            .store_dir
            .join("definitions")
            .join(format!("{}.yaml", name));
        if path.exists() {
            std::fs::remove_file(&path)?;
            debug!(name = %name, "Workflow definition deleted");
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Delete saved execution state
    pub fn delete_state(&self, name: &str) -> crate::Result<bool> {
        let path = self.store_dir.join("state").join(format!("{}.json", name));
        if path.exists() {
            std::fs::remove_file(&path)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workflow::dsl::StepDef;

    fn make_workflow() -> WorkflowDef {
        let mut wf = WorkflowDef::new("test-workflow");
        wf.description = "Test workflow".to_string();
        wf.add_step(StepDef::new("step-1", "scan"));
        wf
    }

    #[test]
    fn test_save_load_definition() {
        let dir = tempfile::tempdir().unwrap();
        let store = WorkflowStore::new(dir.path());

        let wf = make_workflow();
        store.save_definition(&wf).unwrap();

        let loaded = store.load_definition("test-workflow").unwrap();
        assert_eq!(loaded.name, "test-workflow");
        assert_eq!(loaded.step_count(), 1);
    }

    #[test]
    fn test_load_definition_not_found() {
        let dir = tempfile::tempdir().unwrap();
        let store = WorkflowStore::new(dir.path());
        assert!(store.load_definition("nonexistent").is_err());
    }

    #[test]
    fn test_save_load_state() {
        let dir = tempfile::tempdir().unwrap();
        let store = WorkflowStore::new(dir.path());

        let wf = make_workflow();
        let executor = WorkflowExecutor::new(wf);
        store.save_state(&executor).unwrap();

        let loaded = store.load_state("test-workflow").unwrap();
        assert_eq!(loaded.workflow.name, "test-workflow");
    }

    #[test]
    fn test_load_state_not_found() {
        let dir = tempfile::tempdir().unwrap();
        let store = WorkflowStore::new(dir.path());
        assert!(store.load_state("nonexistent").is_err());
    }

    #[test]
    fn test_list_definitions() {
        let dir = tempfile::tempdir().unwrap();
        let store = WorkflowStore::new(dir.path());

        store.save_definition(&make_workflow()).unwrap();

        let mut wf2 = WorkflowDef::new("another-workflow");
        wf2.add_step(StepDef::new("s1", "scan"));
        store.save_definition(&wf2).unwrap();

        let list = store.list_definitions().unwrap();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_list_definitions_empty() {
        let dir = tempfile::tempdir().unwrap();
        let store = WorkflowStore::new(dir.path());
        let list = store.list_definitions().unwrap();
        assert!(list.is_empty());
    }

    #[test]
    fn test_delete_definition() {
        let dir = tempfile::tempdir().unwrap();
        let store = WorkflowStore::new(dir.path());

        store.save_definition(&make_workflow()).unwrap();
        assert!(store.delete_definition("test-workflow").unwrap());
        assert!(!store.delete_definition("test-workflow").unwrap());
    }

    #[test]
    fn test_delete_state() {
        let dir = tempfile::tempdir().unwrap();
        let store = WorkflowStore::new(dir.path());

        let executor = WorkflowExecutor::new(make_workflow());
        store.save_state(&executor).unwrap();
        assert!(store.delete_state("test-workflow").unwrap());
        assert!(!store.delete_state("test-workflow").unwrap());
    }
}
