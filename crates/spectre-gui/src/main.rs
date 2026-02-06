#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Initialize logging first
    init_logging();

    // Configure display server and GPU workarounds
    // This must happen BEFORE Tauri initialization
    #[cfg(target_os = "linux")]
    {
        let config = spectre_gui::display::configure_display_server();
        spectre_gui::display::print_display_config(&config);
    }

    #[cfg(not(target_os = "linux"))]
    {
        println!("🚀 Starting SPECTRE GUI...");
        println!();
    }

    // Run the Tauri application
    spectre_gui::run();
}

/// Initialize logging for the GUI application.
///
/// Uses environment variable RUST_LOG to control verbosity:
/// - Not set: INFO level
/// - RUST_LOG=debug: DEBUG level
/// - RUST_LOG=trace: TRACE level
fn init_logging() {
    use tracing_subscriber::EnvFilter;

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();
}
