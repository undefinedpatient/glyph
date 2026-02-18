use crate::app::{Command, Container, DrawFlag, Drawable, Focusable, Interactable};
use crate::block;
use crate::theme::Theme;
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Offset, Position, Rect};
use ratatui::prelude::Line;
use ratatui::prelude::Stylize;
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::widgets::{BorderType, Widget};
use ratatui::Frame;
use std::any::Any;

pub struct TextFieldState {
    pub is_focused: bool,
    pub label: String,
    pub chars: Vec<char>,
    pub cursor_index: usize,
    pub is_valid: bool,
}
pub struct TextField {
    pub state: TextFieldState,
    pub on_exit: Option<Box<dyn FnMut(Option<&mut dyn Any>,Option<&mut dyn Any>) -> Result<Vec<Command>>>>,
    pub validate: Box<dyn Fn(&str) -> bool>
}

impl TextField {
    pub fn new(label: &str, default: &str, validate: Box<dyn Fn(&str)->bool>) -> Self {
        Self {
            state: TextFieldState {
                is_focused: false,
                label: label.to_string(),
                chars: default.chars().collect(),
                cursor_index: default.len(),
                is_valid: true,
            },
            on_exit: None,
            validate
        }
    }
    pub fn replace(&mut self, content: String) -> () {
        self.state.chars = content.chars().collect();
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

impl From<TextField> for Box<dyn Container> {
    fn from(component: TextField) -> Self {
        Box::new(component)
    }
}
impl Drawable for TextField {
    fn render(&self, frame: &mut Frame, area: Rect, draw_flag: DrawFlag, theme: &dyn Theme) {
        // let text_field_area = area.centered(Constraint::Min(18), Constraint::Min(3));
        let content = self.state.chars.iter().collect::<String>();
        let content_paragraph: Paragraph = Paragraph::new(Line::from(content)).wrap(Wrap{trim: true});
        let mut text_field_block: Block = block!(self.state.label.as_str(),draw_flag,theme);
        if !self.state.is_valid {
            text_field_block = text_field_block.red().title_bottom("Invalid Input");
        }
        let content_area: Rect = text_field_block.inner(area);
        if self.is_focused() {
            let cursor_position: Position = area.as_position().offset(Offset {
                x: 1 + (self.state.cursor_index % content_area.width as usize) as i32,
                y: 1 + (self.state.cursor_index /content_area.width as usize) as i32 ,
            });
            frame.set_cursor_position(cursor_position);
        }
        // Clear.render(area, frame.buffer_mut());
        text_field_block.render(area, frame.buffer_mut());
        content_paragraph.render(content_area, frame.buffer_mut());
    }
}
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
                    // Validation
                    self.state.is_valid = (*self.validate)(self.state.chars.iter().collect::<String>().as_str());
                    Ok(Vec::new())
                }
                _ => Ok(Vec::new()),
            }
        }
    }
}

impl Focusable for TextField {
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
