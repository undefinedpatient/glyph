use crate::app::page::CreateGlyphPage;
use crate::app::popup::{ExitConfirmPopup, MessagePopup};
use crate::app::{page::EntrancePage, Command, Stateful};
use crate::event_handler::{Focusable, Interactable};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

impl Interactable for EntrancePage {
    fn handle(&mut self, key: &KeyEvent) -> Result<Command> {
        match key.kind {
            KeyEventKind::Press=> {
                if let KeyCode::Tab = key.code {
                    if let Some(index) = self.hover_index {
                        self.hover_index = Some((index + 1usize) % self.interactables.len());

                    } else {
                        self.hover_index = Some(0);
                    }
                    return Ok(Command::None)
                }
                if let KeyCode::BackTab = key.code {
                    if let Some(index) = self.hover_index {
                        if index == 0 {
                            self.hover_index = Some(self.interactables.len()-1usize);
                        } else {
                            self.hover_index = Some(index-1usize);
                        }

                    } else {
                        self.hover_index = Some(self.interactables.len()-1usize);
                    }
                    return Ok(Command::None)
                }
                if let KeyCode::Esc = key.code {
                    return Ok(Command::PushPopup(Box::new(ExitConfirmPopup::new(true))));
                }
                if let KeyCode::Enter = key.code {
                    if let Some(index) = self.hover_index {
                        if index == 0 {
                            return Ok(Command::PushView(Box::new(CreateGlyphPage::new())));
                        }
                        if index == 1 {
                            return Ok(Command::PushPopup(Box::new(MessagePopup::new("Not Implemented"))));
                        }
                        if index == 2 {
                            return Ok(Command::PushPopup(Box::new(ExitConfirmPopup::new(true))));
                        }
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
    fn focused_child_ref(&self) -> Option<&dyn Stateful> {
        None
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Stateful>{
        None
    }
}
impl Interactable for CreateGlyphPage {
    fn handle(&mut self, key: &KeyEvent) -> Result<Command> {
        if self.focused_child_ref().is_some() {
            match key.kind {
                KeyEventKind::Press=> {
                    if let KeyCode::Tab = key.code {
                        return Ok(Command::None)
                    }
                    if let KeyCode::BackTab = key.code {
                        if let Some(index) = self.hover_index {
                        } else {
                        }
                        return Ok(Command::None)
                    }
                    if let KeyCode::Esc = key.code {
                        return Ok(Command::PushPopup(Box::new(ExitConfirmPopup::new(true))));
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
    fn focused_child_ref(&self) -> Option<&dyn Stateful> {
        None
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Stateful> {
        None
    }
}