use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::path::PathBuf;

mod conversions;
mod diagnostics;
mod rule;
mod utils;

use rule::{Rule, RuleRegistry};
use utils::glob_expand::expand_globs;

#[derive(Parser)]
#[command(name = "qmd-syntax-helper")]
#[command(about = "Helper tool for converting and fixing Quarto Markdown syntax")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check files for known problems
    Check {
        /// Input files (can be multiple files or glob patterns like "docs/**/*.qmd")
        #[arg(required = true)]
        files: Vec<String>,

        /// Rules to check (defaults to "all")
        #[arg(short = 'r', long = "rule", default_values_t = vec!["all".to_string()])]
        rule: Vec<String>,

        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,

        /// Output results as JSONL
        #[arg(long)]
        json: bool,

        /// Save detailed results to file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Convert/fix problems in files
    Convert {
        /// Input files (can be multiple files or glob patterns like "docs/**/*.qmd")
        #[arg(required = true)]
        files: Vec<String>,

        /// Rules to apply (defaults to "all")
        #[arg(short = 'r', long = "rule", default_values_t = vec!["all".to_string()])]
        rule: Vec<String>,

        /// Edit files in place
        #[arg(short, long)]
        in_place: bool,

        /// Check mode: show what would be changed without modifying files
        #[arg(short, long)]
        check: bool,

        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// List all available rules
    ListRules,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let registry = RuleRegistry::new()?;

    match cli.command {
        Commands::Check {
            files,
            rule: rule_names,
            verbose,
            json,
            output,
        } => {
            let file_paths = expand_globs(&files)?;
            let rules = resolve_rules(&registry, &rule_names)?;

            let mut all_results = Vec::new();

            for file_path in file_paths {
                if verbose && !json {
                    println!("Checking: {}", file_path.display());
                }

                for rule in &rules {
                    match rule.check(&file_path, verbose && !json) {
                        Ok(result) => {
                            all_results.push(result.clone());
                            if !json && result.has_issue {
                                println!("  {} {}", "✗".red(), result.message.unwrap_or_default());
                            }
                        }
                        Err(e) => {
                            if !json {
                                eprintln!("  {} Error checking {}: {}", "✗".red(), rule.name(), e);
                            }
                        }
                    }
                }
            }

            // Output handling
            if json {
                for result in &all_results {
                    println!("{}", serde_json::to_string(result)?);
                }
            }

            if let Some(output_path) = output {
                let mut output_str = String::new();
                for result in &all_results {
                    output_str.push_str(&serde_json::to_string(result)?);
                    output_str.push('\n');
                }
                std::fs::write(output_path, output_str)?;
            }

            Ok(())
        }

        Commands::Convert {
            files,
            rule: rule_names,
            in_place,
            check: check_mode,
            verbose,
        } => {
            let file_paths = expand_globs(&files)?;
            let rules = resolve_rules(&registry, &rule_names)?;

            for file_path in file_paths {
                if verbose {
                    println!("Processing: {}", file_path.display());
                }

                // Apply fixes sequentially, reparsing between each rule
                for rule in &rules {
                    match rule.convert(&file_path, in_place, check_mode, verbose) {
                        Ok(result) => {
                            if result.fixes_applied > 0 {
                                if verbose || check_mode {
                                    println!(
                                        "  {} {} - {}",
                                        if check_mode { "Would fix" } else { "Fixed" },
                                        rule.name(),
                                        result.message.clone().unwrap_or_default()
                                    );
                                }

                                if !in_place && !check_mode && result.message.is_some() {
                                    // Output to stdout if not in-place
                                    print!("{}", result.message.unwrap());
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("  {} Error converting {}: {}", "✗".red(), rule.name(), e);
                            // Stop on first error (transactional)
                            return Err(e);
                        }
                    }
                }
            }

            Ok(())
        }

        Commands::ListRules => {
            println!("{}", "Available rules:".bold());
            for name in registry.list_names() {
                let rule = registry.get(&name)?;
                println!("  {} - {}", name.cyan(), rule.description());
            }
            Ok(())
        }
    }
}

fn resolve_rules(
    registry: &RuleRegistry,
    names: &[String],
) -> Result<Vec<std::sync::Arc<dyn Rule + Send + Sync>>> {
    if names.len() == 1 && names[0] == "all" {
        Ok(registry.all())
    } else {
        let mut rules = Vec::new();
        for name in names {
            rules.push(registry.get(name)?);
        }
        Ok(rules)
    }
}
