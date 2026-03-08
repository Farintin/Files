use crossterm::event::{KeyCode, KeyEvent};
use files_core::filesystem::FileSystem;
use files_core::state::{AppState, Command};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputKind {
    Rename,
    CreateFile,
    CreateDirectory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Input(InputKind),
    ConfirmDelete,
}

pub struct TuiApp<F: FileSystem> {
    pub state: AppState<F>,
    pub mode: Mode,
    pub input_buffer: String,
    pub cursor_position: usize,
}

impl<F: FileSystem> TuiApp<F> {
    pub fn new(state: AppState<F>) -> Self {
        Self {
            state,
            mode: Mode::Normal,
            input_buffer: String::new(),
            cursor_position: 0,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
        match self.mode {
            // ========================
            // NORMAL MODE
            // ========================
            Mode::Normal => match key.code {
                KeyCode::Char('n') => {
                    self.input_buffer.clear();
                    self.cursor_position = 0;
                    self.mode = Mode::Input(InputKind::CreateFile)
                }

                KeyCode::Char('N') => {
                    self.input_buffer.clear();
                    self.cursor_position = 0;
                    self.mode = Mode::Input(InputKind::CreateDirectory);
                }
                KeyCode::Char('d') => {
                    self.mode = Mode::ConfirmDelete;
                }
                KeyCode::Char('r') => {
                    if let Some(entry) = self.state.cursor() {
                        self.input_buffer = entry.name.clone();

                        // Place cursor before extension (if file)
                        if entry.is_dir {
                            self.cursor_position = self.input_buffer.len();
                        } else {
                            self.cursor_position = match self.input_buffer.rfind('.') {
                                Some(pos) if pos > 0 => pos,
                                _ => self.input_buffer.len(),
                            };
                        }

                        self.mode = Mode::Input(InputKind::Rename);
                    }
                }

                KeyCode::Down => {
                    self.state.handle_command(Command::MoveCursorDown)?;
                }

                KeyCode::Up => {
                    self.state.handle_command(Command::MoveCursorUp)?;
                }

                KeyCode::Enter => {
                    self.state.handle_command(Command::Enter)?;
                }

                KeyCode::Backspace => {
                    self.state.handle_command(Command::GoUp)?;
                }

                KeyCode::Char('R') => {
                    self.state.handle_command(Command::Refresh)?;
                }

                _ => {}
            },

            // ========================
            // DELETE MODE
            // ========================
            Mode::ConfirmDelete => match key.code {
                KeyCode::Char('y') => {
                    self.state.handle_command(Command::Delete)?;
                    self.mode = Mode::Normal;
                }
                KeyCode::Char('n') | KeyCode::Esc => {
                    self.mode = Mode::Normal;
                }
                _ => {}
            },

            Mode::Input(kind) => match key.code {
                KeyCode::Esc => {
                    self.mode = Mode::Normal;
                    self.input_buffer.clear();
                    self.cursor_position = 0;
                }

                KeyCode::Enter => {
                    self.submit_input(kind)?;
                }

                KeyCode::Left => {
                    if self.cursor_position > 0 {
                        self.cursor_position -= 1;
                    }
                }

                KeyCode::Right => {
                    if self.cursor_position < self.input_buffer.len() {
                        self.cursor_position += 1;
                    }
                }

                KeyCode::Backspace => {
                    if self.cursor_position > 0 {
                        self.cursor_position -= 1;
                        self.input_buffer.remove(self.cursor_position);
                    }
                }

                KeyCode::Char(c) => {
                    self.input_buffer.insert(self.cursor_position, c);
                    self.cursor_position += 1;
                }

                _ => {}
            },
        }

        Ok(())
    }

    fn submit_input(&mut self, kind: InputKind) -> Result<(), Box<dyn std::error::Error>> {
        if self.input_buffer.trim().is_empty() {
            return Ok(());
        }

        match kind {
            InputKind::Rename => {
                self.state
                    .handle_command(Command::Rename(self.input_buffer.clone()))?;
            }

            InputKind::CreateFile => {
                self.state
                    .handle_command(Command::CreateFile(self.input_buffer.clone()))?;
            }

            InputKind::CreateDirectory => {
                self.state
                    .handle_command(Command::CreateDirectory(self.input_buffer.clone()))?;
            }
        }

        self.input_buffer.clear();
        self.cursor_position = 0;
        self.mode = Mode::Normal;

        Ok(())
    }
}
