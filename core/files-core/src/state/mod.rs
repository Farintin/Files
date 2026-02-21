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
    selected_index: Option<usize>,
    fs: F,
}

impl<F: FileSystem> AppState<F> {
    /// Creates a new AppState for a given directory and its entries.
    pub fn new(current_directory: PathBuf, entries: Vec<FileEntry>, fs: F) -> Self {
        let selected_index = if entries.is_empty() { None } else { Some(0) };

        Self {
            current_directory,
            entries,
            selected_index,
            fs,
        }
    }

    pub fn current_directory(&self) -> &Path {
        &self.current_directory
    }

    pub fn entries(&self) -> &[FileEntry] {
        &self.entries
    }

    /// Returns the currently selected entry, if any.
    pub fn selected(&self) -> Option<&FileEntry> {
        self.selected_index
            .and_then(|index| self.entries.get(index))
    }

    pub(crate) fn refresh(&mut self) -> Result<(), FilesError> {
        let previous_selection = self.selected().map(|e| e.name.clone());

        let mut entries = self.fs.read_directory(&self.current_directory)?;
        sorting::sort_entries(&mut entries);
        self.entries = entries;

        // Try to preserve selection if possible
        if let Some(name) = previous_selection {
            self.selected_index = self.entries.iter().position(|e| e.name == name);
        }

        // If nothing selected and entries exist, select first
        if self.selected_index.is_none() && !self.entries.is_empty() {
            self.selected_index = Some(0);
        }

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

        assert_eq!(state.selected().unwrap().name, "file0");
    }

    #[test]
    fn empty_entries_have_no_selection() {
        let fs = MockFileSystem { entries: vec![] };

        let state = AppState::new(PathBuf::from("/tmp"), vec![], fs);
        assert!(state.selected().is_none());
    }

    #[test]
    fn refresh_preserves_selection_if_possible() {
        let initial_entries = mock_entries(3);

        let fs = MockFileSystem {
            entries: initial_entries.clone(),
        };

        let mut state = AppState::new(PathBuf::from("/tmp"), initial_entries, fs);

        state.select_next(); // select index 1
        let selected_name = state.selected().unwrap().name.clone();

        state.refresh().unwrap();

        assert_eq!(state.selected().unwrap().name, selected_name);
    }
}
