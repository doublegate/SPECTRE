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
//! - [`target`] - Enhanced target management with priority queue and scope enforcement
//! - [`job`] - Scan job orchestration with lifecycle state machine
//! - [`results`] - Results aggregation, formatting, and statistics
//! - [`pipeline`] - Composable data pipeline with typed stages
//! - [`campaign`] - Campaign state management with SQLite persistence
//! - [`plugin`] - Lua 5.4 plugin system with sandboxing
//! - [`orchestration`] - Advanced scan orchestration (chaining, templates, scheduling)
//! - [`workflow`] - Workflow automation with DSL and execution engine
//! - [`recipe`] - Recipe management with storage, versioning, and search
//! - [`report`] - Report generation (HTML, Markdown)
//! - [`export`] - Export formats (CSV, custom templates, incremental)
//! - [`perf`] - Performance optimization (caching, pooling, metrics)
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
#![cfg_attr(test, allow(clippy::disallowed_methods))]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::unused_async)]

pub mod campaign;
pub mod chef;
pub mod comms;
pub mod config;
pub mod error;
pub mod export;
pub mod job;
pub mod logging;
pub mod orchestration;
pub mod perf;
pub mod pipeline;
pub mod plugin;
pub mod recipe;
pub mod report;
pub mod results;
pub mod scan;
pub mod target;
pub mod workflow;

/// Re-export commonly used types
pub use error::{Result, SpectreError};
