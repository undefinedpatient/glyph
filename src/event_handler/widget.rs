use crate::app::widget::{Button, DirectoryList, LineButton, TextField};
use crate::app::Command;
use crate::event_handler::{Focusable, Interactable};
use crate::utils::{get_dir_names, get_file_names};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::any::Any;
use std::path::PathBuf;

impl Interactable for Button {
    fn handle(
        &mut self,
        key: &KeyEvent,
        parent_state: Option<&mut dyn Any>,
    ) -> color_eyre::Result<Vec<Command>> {
        let Some(mut f) = self.on_interact.take() else {
            return Ok(Vec::new());
        };
        let result = f(parent_state);
        self.on_interact = Some(f);
        result
    }
}
impl Interactable for LineButton {
    fn handle(
        &mut self,
        key: &KeyEvent,
        parent_state: Option<&mut dyn Any>,
    ) -> color_eyre::Result<Vec<Command>> {
        let Some(mut f) = self.on_interact.take() else {
            return Ok(Vec::new());
        };
        let result = f(parent_state);
        self.on_interact = Some(f);
        result
    }
}
impl Interactable for DirectoryList {
    fn handle(
        &mut self,
        key: &KeyEvent,
        parent_state: Option<&mut dyn Any>,
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
                            ' ' => {
                                if let Some(hovered_index) = self.state.hovered_index {
                                    if self.state.select_dir || hovered_index >= self.get_num_dirs() {
                                        self.state.selected_index = self.state.hovered_index;
                                    }

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
                        if let Some(selected_index) = self.state.selected_index {
                            let mut entries = get_dir_names(self.state.current_path.as_path()).unwrap_or(Vec::new());
                            entries.append(&mut get_file_names(self.state.current_path.as_path()).unwrap_or(Vec::new()));
                            self.state.selected_file_path = Some(self.state.current_path.join(entries[selected_index].clone()));
                        } else {
                            self.state.selected_file_path = Some(self.state.current_path.clone());
                        }
                        self.set_focus(false);
                        if let Some(mut on_exit) = self.on_exit.take() {
                            let result = on_exit(parent_state, Some(&mut self.state));
                            self.on_exit = Some(on_exit);
                            return result;
                        }
                        return Ok(Vec::new());
                    }
                    if let KeyCode::Enter = key.code {
                        if let Some(index) = self.state.hovered_index {
                            // "cd .."
                            if index == 0 {
                                if let Some(path_buf) = (&self.state.current_path).parent() {
                                    self.state.current_path = path_buf.to_path_buf().clone();
                                }
                                return Ok(Vec::new());
                            }
                            if index < get_dir_names(&self.state.current_path).unwrap_or(Vec::new()).len() {
                                self.state.current_path = self.state.current_path.join(PathBuf::from(
                                    get_dir_names(&self.state.current_path)?[index].to_string(),
                                ));
                                self.state.selected_index = None;
                                self.state.hovered_index = Some(0);
                            }
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
        parent_state: Option<&mut dyn Any>,
    ) -> color_eyre::Result<Vec<Command>> {
        if !self.is_focused() {
            self.set_focus(true);
            Ok(Vec::new())
        } else {
            match key.kind {
                KeyEventKind::Press => {
                    if let KeyCode::Esc = key.code {
                        self.set_focus(false);
                        if let Some(mut on_exit) = self.on_exit.take() {
                            return (*on_exit)(parent_state, Some(&mut self.state));
                        };
                        return Ok(Vec::new());
                    }
                    if let KeyCode::Char(c) = key.code {
                        self.insert_char(c);
                        self.move_to_next_char();
                    }
                    if let KeyCode::Left = key.code {
                        self.move_to_previous_char();
                    }
                    if let KeyCode::Right = key.code {
                        self.move_to_next_char();
                    }
                    if let KeyCode::Backspace = key.code {
                        self.move_to_previous_char();
                        self.delete_char();
                    }
                    Ok(Vec::new())
                }
                _ => Ok(Vec::new()),
            }
        }
    }
}
