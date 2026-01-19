use std::path::PathBuf;
use std::string::String;

#[derive(Clone)]
pub enum PopupConfirmType {
    Exit,
}
pub enum Popup {
    Info(String),
    Warning(String),
    Confirm(PopupConfirmType),
}

pub enum View {
    Entrance
}

// The State Object hold all the data in Navi
pub struct App {
    pub current_path: PathBuf,
    view_stack: Vec<View>,
    popup_stack: Vec<Popup>,


    info_message: Option<String>,
    should_quit: bool,
}

impl App {
    pub fn new(path: &PathBuf) -> App {
        App {
            current_path: path.clone(),
            view_stack: vec![View::Entrance],
            popup_stack: Vec::new(),

            info_message: None,
            should_quit: false,
        }
    }   
    pub fn push_popup(&mut self, popup: Popup) -> () {
        self.popup_stack.push(popup);
    }
    pub fn peek_popup(&self) -> Option<&Popup> {
        self.popup_stack.first()
    }
    pub fn pop_popup(&mut self) -> Option<Popup> {
        self.popup_stack.pop()
    }
    pub fn push_view(&mut self, view: View) -> () {
        self.view_stack.push(view);
    }
    pub fn peek_view(&self) -> Option<&View> {
        self.view_stack.first()
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
}