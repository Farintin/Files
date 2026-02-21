use super::*;

impl<F: FileSystem> AppState<F> {
    /// Moves selection to the next entry.
    pub(crate) fn select_next(&mut self) {
        if let Some(index) = self.selected_index
            && index + 1 < self.entries.len()
        {
            self.selected_index = Some(index + 1);
        }
    }

    /// Moves selection to the previous entry.
    pub(crate) fn select_previous(&mut self) {
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
    use std::path::PathBuf;

    use crate::state::test_utils::{MockFileSystem, mock_entries};

    #[test]
    fn selection_moves_forward_and_backward() {
        let entries = mock_entries(3);
        let fs = MockFileSystem {
            entries: entries.clone(),
        };

        let mut state = AppState::new(PathBuf::from("/tmp"), entries, fs);

        state.select_next();
        assert_eq!(state.selected().unwrap().name, "file1");

        state.select_previous();
        assert_eq!(state.selected().unwrap().name, "file0");
    }

    #[test]
    fn selection_does_not_overflow() {
        let entries = mock_entries(1);
        let fs = MockFileSystem {
            entries: entries.clone(),
        };

        let mut state = AppState::new(PathBuf::from("/tmp"), entries, fs);

        state.select_next();
        assert_eq!(state.selected().unwrap().name, "file0");

        state.select_previous();
        assert_eq!(state.selected().unwrap().name, "file0");
    }
}
