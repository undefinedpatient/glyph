use crate::app::page::CreateGlyphPage;
use crate::app::popup::{ExitConfirmPopup, MessagePopup};
use crate::app::{page::EntrancePage, Command, Container, Convertible};
use crate::event_handler::{Focusable, Interactable};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use crate::app::widget::SimpleButton;

impl Interactable for EntrancePage {
    fn handle(&mut self, key: &KeyEvent) -> Result<Command> {
        match key.kind {
            KeyEventKind::Press=> {
                if let KeyCode::Tab = key.code {
                    if let Some(index) = self.hover_index {
                        self.hover_index = Some((index + 1usize) % self.elements.len());

                    } else {
                        self.hover_index = Some(0);
                    }
                    return Ok(Command::None)
                }
                if let KeyCode::BackTab = key.code {
                    if let Some(index) = self.hover_index {
                        if index == 0 {
                            self.hover_index = Some(self.elements.len()-1usize);
                        } else {
                            self.hover_index = Some(index-1usize);
                        }

                    } else {
                        self.hover_index = Some(self.elements.len()-1usize);
                    }
                    return Ok(Command::None)
                }
                if let KeyCode::Esc = key.code {
                    return Ok(Command::PushPopup(Box::new(ExitConfirmPopup::new(true))));
                }
                if let KeyCode::Enter = key.code {
                    if let Some(index) = self.hover_index {
                        return self.elements[index].handle(key);
                    }
                }
            },
            _ => {}
        }
        Ok(Command::None)
    }
}
impl Focusable for EntrancePage {
    fn is_focused(&self) -> bool {
        self.is_focused
    }
    fn set_focus(&mut self, value: bool) -> () {
        self.is_focused = value;
    }
    fn focused_child_ref(&self) -> Option<&dyn Container> {
        None
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container>{
        None
    }
}
impl Interactable for CreateGlyphPage {
    fn handle(&mut self, key: &KeyEvent) -> Result<Command> {
        if self.focused_child_ref().is_none() {
            match key.kind {
                KeyEventKind::Press=> {
                    if let KeyCode::Tab = key.code {
                        if let Some(index) = self.hover_index {
                            self.hover_index = Some(
                                (index + 1usize) % ( self.elements.len()+self.containers.len())
                            );

                        } else {
                            self.hover_index = Some(0);
                        }
                        return Ok(Command::None)
                    }
                    if let KeyCode::BackTab = key.code {
                        if let Some(index) = self.hover_index {
                            if index == 0 {
                                self.hover_index = Some(( self.elements.len()+self.containers.len())-1usize);
                            } else {
                                self.hover_index = Some(index-1usize);
                            }

                        } else {
                            self.hover_index = Some(( self.elements.len()+self.containers.len())-1usize);
                        }
                        return Ok(Command::None)
                    }
                    if let KeyCode::Esc = key.code {
                        return Ok(Command::PopView);
                    }
                    if let KeyCode::Enter = key.code {
                        if let Some(index) = self.hover_index {
                            if index == 0 {
                                self.containers[index].set_focus(true);
                            } else{
                                return self.elements[index-self.containers.len()].handle(key);
                            }
                        }
                    }
                },
                _ => {}
            }
            Ok(Command::None)

        } else {
            self.focused_child_mut().unwrap().handle(key)?;
        Ok(Command::None)
    }
    }
}
impl Focusable for CreateGlyphPage {
    fn is_focused(&self) -> bool {
        self.is_focused
    }
    fn set_focus(&mut self, value: bool) -> () {
        self.is_focused = value;
    }
    fn focused_child_ref(&self) -> Option<&dyn Container> {
        for container in &self.containers {
            if container.is_focused() {
                return Some(& **container);
            }
        }
        None
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container> {
        for container in &mut self.containers {
            if container.is_focused() {
                return Some(&mut **container);
            }
        }
        None
    }
}