//! Workflow parser — parse YAML/JSON workflow definitions

use super::dsl::WorkflowDef;

/// Workflow parser supporting multiple formats
pub struct WorkflowParser;

impl WorkflowParser {
    /// Parse a workflow from YAML
    pub fn from_yaml(yaml: &str) -> crate::Result<WorkflowDef> {
        serde_yaml::from_str(yaml).map_err(|e| {
            crate::SpectreError::Workflow(crate::error::WorkflowError::ParseError(format!(
                "YAML parse error: {}",
                e
            )))
        })
    }

    /// Parse a workflow from JSON
    pub fn from_json(json: &str) -> crate::Result<WorkflowDef> {
        serde_json::from_str(json).map_err(|e| {
            crate::SpectreError::Workflow(crate::error::WorkflowError::ParseError(format!(
                "JSON parse error: {}",
                e
            )))
        })
    }

    /// Parse a workflow from TOML
    pub fn from_toml(toml_str: &str) -> crate::Result<WorkflowDef> {
        toml::from_str(toml_str).map_err(|e| {
            crate::SpectreError::Workflow(crate::error::WorkflowError::ParseError(format!(
                "TOML parse error: {}",
                e
            )))
        })
    }

    /// Auto-detect format and parse
    pub fn parse(content: &str) -> crate::Result<WorkflowDef> {
        // Try JSON first (starts with '{')
        let trimmed = content.trim();
        if trimmed.starts_with('{') {
            return Self::from_json(content);
        }

        // Try YAML (most common)
        if let Ok(wf) = Self::from_yaml(content) {
            return Ok(wf);
        }

        // Try TOML
        Self::from_toml(content)
    }

    /// Serialize a workflow to YAML
    pub fn to_yaml(workflow: &WorkflowDef) -> crate::Result<String> {
        serde_yaml::to_string(workflow).map_err(|e| {
            crate::SpectreError::Workflow(crate::error::WorkflowError::ParseError(format!(
                "YAML serialization error: {}",
                e
            )))
        })
    }

    /// Serialize a workflow to JSON
    pub fn to_json(workflow: &WorkflowDef) -> crate::Result<String> {
        serde_json::to_string_pretty(workflow).map_err(|e| {
            crate::SpectreError::Workflow(crate::error::WorkflowError::ParseError(format!(
                "JSON serialization error: {}",
                e
            )))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_yaml() {
        let yaml = r#"
name: "recon-workflow"
description: "Basic recon"
steps:
  - name: "initial-scan"
    action: "scan"
    params:
      ports: [80, 443]
  - name: "generate-report"
    action: "report"
"#;
        let wf = WorkflowParser::from_yaml(yaml).unwrap();
        assert_eq!(wf.name, "recon-workflow");
        assert_eq!(wf.step_count(), 2);
    }

    #[test]
    fn test_parse_json() {
        let json = r#"{
            "name": "json-workflow",
            "description": "From JSON",
            "steps": [
                {"name": "step-1", "action": "scan"}
            ]
        }"#;
        let wf = WorkflowParser::from_json(json).unwrap();
        assert_eq!(wf.name, "json-workflow");
    }

    #[test]
    fn test_parse_invalid_yaml() {
        let result = WorkflowParser::from_yaml("invalid: [yaml: broken");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_json() {
        let result = WorkflowParser::from_json("{invalid json}");
        assert!(result.is_err());
    }

    #[test]
    fn test_auto_detect_json() {
        let json = r#"{"name": "auto", "steps": [{"name": "s1", "action": "scan"}]}"#;
        let wf = WorkflowParser::parse(json).unwrap();
        assert_eq!(wf.name, "auto");
    }

    #[test]
    fn test_auto_detect_yaml() {
        let yaml = "name: auto-yaml\nsteps:\n  - name: s1\n    action: scan\n";
        let wf = WorkflowParser::parse(yaml).unwrap();
        assert_eq!(wf.name, "auto-yaml");
    }

    #[test]
    fn test_to_yaml() {
        let mut wf = WorkflowDef::new("test");
        wf.add_step(super::super::dsl::StepDef::new("step-1", "scan"));
        let yaml = WorkflowParser::to_yaml(&wf).unwrap();
        assert!(yaml.contains("test"));
    }

    #[test]
    fn test_to_json() {
        let wf = WorkflowDef::new("test");
        let json = WorkflowParser::to_json(&wf).unwrap();
        assert!(json.contains("test"));
    }

    #[test]
    fn test_roundtrip_yaml() {
        let mut wf = WorkflowDef::new("roundtrip");
        wf.description = "Test roundtrip".to_string();
        wf.add_step(super::super::dsl::StepDef::new("s1", "scan"));

        let yaml = WorkflowParser::to_yaml(&wf).unwrap();
        let parsed = WorkflowParser::from_yaml(&yaml).unwrap();
        assert_eq!(parsed.name, "roundtrip");
        assert_eq!(parsed.step_count(), 1);
    }
}
