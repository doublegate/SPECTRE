use serde::Serialize;
use tauri::command;

/// Version information returned by get_version.
#[derive(Debug, Serialize)]
pub struct VersionInfo {
    pub version: String,
    pub name: String,
    pub description: String,
}

/// Component health status.
#[derive(Debug, Serialize)]
pub struct ComponentStatus {
    pub name: String,
    pub status: String,
    pub version: Option<String>,
    pub details: Option<String>,
}

/// Overall system status.
#[derive(Debug, Serialize)]
pub struct SystemStatus {
    pub components: Vec<ComponentStatus>,
    pub config_loaded: bool,
}

/// Get the current SPECTRE version.
#[command]
pub fn get_version() -> VersionInfo {
    VersionInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        name: "SPECTRE".to_string(),
        description: "Security Platform for Encrypted Comms, Testing, Enumeration, Recon"
            .to_string(),
    }
}

/// Get the status of all components.
#[command]
pub fn get_status() -> SystemStatus {
    let components = vec![
        ComponentStatus {
            name: "ProRT-IP".to_string(),
            status: "available".to_string(),
            version: Some("1.0.0".to_string()),
            details: Some("Network reconnaissance engine".to_string()),
        },
        ComponentStatus {
            name: "CyberChef-MCP".to_string(),
            status: "available".to_string(),
            version: Some("1.9.0".to_string()),
            details: Some("Data analysis via MCP protocol".to_string()),
        },
        ComponentStatus {
            name: "WRAITH-Protocol".to_string(),
            status: "available".to_string(),
            version: Some("2.3.7".to_string()),
            details: Some("Secure communications".to_string()),
        },
    ];

    SystemStatus {
        components,
        config_loaded: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_version() {
        let info = get_version();
        assert_eq!(info.name, "SPECTRE");
        assert!(!info.version.is_empty());
        assert!(info.description.contains("Security Platform"));
    }

    #[test]
    fn test_get_status() {
        let status = get_status();
        assert_eq!(status.components.len(), 3);
        assert!(status.config_loaded);
        assert_eq!(status.components[0].name, "ProRT-IP");
        assert_eq!(status.components[1].name, "CyberChef-MCP");
        assert_eq!(status.components[2].name, "WRAITH-Protocol");
    }

    #[test]
    fn test_version_info_serializable() {
        let info = get_version();
        let json = serde_json::to_string(&info).expect("Test assertion failed");
        assert!(json.contains("SPECTRE"));
    }

    #[test]
    fn test_system_status_serializable() {
        let status = get_status();
        let json = serde_json::to_string(&status).expect("Test assertion failed");
        assert!(json.contains("ProRT-IP"));
        assert!(json.contains("CyberChef-MCP"));
        assert!(json.contains("WRAITH-Protocol"));
    }
}
