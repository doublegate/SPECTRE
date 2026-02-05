//! Receive command (WRAITH-Protocol integration)
//!
//! This module provides the CLI interface for receiving secure messages and files.

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Args;
use tracing::{debug, info, instrument};

/// Arguments for the receive command
#[derive(Args, Debug)]
#[command(
    about = "Receive secure message or file via WRAITH-Protocol",
    long_about = "Receive encrypted messages or files from trusted peers using WRAITH-Protocol.\n\n\
                  Can operate in single-receive or listen mode for continuous reception."
)]
pub struct ReceiveArgs {
    /// Listen continuously for messages
    #[arg(short, long)]
    pub listen: bool,

    /// Filter by sender peer ID or alias
    #[arg(short, long, value_name = "PEER")]
    pub from: Option<String>,

    /// Save files to directory
    #[arg(short = 'd', long, value_name = "DIR")]
    pub output_dir: Option<PathBuf>,

    /// Accept messages from unknown peers
    #[arg(long)]
    pub accept_unknown: bool,

    /// Maximum messages to receive (0 = unlimited in listen mode)
    #[arg(short = 'n', long, default_value = "1")]
    pub count: usize,

    /// Timeout in seconds (0 = wait indefinitely)
    #[arg(short, long, default_value = "0")]
    pub timeout: u64,

    /// Send automatic delivery confirmations
    #[arg(long)]
    pub auto_confirm: bool,

    /// Output received data to stdout
    #[arg(long)]
    pub stdout: bool,

    /// Custom relay server
    #[arg(long, value_name = "URL")]
    pub relay: Option<String>,
}

/// Execute the receive command
#[instrument(skip(args, config_path))]
pub async fn execute(args: ReceiveArgs, config_path: Option<PathBuf>) -> Result<()> {
    info!("Starting secure message receiver");
    debug!(?config_path, "Configuration path");

    // Load configuration
    let config = spectre_core::config::load_config(config_path.as_deref())
        .context("Failed to load configuration")?;

    // Verify identity exists
    let identity = spectre_core::comms::load_identity(&config)
        .context("No identity found. Run 'spectre identity init' first.")?;

    debug!(identity_id = %identity.id(), "Using identity");

    // Build receive options
    let recv_opts = spectre_core::comms::ReceiveOptions {
        filter_sender: args.from.clone(),
        accept_unknown: args.accept_unknown,
        auto_confirm: args.auto_confirm,
        relay: args.relay.clone(),
        timeout: if args.timeout > 0 {
            Some(std::time::Duration::from_secs(args.timeout))
        } else {
            None
        },
    };

    // Create WRAITH client
    let client = spectre_core::comms::create_client(&config, &identity)
        .await
        .context("Failed to create WRAITH client")?;

    if args.listen {
        // Listen mode
        println!("Listening for messages... (Ctrl+C to stop)");

        let max_count = if args.count == 0 {
            usize::MAX
        } else {
            args.count
        };
        let mut received = 0;

        loop {
            if received >= max_count {
                break;
            }

            match client.receive(&recv_opts).await {
                Ok(msg) => {
                    received += 1;
                    handle_received_message(&msg, &args, received).await?;
                },
                Err(spectre_core::SpectreError::Comms(
                    spectre_core::error::CommsError::Timeout,
                )) => {
                    if args.timeout > 0 {
                        println!("Timeout waiting for messages.");
                        break;
                    }
                    // Otherwise continue listening
                },
                Err(e) => {
                    tracing::warn!(error = %e, "Error receiving message");
                },
            }
        }

        println!("\nReceived {received} message(s).");
    } else {
        // Single receive mode
        let max_count = args.count;

        for i in 0..max_count {
            match client.receive(&recv_opts).await {
                Ok(msg) => {
                    handle_received_message(&msg, &args, i + 1).await?;
                },
                Err(spectre_core::SpectreError::Comms(
                    spectre_core::error::CommsError::Timeout,
                )) => {
                    if args.timeout > 0 {
                        anyhow::bail!("Timeout waiting for message");
                    }
                },
                Err(e) => {
                    anyhow::bail!("Failed to receive message: {e}");
                },
            }
        }
    }

    Ok(())
}

/// Handle a received message
async fn handle_received_message(
    msg: &spectre_core::comms::ReceivedMessage,
    args: &ReceiveArgs,
    index: usize,
) -> Result<()> {
    if args.stdout {
        // Output to stdout
        use std::io::Write;
        std::io::stdout().write_all(&msg.data)?;
        return Ok(());
    }

    // Print message info
    println!("\n--- Message #{index} ---");
    println!("  ID: {}", msg.id);
    println!("  From: {}", msg.sender);
    println!("  Size: {} bytes", msg.data.len());
    println!("  Timestamp: {}", msg.timestamp);

    if let Some(expires) = &msg.expires_at {
        println!("  Expires: {expires}");
    }

    // Save or display content
    if let Some(output_dir) = &args.output_dir {
        let filename = format!("message_{}.bin", msg.id);
        let path = output_dir.join(&filename);

        tokio::fs::create_dir_all(output_dir).await?;
        tokio::fs::write(&path, &msg.data).await?;

        println!("  Saved to: {}", path.display());
    } else {
        // Display as text if possible
        if let Ok(text) = std::str::from_utf8(&msg.data) {
            println!("  Content:\n{text}");
        } else {
            println!("  Content: <binary data, use --output-dir to save>");
        }
    }

    info!(
        message_id = %msg.id,
        sender = %msg.sender,
        bytes = msg.data.len(),
        "Message received"
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_receive_args_defaults() {
        let args = ReceiveArgs {
            listen: false,
            from: None,
            output_dir: None,
            accept_unknown: false,
            count: 1,
            timeout: 0,
            auto_confirm: false,
            stdout: false,
            relay: None,
        };

        assert!(!args.listen);
        assert_eq!(args.count, 1);
        assert_eq!(args.timeout, 0);
    }
}
