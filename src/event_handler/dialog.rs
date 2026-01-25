use std::any::Any;
use crate::app::dialog::TextInputDialog;
use crate::app::{Command, Data};
use crate::event_handler::{Focusable, Interactable};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

impl Interactable for TextInputDialog {
    fn handle(&mut self, key: &KeyEvent, data: Option<Data>) -> color_eyre::Result<Command> {
        if self.focused_child_mut().is_none() {
            match key.kind {
                KeyEventKind::Press => {
                    if let KeyCode::Esc = key.code {
                        return Ok(Command::PopDialog);
                    }
                    if let KeyCode::Tab = key.code {
                        if let Some(index) = self.hover_index {
                            self.hover_index = Some(
                                (index + 1usize) % (self.components.len() + self.containers.len()),
                            );
                        } else {
                            self.hover_index = Some(0);
                        }
                        return Ok(Command::None);
                    }
                    if let KeyCode::BackTab = key.code {
                        if let Some(index) = self.hover_index {
                            if index == 0 {
                                self.hover_index =
                                    Some((self.components.len() + self.containers.len()) - 1usize);
                            } else {
                                self.hover_index = Some(index - 1usize);
                            }
                        } else {
                            self.hover_index =
                                Some((self.components.len() + self.containers.len()) - 1usize);
                        }
                        return Ok(Command::None);
                    }
                    if let KeyCode::Enter = key.code {
                        if let Some(index) = self.hover_index {
                            return match index {
                                0 => {
                                    self.containers[0].set_focus(true);
                                    Ok(Command::None)
                                }
                                1 => {
                                    self.components[1].handle(key, None)
                                }
                                2 => {
                                    self.components[2].handle(key, None)
                                }
                                _ => Ok(Command::None),
                            }
                        }
                    }
                    Ok(Command::None)
                },
                _ => Ok(Command::None)
            }
        } else {
            self.focused_child_mut().unwrap().handle(key, None)
        }
    }
}