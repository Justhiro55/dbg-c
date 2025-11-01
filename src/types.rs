use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Match {
    pub file_path: PathBuf,
    pub line_number: usize,     // Start line number
    pub end_line_number: usize, // End line number (same as line_number for single-line statements)
    pub line_content: String,
}
