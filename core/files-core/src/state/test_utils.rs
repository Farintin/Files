use std::path::Path;

use crate::{errors::FilesError, filesystem::FileSystem, models::FileEntry};

#[derive(Clone)]
pub struct MockFileSystem {
    pub entries: Vec<FileEntry>,
}

impl FileSystem for MockFileSystem {
    fn read_directory(&self, _path: &Path) -> Result<Vec<FileEntry>, FilesError> {
        Ok(self.entries.clone())
    }
}

pub fn mock_entries(count: usize) -> Vec<FileEntry> {
    use std::path::PathBuf;

    (0..count)
        .map(|i| FileEntry {
            name: format!("file{}", i),
            path: PathBuf::from(format!("/tmp/file{}", i)),
            is_dir: false,
        })
        .collect()
}
