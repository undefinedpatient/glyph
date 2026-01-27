use crate::app::{Command, Component};
use crate::drawer::DrawFlag;
use crate::utils::{get_dir_names, get_file_names};
use color_eyre::eyre::Result;
use ratatui::style::Stylize;
use ratatui::text::Line;
use std::any::Any;
use std::path::PathBuf;
/*
   Button
*/

pub struct SimpleButton {
    pub label: String,
    pub on_interact: Option<Box<dyn FnMut(Option<&mut dyn Any>) -> Result<Vec<Command>>>>,
}
impl SimpleButton {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            on_interact: None,
        }
    }
    pub fn on_interact(
        &mut self,
        f: Box<dyn FnMut(Option<&mut dyn Any>) -> Result<Vec<Command>>>,
    ) -> Self {
        Self {
            label: self.label.clone(),
            on_interact: Some(f),
        }
    }
}
impl From<SimpleButton> for Box<dyn Component> {
    fn from(component: SimpleButton) -> Self {
        Box::new(component)
    }
}
pub struct LineButton {
    pub label: String,
    pub on_interact: Option<Box<dyn FnMut(Option<&mut dyn Any>) -> Result<Vec<Command>>>>,
}
impl LineButton {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            on_interact: None,
        }
    }
    pub fn on_interact(
        &mut self,
        f: Box<dyn FnMut(Option<&mut dyn Any>) -> Result<Vec<Command>>>,
    ) -> Self {
        Self {
            label: self.label.clone(),
            on_interact: Some(f),
        }
    }

    pub fn as_line(&self, draw_flag: DrawFlag) -> Line<'_> {
        let text = self.label.clone().to_string();
        match draw_flag {
            DrawFlag::HIGHLIGHTING => Line::from(["[", text.as_str(), "]"].concat()).bold(),
            _ => Line::from([" ", text.as_str(), " "].concat()),
        }
    }
}
impl From<LineButton> for Box<dyn Component> {
    fn from(component: LineButton) -> Self {
        Box::new(component)
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
    pub hovered_index: Option<usize>,
    pub selected_index: Option<usize>,
    pub offset: usize,
    pub show_files: bool,
    pub select_dir: bool
}
impl DirectoryList {
    pub(crate) fn new(label: &str, show_files: bool, select_dir: bool) -> Self {
        Self {
            is_focused: false,
            label: label.to_string(),
            line_height: 1,
            current_path: std::env::current_dir().unwrap(),
            hovered_index: None,
            selected_index: None,
            offset: 0,
            show_files: show_files,
            select_dir
        }
    }
    pub fn get_num_files(&self) -> usize {
        get_file_names(&self.current_path).unwrap().len()
    }
    pub fn get_num_dirs(&self) -> usize {
        get_dir_names(&self.current_path).unwrap().len()
    }
    pub fn get_num_entries(&self) -> usize {
        if self.show_files {
            self.get_num_files() + self.get_num_dirs()
        } else {
            self.get_num_dirs()
        }
    }
    pub fn next_entry(&mut self) -> () {
        if let Some(index) = self.hovered_index {
            self.hovered_index = Some((index + 1usize) % self.get_num_entries());
        } else {
            self.hovered_index = Some(0);
        }
    }
    pub fn previous_entry(&mut self) -> () {
        if let Some(index) = self.hovered_index {
            if index == 0 {
                self.hovered_index = Some(self.get_num_entries() - 1usize);
            } else {
                self.hovered_index = Some(index - 1usize);
            }
        } else {
            self.hovered_index = Some(self.get_num_entries() - 1usize);
        }
    }
    pub fn page_up(&mut self) {
        self.offset = self.offset.saturating_sub(4);
    }
    pub fn page_down(&mut self) {
        self.offset += 4;
    }
}

impl From<DirectoryList> for Box<dyn Component> {
    fn from(component: DirectoryList) -> Self {
        Box::new(component)
    }
}
/*
   Text Field
*/

pub enum TextFieldInputMode {
    Normal,
    Edit,
}
pub struct TextField {
    pub is_focused: bool,
    pub label: String,
    pub chars: Vec<char>,
    pub cursor_index: usize,
    pub input_mode: TextFieldInputMode,
}

impl TextField {
    pub fn new(label: &str, default: String) -> Self {
        Self {
            is_focused: false,
            label: label.to_string(),
            chars: default.chars().collect(),
            cursor_index: default.len(),
            input_mode: TextFieldInputMode::Normal,
        }
    }
    pub fn move_to_next_char(&mut self) {
        self.cursor_index = self.cursor_index.saturating_add(1);
        if self.cursor_index >= self.chars.len() {
            self.cursor_index = self.chars.len();
        }
    }
    pub fn move_to_previous_char(&mut self) {
        self.cursor_index = self.cursor_index.saturating_sub(1);
    }
    pub fn insert_char(&mut self, char: char) {
        self.chars.insert(self.cursor_index, char);
    }
    pub fn delete_char(&mut self) {
        if self.cursor_index >= self.chars.len() {
            return;
        }
        self.chars.remove(self.cursor_index);
    }
    pub fn switch_mode(&mut self, mode: TextFieldInputMode) {
        self.input_mode = mode;
    }
    pub fn move_to_end_char(&mut self) {
        self.cursor_index = self.chars.len();
    }
    pub fn next_word(&mut self) {
        if self.chars.len() == 0 {
            return;
        }
        let (index, ch) = self
            .chars
            .iter()
            .enumerate()
            .find(|(i, item)| {
                if **item == ' ' && *i > self.cursor_index {
                    return true;
                }
                false
            })
            .unwrap_or_else(|| (self.chars.len(), self.chars.last().unwrap()));
        self.cursor_index = index;
    }
    pub fn previous_word(&mut self) {
        if self.chars.len() == 0 {
            return;
        }
        let (index, ch) = self
            .chars
            .iter()
            .enumerate()
            .rfind(|(i, item)| {
                if **item == ' ' && *i < self.cursor_index {
                    return true;
                }
                false
            })
            .unwrap_or_else(|| (0, self.chars.last().unwrap()));
        self.cursor_index = index;
    }
}

impl From<TextField> for Box<dyn Component> {
    fn from(component: TextField) -> Self {
        Box::new(component)
    }
}

/*
    Glyph Navigation Bar
 */

pub struct GlyphNavigationBar {
    
}
