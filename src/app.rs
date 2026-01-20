use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::string::String;

use ratatui::widgets::{List, ListState};

use crate::app::states::ApplicationState;

mod states;

#[derive(Clone)]
pub enum PopupConfirmType {
    Exit,
}
pub enum Popup {
    Info(String),
    Warning(String),
    Error(String),
    Confirm(PopupConfirmType),
}

pub enum Dialog {
    CreateGlyphInfo,
}

// A State has component states, which implement Focusable
pub struct DialogState{

}

pub enum View {
    Entrance,
    CreateGlyph,
    OpenGlyph,
    Glyph
}

#[derive(Hash, PartialEq, Eq)]
pub enum ListType {
    CreateGlyph,
    OpenGlyph,
    Glyph
}

trait Focusable {
    fn set_focused() -> ();
    fn is_focused() -> bool;
    fn shift_focus() -> ();
}

// The State Object hold all the data in Navi
pub struct App {
    // UI
    s_views: Vec<View>,
    s_dialogs: Vec<Dialog>,
    s_popup: Vec<Popup>,

    // Widget Level State
    h_list_state: HashMap<ListType, ListState>,
    h_dialog_state: HashMap<Dialog, DialogState>,
    active_dialog: Option<Dialog>,
    active_list: Option<ListType>,

    // Application Level State
    pub state: ApplicationState,
}

impl App {
    pub fn new() -> App {
        App {
            s_views: vec![View::Entrance],
            s_dialogs: Vec::new(),
            s_popup: Vec::new(),

            h_list_state: HashMap::from(
                [
                    (ListType::CreateGlyph, ListState::default()),
                    (ListType::OpenGlyph, ListState::default()),
                    (ListType::Glyph, ListState::default())
                ]
            ),
            h_dialog_state: HashMap::new(),
            active_list: None,
            active_dialog: None,

            state: ApplicationState::new()
        }
    }   
    // State
    pub fn active_list_state_ref(&self) -> Option<&ListState> {
        if let Some(list_type) = &self.active_list {
            return self.h_list_state.get(&list_type);
        }
        None
    }
    pub fn active_list_state_mut(&mut self) -> Option<&mut ListState> {
        if let Some(list_type) = &self.active_list {
            return self.h_list_state.get_mut(&list_type);
        }
        None
    }
    pub fn set_active_list(&mut self, list: ListType) -> () {
        self.active_list = Some(list);
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
    pub fn push_info_message(&mut self) -> () {
        if let Some(message) = self.state.info_message().clone() {
            self.push_popup(Popup::Info(message));
            self.state.reset_info_message();
        }
    }
    pub fn push_warning_message(&mut self) -> () {
        if let Some(message) = self.state.warning_message().clone() {
            self.push_popup(Popup::Warning(message));
            self.state.reset_warning_message();
        }
    }
    pub fn push_error_message(&mut self) -> () {
        if let Some(message) = self.state.error_message().clone() {
            self.push_popup(Popup::Error(message));
            self.state.reset_error_message();
        }
    }
    // 
}