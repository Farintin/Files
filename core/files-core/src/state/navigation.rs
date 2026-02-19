use super::*;

impl<F: FileSystem> AppState<F> {
    /// Enters the currently selected directory, if it is a directory.
    pub fn enter_selected_directory(&mut self) -> Result<(), FilesError> {
        let selected = match self.selected() {
            Some(entry) if entry.is_dir => entry,
            _ => return Ok(()), // Not a directory or nothing selected
        };

        self.current_directory = selected.path.clone();
        self.refresh()
    }

    /// Moves to the parent directory, if it exists.
    pub fn go_up(&mut self) -> Result<(), FilesError> {
        if let Some(parent) = self.current_directory.parent() {
            self.current_directory = parent.to_path_buf();
            self.refresh()?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    use crate::models::FileEntry;
    use crate::state::test_utils::MockFileSystem;

    #[test]
    fn go_up_moves_to_parent_directory() {
        let fs = MockFileSystem { entries: vec![] };

        let mut state = AppState::new(PathBuf::from("/tmp/dir1"), vec![], fs);

        state.go_up().unwrap();

        assert_eq!(state.current_directory, PathBuf::from("/tmp"));
    }

    #[test]
    fn enter_selected_directory_changes_path() {
        let entries = vec![FileEntry {
            name: "dir1".into(),
            path: PathBuf::from("/tmp/dir1"),
            is_dir: true,
        }];

        let fs = MockFileSystem { entries: vec![] };

        let mut state = AppState::new(PathBuf::from("/tmp"), entries, fs);

        state.enter_selected_directory().unwrap();

        assert_eq!(state.current_directory, PathBuf::from("/tmp/dir1"));
    }
}
