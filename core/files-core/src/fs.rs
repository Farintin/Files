use std::fs;
use std::path::Path;

use crate::errors::FilesError;
use crate::models::FileEntry;

/// Reads a directory and returns a sorted list of `FileEntry` objects.
///
/// # Behavior
/// - Returns `FilesError::InvalidPath` if the path is not a directory.
/// - Propagates IO errors using `FilesError`.
/// - Sorts entries with directories first, then files,
///   both in case-insensitive alphabetical order.
pub fn read_directory(path: &Path) -> Result<Vec<FileEntry>, FilesError> {
    // Ensure the provided path is a directory, not just an existing file
    if !path.is_dir() {
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

    // Directories first, then alphabetical (case-insensitive)
    entries.sort_by(|a, b| {
        b.is_dir
            .cmp(&a.is_dir)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn invalid_path_returns_error() {
        let result = read_directory(Path::new("non_existent_path"));
        assert!(matches!(result, Err(FilesError::InvalidPath)));
    }
}
