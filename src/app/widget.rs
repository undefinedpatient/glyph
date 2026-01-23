use crate::app::{Command, Container};
use color_eyre::eyre::Result;
use std::path::PathBuf;
use crate::event_handler::Focusable;
use crate::utils::get_dir_names;

pub struct SimpleButton {
    pub label: String,
    pub on_interact: Option<Box<dyn FnMut() -> Result<Command>>>
}
impl SimpleButton {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            on_interact: None
        }
    }
    pub fn on_interact(&mut self, f: Box<dyn FnMut() -> Result<Command>>) -> Self {
        Self {
            label: self.label.clone(),
            on_interact: Some(f)
        }
    }

}
pub struct DirectoryList {
    pub is_focused: bool,
    pub label: String,
    pub line_height: usize,
    pub current_path: PathBuf,
    pub hover_index: Option<usize>,
}
impl DirectoryList {
    pub(crate) fn new(label: &str) -> Self {
        Self {
            is_focused: false,
            label: label.to_string(),
            line_height: 1,
            current_path: std::env::current_dir().unwrap(),
            hover_index: None
        }
    }
    pub fn get_num_entry(&self) -> usize {
        get_dir_names(&self.current_path).unwrap().len()
    }
}
impl Focusable for DirectoryList {
    fn is_focused(&self) -> bool {
        self.is_focused
    }
    fn set_focus(&mut self, value: bool) -> () {
        self.is_focused = value;
    }
    fn focused_child_ref(&self) -> Option<&dyn Container> {
        None
    }
    fn focused_child_mut(&mut self) -> Option<&mut dyn Container> {
        None
    }
}