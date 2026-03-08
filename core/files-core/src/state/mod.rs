use std::path::Path;
use std::path::PathBuf;

use crate::{errors::FilesError, filesystem::FileSystem, models::FileEntry};

pub use command::Command;

mod navigation;
mod selection;
mod sorting;

pub mod command;

#[derive(Debug)]
pub struct AppState<F: FileSystem> {
    current_directory: PathBuf,
    entries: Vec<FileEntry>,
    cursor_index: Option<usize>,
    fs: F,
}

impl<F: FileSystem> AppState<F> {
    /// Creates a new AppState for a given directory and its entries.
    pub fn new(current_directory: PathBuf, entries: Vec<FileEntry>, fs: F) -> Self {
        let cursor_index = if entries.is_empty() { None } else { Some(0) };

        Self {
            current_directory,
            entries,
            fs,
            cursor_index,
        }
    }

    pub fn current_directory(&self) -> &Path {
        &self.current_directory
    }

    pub fn entries(&self) -> &[FileEntry] {
        &self.entries
    }

    pub fn cursor_index(&self) -> Option<usize> {
        self.cursor_index
    }

    /// Returns the currently selected entry, if any.
    pub fn cursor(&self) -> Option<&FileEntry> {
        self.cursor_index.and_then(|i| self.entries.get(i))
    }

    pub(crate) fn refresh(&mut self) -> Result<(), FilesError> {
        let previous_selection = self.cursor().map(|e| e.name.clone());

        let mut entries = self.fs.read_directory(&self.current_directory)?;
        sorting::sort_entries(&mut entries);
        self.entries = entries;

        // Try to preserve selection if possible
        if let Some(name) = previous_selection {
            self.cursor_index = self.entries.iter().position(|e| e.name == name);
        }

        // If nothing selected and entries exist, select first
        if self.cursor_index.is_none() && !self.entries.is_empty() {
            self.cursor_index = Some(0);
        }

        Ok(())
    }

    fn rename_selected(&mut self, new_name: String) -> Result<(), FilesError> {
        let selected = match self.cursor() {
            Some(entry) => entry.clone(),
            None => return Ok(()),
        };

        if new_name.trim().is_empty() {
            return Ok(());
        }

        let mut new_path = selected.path.clone();
        new_path.set_file_name(&new_name);

        self.fs.rename(&selected.path, &new_path)?;

        // Read fresh entries
        let mut entries = self.fs.read_directory(&self.current_directory)?;
        sorting::sort_entries(&mut entries);
        self.entries = entries;

        // 🔥 Explicitly reselect renamed file
        self.cursor_index = self.entries.iter().position(|e| e.path == new_path);

        // Fallback if somehow not found
        if self.cursor_index.is_none() && !self.entries.is_empty() {
            self.cursor_index = Some(0);
        }

        Ok(())
    }

    pub fn delete_selected(&mut self) -> Result<(), FilesError> {
        let selected = match self.cursor() {
            Some(entry) => entry.clone(),
            None => return Ok(()),
        };

        self.fs.delete(&selected.path)?;

        self.refresh()?;

        if self.entries.is_empty() {
            self.cursor_index = None;
        } else if let Some(i) = self.cursor_index
            && i >= self.entries.len()
        {
            self.cursor_index = Some(self.entries.len() - 1);
        }

        Ok(())
    }

    pub fn create_file(&mut self, name: String) -> Result<(), FilesError> {
        let mut path = self.current_directory.clone();
        path.push(&name);

        self.fs.create_file(&path)?;

        self.refresh()?;
        Ok(())
    }

    pub fn create_directory(&mut self, name: String) -> Result<(), FilesError> {
        let mut path = self.current_directory.clone();
        path.push(&name);

        self.fs.create_dir(&path)?;

        self.refresh()?;
        Ok(())
    }
}

#[cfg(test)]
pub mod test_utils;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    use crate::state::test_utils::{MockFileSystem, mock_entries};

    #[test]
    fn initializes_with_selection() {
        let entries = mock_entries(3);
        let fs = MockFileSystem {
            entries: entries.clone(),
        };

        let state = AppState::new(PathBuf::from("/tmp"), entries, fs);

        assert_eq!(state.cursor().unwrap().name, "file0");
    }

    #[test]
    fn empty_entries_have_no_selection() {
        let fs = MockFileSystem { entries: vec![] };

        let state = AppState::new(PathBuf::from("/tmp"), vec![], fs);
        assert!(state.cursor().is_none());
    }

    #[test]
    fn refresh_preserves_selection_if_possible() {
        let initial_entries = mock_entries(3);

        let fs = MockFileSystem {
            entries: initial_entries.clone(),
        };

        let mut state = AppState::new(PathBuf::from("/tmp"), initial_entries, fs);

        state.select_next(); // select index 1
        let selected_name = state.cursor().unwrap().name.clone();

        state.refresh().unwrap();

        assert_eq!(state.cursor().unwrap().name, selected_name);
    }

    #[test]
    fn rename_selected_does_not_panic() {
        let entries = mock_entries(2);
        let fs = MockFileSystem {
            entries: entries.clone(),
        };

        let mut state = AppState::new(PathBuf::from("/tmp"), entries, fs);

        state.select_next();

        let result = state.handle_command(Command::Rename("new.txt".into()));

        assert!(result.is_ok());
    }

    #[test]
    fn rename_preserves_selection() {
        let entries = mock_entries(3);

        let fs = MockFileSystem {
            entries: entries.clone(),
        };

        let mut state = AppState::new(PathBuf::from("/tmp"), entries, fs);

        state.select_next(); // move to index 1

        state
            .handle_command(Command::Rename("renamed.txt".into()))
            .unwrap();

        // After refresh, selection should still exist
        assert!(state.cursor().is_some());
    }
}
