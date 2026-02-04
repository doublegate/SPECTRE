//! Scan-related types and data structures

use std::net::IpAddr;
use std::path::PathBuf;
use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Scan type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ScanType {
    /// TCP SYN scan (stealth scan)
    Syn,
    /// TCP Connect scan
    #[default]
    Connect,
    /// UDP scan
    Udp,
    /// TCP ACK scan
    Ack,
    /// TCP FIN scan
    Fin,
    /// TCP Xmas scan
    Xmas,
    /// TCP Null scan
    Null,
    /// TCP Window scan
    Window,
}

/// Timing template for scan speed/stealth tradeoff
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TimingTemplate {
    /// Paranoid (T0) - Very slow, IDS evasion
    Paranoid,
    /// Sneaky (T1) - Slow, IDS evasion
    Sneaky,
    /// Polite (T2) - Less bandwidth usage
    Polite,
    /// Normal (T3) - Default timing
    #[default]
    Normal,
    /// Aggressive (T4) - Fast scan
    Aggressive,
    /// Insane (T5) - Very fast, may miss ports
    Insane,
}

/// Protocol type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Protocol {
    /// TCP protocol
    #[default]
    Tcp,
    /// UDP protocol
    Udp,
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tcp => write!(f, "tcp"),
            Self::Udp => write!(f, "udp"),
        }
    }
}

/// Port state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PortState {
    /// Port is open
    Open,
    /// Port is closed
    Closed,
    /// Port is filtered (firewall)
    Filtered,
    /// Port state is unknown
    Unknown,
    /// Port is open but filtered
    OpenFiltered,
    /// Port is closed but filtered
    ClosedFiltered,
}

/// Target specification
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    /// Single IP address
    Ip(IpAddr),
    /// CIDR network range
    Cidr(ipnetwork::IpNetwork),
    /// Hostname to resolve
    Hostname(String),
}

impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ip(ip) => write!(f, "{}", ip),
            Self::Cidr(cidr) => write!(f, "{}", cidr),
            Self::Hostname(host) => write!(f, "{}", host),
        }
    }
}

/// Scan configuration
#[derive(Debug, Clone, Default)]
pub struct ScanConfig {
    /// Type of scan to perform
    pub scan_type: ScanType,
    /// Targets to scan
    pub targets: Vec<Target>,
    /// Ports to scan
    pub ports: Vec<u16>,
    /// Enable service detection
    pub service_detection: bool,
    /// Enable OS detection
    pub os_detection: bool,
    /// Timing template
    pub timing: TimingTemplate,
    /// Rate limit (packets per second, 0 = unlimited)
    pub rate_limit: Option<u64>,
    /// Resolve DNS names
    pub resolve_dns: bool,
    /// Network interface to use
    pub interface: Option<String>,
    /// Custom Lua script to run
    pub script: Option<PathBuf>,
    /// Scan timeout
    pub timeout: Option<Duration>,
}

/// Result of scanning a single host
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    /// Original host specification
    pub host: String,
    /// Resolved or provided IP address
    pub ip: Target,
    /// Resolved hostname (if DNS resolution enabled)
    pub hostname: Option<String>,
    /// Port scan results
    pub ports: Vec<PortResult>,
    /// OS detection result
    pub os: Option<OsInfo>,
    /// Time taken for this host
    pub scan_time: Duration,
}

impl Serialize for Target {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Target {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        // Try parsing as IP first
        if let Ok(ip) = s.parse::<IpAddr>() {
            return Ok(Target::Ip(ip));
        }

        // Try parsing as CIDR
        if let Ok(cidr) = s.parse::<ipnetwork::IpNetwork>() {
            return Ok(Target::Cidr(cidr));
        }

        // Treat as hostname
        Ok(Target::Hostname(s))
    }
}

/// Result for a single port
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortResult {
    /// Port number
    pub port: u16,
    /// Protocol
    pub protocol: Protocol,
    /// Port state
    pub state: PortState,
    /// Detected service name
    pub service: Option<String>,
    /// Detected service version
    pub version: Option<String>,
    /// Service banner
    pub banner: Option<String>,
}

/// OS detection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsInfo {
    /// OS name
    pub name: String,
    /// OS version
    pub version: Option<String>,
    /// Confidence percentage (0-100)
    pub confidence: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_display() {
        let ip_target = Target::Ip("192.168.1.1".parse().unwrap());
        assert_eq!(ip_target.to_string(), "192.168.1.1");

        let hostname_target = Target::Hostname("example.com".to_string());
        assert_eq!(hostname_target.to_string(), "example.com");
    }

    #[test]
    fn test_protocol_display() {
        assert_eq!(Protocol::Tcp.to_string(), "tcp");
        assert_eq!(Protocol::Udp.to_string(), "udp");
    }

    #[test]
    fn test_scan_config_default() {
        let config = ScanConfig::default();
        assert!(matches!(config.scan_type, ScanType::Connect));
        assert!(matches!(config.timing, TimingTemplate::Normal));
    }

    #[test]
    fn test_target_serialization() {
        let target = Target::Ip("192.168.1.1".parse().unwrap());
        let json = serde_json::to_string(&target).unwrap();
        assert_eq!(json, "\"192.168.1.1\"");

        let deserialized: Target = serde_json::from_str(&json).unwrap();
        assert_eq!(target, deserialized);
    }
}
