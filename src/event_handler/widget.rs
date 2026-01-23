use crate::app::widget::{DirectoryList, SimpleButton};
use crate::app::Command;
use crate::event_handler::{Focusable, Interactable};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
impl Interactable for SimpleButton {
    fn handle(&mut self, key: &KeyEvent) -> color_eyre::Result<Command> {
        if let Some(f) = &mut self.on_interact {
            f()
        } else {
            Ok(Command::None)
        }
    }
}
impl Interactable for DirectoryList {
    fn handle(&mut self, key: &KeyEvent) -> color_eyre::Result<Command> {
        if !self.is_focused() {
            self.set_focus(true);
            Ok(Command::None)
        } else {
            match key.kind {
                KeyEventKind::Press => {
                    if let KeyCode::Tab = key.code {
                        if let Some(index) = self.hover_index {
                            self.hover_index = Some(
                                (index + 1usize) % self.get_num_entry()
                            );

                        } else {
                            self.hover_index = Some(0);
                        }
                        return Ok(Command::None)
                    }
                    if let KeyCode::BackTab = key.code {
                        if let Some(index) = self.hover_index {
                            if index == 0 {
                                self.hover_index = Some(self.get_num_entry() - 1usize);
                            } else {
                                self.hover_index = Some(index-1usize);
                            }

                        } else {
                            self.hover_index = Some(self.get_num_entry() - 1usize);
                        }
                        return Ok(Command::None)
                    }
                    if let KeyCode::Esc = key.code {
                        self.set_focus(false);
                    }
                    Ok(Command::None)
                },
                _ => Ok(Command::None),
            }
        }
    }
}