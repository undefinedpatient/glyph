use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::string::String;

use ratatui::widgets::{List, ListState};

#[derive(Clone)]
pub enum PopupConfirmType {
    Exit,
}
pub enum Popup {
    Info(String),
    Warning(String),
    Error(String),
    Confirm(PopupConfirmType),
    CreateGlyphInput(String)
}

pub enum View {
    Entrance,
    CreateGlyph
}

#[derive(Hash, PartialEq, Eq)]
pub enum ListType {
    CreateGlyph,
    OpenGlyph,
    Glyph
}

// The State Object hold all the data in Navi
pub struct App {
    // UI
    s_views: Vec<View>,
    s_popup: Vec<Popup>,

    // State
    h_list_state: HashMap<ListType, ListState>,
    focused_list: Option<ListType>,
    current_path: PathBuf,  // ListType::{CreateGlyph, OpenGlyph} use this path

    info_message: Option<String>,
    warning_message: Option<String>,
    error_message: Option<String>,
    should_quit: bool,
}

impl App {
    pub fn new(path: &PathBuf) -> App {
        App {
            s_views: vec![View::Entrance],
            s_popup: Vec::new(),

            h_list_state: HashMap::from(
                [
                    (ListType::CreateGlyph, ListState::default()),
                    (ListType::OpenGlyph, ListState::default()),
                    (ListType::Glyph, ListState::default())
                ]
            ),
            focused_list: None,
            current_path: path.clone(),

            info_message: None,
            warning_message: None,
            error_message: None,
            should_quit: false,
        }
    }   
    // State
    pub fn focused_list_ref(&self) -> Option<&ListState> {
        if let Some(list_type) = &self.focused_list {
            return self.h_list_state.get(&list_type);
        }
        None
    }
    pub fn focused_list_state_mut(&mut self) -> Option<&mut ListState> {
        if let Some(list_type) = &self.focused_list {
            return self.h_list_state.get_mut(&list_type);
        }
        None
    }
    pub fn set_focusd_list(&mut self, list: ListType) -> () {
        self.focused_list = Some(list);
    }
    pub fn get_current_path(&self) -> &PathBuf {
        &self.current_path
    }
    pub fn set_current_path(&mut self, path_buf: &PathBuf) -> () {
        self.current_path = path_buf.clone();
    }
    // INFO / WARNING / ERROR Messages
    pub fn set_info_message(&mut self, message: &str) -> () {
        self.info_message = Some(String::from(message));
    }
    pub fn set_warning_message(&mut self, message: &str) -> () {
        self.warning_message = Some(String::from(message));
    }
    pub fn set_error_message(&mut self, message: &str) -> () {
        self.error_message = Some(String::from(message));
    }
    pub fn reset_info_message(&mut self) -> () {
        self.info_message = None;
    }
    pub fn reset_warning_message(&mut self) -> () {
        self.warning_message = None;
    }
    pub fn reset_error_message(&mut self) -> () {
        self.error_message = None;
    }
    pub fn info_message(&self) -> Option<String> {
        self.info_message.clone()
    }
    pub fn warning_message(&self) -> Option<String> {
        self.warning_message.clone()
    }
    pub fn error_message(&self) -> Option<String> {
        self.error_message.clone()
    }
    pub fn push_info_message(&mut self) -> () {
        if let Some(message) = self.info_message.clone() {
            self.push_popup(Popup::Info(message));
            self.info_message = None;
        }
    }
    pub fn push_warning_message(&mut self) -> () {
        if let Some(message) = self.warning_message.clone() {
            self.push_popup(Popup::Warning(message));
            self.warning_message = None;
        }
    }
    pub fn push_error_message(&mut self) -> () {
        if let Some(message) = self.error_message.clone() {
            self.push_popup(Popup::Error(message));
            self.error_message = None;
        }
    }
    // Views
    pub fn push_popup(&mut self, popup: Popup) -> () {
        self.s_popup.push(popup);
    }
    pub fn peek_popup_ref(&self) -> Option<&Popup> {
        self.s_popup.last()
    }
    pub fn pop_popup(&mut self) -> Option<Popup> {
        self.s_popup.pop()
    }
    pub fn push_view(&mut self, view: View) -> () {
        self.s_views.push(view);
    }
    pub fn peek_view_ref(&self) -> Option<&View> {
        self.s_views.last()
    }
    pub fn pop_view(&mut self) -> Option<View> {
        self.s_views.pop()
    }
    pub fn set_should_quit(&mut self, flag: bool) -> () {
        self.should_quit = flag;
    }
    pub fn get_should_quit(&mut self) -> &bool {
        &self.should_quit
    }
    // 
}