use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use flop::cli::{Cli, Commands};
use flop::processor::{process_path, process_path_delete};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Off {
            path,
            yes,
            all,
            interactive,
            dry_run,
        } => {
            let target_path = path.unwrap_or_else(|| PathBuf::from("."));
            process_path(&target_path, true, yes, all, interactive, dry_run)?;
        }
        Commands::On {
            path,
            yes,
            all,
            interactive,
            dry_run,
        } => {
            let target_path = path.unwrap_or_else(|| PathBuf::from("."));
            process_path(&target_path, false, yes, all, interactive, dry_run)?;
        }
        Commands::Delete {
            path,
            yes,
            all,
            interactive,
            dry_run,
        } => {
            let target_path = path.unwrap_or_else(|| PathBuf::from("."));
            process_path_delete(&target_path, yes, all, interactive, dry_run)?;
        }
    }

    Ok(())
}
