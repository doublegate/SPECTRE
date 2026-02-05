//! Workflow DSL definitions
//!
//! YAML/TOML-based workflow definition language.

use serde::{Deserialize, Serialize};

/// A workflow definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDef {
    /// Workflow name
    pub name: String,
    /// Workflow description
    #[serde(default)]
    pub description: String,
    /// Workflow version
    #[serde(default = "default_version")]
    pub version: String,
    /// Workflow steps
    pub steps: Vec<StepDef>,
    /// Global variables
    #[serde(default)]
    pub variables: std::collections::HashMap<String, String>,
    /// Tags
    #[serde(default)]
    pub tags: Vec<String>,
}

fn default_version() -> String {
    "1.0.0".to_string()
}

impl WorkflowDef {
    /// Create a new workflow definition
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: String::new(),
            version: "1.0.0".to_string(),
            steps: Vec::new(),
            variables: std::collections::HashMap::new(),
            tags: Vec::new(),
        }
    }

    /// Add a step
    pub fn add_step(&mut self, step: StepDef) {
        self.steps.push(step);
    }

    /// Set a variable
    pub fn set_variable(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }

    /// Get step count
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }
}

/// A step in a workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepDef {
    /// Step name
    pub name: String,
    /// Step type/action
    pub action: String,
    /// Action parameters
    #[serde(default)]
    pub params: std::collections::HashMap<String, serde_json::Value>,
    /// Condition for execution
    #[serde(default)]
    pub condition: Option<Condition>,
    /// Loop specification
    #[serde(default)]
    pub loop_spec: Option<LoopSpec>,
    /// Variable to capture output into
    #[serde(default)]
    pub capture: Option<String>,
    /// Description
    #[serde(default)]
    pub description: String,
    /// Retry count on failure
    #[serde(default)]
    pub retries: u32,
    /// Continue workflow on step failure
    #[serde(default)]
    pub continue_on_error: bool,
}

impl StepDef {
    /// Create a new step definition
    pub fn new(name: &str, action: &str) -> Self {
        Self {
            name: name.to_string(),
            action: action.to_string(),
            params: std::collections::HashMap::new(),
            condition: None,
            loop_spec: None,
            capture: None,
            description: String::new(),
            retries: 0,
            continue_on_error: false,
        }
    }

    /// Set a parameter
    pub fn with_param(mut self, key: &str, value: serde_json::Value) -> Self {
        self.params.insert(key.to_string(), value);
        self
    }

    /// Set condition
    pub fn with_condition(mut self, condition: Condition) -> Self {
        self.condition = Some(condition);
        self
    }

    /// Set loop specification
    pub fn with_loop(mut self, loop_spec: LoopSpec) -> Self {
        self.loop_spec = Some(loop_spec);
        self
    }

    /// Set variable capture
    pub fn with_capture(mut self, var_name: &str) -> Self {
        self.capture = Some(var_name.to_string());
        self
    }

    /// Set retries
    pub fn with_retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }

    /// Set continue on error
    pub fn with_continue_on_error(mut self) -> Self {
        self.continue_on_error = true;
        self
    }
}

/// Condition for step execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    /// Variable to check
    pub variable: String,
    /// Comparison operator
    pub operator: ConditionOp,
    /// Value to compare against
    pub value: String,
}

/// Condition operators
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConditionOp {
    /// Equal
    Eq,
    /// Not equal
    Ne,
    /// Contains (string)
    Contains,
    /// Not empty
    NotEmpty,
    /// Is empty
    IsEmpty,
    /// Greater than (numeric)
    Gt,
    /// Less than (numeric)
    Lt,
}

impl Condition {
    /// Create an equality condition
    pub fn equals(variable: &str, value: &str) -> Self {
        Self {
            variable: variable.to_string(),
            operator: ConditionOp::Eq,
            value: value.to_string(),
        }
    }

    /// Create a not-empty condition
    pub fn not_empty(variable: &str) -> Self {
        Self {
            variable: variable.to_string(),
            operator: ConditionOp::NotEmpty,
            value: String::new(),
        }
    }

    /// Create a contains condition
    pub fn contains(variable: &str, substring: &str) -> Self {
        Self {
            variable: variable.to_string(),
            operator: ConditionOp::Contains,
            value: substring.to_string(),
        }
    }

    /// Evaluate this condition against a variable value
    pub fn evaluate(&self, actual_value: Option<&str>) -> bool {
        let actual = actual_value.unwrap_or("");
        match self.operator {
            ConditionOp::Eq => actual == self.value,
            ConditionOp::Ne => actual != self.value,
            ConditionOp::Contains => actual.contains(&self.value),
            ConditionOp::NotEmpty => !actual.is_empty(),
            ConditionOp::IsEmpty => actual.is_empty(),
            ConditionOp::Gt => {
                let a: f64 = actual.parse().unwrap_or(0.0);
                let b: f64 = self.value.parse().unwrap_or(0.0);
                a > b
            },
            ConditionOp::Lt => {
                let a: f64 = actual.parse().unwrap_or(0.0);
                let b: f64 = self.value.parse().unwrap_or(0.0);
                a < b
            },
        }
    }
}

/// Loop specification for step iteration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopSpec {
    /// Variable containing items to iterate over (comma-separated or JSON array)
    pub over: String,
    /// Variable name for current item
    pub variable: String,
}

impl LoopSpec {
    /// Create a new loop specification
    pub fn new(over: &str, variable: &str) -> Self {
        Self {
            over: over.to_string(),
            variable: variable.to_string(),
        }
    }

    /// Parse items from the source value
    pub fn parse_items(&self, value: &str) -> Vec<String> {
        // Try JSON array first
        if let Ok(items) = serde_json::from_str::<Vec<String>>(value) {
            return items;
        }

        // Fall back to comma-separated
        value
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_def_new() {
        let wf = WorkflowDef::new("test-workflow");
        assert_eq!(wf.name, "test-workflow");
        assert_eq!(wf.version, "1.0.0");
        assert_eq!(wf.step_count(), 0);
    }

    #[test]
    fn test_workflow_def_add_steps() {
        let mut wf = WorkflowDef::new("test");
        wf.add_step(StepDef::new("step-1", "scan"));
        wf.add_step(StepDef::new("step-2", "report"));
        assert_eq!(wf.step_count(), 2);
    }

    #[test]
    fn test_workflow_def_variables() {
        let mut wf = WorkflowDef::new("test");
        wf.set_variable("target", "10.0.0.1");
        assert_eq!(wf.variables.get("target"), Some(&"10.0.0.1".to_string()));
    }

    #[test]
    fn test_step_def_builder() {
        let step = StepDef::new("scan-step", "scan")
            .with_param("ports", serde_json::json!([80, 443]))
            .with_capture("scan_results")
            .with_retries(3)
            .with_continue_on_error();

        assert_eq!(step.name, "scan-step");
        assert_eq!(step.action, "scan");
        assert_eq!(step.capture, Some("scan_results".to_string()));
        assert_eq!(step.retries, 3);
        assert!(step.continue_on_error);
    }

    #[test]
    fn test_condition_equals() {
        let cond = Condition::equals("status", "success");
        assert!(cond.evaluate(Some("success")));
        assert!(!cond.evaluate(Some("failure")));
    }

    #[test]
    fn test_condition_not_empty() {
        let cond = Condition::not_empty("results");
        assert!(cond.evaluate(Some("data")));
        assert!(!cond.evaluate(Some("")));
        assert!(!cond.evaluate(None));
    }

    #[test]
    fn test_condition_contains() {
        let cond = Condition::contains("output", "open");
        assert!(cond.evaluate(Some("port 80 is open")));
        assert!(!cond.evaluate(Some("all ports closed")));
    }

    #[test]
    fn test_condition_ne() {
        let cond = Condition {
            variable: "status".to_string(),
            operator: ConditionOp::Ne,
            value: "failed".to_string(),
        };
        assert!(cond.evaluate(Some("success")));
        assert!(!cond.evaluate(Some("failed")));
    }

    #[test]
    fn test_condition_gt_lt() {
        let gt = Condition {
            variable: "count".to_string(),
            operator: ConditionOp::Gt,
            value: "5".to_string(),
        };
        assert!(gt.evaluate(Some("10")));
        assert!(!gt.evaluate(Some("3")));

        let lt = Condition {
            variable: "count".to_string(),
            operator: ConditionOp::Lt,
            value: "5".to_string(),
        };
        assert!(lt.evaluate(Some("3")));
        assert!(!lt.evaluate(Some("10")));
    }

    #[test]
    fn test_condition_is_empty() {
        let cond = Condition {
            variable: "v".to_string(),
            operator: ConditionOp::IsEmpty,
            value: String::new(),
        };
        assert!(cond.evaluate(Some("")));
        assert!(cond.evaluate(None));
        assert!(!cond.evaluate(Some("data")));
    }

    #[test]
    fn test_loop_spec_parse_csv() {
        let ls = LoopSpec::new("targets", "target");
        let items = ls.parse_items("10.0.0.1, 10.0.0.2, 10.0.0.3");
        assert_eq!(items.len(), 3);
        assert_eq!(items[0], "10.0.0.1");
    }

    #[test]
    fn test_loop_spec_parse_json() {
        let ls = LoopSpec::new("targets", "target");
        let items = ls.parse_items(r#"["10.0.0.1","10.0.0.2"]"#);
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn test_workflow_def_serialization() {
        let mut wf = WorkflowDef::new("test");
        wf.add_step(StepDef::new("step-1", "scan"));
        wf.set_variable("target", "10.0.0.1");

        let json = serde_json::to_string(&wf).unwrap();
        let deserialized: WorkflowDef = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, "test");
        assert_eq!(deserialized.step_count(), 1);
    }

    #[test]
    fn test_workflow_def_yaml() {
        let yaml = r#"
name: "test-workflow"
description: "A test workflow"
steps:
  - name: "scan"
    action: "scan"
    params:
      ports: [80, 443]
"#;
        let wf: WorkflowDef = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(wf.name, "test-workflow");
        assert_eq!(wf.step_count(), 1);
    }

    #[test]
    fn test_step_with_condition_serialization() {
        let step = StepDef::new("conditional", "report")
            .with_condition(Condition::equals("scan_status", "complete"));

        let json = serde_json::to_string(&step).unwrap();
        let deserialized: StepDef = serde_json::from_str(&json).unwrap();
        assert!(deserialized.condition.is_some());
    }
}
