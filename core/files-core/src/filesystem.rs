use std::fs;
use std::io;
use std::path::Path;

use crate::{errors::FilesError, models::FileEntry};

pub trait FileSystem {
    fn read_directory(&self, path: &Path) -> Result<Vec<FileEntry>, FilesError>;

    fn rename(&self, from: &Path, to: &Path) -> Result<(), FilesError>;

    fn delete(&self, path: &Path) -> io::Result<()>;
}

pub struct RealFileSystem;

impl FileSystem for RealFileSystem {
    fn read_directory(&self, path: &Path) -> Result<Vec<FileEntry>, FilesError> {
        crate::fs::read_directory(path)
    }

    fn rename(&self, from: &Path, to: &Path) -> Result<(), FilesError> {
        std::fs::rename(from, to).map_err(FilesError::from)
    }

    fn delete(&self, path: &Path) -> io::Result<()> {
        if path.is_dir() {
            fs::remove_dir_all(path)
        } else {
            fs::remove_file(path)
        }
    }
}
