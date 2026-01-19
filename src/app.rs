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
    Confirm(PopupConfirmType),
    FileExplorer(PathBuf)
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
            should_quit: false,
        }
    }   
    // State
    pub fn focused_list(&self) -> Option<&ListState> {
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
    // Views
    pub fn push_popup(&mut self, popup: Popup) -> () {
        self.s_popup.push(popup);
    }
    pub fn peek_popup(&self) -> Option<&Popup> {
        self.s_popup.last()
    }
    pub fn pop_popup(&mut self) -> Option<Popup> {
        self.s_popup.pop()
    }
    pub fn push_view(&mut self, view: View) -> () {
        self.s_views.push(view);
    }
    pub fn peek_view(&self) -> Option<&View> {
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