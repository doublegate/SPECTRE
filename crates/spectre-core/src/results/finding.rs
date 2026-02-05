//! Finding and Host structures for vulnerability-style results

use std::fmt;

use serde::{Deserialize, Serialize};

/// Finding severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FindingSeverity {
    /// Informational finding
    Info,
    /// Low severity
    Low,
    /// Medium severity
    Medium,
    /// High severity
    High,
    /// Critical severity
    Critical,
}

impl fmt::Display for FindingSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Info => write!(f, "info"),
            Self::Low => write!(f, "low"),
            Self::Medium => write!(f, "medium"),
            Self::High => write!(f, "high"),
            Self::Critical => write!(f, "critical"),
        }
    }
}

/// A security finding from scan analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    /// Finding title
    pub title: String,
    /// Detailed description
    pub description: String,
    /// Severity level
    pub severity: FindingSeverity,
    /// Affected host
    pub host: String,
    /// Affected port (if applicable)
    pub port: Option<u16>,
    /// Affected service
    pub service: Option<String>,
    /// Remediation recommendation
    pub remediation: Option<String>,
    /// CVE references
    pub references: Vec<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
}

impl Finding {
    /// Create a new finding
    pub fn new(title: &str, host: &str, severity: FindingSeverity) -> Self {
        Self {
            title: title.to_string(),
            description: String::new(),
            severity,
            host: host.to_string(),
            port: None,
            service: None,
            remediation: None,
            references: Vec::new(),
            tags: Vec::new(),
        }
    }

    /// Set the description
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    /// Set the affected port
    pub const fn with_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// Set the affected service
    pub fn with_service(mut self, service: &str) -> Self {
        self.service = Some(service.to_string());
        self
    }

    /// Set the remediation recommendation
    pub fn with_remediation(mut self, remediation: &str) -> Self {
        self.remediation = Some(remediation.to_string());
        self
    }

    /// Add a reference
    pub fn add_reference(&mut self, reference: &str) {
        self.references.push(reference.to_string());
    }

    /// Add a tag
    pub fn add_tag(&mut self, tag: &str) {
        self.tags.push(tag.to_string());
    }
}

/// Aggregated host information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Host {
    /// IP address
    pub ip: String,
    /// Resolved hostname
    pub hostname: Option<String>,
    /// Detected OS
    pub os: Option<String>,
    /// Detected OS version
    pub os_version: Option<String>,
    /// Services found on this host
    pub services: Vec<Service>,
    /// Time taken to scan in milliseconds
    pub scan_time_ms: u64,
}

impl Host {
    /// Get the number of open services
    pub fn open_services(&self) -> usize {
        self.services.iter().filter(|s| s.state == "Open").count()
    }

    /// Get a service by port number
    pub fn service_on_port(&self, port: u16) -> Option<&Service> {
        self.services.iter().find(|s| s.port == port)
    }
}

/// A network service found on a host
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    /// Port number
    pub port: u16,
    /// Protocol (tcp/udp)
    pub protocol: String,
    /// Port state
    pub state: String,
    /// Service name
    pub service_name: Option<String>,
    /// Service version
    pub version: Option<String>,
    /// Service banner
    pub banner: Option<String>,
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.service_name.as_deref().unwrap_or("unknown");
        let version = self
            .version
            .as_deref()
            .map(|v| format!(" {}", v))
            .unwrap_or_default();

        write!(
            f,
            "{}/{} {} {}{}",
            self.port, self.protocol, self.state, name, version
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finding_creation() {
        let finding = Finding::new("Open SSH", "10.0.0.1", FindingSeverity::Info);
        assert_eq!(finding.title, "Open SSH");
        assert_eq!(finding.host, "10.0.0.1");
        assert_eq!(finding.severity, FindingSeverity::Info);
    }

    #[test]
    fn test_finding_builder() {
        let finding = Finding::new("Weak SSH", "10.0.0.1", FindingSeverity::Medium)
            .with_description("SSH allows weak ciphers")
            .with_port(22)
            .with_service("ssh")
            .with_remediation("Disable weak ciphers in sshd_config");

        assert_eq!(finding.port, Some(22));
        assert_eq!(finding.service.as_deref(), Some("ssh"));
        assert!(finding.remediation.is_some());
    }

    #[test]
    fn test_finding_references() {
        let mut finding = Finding::new("Test", "10.0.0.1", FindingSeverity::High);
        finding.add_reference("CVE-2024-1234");
        finding.add_tag("ssh");

        assert_eq!(finding.references.len(), 1);
        assert_eq!(finding.tags.len(), 1);
    }

    #[test]
    fn test_severity_ordering() {
        assert!(FindingSeverity::Info < FindingSeverity::Low);
        assert!(FindingSeverity::Low < FindingSeverity::Medium);
        assert!(FindingSeverity::Medium < FindingSeverity::High);
        assert!(FindingSeverity::High < FindingSeverity::Critical);
    }

    #[test]
    fn test_severity_display() {
        assert_eq!(FindingSeverity::Info.to_string(), "info");
        assert_eq!(FindingSeverity::Critical.to_string(), "critical");
    }

    #[test]
    fn test_host_open_services() {
        let host = Host {
            ip: "10.0.0.1".to_string(),
            hostname: None,
            os: None,
            os_version: None,
            services: vec![
                Service {
                    port: 22,
                    protocol: "tcp".to_string(),
                    state: "Open".to_string(),
                    service_name: Some("ssh".to_string()),
                    version: None,
                    banner: None,
                },
                Service {
                    port: 80,
                    protocol: "tcp".to_string(),
                    state: "Closed".to_string(),
                    service_name: None,
                    version: None,
                    banner: None,
                },
            ],
            scan_time_ms: 100,
        };

        assert_eq!(host.open_services(), 1);
        assert!(host.service_on_port(22).is_some());
        assert!(host.service_on_port(443).is_none());
    }

    #[test]
    fn test_service_display() {
        let service = Service {
            port: 22,
            protocol: "tcp".to_string(),
            state: "Open".to_string(),
            service_name: Some("ssh".to_string()),
            version: Some("OpenSSH 8.9".to_string()),
            banner: None,
        };

        let display = service.to_string();
        assert!(display.contains("22/tcp"));
        assert!(display.contains("ssh"));
        assert!(display.contains("OpenSSH 8.9"));
    }
}
