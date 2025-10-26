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

    // Pattern to match C printf-like functions
    let c_functions_pattern = if detect_all {
        // Match all output functions regardless of content
        Regex::new(
            r"(printf|fprintf|sprintf|snprintf|printf_debug|dprintf|puts|fputs|fputc|putchar|fputchar|write|perror)\s*\([^;]*?;",
        )?
    } else {
        // Match only those with "debug" or "DEBUG"
        Regex::new(
            r"(printf|fprintf|sprintf|snprintf|printf_debug|dprintf|puts|fputs|fputc|putchar|fputchar|write|perror)\s*\([^;]*?(debug|DEBUG)[^;]*?;",
        )?
    };

    // Pattern to match C++ streams
    let cpp_stream_pattern = if detect_all {
        // Match all stream output regardless of content
        Regex::new(r"(std::cout|std::cerr|std::clog)\s*<<[^;]*?;")?
    } else {
        // Match only those with "debug" or "DEBUG"
        Regex::new(r"(std::cout|std::cerr|std::clog)\s*<<[^;]*?(debug|DEBUG)[^;]*?;")?
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
