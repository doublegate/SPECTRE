//! Shell completion generation command
//!
//! This module provides the CLI interface for generating shell completions.

use anyhow::Result;
use clap::Args;
use clap_complete::Shell;

/// Arguments for the completions command
#[derive(Args, Debug)]
#[command(
    about = "Generate shell completions",
    long_about = "Generate shell completion scripts for various shells.\n\n\
                  After generating, add the output to your shell's configuration."
)]
pub struct CompletionsArgs {
    /// Target shell
    #[arg(value_enum)]
    pub shell: Shell,

    /// Output file (default: stdout)
    #[arg(short, long)]
    pub output: Option<std::path::PathBuf>,
}

/// Execute the completions command
pub fn execute(args: CompletionsArgs) -> Result<()> {
    let completions = super::generate_completions(args.shell);

    if let Some(output_path) = args.output {
        std::fs::write(&output_path, &completions)?;
        println!("Completions written to: {:?}", output_path);
        println!();
        print_install_instructions(args.shell, Some(&output_path));
    } else {
        println!("{}", completions);
        eprintln!();
        print_install_instructions(args.shell, None);
    }

    Ok(())
}

/// Print installation instructions for each shell
fn print_install_instructions(shell: Shell, output_path: Option<&std::path::Path>) {
    let file_ref = output_path
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "<output_file>".to_string());

    eprintln!("Installation instructions for {}:", shell);
    eprintln!();

    match shell {
        Shell::Bash => {
            eprintln!("  # Add to ~/.bashrc:");
            eprintln!("  source {}", file_ref);
            eprintln!();
            eprintln!("  # Or install system-wide:");
            eprintln!("  sudo cp {} /etc/bash_completion.d/spectre", file_ref);
        },
        Shell::Zsh => {
            eprintln!("  # Add to ~/.zshrc (before compinit):");
            eprintln!("  fpath=(~/.zsh/completions $fpath)");
            eprintln!("  autoload -Uz compinit && compinit");
            eprintln!();
            eprintln!("  # Then copy completion file:");
            eprintln!("  mkdir -p ~/.zsh/completions");
            eprintln!("  cp {} ~/.zsh/completions/_spectre", file_ref);
        },
        Shell::Fish => {
            eprintln!("  # Copy to fish completions directory:");
            eprintln!("  cp {} ~/.config/fish/completions/spectre.fish", file_ref);
        },
        Shell::PowerShell => {
            eprintln!("  # Add to your PowerShell profile:");
            eprintln!("  . {}", file_ref);
            eprintln!();
            eprintln!("  # To find your profile location:");
            eprintln!("  echo $PROFILE");
        },
        Shell::Elvish => {
            eprintln!("  # Add to ~/.elvish/rc.elv:");
            eprintln!("  eval (cat {})", file_ref);
        },
        _ => {
            eprintln!("  See your shell's documentation for completion installation.");
        },
    }

    eprintln!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completions_generation() {
        for shell in [
            Shell::Bash,
            Shell::Zsh,
            Shell::Fish,
            Shell::PowerShell,
            Shell::Elvish,
        ] {
            let completions = super::super::generate_completions(shell);
            assert!(!completions.is_empty());
            assert!(completions.contains("spectre"));
        }
    }
}
