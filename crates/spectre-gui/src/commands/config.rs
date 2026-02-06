use serde::Serialize;
use tauri::command;

/// Configuration summary for the frontend.
#[derive(Debug, Serialize)]
pub struct ConfigSummary {
    pub scan_timing: u8,
    pub scan_default_ports: String,
    pub chef_docker_image: String,
    pub comms_identity_dir: String,
    pub output_format: String,
    pub verbose: u8,
}

/// Get current configuration. Stub - will be wired in Sprint 5.6.
#[command]
pub async fn get_config() -> Result<ConfigSummary, String> {
    Ok(ConfigSummary {
        scan_timing: 3,
        scan_default_ports: "1-1024".to_string(),
        chef_docker_image: "doublegate/cyberchef-mcp:latest".to_string(),
        comms_identity_dir: "~/.spectre/identity".to_string(),
        output_format: "table".to_string(),
        verbose: 0,
    })
}

/// Update a configuration value. Stub.
#[command]
pub async fn set_config(key: String, value: String) -> Result<String, String> {
    tracing::info!(key = %key, value = %value, "Set config (stub)");
    Ok(format!("Set {} = {}", key, value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_config_stub() {
        let result = get_config().await;
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.scan_timing, 3);
        assert!(!config.chef_docker_image.is_empty());
    }

    #[tokio::test]
    async fn test_set_config_stub() {
        let result = set_config("scan.timing".to_string(), "4".to_string()).await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("scan.timing"));
    }
}
