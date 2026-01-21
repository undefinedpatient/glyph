use std::{collections::HashMap, path::{Path, PathBuf}};

use ratatui::widgets::ListState;


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
}




// Widget Level
#[derive(Hash, PartialEq, Eq)]
pub enum ListStateType {
    CreateGlyph,
    OpenGlyph,
    Glyph
}
pub enum DialogStateType {
    CreateGlyphInfo,
}

// A State has component states, which implement Focusable
pub struct DialogState{

}
trait Focusable {
    fn set_focused() -> ();
    fn is_focused() -> bool;
    fn shift_focus() -> ();
}
trait Component {

}
// This represents the Widget Level State
pub struct WidgetStates{
    // Widget Level State
    h_list_state: HashMap<ListStateType, ListState>,
    h_dialog_state: HashMap<DialogStateType, DialogState>,
    active_dialog: Option<DialogStateType>,
    active_list: Option<ListStateType>,
}

impl WidgetStates {
    pub fn new() -> Self {
        WidgetStates {
            h_list_state: HashMap::from(
                [
                    (ListStateType::CreateGlyph, ListState::default()),
                    (ListStateType::OpenGlyph, ListState::default()),
                    (ListStateType::Glyph, ListState::default())
                ]
            ),
            h_dialog_state: HashMap::new(),
            active_list: None,
            active_dialog: None,
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
    pub fn set_active_list(&mut self, list: ListStateType) -> () {
        self.active_list = Some(list);
    }
}

pub struct ViewState{

}

impl ViewState {

}

pub struct EntrancePageState{

}