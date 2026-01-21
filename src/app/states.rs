use std::{collections::HashMap, path::{Path, PathBuf}};

use ratatui::widgets::ListState;

use crate::app::widgets::{ButtonState, TextFieldState};

pub trait Focusable {
    fn set_focused(&mut self, focused: bool) -> ();
    fn is_focused(&self) -> bool;
}
pub enum MessageLevel{
    INFO,
    WARNING,
    ERROR
}
// Application Level
pub struct ApplicationState{
    current_path: PathBuf,
    info_message: Option<String>,
    warning_message: Option<String>,
    error_message: Option<String>,
    should_quit: bool,
}
impl ApplicationState{
    pub fn new() -> Self {
        let path_buf: PathBuf = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();
        ApplicationState { 
            current_path: path_buf,
            info_message: None, 
            warning_message: None, 
            error_message: None, 
            should_quit: false
        }
    }
    pub fn set_should_quit(&mut self, flag: bool) -> () {
        self.should_quit = flag;
    }
    pub fn get_should_quit(&mut self) -> &bool {
        &self.should_quit
    }
    pub fn get_current_path(&self) -> &PathBuf {
        &self.current_path
    }
    pub fn set_current_path(&mut self, path_buf: &PathBuf) -> () {
        self.current_path = path_buf.clone();
    }
    // INFO / WARNING / ERROR Messages
    pub fn set_message(&mut self, message: &str, level: MessageLevel) -> () {
        match level {
            MessageLevel::INFO => {
                self.info_message = Some(String::from(message));
            }
            MessageLevel::WARNING => {
                self.warning_message = Some(String::from(message));
            }
            MessageLevel::ERROR => {
                self.error_message = Some(String::from(message));
            }
        }
    }
    pub fn reset_message(&mut self, level: MessageLevel) -> () {
        match level {
            MessageLevel::INFO => {
                self.info_message = None;
            }
            MessageLevel::WARNING => {
                self.warning_message = None;
            }
            MessageLevel::ERROR => {
                self.error_message = None;
            }
        }
    }
    pub fn message(&self, level: MessageLevel) -> Option<String> {
        match level {
            MessageLevel::INFO => {
                self.info_message.clone()
            }
            MessageLevel::WARNING => {
                self.warning_message.clone()
            }
            MessageLevel::ERROR => {
                self.error_message.clone()
            }
        }
    }
}


// This represents the Widget Level State
pub enum PageState {
    Entrance {

    },
    CreateGlyph {
        list_state: ListState,
    },
}
pub enum DialogState {
    CreateGlyphInfo {
        text_field_state: TextFieldState,
        button_state: ButtonState,
    }
}
