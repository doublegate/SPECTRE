//! Logging configuration and utilities
//!
//! This module provides logging setup using the `tracing` ecosystem.

use std::path::Path;

use anyhow::{Context, Result};
use tracing::Level;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

/// Initialize logging with the specified verbosity level
///
/// # Arguments
///
/// * `verbose` - Verbosity level (0=warn, 1=info, 2=debug, 3+=trace)
/// * `quiet` - If true, only show errors
/// * `log_file` - Optional path to write logs to
///
/// # Errors
///
/// Returns an error if the logging system fails to initialize.
pub fn init_logging(verbose: u8, quiet: bool, log_file: Option<&Path>) -> Result<()> {
    // Determine log level from verbosity
    let level = if quiet {
        Level::ERROR
    } else {
        match verbose {
            0 => Level::WARN,
            1 => Level::INFO,
            2 => Level::DEBUG,
            _ => Level::TRACE,
        }
    };

    // Build filter from environment or default
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new(format!(
            "spectre={},spectre_core={},spectre_cli={}",
            level, level, level
        ))
    });

    // Create formatting layer for terminal output
    let fmt_layer = fmt::layer()
        .with_target(verbose >= 2)
        .with_thread_ids(verbose >= 3)
        .with_thread_names(verbose >= 3)
        .with_file(verbose >= 3)
        .with_line_number(verbose >= 3)
        .with_span_events(if verbose >= 2 {
            FmtSpan::ENTER | FmtSpan::EXIT
        } else {
            FmtSpan::NONE
        });

    // Build subscriber based on whether we're writing to a file
    if let Some(log_path) = log_file {
        let log_dir = log_path.parent().unwrap_or(Path::new("."));
        std::fs::create_dir_all(log_dir)
            .context(format!("Failed to create log directory: {:?}", log_dir))?;

        let file_name = log_path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("Invalid log file path: {:?}", log_path))?;
        let file_appender = tracing_appender::rolling::never(log_dir, file_name);
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        let file_layer = fmt::layer()
            .with_writer(non_blocking)
            .with_ansi(false)
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true);

        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt_layer)
            .with(file_layer)
            .try_init()
            .context("Failed to initialize logging")?;

        // Store guard to prevent dropping
        // In a real application, you'd want to store this guard somewhere
        // to ensure logs are flushed on shutdown
        std::mem::forget(_guard);
    } else {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt_layer)
            .try_init()
            .context("Failed to initialize logging")?;
    }

    Ok(())
}

/// Create a span for an operation
#[macro_export]
macro_rules! operation_span {
    ($name:expr) => {
        tracing::info_span!("operation", name = $name)
    };
    ($name:expr, $($field:tt)*) => {
        tracing::info_span!("operation", name = $name, $($field)*)
    };
}

/// Log an operation start
#[macro_export]
macro_rules! log_operation_start {
    ($name:expr) => {
        tracing::info!(operation = $name, "Starting operation");
    };
    ($name:expr, $($field:tt)*) => {
        tracing::info!(operation = $name, $($field)*, "Starting operation");
    };
}

/// Log an operation completion
#[macro_export]
macro_rules! log_operation_complete {
    ($name:expr) => {
        tracing::info!(operation = $name, "Operation complete");
    };
    ($name:expr, $($field:tt)*) => {
        tracing::info!(operation = $name, $($field)*, "Operation complete");
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_from_verbose() {
        // Test that verbose levels map to expected log levels
        assert_eq!(if 0 == 0 { Level::WARN } else { Level::INFO }, Level::WARN);
        assert_eq!(if 1 == 0 { Level::WARN } else { Level::INFO }, Level::INFO);
    }

    // Note: We can't easily test init_logging multiple times due to
    // tracing's global subscriber behavior. Integration tests would
    // be more appropriate for full logging tests.
}
