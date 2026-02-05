//! Plugin management CLI commands

use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Subcommand};
use colored::Colorize;
use spectre_core::plugin::{self, PluginSandbox};

/// Plugin management arguments
#[derive(Args, Debug)]
pub struct PluginArgs {
    #[command(subcommand)]
    pub command: PluginCommands,
}

/// Plugin subcommands
#[derive(Subcommand, Debug)]
pub enum PluginCommands {
    /// List installed plugins
    List {
        /// Plugin directory to scan
        #[arg(short, long)]
        dir: Option<PathBuf>,
    },

    /// Show plugin information
    Info {
        /// Plugin directory path
        path: PathBuf,
    },

    /// Run a plugin
    Run {
        /// Plugin directory path
        path: PathBuf,

        /// Additional arguments passed to the plugin
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
}

/// Execute plugin commands
pub async fn execute(args: PluginArgs, config_path: Option<PathBuf>) -> Result<()> {
    match args.command {
        PluginCommands::List { dir } => {
            let plugin_dir = dir.unwrap_or_else(|| get_plugin_dir(&config_path));

            println!("{} Scanning: {}", ">>".cyan().bold(), plugin_dir.display());

            let plugins = plugin::discover_plugins(&plugin_dir)?;

            if plugins.is_empty() {
                println!("{}", "No plugins found".yellow());
                return Ok(());
            }

            println!(
                "\n{:<25} {:<12} {:<40}",
                "Name".bold(),
                "Version".bold(),
                "Description".bold()
            );
            println!("{}", "-".repeat(77));

            for p in &plugins {
                println!(
                    "{:<25} {:<12} {:<40}",
                    p.manifest.name, p.manifest.version, p.manifest.description
                );
            }

            println!("\n{} plugin(s) found", plugins.len());
        },

        PluginCommands::Info { path } => {
            let p = plugin::Plugin::load(&path)?;

            println!("{}: {}", "Plugin".bold(), p.manifest.name);
            println!("  Version:     {}", p.manifest.version);
            println!("  Author:      {}", p.manifest.author);
            println!("  Description: {}", p.manifest.description);
            println!("  Main:        {}", p.manifest.main);
            println!("  Path:        {}", p.path.display());

            let perms = p.manifest.permissions.granted_permissions();
            if perms.is_empty() {
                println!("  Permissions: none");
            } else {
                println!("  Permissions:");
                for perm in &perms {
                    println!("    - {perm}");
                }
            }

            println!("  Limits:");
            println!(
                "    Memory:       {} MB",
                p.manifest.limits.max_memory / (1024 * 1024)
            );
            println!("    Timeout:      {}s", p.manifest.limits.timeout_seconds);
            println!("    Instructions: {}", p.manifest.limits.max_instructions);

            if !p.manifest.tags.is_empty() {
                println!("  Tags: {}", p.manifest.tags.join(", "));
            }
        },

        PluginCommands::Run {
            path,
            args: plugin_args,
        } => {
            let p = plugin::Plugin::load(&path)?;

            println!(
                "{} Running plugin: {} v{}",
                ">>".cyan().bold(),
                p.manifest.name,
                p.manifest.version
            );

            let sandbox =
                PluginSandbox::new(p.manifest.permissions.clone(), p.manifest.limits.clone())?;

            // Pass arguments as a global
            if !plugin_args.is_empty() {
                let args_str = plugin_args.join(" ");
                sandbox.set_global("ARGS", &args_str)?;
            }

            let result = sandbox.execute_file(&p.script_path)?;

            if result.success {
                println!("{} Plugin completed successfully", "OK".green().bold());
                if !result.output.is_empty() {
                    println!("\n{}", result.output);
                }
            } else {
                println!("{} Plugin failed", "ERROR".red().bold());
                if let Some(error) = result.error {
                    println!("  {error}");
                }
            }
        },
    }

    Ok(())
}

/// Get the default plugin directory
fn get_plugin_dir(config_path: &Option<PathBuf>) -> PathBuf {
    if let Some(ref config) = config_path {
        if let Some(parent) = config.parent() {
            return parent.join("plugins");
        }
    }

    directories::ProjectDirs::from("com", "spectre", "spectre").map_or_else(
        || PathBuf::from("plugins"),
        |dirs: directories::ProjectDirs| dirs.data_dir().join("plugins"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[derive(Parser, Debug)]
    struct TestCli {
        #[command(subcommand)]
        command: PluginCommands,
    }

    #[test]
    fn test_list_command() {
        let cli = TestCli::try_parse_from(["test", "list"]).unwrap();
        assert!(matches!(cli.command, PluginCommands::List { .. }));
    }

    #[test]
    fn test_info_command() {
        let cli = TestCli::try_parse_from(["test", "info", "/path/to/plugin"]).unwrap();
        assert!(matches!(cli.command, PluginCommands::Info { .. }));
    }

    #[test]
    fn test_run_command() {
        let cli = TestCli::try_parse_from(["test", "run", "/path/to/plugin"]).unwrap();
        assert!(matches!(cli.command, PluginCommands::Run { .. }));
    }
}
