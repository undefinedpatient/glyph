use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use crate::app::{Command, entrance::Entrance};
use crate::event_handler::Interactable;

impl Interactable for Entrance {
    fn handle(&mut self, key: &KeyEvent) -> Result<Command> {
        match key.kind {
            KeyEventKind::Press=> {
                if let KeyCode::Char(code) = key.code {
                    return match code {
                        'q' => {
                            Ok(Command::Quit)
                        },
                        _ => Ok(Command::None)
                    }
                }
            },
            _ => {}
        }
        Ok(Command::None)
    }
}
