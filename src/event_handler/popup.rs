use crate::app::popup::{ConfirmPopup, MessagePopup};
use crate::app::AppCommand::*;
use crate::app::Command;
use crate::app::Command::AppCommand;
use crate::event_handler::Interactable;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use std::any::Any;
use color_eyre::eyre::Result;
impl Interactable for MessagePopup {
    fn handle(
        &mut self,
        key: &KeyEvent,
        parent_state: Option<&mut dyn Any>,
    ) -> color_eyre::Result<Vec<Command>> {
        return match key.kind {
            KeyEventKind::Press => {
                if let KeyCode::Enter = key.code {
                    return Ok(vec![AppCommand(PopPopup)]);
                }
                if let KeyCode::Esc = key.code {
                    return Ok(vec![AppCommand(PopPopup)]);
                }
                Ok(Vec::new())
            }
            _ => Ok(Vec::new()),
        };
    }
}
impl Interactable for ConfirmPopup {
    fn handle(
        &mut self,
        key: &KeyEvent,
        parent_state: Option<&mut dyn Any>,
    ) -> Result<Vec<Command>> {
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
                    if self.focus_index == 0 {
                        return Ok(vec![AppCommand(PopPopup)]);
                    }
                    if self.focus_index == 1 {
                        if let Some(mut function) = self.on_confirm.take() {
                            return (*function)(parent_state);
                        }
                        return Ok(Vec::new());
                    }
                }
                if let KeyCode::Esc = key.code {
                    return Ok(vec![AppCommand(PopPopup)]);
                }
                Ok(Vec::new())
            }
            _ => Ok(Vec::new()),
        };
    }
}
