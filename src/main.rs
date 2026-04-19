use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    prelude::{Backend, CrosstermBackend},
};
use std::io;
use ui::ui;

use crate::app::CurrentScreen;

mod app;
mod ui;

fn main() -> io::Result<()> {
    // setup terminal
    enable_raw_mode()?;

    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res.map(|_| ())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool>
where
    io::Error: From<B::Error>,
{
    app.reset_words();

    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != event::KeyEventKind::Press {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => {
                    if key.modifiers == event::KeyModifiers::ALT {
                        match key.code {
                            KeyCode::Backspace => app.delete_word(),
                            _ => {}
                        }
                        continue;
                    }

                    match key.code {
                        KeyCode::Esc => app.exit(),
                        KeyCode::Backspace => app.delete_char(),
                        KeyCode::Char(' ') => app.handle_space_press(),
                        KeyCode::Char(char) => app.enter_char(char),
                        _ => {}
                    }
                }
                CurrentScreen::Exit => return Ok(true),
            }
        }
    }
}
