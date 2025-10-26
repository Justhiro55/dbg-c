use anyhow::Result;
use std::io::{self, Write};
use std::path::Path;

use crate::editor::{apply_changes, delete_changes};
use crate::finder::find_debug_printfs;
use crate::ui::{display_matches, select_statements_interactive};

pub fn process_path(
    path: &Path,
    uncomment: bool,
    skip_confirm: bool,
    detect_all: bool,
    interactive: bool,
) -> Result<()> {
    let matches = find_debug_printfs(path, uncomment, detect_all)?;

    if matches.is_empty() {
        println!("No matching debug statements found.");
        return Ok(());
    }

    // Interactive mode: let user select specific statements
    let selected_matches = if interactive {
        select_statements_interactive(&matches)?
    } else {
        // Non-interactive: display and confirm
        display_matches(&matches);

        // Ask for confirmation unless --yes flag is set
        if !skip_confirm {
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

            if input.trim().to_lowercase() != "y" {
                println!("\nOperation cancelled.");
                return Ok(());
            }
        }

        matches
    };

    if selected_matches.is_empty() {
        println!("\nNo statements selected.");
        return Ok(());
    }

    apply_changes(&selected_matches, uncomment)?;
    println!(
        "\nSuccessfully processed {} statement(s).",
        selected_matches.len()
    );

    Ok(())
}

pub fn process_path_delete(
    path: &Path,
    skip_confirm: bool,
    detect_all: bool,
    interactive: bool,
) -> Result<()> {
    // Find both commented and uncommented debug statements
    let uncommented_matches = find_debug_printfs(path, false, detect_all)?;
    let commented_matches = find_debug_printfs(path, true, detect_all)?;

    // Combine both lists
    let mut all_matches = uncommented_matches;
    all_matches.extend(commented_matches);

    if all_matches.is_empty() {
        println!("No matching debug statements found.");
        return Ok(());
    }

    // Interactive mode: let user select specific statements
    let selected_matches = if interactive {
        println!("Select statements to DELETE:");
        select_statements_interactive(&all_matches)?
    } else {
        // Non-interactive: display and confirm
        println!(
            "\nFound {} debug statement(s) to delete:\n",
            all_matches.len()
        );
        display_matches(&all_matches);

        // Ask for confirmation unless --yes flag is set
        if !skip_confirm {
            print!("Do you want to delete these statements? (y/n): ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            if input.trim().to_lowercase() != "y" {
                println!("\nOperation cancelled.");
                return Ok(());
            }
        }

        all_matches
    };

    if selected_matches.is_empty() {
        println!("\nNo statements selected.");
        return Ok(());
    }

    delete_changes(&selected_matches)?;
    println!(
        "\nSuccessfully deleted {} statement(s).",
        selected_matches.len()
    );

    Ok(())
}
