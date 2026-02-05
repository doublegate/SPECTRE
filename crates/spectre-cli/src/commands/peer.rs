//! Peer management command (WRAITH-Protocol integration)
//!
//! This module provides the CLI interface for managing trusted peers.

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use tracing::{debug, info, instrument};

/// Arguments for the peer command
#[derive(Args, Debug)]
#[command(
    about = "Manage trusted peers",
    long_about = "Manage trusted peers for WRAITH-Protocol communications.\n\n\
                  Peers must be added before secure communications can be established."
)]
pub struct PeerArgs {
    #[command(subcommand)]
    pub command: PeerCommands,
}

/// Peer subcommands
#[derive(Subcommand, Debug)]
pub enum PeerCommands {
    /// Add a new trusted peer
    Add(AddArgs),

    /// List all peers
    List(ListArgs),

    /// Show peer details
    Show(ShowArgs),

    /// Remove a peer
    Remove(RemoveArgs),

    /// Verify peer's key
    Verify(VerifyArgs),

    /// Update peer information
    Update(UpdateArgs),
}

/// Arguments for peer add
#[derive(Args, Debug)]
pub struct AddArgs {
    /// Peer's public key (file path, base64, or fingerprint)
    #[arg(value_name = "KEY")]
    pub key: String,

    /// Alias for the peer
    #[arg(short, long)]
    pub alias: Option<String>,

    /// Trust level (0-10, default 5)
    #[arg(short, long, default_value = "5", value_parser = clap::value_parser!(u8).range(0..=10))]
    pub trust: u8,

    /// Additional notes
    #[arg(short, long)]
    pub notes: Option<String>,

    /// Skip key verification prompt
    #[arg(long)]
    pub no_verify: bool,
}

/// Arguments for peer list
#[derive(Args, Debug)]
pub struct ListArgs {
    /// Filter by minimum trust level
    #[arg(short, long)]
    pub min_trust: Option<u8>,

    /// Output in JSON format
    #[arg(long)]
    pub json: bool,
}

/// Arguments for peer show
#[derive(Args, Debug)]
pub struct ShowArgs {
    /// Peer ID or alias
    pub peer: String,

    /// Output in JSON format
    #[arg(long)]
    pub json: bool,
}

/// Arguments for peer remove
#[derive(Args, Debug)]
pub struct RemoveArgs {
    /// Peer ID or alias
    pub peer: String,

    /// Skip confirmation prompt
    #[arg(long)]
    pub force: bool,
}

/// Arguments for peer verify
#[derive(Args, Debug)]
pub struct VerifyArgs {
    /// Peer ID or alias
    pub peer: String,

    /// Expected fingerprint for out-of-band verification
    #[arg(short, long)]
    pub fingerprint: Option<String>,
}

/// Arguments for peer update
#[derive(Args, Debug)]
pub struct UpdateArgs {
    /// Peer ID or alias
    pub peer: String,

    /// New alias
    #[arg(short, long)]
    pub alias: Option<String>,

    /// New trust level (0-10)
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(0..=10))]
    pub trust: Option<u8>,

    /// New notes
    #[arg(short, long)]
    pub notes: Option<String>,
}

/// Execute the peer command
#[instrument(skip(args, config_path))]
pub async fn execute(args: PeerArgs, config_path: Option<PathBuf>) -> Result<()> {
    debug!(?config_path, "Configuration path");

    // Load configuration
    let config = spectre_core::config::load_config(config_path.as_deref())
        .context("Failed to load configuration")?;

    match args.command {
        PeerCommands::Add(add_args) => execute_add(add_args, &config).await,
        PeerCommands::List(list_args) => execute_list(list_args, &config).await,
        PeerCommands::Show(show_args) => execute_show(show_args, &config).await,
        PeerCommands::Remove(remove_args) => execute_remove(remove_args, &config).await,
        PeerCommands::Verify(verify_args) => execute_verify(verify_args, &config).await,
        PeerCommands::Update(update_args) => execute_update(update_args, &config).await,
    }
}

/// Execute peer add
async fn execute_add(args: AddArgs, config: &spectre_core::config::Config) -> Result<()> {
    use std::io::Write;

    info!("Adding new peer");

    // Load public key from various sources
    let public_key = load_public_key(&args.key).await?;

    // Display key fingerprint for verification
    let fingerprint = spectre_core::comms::compute_fingerprint(&public_key);

    if !args.no_verify {
        println!("Peer Public Key Fingerprint:");
        println!();
        println!("  {fingerprint}");
        println!();
        print!("Does this match the expected fingerprint? [y/N] ");

        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            anyhow::bail!("Key verification failed. Peer not added.");
        }
    }

    // Create peer
    let peer = spectre_core::comms::Peer::new(
        public_key,
        args.alias.clone(),
        args.trust,
        args.notes.clone(),
    );

    // Save peer
    peer.save(config)?;

    println!();
    println!("Peer added successfully!");
    println!("  ID: {}", peer.id());
    println!("  Alias: {}", args.alias.as_deref().unwrap_or("<none>"));
    println!("  Trust Level: {}/10", args.trust);
    println!("  Fingerprint: {fingerprint}");

    info!(
        peer_id = %peer.id(),
        alias = ?args.alias,
        "Peer added"
    );

    Ok(())
}

/// Execute peer list
async fn execute_list(args: ListArgs, config: &spectre_core::config::Config) -> Result<()> {
    let peers = spectre_core::comms::list_peers(config)?;

    // Filter by trust level if specified
    let peers: Vec<_> = if let Some(min_trust) = args.min_trust {
        peers.into_iter().filter(|p| p.trust >= min_trust).collect()
    } else {
        peers
    };

    if peers.is_empty() {
        println!("No peers found. Add one with 'spectre peer add'.");
        return Ok(());
    }

    if args.json {
        let json = serde_json::to_string_pretty(&peers)?;
        println!("{json}");
    } else {
        println!("Trusted Peers:");
        println!();
        println!(
            "{:<20} {:<16} {:<6} {:<24}",
            "Alias", "ID", "Trust", "Last Seen"
        );
        println!("{}", "-".repeat(70));

        for peer in peers {
            let alias = peer.alias.as_deref().unwrap_or("<none>");
            let id = &peer.id[..16];
            let last_seen = peer
                .last_seen
                .map_or_else(|| "Never".to_string(), |t| t.to_string());

            println!(
                "{:<20} {:<16} {:<6} {:<24}",
                alias, id, peer.trust, last_seen
            );
        }
    }

    Ok(())
}

/// Execute peer show
async fn execute_show(args: ShowArgs, config: &spectre_core::config::Config) -> Result<()> {
    let peer = spectre_core::comms::resolve_peer(&args.peer, config)?;

    if args.json {
        let json = serde_json::to_string_pretty(&peer)?;
        println!("{json}");
    } else {
        println!("Peer Information:");
        println!();
        println!("  ID: {}", peer.id());
        println!("  Alias: {}", peer.alias().unwrap_or("<none>"));
        println!("  Trust Level: {}/10", peer.trust());
        println!("  Fingerprint: {}", peer.fingerprint());
        println!("  Key Type: {}", peer.key_type());
        println!("  Added: {}", peer.created_at());

        if let Some(last_seen) = peer.last_seen() {
            println!("  Last Seen: {last_seen}");
        }

        if let Some(notes) = peer.notes() {
            println!();
            println!("  Notes: {notes}");
        }
    }

    Ok(())
}

/// Execute peer remove
async fn execute_remove(args: RemoveArgs, config: &spectre_core::config::Config) -> Result<()> {
    use std::io::Write;

    let peer = spectre_core::comms::resolve_peer(&args.peer, config)?;

    if !args.force {
        print!(
            "Are you sure you want to remove peer '{}'? [y/N] ",
            peer.alias().unwrap_or_else(|| &peer.id()[..16])
        );

        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            return Ok(());
        }
    }

    spectre_core::comms::remove_peer(peer.id(), config)?;

    println!("Peer removed.");

    info!(peer_id = %peer.id(), "Peer removed");

    Ok(())
}

/// Execute peer verify
async fn execute_verify(args: VerifyArgs, config: &spectre_core::config::Config) -> Result<()> {
    let peer = spectre_core::comms::resolve_peer(&args.peer, config)?;

    let fingerprint = peer.fingerprint();

    println!("Peer Fingerprint:");
    println!();
    println!("  {fingerprint}");
    println!();

    if let Some(expected) = &args.fingerprint {
        let normalized_expected = expected.replace([':', ' '], "").to_uppercase();
        let normalized_actual = fingerprint.replace([':', ' '], "").to_uppercase();

        if normalized_expected == normalized_actual {
            println!("Fingerprint MATCHES expected value.");
            Ok(())
        } else {
            anyhow::bail!(
                "Fingerprint DOES NOT MATCH!\n  Expected: {expected}\n  Actual: {fingerprint}"
            );
        }
    } else {
        println!("Verify this fingerprint through a trusted channel (e.g., in person, phone).");
        Ok(())
    }
}

/// Execute peer update
#[allow(clippy::useless_let_if_seq)] // Multiple conditions set updated
async fn execute_update(args: UpdateArgs, config: &spectre_core::config::Config) -> Result<()> {
    let mut peer = spectre_core::comms::resolve_peer(&args.peer, config)?;

    let mut updated = false;

    if let Some(alias) = args.alias {
        peer.set_alias(Some(alias));
        updated = true;
    }

    if let Some(trust) = args.trust {
        peer.set_trust(trust);
        updated = true;
    }

    if let Some(notes) = args.notes {
        peer.set_notes(Some(notes));
        updated = true;
    }

    if updated {
        peer.save(config)?;
        println!("Peer updated.");
    } else {
        println!("No changes specified.");
    }

    Ok(())
}

/// Load public key from various sources
async fn load_public_key(key: &str) -> Result<Vec<u8>> {
    // Check if it's a file path
    let path = std::path::Path::new(key);
    if path.exists() {
        return tokio::fs::read(path)
            .await
            .context("Failed to read key file");
    }

    // Try to decode as hex first (more specific format)
    if let Ok(decoded) = hex::decode(key.replace(':', "")) {
        return Ok(decoded);
    }

    // Try to decode as base64
    if let Ok(decoded) = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, key) {
        return Ok(decoded);
    }

    anyhow::bail!("Could not parse key. Provide a file path, base64 string, or hex fingerprint.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_load_public_key_hex() {
        let hex_key = "0102030405060708";
        let result = load_public_key(hex_key).await.unwrap();
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[tokio::test]
    async fn test_load_public_key_hex_with_colons() {
        let hex_key = "01:02:03:04:05:06:07:08";
        let result = load_public_key(hex_key).await.unwrap();
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
