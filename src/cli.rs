use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "dbgc")]
#[command(about = "dbgc recursively toggles debug printf statements in C/C++ code", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Comment out debug printf statements
    Off {
        /// Path to file or directory (defaults to current directory)
        path: Option<PathBuf>,
        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
        /// Detect all output functions, not just debug statements
        #[arg(short, long)]
        all: bool,
        /// Interactive mode for selecting specific statements
        #[arg(short, long)]
        interactive: bool,
    },
    /// Uncomment debug printf statements
    On {
        /// Path to file or directory (defaults to current directory)
        path: Option<PathBuf>,
        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
        /// Detect all output functions, not just debug statements
        #[arg(short, long)]
        all: bool,
        /// Interactive mode for selecting specific statements
        #[arg(short, long)]
        interactive: bool,
    },
    /// Delete debug printf statements
    Delete {
        /// Path to file or directory (defaults to current directory)
        path: Option<PathBuf>,
        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
        /// Detect all output functions, not just debug statements
        #[arg(short, long)]
        all: bool,
        /// Interactive mode for selecting specific statements
        #[arg(short, long)]
        interactive: bool,
    },
}
