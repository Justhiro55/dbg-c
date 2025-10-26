use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use regex::Regex;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "dbgc")]
#[command(about = "dbgc recursively toggles debug printf statements in C/C++ code", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Comment out debug printf statements
    Off {
        /// Path to file or directory (defaults to current directory)
        path: Option<PathBuf>,
    },
    /// Uncomment debug printf statements
    On {
        /// Path to file or directory (defaults to current directory)
        path: Option<PathBuf>,
    },
    /// Delete debug printf statements
    Delete {
        /// Path to file or directory (defaults to current directory)
        path: Option<PathBuf>,
    },
}

#[derive(Debug)]
struct Match {
    file_path: PathBuf,
    line_number: usize,
    line_content: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Off { path } => {
            let target_path = path.unwrap_or_else(|| PathBuf::from("."));
            process_path(&target_path, false)?;
        }
        Commands::On { path } => {
            let target_path = path.unwrap_or_else(|| PathBuf::from("."));
            process_path(&target_path, true)?;
        }
        Commands::Delete { path } => {
            let target_path = path.unwrap_or_else(|| PathBuf::from("."));
            process_path_delete(&target_path)?;
        }
    }

    Ok(())
}

fn process_path(path: &Path, uncomment: bool) -> Result<()> {
    let matches = find_debug_printfs(path, uncomment)?;

    if matches.is_empty() {
        println!("No matching debug statements found.");
        return Ok(());
    }

    // Display all matches grouped by file
    println!("\nFound {} debug statement(s):\n", matches.len());

    // Group matches by file
    let mut files_map: std::collections::HashMap<PathBuf, Vec<&Match>> =
        std::collections::HashMap::new();

    for m in &matches {
        files_map.entry(m.file_path.clone()).or_default().push(m);
    }

    // Sort files by path for consistent display
    let mut sorted_files: Vec<_> = files_map.iter().collect();
    sorted_files.sort_by_key(|(path, _)| path.as_path());

    for (file_path, file_matches) in sorted_files {
        // Display filename in color (magenta like ripgrep)
        println!("\x1b[35m{}\x1b[0m", file_path.display());

        // Sort matches by line number
        let mut sorted_matches = file_matches.clone();
        sorted_matches.sort_by_key(|m| m.line_number);

        for m in sorted_matches {
            // Line number in green, followed by colon and content with highlighted debug keyword
            let highlighted = highlight_debug_keyword(&m.line_content);
            println!("\x1b[32m{}\x1b[0m:{}", m.line_number, highlighted.trim());
        }

        println!(); // Empty line between files
    }

    // Ask for confirmation
    print!(
        "Do you want to {} these statements? (y/n): ",
        if uncomment {
            "uncomment"
        } else {
            "comment out"
        }
    );
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if input.trim().to_lowercase() == "y" {
        apply_changes(&matches, uncomment)?;
        println!("\nSuccessfully processed {} statement(s).", matches.len());
    } else {
        println!("\nOperation cancelled.");
    }

    Ok(())
}

fn find_debug_printfs(path: &Path, find_commented: bool) -> Result<Vec<Match>> {
    let mut matches = Vec::new();

    // Pattern to match C printf-like functions with "debug" or "DEBUG" in the string
    let c_functions_pattern = Regex::new(
        r"(printf|fprintf|sprintf|snprintf|printf_debug|dprintf|puts|fputs|fputc|putchar|fputchar|write|perror)\s*\([^;]*?(debug|DEBUG)[^;]*?;",
    )?;

    // Pattern to match C++ streams with "debug" or "DEBUG"
    let cpp_stream_pattern =
        Regex::new(r"(std::cout|std::cerr|std::clog)\s*<<[^;]*?(debug|DEBUG)[^;]*?;")?;

    let comment_pattern = Regex::new(r"^\s*//")?;

    let entries: Vec<_> = if path.is_file() {
        vec![path.to_path_buf()]
    } else {
        WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().is_file()
                    && e.path()
                        .extension()
                        .and_then(|s| s.to_str())
                        .map(|ext| matches!(ext, "c" | "h" | "cpp" | "hpp" | "cc" | "cxx"))
                        .unwrap_or(false)
            })
            .map(|e| e.path().to_path_buf())
            .collect()
    };

    for file_path in entries {
        let content = fs::read_to_string(&file_path)
            .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

        for (line_num, line) in content.lines().enumerate() {
            let is_commented = comment_pattern.is_match(line);
            let has_debug_output =
                c_functions_pattern.is_match(line) || cpp_stream_pattern.is_match(line);

            // Add to matches if it matches our search criteria
            if has_debug_output && is_commented == find_commented {
                matches.push(Match {
                    file_path: file_path.clone(),
                    line_number: line_num + 1,
                    line_content: line.to_string(),
                });
            }
        }
    }

    Ok(matches)
}

fn apply_changes(matches: &[Match], uncomment: bool) -> Result<()> {
    // Group matches by file
    let mut files_map: std::collections::HashMap<PathBuf, Vec<&Match>> =
        std::collections::HashMap::new();

    for m in matches {
        files_map.entry(m.file_path.clone()).or_default().push(m);
    }

    for (file_path, file_matches) in files_map {
        let content = fs::read_to_string(&file_path)?;
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        // Sort by line number in reverse order to avoid index shifting
        let mut sorted_matches = file_matches;
        sorted_matches.sort_by(|a, b| b.line_number.cmp(&a.line_number));

        for m in sorted_matches {
            let idx = m.line_number - 1;
            if idx < lines.len() {
                if uncomment {
                    // Remove the comment
                    lines[idx] = uncomment_line(&lines[idx]);
                } else {
                    // Add comment
                    lines[idx] = comment_line(&lines[idx]);
                }
            }
        }

        let new_content = lines.join("\n") + "\n";
        fs::write(&file_path, new_content)
            .with_context(|| format!("Failed to write file: {}", file_path.display()))?;
    }

    Ok(())
}

fn comment_line(line: &str) -> String {
    // Find the first non-whitespace character and insert // before it
    let trimmed = line.trim_start();
    let leading_spaces = line.len() - trimmed.len();
    format!("{}// {}", " ".repeat(leading_spaces), trimmed)
}

fn uncomment_line(line: &str) -> String {
    // Remove the // comment marker
    let re = Regex::new(r"^(\s*)//\s*(.*)$").unwrap();
    if let Some(caps) = re.captures(line) {
        format!("{}{}", &caps[1], &caps[2])
    } else {
        line.to_string()
    }
}

fn highlight_debug_keyword(line: &str) -> String {
    // Highlight "debug" or "DEBUG" keywords in red
    let re = Regex::new(r"(debug|DEBUG)").unwrap();
    re.replace_all(line, "\x1b[1;31m$1\x1b[0m").to_string()
}

fn process_path_delete(path: &Path) -> Result<()> {
    // Find both commented and uncommented debug statements
    let uncommented_matches = find_debug_printfs(path, false)?;
    let commented_matches = find_debug_printfs(path, true)?;

    // Combine both lists
    let mut all_matches = uncommented_matches;
    all_matches.extend(commented_matches);

    if all_matches.is_empty() {
        println!("No matching debug statements found.");
        return Ok(());
    }

    // Display all matches grouped by file
    println!(
        "\nFound {} debug statement(s) to delete:\n",
        all_matches.len()
    );

    // Group matches by file
    let mut files_map: std::collections::HashMap<PathBuf, Vec<&Match>> =
        std::collections::HashMap::new();

    for m in &all_matches {
        files_map.entry(m.file_path.clone()).or_default().push(m);
    }

    // Sort files by path for consistent display
    let mut sorted_files: Vec<_> = files_map.iter().collect();
    sorted_files.sort_by_key(|(path, _)| path.as_path());

    for (file_path, file_matches) in sorted_files {
        // Display filename in color (magenta like ripgrep)
        println!("\x1b[35m{}\x1b[0m", file_path.display());

        // Sort matches by line number
        let mut sorted_matches = file_matches.clone();
        sorted_matches.sort_by_key(|m| m.line_number);

        for m in sorted_matches {
            // Line number in green, followed by colon and content with highlighted debug keyword
            let highlighted = highlight_debug_keyword(&m.line_content);
            println!("\x1b[32m{}\x1b[0m:{}", m.line_number, highlighted.trim());
        }

        println!(); // Empty line between files
    }

    // Ask for confirmation
    print!("Do you want to delete these statements? (y/n): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if input.trim().to_lowercase() == "y" {
        delete_changes(&all_matches)?;
        println!("\nSuccessfully deleted {} statement(s).", all_matches.len());
    } else {
        println!("\nOperation cancelled.");
    }

    Ok(())
}

fn delete_changes(matches: &[Match]) -> Result<()> {
    // Group matches by file
    let mut files_map: std::collections::HashMap<PathBuf, Vec<&Match>> =
        std::collections::HashMap::new();

    for m in matches {
        files_map.entry(m.file_path.clone()).or_default().push(m);
    }

    for (file_path, file_matches) in files_map {
        let content = fs::read_to_string(&file_path)?;
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        // Sort by line number in reverse order to avoid index shifting
        let mut sorted_matches = file_matches;
        sorted_matches.sort_by(|a, b| b.line_number.cmp(&a.line_number));

        // Collect line numbers to delete
        let mut lines_to_delete: std::collections::HashSet<usize> =
            std::collections::HashSet::new();
        for m in sorted_matches {
            lines_to_delete.insert(m.line_number - 1);
        }

        // Filter out lines to delete
        let new_lines: Vec<String> = lines
            .into_iter()
            .enumerate()
            .filter(|(idx, _)| !lines_to_delete.contains(idx))
            .map(|(_, line)| line)
            .collect();

        let new_content = new_lines.join("\n") + "\n";
        fs::write(&file_path, new_content)
            .with_context(|| format!("Failed to write file: {}", file_path.display()))?;
    }

    Ok(())
}
