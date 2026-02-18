use std::path::PathBuf;

use crate::models::FileEntry;

#[derive(Debug)]
pub struct AppState {
    pub current_directory: PathBuf,
    pub entries: Vec<FileEntry>,
    pub selected_index: Option<usize>,
}

impl AppState {
    /// Creates a new AppState for a given directory and its entries.
    pub fn new(current_directory: PathBuf, entries: Vec<FileEntry>) -> Self {
        let selected_index = if entries.is_empty() { None } else { Some(0) };

        Self {
            current_directory,
            entries,
            selected_index,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::FileEntry;

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
        let state = AppState::new(PathBuf::from("/tmp"), entries);

        assert_eq!(state.selected_index, Some(0));
    }

    #[test]
    fn empty_entries_have_no_selection() {
        let state = AppState::new(PathBuf::from("/tmp"), vec![]);
        assert_eq!(state.selected_index, None);
    }

    #[test]
    fn selection_moves_forward_and_backward() {
        let entries = mock_entries(3);
        let mut state = AppState::new(PathBuf::from("/tmp"), entries);

        state.select_next();
        assert_eq!(state.selected_index, Some(1));

        state.select_previous();
        assert_eq!(state.selected_index, Some(0));
    }

    #[test]
    fn selection_does_not_overflow() {
        let entries = mock_entries(1);
        let mut state = AppState::new(PathBuf::from("/tmp"), entries);

        state.select_next();
        assert_eq!(state.selected_index, Some(0));

        state.select_previous();
        assert_eq!(state.selected_index, Some(0));
    }
}
