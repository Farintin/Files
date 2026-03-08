use std::fs;
use std::io;
use std::path::Path;

pub fn delete(path: &Path) -> io::Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
}

pub fn create_file(path: &Path) -> io::Result<()> {
    fs::File::create(path)?;
    Ok(())
}

pub fn create_dir(path: &Path) -> io::Result<()> {
    fs::create_dir(path)?;
    Ok(())
}

pub fn rename(from: &Path, to: &Path) -> io::Result<()> {
    fs::rename(from, to)
}

pub fn copy(from: &Path, to: &Path) -> io::Result<()> {
    if from.is_dir() {
        fs::create_dir_all(to)?;
        for entry in fs::read_dir(from)? {
            let entry = entry?;
            let src = entry.path();
            let dst = to.join(entry.file_name());
            copy(&src, &dst)?;
        }
        Ok(())
    } else {
        fs::copy(from, to)?;
        Ok(())
    }
}
