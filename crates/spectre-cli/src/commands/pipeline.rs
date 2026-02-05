//! Pipeline management CLI commands

use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Subcommand};
use colored::Colorize;
use spectre_core::pipeline::{Pipeline, PipelineBuilder, PipelineData};

/// Pipeline management arguments
#[derive(Args, Debug)]
pub struct PipelineArgs {
    #[command(subcommand)]
    pub command: PipelineCommands,
}

/// Pipeline subcommands
#[derive(Subcommand, Debug)]
pub enum PipelineCommands {
    /// Run a pipeline with input data
    Run {
        /// Pipeline name
        name: String,

        /// Input data (text)
        #[arg(short, long)]
        input: Option<String>,

        /// Input file
        #[arg(short = 'f', long)]
        input_file: Option<PathBuf>,

        /// Continue on stage errors
        #[arg(long)]
        continue_on_error: bool,
    },

    /// List available pipeline templates
    List,

    /// Show pipeline details
    Show {
        /// Pipeline name
        name: String,
    },
}

/// Execute pipeline commands
pub async fn execute(args: PipelineArgs, _config_path: Option<PathBuf>) -> Result<()> {
    match args.command {
        PipelineCommands::Run {
            name,
            input,
            input_file,
            continue_on_error,
        } => {
            // Get input data
            let data = if let Some(text) = input {
                PipelineData::Text(text)
            } else if let Some(path) = input_file {
                let content = std::fs::read_to_string(&path)?;
                PipelineData::Text(content)
            } else {
                PipelineData::Empty
            };

            // Build pipeline based on name
            let mut pipeline = build_pipeline(&name, continue_on_error)?;

            println!("{} Running pipeline '{}'...", ">>".cyan().bold(), name);

            let result = pipeline.execute(data).await?;

            // Display results
            let metrics = pipeline.metrics();
            println!(
                "\n{} Pipeline complete in {:?}",
                "OK".green().bold(),
                metrics.total_duration
            );

            for stage_metric in &metrics.stage_metrics {
                println!(
                    "  Stage '{}': {} -> {} items ({:?})",
                    stage_metric.name,
                    stage_metric.items_in,
                    stage_metric.items_out,
                    stage_metric.duration
                );
            }

            // Output result
            let output = result.to_text();
            if !output.is_empty() {
                println!("\n{}", "Output:".bold());
                println!("{}", output);
            }
        },

        PipelineCommands::List => {
            println!("{}", "Available Pipeline Templates".bold());
            println!("{}", "-".repeat(50));
            println!("  {:<20} Pass data through unchanged", "passthrough");
            println!("  {:<20} Convert text to uppercase", "uppercase");
            println!("  {:<20} Passthrough + uppercase chain", "analyze");
            println!(
                "\n{}: Use 'spectre pipeline run <name>' to execute",
                "Hint".yellow()
            );
        },

        PipelineCommands::Show { name } => match name.as_str() {
            "passthrough" => {
                println!("{}: passthrough", "Pipeline".bold());
                println!("  Stages: 1");
                println!("  1. passthrough - Pass data through unchanged");
            },
            "uppercase" => {
                println!("{}: uppercase", "Pipeline".bold());
                println!("  Stages: 1");
                println!("  1. uppercase - Convert text to uppercase");
            },
            "analyze" => {
                println!("{}: analyze", "Pipeline".bold());
                println!("  Stages: 2");
                println!("  1. input - Passthrough stage");
                println!("  2. transform - Uppercase transform");
            },
            _ => {
                println!("{} Unknown pipeline: {}", "Error:".red().bold(), name);
            },
        },
    }

    Ok(())
}

/// Build a pipeline from a template name
fn build_pipeline(name: &str, continue_on_error: bool) -> Result<Pipeline> {
    let pipeline = match name {
        "passthrough" => PipelineBuilder::new("passthrough")
            .passthrough("passthrough")
            .continue_on_error(continue_on_error)
            .build(),
        "uppercase" => PipelineBuilder::new("uppercase")
            .uppercase("uppercase")
            .continue_on_error(continue_on_error)
            .build(),
        "analyze" => PipelineBuilder::new("analyze")
            .passthrough("input")
            .uppercase("transform")
            .continue_on_error(continue_on_error)
            .build(),
        _ => {
            anyhow::bail!("Unknown pipeline template: {}", name);
        },
    };

    Ok(pipeline)
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[derive(Parser, Debug)]
    struct TestCli {
        #[command(subcommand)]
        command: PipelineCommands,
    }

    #[test]
    fn test_run_command() {
        let cli = TestCli::try_parse_from(["test", "run", "uppercase", "-i", "hello"]).unwrap();
        assert!(matches!(cli.command, PipelineCommands::Run { name, .. } if name == "uppercase"));
    }

    #[test]
    fn test_list_command() {
        let cli = TestCli::try_parse_from(["test", "list"]).unwrap();
        assert!(matches!(cli.command, PipelineCommands::List));
    }

    #[test]
    fn test_show_command() {
        let cli = TestCli::try_parse_from(["test", "show", "analyze"]).unwrap();
        assert!(matches!(cli.command, PipelineCommands::Show { name } if name == "analyze"));
    }
}
