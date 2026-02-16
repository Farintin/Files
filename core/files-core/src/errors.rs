use std::fmt;
use std::io;

#[derive(Debug)]
pub enum FilesError {
    Io(io::Error),
    InvalidPath,
}

impl fmt::Display for FilesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FilesError::Io(err) => write!(f, "IO error: {}", err),
            FilesError::InvalidPath => write!(f, "Invalid path provided"),
        }
    }
}

impl From<io::Error> for FilesError {
    fn from(err: io::Error) -> Self {
        FilesError::Io(err)
    }
}