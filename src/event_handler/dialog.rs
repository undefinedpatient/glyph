use crate::app::dialog::TextInputDialog;
use crate::app::Command;
use crate::event_handler::{Focusable, Interactable};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

impl Interactable for TextInputDialog {
    fn handle(&mut self, key: &KeyEvent) -> color_eyre::Result<Command> {
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
                            if index == 0 {
                                self.containers[0].set_focus(true);
                            }
                            else if index > 0 && index < 3 {
                                return self.components[index-1].handle(key);
                            }
                            return Ok(Command::None);
                        }
                    }
                    Ok(Command::None)
                },
                _ => Ok(Command::None)
            }
        } else {
            self.focused_child_mut().unwrap().handle(key)
        }
    }
}