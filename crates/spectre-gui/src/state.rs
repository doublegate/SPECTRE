use std::path::PathBuf;
use std::sync::Arc;

use spectre_core::campaign::CampaignStore;
use spectre_core::config::Config;
use tokio::sync::{Mutex, RwLock};

/// Shared application state managed by Tauri.
///
/// Held behind `Arc<RwLock<_>>` so IPC command handlers can access it
/// concurrently from any thread.
pub struct AppState {
    /// SPECTRE configuration
    pub config: RwLock<Config>,
    /// Campaign persistent storage
    pub campaign_store: Arc<Mutex<Option<CampaignStore>>>,
}

impl AppState {
    /// Create a new `AppState` with the given config.
    pub fn new(config: Config) -> Arc<Self> {
        Arc::new(Self {
            config: RwLock::new(config),
            campaign_store: Arc::new(Mutex::new(None)),
        })
    }

    /// Get or initialize the campaign store with a default database path.
    ///
    /// Uses ~/.spectre/campaigns.db by default.
    pub async fn get_campaign_store(&self) -> Result<Arc<Mutex<Option<CampaignStore>>>, String> {
        let mut store_guard = self.campaign_store.lock().await;

        if store_guard.is_none() {
            let db_path = Self::default_campaign_db_path();

            // Ensure parent directory exists
            if let Some(parent) = db_path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create campaign directory: {}", e))?;
            }

            let store = CampaignStore::open(&db_path)
                .map_err(|e| format!("Failed to open campaign database: {}", e))?;

            *store_guard = Some(store);
        }

        Ok(self.campaign_store.clone())
    }

    /// Get the default campaign database path.
    fn default_campaign_db_path() -> PathBuf {
        if let Some(home) = dirs::home_dir() {
            home.join(".spectre").join("campaigns.db")
        } else {
            PathBuf::from("campaigns.db")
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            config: RwLock::new(Config::default()),
            campaign_store: Arc::new(Mutex::new(None)),
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
        let _config = state
            .config
            .try_read()
            .expect("Failed to read config in test");
    }

    #[tokio::test]
    async fn test_app_state_config_read() {
        let state = AppState::new(Config::default());
        let config = state.config.read().await;
        // Default config should have default general settings
        assert!(config.general.verbosity == 0);
    }
}
