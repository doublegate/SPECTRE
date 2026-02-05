//! Predefined scan templates for common scan configurations
//!
//! Provides ready-to-use scan templates: quick-recon, full-audit,
//! stealth-enum, web-enum, etc.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tracing::info;

use crate::scan::{ScanType, TimingTemplate};

/// A predefined scan template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanTemplate {
    /// Template name
    pub name: String,
    /// Template description
    pub description: String,
    /// Scan type to use
    pub scan_type: ScanType,
    /// Ports to scan
    pub ports: Vec<u16>,
    /// Timing template
    pub timing: TimingTemplate,
    /// Enable service detection
    pub service_detection: bool,
    /// Enable OS detection
    pub os_detection: bool,
    /// Tags for categorization
    pub tags: Vec<String>,
}

impl ScanTemplate {
    /// Create a new scan template
    pub fn new(name: &str, scan_type: ScanType) -> Self {
        Self {
            name: name.to_string(),
            description: String::new(),
            scan_type,
            ports: Vec::new(),
            timing: TimingTemplate::Normal,
            service_detection: false,
            os_detection: false,
            tags: Vec::new(),
        }
    }

    /// Quick reconnaissance template - fast scan of common ports
    pub fn quick_recon() -> Self {
        Self {
            name: "quick-recon".to_string(),
            description: "Fast scan of top 100 common ports".to_string(),
            scan_type: ScanType::Connect,
            ports: top_100_ports(),
            timing: TimingTemplate::Aggressive,
            service_detection: false,
            os_detection: false,
            tags: vec!["recon".to_string(), "fast".to_string()],
        }
    }

    /// Full audit template - comprehensive scan with service and OS detection
    pub fn full_audit() -> Self {
        Self {
            name: "full-audit".to_string(),
            description: "Comprehensive scan of all common ports with service/OS detection"
                .to_string(),
            scan_type: ScanType::Syn,
            ports: top_1000_ports(),
            timing: TimingTemplate::Normal,
            service_detection: true,
            os_detection: true,
            tags: vec!["audit".to_string(), "comprehensive".to_string()],
        }
    }

    /// Stealth enumeration template - slow and careful scan
    pub fn stealth_enum() -> Self {
        Self {
            name: "stealth-enum".to_string(),
            description: "Slow, stealthy enumeration to avoid IDS detection".to_string(),
            scan_type: ScanType::Syn,
            ports: top_100_ports(),
            timing: TimingTemplate::Sneaky,
            service_detection: false,
            os_detection: false,
            tags: vec!["stealth".to_string(), "ids-evasion".to_string()],
        }
    }

    /// Web enumeration template - scan web-related ports
    pub fn web_enum() -> Self {
        Self {
            name: "web-enum".to_string(),
            description: "Enumerate web services on common HTTP/HTTPS ports".to_string(),
            scan_type: ScanType::Connect,
            ports: vec![80, 443, 8080, 8443, 8000, 8888, 3000, 5000, 9090, 9443],
            timing: TimingTemplate::Normal,
            service_detection: true,
            os_detection: false,
            tags: vec!["web".to_string(), "http".to_string()],
        }
    }

    /// UDP scan template
    pub fn udp_scan() -> Self {
        Self {
            name: "udp-scan".to_string(),
            description: "Scan common UDP services".to_string(),
            scan_type: ScanType::Udp,
            ports: vec![53, 67, 68, 69, 123, 161, 162, 500, 514, 1900],
            timing: TimingTemplate::Normal,
            service_detection: true,
            os_detection: false,
            tags: vec!["udp".to_string()],
        }
    }

    /// Set description
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    /// Set ports
    pub fn with_ports(mut self, ports: Vec<u16>) -> Self {
        self.ports = ports;
        self
    }

    /// Set timing template
    pub fn with_timing(mut self, timing: TimingTemplate) -> Self {
        self.timing = timing;
        self
    }
}

/// Library of scan templates
#[derive(Debug, Clone)]
pub struct TemplateLibrary {
    templates: HashMap<String, ScanTemplate>,
}

impl TemplateLibrary {
    /// Create a new template library with built-in templates
    pub fn new() -> Self {
        let mut lib = Self {
            templates: HashMap::new(),
        };
        lib.load_builtins();
        lib
    }

    /// Load built-in templates
    fn load_builtins(&mut self) {
        let builtins = vec![
            ScanTemplate::quick_recon(),
            ScanTemplate::full_audit(),
            ScanTemplate::stealth_enum(),
            ScanTemplate::web_enum(),
            ScanTemplate::udp_scan(),
        ];

        for template in builtins {
            self.templates.insert(template.name.clone(), template);
        }

        info!(
            count = self.templates.len(),
            "Loaded built-in scan templates"
        );
    }

    /// Get a template by name
    pub fn get(&self, name: &str) -> Option<&ScanTemplate> {
        self.templates.get(name)
    }

    /// Register a custom template
    pub fn register(&mut self, template: ScanTemplate) {
        self.templates.insert(template.name.clone(), template);
    }

    /// List all available templates
    pub fn list(&self) -> Vec<&ScanTemplate> {
        let mut templates: Vec<_> = self.templates.values().collect();
        templates.sort_by(|a, b| a.name.cmp(&b.name));
        templates
    }

    /// Search templates by tag
    pub fn search_by_tag(&self, tag: &str) -> Vec<&ScanTemplate> {
        self.templates
            .values()
            .filter(|t| t.tags.iter().any(|tt| tt == tag))
            .collect()
    }

    /// Get the number of templates
    pub fn count(&self) -> usize {
        self.templates.len()
    }
}

impl Default for TemplateLibrary {
    fn default() -> Self {
        Self::new()
    }
}

/// Return the top 100 common ports
fn top_100_ports() -> Vec<u16> {
    vec![
        7, 20, 21, 22, 23, 25, 43, 53, 67, 68, 69, 79, 80, 88, 110, 111, 113, 119, 123, 135, 137,
        138, 139, 143, 161, 162, 179, 194, 201, 264, 389, 443, 445, 464, 500, 512, 513, 514, 515,
        520, 521, 540, 548, 554, 587, 631, 636, 873, 902, 989, 990, 993, 995, 1025, 1026, 1080,
        1194, 1433, 1434, 1521, 1701, 1723, 1883, 1900, 2049, 2082, 2083, 2086, 2087, 2181, 2222,
        2375, 2376, 3128, 3268, 3306, 3389, 3690, 4443, 4444, 5000, 5060, 5222, 5432, 5555, 5672,
        5900, 5901, 5984, 6379, 6660, 6667, 7001, 7002, 8000, 8080, 8443, 8888, 9090, 9200, 9443,
    ]
}

/// Return the top 1000 common ports (abbreviated for practical use)
fn top_1000_ports() -> Vec<u16> {
    // Use a range of the most commonly scanned ports
    let mut ports = top_100_ports();
    // Add additional commonly used ports
    let extra: Vec<u16> = (1..=1024).collect();
    for p in extra {
        if !ports.contains(&p) {
            ports.push(p);
        }
    }
    ports.sort();
    ports.dedup();
    ports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_quick_recon() {
        let template = ScanTemplate::quick_recon();
        assert_eq!(template.name, "quick-recon");
        assert!(!template.ports.is_empty());
        assert!(!template.service_detection);
        assert!(matches!(template.timing, TimingTemplate::Aggressive));
    }

    #[test]
    fn test_template_full_audit() {
        let template = ScanTemplate::full_audit();
        assert_eq!(template.name, "full-audit");
        assert!(template.service_detection);
        assert!(template.os_detection);
    }

    #[test]
    fn test_template_stealth_enum() {
        let template = ScanTemplate::stealth_enum();
        assert_eq!(template.name, "stealth-enum");
        assert!(matches!(template.timing, TimingTemplate::Sneaky));
    }

    #[test]
    fn test_template_web_enum() {
        let template = ScanTemplate::web_enum();
        assert!(template.ports.contains(&80));
        assert!(template.ports.contains(&443));
        assert!(template.ports.contains(&8080));
    }

    #[test]
    fn test_template_udp() {
        let template = ScanTemplate::udp_scan();
        assert!(matches!(template.scan_type, ScanType::Udp));
    }

    #[test]
    fn test_template_builder_methods() {
        let template = ScanTemplate::new("custom", ScanType::Fin)
            .with_description("Custom scan")
            .with_ports(vec![1234])
            .with_timing(TimingTemplate::Paranoid);

        assert_eq!(template.name, "custom");
        assert_eq!(template.description, "Custom scan");
        assert_eq!(template.ports, vec![1234]);
    }

    #[test]
    fn test_template_library_new() {
        let lib = TemplateLibrary::new();
        assert!(lib.count() >= 5); // At least the 5 built-in templates
    }

    #[test]
    fn test_template_library_get() {
        let lib = TemplateLibrary::new();
        assert!(lib.get("quick-recon").is_some());
        assert!(lib.get("nonexistent").is_none());
    }

    #[test]
    fn test_template_library_register() {
        let mut lib = TemplateLibrary::new();
        let initial = lib.count();
        lib.register(ScanTemplate::new("custom", ScanType::Connect));
        assert_eq!(lib.count(), initial + 1);
        assert!(lib.get("custom").is_some());
    }

    #[test]
    fn test_template_library_list() {
        let lib = TemplateLibrary::new();
        let list = lib.list();
        assert!(!list.is_empty());
        // Verify sorted by name
        for window in list.windows(2) {
            assert!(window[0].name <= window[1].name);
        }
    }

    #[test]
    fn test_template_library_search_by_tag() {
        let lib = TemplateLibrary::new();
        let recon = lib.search_by_tag("recon");
        assert!(!recon.is_empty());
        let nonexistent = lib.search_by_tag("nonexistent-tag");
        assert!(nonexistent.is_empty());
    }

    #[test]
    fn test_template_serialization() {
        let template = ScanTemplate::quick_recon();
        let json = serde_json::to_string(&template).unwrap();
        let deserialized: ScanTemplate = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, "quick-recon");
    }
}
