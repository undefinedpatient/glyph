use std::path::{Path, PathBuf};
use std::string::String;

#[derive(Clone)]
pub enum PopupConfirmType {
    Exit,
}
pub enum Popup {
    Info(String),
    Warning(String),
    Confirm(PopupConfirmType),
    FileExplorer(PathBuf)
}

pub enum View {
    Entrance,
    CreateGlyph
}

// The State Object hold all the data in Navi
pub struct App {
    pub current_path: PathBuf,
    view_stack: Vec<View>,
    popup_stack: Vec<Popup>,
    section_index: u8, // Which section in view is underfocused

    info_message: Option<String>,
    should_quit: bool,
}

impl App {
    pub fn new(path: &PathBuf) -> App {
        App {
            current_path: path.clone(),
            view_stack: vec![View::Entrance],
            popup_stack: Vec::new(),
            section_index: 0,

            info_message: None,
            should_quit: false,
        }
    }   
    pub fn push_popup(&mut self, popup: Popup) -> () {
        self.popup_stack.push(popup);
    }
    pub fn peek_popup(&self) -> Option<&Popup> {
        self.popup_stack.last()
    }
    pub fn pop_popup(&mut self) -> Option<Popup> {
        self.popup_stack.pop()
    }
    pub fn push_view(&mut self, view: View) -> () {
        self.view_stack.push(view);
    }
    pub fn peek_view(&self) -> Option<&View> {
        self.view_stack.last()
    }
    pub fn pop_view(&mut self) -> Option<View> {
        self.view_stack.pop()
    }
    pub fn set_should_quit(&mut self, flag: bool) -> () {
        self.should_quit = flag;
    }
    pub fn get_should_quit(&mut self) -> &bool {
        &self.should_quit
    }
    pub fn get_section_index(&self) -> u8 {
        self.section_index
    }
    pub fn next_section(&mut self) -> u8 {
        self.section_index = (self.section_index + 1) % u8::MAX;
        self.section_index
    }
    pub fn previous_section(&mut self) -> u8 {
        self.section_index = (self.section_index - 1) % u8::MAX;
        self.section_index
    }

}