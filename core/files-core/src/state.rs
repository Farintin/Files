use std::path::PathBuf;

use crate::models::FileEntry;

#[derive(Debug)]
pub struct AppState {
    pub current_directory: PathBuf,
    pub entries: Vec<FileEntry>,
}