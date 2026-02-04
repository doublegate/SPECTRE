//! SPECTRE Core Library
//!
//! This crate provides the core functionality for the SPECTRE unified security toolkit.
//!
//! # Modules
//!
//! - [`config`] - Configuration management with multi-source loading
//! - [`error`] - Error types and handling
//! - [`logging`] - Logging configuration and utilities
//! - [`scan`] - Network scanning interface (ProRT-IP integration)
//! - [`chef`] - Data transformation interface (CyberChef-MCP integration)
//! - [`comms`] - Secure communications interface (WRAITH integration)
//!
//! # Example
//!
//! ```no_run
//! use spectre_core::{config, scan, chef, comms};
//!
//! # async fn example() -> anyhow::Result<()> {
//! // Load configuration
//! let config = config::load_config(None)?;
//!
//! // Create a scanner
//! let scanner = scan::create_scanner(&config)?;
//!
//! // Create CyberChef client
//! let chef = chef::create_chef_client(&config).await?;
//!
//! // Load identity for WRAITH
//! let identity = comms::load_identity(&config)?;
//! # Ok(())
//! # }
//! ```

#![warn(clippy::all)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::unused_async)]

pub mod chef;
pub mod comms;
pub mod config;
pub mod error;
pub mod logging;
pub mod scan;

/// Re-export commonly used types
pub use error::{Result, SpectreError};
