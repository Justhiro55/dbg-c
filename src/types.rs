use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Match {
    pub file_path: PathBuf,
    pub line_number: usize,
    pub line_content: String,
}
