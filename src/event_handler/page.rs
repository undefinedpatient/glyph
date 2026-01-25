use crate::app::page::CreateGlyphPage;
use crate::app::popup::{ExitConfirmPopup, MessagePopup};
use crate::app::{page::EntrancePage, Command};
use crate::event_handler::{Focusable, Interactable};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use std::any::Any;
use crate::app::dialog::CreateGlyphDialog;
use crate::utils::create_glyph;

impl Interactable for EntrancePage {
    fn handle(&mut self, key: &KeyEvent, data: Option<&mut dyn Any>) -> color_eyre::Result<Vec<Command>> {
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
    fn handle(&mut self, key: &KeyEvent, data: Option<&mut dyn Any>) -> color_eyre::Result<Vec<Command>> {
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
                            match index {
                                0 => { // Directory List
                                    self.containers[index].set_focus(true);
                                }
                                1=> { // Back Button
                                    return self.components[0].handle(key, None);
                                }
                                2 => { // Confirm Button
                                    return self.components[1].handle(key, Some(&mut self.state));
                                }
                                _ => {

                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            Ok(Vec::new())
        } else {
            let index: usize = self.focused_child_index().unwrap();
            let mut result = self.containers[index].handle(
                key,
                Some(&mut self.state.path_to_create)
            );
            result
        }
    }
}
