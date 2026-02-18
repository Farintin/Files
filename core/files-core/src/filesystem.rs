use std::path::Path;

use crate::{errors::FilesError, models::FileEntry};

pub trait FileSystem {
    fn read_directory(&self, path: &Path) -> Result<Vec<FileEntry>, FilesError>;
}

pub struct RealFileSystem;

impl FileSystem for RealFileSystem {
    fn read_directory(&self, path: &Path) -> Result<Vec<FileEntry>, FilesError> {
        crate::fs::read_directory(path)
    }
}
