use anyhow::{Context, Result};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

use crate::types::Match;

pub fn apply_changes(matches: &[Match], uncomment: bool) -> Result<()> {
    // Group matches by file
    let mut files_map: HashMap<PathBuf, Vec<&Match>> = HashMap::new();

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
            // For multiline statements, only comment/uncomment the first line
            // This will effectively disable/enable the entire statement
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

pub fn delete_changes(matches: &[Match]) -> Result<()> {
    // Group matches by file
    let mut files_map: HashMap<PathBuf, Vec<&Match>> = HashMap::new();

    for m in matches {
        files_map.entry(m.file_path.clone()).or_default().push(m);
    }

    for (file_path, file_matches) in files_map {
        let content = fs::read_to_string(&file_path)?;
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        // Sort by line number in reverse order to avoid index shifting
        let mut sorted_matches = file_matches;
        sorted_matches.sort_by(|a, b| b.line_number.cmp(&a.line_number));

        // Collect line numbers to delete (all lines from start to end of each statement)
        let mut lines_to_delete: HashSet<usize> = HashSet::new();
        for m in sorted_matches {
            for line_num in m.line_number..=m.end_line_number {
                lines_to_delete.insert(line_num - 1);
            }
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

fn comment_line(line: &str) -> String {
    // Find the first non-whitespace character and insert // before it
    let trimmed = line.trim_start();
    let leading_whitespace = &line[..line.len() - trimmed.len()];
    format!("{}// {}", leading_whitespace, trimmed)
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
