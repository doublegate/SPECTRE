//! Built-in workflow templates

use std::collections::HashMap;

use super::dsl::{StepDef, WorkflowDef};

/// Built-in workflow templates
pub struct WorkflowTemplates {
    templates: HashMap<String, WorkflowDef>,
}

impl WorkflowTemplates {
    /// Create a new template collection with built-in templates
    pub fn new() -> Self {
        let mut templates = HashMap::new();

        templates.insert("recon-to-report".to_string(), Self::recon_to_report());
        templates.insert("vuln-scan-chain".to_string(), Self::vuln_scan_chain());
        templates.insert("web-audit".to_string(), Self::web_audit());

        Self { templates }
    }

    /// Get a template by name
    pub fn get(&self, name: &str) -> Option<&WorkflowDef> {
        self.templates.get(name)
    }

    /// List all template names
    pub fn list(&self) -> Vec<&str> {
        let mut names: Vec<_> = self.templates.keys().map(|s| s.as_str()).collect();
        names.sort();
        names
    }

    /// Get template count
    pub fn count(&self) -> usize {
        self.templates.len()
    }

    /// Recon-to-report workflow template
    fn recon_to_report() -> WorkflowDef {
        let mut wf = WorkflowDef::new("recon-to-report");
        wf.description = "Perform reconnaissance and generate a report".to_string();
        wf.tags = vec!["recon".to_string(), "report".to_string()];

        wf.add_step(
            StepDef::new("discovery", "scan")
                .with_param(
                    "template",
                    serde_json::Value::String("quick-recon".to_string()),
                )
                .with_capture("discovery_results"),
        );
        wf.add_step(
            StepDef::new("detailed-scan", "scan")
                .with_param(
                    "template",
                    serde_json::Value::String("full-audit".to_string()),
                )
                .with_capture("detailed_results"),
        );
        wf.add_step(
            StepDef::new("generate-report", "report")
                .with_param("format", serde_json::Value::String("html".to_string())),
        );

        wf
    }

    /// Vulnerability scan chain template
    fn vuln_scan_chain() -> WorkflowDef {
        let mut wf = WorkflowDef::new("vuln-scan-chain");
        wf.description = "Multi-stage vulnerability scanning workflow".to_string();
        wf.tags = vec!["vulnerability".to_string(), "assessment".to_string()];

        wf.add_step(
            StepDef::new("port-scan", "scan")
                .with_param(
                    "template",
                    serde_json::Value::String("quick-recon".to_string()),
                )
                .with_capture("open_ports"),
        );
        wf.add_step(
            StepDef::new("service-enum", "scan")
                .with_param("service_detection", serde_json::Value::Bool(true))
                .with_capture("services"),
        );
        wf.add_step(
            StepDef::new("export-results", "export")
                .with_param("format", serde_json::Value::String("csv".to_string())),
        );

        wf
    }

    /// Web audit workflow template
    fn web_audit() -> WorkflowDef {
        let mut wf = WorkflowDef::new("web-audit");
        wf.description = "Web application audit workflow".to_string();
        wf.tags = vec!["web".to_string(), "audit".to_string()];

        wf.add_step(
            StepDef::new("web-discovery", "scan")
                .with_param(
                    "template",
                    serde_json::Value::String("web-enum".to_string()),
                )
                .with_capture("web_hosts"),
        );
        wf.add_step(
            StepDef::new("generate-report", "report")
                .with_param("format", serde_json::Value::String("markdown".to_string())),
        );

        wf
    }
}

impl Default for WorkflowTemplates {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_templates_new() {
        let templates = WorkflowTemplates::new();
        assert!(templates.count() >= 3);
    }

    #[test]
    fn test_get_template() {
        let templates = WorkflowTemplates::new();
        assert!(templates.get("recon-to-report").is_some());
        assert!(templates.get("vuln-scan-chain").is_some());
        assert!(templates.get("web-audit").is_some());
        assert!(templates.get("nonexistent").is_none());
    }

    #[test]
    fn test_list_templates() {
        let templates = WorkflowTemplates::new();
        let list = templates.list();
        assert!(list.len() >= 3);
        // Verify sorted
        for window in list.windows(2) {
            assert!(window[0] <= window[1]);
        }
    }

    #[test]
    fn test_recon_to_report_template() {
        let templates = WorkflowTemplates::new();
        let wf = templates.get("recon-to-report").unwrap();
        assert_eq!(wf.step_count(), 3);
    }

    #[test]
    fn test_vuln_scan_chain_template() {
        let templates = WorkflowTemplates::new();
        let wf = templates.get("vuln-scan-chain").unwrap();
        assert!(wf.step_count() >= 2);
    }
}
