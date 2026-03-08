use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use files_core::{
    filesystem::{FileSystem, RealFileSystem},
    state::AppState,
};

mod app;
use app::{InputKind, Mode, TuiApp};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let fs = RealFileSystem;
    let cwd = std::env::current_dir()?;
    let entries = fs.read_directory(&cwd)?;
    let state = AppState::new(cwd, entries, fs);

    let mut app = TuiApp::new(state);

    loop {
        terminal.draw(|f| {
            let size = f.size();

            // ========================
            // LAYOUT (always 3 rows)
            // ========================
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(1),    // File list
                    Constraint::Length(3), // Rename input
                    Constraint::Length(1), // Status bar
                ])
                .split(size);

            // ========================
            // FILE LIST
            // ========================
            let items: Vec<ListItem> = app
                .state
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
            list_state.select(app.state.cursor_index());

            let list = List::new(items)
                .block(
                    Block::default()
                        .title(app.state.current_directory().to_string_lossy().to_string())
                        .borders(Borders::ALL),
                )
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

            f.render_stateful_widget(list, chunks[0], &mut list_state);

            // ========================
            // RENAME INPUT
            // ========================
            if let Mode::Input(kind) = app.mode {
                let label = match kind {
    InputKind::Rename => "Rename",
    InputKind::CreateFile => "New file",
    InputKind::CreateDirectory => "New directory",
};

                let input = Paragraph::new(format!("{}: {}", label, app.input_buffer))
                    .block(Block::default().borders(Borders::ALL));

                f.render_widget(input, chunks[1]);
                let label_len = match kind {
    InputKind::Rename => 8,
    InputKind::CreateFile => 10,
    InputKind::CreateDirectory => 15,
};

                // Cursor position
                let x = chunks[1].x + 1 + label_len + app.cursor_position as u16;
                let y = chunks[1].y + 1;

                f.set_cursor(x, y);
            }
            // ========================
            // DELETE CONFIRMATION
            // ========================
            if app.mode == Mode::ConfirmDelete
                && let Some(entry) = app.state.cursor()
            {
                let kind = if entry.is_dir { "directory" } else { "file" };
                let text = format!("Delete {} \"{}\"? (y/n)", kind, entry.name);

                let popup = Paragraph::new(text).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Confirm Delete"),
                );

                f.render_widget(popup, chunks[1]);
            }

            // ========================
            // STATUS BAR
            // ========================
            let total = app.state.entries().len();
            let current = app.state.cursor_index().map(|i| i + 1).unwrap_or(0);

            let status_text = match app.mode {
    Mode::Normal => format!(
        " NORMAL | {}/{} | r:rename n:new-file N:new-dir d:delete ↑↓:move Enter:open Backspace:up q:quit ",
        current, total
    ),

    Mode::Input(InputKind::Rename) => format!(
        " RENAME | {}/{} | type new name • Enter:confirm • Esc:cancel ",
        current, total
    ),

    Mode::Input(InputKind::CreateFile) => format!(
        " CREATE FILE | {}/{} | type file name • Enter:create • Esc:cancel ",
        current, total
    ),

    Mode::Input(InputKind::CreateDirectory) => format!(
        " CREATE DIR | {}/{} | type directory name • Enter:create • Esc:cancel ",
        current, total
    ),

    Mode::ConfirmDelete => format!(
        " DELETE | {}/{} | y:confirm • n/Esc:cancel ",
        current, total
    ),
};

            let status = Paragraph::new(status_text).style(
                Style::default()
                    .bg(Color::DarkGray)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            );

            f.render_widget(status, chunks[2]);
        })?;

        if event::poll(Duration::from_millis(200))?
            && let Event::Key(key) = event::read()?
        {
            if key.code == KeyCode::Char('q') && app.mode == Mode::Normal {
                break;
            }

            let previous_mode = app.mode;
            app.handle_key(key)?;

            if previous_mode != app.mode {
                match app.mode {
                    Mode::Input(_) => terminal.show_cursor()?,
                    Mode::Normal => terminal.hide_cursor()?,
                    Mode::ConfirmDelete => terminal.hide_cursor()?,
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
