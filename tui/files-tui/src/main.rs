use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};

use files_core::{
    filesystem::{FileSystem, RealFileSystem},
    state::{AppState, Command},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize app state
    let fs = RealFileSystem;
    let cwd = std::env::current_dir()?;
    let entries = fs.read_directory(&cwd)?;
    let mut state = AppState::new(cwd, entries, fs);

    // Event loop
    loop {
        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1)])
                .split(size);

            let items: Vec<ListItem> = state
                .entries()
                .iter()
                .map(|e| {
                    let name = if e.is_dir {
                        format!("📁 {}", e.name)
                    } else {
                        format!("📄 {}", e.name)
                    };
                    ListItem::new(name)
                })
                .collect();

            let mut list_state = ListState::default();
            list_state.select(state.selected_index());

            let list = List::new(items)
                .block(
                    Block::default()
                        .title(state.current_directory().to_string_lossy().to_string())
                        .borders(Borders::ALL),
                )
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

            f.render_stateful_widget(list, chunks[0], &mut list_state);
        })?;

        // Input handling
        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => {
                        state.handle_command(Command::SelectNext)?;
                    }
                    KeyCode::Up => {
                        state.handle_command(Command::SelectPrevious)?;
                    }
                    KeyCode::Enter => {
                        state.handle_command(Command::Enter)?;
                    }
                    KeyCode::Backspace => {
                        state.handle_command(Command::GoUp)?;
                    }
                    _ => {}
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}