//! TUI dashboard command
//!
//! Launches the SPECTRE terminal user interface with real-time
//! multi-panel dashboard for scan monitoring, data analysis,
//! secure communications, and campaign management.

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Args;
use tracing::{debug, info, instrument};

/// Arguments for the tui command
#[derive(Args, Debug)]
#[command(
    about = "Launch the TUI dashboard",
    long_about = "Launch the SPECTRE terminal user interface.\n\n\
                  Provides a real-time, multi-panel dashboard with:\n\
                  • Recon panel - scan monitoring and results\n\
                  • Analysis panel - CyberChef data transformation\n\
                  • Comms panel - WRAITH secure communications\n\
                  • Campaign panel - operation management\n\n\
                  Keybindings: Tab/Shift-Tab to switch panels, F1 for help, q to quit."
)]
pub struct TuiArgs;

/// Execute the tui command
#[instrument(skip(config_path))]
pub async fn execute(_args: TuiArgs, config_path: Option<PathBuf>) -> Result<()> {
    info!("Launching SPECTRE TUI dashboard");
    debug!(?config_path, "Configuration path");

    let config = spectre_core::config::load_config(config_path.as_deref())
        .context("Failed to load configuration")?;

    spectre_tui::run(&config).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tui_args_default() {
        // TuiArgs has no fields, verify it can be constructed
        let _args = TuiArgs;
    }
}
