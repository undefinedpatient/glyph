use crate::app::widget::{DirectoryList, SimpleButton};
use crate::app::Command;
use crate::event_handler::{Focusable, Interactable};
use crate::utils::get_dir_names;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers, ModifierKeyCode};
use std::path::PathBuf;

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
                    if let KeyCode::Char(char) = key.code {
                        return match char {
                            'j' => {
                                self.next_entry();
                                Ok(Command::None)
                            },
                            'k' => {
                                self.previous_entry();
                                Ok(Command::None)
                            }
                            'u' => {
                                if key.modifiers.contains(KeyModifiers::CONTROL) {
                                    self.page_up();
                                }
                                Ok(Command::None)
                            }
                            'd' => {
                                if key.modifiers.contains(KeyModifiers::CONTROL) {
                                    self.page_down();
                                }
                                Ok(Command::None)
                            }
                            _ => {
                                Ok(Command::None)
                            }
                        }
                    }
                    if let KeyCode::Tab = key.code {
                        self.next_entry();
                        return Ok(Command::None);
                    }
                    if let KeyCode::BackTab = key.code {
                        self.previous_entry();
                        return Ok(Command::None);
                    }
                    if let KeyCode::Esc = key.code {
                        self.set_focus(false);
                    }
                    if let KeyCode::Enter = key.code {
                        if let Some(index) = self.hover_index {
                            // "cd .."
                            if index == 0 {
                                if let Some(path_buf) = (&self.current_path).parent() {
                                    self.current_path = path_buf.to_path_buf().clone();
                                }
                                return Ok(Command::None);
                            }
                            self.current_path = self.current_path.join(PathBuf::from(
                                get_dir_names(&self.current_path)?[index].to_string(),
                            ));
                            return Ok(Command::None);
                        }
                    }
                    Ok(Command::None)
                }
                _ => Ok(Command::None),
            }
        }
    }
}
