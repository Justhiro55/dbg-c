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
            debug,
            yes,
            preview,
        } => {
            let target_path = path.unwrap_or_else(|| PathBuf::from("."));
            let all = !debug;
            let interactive = !yes;
            process_path(&target_path, true, false, all, interactive, preview)?;
        }
        Commands::On {
            path,
            debug,
            yes,
            preview,
        } => {
            let target_path = path.unwrap_or_else(|| PathBuf::from("."));
            let all = !debug;
            let interactive = !yes;
            process_path(&target_path, false, false, all, interactive, preview)?;
        }
        Commands::Delete {
            path,
            debug,
            yes,
            preview,
        } => {
            let target_path = path.unwrap_or_else(|| PathBuf::from("."));
            let all = !debug;
            let interactive = !yes;
            process_path_delete(&target_path, false, all, interactive, preview)?;
        }
    }

    Ok(())
}
