use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "flop")]
#[command(about = "flop recursively toggles debug printf statements in C/C++ code")]
#[command(long_about = "\
flop recursively toggles debug printf statements in C/C++ code

EXAMPLES:
    flop off                      Enable debug output in current directory
    flop on src/                  Disable debug output in src/
    flop off --dry-run src/       Preview changes without modifying files
    flop off --interactive src/   Interactively select statements
    flop delete --yes src/        Delete debug statements without confirmation

COMMON OPTIONS:
    -y, --yes          Skip confirmation prompt
    -a, --all          Detect all output functions, not just debug statements
    -i, --interactive  Interactive mode for selecting specific statements
    -d, --dry-run      Dry run mode - show what would be changed without modifying files
")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Uncomment debug printf statements (enable debug output)
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
        /// Dry run mode - show what would be changed without modifying files
        #[arg(short = 'd', long)]
        dry_run: bool,
    },
    /// Comment out debug printf statements (disable debug output)
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
        /// Dry run mode - show what would be changed without modifying files
        #[arg(short = 'd', long)]
        dry_run: bool,
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
        /// Dry run mode - show what would be changed without modifying files
        #[arg(short = 'd', long)]
        dry_run: bool,
    },
}
