use crate::app::{Command, Container};
use crate::event_handler::Focusable;
use crate::utils::get_dir_names;
use color_eyre::eyre::Result;
use std::path::PathBuf;


/*
    Button
 */


pub struct SimpleButton {
    pub label: String,
    pub on_interact: Option<Box<dyn FnMut() -> Result<Command>>>,
}
impl SimpleButton {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            on_interact: None,
        }
    }
    pub fn on_interact(&mut self, f: Box<dyn FnMut() -> Result<Command>>) -> Self {
        Self {
            label: self.label.clone(),
            on_interact: Some(f),
        }
    }
}


/*
    Directory Lists
 */


pub struct DirectoryList {
    pub is_focused: bool,
    pub label: String,
    pub line_height: usize,
    pub current_path: PathBuf,
    pub hover_index: Option<usize>,
    pub offset: usize,
}
impl DirectoryList {
    pub(crate) fn new(label: &str) -> Self {
        Self {
            is_focused: false,
            label: label.to_string(),
            line_height: 1,
            current_path: std::env::current_dir().unwrap(),
            hover_index: None,
            offset: 0,
        }
    }
    pub fn get_num_entry(&self) -> usize {
        get_dir_names(&self.current_path).unwrap().len()
    }
    pub fn next_entry(&mut self) -> () {
        if let Some(index) = self.hover_index {
            self.hover_index = Some((index + 1usize) % self.get_num_entry());
        } else {
            self.hover_index = Some(0);
        }
    }
    pub fn previous_entry(&mut self) -> () {
        if let Some(index) = self.hover_index {
            if index == 0 {
                self.hover_index = Some(self.get_num_entry() - 1usize);
            } else {
                self.hover_index = Some(index - 1usize);
            }
        } else {
            self.hover_index = Some(self.get_num_entry() - 1usize);
        }
    }
    pub fn page_up(&mut self) {
        self.offset = self.offset.saturating_sub(4);
    }
    pub fn page_down(&mut self) {
        self.offset += 4;
    }
}


/*
    Text Field
 */


pub enum TextFieldInputMode {
    NORMAL,
    EDIT
}
pub struct TextField {
    pub label: String,
    pub chars: Vec<char>,
    pub cursor_index: usize,
    pub input_mode: TextFieldInputMode,
}

impl TextField {
    pub fn new(label: &str, chars: Vec<char>) -> Self {
        Self {
            label: label.to_string(),
            chars: chars,
            cursor_index: 0,
            input_mode: TextFieldInputMode::NORMAL,
        }
    }
}