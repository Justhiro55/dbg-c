use anyhow::Result;
use console::Style;
use dialoguer::{theme::Theme, MultiSelect};
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

use crate::types::Match;

// Custom theme with highlighted cursor position
struct HighlightedTheme;

impl Theme for HighlightedTheme {
    fn format_prompt(&self, f: &mut dyn fmt::Write, prompt: &str) -> fmt::Result {
        write!(f, "{}", Style::new().bold().apply_to(prompt))
    }

    fn format_multi_select_prompt_item(
        &self,
        f: &mut dyn fmt::Write,
        text: &str,
        checked: bool,
        active: bool,
    ) -> fmt::Result {
        let checkbox = if checked { "[x]" } else { "[ ]" };

        if active {
            // Active item: reverse video (inverted colors) with cyan background
            write!(
                f,
                "{} {}",
                Style::new().cyan().bold().apply_to(checkbox),
                Style::new().on_cyan().black().apply_to(text)
            )
        } else {
            // Inactive item: normal display
            write!(f, "{} {}", checkbox, text)
        }
    }
}

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

    // Sort matches by file path and line number for better grouping
    let mut sorted_matches: Vec<&Match> = matches.iter().collect();
    sorted_matches.sort_by(|a, b| {
        a.file_path
            .cmp(&b.file_path)
            .then(a.line_number.cmp(&b.line_number))
    });

    // Create display items for selection
    // Show full path with each line to avoid checkbox on filename-only rows
    let items: Vec<String> = sorted_matches
        .iter()
        .map(|m| {
            let file_style = Style::new().magenta();
            let line_style = Style::new().green();

            format!(
                "{}:{}  {}",
                file_style.apply_to(m.file_path.display()),
                line_style.apply_to(m.line_number),
                m.line_content.trim()
            )
        })
        .collect();

    // Default: select all statements
    let defaults = vec![true; sorted_matches.len()];

    let selections = MultiSelect::with_theme(&HighlightedTheme)
        .with_prompt("Select statements (Space: toggle, a: all, Enter: confirm, Esc: cancel)")
        .items(&items)
        .defaults(&defaults)
        .interact_opt()?;

    if let Some(selected_indices) = selections {
        let selected: Vec<Match> = selected_indices
            .iter()
            .map(|&i| sorted_matches[i].clone())
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
