use std::any::Any;
use std::path::Components;
use crate::app::page::CreateGlyphPage;
use crate::app::popup::ExitConfirmPopup;
use crate::app::{page::EntrancePage, Command, Component, Data, DataPackage};
use crate::event_handler::{Focusable, Interactable};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

impl Interactable for EntrancePage {
    fn handle(&mut self, key: &KeyEvent, data: Option<DataPackage>) -> color_eyre::Result<Vec<Command>> {
        match key.kind {
            KeyEventKind::Press => {
                if let KeyCode::Tab = key.code {
                    if let Some(index) = self.hover_index {
                        self.hover_index = Some((index + 1usize) % self.components.len());
                    } else {
                        self.hover_index = Some(0);
                    }
                    return Ok(Vec::new());
                }
                if let KeyCode::BackTab = key.code {
                    if let Some(index) = self.hover_index {
                        if index == 0 {
                            self.hover_index = Some(self.components.len() - 1usize);
                        } else {
                            self.hover_index = Some(index - 1usize);
                        }
                    } else {
                        self.hover_index = Some(self.components.len() - 1usize);
                    }
                    return Ok(Vec::new());
                }
                if let KeyCode::Esc = key.code {
                    return Ok(vec![Command::PushPopup(Box::new(ExitConfirmPopup::new(true)))]);
                }
                if let KeyCode::Enter = key.code {
                    if let Some(index) = self.hover_index {
                        return self.components[index].as_mut().handle(key, None);
                    }
                }
            }
            _ => {}
        }
        Ok(Vec::new())
    }
}
impl Interactable for CreateGlyphPage {
    fn handle(&mut self, key: &KeyEvent, data: Option<DataPackage>) -> color_eyre::Result<Vec<Command>> {
        if self.focused_child_ref().is_none() {
            match key.kind {
                KeyEventKind::Press => {
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
                    if let KeyCode::Esc = key.code {
                        return Ok(vec![Command::PopPage]);
                    }
                    if let KeyCode::Enter = key.code {
                        if let Some(index) = self.hover_index {
                            if index == 0 {
                                self.containers[index].set_focus(true);
                            } else {
                                return self.components[index - self.containers.len()].handle(key, None);
                            }
                        }
                    }
                }
                _ => {}
            }
            Ok(Vec::new())
        } else {
            self.focused_child_mut().unwrap().handle(key, None)
        }
    }
}
