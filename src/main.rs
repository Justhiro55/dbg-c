use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use flop_cli::cli::{Cli, Commands};
use flop_cli::processor::{process_path, process_path_delete};

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
            let skip_confirm = yes;
            process_path(&target_path, true, skip_confirm, all, interactive, preview)?;
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
            let skip_confirm = yes;
            process_path(&target_path, false, skip_confirm, all, interactive, preview)?;
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
            let skip_confirm = yes;
            process_path_delete(&target_path, skip_confirm, all, interactive, preview)?;
        }
    }

    Ok(())
}
