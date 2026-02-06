//! SPECTRE GUI Application
//!
//! Cross-platform desktop GUI for SPECTRE using Tauri 2.0 + React.
//! This crate provides both the Tauri backend (IPC command handlers,
//! state management, event streaming) and the binary entry point.

pub mod commands;
pub mod events;
pub mod state;

use state::AppState;

/// Run the Tauri application.
///
/// Called from `main.rs`. Sets up the Tauri Builder with plugins,
/// managed state, and IPC command handlers.
#[allow(clippy::disallowed_methods)] // tauri::generate_context! uses unwrap internally
pub fn run() {
    let config = spectre_core::config::Config::default();
    let app_state = AppState::new(config);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            // Status
            commands::status::get_version,
            commands::status::get_status,
            // Scan
            commands::scan::start_scan,
            commands::scan::stop_scan,
            commands::scan::get_scan_results,
            // Chef
            commands::chef::execute_chef,
            commands::chef::list_chef_operations,
            // Comms
            commands::comms::get_identity,
            commands::comms::list_peers,
            commands::comms::send_data,
            // Campaign
            commands::campaign::create_campaign,
            commands::campaign::list_campaigns,
            commands::campaign::get_campaign,
            commands::campaign::advance_campaign,
            // Config
            commands::config::get_config,
            commands::config::set_config,
            // Results
            commands::results::get_dashboard_stats,
            commands::results::get_findings,
            // Report
            commands::report::generate_report,
            commands::report::export_data,
            // Target
            commands::target::parse_targets,
        ])
        .run(tauri::generate_context!())
        .expect("error running SPECTRE GUI");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_creation() {
        let config = spectre_core::config::Config::default();
        let state = AppState::new(config);
        assert!(std::sync::Arc::strong_count(&state) == 1);
    }
}
