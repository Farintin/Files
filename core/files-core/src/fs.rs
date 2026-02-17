use std::fs;             // Standard filesystem module
use std::path::Path;      // For handling cross-platform paths

use crate::errors::FilesError; // Custom error handling
use crate::models::FileEntry;  // A custom struct (likely holding name, path, is_dir)

/// Reads a directory and returns a sorted list of FileEntry objects.
///
/// Returns a `FilesError` if the path is invalid or if reading fails.
/// The result is sorted with directories first, then files, both alphabetically.
pub fn read_directory(path: &Path) -> Result<Vec<FileEntry>, FilesError> {
    if !path.exists() {
        return Err(FilesError::InvalidPath);
    }

    let mut entries = Vec::new();

    for entry in fs::read_dir(path).map_err(FilesError::Io)? {
        let entry = entry.map_err(FilesError::Io)?;
        let metadata = entry.metadata().map_err(FilesError::Io)?; // Gets info like "is this a folder?"

    entries.push(FileEntry {
        name: entry.file_name().to_string_lossy().into_owned(),
        path: entry.path(),
        is_dir: metadata.is_dir(),
    });
}
    // Sorting Logic: Directories first, then alphabetical case-insensitive
    entries.sort_by(|a, b| {
    match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,    // Directory comes before File. A is dir, B is file -> A comes first
        (false, true) => std::cmp::Ordering::Greater, // File comes after Directory. A is file, B is dir -> B comes first
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()), // Both Same type? Alphabetical, Sort A-Z
    }
});

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn invalid_path_returns_error() {
        // Attempt to read a path that definitely doesn't exist
        let result = read_directory(Path::new("non_existent_path_xy123"));
        
        // Assert an Err, specifically FilesError::InvalidPath
        assert!(result.is_err());
        match result {
            Err(FilesError::InvalidPath) => assert!(true),
            _ => panic!("Expected InvalidPath error"),
        }
    }
}