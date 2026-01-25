use crate::app::popup::{ExitConfirmPopup, MessagePopup};
use crate::app::Command;
use crate::event_handler::Interactable;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use std::any::Any;

impl Interactable for MessagePopup {
    fn handle(&mut self, key: &KeyEvent, data: Option<&mut dyn Any>) -> color_eyre::Result<Vec<Command>> {
        return match key.kind {
            KeyEventKind::Press => {
                if let KeyCode::Enter = key.code {
                    return Ok(vec![Command::PopPopup]);
                }
                if let KeyCode::Esc = key.code {
                    return Ok(vec![Command::PopPopup]);
                }
                Ok(Vec::new())
            }
            _ => Ok(Vec::new()),
        };
    }
}
impl Interactable for ExitConfirmPopup {
    fn handle(&mut self, key: &KeyEvent, data: Option<&mut dyn Any>) -> color_eyre::Result<Vec<Command>> {
        return match key.kind {
            KeyEventKind::Press => {
                if let KeyCode::Tab = key.code {
                    self.focus_index = (self.focus_index + 1) % 2;
                }
                if let KeyCode::BackTab = key.code {
                    if self.focus_index == 0 {
                        self.focus_index = 1;
                    } else {
                        self.focus_index -= 1;
                    }
                }
                if let KeyCode::Enter = key.code {
                    return if self.focus_index == 1 {
                        Ok(vec![Command::Quit])
                    } else {
                        Ok(vec![Command::PopPopup])
                    };
                }
                if let KeyCode::Esc = key.code {
                    return Ok(vec![Command::PopPopup]);
                }
                Ok(Vec::new())
            }
            _ => Ok(Vec::new()),
        };
    }
}
