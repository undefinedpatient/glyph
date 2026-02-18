use crate::app::{Command, Container, DrawFlag, Drawable, Focusable, Interactable};
use crate::block;
use crate::theme::Theme;
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Offset, Position, Rect};
use ratatui::prelude::Line;
use ratatui::style::Stylize;
use ratatui::widgets::BorderType;
use ratatui::widgets::{Block, Widget};
use ratatui::Frame;
use std::any::Any;

pub struct NumberFieldState {
    pub is_focused: bool,
    pub label: String,
    pub chars: Vec<char>,
    pub cursor_index: usize,
    pub is_valid: bool,
}
pub struct NumberField {
    pub state: NumberFieldState,
    pub on_exit: Option<Box<dyn FnMut(Option<&mut dyn Any>,Option<&mut dyn Any>) -> Result<Vec<Command>>>>,
    pub validate: Box<dyn Fn(&str) -> bool>
}

impl NumberField {
    pub fn new(label: &str, default: i16, validate: Box<dyn Fn(&str)->bool>) -> Self {
        Self {
            state: NumberFieldState {
                is_focused: false,
                label: label.to_string(),
                chars: default.to_string().chars().collect(),
                cursor_index: default.to_string().len(),
                is_valid: true,
            },
            on_exit: None,
            validate,
        }
    }
    pub fn replace(&mut self, content: i16) -> () {
        self.state.chars = content.to_string().chars().collect();
        self.state.cursor_index = self.state.chars.len();
    }
    pub fn on_exit(mut self, on_exit: Box<dyn FnMut(Option<&mut dyn Any>,Option<&mut dyn Any>)-> Result<Vec<Command>>>) -> Self {
        self.on_exit = Some(on_exit);
        self
    }
    pub fn move_to_next_char(&mut self) {
        self.state.cursor_index = self.state.cursor_index.saturating_add(1);
        if self.state.cursor_index >= self.state.chars.len() {
            self.state.cursor_index = self.state.chars.len();
        }
    }
    pub fn move_to_previous_char(&mut self) {
        self.state.cursor_index = self.state.cursor_index.saturating_sub(1);
    }
    pub fn insert_char(&mut self, char: char) {
        self.state.chars.insert(self.state.cursor_index, char);
    }
    pub fn delete_char(&mut self) {
        if self.state.cursor_index >= self.state.chars.len() {
            return;
        }
        self.state.chars.remove(self.state.cursor_index);
    }
}

impl From<NumberField> for Box<dyn Container> {
    fn from(component: NumberField) -> Self {
        Box::new(component)
    }
}

impl Drawable for NumberField {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        let text_field_area = area.centered(Constraint::Min(18), Constraint::Min(3));
        let text = self.state.chars.iter().collect::<String>();
        let text_line: Line = Line::from(text);
        let mut number_field_block: Block = block!(self.state.label.as_str(),draw_flag,theme);
        if !self.state.is_valid {
            number_field_block = number_field_block.red().title_bottom("Invalid Input");
        }
        let text_line_area: Rect = number_field_block.inner(text_field_area);
        if self.is_focused() {
            let cursor_position: Position = text_field_area.as_position().offset(Offset {
                x: 1 + self.state.cursor_index as i32,
                y: 1,
            });
            frame.set_cursor_position(cursor_position);
        }
        number_field_block.render(text_field_area, frame.buffer_mut());
        text_line.render(text_line_area, frame.buffer_mut());
    }
}
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
                    // Validation
                    self.state.is_valid = (*self.validate)(self.state.chars.iter().collect::<String>().as_str());
                    Ok(Vec::new())
                }
                _ => Ok(Vec::new()),
            }
        }
    }
}
impl Focusable for NumberField {
    fn is_focused(&self) -> bool {
        self.state.is_focused
    }
    fn set_focus(&mut self, value: bool) -> () {
        self.state.is_focused = value;
    }
    fn focused_child_ref(&self) -> Option<&dyn Container> {
        None
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container> {
        None
    }
    fn focused_child_index(&self) -> Option<usize> {
        None
    }
}
