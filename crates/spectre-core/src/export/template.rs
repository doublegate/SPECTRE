//! Custom export templates

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// An export template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportTemplate {
    /// Template name
    pub name: String,
    /// Template description
    pub description: String,
    /// Template body with `{{variable}}` placeholders
    pub body: String,
    /// Header (prepended to output)
    pub header: Option<String>,
    /// Footer (appended to output)
    pub footer: Option<String>,
    /// Separator between items
    pub separator: String,
}

impl ExportTemplate {
    /// Create a new export template
    pub fn new(name: &str, body: &str) -> Self {
        Self {
            name: name.to_string(),
            description: String::new(),
            body: body.to_string(),
            header: None,
            footer: None,
            separator: "\n".to_string(),
        }
    }

    /// Set description
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    /// Set header
    pub fn with_header(mut self, header: &str) -> Self {
        self.header = Some(header.to_string());
        self
    }

    /// Set footer
    pub fn with_footer(mut self, footer: &str) -> Self {
        self.footer = Some(footer.to_string());
        self
    }

    /// Set separator
    pub fn with_separator(mut self, sep: &str) -> Self {
        self.separator = sep.to_string();
        self
    }
}

/// Template engine for rendering export templates
pub struct ExportTemplateEngine;

impl ExportTemplateEngine {
    /// Render a template with a single set of variables
    pub fn render(template: &ExportTemplate, variables: &HashMap<String, String>) -> String {
        let mut result = String::new();

        if let Some(ref header) = template.header {
            result.push_str(header);
            result.push('\n');
        }

        let rendered_body = Self::substitute(&template.body, variables);
        result.push_str(&rendered_body);

        if let Some(ref footer) = template.footer {
            result.push('\n');
            result.push_str(footer);
        }

        result
    }

    /// Render a template with multiple sets of variables (one per item)
    pub fn render_items(template: &ExportTemplate, items: &[HashMap<String, String>]) -> String {
        let mut result = String::new();

        if let Some(ref header) = template.header {
            result.push_str(header);
            result.push('\n');
        }

        let rendered: Vec<String> = items
            .iter()
            .map(|vars| Self::substitute(&template.body, vars))
            .collect();

        result.push_str(&rendered.join(&template.separator));

        if let Some(ref footer) = template.footer {
            result.push('\n');
            result.push_str(footer);
        }

        result
    }

    /// Substitute `{{variable}}` placeholders
    fn substitute(template: &str, variables: &HashMap<String, String>) -> String {
        let mut result = template.to_string();
        for (key, value) in variables {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_new() {
        let tpl = ExportTemplate::new("test", "{{host}}:{{port}}");
        assert_eq!(tpl.name, "test");
        assert_eq!(tpl.body, "{{host}}:{{port}}");
    }

    #[test]
    fn test_template_builder() {
        let tpl = ExportTemplate::new("test", "{{data}}")
            .with_description("Test template")
            .with_header("--- START ---")
            .with_footer("--- END ---")
            .with_separator("; ");

        assert_eq!(tpl.description, "Test template");
        assert!(tpl.header.is_some());
        assert!(tpl.footer.is_some());
        assert_eq!(tpl.separator, "; ");
    }

    #[test]
    fn test_render_single() {
        let tpl = ExportTemplate::new("test", "Host: {{host}}, Port: {{port}}");
        let mut vars = HashMap::new();
        vars.insert("host".to_string(), "10.0.0.1".to_string());
        vars.insert("port".to_string(), "80".to_string());

        let result = ExportTemplateEngine::render(&tpl, &vars);
        assert_eq!(result, "Host: 10.0.0.1, Port: 80");
    }

    #[test]
    fn test_render_with_header_footer() {
        let tpl = ExportTemplate::new("test", "{{value}}")
            .with_header("BEGIN")
            .with_footer("END");

        let mut vars = HashMap::new();
        vars.insert("value".to_string(), "data".to_string());

        let result = ExportTemplateEngine::render(&tpl, &vars);
        assert!(result.starts_with("BEGIN"));
        assert!(result.ends_with("END"));
        assert!(result.contains("data"));
    }

    #[test]
    fn test_render_items() {
        let tpl = ExportTemplate::new("test", "{{host}}:{{port}}").with_separator("\n");

        let items = vec![
            {
                let mut m = HashMap::new();
                m.insert("host".to_string(), "10.0.0.1".to_string());
                m.insert("port".to_string(), "80".to_string());
                m
            },
            {
                let mut m = HashMap::new();
                m.insert("host".to_string(), "10.0.0.2".to_string());
                m.insert("port".to_string(), "443".to_string());
                m
            },
        ];

        let result = ExportTemplateEngine::render_items(&tpl, &items);
        assert!(result.contains("10.0.0.1:80"));
        assert!(result.contains("10.0.0.2:443"));
    }

    #[test]
    fn test_render_unknown_variable() {
        let tpl = ExportTemplate::new("test", "{{known}} {{unknown}}");
        let mut vars = HashMap::new();
        vars.insert("known".to_string(), "value".to_string());

        let result = ExportTemplateEngine::render(&tpl, &vars);
        assert_eq!(result, "value {{unknown}}");
    }

    #[test]
    fn test_template_serialization() {
        let tpl = ExportTemplate::new("test", "{{data}}")
            .with_header("H")
            .with_footer("F");

        let json = serde_json::to_string(&tpl).unwrap();
        let deserialized: ExportTemplate = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, "test");
    }
}
