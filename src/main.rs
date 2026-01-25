use std::io;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};

use ratatui::backend::Backend;
use ratatui::prelude::CrosstermBackend;
use ratatui::style::Stylize;
use ratatui::Terminal;

mod app;
mod drawer;
mod event_handler;
mod utils;
mod focus_handler;
mod state;

use app::Application;
use drawer::draw;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Init
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;
    // Main
    let mut app: Application = Application::new();
    let result = run(&mut terminal, &mut app);
    // Restore
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    // Process the result
    match result {
        Ok(m) => println!("Navi Exit"),
        Err(m) => println!("Navi Exit with Error."),
    }
    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>, app: &mut Application) -> io::Result<bool> {
    loop {
        terminal.draw(|frame| draw(frame, app));
        if let Event::Key(key) = crossterm::event::read()? {
            event_handler::handle_key_events(&key, app);
        }
        if app.state.should_quit {
            break;
        }
    }
    Ok(true)
}
