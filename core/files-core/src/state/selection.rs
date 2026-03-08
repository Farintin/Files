use super::*;

impl<F: FileSystem> AppState<F> {
    /// Moves selection to the next entry.
    pub(crate) fn select_next(&mut self) {
        if self.entries.is_empty() {
            self.cursor_index = None;
            return;
        }

        self.cursor_index = Some(match self.cursor_index {
            Some(i) if i + 1 < self.entries.len() => i + 1,
            _ => 0,
        });
    }

    /// Moves selection to the previous entry.
    pub(crate) fn select_previous(&mut self) {
        if self.entries.is_empty() {
            self.cursor_index = None;
            return;
        }

        self.cursor_index = Some(match self.cursor_index {
            Some(0) | None => self.entries.len() - 1,
            Some(i) => i - 1,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    use crate::state::test_utils::{MockFileSystem, mock_entries};

    fn make_state_with_n_entries(n: usize) -> AppState<MockFileSystem> {
        let entries = mock_entries(n);

        let fs = MockFileSystem {
            entries: entries.clone(),
        };

        AppState::new(PathBuf::from("/tmp"), entries, fs)
    }

    #[test]
    fn selection_moves_forward_and_backward() {
        let entries = mock_entries(3);
        let fs = MockFileSystem {
            entries: entries.clone(),
        };

        let mut state = AppState::new(PathBuf::from("/tmp"), entries, fs);

        state.select_next();
        assert_eq!(state.cursor().unwrap().name, "file1");

        state.select_previous();
        assert_eq!(state.cursor().unwrap().name, "file0");
    }

    #[test]
    fn single_entry_always_selects_itself() {
        let mut state = make_state_with_n_entries(1);

        state.select_next();
        assert_eq!(state.cursor_index, Some(0));

        state.select_previous();
        assert_eq!(state.cursor_index, Some(0));
    }

    #[test]
    fn select_next_wraps_to_top() {
        let mut state = make_state_with_n_entries(3);

        state.cursor_index = Some(2);
        state.select_next();

        assert_eq!(state.cursor_index, Some(0));
    }

    #[test]
    fn select_previous_wraps_to_bottom() {
        let mut state = make_state_with_n_entries(3);

        state.cursor_index = Some(0);
        state.select_previous();

        assert_eq!(state.cursor_index, Some(2));
    }

    #[test]
    fn select_next_empty_directory() {
        let mut state = make_state_with_n_entries(0);

        state.select_next();

        assert_eq!(state.cursor_index, None);
    }
}
