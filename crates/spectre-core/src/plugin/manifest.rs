//! Plugin manifest parsing (plugin.toml)

use std::path::Path;

use serde::{Deserialize, Serialize};

/// Plugin permissions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PluginPermission {
    /// Network access
    Network,
    /// Filesystem read access
    FilesystemRead,
    /// Filesystem write access
    FilesystemWrite,
    /// Execute external commands
    Execute,
}

impl std::fmt::Display for PluginPermission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Network => write!(f, "network"),
            Self::FilesystemRead => write!(f, "filesystem:read"),
            Self::FilesystemWrite => write!(f, "filesystem:write"),
            Self::Execute => write!(f, "execute"),
        }
    }
}

/// Plugin manifest (plugin.toml)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    /// Plugin name
    pub name: String,
    /// Plugin version (semver)
    pub version: String,
    /// Plugin description
    pub description: String,
    /// Plugin author
    pub author: String,
    /// Main Lua script file
    pub main: String,
    /// Plugin permissions
    #[serde(default)]
    pub permissions: PermissionConfig,
    /// Resource limits
    #[serde(default)]
    pub limits: ResourceLimits,
    /// Plugin dependencies (other plugin names)
    #[serde(default)]
    pub dependencies: Vec<String>,
    /// Tags for categorization
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Permission configuration in manifest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PermissionConfig {
    /// Allow network access
    #[serde(default)]
    pub network: bool,
    /// Allow filesystem read
    #[serde(default)]
    pub filesystem: bool,
    /// Allow filesystem write
    #[serde(default)]
    pub filesystem_write: bool,
    /// Allow executing external commands
    #[serde(default)]
    pub execute: bool,
}

impl PermissionConfig {
    /// Get the list of granted permissions
    pub fn granted_permissions(&self) -> Vec<PluginPermission> {
        let mut perms = Vec::new();
        if self.network {
            perms.push(PluginPermission::Network);
        }
        if self.filesystem {
            perms.push(PluginPermission::FilesystemRead);
        }
        if self.filesystem_write {
            perms.push(PluginPermission::FilesystemWrite);
        }
        if self.execute {
            perms.push(PluginPermission::Execute);
        }
        perms
    }

    /// Check if a specific permission is granted
    pub fn has_permission(&self, perm: PluginPermission) -> bool {
        match perm {
            PluginPermission::Network => self.network,
            PluginPermission::FilesystemRead => self.filesystem,
            PluginPermission::FilesystemWrite => self.filesystem_write,
            PluginPermission::Execute => self.execute,
        }
    }
}

/// Resource limits for plugin execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum memory in bytes (default: 16 MB)
    #[serde(default = "default_memory_limit")]
    pub max_memory: usize,
    /// Maximum execution time in seconds (default: 30)
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
    /// Maximum number of instructions (default: 10_000_000)
    #[serde(default = "default_instruction_limit")]
    pub max_instructions: u64,
}

fn default_memory_limit() -> usize {
    16 * 1024 * 1024 // 16 MB
}

fn default_timeout() -> u64 {
    30
}

fn default_instruction_limit() -> u64 {
    10_000_000
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory: default_memory_limit(),
            timeout_seconds: default_timeout(),
            max_instructions: default_instruction_limit(),
        }
    }
}

impl PluginManifest {
    /// Load a plugin manifest from a TOML file
    pub fn load(path: &Path) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path).map_err(|e| {
            crate::SpectreError::Plugin(crate::error::PluginError::InvalidManifest(format!(
                "Failed to read manifest: {}",
                e
            )))
        })?;

        let manifest: PluginManifest = toml::from_str(&content).map_err(|e| {
            crate::SpectreError::Plugin(crate::error::PluginError::InvalidManifest(format!(
                "Failed to parse manifest: {}",
                e
            )))
        })?;

        manifest.validate()?;

        Ok(manifest)
    }

    /// Validate the manifest
    fn validate(&self) -> crate::Result<()> {
        if self.name.is_empty() {
            return Err(crate::SpectreError::Plugin(
                crate::error::PluginError::InvalidManifest("Plugin name is required".to_string()),
            ));
        }

        if self.version.is_empty() {
            return Err(crate::SpectreError::Plugin(
                crate::error::PluginError::InvalidManifest(
                    "Plugin version is required".to_string(),
                ),
            ));
        }

        if self.main.is_empty() {
            return Err(crate::SpectreError::Plugin(
                crate::error::PluginError::InvalidManifest(
                    "Plugin main script is required".to_string(),
                ),
            ));
        }

        // Validate main script ends in .lua
        if !self.main.ends_with(".lua") {
            return Err(crate::SpectreError::Plugin(
                crate::error::PluginError::InvalidManifest(
                    "Main script must be a .lua file".to_string(),
                ),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_load() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("plugin.toml");

        let content = r#"
name = "test-plugin"
version = "1.0.0"
description = "A test plugin"
author = "Test Author"
main = "main.lua"
tags = ["test", "example"]

[permissions]
network = true
filesystem = false

[limits]
max_memory = 8388608
timeout_seconds = 10
"#;
        std::fs::write(&path, content).unwrap();

        let manifest = PluginManifest::load(&path).unwrap();
        assert_eq!(manifest.name, "test-plugin");
        assert_eq!(manifest.version, "1.0.0");
        assert!(manifest.permissions.network);
        assert!(!manifest.permissions.filesystem);
        assert_eq!(manifest.limits.max_memory, 8_388_608);
        assert_eq!(manifest.limits.timeout_seconds, 10);
        assert_eq!(manifest.tags.len(), 2);
    }

    #[test]
    fn test_manifest_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("plugin.toml");

        let content = r#"
name = "minimal"
version = "0.1.0"
description = "Minimal plugin"
author = "Test"
main = "init.lua"
"#;
        std::fs::write(&path, content).unwrap();

        let manifest = PluginManifest::load(&path).unwrap();
        assert!(!manifest.permissions.network);
        assert!(!manifest.permissions.filesystem);
        assert_eq!(manifest.limits.max_memory, 16 * 1024 * 1024);
        assert_eq!(manifest.limits.timeout_seconds, 30);
    }

    #[test]
    fn test_manifest_validation_no_name() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("plugin.toml");

        let content = r#"
name = ""
version = "1.0.0"
description = "test"
author = "test"
main = "main.lua"
"#;
        std::fs::write(&path, content).unwrap();
        let result = PluginManifest::load(&path);
        assert!(result.is_err());
    }

    #[test]
    fn test_manifest_validation_bad_main() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("plugin.toml");

        let content = r#"
name = "test"
version = "1.0.0"
description = "test"
author = "test"
main = "main.py"
"#;
        std::fs::write(&path, content).unwrap();
        let result = PluginManifest::load(&path);
        assert!(result.is_err());
    }

    #[test]
    fn test_permission_config() {
        let config = PermissionConfig {
            network: true,
            filesystem: true,
            filesystem_write: false,
            execute: false,
        };

        let perms = config.granted_permissions();
        assert_eq!(perms.len(), 2);
        assert!(config.has_permission(PluginPermission::Network));
        assert!(config.has_permission(PluginPermission::FilesystemRead));
        assert!(!config.has_permission(PluginPermission::FilesystemWrite));
        assert!(!config.has_permission(PluginPermission::Execute));
    }

    #[test]
    fn test_permission_display() {
        assert_eq!(PluginPermission::Network.to_string(), "network");
        assert_eq!(
            PluginPermission::FilesystemRead.to_string(),
            "filesystem:read"
        );
        assert_eq!(
            PluginPermission::FilesystemWrite.to_string(),
            "filesystem:write"
        );
        assert_eq!(PluginPermission::Execute.to_string(), "execute");
    }

    #[test]
    fn test_resource_limits_default() {
        let limits = ResourceLimits::default();
        assert_eq!(limits.max_memory, 16 * 1024 * 1024);
        assert_eq!(limits.timeout_seconds, 30);
        assert_eq!(limits.max_instructions, 10_000_000);
    }
}
