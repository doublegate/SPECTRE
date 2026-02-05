//! CyberChef-MCP integration command
//!
//! This module provides the CLI interface for data transformation using `CyberChef` via MCP.

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use tracing::{debug, info, instrument};

use crate::output::{OutputFormat, OutputWriter};

/// Arguments for the chef command
#[derive(Args, Debug)]
#[command(
    about = "Data transformation and analysis via CyberChef-MCP v1.9.0",
    long_about = "Perform data transformation and analysis operations using CyberChef-MCP v1.9.0.\n\n\
                  Supports 463+ operations including encoding, encryption, compression, and more.\n\
                  Operations can be chained together using recipes.\n\
                  v1.9.0: Worker thread pool, streaming progress, configurable transport."
)]
pub struct ChefArgs {
    #[command(subcommand)]
    pub command: Option<ChefCommands>,

    /// Operation to perform (e.g., `From_Base64`, `To_Hex`)
    #[arg(value_name = "OPERATION")]
    pub operation: Option<String>,

    /// Input data as string
    #[arg(short = 'i', long, value_name = "DATA", conflicts_with = "file")]
    pub input: Option<String>,

    /// Input data from file
    #[arg(short = 'f', long, value_name = "FILE", conflicts_with = "input")]
    pub file: Option<PathBuf>,

    /// Recipe file (JSON format with chained operations)
    #[arg(short = 'r', long, value_name = "FILE")]
    pub recipe: Option<PathBuf>,

    /// Operation arguments (key=value format)
    #[arg(short = 'a', long = "arg", value_name = "KEY=VALUE")]
    pub args: Vec<String>,

    /// Output format
    #[arg(short = 'o', long, value_enum, default_value = "raw")]
    pub output: ChefOutputFormat,

    /// Output file path
    #[arg(long, value_name = "FILE")]
    pub output_file: Option<PathBuf>,

    /// Check CyberChef-MCP health
    #[arg(long)]
    pub health: bool,
}

/// Chef-specific output format
#[derive(Debug, Clone, Copy, Default, clap::ValueEnum)]
pub enum ChefOutputFormat {
    /// Raw output (default for data operations)
    #[default]
    Raw,
    /// Hexadecimal representation
    Hex,
    /// Base64 encoded
    Base64,
    /// JSON output with metadata
    Json,
}

/// Chef subcommands for management operations
#[derive(Subcommand, Debug)]
pub enum ChefCommands {
    /// Set up CyberChef-MCP container
    Setup(SetupArgs),

    /// List available operations
    List(ListArgs),

    /// Show operation details
    #[command(name = "info")]
    Info(HelpArgs),

    /// Manage recipes
    Recipe(RecipeArgs),

    /// Show worker thread pool statistics (v1.9.0)
    #[command(name = "worker-stats")]
    WorkerStats,
}

/// Arguments for setup subcommand
#[derive(Args, Debug)]
pub struct SetupArgs {
    /// Pull latest container image
    #[arg(long)]
    pub pull: bool,

    /// Start the container
    #[arg(long)]
    pub start: bool,

    /// Stop the container
    #[arg(long)]
    pub stop: bool,

    /// Remove the container
    #[arg(long)]
    pub remove: bool,

    /// Show container status
    #[arg(long)]
    pub status: bool,
}

/// Arguments for list subcommand
#[derive(Args, Debug)]
pub struct ListArgs {
    /// Filter operations by category
    #[arg(long)]
    pub category: Option<String>,

    /// Search operations by name
    #[arg(long)]
    pub search: Option<String>,

    /// Output format
    #[arg(short = 'o', long, value_enum, default_value = "table")]
    pub output: OutputFormat,
}

/// Arguments for help subcommand
#[derive(Args, Debug)]
pub struct HelpArgs {
    /// Operation name
    pub operation: String,
}

/// Arguments for recipe subcommand
#[derive(Args, Debug)]
pub struct RecipeArgs {
    #[command(subcommand)]
    pub command: RecipeCommands,
}

/// Recipe management subcommands
#[derive(Subcommand, Debug)]
pub enum RecipeCommands {
    /// Create a new recipe
    Create {
        /// Recipe name
        name: String,
        /// Operations to include (in order)
        #[arg(short, long = "op")]
        operations: Vec<String>,
    },

    /// List saved recipes
    List,

    /// Show recipe contents
    Show {
        /// Recipe name or file path
        name: String,
    },

    /// Delete a recipe
    Delete {
        /// Recipe name
        name: String,
    },
}

/// Execute the chef command
#[instrument(skip(args, config_path))]
pub async fn execute(args: ChefArgs, config_path: Option<PathBuf>) -> Result<()> {
    info!("Starting CyberChef-MCP operation");
    debug!(?config_path, "Configuration path");

    // Load configuration
    let config = spectre_core::config::load_config(config_path.as_deref())
        .context("Failed to load configuration")?;

    // Handle health check
    if args.health {
        return execute_health_check(&config).await;
    }

    // Handle subcommands
    if let Some(command) = args.command {
        return match command {
            ChefCommands::Setup(setup_args) => execute_setup(setup_args, &config).await,
            ChefCommands::List(list_args) => execute_list(list_args, &config).await,
            ChefCommands::Info(help_args) => execute_help(help_args, &config).await,
            ChefCommands::Recipe(recipe_args) => execute_recipe(recipe_args, &config).await,
            ChefCommands::WorkerStats => execute_worker_stats(&config).await,
        };
    }

    // Execute operation
    let operation = args
        .operation
        .expect("operation required when no subcommand");

    // Get input data
    let input = get_input(&args.input, args.file.as_deref())
        .await
        .context("Failed to read input")?;

    // Parse operation arguments
    let op_args = parse_operation_args(&args.args)?;

    // Check if using recipe
    if let Some(recipe_path) = &args.recipe {
        return execute_recipe_file(
            recipe_path,
            &input,
            &config,
            args.output,
            args.output_file.as_deref(),
        )
        .await;
    }

    // Create MCP client
    let chef = spectre_core::chef::create_chef_client(&config)
        .await
        .context("Failed to connect to CyberChef-MCP")?;

    // Execute operation
    let result = chef
        .execute(&operation, &input, &op_args)
        .await
        .context(format!("Operation '{operation}' failed"))?;

    // Output result
    output_result(&result, args.output, args.output_file.as_deref())?;

    info!("Operation completed successfully");
    Ok(())
}

/// Execute health check
async fn execute_health_check(config: &spectre_core::config::Config) -> Result<()> {
    let chef = spectre_core::chef::create_chef_client(config).await;

    match chef {
        Ok(client) => {
            let health = client.health_check().await?;
            println!(
                "CyberChef-MCP Status: {}",
                if health.healthy {
                    "Healthy"
                } else {
                    "Unhealthy"
                }
            );
            println!("  Container: {}", health.container_status);
            println!("  MCP Version: {}", health.mcp_version);
            println!("  Operations Available: {}", health.operation_count);
            Ok(())
        },
        Err(e) => {
            println!("CyberChef-MCP Status: Disconnected");
            println!("  Error: {e}");
            println!("\nRun 'spectre chef setup --start' to start the container.");
            Ok(())
        },
    }
}

/// Execute worker-stats subcommand (v1.9.0)
async fn execute_worker_stats(config: &spectre_core::config::Config) -> Result<()> {
    let chef = spectre_core::chef::create_chef_client(config)
        .await
        .context("Failed to connect to CyberChef-MCP")?;

    let stats = chef.worker_stats().await?;

    println!("Worker Thread Pool Statistics");
    println!("  Enabled: {}", stats.enabled);

    if stats.enabled {
        if let Some(threads) = stats.threads {
            println!("  Threads: {threads}");
        }
        if let Some(completed) = stats.completed {
            println!("  Completed Tasks: {completed}");
        }
        if let Some(waiting) = stats.waiting {
            println!("  Queued Tasks: {waiting}");
        }
        if let Some(utilization) = stats.utilization {
            println!("  Utilization: {:.1}%", utilization * 100.0);
        }
    } else if let Some(message) = &stats.message {
        println!("  {message}");
    }

    Ok(())
}

/// Execute setup subcommand
async fn execute_setup(args: SetupArgs, config: &spectre_core::config::Config) -> Result<()> {
    let docker = spectre_core::chef::DockerManager::new(config)
        .await
        .context("Failed to connect to Docker")?;

    if args.pull {
        println!("Pulling CyberChef-MCP image...");
        docker.pull_image().await?;
        println!("Image pulled successfully.");
    }

    if args.start {
        println!("Starting CyberChef-MCP container...");
        docker.start_container().await?;
        println!("Container started successfully.");
    }

    if args.stop {
        println!("Stopping CyberChef-MCP container...");
        docker.stop_container().await?;
        println!("Container stopped.");
    }

    if args.remove {
        println!("Removing CyberChef-MCP container...");
        docker.remove_container().await?;
        println!("Container removed.");
    }

    if args.status || (!args.pull && !args.start && !args.stop && !args.remove) {
        let status = docker.container_status().await?;
        println!("Container Status: {status}");
    }

    Ok(())
}

/// Execute list subcommand
async fn execute_list(args: ListArgs, config: &spectre_core::config::Config) -> Result<()> {
    let chef = spectre_core::chef::create_chef_client(config)
        .await
        .context("Failed to connect to CyberChef-MCP")?;

    let operations = chef
        .list_operations(args.category.as_deref(), args.search.as_deref())
        .await?;

    let mut writer = OutputWriter::new(args.output, None)?;
    writer.write_operations(&operations)?;

    Ok(())
}

/// Execute help subcommand
async fn execute_help(args: HelpArgs, config: &spectre_core::config::Config) -> Result<()> {
    let chef = spectre_core::chef::create_chef_client(config)
        .await
        .context("Failed to connect to CyberChef-MCP")?;

    let help = chef.operation_help(&args.operation).await?;

    println!("Operation: {}", help.name);
    println!("Category: {}", help.category);
    println!("\nDescription:");
    println!("  {}", help.description);

    if !help.args.is_empty() {
        println!("\nArguments:");
        for arg in &help.args {
            println!("  --{}: {} ({})", arg.name, arg.description, arg.arg_type);
            if let Some(default) = &arg.default {
                println!("      Default: {default}");
            }
        }
    }

    if !help.examples.is_empty() {
        println!("\nExamples:");
        for example in &help.examples {
            println!("  {example}");
        }
    }

    Ok(())
}

/// Execute recipe subcommand
async fn execute_recipe(args: RecipeArgs, config: &spectre_core::config::Config) -> Result<()> {
    match args.command {
        RecipeCommands::Create { name, operations } => {
            let recipe = spectre_core::chef::Recipe::new(&name, operations);
            recipe.save(config)?;
            println!("Recipe '{name}' created successfully.");
        },
        RecipeCommands::List => {
            let recipes = spectre_core::chef::Recipe::list(config)?;
            println!("Saved Recipes:");
            for recipe in recipes {
                println!("  - {recipe}");
            }
        },
        RecipeCommands::Show { name } => {
            let recipe = spectre_core::chef::Recipe::load(&name, config)?;
            println!("Recipe: {}", recipe.name);
            println!("Operations:");
            for (i, op) in recipe.operations.iter().enumerate() {
                println!("  {}. {}", i + 1, op);
            }
        },
        RecipeCommands::Delete { name } => {
            spectre_core::chef::Recipe::delete(&name, config)?;
            println!("Recipe '{name}' deleted.");
        },
    }

    Ok(())
}

/// Execute a recipe file
async fn execute_recipe_file(
    recipe_path: &std::path::Path,
    input: &[u8],
    config: &spectre_core::config::Config,
    output_format: ChefOutputFormat,
    output_file: Option<&std::path::Path>,
) -> Result<()> {
    let recipe = spectre_core::chef::Recipe::load_from_file(recipe_path)?;

    let chef = spectre_core::chef::create_chef_client(config)
        .await
        .context("Failed to connect to CyberChef-MCP")?;

    let result = chef.execute_recipe(&recipe, input).await?;

    output_result(&result, output_format, output_file)?;

    Ok(())
}

/// Get input data from string or file
async fn get_input(input: &Option<String>, file: Option<&std::path::Path>) -> Result<Vec<u8>> {
    if let Some(data) = input {
        Ok(data.as_bytes().to_vec())
    } else if let Some(path) = file {
        tokio::fs::read(path)
            .await
            .context(format!("Failed to read input file: {}", path.display()))
    } else {
        // Read from stdin
        use tokio::io::AsyncReadExt;
        let mut buffer = Vec::new();
        tokio::io::stdin().read_to_end(&mut buffer).await?;
        Ok(buffer)
    }
}

/// Parse operation arguments from key=value format
fn parse_operation_args(args: &[String]) -> Result<std::collections::HashMap<String, String>> {
    let mut map = std::collections::HashMap::new();

    for arg in args {
        let parts: Vec<&str> = arg.splitn(2, '=').collect();
        if parts.len() != 2 {
            anyhow::bail!("Invalid argument format '{arg}', expected key=value");
        }
        map.insert(parts[0].to_string(), parts[1].to_string());
    }

    Ok(map)
}

/// Output result in the specified format
#[allow(clippy::disallowed_methods)]
fn output_result(
    result: &[u8],
    format: ChefOutputFormat,
    output_file: Option<&std::path::Path>,
) -> Result<()> {
    let output = match format {
        ChefOutputFormat::Raw => result.to_vec(),
        ChefOutputFormat::Hex => hex::encode(result).into_bytes(),
        ChefOutputFormat::Base64 => {
            use base64::Engine;
            base64::engine::general_purpose::STANDARD
                .encode(result)
                .into_bytes()
        },
        ChefOutputFormat::Json => {
            let json = serde_json::json!({
                "result": String::from_utf8_lossy(result),
                "length": result.len(),
                "hex": hex::encode(result),
            });
            serde_json::to_vec_pretty(&json)?
        },
    };

    if let Some(path) = output_file {
        std::fs::write(path, &output)?;
    } else {
        use std::io::Write;
        std::io::stdout().write_all(&output)?;
        // Add newline if output doesn't end with one
        if !output.ends_with(b"\n") {
            println!();
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_operation_args() {
        let args = vec![
            "key1=value1".to_string(),
            "key2=value with spaces".to_string(),
        ];
        let result = parse_operation_args(&args).unwrap();

        assert_eq!(result.get("key1"), Some(&"value1".to_string()));
        assert_eq!(result.get("key2"), Some(&"value with spaces".to_string()));
    }

    #[test]
    fn test_parse_operation_args_invalid() {
        let args = vec!["invalid_format".to_string()];
        let result = parse_operation_args(&args);
        assert!(result.is_err());
    }
}
