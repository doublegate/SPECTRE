//! Send command (WRAITH-Protocol integration)
//!
//! This module provides the CLI interface for sending secure messages and files.

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Args;
use tracing::{debug, info, instrument};

/// Arguments for the send command
#[derive(Args, Debug)]
#[command(
    about = "Send secure message or file via WRAITH-Protocol",
    long_about = "Send encrypted messages or files to trusted peers using WRAITH-Protocol.\n\n\
                  All communications are end-to-end encrypted with forward secrecy.\n\
                  Supports text messages, files, and streaming data."
)]
pub struct SendArgs {
    /// Peer ID or alias to send to
    #[arg(value_name = "PEER")]
    pub peer: String,

    /// Message to send (text)
    #[arg(short, long, value_name = "MESSAGE", conflicts_with = "file")]
    pub message: Option<String>,

    /// File to send
    #[arg(short, long, value_name = "FILE", conflicts_with = "message")]
    pub file: Option<PathBuf>,

    /// Send data from stdin
    #[arg(long, conflicts_with_all = ["message", "file"])]
    pub stdin: bool,

    /// Request delivery confirmation
    #[arg(long)]
    pub confirm: bool,

    /// Set message priority (0-9, default 5)
    #[arg(short, long, default_value = "5", value_parser = clap::value_parser!(u8).range(0..=9))]
    pub priority: u8,

    /// Message expiration in seconds (0 = no expiration)
    #[arg(short = 'x', long, default_value = "0")]
    pub expires: u64,

    /// Enable compression
    #[arg(short = 'z', long)]
    pub compress: bool,

    /// Use opportunistic encryption (no peer verification)
    #[arg(long)]
    pub opportunistic: bool,

    /// Custom relay server
    #[arg(long, value_name = "URL")]
    pub relay: Option<String>,

    /// Timeout in seconds
    #[arg(short, long, default_value = "30")]
    pub timeout: u64,
}

/// Execute the send command
#[instrument(skip(args, config_path), fields(peer = %args.peer))]
pub async fn execute(args: SendArgs, config_path: Option<PathBuf>) -> Result<()> {
    info!("Preparing to send secure message");
    debug!(?config_path, "Configuration path");

    // Load configuration
    let config = spectre_core::config::load_config(config_path.as_deref())
        .context("Failed to load configuration")?;

    // Verify identity exists
    let identity = spectre_core::comms::load_identity(&config)
        .context("No identity found. Run 'spectre identity init' first.")?;

    debug!(identity_id = %identity.id(), "Using identity");

    // Resolve peer
    let peer = spectre_core::comms::resolve_peer(&args.peer, &config).context(format!(
        "Peer '{}' not found. Add with 'spectre peer add'.",
        args.peer
    ))?;

    debug!(peer_id = %peer.id(), "Resolved peer");

    // Get data to send
    let data = get_send_data(&args.message, args.file.as_deref(), args.stdin)
        .await
        .context("Failed to get data to send")?;

    let data_len = data.len();
    debug!(data_size = data_len, "Data prepared");

    // Build send options
    let send_opts = spectre_core::comms::SendOptions {
        confirm_delivery: args.confirm,
        priority: args.priority,
        expires_in: if args.expires > 0 {
            Some(std::time::Duration::from_secs(args.expires))
        } else {
            None
        },
        compress: args.compress,
        opportunistic: args.opportunistic,
        relay: args.relay,
        timeout: std::time::Duration::from_secs(args.timeout),
    };

    // Create WRAITH client
    let client = spectre_core::comms::create_client(&config, &identity)
        .await
        .context("Failed to create WRAITH client")?;

    // Send with progress
    let progress = indicatif::ProgressBar::new(data_len as u64);
    progress.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec})")?
            .progress_chars("#>-"),
    );

    let progress_clone = progress.clone();
    let result = client
        .send(
            &peer,
            &data,
            &send_opts,
            Some(Box::new(move |sent| {
                progress_clone.set_position(sent as u64);
            })),
        )
        .await;

    progress.finish_and_clear();

    match result {
        Ok(receipt) => {
            println!("Message sent successfully.");
            println!("  Message ID: {}", receipt.message_id);
            println!("  Size: {data_len} bytes");

            if args.confirm {
                println!(
                    "  Delivery: {}",
                    if receipt.delivered {
                        "Confirmed"
                    } else {
                        "Pending"
                    }
                );
            }

            if let Some(timestamp) = receipt.timestamp {
                println!("  Timestamp: {timestamp}");
            }

            info!(
                message_id = %receipt.message_id,
                bytes = data_len,
                "Message sent successfully"
            );
            Ok(())
        },
        Err(e) => {
            anyhow::bail!("Failed to send message: {e}");
        },
    }
}

/// Get data to send from various sources
async fn get_send_data(
    message: &Option<String>,
    file: Option<&std::path::Path>,
    stdin: bool,
) -> Result<Vec<u8>> {
    if let Some(msg) = message {
        Ok(msg.as_bytes().to_vec())
    } else if let Some(path) = file {
        tokio::fs::read(path)
            .await
            .context(format!("Failed to read file: {}", path.display()))
    } else if stdin {
        use tokio::io::AsyncReadExt;
        let mut buffer = Vec::new();
        tokio::io::stdin().read_to_end(&mut buffer).await?;
        Ok(buffer)
    } else {
        anyhow::bail!("No data specified. Use --message, --file, or --stdin.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_send_data_message() {
        let data = get_send_data(&Some("Hello, World!".to_string()), None, false)
            .await
            .unwrap();
        assert_eq!(data, b"Hello, World!");
    }

    #[tokio::test]
    async fn test_get_send_data_none() {
        let result = get_send_data(&None, None, false).await;
        assert!(result.is_err());
    }
}
