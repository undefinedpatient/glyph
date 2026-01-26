use crate::app::widget::{DirectoryList, LineButton, SimpleButton, TextField, TextFieldInputMode};
use crate::app::Command;
use crate::event_handler::{Focusable, Interactable};
use crate::utils::get_dir_names;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::any::Any;
use std::path::PathBuf;

impl Interactable for SimpleButton {
    fn handle(
        &mut self,
        key: &KeyEvent,
        data: Option<&mut dyn Any>,
    ) -> color_eyre::Result<Vec<Command>> {
        let Some(mut f) = self.on_interact.take() else {
            return Ok(Vec::new());
        };
        let result = f(data);
        self.on_interact = Some(f);
        result
    }
}
impl Interactable for LineButton {
    fn handle(
        &mut self,
        key: &KeyEvent,
        data: Option<&mut dyn Any>,
    ) -> color_eyre::Result<Vec<Command>> {
        let Some(mut f) = self.on_interact.take() else {
            return Ok(Vec::new());
        };
        let result = f(data);
        self.on_interact = Some(f);
        result
    }
}
impl Interactable for DirectoryList {
    fn handle(
        &mut self,
        key: &KeyEvent,
        data: Option<&mut dyn Any>,
    ) -> color_eyre::Result<Vec<Command>> {
        if !self.is_focused() {
            self.set_focus(true);
            Ok(Vec::new())
        } else {
            match key.kind {
                KeyEventKind::Press => {
                    if let KeyCode::Char(char) = key.code {
                        return match char {
                            'j' => {
                                self.next_entry();
                                Ok(Vec::new())
                            }
                            'k' => {
                                self.previous_entry();
                                Ok(Vec::new())
                            }
                            'u' => {
                                if key.modifiers.contains(KeyModifiers::CONTROL) {
                                    self.page_up();
                                }
                                Ok(Vec::new())
                            }
                            'd' => {
                                if key.modifiers.contains(KeyModifiers::CONTROL) {
                                    self.page_down();
                                }
                                Ok(Vec::new())
                            }
                            _ => Ok(Vec::new()),
                        };
                    }
                    if let KeyCode::Tab = key.code {
                        self.next_entry();
                        return Ok(Vec::new());
                    }
                    if let KeyCode::BackTab = key.code {
                        self.previous_entry();
                        return Ok(Vec::new());
                    }
                    if let KeyCode::Esc = key.code {
                        self.set_focus(false);
                        let mut parent_data = data.unwrap().downcast_mut::<PathBuf>().unwrap();
                        *parent_data = self.current_path.clone();
                        return Ok(Vec::new());
                    }
                    if let KeyCode::Enter = key.code {
                        if let Some(index) = self.hover_index {
                            // "cd .."
                            if index == 0 {
                                if let Some(path_buf) = (&self.current_path).parent() {
                                    self.current_path = path_buf.to_path_buf().clone();
                                }
                                return Ok(Vec::new());
                            }
                            self.current_path = self.current_path.join(PathBuf::from(
                                get_dir_names(&self.current_path)?[index].to_string(),
                            ));
                            return Ok(Vec::new());
                        }
                    }
                    Ok(Vec::new())
                }
                _ => Ok(Vec::new()),
            }
        }
    }
}

/*
   Text Field
*/

impl Interactable for TextField {
    fn handle(
        &mut self,
        key: &KeyEvent,
        data: Option<&mut dyn Any>,
    ) -> color_eyre::Result<Vec<Command>> {
        if !self.is_focused() {
            self.set_focus(true);
            Ok(Vec::new())
        } else {
            match self.input_mode {
                TextFieldInputMode::Normal => {
                    if let KeyCode::Esc = key.code {
                        self.switch_mode(TextFieldInputMode::Normal);
                        self.set_focus(false);
                        let mut name = data.unwrap().downcast_mut::<String>().unwrap();
                        *name = self.chars.iter().collect::<String>();
                        return Ok(Vec::new());
                    }
                    if let KeyCode::Char(i) = key.code {
                        match i {
                            'i' => {
                                self.switch_mode(TextFieldInputMode::Edit);
                            }
                            'h' => {
                                self.move_to_previous_char();
                            }
                            'l' => {
                                self.move_to_next_char();
                            }
                            'x' => {
                                self.delete_char();
                            }
                            'A' => {
                                self.switch_mode(TextFieldInputMode::Edit);
                                self.move_to_end_char();
                            }
                            'w' | 'W' => {
                                self.next_word();
                            }
                            'b' | 'B' => {
                                self.previous_word();
                            }
                            'e' | 'E' => {
                                self.next_word();
                                self.move_to_previous_char();
                            }

                            _ => return Ok(Vec::new()),
                        }
                    }
                    if let KeyCode::Left = key.code {
                        self.move_to_previous_char();
                    }
                    if let KeyCode::Right = key.code {
                        self.move_to_next_char();
                    }
                    Ok(Vec::new())
                }
                TextFieldInputMode::Edit => {
                    if let KeyCode::Esc = key.code {
                        self.switch_mode(TextFieldInputMode::Normal);
                    }
                    if let KeyCode::Char(c) = key.code {
                        self.insert_char(c);
                        self.move_to_next_char();
                    }
                    if let KeyCode::Backspace = key.code {
                        self.move_to_previous_char();
                        self.delete_char();
                    }
                    if let KeyCode::Left = key.code {
                        self.move_to_previous_char();
                    }
                    if let KeyCode::Right = key.code {
                        self.move_to_next_char();
                    }
                    Ok(Vec::new())
                }
            }
        }
    }
}
