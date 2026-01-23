use crate::app::{page::Entrance, Command, Stateful};
use crate::event_handler::{Focusable, Interactable};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

impl Interactable for Entrance {
    fn handle(&mut self, key: &KeyEvent) -> Result<Command> {
        match key.kind {
            KeyEventKind::Press=> {
                if let KeyCode::Char(code) = key.code {
                    return match code {
                        'q' => {
                            Ok(Command::Quit)
                        },
                        'j' => {
                            if let Some(index) = self.hover_index {
                                self.buttons[index].is_highlighted = false;
                                self.hover_index = Some((index + 1usize) % self.buttons.len());
                                self.buttons[self.hover_index.unwrap()].is_highlighted = true;

                            } else {
                                self.hover_index = Some(0);
                                self.buttons[0].is_highlighted = true;
                            }
                            Ok(Command::None)
                        },
                        'k' => {
                            if let Some(index) = self.hover_index {
                                self.buttons[index].is_highlighted = false;
                                if index == 0 {
                                    self.hover_index = Some(self.buttons.len()-1usize);
                                    self.buttons[self.hover_index.unwrap()].is_highlighted = true;
                                } else {
                                    self.hover_index = Some(index-1usize);
                                    self.buttons[self.hover_index.unwrap()].is_highlighted = true;
                                }

                            } else {
                                self.hover_index = Some(self.buttons.len()-1usize);
                                self.buttons[self.hover_index.unwrap()].is_highlighted = true;
                            }
                            Ok(Command::None)
                        },
                        _ => Ok(Command::None)
                    }
                }
                if let KeyCode::Esc = key.code {
                    if let Some(index) = self.hover_index {
                        self.buttons[index].is_highlighted = false;
                        self.hover_index = None
                    }
                }
            },
            _ => {}
        }
        Ok(Command::None)
    }
}
impl Focusable for Entrance {
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
