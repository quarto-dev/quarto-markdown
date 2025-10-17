use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod conversions;
mod utils;

use conversions::grid_tables::GridTableConverter;

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
    /// Convert grid tables to list-table format
    UngridTables {
        /// Input files (can be multiple files or glob patterns)
        #[arg(required = true)]
        files: Vec<PathBuf>,

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
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::UngridTables {
            files,
            in_place,
            check,
            verbose,
        } => {
            let converter = GridTableConverter::new()?;

            for file_path in files {
                if verbose {
                    println!("Processing: {}", file_path.display());
                }

                converter.process_file(&file_path, in_place, check, verbose)?;
            }

            Ok(())
        }
    }
}
