use std::fs;
use std::path::Path;

use crate::errors::FilesError;
use crate::models::FileEntry;

pub fn read_directory(path: &Path) -> Result<Vec<FileEntry>, FilesError> {
    if !path.exists() {
        return Err(FilesError::InvalidPath);
    }

    let mut entries = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        entries.push(FileEntry {
            name: entry.file_name().to_string_lossy().into_owned(),
            path: entry.path(),
            is_dir: metadata.is_dir(),
        });
    }

    Ok(entries)
}