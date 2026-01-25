use crate::app::dialog::CreateGlyphDialog;
use crate::app::Command;
use crate::event_handler::{Focusable, Interactable};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use std::any::Any;
use std::path::PathBuf;
use crate::app::popup::MessagePopup;

impl Interactable for CreateGlyphDialog {
    fn handle(&mut self, key: &KeyEvent, data: Option<&mut dyn Any>) -> color_eyre::Result<Vec<Command>> {
        if self.focused_child_mut().is_none() {
            match key.kind {
                KeyEventKind::Press => {
                    if let KeyCode::Esc = key.code {
                        return Ok(vec![Command::PopDialog]);
                    }
                    if let KeyCode::Tab = key.code {
                        if let Some(index) = self.hover_index {
                            self.hover_index = Some(
                                (index + 1usize) % (self.components.len() + self.containers.len()),
                            );
                        } else {
                            self.hover_index = Some(0);
                        }
                        return Ok(Vec::new());
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
                        return Ok(Vec::new());
                    }
                    if let KeyCode::Enter = key.code {
                        if let Some(index) = self.hover_index {
                            return match index {
                                0 => {
                                    self.containers[0].set_focus(true);
                                    Ok(Vec::new())
                                }
                                1 => {
                                    self.components[0].handle(key, None)
                                }
                                2 => {
                                    let mut commands: Vec<Command> = Vec::new();
                                    commands.push(Command::PopDialog);
                                    commands.push(Command::CreateGlyph(
                                        self.path_buf.clone(),
                                        self.state.new_glyph_name.clone()
                                    ));
                                    commands.push(Command::PushPopup(
                                        Box::new(
                                            MessagePopup::new(["New Glyph Created at:\n", self.path_buf.to_str().unwrap()].concat().as_str())
                                        )
                                    ));
                                    Ok(commands)
                                    // return Ok(vec![Command::PopDialog, Command::CreateGlyph("",self.state.new_glyph_name.clone())]);
                                    // self.components[1].handle(key, Some(&mut self.state.new_glyph_name))
                                }
                                _ => Ok(Vec::new()),
                            }
                        }
                    }
                    Ok(Vec::new())
                },
                _ => Ok(Vec::new())
            }
        } else {
            let result = self.focused_child_mut().unwrap().handle(key, None);
            result
        }
    }
}