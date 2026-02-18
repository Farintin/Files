use std::path::PathBuf;

use crate::{errors::FilesError, filesystem::FileSystem, models::FileEntry};

#[derive(Debug)]
pub struct AppState<F: FileSystem> {
    pub current_directory: PathBuf,
    pub entries: Vec<FileEntry>,
    pub selected_index: Option<usize>,
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

    /// Returns the currently selected entry, if any.
    pub fn selected(&self) -> Option<&FileEntry> {
        self.selected_index
            .and_then(|index| self.entries.get(index))
    }

    /// Moves selection to the next entry.
    pub fn select_next(&mut self) {
        if let Some(index) = self.selected_index
            && index + 1 < self.entries.len()
        {
            self.selected_index = Some(index + 1);
        }
    }

    /// Moves selection to the previous entry.
    pub fn select_previous(&mut self) {
        if let Some(index) = self.selected_index
            && index > 0
        {
            self.selected_index = Some(index - 1);
        }
    }

    pub fn refresh(&mut self) -> Result<(), FilesError> {
        let previous_selection = self.selected().map(|e| e.name.clone());

        let entries = self.fs.read_directory(&self.current_directory)?;

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
mod tests {
    use super::*;

    struct MockFileSystem {
        entries: Vec<FileEntry>,
    }

    impl FileSystem for MockFileSystem {
        fn read_directory(&self, _path: &std::path::Path) -> Result<Vec<FileEntry>, FilesError> {
            Ok(self.entries.clone())
        }
    }

    fn mock_entries(count: usize) -> Vec<FileEntry> {
        (0..count)
            .map(|i| FileEntry {
                name: format!("file{}", i),
                path: PathBuf::from(format!("/tmp/file{}", i)),
                is_dir: false,
            })
            .collect()
    }

    #[test]
    fn initializes_with_selection() {
        let entries = mock_entries(3);
        let fs = MockFileSystem {
            entries: entries.clone(),
        };

        let state = AppState::new(PathBuf::from("/tmp"), entries, fs);

        assert_eq!(state.selected_index, Some(0));
    }

    #[test]
    fn empty_entries_have_no_selection() {
        let fs = MockFileSystem { entries: vec![] };

        let state = AppState::new(PathBuf::from("/tmp"), vec![], fs);
        assert_eq!(state.selected_index, None);
    }

    #[test]
    fn selection_moves_forward_and_backward() {
        let entries = mock_entries(3);
        let fs = MockFileSystem {
            entries: entries.clone(),
        };

        let mut state = AppState::new(PathBuf::from("/tmp"), entries, fs);

        state.select_next();
        assert_eq!(state.selected_index, Some(1));

        state.select_previous();
        assert_eq!(state.selected_index, Some(0));
    }

    #[test]
    fn selection_does_not_overflow() {
        let entries = mock_entries(1);
        let fs = MockFileSystem {
            entries: entries.clone(),
        };

        let mut state = AppState::new(PathBuf::from("/tmp"), entries, fs);

        state.select_next();
        assert_eq!(state.selected_index, Some(0));

        state.select_previous();
        assert_eq!(state.selected_index, Some(0));
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
