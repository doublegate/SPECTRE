use std::sync::Arc;

use serde::{Deserialize, Serialize};
use spectre_core::config::Config;
use tauri::{command, State};

use crate::state::AppState;

/// Full configuration for the frontend (mirrors spectre-core::config::Config).
#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub general: GeneralConfig,
    pub scan: ScanConfig,
    pub chef: ChefConfig,
    pub comms: CommsConfig,
    pub output: OutputConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub verbosity: u8,
    pub color: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanConfig {
    pub default_ports: String,
    pub default_timing: u8,
    pub service_detection: bool,
    pub os_detection: bool,
    pub resolve_dns: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChefConfig {
    pub docker_image: String,
    pub container_name: String,
    pub auto_start: bool,
    pub timeout: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommsConfig {
    pub default_expiration: u64,
    pub compress: bool,
    pub timeout: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputConfig {
    pub format: String,
    pub timestamps: bool,
    pub pretty_json: bool,
}

impl From<&Config> for AppConfig {
    fn from(config: &Config) -> Self {
        Self {
            general: GeneralConfig {
                verbosity: config.general.verbosity,
                color: config.general.color,
            },
            scan: ScanConfig {
                default_ports: config.scan.default_ports.clone(),
                default_timing: config.scan.default_timing,
                service_detection: config.scan.service_detection,
                os_detection: config.scan.os_detection,
                resolve_dns: config.scan.resolve_dns,
            },
            chef: ChefConfig {
                docker_image: config.chef.docker_image.clone(),
                container_name: config.chef.container_name.clone(),
                auto_start: config.chef.auto_start,
                timeout: config.chef.timeout,
            },
            comms: CommsConfig {
                default_expiration: config.comms.default_expiration,
                compress: config.comms.compress,
                timeout: config.comms.timeout,
            },
            output: OutputConfig {
                format: config.output.format.clone(),
                timestamps: config.output.timestamps,
                pretty_json: config.output.pretty_json,
            },
        }
    }
}

/// Partial configuration update request
#[derive(Debug, Deserialize)]
pub struct ConfigUpdate {
    pub general: Option<GeneralConfig>,
    pub scan: Option<ScanConfig>,
    pub chef: Option<ChefConfig>,
    pub comms: Option<CommsConfig>,
    pub output: Option<OutputConfig>,
}

/// Get current configuration.
#[command]
pub async fn get_config(state: State<'_, Arc<AppState>>) -> Result<AppConfig, String> {
    let config = state.config.read().await;
    Ok(AppConfig::from(&*config))
}

/// Update configuration with partial updates.
#[command]
pub async fn update_config(
    state: State<'_, Arc<AppState>>,
    updates: ConfigUpdate,
) -> Result<AppConfig, String> {
    let mut config = state.config.write().await;

    // Apply updates to each section
    if let Some(general) = updates.general {
        config.general.verbosity = general.verbosity;
        config.general.color = general.color;
    }

    if let Some(scan) = updates.scan {
        config.scan.default_ports = scan.default_ports;
        config.scan.default_timing = scan.default_timing;
        config.scan.service_detection = scan.service_detection;
        config.scan.os_detection = scan.os_detection;
        config.scan.resolve_dns = scan.resolve_dns;
    }

    if let Some(chef) = updates.chef {
        config.chef.docker_image = chef.docker_image;
        config.chef.container_name = chef.container_name;
        config.chef.auto_start = chef.auto_start;
        config.chef.timeout = chef.timeout;
    }

    if let Some(comms) = updates.comms {
        config.comms.default_expiration = comms.default_expiration;
        config.comms.compress = comms.compress;
        config.comms.timeout = comms.timeout;
    }

    if let Some(output) = updates.output {
        config.output.format = output.format;
        config.output.timestamps = output.timestamps;
        config.output.pretty_json = output.pretty_json;
    }

    Ok(AppConfig::from(&*config))
}

/// Reset configuration to defaults.
#[command]
pub async fn reset_config(state: State<'_, Arc<AppState>>) -> Result<AppConfig, String> {
    let mut config = state.config.write().await;
    *config = Config::default();
    Ok(AppConfig::from(&*config))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_config_conversion() {
        let core_config = Config::default();
        let app_config = AppConfig::from(&core_config);

        assert_eq!(app_config.general.verbosity, 0);
        assert_eq!(app_config.scan.default_timing, 3);
        assert!(!app_config.chef.docker_image.is_empty());
        assert_eq!(app_config.output.format, "table");
    }

    #[tokio::test]
    async fn test_config_update_scan() {
        let state = AppState::new(Config::default());

        // Verify initial state
        {
            let config = state.config.read().await;
            assert_eq!(config.scan.default_timing, 3);
        }

        // Update scan config directly (simulating what update_config does)
        {
            let mut config = state.config.write().await;
            config.scan.default_timing = 5;
            config.scan.default_ports = "1-65535".to_string();
        }

        // Verify update
        {
            let config = state.config.read().await;
            assert_eq!(config.scan.default_timing, 5);
            assert_eq!(config.scan.default_ports, "1-65535");
        }
    }

    #[tokio::test]
    async fn test_config_update_chef() {
        let state = AppState::new(Config::default());

        // Update chef config
        {
            let mut config = state.config.write().await;
            config.chef.docker_image = "custom-image:latest".to_string();
            config.chef.auto_start = false;
        }

        // Verify update
        {
            let config = state.config.read().await;
            assert_eq!(config.chef.docker_image, "custom-image:latest");
            assert!(!config.chef.auto_start);
        }
    }

    #[tokio::test]
    async fn test_config_reset() {
        let state = AppState::new(Config::default());

        // Modify config
        {
            let mut config = state.config.write().await;
            config.scan.default_timing = 5;
            config.general.verbosity = 3;
        }

        // Reset it
        {
            let mut config = state.config.write().await;
            *config = Config::default();
        }

        // Verify reset
        {
            let config = state.config.read().await;
            assert_eq!(config.scan.default_timing, 3);
            assert_eq!(config.general.verbosity, 0);
        }
    }

    #[tokio::test]
    async fn test_app_config_all_fields() {
        let mut core_config = Config::default();
        core_config.general.verbosity = 2;
        core_config.scan.service_detection = true;
        core_config.comms.compress = true;

        let app_config = AppConfig::from(&core_config);

        assert_eq!(app_config.general.verbosity, 2);
        assert!(app_config.scan.service_detection);
        assert!(app_config.comms.compress);
    }
}
