use super::AppState;
use crate::{errors::FilesError, filesystem::FileSystem};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    MoveCursorDown,
    MoveCursorUp,
    Enter,
    GoUp,
    Refresh,
    Rename(String),
    Delete,
    CreateFile(String),
    CreateDirectory(String),
}

impl<F: FileSystem> AppState<F> {
    pub fn handle_command(&mut self, command: Command) -> Result<(), FilesError> {
        match command {
            Command::MoveCursorDown => {
                self.select_next();
                Ok(())
            }
            Command::MoveCursorUp => {
                self.select_previous();
                Ok(())
            }
            Command::Enter => self.enter_selected_directory(),
            Command::GoUp => self.go_up(),
            Command::Refresh => self.refresh(),
            Command::Rename(new_name) => self.rename_selected(new_name),
            Command::Delete => {
                self.delete_selected()?;
                Ok(())
            }
            Command::CreateFile(name) => self.create_file(name),
            Command::CreateDirectory(name) => self.create_directory(name),
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

        state.handle_command(Command::MoveCursorDown).unwrap();

        assert_eq!(state.cursor().unwrap().name, "file1");
    }
}
