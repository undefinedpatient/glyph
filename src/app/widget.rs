use crate::app::Command;
use color_eyre::eyre::Result;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Line;
use ratatui::style::Stylize;
use ratatui::widgets::{ListState, Widget};
use std::path::PathBuf;

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
    pub fn render_highlighted(&self, area: Rect, buf: &mut Buffer) {
        Line::from(
            [
                "[",
                self.label.as_str(),
                "]"
            ].concat()
        ).bold().centered().render(area, buf);
    }

}
pub struct DirectoryList {
    pub label: String,
    pub is_highlighted: bool,
    pub list_state: ListState,
    pub current_path: PathBuf,
}
impl DirectoryList {
    fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            is_highlighted: false,
            list_state: ListState::default(),
            current_path: std::env::current_dir().unwrap(),
        }
    }
}