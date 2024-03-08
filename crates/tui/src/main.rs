mod app;
mod cli;
mod components;
mod events;

use std::io;
use std::panic;

use app::App;
use events::{Event, EventHandler};
use glm::{FileManager, ListState};

use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> anyhow::Result<()> {
    let path = crate::cli::parse();
    let file_manager = FileManager::<ListState>::new(path)?;

    setup_terminal()?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut app = App::new(file_manager, terminal.size()?);

    terminal.clear()?;

    while app.is_running {
        terminal.draw(|f| {
            if app.draw(f).is_err() {
                std::process::exit(1);
            }
        })?;
        terminal.show_cursor()?;
        match events.next()? {
            Event::Key(event) => app.handle_key_event(event)?,
            Event::Tick => app.tick()?,
        }
    }

    reset_terminal()?;

    Ok(())
}

fn setup_terminal() -> anyhow::Result<()> {
    enable_raw_mode()?;
    crossterm::execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic| {
        reset_terminal().expect("failed to reset the terminal on panic");
        panic_hook(panic);
    }));
    Ok(())
}

fn reset_terminal() -> anyhow::Result<()> {
    disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
