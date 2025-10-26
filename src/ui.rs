use anyhow::Result;
use console::Style;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use regex::Regex;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::types::Match;

pub fn display_matches(matches: &[Match]) {
    // Display all matches grouped by file
    println!("\nFound {} debug statement(s):\n", matches.len());

    // Group matches by file
    let mut files_map: HashMap<PathBuf, Vec<&Match>> = HashMap::new();

    for m in matches {
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
}

pub fn select_statements_interactive(matches: &[Match]) -> Result<Vec<Match>> {
    println!("\nFound {} debug statement(s)\n", matches.len());

    // Create display items for selection
    let items: Vec<String> = matches
        .iter()
        .map(|m| {
            let file_style = Style::new().magenta();
            let line_style = Style::new().green();
            format!(
                "{} {}:{}",
                file_style.apply_to(m.file_path.display()),
                line_style.apply_to(m.line_number),
                m.line_content.trim()
            )
        })
        .collect();

    println!(
        "Use arrow keys to navigate, Space to toggle, Enter to confirm, Ctrl-C or q to cancel\n"
    );

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .items(&items)
        .interact_opt()?;

    if let Some(selected_indices) = selections {
        let selected: Vec<Match> = selected_indices
            .iter()
            .map(|&i| matches[i].clone())
            .collect();
        Ok(selected)
    } else {
        // User cancelled (Ctrl-C or Esc)
        println!("\nOperation cancelled.");
        Ok(vec![])
    }
}

fn highlight_debug_keyword(line: &str) -> String {
    // Highlight "debug" or "DEBUG" keywords in red
    let re = Regex::new(r"(debug|DEBUG)").unwrap();
    re.replace_all(line, "\x1b[1;31m$1\x1b[0m").to_string()
}
