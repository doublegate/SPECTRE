//! SPECTRE TUI Dashboard
//!
//! This crate provides the terminal user interface for the SPECTRE unified
//! offensive security toolkit. It offers a real-time, multi-panel dashboard
//! for monitoring and controlling scan operations, data analysis, secure
//! communications, and campaign management.
//!
//! # Architecture
//!
//! The TUI is built on [ratatui](https://ratatui.rs) with [crossterm](https://docs.rs/crossterm)
//! as the backend. It uses a single-threaded render loop with async event
//! handling via [tokio].
//!
//! # Modules
//!
//! - [`app`] - Main application state and logic
//! - [`event`] - Event handling (keyboard, mouse, tick)
//! - [`terminal`] - Terminal initialization and restoration
//! - [`tui`] - Main run loop entry point
//! - [`layout`] - Panel layout computation
//! - [`theme`] - Color themes and styling
//! - [`command`] - Command input and parsing
//! - [`keybindings`] - Keyboard shortcut definitions
//! - [`scan_state`] - Real-time scan progress tracking
//! - [`panels`] - Panel implementations (recon, analysis, comms, campaign)
//! - [`widgets`] - Reusable widget components

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

pub mod app;
pub mod command;
pub mod event;
pub mod keybindings;
pub mod layout;
pub mod panels;
pub mod scan_state;
pub mod terminal;
pub mod theme;
pub mod tui;
pub mod widgets;

// Re-export primary entry points
pub use app::App;
pub use tui::run;
