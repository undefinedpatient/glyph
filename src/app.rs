use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::string::String;

use ratatui::widgets::{List, ListState};

use crate::app::states::{ApplicationState, WidgetStates};

pub mod states;
pub mod views;
pub use views::*;



// The State Object hold all the data in Navi
pub struct App {
    // UI
    s_pages: Vec<PageView>,
    s_dialogs: Vec<DialogView>,
    s_popup: Vec<PopupView>,

    // Application Level State
    pub state: ApplicationState,
    pub widget_states: WidgetStates
}

impl App {
    pub fn new() -> App {
        App {
            s_pages: vec![PageView::Entrance],
            s_dialogs: Vec::new(),
            s_popup: Vec::new(),


            state: ApplicationState::new(),
            widget_states: WidgetStates::new()
        }
    }   
    // Views
    pub fn push_popup(&mut self, popup: PopupView) -> () {
        self.s_popup.push(popup);
    }
    pub fn peek_popup_ref(&self) -> Option<&PopupView> {
        self.s_popup.last()
    }
    pub fn pop_popup(&mut self) -> Option<PopupView> {
        self.s_popup.pop()
    }
    pub fn push_dialog(&mut self, dialog: DialogView) -> () {
        self.s_dialogs.push(dialog);
    }
    pub fn peek_dialog_ref(&self) -> Option<&DialogView> {
        self.s_dialogs.last()
    }
    pub fn pop_dialog(&mut self) -> Option<DialogView> {
        self.s_dialogs.pop()
    }
    pub fn push_page(&mut self, view: PageView) -> () {
        self.s_pages.push(view);
    }
    pub fn peek_page_ref(&self) -> Option<&PageView> {
        self.s_pages.last()
    }
    pub fn pop_page(&mut self) -> Option<PageView> {
        self.s_pages.pop()
    }
    pub fn push_info_message(&mut self) -> () {
        if let Some(message) = self.state.info_message().clone() {
            self.push_popup(PopupView::Info(message));
            self.state.reset_info_message();
        }
    }
    pub fn push_warning_message(&mut self) -> () {
        if let Some(message) = self.state.warning_message().clone() {
            self.push_popup(PopupView::Warning(message));
            self.state.reset_warning_message();
        }
    }
    pub fn push_error_message(&mut self) -> () {
        if let Some(message) = self.state.error_message().clone() {
            self.push_popup(PopupView::Error(message));
            self.state.reset_error_message();
        }
    }
    // 
}