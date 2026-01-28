use crate::app::{Command, Component};
use crate::drawer::DrawFlag;
use crate::state::widget::TextFieldWidgetState;
use crate::utils::{cycle_offset, get_dir_names, get_file_names};
use color_eyre::eyre::Result;
use ratatui::style::Stylize;
use ratatui::text::Line;
use std::any::Any;
use std::path::PathBuf;

/*

    All widget's on_xxx() function only takes its direct parent's state.

 */



/*
    Button
    - on_interact(parent_state)
 */
pub struct Button {
    pub label: String,

    pub on_interact: Option<Box<dyn FnMut(Option<&mut dyn Any>) -> Result<Vec<Command>>>>,
}
impl Button {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            on_interact: None,
        }
    }
    pub fn on_interact(
        mut self,
        f: Box<dyn FnMut(Option<&mut dyn Any>) -> Result<Vec<Command>>>,
    ) -> Self {
        self.on_interact = Some(f);
        self
    }
}
impl From<Button> for Box<dyn Component> {
    fn from(component: Button) -> Self {
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
        mut self,
        f: Box<dyn FnMut(Option<&mut dyn Any>) -> Result<Vec<Command>>>,
    ) -> Self {
        self.on_interact = Some(f);
        self
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
            let num_entries = self.get_num_entries();
            self.hovered_index = Some(cycle_offset(index as u16, 1, num_entries as u16) as usize);
        } else {
            self.hovered_index = Some(0);
        }
    }
    pub fn previous_entry(&mut self) -> () {
        let num_entries = self.get_num_entries();
        if let Some(index) = self.hovered_index {
            self.hovered_index = Some(cycle_offset(index as u16, -1, num_entries as u16) as usize);
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
   - on_exit(parent_state)
*/
pub struct TextField {
    pub state: TextFieldWidgetState,
    pub on_exit: Option<Box<dyn FnMut(Option<&mut dyn Any>) -> Result<Vec<Command>>>>,
}

impl TextField {
    pub fn new(label: &str, default: String) -> Self {
        Self {
            state: TextFieldWidgetState {
                is_focused: false,
                label: label.to_string(),
                chars: default.chars().collect(),
                cursor_index: default.len(),
            },
            on_exit: None,
        }
    }
    pub fn on_exit(mut self, on_exit: Box<dyn FnMut(Option<&mut dyn Any>)-> Result<Vec<Command>>>) -> Self {
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

impl From<TextField> for Box<dyn Component> {
    fn from(component: TextField) -> Self {
        Box::new(component)
    }
}
