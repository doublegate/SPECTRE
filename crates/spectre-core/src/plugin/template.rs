//! Plugin templates — scaffolding for new plugin development
//!
//! Generates boilerplate plugin files (manifest + Lua script) for
//! common plugin types.

use std::path::Path;

use tracing::info;

/// Plugin template type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginTemplateType {
    /// Basic plugin (minimal boilerplate)
    Basic,
    /// Scanner plugin (service probe / detection)
    Scanner,
    /// Report plugin (custom report generation)
    Report,
    /// Workflow plugin (custom workflow steps)
    Workflow,
    /// Analysis plugin (data analysis / CyberChef integration)
    Analysis,
}

impl std::fmt::Display for PluginTemplateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Basic => write!(f, "basic"),
            Self::Scanner => write!(f, "scanner"),
            Self::Report => write!(f, "report"),
            Self::Workflow => write!(f, "workflow"),
            Self::Analysis => write!(f, "analysis"),
        }
    }
}

impl PluginTemplateType {
    /// List all available template types
    pub fn all() -> Vec<Self> {
        vec![
            Self::Basic,
            Self::Scanner,
            Self::Report,
            Self::Workflow,
            Self::Analysis,
        ]
    }

    /// Parse from string
    pub fn from_str_name(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "basic" => Some(Self::Basic),
            "scanner" | "scan" => Some(Self::Scanner),
            "report" => Some(Self::Report),
            "workflow" => Some(Self::Workflow),
            "analysis" => Some(Self::Analysis),
            _ => None,
        }
    }
}

/// Plugin template generator
pub struct PluginTemplateGenerator;

impl PluginTemplateGenerator {
    /// Generate a new plugin from a template
    ///
    /// Creates the plugin directory structure at `output_dir/name/`.
    pub fn generate(
        name: &str,
        template_type: PluginTemplateType,
        author: &str,
        output_dir: &Path,
    ) -> crate::Result<std::path::PathBuf> {
        let plugin_dir = output_dir.join(name);
        std::fs::create_dir_all(&plugin_dir)?;

        // Generate manifest
        let manifest = Self::generate_manifest(name, template_type, author);
        std::fs::write(plugin_dir.join("plugin.toml"), manifest)?;

        // Generate main script
        let script = Self::generate_script(name, template_type);
        std::fs::write(plugin_dir.join("init.lua"), script)?;

        info!(name = %name, template = %template_type, "Plugin generated");

        Ok(plugin_dir)
    }

    /// Generate the plugin.toml manifest content
    pub fn generate_manifest(
        name: &str,
        template_type: PluginTemplateType,
        author: &str,
    ) -> String {
        let (description, tags, needs_network) = match template_type {
            PluginTemplateType::Basic => (format!("{} plugin", name), "utility".to_string(), false),
            PluginTemplateType::Scanner => (
                format!("{} scanner plugin", name),
                "scanner, network".to_string(),
                true,
            ),
            PluginTemplateType::Report => (
                format!("{} report plugin", name),
                "report, output".to_string(),
                false,
            ),
            PluginTemplateType::Workflow => (
                format!("{} workflow plugin", name),
                "workflow, automation".to_string(),
                false,
            ),
            PluginTemplateType::Analysis => (
                format!("{} analysis plugin", name),
                "analysis, data".to_string(),
                false,
            ),
        };

        format!(
            r#"name = "{name}"
version = "0.1.0"
description = "{description}"
author = "{author}"
main = "init.lua"
tags = [{tags}]

[permissions]
network = {needs_network}
filesystem = false

[limits]
max_memory = 16777216
timeout_seconds = 30
"#,
            name = name,
            description = description,
            author = author,
            tags = tags
                .split(", ")
                .map(|t| format!("\"{}\"", t))
                .collect::<Vec<_>>()
                .join(", "),
            needs_network = needs_network,
        )
    }

    /// Generate the init.lua script content
    pub fn generate_script(name: &str, template_type: PluginTemplateType) -> String {
        match template_type {
            PluginTemplateType::Basic => Self::basic_script(name),
            PluginTemplateType::Scanner => Self::scanner_script(name),
            PluginTemplateType::Report => Self::report_script(name),
            PluginTemplateType::Workflow => Self::workflow_script(name),
            PluginTemplateType::Analysis => Self::analysis_script(name),
        }
    }

    fn basic_script(name: &str) -> String {
        format!(
            r#"-- {name} plugin for SPECTRE
-- Template: basic

spectre.log("Loading {name} plugin v" .. spectre.version())

-- Plugin initialization
function plugin_init()
    spectre.log("{name}: initialized")
end

-- Main entry point
function plugin_run(args)
    spectre.log("{name}: running")
    _spectre_output = "{name}:completed"
end

plugin_init()
"#,
            name = name,
        )
    }

    fn scanner_script(name: &str) -> String {
        format!(
            r#"-- {name} scanner plugin for SPECTRE
-- Template: scanner

spectre.log("Loading {name} scanner plugin")

-- Custom service probe
function probe_service(host, port)
    spectre.log("{name}: probing " .. host .. ":" .. port)
    -- Implement service detection logic here
    return nil
end

-- Hook: called when a new host is discovered
function on_host_discovered(context)
    spectre.log("{name}: new host " .. (context.host or "unknown"))
end

-- Hook: called when a service is identified
function on_service_identified(context)
    spectre.log("{name}: service " .. (context.service or "unknown"))
end

-- Main entry point
function plugin_run(args)
    spectre.log("{name}: scanner running")
    _spectre_output = "{name}:scan_complete"
end
"#,
            name = name,
        )
    }

    fn report_script(name: &str) -> String {
        format!(
            r#"-- {name} report plugin for SPECTRE
-- Template: report

spectre.log("Loading {name} report plugin")

-- Generate a custom report section
function generate_section(findings)
    spectre.log("{name}: generating report section")
    local output = "== Custom Report Section ==\n"
    -- Process findings and build output
    return output
end

-- Hook: called before report generation
function on_pre_report(context)
    spectre.log("{name}: preparing report data")
end

-- Hook: called after report generation
function on_post_report(context)
    spectre.log("{name}: report complete")
end

-- Main entry point
function plugin_run(args)
    _spectre_output = generate_section({{}})
end
"#,
            name = name,
        )
    }

    fn workflow_script(name: &str) -> String {
        format!(
            r#"-- {name} workflow plugin for SPECTRE
-- Template: workflow

spectre.log("Loading {name} workflow plugin")

-- Custom workflow step handler
function step_handler(step_name, params)
    spectre.log("{name}: executing step " .. step_name)
    -- Implement custom step logic
    return "step_completed"
end

-- Hook: called before a workflow step
function on_pre_workflow_step(context)
    spectre.log("{name}: pre-step " .. (context.step or "unknown"))
end

-- Hook: called after a workflow step
function on_post_workflow_step(context)
    spectre.log("{name}: post-step " .. (context.step or "unknown"))
end

-- Main entry point
function plugin_run(args)
    _spectre_output = step_handler("default", {{}})
end
"#,
            name = name,
        )
    }

    fn analysis_script(name: &str) -> String {
        format!(
            r#"-- {name} analysis plugin for SPECTRE
-- Template: analysis

spectre.log("Loading {name} analysis plugin")

-- Analyze scan data
function analyze(data)
    spectre.log("{name}: analyzing data")
    -- Implement analysis logic
    local result = {{}}
    return spectre.json_encode(result)
end

-- Transform data between formats
function transform(input, format)
    spectre.log("{name}: transforming to " .. (format or "default"))
    return input
end

-- Main entry point
function plugin_run(args)
    local result = analyze(args)
    _spectre_output = result
end
"#,
            name = name,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_type_display() {
        assert_eq!(PluginTemplateType::Basic.to_string(), "basic");
        assert_eq!(PluginTemplateType::Scanner.to_string(), "scanner");
        assert_eq!(PluginTemplateType::Report.to_string(), "report");
        assert_eq!(PluginTemplateType::Workflow.to_string(), "workflow");
        assert_eq!(PluginTemplateType::Analysis.to_string(), "analysis");
    }

    #[test]
    fn test_template_type_all() {
        let all = PluginTemplateType::all();
        assert_eq!(all.len(), 5);
    }

    #[test]
    fn test_template_type_from_str() {
        assert_eq!(
            PluginTemplateType::from_str_name("basic"),
            Some(PluginTemplateType::Basic)
        );
        assert_eq!(
            PluginTemplateType::from_str_name("scanner"),
            Some(PluginTemplateType::Scanner)
        );
        assert_eq!(
            PluginTemplateType::from_str_name("scan"),
            Some(PluginTemplateType::Scanner)
        );
        assert_eq!(
            PluginTemplateType::from_str_name("REPORT"),
            Some(PluginTemplateType::Report)
        );
        assert!(PluginTemplateType::from_str_name("nonexistent").is_none());
    }

    #[test]
    fn test_generate_basic_plugin() {
        let dir = tempfile::tempdir().unwrap();
        let path = PluginTemplateGenerator::generate(
            "test-basic",
            PluginTemplateType::Basic,
            "Test Author",
            dir.path(),
        )
        .unwrap();

        assert!(path.join("plugin.toml").exists());
        assert!(path.join("init.lua").exists());

        let manifest = std::fs::read_to_string(path.join("plugin.toml")).unwrap();
        assert!(manifest.contains("test-basic"));
        assert!(manifest.contains("Test Author"));

        let script = std::fs::read_to_string(path.join("init.lua")).unwrap();
        assert!(script.contains("test-basic"));
        assert!(script.contains("plugin_run"));
    }

    #[test]
    fn test_generate_scanner_plugin() {
        let dir = tempfile::tempdir().unwrap();
        let path = PluginTemplateGenerator::generate(
            "my-scanner",
            PluginTemplateType::Scanner,
            "Author",
            dir.path(),
        )
        .unwrap();

        let manifest = std::fs::read_to_string(path.join("plugin.toml")).unwrap();
        assert!(manifest.contains("network = true"));
        assert!(manifest.contains("\"scanner\""));

        let script = std::fs::read_to_string(path.join("init.lua")).unwrap();
        assert!(script.contains("probe_service"));
    }

    #[test]
    fn test_generate_report_plugin() {
        let dir = tempfile::tempdir().unwrap();
        let path = PluginTemplateGenerator::generate(
            "my-report",
            PluginTemplateType::Report,
            "Author",
            dir.path(),
        )
        .unwrap();

        let script = std::fs::read_to_string(path.join("init.lua")).unwrap();
        assert!(script.contains("generate_section"));
        assert!(script.contains("on_pre_report"));
    }

    #[test]
    fn test_generate_workflow_plugin() {
        let dir = tempfile::tempdir().unwrap();
        let path = PluginTemplateGenerator::generate(
            "my-workflow",
            PluginTemplateType::Workflow,
            "Author",
            dir.path(),
        )
        .unwrap();

        let script = std::fs::read_to_string(path.join("init.lua")).unwrap();
        assert!(script.contains("step_handler"));
        assert!(script.contains("on_pre_workflow_step"));
    }

    #[test]
    fn test_generate_analysis_plugin() {
        let dir = tempfile::tempdir().unwrap();
        let path = PluginTemplateGenerator::generate(
            "my-analysis",
            PluginTemplateType::Analysis,
            "Author",
            dir.path(),
        )
        .unwrap();

        let script = std::fs::read_to_string(path.join("init.lua")).unwrap();
        assert!(script.contains("analyze"));
        assert!(script.contains("transform"));
    }

    #[test]
    fn test_generate_manifest_content() {
        let manifest =
            PluginTemplateGenerator::generate_manifest("test", PluginTemplateType::Basic, "Test");
        assert!(manifest.contains("name = \"test\""));
        assert!(manifest.contains("version = \"0.1.0\""));
        assert!(manifest.contains("main = \"init.lua\""));
    }

    #[test]
    fn test_generate_script_content() {
        for template_type in PluginTemplateType::all() {
            let script = PluginTemplateGenerator::generate_script("test", template_type);
            assert!(
                script.contains("plugin_run"),
                "Template {} should contain plugin_run",
                template_type
            );
        }
    }
}
