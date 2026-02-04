//! Identity management command (WRAITH-Protocol integration)
//!
//! This module provides the CLI interface for managing cryptographic identities.

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use tracing::{debug, info, instrument};

/// Arguments for the identity command
#[derive(Args, Debug)]
#[command(
    about = "Manage cryptographic identities",
    long_about = "Manage WRAITH-Protocol cryptographic identities.\n\n\
                  Identities are used for secure communications and peer authentication.\n\
                  Each identity contains a public/private key pair."
)]
pub struct IdentityArgs {
    #[command(subcommand)]
    pub command: IdentityCommands,
}

/// Identity subcommands
#[derive(Subcommand, Debug)]
pub enum IdentityCommands {
    /// Initialize a new identity
    Init(InitArgs),

    /// Show current identity
    Show(ShowArgs),

    /// Export identity (public key only by default)
    Export(ExportArgs),

    /// Import an identity
    Import(ImportArgs),

    /// List all identities
    List,

    /// Switch active identity
    Switch(SwitchArgs),

    /// Delete an identity
    Delete(DeleteArgs),
}

/// Arguments for identity init
#[derive(Args, Debug)]
pub struct InitArgs {
    /// Identity name/alias
    #[arg(short, long)]
    pub name: Option<String>,

    /// Key type to use
    #[arg(short, long, value_enum, default_value = "ed25519")]
    pub key_type: KeyType,

    /// Additional comment/description
    #[arg(long)]
    pub comment: Option<String>,

    /// Force overwrite existing identity
    #[arg(long)]
    pub force: bool,
}

/// Key type for identity generation
#[derive(Debug, Clone, Copy, Default, clap::ValueEnum)]
pub enum KeyType {
    /// Ed25519 (default, recommended)
    #[default]
    Ed25519,
    /// X25519 (for key exchange only)
    X25519,
    /// Kyber (post-quantum)
    Kyber,
}

/// Arguments for identity show
#[derive(Args, Debug)]
pub struct ShowArgs {
    /// Identity to show (default: active)
    #[arg(value_name = "NAME")]
    pub name: Option<String>,

    /// Show private key (dangerous!)
    #[arg(long)]
    pub show_private: bool,

    /// Output in JSON format
    #[arg(long)]
    pub json: bool,
}

/// Arguments for identity export
#[derive(Args, Debug)]
pub struct ExportArgs {
    /// Identity to export (default: active)
    #[arg(value_name = "NAME")]
    pub name: Option<String>,

    /// Output file path
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Include private key (dangerous!)
    #[arg(long)]
    pub include_private: bool,

    /// Output format
    #[arg(short, long, value_enum, default_value = "pem")]
    pub format: ExportFormat,
}

/// Export format options
#[derive(Debug, Clone, Copy, Default, clap::ValueEnum)]
pub enum ExportFormat {
    /// PEM format (default)
    #[default]
    Pem,
    /// JSON format
    Json,
    /// Raw binary
    Raw,
    /// QR code (terminal display)
    Qr,
}

/// Arguments for identity import
#[derive(Args, Debug)]
pub struct ImportArgs {
    /// Input file path
    #[arg(value_name = "FILE")]
    pub input: PathBuf,

    /// Name for imported identity
    #[arg(short, long)]
    pub name: Option<String>,

    /// Set as active identity
    #[arg(long)]
    pub activate: bool,
}

/// Arguments for identity switch
#[derive(Args, Debug)]
pub struct SwitchArgs {
    /// Identity name to switch to
    pub name: String,
}

/// Arguments for identity delete
#[derive(Args, Debug)]
pub struct DeleteArgs {
    /// Identity name to delete
    pub name: String,

    /// Skip confirmation prompt
    #[arg(long)]
    pub force: bool,
}

/// Execute the identity command
#[instrument(skip(args, config_path))]
pub async fn execute(args: IdentityArgs, config_path: Option<PathBuf>) -> Result<()> {
    debug!(?config_path, "Configuration path");

    // Load configuration
    let config = spectre_core::config::load_config(config_path.as_deref())
        .context("Failed to load configuration")?;

    match args.command {
        IdentityCommands::Init(init_args) => execute_init(init_args, &config).await,
        IdentityCommands::Show(show_args) => execute_show(show_args, &config).await,
        IdentityCommands::Export(export_args) => execute_export(export_args, &config).await,
        IdentityCommands::Import(import_args) => execute_import(import_args, &config).await,
        IdentityCommands::List => execute_list(&config).await,
        IdentityCommands::Switch(switch_args) => execute_switch(switch_args, &config).await,
        IdentityCommands::Delete(delete_args) => execute_delete(delete_args, &config).await,
    }
}

/// Execute identity init
async fn execute_init(args: InitArgs, config: &spectre_core::config::Config) -> Result<()> {
    info!("Initializing new identity");

    // Check for existing identity
    if !args.force {
        if let Ok(existing) = spectre_core::comms::load_identity(config) {
            anyhow::bail!(
                "Identity '{}' already exists. Use --force to overwrite.",
                existing.name().unwrap_or("default")
            );
        }
    }

    let name = args.name.unwrap_or_else(|| "default".to_string());

    // Generate identity
    let key_type = match args.key_type {
        KeyType::Ed25519 => spectre_core::comms::KeyType::Ed25519,
        KeyType::X25519 => spectre_core::comms::KeyType::X25519,
        KeyType::Kyber => spectre_core::comms::KeyType::Kyber,
    };

    let identity =
        spectre_core::comms::Identity::generate(&name, key_type, args.comment.as_deref())
            .context("Failed to generate identity")?;

    // Save identity
    identity.save(config)?;

    println!("Identity initialized successfully!");
    println!();
    println!("  Name: {}", identity.name().unwrap_or("default"));
    println!("  ID: {}", identity.id());
    println!("  Public Key: {}", identity.public_key_fingerprint());
    println!("  Key Type: {:?}", args.key_type);
    println!();
    println!("Share your public key with peers using:");
    println!("  spectre identity export --output mykey.pub");

    info!(
        identity_id = %identity.id(),
        "Identity created"
    );

    Ok(())
}

/// Execute identity show
async fn execute_show(args: ShowArgs, config: &spectre_core::config::Config) -> Result<()> {
    let identity = if let Some(name) = &args.name {
        spectre_core::comms::load_identity_by_name(name, config)?
    } else {
        spectre_core::comms::load_identity(config)?
    };

    if args.json {
        let json = identity.to_json(args.show_private)?;
        println!("{}", serde_json::to_string_pretty(&json)?);
    } else {
        println!("Identity Information:");
        println!();
        println!("  Name: {}", identity.name().unwrap_or("default"));
        println!("  ID: {}", identity.id());
        println!("  Public Key: {}", identity.public_key_fingerprint());
        println!("  Key Type: {}", identity.key_type());
        println!("  Created: {}", identity.created_at());

        if let Some(comment) = identity.comment() {
            println!("  Comment: {}", comment);
        }

        if args.show_private {
            println!();
            println!("  WARNING: Private key displayed below!");
            println!("  Private Key: {}", identity.private_key_encoded()?);
        }
    }

    Ok(())
}

/// Execute identity export
async fn execute_export(args: ExportArgs, config: &spectre_core::config::Config) -> Result<()> {
    let identity = if let Some(name) = &args.name {
        spectre_core::comms::load_identity_by_name(name, config)?
    } else {
        spectre_core::comms::load_identity(config)?
    };

    let data = identity.export(args.include_private, args.format.into())?;

    if let Some(output_path) = &args.output {
        tokio::fs::write(output_path, &data).await?;
        println!("Identity exported to: {:?}", output_path);
    } else {
        // Print to stdout
        use std::io::Write;
        std::io::stdout().write_all(&data)?;
    }

    Ok(())
}

/// Execute identity import
async fn execute_import(args: ImportArgs, config: &spectre_core::config::Config) -> Result<()> {
    let data = tokio::fs::read(&args.input).await?;

    let mut identity = spectre_core::comms::Identity::import(&data)?;

    if let Some(name) = args.name {
        identity.set_name(&name);
    }

    identity.save(config)?;

    if args.activate {
        spectre_core::comms::set_active_identity(identity.id(), config)?;
    }

    println!("Identity imported successfully!");
    println!("  Name: {}", identity.name().unwrap_or("default"));
    println!("  ID: {}", identity.id());

    Ok(())
}

/// Execute identity list
async fn execute_list(config: &spectre_core::config::Config) -> Result<()> {
    let identities = spectre_core::comms::list_identities(config)?;
    let active = spectre_core::comms::get_active_identity_id(config).ok();

    if identities.is_empty() {
        println!("No identities found. Create one with 'spectre identity init'.");
        return Ok(());
    }

    println!("Identities:");
    println!();

    for id in identities {
        let marker = if Some(&id.id) == active.as_ref() {
            "*"
        } else {
            " "
        };

        println!(
            "{} {} ({})",
            marker,
            id.name.as_deref().unwrap_or("default"),
            &id.id[..16]
        );
    }

    println!();
    println!("* = active identity");

    Ok(())
}

/// Execute identity switch
async fn execute_switch(args: SwitchArgs, config: &spectre_core::config::Config) -> Result<()> {
    let identity = spectre_core::comms::load_identity_by_name(&args.name, config)?;

    spectre_core::comms::set_active_identity(identity.id(), config)?;

    println!("Switched to identity: {}", args.name);

    Ok(())
}

/// Execute identity delete
async fn execute_delete(args: DeleteArgs, config: &spectre_core::config::Config) -> Result<()> {
    if !args.force {
        print!(
            "Are you sure you want to delete identity '{}'? [y/N] ",
            args.name
        );
        use std::io::Write;
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            return Ok(());
        }
    }

    spectre_core::comms::delete_identity(&args.name, config)?;

    println!("Identity '{}' deleted.", args.name);

    Ok(())
}

impl From<ExportFormat> for spectre_core::comms::ExportFormat {
    fn from(format: ExportFormat) -> Self {
        match format {
            ExportFormat::Pem => Self::Pem,
            ExportFormat::Json => Self::Json,
            ExportFormat::Raw => Self::Raw,
            ExportFormat::Qr => Self::Qr,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_type_default() {
        let key_type = KeyType::default();
        assert!(matches!(key_type, KeyType::Ed25519));
    }

    #[test]
    fn test_export_format_default() {
        let format = ExportFormat::default();
        assert!(matches!(format, ExportFormat::Pem));
    }
}
