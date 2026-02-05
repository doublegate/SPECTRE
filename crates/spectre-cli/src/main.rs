//! SPECTRE CLI - Unified Offensive Security Toolkit
//!
//! Security Platform for Encrypted Comms, Testing, Enumeration, Recon
//!
//! This CLI orchestrates three components:
//! - **ProRT-IP**: Network reconnaissance and scanning
//! - **CyberChef-MCP**: Data analysis and transformation
//! - **WRAITH-Protocol**: Secure encrypted communications

#![cfg_attr(test, allow(clippy::disallowed_methods))]
#![allow(clippy::unused_async)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::ref_option)] // CLI helper functions use &Option<T> from clap args // CLI args structs need multiple bool flags // CLI handlers are async for dispatch trait uniformity

use anyhow::Result;
use clap::Parser;
use tracing::info;

mod commands;
mod output;

use commands::Cli;

/// Application entry point
#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Initialize logging based on verbosity
    spectre_core::logging::init_logging(cli.verbose, cli.quiet, cli.log_file.as_deref())?;

    info!(version = env!("CARGO_PKG_VERSION"), "SPECTRE CLI starting");

    // Execute the command
    commands::execute(cli).await
}
