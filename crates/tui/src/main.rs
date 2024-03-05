mod app;
mod cli;
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
    let mut app = App::new(file_manager);

    terminal.clear()?;

    while app.is_running {
        match events.next()? {
            // TODO: probably not useful to quit on any event.
            Event::Key(_) => app.is_running = false,
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
