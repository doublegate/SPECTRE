//! Lua 5.4 plugin system module
//!
//! Provides a sandboxed plugin environment using mlua:
//! - Plugin discovery from configured directories
//! - Plugin manifest parsing (plugin.toml)
//! - Permission system (network, filesystem, etc.)
//! - Resource limits (memory, CPU time)
//! - `spectre.*` Lua API for plugin interaction

mod api;
pub mod manifest;
mod sandbox;

use std::path::{Path, PathBuf};

use tracing::{debug, info, instrument, warn};

pub use manifest::{PluginManifest, PluginPermission};
pub use sandbox::PluginSandbox;

/// A discovered plugin
#[derive(Debug, Clone)]
pub struct Plugin {
    /// Plugin manifest
    pub manifest: PluginManifest,
    /// Path to the plugin directory
    pub path: PathBuf,
    /// Main script path
    pub script_path: PathBuf,
}

impl Plugin {
    /// Load a plugin from a directory
    #[instrument(skip_all, fields(path = %path.display()))]
    pub fn load(path: &Path) -> crate::Result<Self> {
        info!("Loading plugin");

        let manifest_path = path.join("plugin.toml");
        if !manifest_path.exists() {
            return Err(crate::SpectreError::Plugin(
                crate::error::PluginError::InvalidManifest(format!(
                    "No plugin.toml found in {}",
                    path.display()
                )),
            ));
        }

        let manifest = PluginManifest::load(&manifest_path)?;

        let script_path = path.join(&manifest.main);
        if !script_path.exists() {
            return Err(crate::SpectreError::Plugin(
                crate::error::PluginError::NotFound(format!(
                    "Main script not found: {}",
                    script_path.display()
                )),
            ));
        }

        debug!(
            name = %manifest.name,
            version = %manifest.version,
            "Plugin loaded"
        );

        Ok(Self {
            manifest,
            path: path.to_path_buf(),
            script_path,
        })
    }
}

/// Discover plugins in a directory
///
/// Scans the given directory for subdirectories containing a `plugin.toml` file.
pub fn discover_plugins(plugin_dir: &Path) -> crate::Result<Vec<Plugin>> {
    if !plugin_dir.exists() {
        return Ok(Vec::new());
    }

    let mut plugins = Vec::new();

    for entry in std::fs::read_dir(plugin_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            match Plugin::load(&path) {
                Ok(plugin) => {
                    info!(name = %plugin.manifest.name, "Discovered plugin");
                    plugins.push(plugin);
                },
                Err(e) => {
                    warn!(
                        path = %path.display(),
                        error = %e,
                        "Failed to load plugin, skipping"
                    );
                },
            }
        }
    }

    Ok(plugins)
}

/// Plugin execution result
#[derive(Debug, Clone)]
pub struct PluginResult {
    /// Whether the plugin executed successfully
    pub success: bool,
    /// Output from the plugin
    pub output: String,
    /// Error message (if any)
    pub error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discover_plugins_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let plugins = discover_plugins(dir.path()).unwrap();
        assert!(plugins.is_empty());
    }

    #[test]
    fn test_discover_plugins_nonexistent() {
        let plugins = discover_plugins(Path::new("/nonexistent/path")).unwrap();
        assert!(plugins.is_empty());
    }

    #[test]
    fn test_load_plugin_no_manifest() {
        let dir = tempfile::tempdir().unwrap();
        let result = Plugin::load(dir.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_load_plugin_with_manifest() {
        let dir = tempfile::tempdir().unwrap();

        // Create plugin.toml
        let manifest = r#"
name = "test-plugin"
version = "1.0.0"
description = "A test plugin"
author = "Test Author"
main = "main.lua"

[permissions]
network = false
filesystem = false
"#;
        std::fs::write(dir.path().join("plugin.toml"), manifest).unwrap();
        std::fs::write(dir.path().join("main.lua"), "-- test plugin").unwrap();

        let plugin = Plugin::load(dir.path()).unwrap();
        assert_eq!(plugin.manifest.name, "test-plugin");
        assert_eq!(plugin.manifest.version, "1.0.0");
    }

    #[test]
    fn test_discover_plugins_with_valid() {
        let dir = tempfile::tempdir().unwrap();

        // Create a plugin subdirectory
        let plugin_dir = dir.path().join("my-plugin");
        std::fs::create_dir(&plugin_dir).unwrap();

        let manifest = r#"
name = "my-plugin"
version = "0.1.0"
description = "My test plugin"
author = "Test"
main = "init.lua"
"#;
        std::fs::write(plugin_dir.join("plugin.toml"), manifest).unwrap();
        std::fs::write(plugin_dir.join("init.lua"), "-- init").unwrap();

        let plugins = discover_plugins(dir.path()).unwrap();
        assert_eq!(plugins.len(), 1);
        assert_eq!(plugins[0].manifest.name, "my-plugin");
    }
}
