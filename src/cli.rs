use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "flop")]
#[command(about = "Interactively flip debug output statements")]
#[command(long_about = "\
Interactively flip debug output statements

EXAMPLES:
    flop on                       Disable all output (interactive)
    flop on -d                    Disable debug output only (interactive)
    flop on -y                    Disable all output (batch, with confirmation)
    flop on -dy                   Disable debug output only (batch)
    flop off -p                   Preview what would be enabled
    flop delete -d src/           Delete debug statements in src/ (interactive)

COMMON OPTIONS:
    -d, --debug    Only process output statements containing 'debug' keyword
    -y, --yes      Skip interactive selection (batch mode, confirmation still required)
    -p, --preview  Preview mode - show what would be changed without modifying files
")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Uncomment output statements (enable output)
    Off {
        /// Path to file or directory (defaults to current directory)
        path: Option<PathBuf>,
        /// Only process output statements containing 'debug' keyword
        #[arg(short, long)]
        debug: bool,
        /// Skip interactive selection (batch mode, confirmation still required)
        #[arg(short, long)]
        yes: bool,
        /// Preview mode - show what would be changed without modifying files
        #[arg(short, long)]
        preview: bool,
    },
    /// Comment out output statements (disable output)
    On {
        /// Path to file or directory (defaults to current directory)
        path: Option<PathBuf>,
        /// Only process output statements containing 'debug' keyword
        #[arg(short, long)]
        debug: bool,
        /// Skip interactive selection (batch mode, confirmation still required)
        #[arg(short, long)]
        yes: bool,
        /// Preview mode - show what would be changed without modifying files
        #[arg(short, long)]
        preview: bool,
    },
    /// Delete output statements
    Delete {
        /// Path to file or directory (defaults to current directory)
        path: Option<PathBuf>,
        /// Only process output statements containing 'debug' keyword
        #[arg(short, long)]
        debug: bool,
        /// Skip interactive selection (batch mode, confirmation still required)
        #[arg(short, long)]
        yes: bool,
        /// Preview mode - show what would be changed without modifying files
        #[arg(short, long)]
        preview: bool,
    },
}
