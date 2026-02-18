use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::path::PathBuf;
use std::{fs, io};

use color_eyre::eyre::Result;
use ratatui::backend::Backend;
use ratatui::prelude::CrosstermBackend;
use ratatui::style::Stylize;
use ratatui::Terminal;
use rusqlite::Connection;

mod app;
mod utils;
mod models;
mod theme;
mod services;
mod db;

use app::Application;
use crate::app::{draw, handle_key_events};
use crate::db::GlyphRepository;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let cli_result: (bool, Option<Connection>) = handle_cli(&args)?;
    if cli_result.0 {
        return Ok(());
    }

    // Init
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;
    // Main
    let mut app: Application = Application::new();
    if cli_result.1.is_some() {
        app = Application::from(cli_result.1.unwrap());
    }
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
            handle_key_events(&key, app);
        }
        if app.state.should_quit {
            break;
        }
    }
    Ok(true)
}

/// Handle CLI command, return whether should the program exit immediately right after execution.
fn handle_cli(args: &Vec<String>) -> Result<(bool, Option<Connection>)> {
    if args.len() == 1 {
        return Ok((false, None));
    }
    if let Some(arg) = args.get(1) {
        match arg.as_str() {
            "new" => {
                let new_glyph_name: String = args.get(2).unwrap_or(&String::from("untitled_glyph")).clone();
                GlyphRepository::init_glyph_db(&PathBuf::from(new_glyph_name+".glyph"))?;
                return Ok((true, None));
            }
            "delete" => {
                fs::remove_file(&args.get(2).unwrap())?;
                return Ok((true, None));
            }
            "open" => {
                let glyph_path: String = args.get(2).unwrap().clone();
                if !fs::exists(&glyph_path)? {
                    println!("Glyph does not exist: {}", glyph_path);
                    return Ok((true, None));
                };
                let connection = GlyphRepository::init_glyph_db(&PathBuf::from(glyph_path))?;
                return Ok((false, Some(connection)));
            }
            _ => {
                println!("Invalid Command\nAvailable commands: \n - new\n - delete")
            }
        }
    }
    Ok((true, None))
}