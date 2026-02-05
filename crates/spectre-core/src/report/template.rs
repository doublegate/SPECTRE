//! Report template configuration

use serde::{Deserialize, Serialize};

/// Report configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportConfig {
    /// Report title
    pub title: String,
    /// Sections to include
    pub sections: Vec<ReportSection>,
    /// Include executive summary
    pub include_summary: bool,
    /// Include findings table
    pub include_findings: bool,
    /// Include technical details
    pub include_details: bool,
    /// Custom CSS for HTML reports
    pub custom_css: Option<String>,
}

impl Default for ReportConfig {
    fn default() -> Self {
        Self {
            title: "SPECTRE Scan Report".to_string(),
            sections: vec![
                ReportSection::Summary,
                ReportSection::Findings,
                ReportSection::HostDetails,
                ReportSection::Statistics,
            ],
            include_summary: true,
            include_findings: true,
            include_details: true,
            custom_css: None,
        }
    }
}

/// Report sections
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportSection {
    /// Executive summary
    Summary,
    /// Findings table
    Findings,
    /// Per-host details
    HostDetails,
    /// Statistics
    Statistics,
    /// Custom section
    Custom(String),
}

/// Report template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportTemplate {
    /// Template name
    pub name: String,
    /// Template description
    pub description: String,
    /// Report configuration
    pub config: ReportConfig,
}

impl ReportTemplate {
    /// Default full report template
    pub fn full() -> Self {
        Self {
            name: "full".to_string(),
            description: "Full report with all sections".to_string(),
            config: ReportConfig::default(),
        }
    }

    /// Summary-only report template
    pub fn summary_only() -> Self {
        Self {
            name: "summary".to_string(),
            description: "Executive summary only".to_string(),
            config: ReportConfig {
                title: "SPECTRE Executive Summary".to_string(),
                sections: vec![ReportSection::Summary],
                include_summary: true,
                include_findings: false,
                include_details: false,
                custom_css: None,
            },
        }
    }

    /// Findings-focused report template
    pub fn findings_only() -> Self {
        Self {
            name: "findings".to_string(),
            description: "Findings table only".to_string(),
            config: ReportConfig {
                title: "SPECTRE Findings Report".to_string(),
                sections: vec![ReportSection::Findings],
                include_summary: false,
                include_findings: true,
                include_details: false,
                custom_css: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ReportConfig::default();
        assert!(config.include_summary);
        assert!(config.include_findings);
        assert!(config.include_details);
        assert!(!config.sections.is_empty());
    }

    #[test]
    fn test_template_full() {
        let template = ReportTemplate::full();
        assert_eq!(template.name, "full");
        assert_eq!(template.config.sections.len(), 4);
    }

    #[test]
    fn test_template_summary() {
        let template = ReportTemplate::summary_only();
        assert_eq!(template.config.sections.len(), 1);
        assert!(!template.config.include_findings);
    }

    #[test]
    fn test_template_findings() {
        let template = ReportTemplate::findings_only();
        assert!(template.config.include_findings);
        assert!(!template.config.include_summary);
    }

    #[test]
    fn test_report_section_custom() {
        let section = ReportSection::Custom("methodology".to_string());
        assert!(matches!(section, ReportSection::Custom(ref s) if s == "methodology"));
    }

    #[test]
    fn test_template_serialization() {
        let template = ReportTemplate::full();
        let json = serde_json::to_string(&template).unwrap();
        let deserialized: ReportTemplate = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, "full");
    }
}
