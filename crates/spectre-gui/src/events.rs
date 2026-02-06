use serde::{Deserialize, Serialize};

/// Scan progress event payload, emitted via `app_handle.emit("scan:progress", ...)`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgressEvent {
    /// Number of hosts scanned so far
    pub completed: usize,
    /// Total number of hosts to scan
    pub total: usize,
    /// Progress as a percentage (0.0 - 100.0)
    pub percent: f64,
}

/// Scan result event payload, emitted via `app_handle.emit("scan:result", ...)`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResultEvent {
    /// Host address
    pub host: String,
    /// Port number
    pub port: u16,
    /// Port state (open, closed, filtered)
    pub state: String,
    /// Protocol (tcp/udp)
    pub protocol: String,
    /// Detected service name
    pub service: Option<String>,
    /// Detected service version
    pub version: Option<String>,
}

/// Scan completion event payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanCompleteEvent {
    /// Total hosts scanned
    pub hosts_scanned: usize,
    /// Total open ports found
    pub open_ports: usize,
    /// Scan duration in milliseconds
    pub duration_ms: u64,
}

/// Scan error event payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanErrorEvent {
    /// Error message
    pub error: String,
}

/// Status event payload for general status updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEvent {
    /// Status message
    pub message: String,
    /// Event severity (info, warn, error)
    pub level: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_progress_event_serialize() {
        let event = ScanProgressEvent {
            completed: 50,
            total: 100,
            percent: 50.0,
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"completed\":50"));
        assert!(json.contains("\"total\":100"));
    }

    #[test]
    fn test_scan_result_event_serialize() {
        let event = ScanResultEvent {
            host: "10.0.0.1".to_string(),
            port: 80,
            state: "open".to_string(),
            protocol: "tcp".to_string(),
            service: Some("http".to_string()),
            version: None,
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"host\":\"10.0.0.1\""));
        assert!(json.contains("\"port\":80"));
    }

    #[test]
    fn test_scan_complete_event_serialize() {
        let event = ScanCompleteEvent {
            hosts_scanned: 256,
            open_ports: 42,
            duration_ms: 15000,
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"hosts_scanned\":256"));
    }

    #[test]
    fn test_status_event_serialize() {
        let event = StatusEvent {
            message: "Scan started".to_string(),
            level: "info".to_string(),
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"level\":\"info\""));
    }

    #[test]
    fn test_scan_progress_event_deserialize() {
        let json = r#"{"completed":25,"total":50,"percent":50.0}"#;
        let event: ScanProgressEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event.completed, 25);
        assert_eq!(event.total, 50);
    }
}
