use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::types::Match;

pub fn find_debug_printfs(
    path: &Path,
    find_commented: bool,
    detect_all: bool,
) -> Result<Vec<Match>> {
    let mut matches = Vec::new();

    // Pattern to match C printf-like functions (multiline support with (?s))
    // Match from function name to closing paren, then optional whitespace and semicolon
    let c_functions_pattern = if detect_all {
        // Match all output functions regardless of content
        Regex::new(
            r"(?s)(printf|fprintf|sprintf|snprintf|printf_debug|dprintf|puts|fputs|fputc|putchar|fputchar|write|perror)\s*\([^)]*\)\s*;",
        )?
    } else {
        // Match only those with "debug" or "DEBUG"
        Regex::new(
            r"(?s)(printf|fprintf|sprintf|snprintf|printf_debug|dprintf|puts|fputs|fputc|putchar|fputchar|write|perror)\s*\([^)]*?(debug|DEBUG)[^)]*\)\s*;",
        )?
    };

    // Pattern to match C++ streams (multiline support with (?s))
    let cpp_stream_pattern = if detect_all {
        // Match all stream output regardless of content
        Regex::new(r"(?s)(std::cout|std::cerr|std::clog)\s*<<[^;]*?;")?
    } else {
        // Match only those with "debug" or "DEBUG"
        Regex::new(r"(?s)(std::cout|std::cerr|std::clog)\s*<<[^;]*?(debug|DEBUG)[^;]*?;")?
    };

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

        // Find all C-style function calls
        for cap in c_functions_pattern.find_iter(&content) {
            let match_str = cap.as_str();
            let start_offset = cap.start();
            let end_offset = cap.end();

            // Calculate line numbers from byte offsets
            // Count newlines before the start position, then add 1
            let line_number = content[..start_offset].matches('\n').count() + 1;
            // For end line, count newlines up to the end position
            let end_line_number = content[..end_offset].matches('\n').count() + 1;

            // Get the line content (for display purposes, we'll get the first line of the match)
            let line_start_offset = content[..start_offset]
                .rfind('\n')
                .map(|pos| pos + 1)
                .unwrap_or(0);
            let line_content = content[line_start_offset..]
                .lines()
                .next()
                .unwrap_or("")
                .to_string();

            // Check if commented (check the beginning of the statement)
            let is_commented = comment_pattern.is_match(&line_content);

            if is_commented == find_commented {
                // Extract original lines for multiline display
                let multiline_content: Vec<String> =
                    match_str.lines().map(|s| s.to_string()).collect();

                matches.push(Match {
                    file_path: file_path.clone(),
                    line_number,
                    end_line_number,
                    line_content: match_str.replace('\n', " ").trim().to_string(),
                    multiline_content,
                });
            }
        }

        // Find all C++ stream operations
        for cap in cpp_stream_pattern.find_iter(&content) {
            let match_str = cap.as_str();
            let start_offset = cap.start();
            let end_offset = cap.end();

            // Calculate line numbers from byte offsets
            // Count newlines before the start position, then add 1
            let line_number = content[..start_offset].matches('\n').count() + 1;
            // For end line, count newlines up to the end position
            let end_line_number = content[..end_offset].matches('\n').count() + 1;

            // Get the line content
            let line_start_offset = content[..start_offset]
                .rfind('\n')
                .map(|pos| pos + 1)
                .unwrap_or(0);
            let line_content = content[line_start_offset..]
                .lines()
                .next()
                .unwrap_or("")
                .to_string();

            // Check if commented
            let is_commented = comment_pattern.is_match(&line_content);

            if is_commented == find_commented {
                // Extract original lines for multiline display
                let multiline_content: Vec<String> =
                    match_str.lines().map(|s| s.to_string()).collect();

                matches.push(Match {
                    file_path: file_path.clone(),
                    line_number,
                    end_line_number,
                    line_content: match_str.replace('\n', " ").trim().to_string(),
                    multiline_content,
                });
            }
        }
    }

    Ok(matches)
}
