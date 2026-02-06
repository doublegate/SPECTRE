use std::sync::Arc;

use spectre_core::config::Config;
use tokio::sync::RwLock;

/// Shared application state managed by Tauri.
///
/// Held behind `Arc<RwLock<_>>` so IPC command handlers can access it
/// concurrently from any thread.
pub struct AppState {
    /// SPECTRE configuration
    pub config: RwLock<Config>,
}

impl AppState {
    /// Create a new `AppState` with the given config.
    pub fn new(config: Config) -> Arc<Self> {
        Arc::new(Self {
            config: RwLock::new(config),
        })
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            config: RwLock::new(Config::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_new() {
        let state = AppState::new(Config::default());
        assert!(Arc::strong_count(&state) == 1);
    }

    #[test]
    fn test_app_state_default() {
        let state = AppState::default();
        // Should not panic
        let _config = state.config.try_read().unwrap();
    }

    #[tokio::test]
    async fn test_app_state_config_read() {
        let state = AppState::new(Config::default());
        let config = state.config.read().await;
        // Default config should have default general settings
        assert!(config.general.verbosity == 0);
    }
}
