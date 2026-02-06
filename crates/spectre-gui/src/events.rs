use serde::{Deserialize, Serialize};
use spectre_core::scan::{PortResult, ScanResult};

/// Scan progress event payload, emitted via `app_handle.emit("scan:progress", ...)`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgressEvent {
    /// Unique scan identifier
    pub scan_id: String,
    /// Number of hosts scanned so far
    pub completed: usize,
    /// Total number of hosts to scan
    pub total: usize,
    /// Progress as a percentage (0.0 - 100.0)
    pub percent: f64,
    /// Current target being scanned
    pub current_target: Option<String>,
    /// Scan rate (packets per second)
    pub rate_pps: Option<u64>,
}

/// Scan result event payload, emitted via `app_handle.emit("scan:result", ...)`.
/// Contains full host information with all discovered ports.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResultEvent {
    /// Unique scan identifier
    pub scan_id: String,
    /// Host address
    pub host: String,
    /// Resolved hostname
    pub hostname: Option<String>,
    /// All port results for this host
    pub ports: Vec<PortInfo>,
    /// OS detection result
    pub os: Option<OsInfo>,
}

/// Port information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortInfo {
    /// Port number
    pub port: u16,
    /// Protocol (tcp/udp)
    pub protocol: String,
    /// Port state (open, closed, filtered)
    pub state: String,
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
    /// Detection confidence (0-100)
    pub confidence: u8,
}

/// Scan completion event payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanCompleteEvent {
    /// Unique scan identifier
    pub scan_id: String,
    /// Total hosts scanned
    pub hosts_scanned: usize,
    /// Total open ports found
    pub open_ports: usize,
    /// Services discovered
    pub services_found: usize,
    /// Scan duration in milliseconds
    pub duration_ms: u64,
}

/// Scan error event payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanErrorEvent {
    /// Unique scan identifier
    pub scan_id: String,
    /// Error message
    pub error: String,
    /// Target that failed (if applicable)
    pub target: Option<String>,
}

/// Status event payload for general status updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEvent {
    /// Status message
    pub message: String,
    /// Event severity (info, warn, error)
    pub level: String,
}

// Conversion from spectre-core types
impl From<PortResult> for PortInfo {
    fn from(pr: PortResult) -> Self {
        Self {
            port: pr.port,
            protocol: pr.protocol.to_string(),
            state: format!("{:?}", pr.state).to_lowercase(),
            service: pr.service,
            version: pr.version,
            banner: pr.banner,
        }
    }
}

impl From<spectre_core::scan::OsInfo> for OsInfo {
    fn from(os: spectre_core::scan::OsInfo) -> Self {
        Self {
            name: os.name,
            version: os.version,
            confidence: os.confidence,
        }
    }
}

impl ScanResultEvent {
    /// Create from spectre-core ScanResult
    pub fn from_scan_result(scan_id: String, result: ScanResult) -> Self {
        Self {
            scan_id,
            host: result.host,
            hostname: result.hostname,
            ports: result.ports.into_iter().map(PortInfo::from).collect(),
            os: result.os.map(OsInfo::from),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_progress_event_serialize() {
        let event = ScanProgressEvent {
            scan_id: "scan_123".to_string(),
            completed: 50,
            total: 100,
            percent: 50.0,
            current_target: Some("10.0.0.1".to_string()),
            rate_pps: Some(1000),
        };
        let json = serde_json::to_string(&event).expect("Failed to serialize ScanProgressEvent");
        assert!(json.contains("\"scan_id\":\"scan_123\""));
        assert!(json.contains("\"completed\":50"));
        assert!(json.contains("\"total\":100"));
    }

    #[test]
    fn test_scan_result_event_serialize() {
        let event = ScanResultEvent {
            scan_id: "scan_123".to_string(),
            host: "10.0.0.1".to_string(),
            hostname: Some("example.com".to_string()),
            ports: vec![PortInfo {
                port: 80,
                protocol: "tcp".to_string(),
                state: "open".to_string(),
                service: Some("http".to_string()),
                version: Some("nginx 1.24".to_string()),
                banner: None,
            }],
            os: None,
        };
        let json = serde_json::to_string(&event).expect("Failed to serialize ScanResultEvent");
        assert!(json.contains("\"scan_id\":\"scan_123\""));
        assert!(json.contains("\"host\":\"10.0.0.1\""));
        assert!(json.contains("\"port\":80"));
    }

    #[test]
    fn test_scan_complete_event_serialize() {
        let event = ScanCompleteEvent {
            scan_id: "scan_123".to_string(),
            hosts_scanned: 256,
            open_ports: 42,
            services_found: 15,
            duration_ms: 15000,
        };
        let json = serde_json::to_string(&event).expect("Failed to serialize ScanCompleteEvent");
        assert!(json.contains("\"scan_id\":\"scan_123\""));
        assert!(json.contains("\"hosts_scanned\":256"));
        assert!(json.contains("\"services_found\":15"));
    }

    #[test]
    fn test_scan_error_event_serialize() {
        let event = ScanErrorEvent {
            scan_id: "scan_123".to_string(),
            error: "Connection refused".to_string(),
            target: Some("10.0.0.1".to_string()),
        };
        let json = serde_json::to_string(&event).expect("Failed to serialize ScanErrorEvent");
        assert!(json.contains("\"scan_id\":\"scan_123\""));
        assert!(json.contains("\"error\":\"Connection refused\""));
    }

    #[test]
    fn test_status_event_serialize() {
        let event = StatusEvent {
            message: "Scan started".to_string(),
            level: "info".to_string(),
        };
        let json = serde_json::to_string(&event).expect("Failed to serialize StatusEvent");
        assert!(json.contains("\"level\":\"info\""));
    }

    #[test]
    fn test_scan_progress_event_deserialize() {
        let json = r#"{"scan_id":"scan_123","completed":25,"total":50,"percent":50.0,"current_target":null,"rate_pps":null}"#;
        let event: ScanProgressEvent =
            serde_json::from_str(json).expect("Failed to deserialize ScanProgressEvent");
        assert_eq!(event.scan_id, "scan_123");
        assert_eq!(event.completed, 25);
        assert_eq!(event.total, 50);
    }

    #[test]
    fn test_port_info_from_port_result() {
        use spectre_core::scan::{PortResult, PortState, Protocol};
        let port_result = PortResult {
            port: 80,
            protocol: Protocol::Tcp,
            state: PortState::Open,
            service: Some("http".to_string()),
            version: Some("nginx".to_string()),
            banner: Some("200 OK".to_string()),
        };
        let port_info = PortInfo::from(port_result);
        assert_eq!(port_info.port, 80);
        assert_eq!(port_info.protocol, "tcp");
        assert_eq!(port_info.state, "open");
        assert_eq!(port_info.service, Some("http".to_string()));
    }

    #[test]
    fn test_os_info_from_core() {
        let os = spectre_core::scan::OsInfo {
            name: "Linux".to_string(),
            version: Some("5.15".to_string()),
            confidence: 90,
        };
        let os_info = OsInfo::from(os);
        assert_eq!(os_info.name, "Linux");
        assert_eq!(os_info.version, Some("5.15".to_string()));
        assert_eq!(os_info.confidence, 90);
    }
}
