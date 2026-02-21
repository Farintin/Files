use super::AppState;
use crate::{errors::FilesError, filesystem::FileSystem};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    SelectNext,
    SelectPrevious,
    Enter,
    GoUp,
    Refresh,
}

impl<F: FileSystem> AppState<F> {
    pub fn handle_command(&mut self, command: Command) -> Result<(), FilesError> {
        match command {
            Command::SelectNext => {
                self.select_next();
                Ok(())
            }
            Command::SelectPrevious => {
                self.select_previous();
                Ok(())
            }
            Command::Enter => self.enter_selected_directory(),
            Command::GoUp => self.go_up(),
            Command::Refresh => self.refresh(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    use crate::state::test_utils::{MockFileSystem, mock_entries};

    #[test]
    fn select_next_command_changes_selection() {
        let entries = mock_entries(2);

        let fs = MockFileSystem {
            entries: entries.clone(),
        };

        let mut state = AppState::new(PathBuf::from("/tmp"), entries, fs);

        state.handle_command(Command::SelectNext).unwrap();

        assert_eq!(state.selected().unwrap().name, "file1");
    }
}
