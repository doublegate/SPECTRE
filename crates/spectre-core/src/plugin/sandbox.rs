//! Lua sandbox implementation for safe plugin execution

use mlua::{Lua, Value};
use tracing::{debug, info, instrument};

use super::manifest::{PermissionConfig, ResourceLimits};
use super::PluginResult;

/// Sandboxed Lua execution environment
///
/// Provides a restricted Lua runtime with:
/// - Removed dangerous modules (os, io, loadfile, etc.)
/// - Memory limits
/// - `spectre.*` API surface for plugin interaction
pub struct PluginSandbox {
    lua: Lua,
    permissions: PermissionConfig,
    limits: ResourceLimits,
}

impl PluginSandbox {
    /// Create a new sandboxed Lua environment
    #[instrument(skip_all)]
    pub fn new(permissions: PermissionConfig, limits: ResourceLimits) -> crate::Result<Self> {
        info!("Creating plugin sandbox");

        let lua = Lua::new();

        // Set memory limit
        lua.set_memory_limit(limits.max_memory)?;

        // Remove dangerous globals
        Self::restrict_globals(&lua)?;

        // Register spectre API
        super::api::register_spectre_api(&lua, &permissions)?;

        debug!("Plugin sandbox ready");

        Ok(Self {
            lua,
            permissions,
            limits,
        })
    }

    /// Remove dangerous globals from the Lua environment
    fn restrict_globals(lua: &Lua) -> crate::Result<()> {
        let globals = lua.globals();

        // Remove dangerous modules
        let to_remove = ["os", "io", "loadfile", "dofile", "require"];

        for name in &to_remove {
            globals.set(*name, Value::Nil)?;
        }

        Ok(())
    }

    /// Execute a Lua script in the sandbox
    #[instrument(skip(self, script), fields(script_len = script.len()))]
    pub fn execute(&self, script: &str) -> crate::Result<PluginResult> {
        info!("Executing plugin script");

        match self.lua.load(script).exec() {
            Ok(()) => {
                // Try to get return value from spectre.result
                let output = self
                    .lua
                    .globals()
                    .get::<String>("_spectre_output")
                    .unwrap_or_default();

                Ok(PluginResult {
                    success: true,
                    output,
                    error: None,
                })
            },
            Err(e) => {
                let error_msg = e.to_string();
                Ok(PluginResult {
                    success: false,
                    output: String::new(),
                    error: Some(error_msg),
                })
            },
        }
    }

    /// Execute a Lua script file in the sandbox
    pub fn execute_file(&self, path: &std::path::Path) -> crate::Result<PluginResult> {
        let script = std::fs::read_to_string(path).map_err(|e| {
            crate::SpectreError::Plugin(crate::error::PluginError::RuntimeError(format!(
                "Failed to read script file: {}",
                e
            )))
        })?;

        self.execute(&script)
    }

    /// Set a global variable in the sandbox
    pub fn set_global(&self, name: &str, value: &str) -> crate::Result<()> {
        self.lua.globals().set(name, value.to_string())?;
        Ok(())
    }

    /// Get a global variable from the sandbox
    pub fn get_global(&self, name: &str) -> crate::Result<Option<String>> {
        let value: Value = self.lua.globals().get(name)?;
        match value {
            Value::String(s) => Ok(Some(s.to_str()?.to_string())),
            Value::Nil => Ok(None),
            _ => Ok(Some(format!("{:?}", value))),
        }
    }

    /// Get the permissions for this sandbox
    pub fn permissions(&self) -> &PermissionConfig {
        &self.permissions
    }

    /// Get the resource limits for this sandbox
    pub fn limits(&self) -> &ResourceLimits {
        &self.limits
    }
}

/// Convert mlua errors to SpectreError
impl From<mlua::Error> for crate::SpectreError {
    fn from(err: mlua::Error) -> Self {
        Self::Plugin(crate::error::PluginError::RuntimeError(err.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_sandbox() -> PluginSandbox {
        PluginSandbox::new(PermissionConfig::default(), ResourceLimits::default()).unwrap()
    }

    #[test]
    fn test_sandbox_creation() {
        let sandbox = make_sandbox();
        assert!(!sandbox.permissions().network);
    }

    #[test]
    fn test_execute_simple_script() {
        let sandbox = make_sandbox();
        let result = sandbox.execute("_spectre_output = 'hello world'").unwrap();
        assert!(result.success);
        assert_eq!(result.output, "hello world");
    }

    #[test]
    fn test_execute_math() {
        let sandbox = make_sandbox();
        let result = sandbox
            .execute("_spectre_output = tostring(2 + 2)")
            .unwrap();
        assert!(result.success);
        assert_eq!(result.output, "4");
    }

    #[test]
    fn test_execute_error() {
        let sandbox = make_sandbox();
        let result = sandbox.execute("error('test error')").unwrap();
        assert!(!result.success);
        assert!(result.error.is_some());
        assert!(result.error.unwrap().contains("test error"));
    }

    #[test]
    fn test_sandbox_no_os() {
        let sandbox = make_sandbox();
        let result = sandbox.execute("os.execute('ls')").unwrap();
        assert!(!result.success);
    }

    #[test]
    fn test_sandbox_no_io() {
        let sandbox = make_sandbox();
        let result = sandbox.execute("io.open('/etc/passwd')").unwrap();
        assert!(!result.success);
    }

    #[test]
    fn test_sandbox_no_loadfile() {
        let sandbox = make_sandbox();
        let result = sandbox.execute("loadfile('/etc/passwd')").unwrap();
        assert!(!result.success);
    }

    #[test]
    fn test_set_get_global() {
        let sandbox = make_sandbox();
        sandbox.set_global("test_var", "test_value").unwrap();
        let value = sandbox.get_global("test_var").unwrap();
        assert_eq!(value, Some("test_value".to_string()));
    }

    #[test]
    fn test_get_nonexistent_global() {
        let sandbox = make_sandbox();
        let value = sandbox.get_global("nonexistent").unwrap();
        assert!(value.is_none());
    }

    #[test]
    fn test_spectre_api_available() {
        let sandbox = make_sandbox();
        let result = sandbox.execute("spectre.log('test message')").unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_spectre_version() {
        let sandbox = make_sandbox();
        let result = sandbox
            .execute("_spectre_output = spectre.version()")
            .unwrap();
        assert!(result.success);
        assert!(!result.output.is_empty());
    }

    #[test]
    fn test_execute_file_not_found() {
        let sandbox = make_sandbox();
        let result = sandbox.execute_file(std::path::Path::new("/nonexistent/script.lua"));
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_lua_file() {
        let dir = tempfile::tempdir().unwrap();
        let script_path = dir.path().join("test.lua");
        std::fs::write(&script_path, "_spectre_output = 'from file'").unwrap();

        let sandbox = make_sandbox();
        let result = sandbox.execute_file(&script_path).unwrap();
        assert!(result.success);
        assert_eq!(result.output, "from file");
    }
}
