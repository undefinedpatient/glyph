use crate::app::widget::{Button, DirectoryList, LineButton, NumberField, OptionMenu, TextEditor, TextField};
use crate::app::Command;
use crate::event_handler::{is_cycle_backward_hover_key, is_cycle_forward_hover_key, Focusable, Interactable};
use crate::state::widget::EditMode;
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
            if is_cycle_forward_hover_key(key) {
                self.next_entry();
            }
            if is_cycle_backward_hover_key(key) {
                self.previous_entry();
            }
            match key.kind {
                KeyEventKind::Press => {
                    if let KeyCode::Char(char) = key.code {
                        return match char {
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
                                    if let Some(selected_index) = self.state.selected_index {
                                        if selected_index == hovered_index {
                                            self.state.selected_index = None;
                                            return Ok(Vec::new());
                                        }
                                    }
                                    if self.state.select_dir || hovered_index >= self.get_num_dirs() {
                                        self.state.selected_index = self.state.hovered_index;
                                    }
                                }
                                Ok(Vec::new())
                            }
                            _ => Ok(Vec::new()),
                        };
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
                            let result = (*on_exit)(parent_state, Some(&mut self.state));
                            self.on_exit = Some(on_exit);
                            return result;
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
/*
   Number Field
*/

impl Interactable for NumberField {
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
                            let result = (*on_exit)(parent_state, Some(&mut self.state));
                            self.on_exit = Some(on_exit);
                            return result;
                        };
                        return Ok(Vec::new());
                    }
                    if let KeyCode::Char(c) = key.code {
                        if c.is_numeric() {
                            self.insert_char(c);
                            self.move_to_next_char();
                        }
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

/*
    Text Editor
 */
impl Interactable for TextEditor {
    fn handle(&mut self, key: &KeyEvent, parent_state: Option<&mut dyn Any>) -> color_eyre::Result<Vec<Command>> {
        match self.state.mode {
            EditMode::Normal => {
                handle_normal_mode(self, key, parent_state)
            }
            EditMode::Insert => {
                handle_insert_mode(self, key)
            }
            EditMode::Visual => {
                handle_visual_mode(self, key)
            }
            EditMode::VisualLine => {
                handle_visual_line_mode(self, key)
            }
        }
    }
}
fn handle_normal_mode(me: &mut TextEditor, key: &KeyEvent, parent_state: Option<&mut dyn Any>) -> color_eyre::eyre::Result<Vec<Command>> {
    match key.kind {
        KeyEventKind::Press => {
            if let KeyCode::Esc = key.code {
                me.set_focus(false);
                if let Some(mut on_exit) = me.on_exit.take() {
                    let result = (*on_exit)(parent_state, Some(&mut me.state));
                    me.on_exit = Some(on_exit);
                    return result;
                };
                return Ok(Vec::new());
            }
            if let KeyCode::Char(c) = key.code {
                match c {
                    'h' => {
                        me.move_to_previous_char();
                    }
                    'j' => {
                        me.move_to_next_line();
                    }
                    'e' => {
                        me.move_to_next_char();
                        me.move_to_next_word()?;
                        me.move_to_previous_char();
                    }
                    'k' => {
                        me.move_to_previous_line();
                    }
                    'l' => {
                        me.move_to_next_char();
                    }
                    'a' => {
                        me.move_to_next_char();
                        me.switch_mode(EditMode::Insert);
                    }
                    'A' => {
                        me.move_to_end_of_line();
                        me.switch_mode(EditMode::Insert);
                    }
                    '0' => {
                        me.move_to_start_of_line();
                    }
                    '$' => {
                        me.move_to_end_of_line();
                    }
                    'I' => {
                        me.move_to_start_of_line();
                        me.switch_mode(EditMode::Insert);
                    }
                    'J' => {
                        me.merge_with_next_line();
                    }
                    'w' => {
                        me.move_to_next_word()?;
                    }
                    'b' => {
                        me.move_to_previous_word()?;
                    }
                    'x' => {
                        me.delete_char();
                    }
                    'd' => {
                        if key.modifiers.contains(KeyModifiers::CONTROL) {
                            me.scroll_vertical_offset(16)
                        }
                    }
                    'u' => {
                        if key.modifiers.contains(KeyModifiers::CONTROL) {
                            me.scroll_vertical_offset(-16)
                        }
                    }
                    'o' => {
                        me.insert_new_line_below();
                        me.move_to_next_line();
                        me.switch_mode(EditMode::Insert);
                    }
                    'O' => {
                        me.insert_new_line_above();
                        me.switch_mode(EditMode::Insert);
                    }
                    'i' => {
                        me.switch_mode(EditMode::Insert);
                    }
                    // 'v' => {
                    //     me.switch_mode(EditMode::Visual);
                    // }
                    // 'V' => {
                    //     me.switch_mode(EditMode::VisualLine);
                    // }
                    _ => {

                    }
                }
            }
            if let KeyCode::Left = key.code {
                me.move_to_previous_char();
            }
            if let KeyCode::Right = key.code {
                me.move_to_next_char();
            }
            if let KeyCode::Up = key.code {
                me.move_to_previous_line();
            }
            if let KeyCode::Down = key.code {
                me.move_to_next_line();
            }
            if let KeyCode::Backspace = key.code {
                if me.get_cursor_position().0 == 0 {
                    me.move_to_previous_line();
                    me.move_to_end_of_line();
                } else {
                    me.move_to_previous_char();
                }
            }
            Ok(Vec::new())
        }
        _ => Ok(Vec::new()),
    }
}
fn handle_insert_mode(me: &mut TextEditor, key: &KeyEvent) -> color_eyre::eyre::Result<Vec<Command>> {
    match key.kind {
        KeyEventKind::Press => {
            if let KeyCode::Esc = key.code {
                me.switch_mode(EditMode::Normal);
            }
            if let KeyCode::Char(c) = key.code {
                me.insert_char(c);
            }
            if let KeyCode::Left = key.code {
                me.move_to_previous_char();
            }
            if let KeyCode::Right = key.code {
                me.move_to_next_char();
            }
            if let KeyCode::Up = key.code {
                me.move_to_previous_line();
            }
            if let KeyCode::Down = key.code {
                me.move_to_next_line();
            }
            if let KeyCode::Tab = key.code {
                me.insert_char(' ');
                me.insert_char(' ');
                me.insert_char(' ');
                me.insert_char(' ');
            }
            if let KeyCode::Backspace = key.code {
                if me.get_cursor_position().0 == 0 {
                    me.move_to_previous_line();
                    me.move_to_end_of_line();
                    me.merge_with_next_line();
                }
                else {
                    me.move_to_previous_char();
                    me.delete_char();
                }
            }
            if let KeyCode::Enter = key.code {
                me.cut_into_next_newline();
            }
            Ok(Vec::new())
        }
        _ => Ok(Vec::new()),
    }

}
fn handle_visual_mode(me: &mut TextEditor, key: &KeyEvent) -> color_eyre::eyre::Result<Vec<Command>> {
    match key.kind {
        KeyEventKind::Press => {
            if let KeyCode::Esc = key.code {
                me.switch_mode(EditMode::Normal);
            }
            if let KeyCode::Char(c) = key.code {
                match c {
                    'h' => {
                        me.move_to_previous_char();
                    }
                    'j' => {
                        me.move_to_next_line();
                    }
                    'k' => {
                        me.move_to_previous_line();
                    }
                    'l' => {
                        me.move_to_next_char();
                    }
                    'i' => {
                        me.switch_mode(EditMode::Insert);
                    }
                    _ => {

                    }
                }
            }
            if let KeyCode::Left = key.code {
                me.move_to_next_char();
            }
            if let KeyCode::Right = key.code {
                me.move_to_previous_char();
            }
            if let KeyCode::Up = key.code {
                me.move_to_next_line();
            }
            if let KeyCode::Down = key.code {
                me.move_to_previous_line();
            }
            if let KeyCode::Backspace = key.code {
                me.move_to_previous_char();
            }
            Ok(Vec::new())
        }
        _ => Ok(Vec::new()),
    }

}
fn handle_visual_line_mode(me: &mut TextEditor, key: &KeyEvent) -> color_eyre::eyre::Result<Vec<Command>> {
    match key.kind {
        KeyEventKind::Press => {
            if let KeyCode::Esc = key.code {
                me.switch_mode(EditMode::Normal);
            }
            if let KeyCode::Char(c) = key.code {
                match c {
                    'h' => {
                        me.move_to_previous_char();
                    }
                    'j' => {
                        me.move_to_next_line();
                    }
                    'k' => {
                        me.move_to_previous_line();
                    }
                    'l' => {
                        me.move_to_next_char();
                    }
                    'i' => {
                        me.switch_mode(EditMode::Insert);
                    }
                    _ => {

                    }
                }
            }
            if let KeyCode::Left = key.code {
                me.move_to_next_char();
            }
            if let KeyCode::Right = key.code {
                me.move_to_previous_char();
            }
            if let KeyCode::Up = key.code {
                me.move_to_next_line();
            }
            if let KeyCode::Down = key.code {
                me.move_to_previous_line();
            }
            if let KeyCode::Backspace = key.code {
                me.move_to_previous_char();
            }
            Ok(Vec::new())
        }
        _ => Ok(Vec::new()),
    }

}

impl Interactable for OptionMenu {
    fn handle(
        &mut self,
        key: &KeyEvent,
        parent_state: Option<&mut dyn Any>,
    ) -> color_eyre::Result<Vec<Command>> {
        let len: u8 = self.state.options.len() as u8;
        self.state.current_index = (self.state.current_index + 1) % len;
        
        
        let Some(mut f) = self.on_update.take() else {
            return Ok(Vec::new());
        };
        let result = f(parent_state, Some(&mut self.state));
        self.on_update = Some(f);
        result
    }
}