use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::string::String;

use ratatui::widgets::{List, ListState};

use crate::app::states::{ApplicationState, WidgetStates};

pub mod states;

#[derive(Clone)]
pub enum View {
    Entrance,
    CreateGlyph,
    OpenGlyph,
    Glyph
}

pub enum Dialog {
    CreateGlyphInfo,
}

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


// The State Object hold all the data in Navi
pub struct App {
    // UI
    s_views: Vec<View>,
    s_dialogs: Vec<Dialog>,
    s_popup: Vec<Popup>,


    // Application Level State
    pub state: ApplicationState,
    pub widget_states: WidgetStates
}

impl App {
    pub fn new() -> App {
        App {
            s_views: vec![View::Entrance],
            s_dialogs: Vec::new(),
            s_popup: Vec::new(),


            state: ApplicationState::new(),
            widget_states: WidgetStates::new()
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