use std::io;

use crate::app::App;
use crossterm::event::{KeyCode, KeyEventKind, KeyEvent};
use color_eyre::eyre::{Ok, Result};

pub fn handle_key_events(key: &KeyEvent, app: &mut App) -> Result<()> {
    match key.kind {
        KeyEventKind::Press=> {
            if let KeyCode::Char(code) = key.code {
                match code {
                    'q' => {
                        app.set_should_quit(true);
                        return Ok(())
                    },
                    _ => return Ok(())
                }
            }
            Ok(())
        },
        KeyEventKind::Release=> Ok(()),
        KeyEventKind::Repeat=> Ok(()),
    }
}