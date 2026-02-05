//! Configuration loading and merging utilities

use std::path::{Path, PathBuf};

use directories::ProjectDirs;

use super::Config;
use crate::error::ConfigError;

/// Validation result from config check
#[derive(Debug, Default)]
pub struct ValidationResult {
    /// Errors found during validation
    pub errors: Vec<ValidationIssue>,
    /// Warnings found during validation
    pub warnings: Vec<ValidationIssue>,
}

/// A single validation issue
#[derive(Debug)]
pub struct ValidationIssue {
    /// Configuration key
    pub key: String,
    /// Issue message
    pub message: String,
}

/// Load configuration from all sources, merging with proper precedence
///
/// # Arguments
///
/// * `override_path` - Optional path to override config discovery
///
/// # Returns
///
/// The merged configuration, or an error if loading fails.
pub fn load_config(override_path: Option<&Path>) -> crate::Result<Config> {
    let mut config = Config::default();
    let mut sources = Vec::new();

    // Load system config
    let system_path = system_config_path();
    if system_path.exists() {
        if let Ok(system_config) = load_from_file(&system_path) {
            config = merge_config(config, system_config);
            sources.push(system_path);
        }
    }

    // Load user config
    if let Some(user_path) = user_config_path() {
        if user_path.exists() {
            if let Ok(user_config) = load_from_file(&user_path) {
                config = merge_config(config, user_config);
                sources.push(user_path);
            }
        }
    }

    // Load project config
    let project_path = if let Some(path) = override_path {
        path.to_path_buf()
    } else {
        PathBuf::from("./spectre.toml")
    };

    if project_path.exists() {
        let project_config = load_from_file(&project_path)?;
        config = merge_config(config, project_config);
        sources.push(project_path.clone());
        config.set_loaded_from(project_path);
    }

    // Apply environment variable overrides
    config = apply_env_overrides(config)?;

    // Store sources
    for source in sources {
        config.add_source(source);
    }

    Ok(config)
}

/// Load configuration from a specific file
fn load_from_file(path: &Path) -> crate::Result<Config> {
    let content = std::fs::read_to_string(path).map_err(|_e| {
        crate::SpectreError::Config(ConfigError::NotFound {
            path: path.display().to_string(),
        })
    })?;

    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

/// Merge two configurations, with the second taking precedence
fn merge_config(base: Config, overlay: Config) -> Config {
    // For now, simple overlay wins approach
    // In a more sophisticated implementation, we'd merge field by field
    Config {
        general: super::GeneralConfig {
            verbosity: if overlay.general.verbosity != 0 {
                overlay.general.verbosity
            } else {
                base.general.verbosity
            },
            color: overlay.general.color,
            data_dir: overlay.general.data_dir.or(base.general.data_dir),
            temp_dir: overlay.general.temp_dir.or(base.general.temp_dir),
        },
        scan: super::ScanConfig {
            default_ports: if overlay.scan.default_ports == "common" {
                base.scan.default_ports
            } else {
                overlay.scan.default_ports
            },
            default_timing: overlay.scan.default_timing,
            max_rate: if overlay.scan.max_rate != 0 {
                overlay.scan.max_rate
            } else {
                base.scan.max_rate
            },
            interface: overlay.scan.interface.or(base.scan.interface),
            service_detection: overlay.scan.service_detection || base.scan.service_detection,
            os_detection: overlay.scan.os_detection || base.scan.os_detection,
            resolve_dns: overlay.scan.resolve_dns,
            script_dir: overlay.scan.script_dir.or(base.scan.script_dir),
        },
        chef: super::ChefConfig {
            mcp_endpoint: if overlay.chef.mcp_endpoint == "http://localhost:3001" {
                base.chef.mcp_endpoint
            } else {
                overlay.chef.mcp_endpoint
            },
            docker_image: overlay.chef.docker_image,
            container_name: overlay.chef.container_name,
            auto_start: overlay.chef.auto_start,
            recipe_dir: overlay.chef.recipe_dir.or(base.chef.recipe_dir),
            timeout: overlay.chef.timeout,
        },
        comms: super::CommsConfig {
            relay_url: overlay.comms.relay_url.or(base.comms.relay_url),
            identity_dir: overlay.comms.identity_dir.or(base.comms.identity_dir),
            peer_dir: overlay.comms.peer_dir.or(base.comms.peer_dir),
            default_expiration: overlay.comms.default_expiration,
            compress: overlay.comms.compress || base.comms.compress,
            timeout: overlay.comms.timeout,
        },
        output: super::OutputConfig {
            format: if overlay.output.format == "table" {
                base.output.format
            } else {
                overlay.output.format
            },
            output_dir: overlay.output.output_dir.or(base.output.output_dir),
            timestamps: overlay.output.timestamps,
            pretty_json: overlay.output.pretty_json,
        },
        loaded_from: None,
        sources: Vec::new(),
    }
}

/// Apply environment variable overrides to config
fn apply_env_overrides(mut config: Config) -> crate::Result<Config> {
    // SPECTRE_VERBOSITY
    if let Ok(val) = std::env::var("SPECTRE_VERBOSITY") {
        config.general.verbosity = val.parse().map_err(|_| {
            crate::SpectreError::Config(ConfigError::EnvVar(
                "SPECTRE_VERBOSITY must be a number".to_string(),
            ))
        })?;
    }

    // SPECTRE_COLOR
    if let Ok(val) = std::env::var("SPECTRE_COLOR") {
        config.general.color = val.parse().unwrap_or(true);
    }

    // SPECTRE_DATA_DIR
    if let Ok(val) = std::env::var("SPECTRE_DATA_DIR") {
        config.general.data_dir = Some(PathBuf::from(val));
    }

    // SPECTRE_SCAN_INTERFACE
    if let Ok(val) = std::env::var("SPECTRE_SCAN_INTERFACE") {
        config.scan.interface = Some(val);
    }

    // SPECTRE_CHEF_ENDPOINT
    if let Ok(val) = std::env::var("SPECTRE_CHEF_ENDPOINT") {
        config.chef.mcp_endpoint = val;
    }

    // SPECTRE_RELAY_URL
    if let Ok(val) = std::env::var("SPECTRE_RELAY_URL") {
        config.comms.relay_url = Some(val);
    }

    Ok(config)
}

/// Get the system configuration path
pub fn system_config_path() -> PathBuf {
    PathBuf::from("/etc/spectre/spectre.toml")
}

/// Get the user configuration path
pub fn user_config_path() -> Option<PathBuf> {
    ProjectDirs::from("com", "spectre", "spectre")
        .map(|dirs| dirs.config_dir().join("spectre.toml"))
}

/// Validate a configuration file
pub fn validate_config(path: Option<&Path>) -> crate::Result<ValidationResult> {
    let config = load_config(path)?;
    let mut result = ValidationResult::default();

    // Validate scan settings
    if config.scan.default_timing > 5 {
        result.errors.push(ValidationIssue {
            key: "scan.default_timing".to_string(),
            message: "Timing must be between 0 and 5".to_string(),
        });
    }

    // Validate chef endpoint
    if !config.chef.mcp_endpoint.starts_with("http://")
        && !config.chef.mcp_endpoint.starts_with("https://")
    {
        result.errors.push(ValidationIssue {
            key: "chef.mcp_endpoint".to_string(),
            message: "Endpoint must start with http:// or https://".to_string(),
        });
    }

    // Warnings
    if config.comms.relay_url.is_none() {
        result.warnings.push(ValidationIssue {
            key: "comms.relay_url".to_string(),
            message: "No relay URL configured; direct peer connections only".to_string(),
        });
    }

    Ok(result)
}

/// Generate a default configuration file
pub fn generate_default_config() -> String {
    r#"# SPECTRE Configuration File
# https://github.com/doublegate/SPECTRE

[general]
# Verbosity level (0=warn, 1=info, 2=debug, 3=trace)
verbosity = 0
# Enable colored output
color = true
# Data directory (default: platform-specific)
# data_dir = "~/.local/share/spectre"

[scan]
# Default ports to scan ("common", "all", or port specification)
default_ports = "common"
# Default timing template (0=paranoid, 1=sneaky, 2=polite, 3=normal, 4=aggressive, 5=insane)
default_timing = 3
# Maximum packets per second (0 = unlimited)
max_rate = 0
# Network interface to use (default: auto-detect)
# interface = "eth0"
# Enable service detection by default
service_detection = false
# Enable OS detection by default
os_detection = false
# Resolve DNS names
resolve_dns = true
# Custom Lua script directory
# script_dir = "~/.config/spectre/scripts"

[chef]
# CyberChef-MCP endpoint
mcp_endpoint = "http://localhost:3001"
# Docker image for CyberChef-MCP
docker_image = "doublegate/cyberchef-mcp:latest"
# Container name
container_name = "spectre-cyberchef"
# Auto-start container if not running
auto_start = true
# Recipe directory
# recipe_dir = "~/.config/spectre/recipes"
# Connection timeout in seconds
timeout = 30

[comms]
# Default relay server URL
# relay_url = "wss://relay.example.com"
# Identity storage directory
# identity_dir = "~/.config/spectre/identities"
# Peer storage directory
# peer_dir = "~/.config/spectre/peers"
# Default message expiration in seconds (0 = no expiration)
default_expiration = 0
# Enable compression by default
compress = false
# Connection timeout in seconds
timeout = 30

[output]
# Default output format (table, json, csv, yaml)
format = "table"
# Default output directory
# output_dir = "~/spectre-output"
# Include timestamps in output
timestamps = true
# Pretty print JSON output
pretty_json = true
"#
    .to_string()
}

/// Generate a minimal configuration file
pub fn generate_minimal_config() -> String {
    r#"# SPECTRE Configuration File (Minimal)
# https://github.com/doublegate/SPECTRE

[general]
color = true

[scan]
default_ports = "common"
default_timing = 3

[chef]
mcp_endpoint = "http://localhost:3001"
auto_start = true

[comms]
# relay_url = "wss://relay.example.com"

[output]
format = "table"
"#
    .to_string()
}

/// Set a value in a TOML document using dot notation
pub fn set_value(doc: &mut toml_edit::DocumentMut, key: &str, value: &str) -> crate::Result<()> {
    let parts: Vec<&str> = key.split('.').collect();

    if parts.len() == 2 {
        let section = parts[0];
        let field = parts[1];

        // Get or create section
        if doc.get(section).is_none() {
            doc[section] = toml_edit::table();
        }

        // Set value (try to preserve type)
        if let Ok(num) = value.parse::<i64>() {
            doc[section][field] = toml_edit::value(num);
        } else if let Ok(b) = value.parse::<bool>() {
            doc[section][field] = toml_edit::value(b);
        } else {
            doc[section][field] = toml_edit::value(value);
        }

        Ok(())
    } else {
        Err(crate::SpectreError::Config(ConfigError::InvalidValue {
            key: key.to_string(),
            message: "Key must be in format 'section.field'".to_string(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_config_path() {
        let path = system_config_path();
        assert_eq!(path, PathBuf::from("/etc/spectre/spectre.toml"));
    }

    #[test]
    fn test_user_config_path() {
        let path = user_config_path();
        // Should return Some on most systems
        if let Some(p) = path {
            assert!(p.to_string_lossy().contains("spectre"));
        }
    }

    #[test]
    fn test_generate_default_config() {
        let config = generate_default_config();
        assert!(config.contains("[general]"));
        assert!(config.contains("[scan]"));
        assert!(config.contains("[chef]"));
        assert!(config.contains("[comms]"));
    }

    #[test]
    fn test_generate_minimal_config() {
        let config = generate_minimal_config();
        assert!(config.contains("[general]"));
        assert!(config.len() < generate_default_config().len());
    }

    #[test]
    fn test_set_value() {
        let mut doc = "".parse::<toml_edit::DocumentMut>().unwrap();
        set_value(&mut doc, "general.verbosity", "2").unwrap();
        assert!(doc.to_string().contains("verbosity = 2"));
    }

    #[test]
    fn test_set_value_invalid_key() {
        let mut doc = "".parse::<toml_edit::DocumentMut>().unwrap();
        let result = set_value(&mut doc, "invalid", "value");
        assert!(result.is_err());
    }
}
