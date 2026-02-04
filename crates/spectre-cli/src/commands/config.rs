//! Configuration management command
//!
//! This module provides the CLI interface for configuration management.

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use tracing::{debug, info, instrument};

/// Arguments for the config command
#[derive(Args, Debug)]
#[command(
    about = "Configuration management",
    long_about = "Manage SPECTRE configuration files.\n\n\
                  Configuration is loaded from multiple sources with precedence:\n\
                  CLI args > env vars > project > user > system"
)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommands,
}

/// Config subcommands
#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Initialize a new configuration file
    Init(InitArgs),

    /// Show effective configuration
    Show(ShowArgs),

    /// Check configuration validity
    Check(CheckArgs),

    /// Get a specific configuration value
    Get(GetArgs),

    /// Set a configuration value
    Set(SetArgs),

    /// Show configuration file paths
    Paths,
}

/// Arguments for config init
#[derive(Args, Debug)]
pub struct InitArgs {
    /// Output path (default: ./spectre.toml)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Create user-level config instead of project
    #[arg(long)]
    pub user: bool,

    /// Overwrite existing config
    #[arg(long)]
    pub force: bool,

    /// Use minimal configuration
    #[arg(long)]
    pub minimal: bool,
}

/// Arguments for config show
#[derive(Args, Debug)]
pub struct ShowArgs {
    /// Output in JSON format
    #[arg(long)]
    pub json: bool,

    /// Show only specified section
    #[arg(short, long)]
    pub section: Option<String>,

    /// Show where each value was loaded from
    #[arg(long)]
    pub sources: bool,
}

/// Arguments for config check
#[derive(Args, Debug)]
pub struct CheckArgs {
    /// Configuration file to check (default: discovered config)
    #[arg(value_name = "FILE")]
    pub file: Option<PathBuf>,

    /// Show warnings as well as errors
    #[arg(short, long)]
    pub warnings: bool,
}

/// Arguments for config get
#[derive(Args, Debug)]
pub struct GetArgs {
    /// Configuration key (dot notation, e.g., scan.default_ports)
    pub key: String,
}

/// Arguments for config set
#[derive(Args, Debug)]
pub struct SetArgs {
    /// Configuration key (dot notation)
    pub key: String,

    /// Value to set
    pub value: String,

    /// Target config file (default: project config)
    #[arg(short, long)]
    pub file: Option<PathBuf>,
}

/// Execute the config command
#[instrument(skip(args, config_path))]
pub async fn execute(args: ConfigArgs, config_path: Option<PathBuf>) -> Result<()> {
    debug!(?config_path, "Configuration path override");

    match args.command {
        ConfigCommands::Init(init_args) => execute_init(init_args).await,
        ConfigCommands::Show(show_args) => execute_show(show_args, config_path).await,
        ConfigCommands::Check(check_args) => execute_check(check_args, config_path).await,
        ConfigCommands::Get(get_args) => execute_get(get_args, config_path).await,
        ConfigCommands::Set(set_args) => execute_set(set_args).await,
        ConfigCommands::Paths => execute_paths().await,
    }
}

/// Execute config init
async fn execute_init(args: InitArgs) -> Result<()> {
    info!("Initializing configuration");

    let output_path = if args.user {
        spectre_core::config::user_config_path()
            .context("Could not determine user config directory")?
    } else {
        args.output.unwrap_or_else(|| PathBuf::from("spectre.toml"))
    };

    // Check if file exists
    if output_path.exists() && !args.force {
        anyhow::bail!(
            "Configuration file already exists: {:?}\nUse --force to overwrite.",
            output_path
        );
    }

    // Generate config content
    let content = if args.minimal {
        spectre_core::config::generate_minimal_config()
    } else {
        spectre_core::config::generate_default_config()
    };

    // Create parent directories if needed
    if let Some(parent) = output_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    // Write config
    tokio::fs::write(&output_path, content).await?;

    println!("Configuration file created: {:?}", output_path);

    if !args.minimal {
        println!();
        println!("Edit the file to customize settings, then run:");
        println!("  spectre config check");
    }

    Ok(())
}

/// Execute config show
async fn execute_show(args: ShowArgs, config_path: Option<PathBuf>) -> Result<()> {
    let config = spectre_core::config::load_config(config_path.as_deref())
        .context("Failed to load configuration")?;

    if args.json {
        let json = if let Some(section) = &args.section {
            config.section_as_json(section)?
        } else {
            config.to_json()?
        };
        println!("{}", serde_json::to_string_pretty(&json)?);
    } else {
        if let Some(section) = &args.section {
            let toml = config.section_as_toml(section)?;
            println!("{}", toml);
        } else {
            let toml = config.to_toml()?;
            println!("{}", toml);
        }

        if args.sources {
            println!();
            println!("Configuration Sources:");
            for source in config.sources() {
                println!("  - {:?}", source);
            }
        }
    }

    Ok(())
}

/// Execute config check
async fn execute_check(args: CheckArgs, config_path: Option<PathBuf>) -> Result<()> {
    let path = args.file.or(config_path);

    println!("Checking configuration...");
    println!();

    let result = spectre_core::config::validate_config(path.as_deref())?;

    if result.errors.is_empty() && (result.warnings.is_empty() || !args.warnings) {
        println!("Configuration is valid.");
        return Ok(());
    }

    // Show errors
    if !result.errors.is_empty() {
        println!("Errors:");
        for error in &result.errors {
            println!("  [ERROR] {}: {}", error.key, error.message);
        }
    }

    // Show warnings if requested
    if args.warnings && !result.warnings.is_empty() {
        println!();
        println!("Warnings:");
        for warning in &result.warnings {
            println!("  [WARN] {}: {}", warning.key, warning.message);
        }
    }

    if !result.errors.is_empty() {
        anyhow::bail!("Configuration has {} error(s)", result.errors.len());
    }

    Ok(())
}

/// Execute config get
async fn execute_get(args: GetArgs, config_path: Option<PathBuf>) -> Result<()> {
    let config = spectre_core::config::load_config(config_path.as_deref())
        .context("Failed to load configuration")?;

    let value = config.get(&args.key)?;

    println!("{}", value);

    Ok(())
}

/// Execute config set
async fn execute_set(args: SetArgs) -> Result<()> {
    let file_path = args.file.unwrap_or_else(|| PathBuf::from("spectre.toml"));

    if !file_path.exists() {
        anyhow::bail!(
            "Configuration file not found: {:?}\nRun 'spectre config init' first.",
            file_path
        );
    }

    // Load existing config
    let content = tokio::fs::read_to_string(&file_path).await?;
    let mut doc = content.parse::<toml_edit::DocumentMut>()?;

    // Set the value
    spectre_core::config::set_value(&mut doc, &args.key, &args.value)?;

    // Write back
    tokio::fs::write(&file_path, doc.to_string()).await?;

    println!("Set {} = {}", args.key, args.value);

    Ok(())
}

/// Execute config paths
async fn execute_paths() -> Result<()> {
    println!("Configuration File Paths (in order of precedence):");
    println!();

    // Project config
    let project_path = PathBuf::from("./spectre.toml");
    let project_exists = project_path.exists();
    println!(
        "  1. Project:  {:?} {}",
        project_path,
        if project_exists { "(found)" } else { "" }
    );

    // User config
    if let Some(user_path) = spectre_core::config::user_config_path() {
        let user_exists = user_path.exists();
        println!(
            "  2. User:     {:?} {}",
            user_path,
            if user_exists { "(found)" } else { "" }
        );
    } else {
        println!("  2. User:     <not available>");
    }

    // System config
    let system_path = spectre_core::config::system_config_path();
    let system_exists = system_path.exists();
    println!(
        "  3. System:   {:?} {}",
        system_path,
        if system_exists { "(found)" } else { "" }
    );

    println!();
    println!("Environment Variables:");
    println!("  SPECTRE_CONFIG - Override config file path");
    println!("  SPECTRE_*      - Override specific settings");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_commands_variants() {
        // Just ensure the enum variants exist and can be matched
        let commands = vec![ConfigCommands::Paths];

        for cmd in commands {
            match cmd {
                ConfigCommands::Init(_) => {},
                ConfigCommands::Show(_) => {},
                ConfigCommands::Check(_) => {},
                ConfigCommands::Get(_) => {},
                ConfigCommands::Set(_) => {},
                ConfigCommands::Paths => {},
            }
        }
    }
}
