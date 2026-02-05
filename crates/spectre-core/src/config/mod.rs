//! Configuration management module
//!
//! This module handles loading, merging, and validating configuration from multiple sources.
//!
//! # Configuration Sources (in order of precedence)
//!
//! 1. CLI arguments (highest priority)
//! 2. Environment variables (`SPECTRE_*`)
//! 3. Project configuration (`./spectre.toml`)
//! 4. User configuration (`~/.config/spectre/spectre.toml`)
//! 5. System configuration (`/etc/spectre/spectre.toml`)

mod loader;

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub use loader::{
    generate_default_config, generate_minimal_config, load_config, set_value, system_config_path,
    user_config_path, validate_config, ValidationResult,
};

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    /// General settings
    pub general: GeneralConfig,

    /// Scan settings (ProRT-IP)
    pub scan: ScanConfig,

    /// Chef settings (CyberChef-MCP)
    pub chef: ChefConfig,

    /// Communications settings (WRAITH)
    pub comms: CommsConfig,

    /// Output settings
    pub output: OutputConfig,

    /// Path this config was loaded from
    #[serde(skip)]
    loaded_from: Option<PathBuf>,

    /// Sources that contributed to this config
    #[serde(skip)]
    sources: Vec<PathBuf>,
}

/// General configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GeneralConfig {
    /// Default verbosity level
    pub verbosity: u8,

    /// Enable color output
    pub color: bool,

    /// Data directory for storing state
    pub data_dir: Option<PathBuf>,

    /// Temp directory for temporary files
    pub temp_dir: Option<PathBuf>,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            verbosity: 0,
            color: true,
            data_dir: None,
            temp_dir: None,
        }
    }
}

/// Scan configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ScanConfig {
    /// Default ports to scan
    pub default_ports: String,

    /// Default timing template (0-5)
    pub default_timing: u8,

    /// Maximum packets per second (0 = unlimited)
    pub max_rate: u64,

    /// Network interface to use
    pub interface: Option<String>,

    /// Enable service detection by default
    pub service_detection: bool,

    /// Enable OS detection by default
    pub os_detection: bool,

    /// Resolve DNS names
    pub resolve_dns: bool,

    /// Script directory for custom Lua scripts
    pub script_dir: Option<PathBuf>,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            default_ports: "common".to_string(),
            default_timing: 3,
            max_rate: 0,
            interface: None,
            service_detection: false,
            os_detection: false,
            resolve_dns: true,
            script_dir: None,
        }
    }
}

impl ScanConfig {
    /// Get the network interface setting
    pub fn interface(&self) -> Option<&str> {
        self.interface.as_deref()
    }
}

/// Chef configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ChefConfig {
    /// MCP server endpoint
    pub mcp_endpoint: String,

    /// Docker image to use
    pub docker_image: String,

    /// Container name
    pub container_name: String,

    /// Auto-start container if not running
    pub auto_start: bool,

    /// Recipe directory
    pub recipe_dir: Option<PathBuf>,

    /// Connection timeout in seconds
    pub timeout: u64,

    /// Enable worker thread pool for CPU-intensive operations (v1.9.0)
    pub enable_workers: bool,

    /// Maximum worker threads (v1.9.0, default: 4)
    pub worker_threads: u32,

    /// Enable streaming for large operations (v1.9.0)
    pub enable_streaming: bool,
}

impl Default for ChefConfig {
    fn default() -> Self {
        Self {
            mcp_endpoint: "http://localhost:3001".to_string(),
            docker_image: "doublegate/cyberchef-mcp:latest".to_string(),
            container_name: "spectre-cyberchef".to_string(),
            auto_start: true,
            recipe_dir: None,
            timeout: 30,
            enable_workers: false,
            worker_threads: 4,
            enable_streaming: true,
        }
    }
}

impl ChefConfig {
    /// Get the MCP endpoint
    pub fn mcp_endpoint(&self) -> &str {
        &self.mcp_endpoint
    }
}

/// Communications configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CommsConfig {
    /// Default relay server URL
    pub relay_url: Option<String>,

    /// Identity storage directory
    pub identity_dir: Option<PathBuf>,

    /// Peer storage directory
    pub peer_dir: Option<PathBuf>,

    /// Default message expiration in seconds (0 = no expiration)
    pub default_expiration: u64,

    /// Enable compression by default
    pub compress: bool,

    /// Connection timeout in seconds
    pub timeout: u64,
}

impl Default for CommsConfig {
    fn default() -> Self {
        Self {
            relay_url: None,
            identity_dir: None,
            peer_dir: None,
            default_expiration: 0,
            compress: false,
            timeout: 30,
        }
    }
}

impl CommsConfig {
    /// Get the relay URL setting
    pub fn relay_url(&self) -> Option<&str> {
        self.relay_url.as_deref()
    }
}

/// Output configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct OutputConfig {
    /// Default output format
    pub format: String,

    /// Default output directory
    pub output_dir: Option<PathBuf>,

    /// Include timestamps in output
    pub timestamps: bool,

    /// Pretty print JSON output
    pub pretty_json: bool,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            format: "table".to_string(),
            output_dir: None,
            timestamps: true,
            pretty_json: true,
        }
    }
}

impl Config {
    /// Get the path this config was loaded from
    pub fn loaded_from(&self) -> Option<&Path> {
        self.loaded_from.as_deref()
    }

    /// Get the sources that contributed to this config
    pub fn sources(&self) -> &[PathBuf] {
        &self.sources
    }

    /// Set the loaded-from path
    pub fn set_loaded_from(&mut self, path: PathBuf) {
        self.loaded_from = Some(path);
    }

    /// Add a source path
    pub fn add_source(&mut self, path: PathBuf) {
        self.sources.push(path);
    }

    /// Convert config to TOML string
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }

    /// Convert config to JSON value
    pub fn to_json(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }

    /// Get a section as TOML string
    pub fn section_as_toml(&self, section: &str) -> crate::Result<String> {
        let json = self.to_json()?;
        let section_value =
            json.get(section)
                .ok_or_else(|| crate::error::ConfigError::InvalidValue {
                    key: section.to_string(),
                    message: "Section not found".to_string(),
                })?;

        // Convert section back through serde
        let toml_value: toml::Value = serde_json::from_value(section_value.clone())?;
        Ok(toml::to_string_pretty(&toml_value)?)
    }

    /// Get a section as JSON value
    pub fn section_as_json(&self, section: &str) -> crate::Result<serde_json::Value> {
        let json = self.to_json()?;
        json.get(section).cloned().ok_or_else(|| {
            crate::SpectreError::Config(crate::error::ConfigError::InvalidValue {
                key: section.to_string(),
                message: "Section not found".to_string(),
            })
        })
    }

    /// Get a value by dot-notation key
    pub fn get(&self, key: &str) -> crate::Result<String> {
        let json = self.to_json()?;
        let mut current = &json;

        for part in key.split('.') {
            current = current.get(part).ok_or_else(|| {
                crate::SpectreError::Config(crate::error::ConfigError::InvalidValue {
                    key: key.to_string(),
                    message: format!("Key '{}' not found", part),
                })
            })?;
        }

        // Convert value to string representation
        Ok(match current {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Bool(b) => b.to_string(),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Null => "null".to_string(),
            _ => serde_json::to_string(current)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.general.color);
        assert_eq!(config.scan.default_timing, 3);
        assert_eq!(config.chef.mcp_endpoint, "http://localhost:3001");
    }

    #[test]
    fn test_config_to_toml() {
        let config = Config::default();
        let toml = config.to_toml().unwrap();
        assert!(toml.contains("[general]"));
        assert!(toml.contains("[scan]"));
    }

    #[test]
    fn test_config_to_json() {
        let config = Config::default();
        let json = config.to_json().unwrap();
        assert!(json.get("general").is_some());
        assert!(json.get("scan").is_some());
    }

    #[test]
    fn test_config_get_value() {
        let config = Config::default();
        let value = config.get("scan.default_timing").unwrap();
        assert_eq!(value, "3");
    }

    #[test]
    fn test_config_get_invalid_key() {
        let config = Config::default();
        let result = config.get("invalid.key.path");
        assert!(result.is_err());
    }
}
